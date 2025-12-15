//! Core processor functionality for file processing operations

use std::fs;
use std::path::Path;
use std::time::Duration;

use crate::process::config::ProcessConfig;
use crate::process::handlers::FileHandler;
use crate::process::stats::ProcessingStats;
use crate::process::types::{FileFormat, ProcessError, ProcessingMode};
use crate::process::validation::ProcessValidator;
use crate::process::video::VideoProcessor;

/// Main processor struct for handling process operations
pub struct Processor {
    config: ProcessConfig,
    stats: ProcessingStats,
}

impl Processor {
    /// Create a new processor with the given configuration
    pub fn new(config: ProcessConfig) -> Result<Self, ProcessError> {
        // Basic validation (only if validation is enabled)
        if config.processing_options.enable_validation {
            if config.input_path.is_empty() {
                return Err(ProcessError::InvalidInput(
                    "Input path cannot be empty".to_string(),
                ));
            }

            if config.output_path.is_empty() {
                return Err(ProcessError::InvalidInput(
                    "Output path cannot be empty".to_string(),
                ));
            }
        }

        // Validate processing mode compatibility
        ProcessValidator::validate_processing_mode(&config)?;

        Ok(Self {
            config,
            stats: ProcessingStats::new(),
        })
    }

    /// Process from source to destination
    pub fn process_from_source(
        &mut self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), ProcessError> {
        if self.config.processing_options.verbose_logging {
            println!("Starting process from {} to {}", input_path, output_path);
        }

        // Reset stats for new processing session
        self.stats = ProcessingStats::new();

        // Check input existence & output validity
        ProcessValidator::validate_input(&self.config, input_path)?;
        ProcessValidator::validate_output_path(&self.config, output_path)?;

        // Create output directory if needed
        if self.config.processing_options.create_output_directory {
            ProcessValidator::ensure_output_directory(output_path)?;
        }

        // Process based on mode
        match self.config.processing_mode {
            ProcessingMode::SingleFile => self.process_single_file(input_path, output_path)?,
            ProcessingMode::BatchFiles => self.process_batch_files(input_path, output_path)?,
            ProcessingMode::DirectoryProcess => self.process_directory(input_path, output_path)?,
            ProcessingMode::StreamProcess => self.process_stream_data(input_path, output_path)?,
        }

        // Finalize stats
        self.stats.finalize();

        if self.config.processing_options.verbose_logging {
            println!("Process completed successfully");
            println!("Files processed: {}", self.stats.files_processed);
            println!("Files failed: {}", self.stats.files_failed);
            println!("Success rate: {:.2}%", self.stats.success_rate());
            println!("Processing time: {:?}", self.stats.processing_time);
        }

        Ok(())
    }

    /// Process a single file
    fn process_single_file(
        &mut self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), ProcessError> {
        let input_file = Path::new(input_path);
        let output_file = Path::new(output_path);

        // Check file size limits
        if let Some(max_size_mb) = self.config.processing_options.max_file_size_mb {
            let file_size = fs::metadata(input_file)
                .map_err(|e| ProcessError::IoError(format!("Failed to get file metadata: {}", e)))?
                .len();

            let max_size_bytes = max_size_mb * 1024 * 1024;
            if file_size > max_size_bytes {
                return Err(ProcessError::ValidationError(format!(
                    "File size ({} bytes) exceeds maximum allowed size ({} MB)",
                    file_size, max_size_mb
                )));
            }
        }

        // Backup original if requested
        if self.config.processing_options.backup_original {
            self.backup_file(input_file)?;
        }

        // Determine file format and process accordingly
        let file_format = FileHandler::detect_file_format(input_file)?;

        // Use Handler to process
        FileHandler::process_file_by_format(
            input_file,
            output_file,
            &file_format,
            &self.config.processing_options,
        )?;

        // Update stats
        let file_size = fs::metadata(input_file)
            .map_err(|e| ProcessError::IoError(format!("Failed to get file size: {}", e)))?
            .len();
        self.stats.add_processed_file(file_size);

        Ok(())
    }

