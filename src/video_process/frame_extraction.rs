use opencv::{
    core::Vector,
    imgcodecs,
    prelude::*,
    videoio::{self, CAP_PROP_FRAME_COUNT, VideoCapture},
};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::video_process::helpers::get_output_path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtractionMode {
    OpenCVSequential,
    OpenCVInterval,
    FFmpeg,
    FFmpegInterval,
    Parallel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaveMode {
    SingleDirectory,
    MultipleDirectory,
}

impl Default for ExtractionMode {
    fn default() -> Self {
        ExtractionMode::OpenCVInterval
    }
}

impl Default for SaveMode {
    fn default() -> Self {
        SaveMode::MultipleDirectory
    }
}

pub struct FrameExtractor {
    video_path: String,
    output_dir: String,
    frame_interval: usize,
    extraction_mode: ExtractionMode,
    save_mode: SaveMode,
}

impl FrameExtractor {
    pub fn new(video_path: &str, output_dir: &str) -> Self {
        Self {
            video_path: video_path.to_string(),
            output_dir: output_dir.to_string(),
            frame_interval: 1,
            extraction_mode: ExtractionMode::default(),
            save_mode: SaveMode::default(),
        }
    }

    pub fn with_interval(mut self, interval: usize) -> Self {
        self.frame_interval = interval.max(1);
        self
    }

    pub fn with_mode(mut self, mode: ExtractionMode) -> Self {
        self.extraction_mode = mode;
        self
    }

    pub fn with_save_mode(mut self, mode: SaveMode) -> Self {
        self.save_mode = mode;
        self
    }

    pub fn video_path(&self) -> &str {
        &self.video_path
    }

    pub fn output_dir(&self) -> &str {
        &self.output_dir
    }

    pub fn frame_interval(&self) -> usize {
        self.frame_interval
    }

    pub fn extraction_mode(&self) -> ExtractionMode {
        self.extraction_mode
    }

    pub fn save_mode(&self) -> SaveMode {
        self.save_mode
    }

    pub fn extract(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match self.extraction_mode {
            ExtractionMode::OpenCVSequential => self.extract_opencv_sequential(),
            ExtractionMode::OpenCVInterval => self.extract_opencv_interval(),
            ExtractionMode::FFmpeg => self.extract_ffmpeg_all(),
            ExtractionMode::FFmpegInterval => self.extract_ffmpeg_interval(),
            ExtractionMode::Parallel => self.extract_parallel(),
        }
    }

    fn get_save_mode_str(&self) -> &str {
        match self.save_mode {
            SaveMode::SingleDirectory => "single_directory",
            SaveMode::MultipleDirectory => "multiple_directory",
        }
    }

    fn extract_opencv_sequential(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut cap = VideoCapture::from_file(&self.video_path, videoio::CAP_ANY.into())?;
        let total_frames = cap.get(CAP_PROP_FRAME_COUNT)? as usize;

        fs::create_dir_all(&self.output_dir)?;

        for frame_number in 0..total_frames {
            let mut frame = Mat::default();
            if cap.read(&mut frame)? {
                let output_path = get_output_path(
                    &self.output_dir,
                    &self.video_path,
                    frame_number,
                    self.get_save_mode_str(),
                )?;
                imgcodecs::imwrite(&output_path, &frame, &Vector::new())?;
                println!("Saved frame {} of {}", frame_number + 1, total_frames);
            }
        }
        Ok(())
    }

    fn extract_opencv_interval(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut cap = VideoCapture::from_file(&self.video_path, videoio::CAP_ANY.into())?;
        let total_frames = cap.get(CAP_PROP_FRAME_COUNT)? as usize;

        fs::create_dir_all(&self.output_dir)?;

        for frame_number in (0..total_frames).step_by(self.frame_interval) {
            let mut frame = Mat::default();
            cap.set(videoio::CAP_PROP_POS_FRAMES, frame_number as f64)?;

            if cap.read(&mut frame)? {
                let output_path = get_output_path(
                    &self.output_dir,
                    &self.video_path,
                    frame_number,
                    self.get_save_mode_str(),
                )?;
                imgcodecs::imwrite(&output_path, &frame, &Vector::new())?;
                println!("Saved frame {} of {}", frame_number + 1, total_frames);
            }
        }
        Ok(())
    }

    fn extract_ffmpeg_all(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        fs::create_dir_all(&self.output_dir)?;

        let output_pattern = self.get_ffmpeg_output_pattern("%04d");

        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&self.video_path)
            .arg(&output_pattern)
            .output()?;

        if !output.status.success() {
            return Err(
                format!("ffmpeg failed: {}", String::from_utf8_lossy(&output.stderr)).into(),
            );
        }
        Ok(())
    }

    fn extract_ffmpeg_interval(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        fs::create_dir_all(&self.output_dir)?;

        let output_pattern = self.get_ffmpeg_output_pattern("%d");

        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(&self.video_path)
            .arg("-vf")
            .arg(format!("select=not(mod(n\\,{}))", self.frame_interval))
            .arg("-vsync")
            .arg("0")
            .arg("-frame_pts")
            .arg("1")
            .arg("-start_number")
            .arg("0")
            .arg(&output_pattern)
            .output()?;

        if !output.status.success() {
            return Err(
                format!("ffmpeg failed: {}", String::from_utf8_lossy(&output.stderr)).into(),
            );
        }
        Ok(())
    }

    fn extract_parallel(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let cap = VideoCapture::from_file(&self.video_path, videoio::CAP_ANY.into())?;
        let total_frames = cap.get(CAP_PROP_FRAME_COUNT)? as usize;

        let frame_indices: Vec<usize> = (0..total_frames).step_by(self.frame_interval).collect();
        let video_path = self.video_path.clone();
        let output_dir = self.output_dir.clone();
        let save_mode = self.get_save_mode_str().to_string();

        frame_indices.par_iter().try_for_each(|&frame_index| {
            let mut cap = VideoCapture::from_file(&video_path, videoio::CAP_ANY.into())?;
            let mut frame = Mat::default();

            cap.set(videoio::CAP_PROP_POS_FRAMES, frame_index as f64)?;
            if cap.read(&mut frame)? {
                let output_path =
                    get_output_path(&output_dir, &video_path, frame_index, &save_mode)?;
                imgcodecs::imwrite(&output_path, &frame, &Vector::new())?;
                println!("Saved frame {} from {}", frame_index, video_path);
            }
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })?;

        Ok(())
    }

    fn get_ffmpeg_output_pattern(&self, frame_format: &str) -> String {
        let video_name = Path::new(&self.video_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        match self.save_mode {
            SaveMode::SingleDirectory => {
                format!("{}/{}_{}.jpg", self.output_dir, video_name, frame_format)
            }
            SaveMode::MultipleDirectory => {
                let video_output_dir = Path::new(&self.output_dir).join(video_name);
                let _ = fs::create_dir_all(&video_output_dir);
                format!(
                    "{}/frame_{}.jpg",
                    video_output_dir.to_str().unwrap(),
                    frame_format
                )
            }
        }
    }
}
