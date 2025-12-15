//! Video processing functionality for the process module

use opencv::{
    core::{Mat, Size},
    prelude::*,
    videoio,
};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use super::hw_accel::HardwareAcceleratedCapture;
use crate::process::config::VideoExtractionConfig;
use crate::process::stats::ProcessingStats;
use crate::process::types::ProcessError;

// Import new worker modules
use super::workers::creator::VideoCreator;
use super::workers::extractors::FrameExtractor;
use super::workers::scanner::VideoScanner;

/// Video processing functionality
pub struct VideoProcessor;

impl VideoProcessor {
    /// Run video extraction processing
    pub fn run_video_extraction(
        config_path: &str,
        stats: &mut ProcessingStats,
    ) -> Result<(), ProcessError> {
        let start_time = Instant::now();

        let config_data = fs::read_to_string(config_path).map_err(|e| {
            ProcessError::IoError(format!("Unable to read config file {}: {}", config_path, e))
        })?;

        // Config parsing logic
        let video_config =
            match serde_json::from_str::<crate::process::config::ProcessConfig>(&config_data) {
                Ok(process_config) => {
                    if let Some(vc) = process_config.video_config {
                        vc
                    } else {
                        return Err(ProcessError::ConfigurationError(
                        "Config file is a valid ProcessConfig but missing 'video_config' field."
                            .to_string(),
                    ));
                    }
                }
                Err(_) => {
                    // Fallback: Try to parse as VideoExtractionConfig directly
                    let deserializer = &mut serde_json::Deserializer::from_str(&config_data);
                    serde_path_to_error::deserialize(deserializer).map_err(|e| {
                        ProcessError::ConfigurationError(format!(
                            "Error parsing config.json at '{}': {}",
                            e.path(),
                            e
                        ))
                    })?
                }
            };

        let config = Arc::new(video_config);
        let temp_dirs_created = Arc::new(Mutex::new(Vec::<PathBuf>::new()));

        // Use Scanner Worker
        let video_files_by_dir = VideoScanner::scan(&config)?;

        let processing_mode = config.processing_mode.as_deref().unwrap_or("parallel");

        match processing_mode {
            "sequential" => {
                println!("Running in sequential mode.");
                for (dir_path, video_list) in video_files_by_dir {
                    if let Err(e) = Self::process_video_directory(
                        dir_path.clone(),
                        video_list,
                        Arc::clone(&config),
                        Arc::clone(&temp_dirs_created),
                    ) {
                        eprintln!("Error processing directory {}: {}", dir_path, e);
                        stats.add_failed_file(format!("Directory {}: {}", dir_path, e));
                    }
                }
            }
            "parallel" | _ => {
                println!("Running in parallel mode.");
                let num_threads = config.num_threads.unwrap_or_else(num_cpus::get);
                rayon::ThreadPoolBuilder::new()
                    .num_threads(num_threads)
                    .build_global()
                    .map_err(|e| {
                        ProcessError::ProcessingFailed(format!(
                            "Failed to build thread pool: {}",
                            e
                        ))
                    })?;

                video_files_by_dir
                    .into_par_iter()
                    .for_each(|(dir_path, video_list)| {
                        if let Err(e) = Self::process_video_directory(
                            dir_path.clone(),
                            video_list,
                            Arc::clone(&config),
                            Arc::clone(&temp_dirs_created),
                        ) {
                            eprintln!("Error processing directory in parallel {}: {}", dir_path, e);
                        }
                    });
            }
        }

        // Cleanup temporary directories
        {
            let dirs_to_clean = temp_dirs_created.lock().unwrap();
            for dir in dirs_to_clean.iter() {
                println!("Cleaning up temporary directory: {}", dir.display());
                if let Err(e) = fs::remove_dir_all(dir) {
                    eprintln!(
                        "Warning: Failed to remove temporary directory {}: {}",
                        dir.display(),
                        e
                    );
                }
            }
        }

        let duration = start_time.elapsed();
        println!("Total execution time: {:?}", duration);
        stats.processing_time = duration;

        Ok(())
    }

