use crate::annotation::overlay::add_text_overlay_with_position;
use crate::annotation::types::{AnnotationConfig, AnnotationType, DataSource, format_timestamp};
use opencv::{core::Vector, imgcodecs};
use regex::Regex;
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct FrameAnnotator {
    config: AnnotationConfig,
}

impl FrameAnnotator {
    pub fn new(config: AnnotationConfig) -> Self {
        Self { config }
    }

    pub fn process(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &self.config.input {
            DataSource::Image(path) => self.process_single_image(path),
            DataSource::FrameDir(dir) => self.process_video_frames(dir),
        }
    }

    fn process_single_image(
        &self,
        input_path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut frame = imgcodecs::imread(input_path, imgcodecs::IMREAD_COLOR)?;

        // Determine annotation text
        let filename = Path::new(input_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let text = match &self.config.annotation_type {
            AnnotationType::Filename => filename.to_string(),
            AnnotationType::Custom(s) => s.clone(),
            AnnotationType::Timestamp => {
                // For single image, timestamp might not make sense without index,
                // but we can default to 0 or skip. Let's use 00:00:00.000 or filename
                "00:00:00.000".to_string()
            }
        };

        add_text_overlay_with_position(&mut frame, &text, self.config.text_position.clone())?;

        // Ensure output directory exists (if path contains one)
        if let Some(parent) = Path::new(&self.config.output_path).parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        imgcodecs::imwrite(&self.config.output_path, &frame, &Vector::new())?;
        println!("Saved annotated image to: {}", self.config.output_path);
        Ok(())
    }

    fn process_video_frames(
        &self,
        frames_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output_dir = Path::new(frames_dir);
        let temp_dir = output_dir.join("temp_annotated");
        fs::create_dir_all(&temp_dir)?;

        let mut image_files: Vec<_> = fs::read_dir(output_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext.eq_ignore_ascii_case("jpg"))
                    .unwrap_or(false)
            })
            .collect();

        image_files.sort_by_key(|entry| entry.path());

        let frame_regex = Regex::new(r"(\d+)")?;
        let fps = self.config.source_fps.unwrap_or(30.0);

        for entry in &image_files {
            let path = entry.path();
            let filename = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown");

            let annotation_text = match &self.config.annotation_type {
                AnnotationType::Filename => filename.to_string(),
                AnnotationType::Timestamp => {
                    let frame_index = frame_regex
                        .find(filename)
                        .and_then(|m| m.as_str().parse::<usize>().ok())
                        .unwrap_or(0);
                    format_timestamp(frame_index, fps)
                }
                AnnotationType::Custom(text) => text.clone(),
            };

            let mut img = imgcodecs::imread(path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;

            add_text_overlay_with_position(
                &mut img,
                &annotation_text,
                self.config.text_position.clone(),
            )?;

            let output_path = temp_dir.join(filename);
            imgcodecs::imwrite(output_path.to_str().unwrap(), &img, &Vector::new())?;
        }

        // Video Generation
        if let Some(video_config) = &self.config.video_encoding {
            // If output_path is just a filename, put it in the frames dir?
            // The requirement was unified "output_path". Let's use full path.
            let output_video_path = &self.config.output_path;

            let mut cmd = Command::new("ffmpeg");
            cmd.arg("-y")
                .arg("-framerate")
                .arg(video_config.fps.to_string())
                .arg("-pattern_type")
                .arg("glob")
                .arg("-i")
                .arg(format!("{}/*.jpg", temp_dir.to_str().unwrap()))
                .arg("-c:v")
                .arg("libx264")
                .arg("-pix_fmt")
                .arg("yuv420p")
                .arg(output_video_path);

            let output = cmd.output()?;
            if !output.status.success() {
                return Err(format!(
                    "Failed to create video: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
                .into());
            }
            println!("Successfully created video: {}", output_video_path);
        }

        fs::remove_dir_all(temp_dir)?;

        Ok(())
    }
}