    /// Process multiple files in batch
    fn process_batch_files(
        &mut self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), ProcessError> {
        let input_dir = Path::new(input_path);
        let output_dir = Path::new(output_path);

        if !input_dir.is_dir() {
            return Err(ProcessError::InvalidInput(
                "Batch processing requires input directory".to_string(),
            ));
        }

        let entries = fs::read_dir(input_dir)
            .map_err(|e| ProcessError::IoError(format!("Failed to read directory: {}", e)))?;

        for entry in entries {
            let entry =
                entry.map_err(|e| ProcessError::IoError(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.is_file() {
                let file_name = path.file_name().ok_or_else(|| {
                    ProcessError::ProcessingFailed("Invalid file name".to_string())
                })?;
                let output_file = output_dir.join(file_name);

                match self.process_single_file(
                    path.to_str().unwrap_or(""),
                    output_file.to_str().unwrap_or(""),
                ) {
                    Ok(_) => {
                        if self.config.processing_options.verbose_logging {
                            println!("Successfully processed: {:?}", path);
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to process {:?}: {}", path, e);
                        self.stats.add_failed_file(error_msg.clone());
                        if self.config.processing_options.verbose_logging {
                            eprintln!("{}", error_msg);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Process entire directory recursively
    fn process_directory(
        &mut self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), ProcessError> {
        let input_dir = Path::new(input_path);
        let output_dir = Path::new(output_path);

        self.process_directory_recursive(input_dir, output_dir, input_dir)?;
        Ok(())
    }

    /// Recursive directory processing helper
    fn process_directory_recursive(
        &mut self,
        current_dir: &Path,
        output_base: &Path,
        input_base: &Path,
    ) -> Result<(), ProcessError> {
        let entries = fs::read_dir(current_dir)
            .map_err(|e| ProcessError::IoError(format!("Failed to read directory: {}", e)))?;

        for entry in entries {
            let entry =
                entry.map_err(|e| ProcessError::IoError(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively process subdirectories
                self.process_directory_recursive(&path, output_base, input_base)?;
            } else if path.is_file() {
                // Calculate relative path and create corresponding output path
                let relative_path = path.strip_prefix(input_base).map_err(|e| {
                    ProcessError::ProcessingFailed(format!(
                        "Failed to calculate relative path: {}",
                        e
                    ))
                })?;
                let output_file = output_base.join(relative_path);

                // Ensure output directory exists
                if let Some(parent) = output_file.parent() {
                    fs::create_dir_all(parent).map_err(|e| {
                        ProcessError::IoError(format!("Failed to create output directory: {}", e))
                    })?;
                }

                // Process the file
                match self.process_single_file(
                    path.to_str().unwrap_or(""),
                    output_file.to_str().unwrap_or(""),
                ) {
                    Ok(_) => {
                        if self.config.processing_options.verbose_logging {
                            println!("Successfully processed: {:?}", path);
                        }
                    }
                    Err(e) => {
                        let error_msg = format!("Failed to process {:?}: {}", path, e);
                        self.stats.add_failed_file(error_msg.clone());
                        if self.config.processing_options.verbose_logging {
                            eprintln!("{}", error_msg);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Process stream data (placeholder for stream processing)
    fn process_stream_data(
        &mut self,
        input_path: &str,
        output_path: &str,
    ) -> Result<(), ProcessError> {
        if self.config.processing_options.verbose_logging {
            println!(
                "Processing stream data from {} to {}",
                input_path, output_path
            );
        }

        // Placeholder implementation for stream processing
        std::thread::sleep(Duration::from_millis(100));

        self.stats.add_processed_file(0); // Stream data doesn't have traditional file size

        Ok(())
    }

    /// Backup original file
    fn backup_file(&self, file_path: &Path) -> Result<(), ProcessError> {
        let backup_path = file_path.with_extension(format!(
            "{}.backup",
            file_path
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
        ));

        fs::copy(file_path, backup_path)
            .map_err(|e| ProcessError::IoError(format!("Failed to create backup: {}", e)))?;

        Ok(())
    }

    /// Get current configuration
    pub fn get_config(&self) -> &ProcessConfig {
        &self.config
    }

    /// Get processing statistics
    pub fn get_stats(&self) -> &ProcessingStats {
        &self.stats
    }

    /// Get supported file formats
    pub fn get_supported_formats(&self) -> &Vec<FileFormat> {
        &self.config.supported_formats
    }

    /// Check if a file format is supported
    pub fn is_format_supported(&self, format: &FileFormat) -> bool {
        self.config.supported_formats.contains(format)
    }

    /// Delegated run video extraction
    pub fn run_video_extraction(&mut self, config_path: &str) -> Result<(), ProcessError> {
        VideoProcessor::run_video_extraction(config_path, &mut self.stats)
    }
}
