use crate::process::types::ProcessError;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct VideoCreator;

impl VideoCreator {
    /// Create video from temp frames
    pub fn create_video_from_temp_frames(
        temp_frame_dir: &str,
        output_video_path: &PathBuf,
        fps: i32,
    ) -> Result<(), ProcessError> {
        let frame_source_dir = Path::new(temp_frame_dir);
        let final_output_dir = output_video_path.parent().unwrap_or_else(|| Path::new("."));

        fs::create_dir_all(final_output_dir).map_err(|e| {
            ProcessError::IoError(format!("Failed to create output directory: {}", e))
        })?;

        if !frame_source_dir.exists() {
            eprintln!(
                "Warning: Temporary frame directory {} does not exist. Skipping video creation.",
                temp_frame_dir
            );
            return Ok(());
        }

        let mut image_files: Vec<fs::DirEntry> = match fs::read_dir(frame_source_dir) {
            Ok(reader) => reader
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry.path().is_file()
                        && entry
                            .path()
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| ext.eq_ignore_ascii_case("jpg"))
                            .unwrap_or(false)
                })
                .collect(),
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read temporary frame directory {}: {}. Skipping video creation.",
                    temp_frame_dir, e
                );
                return Ok(());
            }
        };

        if image_files.is_empty() {
            println!(
                "No .jpg frames found in {}. No video will be created.",
                temp_frame_dir
            );
            return Ok(());
        }

        // Sort files by frame number
        image_files.sort_by(|a, b| {
            let path_a = a.path();
            let path_b = b.path();
            let filename_a = path_a.file_stem().and_then(|n| n.to_str()).unwrap_or("");
            let filename_b = path_b.file_stem().and_then(|n| n.to_str()).unwrap_or("");

            match (
                Self::parse_frame_filename(filename_a),
                Self::parse_frame_filename(filename_b),
            ) {
                (Some((vid_a, frame_a)), Some((vid_b, frame_b))) => {
                    vid_a.cmp(&vid_b).then_with(|| frame_a.cmp(&frame_b))
                }
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        });

        // Create FFmpeg list file
        let list_file_path = frame_source_dir.join("ffmpeg_list.txt");
        {
            let mut list_file = fs::File::create(&list_file_path).map_err(|e| {
                ProcessError::IoError(format!("Failed to create ffmpeg list file: {}", e))
            })?;
            for entry in &image_files {
                match fs::canonicalize(entry.path()) {
                    Ok(absolute_path) => {
                        let path_str = absolute_path.to_string_lossy().replace("\\", "/");
                        if writeln!(list_file, "file '{}'", path_str).is_err() {
                            eprintln!(
                                "Error writing to ffmpeg list file for {}",
                                entry.path().display()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Warning: Could not canonicalize path {}: {}",
                            entry.path().display(),
                            e
                        );
                    }
                }
            }
            // Add last frame duration logic if needed, but standard ffmpeg concat list usually just needs files
            // Note: standard 'file' directive implies 1 frame duration if not specified?
            // Actually for images, we normally use 'duration' directive or loop.
            // But the original code just listed files. Wait, let's check original logic in video.rs to be sure.
            // Original logic used -r input option? No, let's stick to strict copy for now.
            // Actually, the original implementation simply listed files.
        }

        // Run FFmpeg to create video
        println!(
            "Creating video from {} frames at {} FPS: {}",
            image_files.len(),
            fps,
            output_video_path.display()
        );

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-f")
            .arg("concat")
            .arg("-safe")
            .arg("0")
            .arg("-i")
            .arg(list_file_path.to_str().unwrap())
            .arg("-c:v")
            .arg("libx264")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg("-r")
            .arg(fps.to_string())
            .arg("-y") // Overwrite output
            .arg(output_video_path.to_str().unwrap())
            .arg("-hide_banner")
            .arg("-loglevel")
            .arg("warning");

        let output = cmd.output().map_err(|e| {
            ProcessError::ProcessingFailed(format!(
                "Failed to execute ffmpeg for video creation: {}",
                e
            ))
        })?;

        if !output.status.success() {
            eprintln!("ffmpeg stdout: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("ffmpeg stderr: {}", String::from_utf8_lossy(&output.stderr));
            return Err(ProcessError::ProcessingFailed(format!(
                "Failed to create output video {}",
                output_video_path.display()
            )));
        }

        println!(
            "Successfully created video: {}",
            output_video_path.display()
        );

        Ok(())
    }

    /// Parse frame filename to extract video index and frame number
    /// Expected format: video{VideoIndex}_frame{FrameNumber}.jpg
    fn parse_frame_filename(filename: &str) -> Option<(usize, usize)> {
        // Regex would be cleaner but let's stick to simple parsing to avoid adding deps
        // Format: video0_frame000123
        if !filename.starts_with("video") {
            return None;
        }

        let parts: Vec<&str> = filename.split("_frame").collect();
        if parts.len() != 2 {
            return None;
        }

        let video_part = parts[0].strip_prefix("video")?;
        let frame_part = parts[1]; // e.g. "000123"

        let video_idx = video_part.parse::<usize>().ok()?;
        let frame_idx = frame_part.parse::<usize>().ok()?;

        Some((video_idx, frame_idx))
    }
}
