use crate::process::hw_accel::HardwareAcceleratedCapture;
use crate::process::types::ProcessError;
use opencv::{
    core::{Mat, Vector},
    imgcodecs,
    prelude::*,
    videoio,
};
use std::fs;
use std::path::Path;
use std::process::Command;

pub struct FrameExtractor;

impl FrameExtractor {
    /// Extract frames using OpenCV
    pub fn extract_frames_opencv(
        video_filename: &str,
        video_index: usize,
        temp_frame_dir: &str,
        frame_interval: usize,
        hw_config: &crate::process::hw_accel::HardwareAccelConfig,
    ) -> Result<(), ProcessError> {
        fs::create_dir_all(temp_frame_dir).map_err(|e| {
            ProcessError::IoError(format!("Failed to create temp frame directory: {}", e))
        })?;

        let mut cap = HardwareAcceleratedCapture::create_capture(video_filename, hw_config)
            .map_err(|e| {
                ProcessError::ProcessingFailed(format!(
                    "Failed to open video {}: {}",
                    video_filename, e
                ))
            })?;

        if !cap
            .is_opened()
            .map_err(|e| ProcessError::ProcessingFailed(format!("OpenCV error: {}", e)))?
        {
            return Err(ProcessError::ProcessingFailed(format!(
                "Failed to open video: {}",
                video_filename
            )));
        }

        let total_frames = cap.get(videoio::CAP_PROP_FRAME_COUNT).map_err(|e| {
            ProcessError::ProcessingFailed(format!("Failed to get frame count: {}", e))
        })? as usize;

        for frame_number in (0..total_frames).step_by(frame_interval) {
            let mut frame = Mat::default();
            if !cap
                .set(videoio::CAP_PROP_POS_FRAMES, frame_number as f64)
                .map_err(|e| {
                    ProcessError::ProcessingFailed(format!("Failed to seek frame: {}", e))
                })?
            {
                eprintln!(
                    "Warning: Failed to seek to frame {} in {}",
                    frame_number, video_filename
                );
                continue;
            }

            if cap.read(&mut frame).map_err(|e| {
                ProcessError::ProcessingFailed(format!("Failed to read frame: {}", e))
            })? {
                if frame.empty() {
                    eprintln!(
                        "Warning: Read empty frame at index {} from {}",
                        frame_number, video_filename
                    );
                    continue;
                }
                let output_path = format!(
                    "{}/video{:03}_frame{:07}.jpg",
                    temp_frame_dir, video_index, frame_number
                );
                imgcodecs::imwrite(&output_path, &frame, &Vector::new()).map_err(|e| {
                    ProcessError::ProcessingFailed(format!("Failed to write frame: {}", e))
                })?;
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Extract frames using FFmpeg
    pub fn extract_frames_ffmpeg(
        video_filename: &str,
        video_index: usize,
        temp_frame_dir: &str,
        frame_interval: usize,
    ) -> Result<(), ProcessError> {
        fs::create_dir_all(temp_frame_dir).map_err(|e| {
            ProcessError::IoError(format!("Failed to create temp frame directory: {}", e))
        })?;

        if frame_interval == 0 {
            return Err(ProcessError::ValidationError(
                "frame_interval must be greater than 0 for ffmpeg extraction.".to_string(),
            ));
        }

        let output_pattern =
            Path::new(temp_frame_dir).join(format!("video{}_frame%06d.jpg", video_index));
        let output_pattern_str = output_pattern.to_str().ok_or_else(|| {
            ProcessError::ProcessingFailed("Invalid output path pattern".to_string())
        })?;

        let mut cmd = Command::new("ffmpeg");
        cmd.arg("-i")
            .arg(video_filename)
            .arg("-vf")
            .arg(format!("select=not(mod(n\\,{}))", frame_interval))
            .arg("-vsync")
            .arg("vfr")
            .arg("-q:v")
            .arg("2")
            .arg(output_pattern_str)
            .arg("-hide_banner")
            .arg("-loglevel")
            .arg("warning");

        println!(
            "Running ffmpeg frame extraction for video {}: {}",
            video_index, video_filename
        );

        let output = cmd.output().map_err(|e| {
            ProcessError::ProcessingFailed(format!("Failed to execute ffmpeg: {}", e))
        })?;

        if !output.status.success() {
            eprintln!("ffmpeg stdout: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("ffmpeg stderr: {}", String::from_utf8_lossy(&output.stderr));
            return Err(ProcessError::ProcessingFailed(format!(
                "ffmpeg frame extraction failed for video {}",
                video_filename
            )));
        }

        println!(
            "Successfully extracted frames using ffmpeg for video {}: {}",
            video_index, video_filename
        );
        Ok(())
    }
}