    /// Process video directory
    fn process_video_directory(
        input_dir_path: String,
        video_list: Vec<PathBuf>,
        config: Arc<VideoExtractionConfig>,
        temp_dirs_created: Arc<Mutex<Vec<PathBuf>>>,
    ) -> Result<(), ProcessError> {
        let dir_tag = std::path::Path::new(&input_dir_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("default")
            .to_string();

        println!(
            "Thread {:?} processing directory: {} ({} videos, tag: '{}')",
            thread::current().id(),
            input_dir_path,
            video_list.len(),
            dir_tag
        );

        let output_base = PathBuf::from(&config.output_directory);
        fs::create_dir_all(&output_base).map_err(|e| {
            ProcessError::IoError(format!("Failed to create output directory: {}", e))
        })?;

        let output_video_file = format!("{}_{}.mp4", config.output_prefix, dir_tag);
        let output_video_path = output_base.join(output_video_file);

        // Determine modes
        let creation_mode = config
            .video_creation_mode
            .as_deref()
            .unwrap_or("temp_frames");
        let use_ffmpeg_extraction = config.extraction_mode == "ffmpeg";

        println!(
            "Processing with: Extraction = {}, Creation = {}",
            if use_ffmpeg_extraction {
                "ffmpeg"
            } else {
                "opencv"
            },
            creation_mode
        );

        let mut sorted_video_list = video_list;
        sorted_video_list.sort();

        // Process based on modes
        if creation_mode == "direct" && config.extraction_mode == "opencv" {
            Self::process_direct_opencv(&sorted_video_list, &output_video_path, &config)
        } else if creation_mode == "direct" && use_ffmpeg_extraction {
            Self::process_direct_ffmpeg(
                &sorted_video_list,
                &output_video_path,
                &config,
                &output_base,
                &dir_tag,
                temp_dirs_created,
            )
        } else if creation_mode == "skip" || creation_mode == "none" {
            // Extraction only mode
            Self::process_extraction_only(&sorted_video_list, &config, &output_base, &dir_tag)
        } else {
            // Default temp frames mode
            Self::process_temp_frames(
                &sorted_video_list,
                &output_video_path,
                &config,
                &output_base,
                &dir_tag,
                temp_dirs_created,
            )
        }
    }

    /// Process using direct OpenCV method (memory-efficient)
    /// NOTE: This specific optimization is kept here as it tightly integrates capture and writer
    /// without intermediate files, which is different from strict "workers" pattern.
    fn process_direct_opencv(
        video_list: &[PathBuf],
        output_video_path: &PathBuf,
        config: &VideoExtractionConfig,
    ) -> Result<(), ProcessError> {
        println!("Using memory-efficient direct OpenCV processing.");
        let mut output_writer: Option<videoio::VideoWriter> = None;
        let mut output_frame_size: Option<Size> = None;
        let mut videos_processed_count = 0;

        for (video_index, video_path) in video_list.iter().enumerate() {
            println!(
                "  Thread {:?} processing video {}/{}: {}",
                thread::current().id(),
                video_index + 1,
                video_list.len(),
                video_path.display()
            );

            let mut cap = match HardwareAcceleratedCapture::create_capture(
                video_path.to_str().unwrap(),
                &config.hardware_acceleration,
            ) {
                Ok(cap) => cap,
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to create capture for video {}, skipping: {}",
                        video_path.display(),
                        e
                    );
                    continue;
                }
            };

            if !cap
                .is_opened()
                .map_err(|e| ProcessError::ProcessingFailed(format!("OpenCV error: {}", e)))?
            {
                eprintln!(
                    "Warning: OpenCV failed to open video {}, skipping.",
                    video_path.display()
                );
                continue;
            }

            // Initialize writer from first valid video
            if output_writer.is_none() {
                let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH).map_err(|e| {
                    ProcessError::ProcessingFailed(format!("Failed to get frame width: {}", e))
                })? as i32;
                let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT).map_err(|e| {
                    ProcessError::ProcessingFailed(format!("Failed to get frame height: {}", e))
                })? as i32;

