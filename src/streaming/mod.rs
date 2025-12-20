mod extractor;
mod helpers;
mod sampler;
mod types;

pub use extractor::extract_frame;
pub use extractor::extract_frames_interval;
pub use extractor::get_stream_info;
pub use helpers::{get_video_capture, mat_to_base64_jpeg};
pub use sampler::stream_frames;
pub use sampler::stream_frames_sampled;
pub use types::FrameData;
pub use types::SamplingStrategy;
pub use types::StreamProgress;
pub use types::StreamResult;
