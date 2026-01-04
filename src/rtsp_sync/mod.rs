//! RTSP Sync Module
//!
//! Provides synchronized RTSP stream capture with support for:
//! - Wall-clock aligned HLS streaming
//! - Latency monitoring with PTS tracking
//! - Multi-stream recording
//! - Mode-based stream processing (Preview, Latency, Recording, Sync)

pub mod ffmpeg_utils;
pub mod latency;
pub mod processor;
pub mod synchronizer;
pub mod types;

// Re-export commonly used items
pub use ffmpeg_utils::FFmpegUtils;
pub use latency::LatencyMonitor;
pub use processor::StreamProcessor;
pub use synchronizer::{spawn_single_hls_stream, start_ffmpeg_hls, start_ffmpeg_sync_hls};
pub use types::{
    HLSSyncConfig, LatencyMonitorConfig, LogMessage, Mode, RtspSyncConfig, RtspSyncError,
    StreamMetadata, TimeInfo,
};
