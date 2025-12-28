use opencv::{
    core::{Mat, Size},
    imgproc,
    prelude::*,
    videoio,
};

use crate::streaming::helpers::{get_video_capture, mat_to_base64_jpeg};
use crate::streaming::strategy::SamplingStrategy;
use crate::streaming::types::FrameData;

// StreamExtractor Struct
// ============================================

use opencv::videoio::VideoCapture;

/// Defines the extraction mode for reading frames.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtractionMode {
    /// Seek to each frame (default). Best for sparse sampling.
    Seek,
    /// Read frames sequentially. Best for continuous reading.
    Sequential,
}

pub struct StreamExtractor {
    video_path: String,
    cap: VideoCapture,
    total_frames: usize,
    strategy: SamplingStrategy,
    mode: ExtractionMode,
}

impl StreamExtractor {
    /// Create a new StreamExtractor for a video file with an optional sampling strategy.
    /// If no strategy is provided, it defaults to processing every frame.
    /// Default extraction mode is Seek.
    pub fn new(video_path: &str, strategy: Option<SamplingStrategy>) -> Result<Self, String> {
        let cap = get_video_capture(video_path)?;
        let total_frames = cap.get(videoio::CAP_PROP_FRAME_COUNT).unwrap_or(0.0) as usize;

        Ok(Self {
            video_path: video_path.to_string(),
            cap,
            total_frames,
            strategy: strategy.unwrap_or_default(),
            mode: ExtractionMode::Seek,
        })
    }

    /// Get the video file path.
    pub fn video_path(&self) -> &str {
        &self.video_path
    }

    /// Get total frame count.
    pub fn total_frames(&self) -> usize {
        self.total_frames
    }

    /// Update the sampling strategy.
    pub fn set_strategy(&mut self, strategy: SamplingStrategy) {
        self.strategy = strategy;
    }

    /// Set the extraction mode.
    pub fn set_mode(&mut self, mode: ExtractionMode) {
        self.mode = mode;
    }

    /// Extract frames using the stored sampling strategy with optional scale factor.
    pub fn extract(&mut self, scale_factor: Option<f64>) -> Result<Vec<FrameData>, String> {
        match self.mode {
            ExtractionMode::Seek => self.extract_seek(scale_factor),
            ExtractionMode::Sequential => self.extract_sequential(scale_factor),
        }
    }

    fn extract_seek(&mut self, scale_factor: Option<f64>) -> Result<Vec<FrameData>, String> {
        let frame_indices = self.strategy.get_frame_indices(self.total_frames);
        let mut frames = Vec::new();
        let mut frame = Mat::default();

        for frame_index in frame_indices {
            self.cap
                .set(videoio::CAP_PROP_POS_FRAMES, frame_index as f64)
                .map_err(|e| format!("Failed to seek to frame {}: {}", frame_index, e))?;

            if self.cap.read(&mut frame).unwrap_or(false) && !frame.empty() {
                self.process_and_add_frame(&frame, frame_index, scale_factor, &mut frames)?;
            }
        }
        Ok(frames)
    }

    fn extract_sequential(&mut self, scale_factor: Option<f64>) -> Result<Vec<FrameData>, String> {
        let target_indices = self.strategy.get_frame_indices(self.total_frames);
        // Optimization: Convert to HashSet for O(1) lookup if indices are sparse?
        // For now, vector lookup is okay for small sets, but sorting helps.
        // Assuming indices are sorted from strategy.

        let mut frames = Vec::new();
        let mut frame = Mat::default();
        let mut current_frame_idx = 0;

        // Reset to beginning
        self.cap
            .set(videoio::CAP_PROP_POS_FRAMES, 0.0)
            .map_err(|e| format!("Failed to seek to start: {}", e))?;

        // Simple sequential read
        while self.cap.read(&mut frame).unwrap_or(false) {
            if frame.empty() {
                break;
            }

            if target_indices.contains(&current_frame_idx) {
                self.process_and_add_frame(&frame, current_frame_idx, scale_factor, &mut frames)?;
            }

            // Optimization: Stop if we passed the last target index
            if let Some(&last) = target_indices.last() {
                if current_frame_idx >= last {
                    break;
                }
            }

            current_frame_idx += 1;
        }

        Ok(frames)
    }

    fn process_and_add_frame(
        &self,
        frame: &Mat,
        index: usize,
        scale_factor: Option<f64>,
        frames: &mut Vec<FrameData>,
    ) -> Result<(), String> {
        let processed_frame = if let Some(factor) = scale_factor {
            if factor != 1.0 {
                let mut resized = Mat::default();
                let new_size = Size::new(
                    (frame.cols() as f64 * factor) as i32,
                    (frame.rows() as f64 * factor) as i32,
                );
                imgproc::resize(
                    &frame,
                    &mut resized,
                    new_size,
                    0.0,
                    0.0,
                    imgproc::INTER_AREA,
                )
                .map_err(|e| e.to_string())?;
                resized
            } else {
                frame.clone()
            }
        } else {
            frame.clone()
        };

        let frame_data = mat_to_base64_jpeg(&processed_frame)?;
        frames.push(FrameData {
            index,
            data: frame_data,
        });
        Ok(())
    }
}
