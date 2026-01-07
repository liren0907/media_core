mod extractor;
mod helpers;
pub mod pipeline;
mod strategy;
mod types;

pub use extractor::ExtractionMode;
pub use extractor::StreamExtractor;
pub use helpers::{get_stream_info, get_video_capture, mat_to_base64_jpeg};

pub use strategy::SamplingStrategy;
pub use types::FrameData;
pub use types::StreamProgress;