                if width > 0 && height > 0 {
                    let size = Size::new(width, height);
                    println!(
                        "Determined output frame size {:?} from video {}",
                        size,
                        video_path.display()
                    );
                    output_frame_size = Some(size);

                    let fourcc = videoio::VideoWriter::fourcc('a', 'v', 'c', '1').map_err(|e| {
                        ProcessError::ProcessingFailed(format!("Failed to create fourcc: {}", e))
                    })?;
                    let writer = videoio::VideoWriter::new(
                        output_video_path.to_str().unwrap(),
                        fourcc,
                        config.output_fps as f64,
                        size,
                        true,
                    )
                    .map_err(|e| {
                        ProcessError::ProcessingFailed(format!(
                            "Failed to create VideoWriter: {}",
                            e
                        ))
                    })?;

                    if !writer.is_opened().map_err(|e| {
                        ProcessError::ProcessingFailed(format!("VideoWriter error: {}", e))
                    })? {
                        return Err(ProcessError::ProcessingFailed(format!(
                            "Failed to open VideoWriter for output file {}",
                            output_video_path.display()
                        )));
                    }
                    output_writer = Some(writer);
                } else {
                    eprintln!(
                        "Warning: Could not get valid frame size from video {}, trying next video.",
                        video_path.display()
                    );
                    continue;
                }
            }

            // Process frames
            if let Some(ref mut writer) = output_writer {
                let expected_size = output_frame_size.unwrap();
                let total_frames_cv = cap.get(videoio::CAP_PROP_FRAME_COUNT).map_err(|e| {
                    ProcessError::ProcessingFailed(format!("Failed to get frame count: {}", e))
                })? as usize;

                for frame_number in (0..total_frames_cv).step_by(config.frame_interval) {
                    let mut frame = Mat::default();
                    if !cap
                        .set(videoio::CAP_PROP_POS_FRAMES, frame_number as f64)
                        .map_err(|e| {
                            ProcessError::ProcessingFailed(format!("Failed to seek frame: {}", e))
                        })?
                    {
                        eprintln!(
                            "Warning: OpenCV failed to seek to frame {} in {}. Skipping frame.",
                            frame_number,
                            video_path.display()
                        );
                        continue;
                    }

                    if cap.read(&mut frame).map_err(|e| {
                        ProcessError::ProcessingFailed(format!("Failed to read frame: {}", e))
                    })? {
                        if frame.empty() {
                            eprintln!(
                                "Warning: OpenCV read empty frame at index {} from {}. Skipping frame.",
                                frame_number,
                                video_path.display()
                            );
                            continue;
                        }

                        // Check frame size
                        if frame.size().map_err(|e| {
                            ProcessError::ProcessingFailed(format!(
                                "Failed to get frame size: {}",
                                e
                            ))
                        })? != expected_size
                        {
                            eprintln!(
                                "Warning: Frame {} size does not match writer size in video {}. Skipping frame.",
                                frame_number,
                                video_path.display()
                            );
                            continue;
                        }

                        // Write the frame
                        if let Err(e) = writer.write(&frame) {
                            eprintln!(
                                "Error writing frame {} from video {}: {}. Aborting directory.",
                                frame_number,
                                video_path.display(),
                                e
                            );
                            let _ = writer.release();
                            return Err(ProcessError::ProcessingFailed(format!(
                                "VideoWriter write error: {}",
                                e
                            )));
                        }
                    } else {
                        println!(
                            "Finished reading frames or encountered read error for video {}",
                            video_path.display()
                        );
                        break;
                    }
                }
                videos_processed_count += 1;
            }
        }

        // Release writer
        if let Some(mut writer) = output_writer {
            println!("Releasing VideoWriter for {}", output_video_path.display());
            writer.release().map_err(|e| {
                ProcessError::ProcessingFailed(format!("Failed to release writer: {}", e))
            })?;
            if videos_processed_count == 0 {
                println!(
                    "No videos successfully processed to create output file {}",
                    output_video_path.display()
                );
                let _ = fs::remove_file(&output_video_path);
            } else {
                println!(
                    "Successfully created video (direct/opencv): {}",
                    output_video_path.display()
                );
            }
        } else {
            println!("VideoWriter was never initialized. No output file created.");
        }

        Ok(())
    }

    /// Process using direct FFmpeg method (Extract -> Join)
    fn process_direct_ffmpeg(
        video_list: &[PathBuf],
        output_video_path: &PathBuf,
        config: &VideoExtractionConfig,
        output_base: &PathBuf,
        dir_tag: &str,
        temp_dirs_created: Arc<Mutex<Vec<PathBuf>>>,
    ) -> Result<(), ProcessError> {
        println!("Using ffmpeg extraction with direct creation.");

        let dir_name = format!(
            "{}_{}_ffmpeg_direct_temp_{:?}",
            config.output_prefix,
            dir_tag,
            thread::current().id()
        );
        let temp_path = output_base.join(dir_name);
        fs::create_dir_all(&temp_path).map_err(|e| {
            ProcessError::IoError(format!("Failed to create temp directory: {}", e))
        })?;
        temp_dirs_created.lock().unwrap().push(temp_path.clone());
        println!(
            "Created transient temp directory for ffmpeg: {}",
            temp_path.display()
        );

        // Use Extractor Worker
        for (video_index, video_path) in video_list.iter().enumerate() {
            println!(
                "  Thread {:?} extracting via ffmpeg from video {}/{}: {}",
                thread::current().id(),
                video_index + 1,
                video_list.len(),
                video_path.display()
            );
            FrameExtractor::extract_frames_ffmpeg(
                video_path.to_str().unwrap(),
                video_index,
                temp_path.to_str().unwrap(),
                config.frame_interval,
            )?;
        }

        // Use Creator Worker
        VideoCreator::create_video_from_temp_frames(
            temp_path.to_str().unwrap(),
            output_video_path,
            config.output_fps,
        )
    }

    /// Process using temp frames method
    fn process_temp_frames(
        video_list: &[PathBuf],
        output_video_path: &PathBuf,
        config: &VideoExtractionConfig,
        output_base: &PathBuf,
        dir_tag: &str,
        temp_dirs_created: Arc<Mutex<Vec<PathBuf>>>,
    ) -> Result<(), ProcessError> {
        println!("Using temp frames approach.");

        let dir_name = format!(
            "{}_{}_temp_{:?}",
            config.output_prefix,
            dir_tag,
            thread::current().id()
        );
        let temp_path = output_base.join(dir_name);
        fs::create_dir_all(&temp_path).map_err(|e| {
            ProcessError::IoError(format!("Failed to create temp directory: {}", e))
        })?;
        temp_dirs_created.lock().unwrap().push(temp_path.clone());
        println!(
            "Created transient temp directory for frames: {}",
            temp_path.display()
        );

        // Use Extractor Worker
        for (video_index, video_path) in video_list.iter().enumerate() {
            println!(
                "  Thread {:?} extracting from video {}/{}: {}",
                thread::current().id(),
                video_index + 1,
                video_list.len(),
                video_path.display()
            );
            if config.extraction_mode == "ffmpeg" {
                FrameExtractor::extract_frames_ffmpeg(
                    video_path.to_str().unwrap(),
                    video_index,
                    temp_path.to_str().unwrap(),
                    config.frame_interval,
                )?;
            } else {
                FrameExtractor::extract_frames_opencv(
                    video_path.to_str().unwrap(),
                    video_index,
                    temp_path.to_str().unwrap(),
                    config.frame_interval,
                    &config.hardware_acceleration,
                )?;
            }
        }

        // Use Creator Worker
        VideoCreator::create_video_from_temp_frames(
            temp_path.to_str().unwrap(),
            output_video_path,
            config.output_fps,
        )
    }

    /// Process using extraction only (no video creation)
    fn process_extraction_only(
        video_list: &[PathBuf],
        config: &VideoExtractionConfig,
        output_base: &PathBuf,
        dir_tag: &str,
    ) -> Result<(), ProcessError> {
        println!("Using extraction only mode (no video creation).");

        // Use output directory directly, do NOT add to temp_dirs_created
        let dir_name = format!("{}_{}_frames", config.output_prefix, dir_tag);
        let output_path = output_base.join(dir_name);

        fs::create_dir_all(&output_path).map_err(|e| {
            ProcessError::IoError(format!("Failed to create output directory: {}", e))
        })?;

        println!(
            "Extracting frames to persistent directory: {}",
            output_path.display()
        );

        // Use Extractor Worker
        for (video_index, video_path) in video_list.iter().enumerate() {
            if config.extraction_mode == "ffmpeg" {
                FrameExtractor::extract_frames_ffmpeg(
                    video_path.to_str().unwrap(),
                    video_index,
                    output_path.to_str().unwrap(),
                    config.frame_interval,
                )?;
            } else {
                FrameExtractor::extract_frames_opencv(
                    video_path.to_str().unwrap(),
                    video_index,
                    output_path.to_str().unwrap(),
                    config.frame_interval,
                    &config.hardware_acceleration,
                )?;
            }
        }

        Ok(())
    }
}
