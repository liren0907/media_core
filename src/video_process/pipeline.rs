//! Video Process Pipeline Steps
//!
//! This module provides pipeline steps for extracting frames to disk.
//! Unlike the `streaming` module which stores frames in memory,
//! this module saves frames directly to the filesystem.

use crate::pipeline::{MediaContext, PipelineError, PipelineStep};
use crate::video_process::{ExtractionMode, FrameExtractor, SaveMode};
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// VIDEO PROCESS RESULT TYPE
// ============================================================================

/// Result of a video process operation
#[derive(Debug, Clone, Default)]
pub struct VideoProcessResult {
    /// Output directory where frames were saved
    pub output_dir: String,
    /// Number of frames extracted
    pub frames_extracted: usize,
    /// Extraction mode used
    pub extraction_mode: String,
    /// Save mode used
    pub save_mode: String,
    /// Whether extraction was successful
    pub success: bool,
}

// ============================================================================
// EXTRACT FRAMES TO DISK STEP
// ============================================================================

/// A pipeline step that extracts frames from a video and saves them to disk.
///
/// This differs from `streaming::ExtractFrames` which keeps frames in memory.
/// Use this step when you need to persist frames to the filesystem.
///
/// # Example
/// ```rust
/// let step = ExtractFramesToDisk::new("output/frames")
///     .interval(30)
///     .mode(ExtractionMode::Parallel)
///     .save_mode(SaveMode::SingleDirectory);
/// ```
pub struct ExtractFramesToDisk {
    output_dir: PathBuf,
    interval: usize,
    extraction_mode: ExtractionMode,
    save_mode: SaveMode,
}

impl ExtractFramesToDisk {
    /// Create a new ExtractFramesToDisk step with the specified output directory.
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
            interval: 30,
            extraction_mode: ExtractionMode::default(),
            save_mode: SaveMode::default(),
        }
    }

    /// Set the frame extraction interval (default: 30 = every 30th frame)
    pub fn interval(mut self, interval: usize) -> Self {
        self.interval = interval.max(1);
        self
    }

    /// Set the extraction mode
    pub fn mode(mut self, mode: ExtractionMode) -> Self {
        self.extraction_mode = mode;
        self
    }

    /// Set the save mode (SingleDirectory or MultipleDirectory)
    pub fn save_mode(mut self, mode: SaveMode) -> Self {
        self.save_mode = mode;
        self
    }
}

impl PipelineStep<MediaContext> for ExtractFramesToDisk {
    fn name(&self) -> &str {
        "ExtractFramesToDisk"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let input_path = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError(
                "ExtractFramesToDisk requires a File source".to_string(),
            )
        })?;

        // Create output directory
        fs::create_dir_all(&self.output_dir).map_err(|e| PipelineError::StepFailed {
            step_name: self.name().to_string(),
            error: format!("Failed to create output directory: {}", e),
        })?;

        // Run frame extraction
        let extractor = FrameExtractor::new(input_path, self.output_dir.to_str().unwrap())
            .with_interval(self.interval)
            .with_mode(self.extraction_mode)
            .with_save_mode(self.save_mode);

        extractor.extract().map_err(|e| PipelineError::StepFailed {
            step_name: self.name().to_string(),
            error: format!("Frame extraction failed: {}", e),
        })?;

        // Count extracted frames
        let frames_extracted = fs::read_dir(&self.output_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| {
                        e.path()
                            .extension()
                            .map(|ext| ext == "jpg" || ext == "png")
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0);

        // Store result in context
        context.video_process_result = Some(VideoProcessResult {
            output_dir: self.output_dir.to_string_lossy().to_string(),
            frames_extracted,
            extraction_mode: format!("{:?}", self.extraction_mode),
            save_mode: format!("{:?}", self.save_mode),
            success: true,
        });

        Ok(())
    }
}
