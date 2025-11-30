//! Process Module
//!
//! This module provides process-based functionality for media processing.
//! It operates independently from the RTSP module and follows a modular
//! approach for better maintainability and organization.

// Module declarations
pub mod config;
pub mod factories;
pub mod hw_accel;
pub mod processor;
pub mod stats;
pub mod types;
pub mod video;

// Re-export commonly used items for convenience
pub use config::{
    generate_default_config, ProcessConfig, ProcessingOptions, VideoExtractionConfig,
};
pub use factories::{
    create_processor, create_processor_with_mode, create_processor_with_options,
    create_video_processor,
};
pub use hw_accel::{HardwareAccelConfig, HardwareAcceleratedCapture};
pub use processor::Processor;
pub use stats::ProcessingStats;
pub use types::{
    get_default_supported_formats, AudioFormat, DocumentFormat, FileFormat, ImageFormat,
    ProcessError, ProcessingMode, VideoFormat,
};
pub use video::VideoProcessor;
