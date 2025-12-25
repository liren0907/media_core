mod extractor;
mod helpers;
mod sampler;
mod strategy;
mod types;

pub use extractor::extract_frame;
pub use extractor::extract_frames_interval;
pub use helpers::{get_stream_info, get_video_capture, mat_to_base64_jpeg};
pub use sampler::stream_frames;
pub use sampler::stream_frames_sampled;
pub use strategy::SamplingStrategy;
pub use types::FrameData;
pub use types::StreamProgress;
