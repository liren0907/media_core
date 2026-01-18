use crate::annotation::overlay::add_text_overlay_with_position;
use crate::annotation::types::{AnnotationConfig, AnnotationType, DataSource, format_timestamp};
use opencv::{core::Vector, imgcodecs, prelude::*};
use regex::Regex;
use std::fs;
use std::path::Path;

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
            DataSource::Video(path) => self.process_video_file(path),
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

    fn process_video_file(
        &self,
        input_path: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut cap =
            opencv::videoio::VideoCapture::from_file(input_path, opencv::videoio::CAP_ANY)?;
        if !cap.is_opened()? {
            return Err(format!("Failed to open video file: {}", input_path).into());
        }

        let fps = if let Some(source_fps) = self.config.source_fps {
            source_fps
        } else {
            cap.get(opencv::videoio::CAP_PROP_FPS)?
        };

        let width = cap.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)? as i32;
        let height = cap.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
        let size = opencv::core::Size::new(width, height);

        // Initialize VideoWriter
        let output_fps = if let Some(video_config) = &self.config.video_encoding {
            video_config.fps as f64
        } else {
            fps
        };

        let fourcc = opencv::videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;
        let mut writer = opencv::videoio::VideoWriter::new(
            &self.config.output_path,
            fourcc,
            output_fps,
            size,
            true,
        )?;

        if !writer.is_opened()? {
            return Err(
                format!("Failed to open VideoWriter for {}", self.config.output_path).into(),
            );
        }

        let mut frame = opencv::prelude::Mat::default();
        let mut frame_count = 0;

        loop {
            if !cap.read(&mut frame)? || frame.empty() {
                break;
            }

            let annotation_text = match &self.config.annotation_type {
                AnnotationType::Filename => Path::new(input_path)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                AnnotationType::Timestamp => format_timestamp(frame_count, fps),
                AnnotationType::Custom(text) => text.clone(),
            };

            add_text_overlay_with_position(
                &mut frame,
                &annotation_text,
                self.config.text_position.clone(),
            )?;

            writer.write(&frame)?;
            frame_count += 1;
        }

        println!("Successfully created video: {}", self.config.output_path);
        Ok(())
    }

    fn process_video_frames(
        &self,
        frames_dir: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output_dir = Path::new(frames_dir);

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

        if image_files.is_empty() {
            return Ok(());
        }

        let frame_regex = Regex::new(r"(\d+)")?;
        let fps = self.config.source_fps.unwrap_or(30.0);

        // Initialize VideoWriter if video encoding is enabled
        let mut video_writer = if let Some(video_config) = &self.config.video_encoding {
            // Read the first frame to determine size
            let first_frame_path = image_files[0].path();
            let first_frame =
                imgcodecs::imread(first_frame_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
            let size = opencv::core::Size::new(first_frame.cols(), first_frame.rows());

            // Use 'mp4v' or 'avc1' for MP4. 'mp4v' is generally safe for OpenCV's default backend on most systems.
            // On macOS, it might use AVFoundation.
            let fourcc = opencv::videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;

            let writer = opencv::videoio::VideoWriter::new(
                &self.config.output_path,
                fourcc, // apiPreference (0 = auto)
                video_config.fps as f64,
                size,
                true, // isColor
            )?;

            if !writer.is_opened()? {
                return Err(
                    format!("Failed to open VideoWriter for {}", self.config.output_path).into(),
                );
            }

            Some(writer)
        } else {
            None
        };

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

            // Write to video if writer exists
            if let Some(writer) = &mut video_writer {
                writer.write(&img)?;
            }
        }

        if video_writer.is_some() {
            println!("Successfully created video: {}", self.config.output_path);
        }

        Ok(())
    }
}
