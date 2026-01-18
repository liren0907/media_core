//! Process Pipeline Steps
//!
//! This module provides pipeline steps for file processing operations.

use crate::pipeline::{MediaContext, PipelineError, PipelineStep};
use crate::process::{ProcessConfig, ProcessingMode, Processor};
use std::path::{Path, PathBuf};

// ============================================================================
// PROCESS FILES RESULT TYPE
// ============================================================================

/// Result of a file processing operation
#[derive(Debug, Clone, Default)]
pub struct ProcessFilesResult {
    /// Number of files successfully processed
    pub files_processed: u64,
    /// Number of files that failed
    pub files_failed: u64,
    /// Total size of processed files in bytes
    pub total_size_bytes: u64,
    /// Output directory
    pub output_dir: String,
    /// Processing mode used
    pub processing_mode: String,
    /// Whether processing was successful
    pub success: bool,
}

// ============================================================================
// PROCESS FILES STEP
// ============================================================================

/// A pipeline step that processes files from input to output.
///
/// This step wraps the `Processor` for use in the pipeline pattern.
///
/// # Example
/// ```rust
/// let step = ProcessFiles::new("output/processed")
///     .mode(ProcessingMode::DirectoryProcess)
///     .overwrite(true);
/// ```
pub struct ProcessFiles {
    output_dir: PathBuf,
    processing_mode: ProcessingMode,
    overwrite_existing: bool,
}

impl ProcessFiles {
    /// Create a new ProcessFiles step with the specified output directory.
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
            processing_mode: ProcessingMode::SingleFile,
            overwrite_existing: false,
        }
    }

    /// Set the processing mode
    pub fn mode(mut self, mode: ProcessingMode) -> Self {
        self.processing_mode = mode;
        self
    }

    /// Set whether to overwrite existing files
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite_existing = overwrite;
        self
    }
}

impl PipelineStep<MediaContext> for ProcessFiles {
    fn name(&self) -> &str {
        "ProcessFiles"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let input_path = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError("ProcessFiles requires a File source".to_string())
        })?;

        // Build config
        let config = ProcessConfig {
            input_path: input_path.to_string(),
            output_path: self.output_dir.to_string_lossy().to_string(),
            processing_mode: self.processing_mode.clone(),
            ..Default::default()
        };

        // Create processor
        let mut processor = Processor::new(config).map_err(|e| PipelineError::StepFailed {
            step_name: self.name().to_string(),
            error: format!("Failed to create processor: {}", e),
        })?;

        // Run processing
        processor
            .process_from_source(input_path, self.output_dir.to_str().unwrap())
            .map_err(|e| PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: format!("Processing failed: {}", e),
            })?;

        // Get stats
        let stats = processor.get_stats();

        // Store result in context
        context.process_result = Some(ProcessFilesResult {
            files_processed: stats.files_processed,
            files_failed: stats.files_failed,
            total_size_bytes: stats.total_size_processed,
            output_dir: self.output_dir.to_string_lossy().to_string(),
            processing_mode: format!("{:?}", self.processing_mode),
            success: stats.files_failed == 0,
        });

        Ok(())
    }
}
