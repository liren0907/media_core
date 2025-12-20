//! Type definitions for media metadata processing.

use serde::{Deserialize, Serialize};

/// Unified media metadata structure for both video and image files
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct MediaMetadata {
    // Core file information
    pub file_path: String,
    pub file_size_bytes: u64,
    pub file_size_mb: f64,

    // Media properties (video-specific, None for images)
    pub frame_count: Option<i32>,
    pub fps: Option<f64>,
    pub width: i32,
    pub height: i32,
    pub resolution: String,
    pub duration_seconds: Option<f64>,
    pub duration_formatted: Option<String>,
    pub codec_name: Option<String>,

    // Color and depth information
    pub color_space: String,
    pub bit_depth: i32,
    pub channels: i32,

    // Enhanced metrics
    pub format: String,
    pub aspect_ratio: f64,
    pub bitrate_mbps: Option<f64>,
    pub total_pixels: i64,
    pub quality_category: String,
    pub estimated_memory_mb: f64,

    // Media type indicator
    pub media_type: String,

    // Optional thumbnail (base64 encoded JPEG)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
}
