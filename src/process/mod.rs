//! Process Module
//!
//! This module provides process-based functionality for media processing.
//! It operates independently from the RTSP module and follows a modular
//! approach for better maintainability and organization.

// Module declarations
pub mod config;
pub mod factories;
pub mod handlers;
pub mod hw_accel;
pub mod processor;
pub mod stats;
pub mod types;
pub mod validation;
pub mod video;
pub mod workers;

// Re-export commonly used items for convenience
pub use config::{
    ProcessConfig, ProcessingOptions, VideoExtractionConfig, generate_default_config,
};
pub use factories::{
    create_processor, create_processor_with_mode, create_processor_with_options,
    create_video_processor,
};
pub use hw_accel::{HardwareAccelConfig, HardwareAcceleratedCapture};
pub use processor::Processor;
pub use stats::ProcessingStats;
pub use types::{
    AudioFormat, DocumentFormat, FileFormat, ImageFormat, ProcessError, ProcessingMode,
    VideoFormat, get_default_supported_formats,
};
pub use video::VideoProcessor;
