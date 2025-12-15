use crate::process::config::ProcessConfig;
use crate::process::types::{ProcessError, ProcessingMode};
use std::fs;
use std::path::Path;

/// Validator for process configuration and paths
pub struct ProcessValidator;

impl ProcessValidator {
    /// Validate processing mode and configuration compatibility
    pub fn validate_processing_mode(config: &ProcessConfig) -> Result<(), ProcessError> {
        // Skip validation if validation is disabled (useful for tests)
        if !config.processing_options.enable_validation {
            return Ok(());
        }

        match config.processing_mode {
            ProcessingMode::DirectoryProcess => {
                let path = Path::new(&config.input_path);
                if path.exists() && !path.is_dir() {
                    return Err(ProcessError::ConfigurationError(
                        "Directory processing mode requires input path to be a directory"
                            .to_string(),
                    ));
                }
            }
            ProcessingMode::SingleFile => {
                let path = Path::new(&config.input_path);
                if path.exists() && !path.is_file() {
                    return Err(ProcessError::ConfigurationError(
                        "Single file processing mode requires input path to be a file".to_string(),
                    ));
                }
            }
            _ => {} // Other modes are flexible
        }
        Ok(())
    }

    /// Validate input path
    pub fn validate_input(config: &ProcessConfig, input_path: &str) -> Result<(), ProcessError> {
        if !config.processing_options.enable_validation {
            return Ok(());
        }

        if input_path.is_empty() {
            return Err(ProcessError::InvalidInput(
                "Input path is empty".to_string(),
            ));
        }

        let path = Path::new(input_path);
        if !path.exists() {
            return Err(ProcessError::InvalidInput(format!(
                "Input path does not exist: {}",
                input_path
            )));
        }

        Ok(())
    }

    /// Validate output path
    pub fn validate_output_path(
        config: &ProcessConfig,
        output_path: &str,
    ) -> Result<(), ProcessError> {
        if !config.processing_options.enable_validation {
            return Ok(());
        }

        if output_path.is_empty() {
            return Err(ProcessError::InvalidInput(
                "Output path is empty".to_string(),
            ));
        }

        let path = Path::new(output_path);
        if let Some(parent) = path.parent() {
            if !parent.exists() && !config.processing_options.create_output_directory {
                return Err(ProcessError::InvalidInput(format!(
                    "Output directory does not exist: {}",
                    parent.display()
                )));
            }
        }

        // Check if output file exists and overwrite is not allowed
        if path.exists() && path.is_file() && !config.processing_options.overwrite_existing {
            return Err(ProcessError::ValidationError(format!(
                "Output file already exists: {}",
                output_path
            )));
        }

        Ok(())
    }

    /// Ensure output directory exists
    pub fn ensure_output_directory(output_path: &str) -> Result<(), ProcessError> {
        let path = Path::new(output_path);
        let dir = if path.is_dir() {
            path
        } else {
            path.parent().unwrap_or(Path::new("."))
        };

        fs::create_dir_all(dir).map_err(|e| {
            ProcessError::IoError(format!("Failed to create output directory: {}", e))
        })?;

        Ok(())
    }
}
