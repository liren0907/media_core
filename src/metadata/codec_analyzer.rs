//! Codec detection and color space analysis for media files.

use opencv::prelude::MatTraitConst;

/// Detect codec name from FourCC code
pub fn detect_codec_name(codec_fourcc: i32) -> String {
    let c1 = ((codec_fourcc & 0xFF) as u8) as char;
    let c2 = (((codec_fourcc >> 8) & 0xFF) as u8) as char;
    let c3 = (((codec_fourcc >> 16) & 0xFF) as u8) as char;
    let c4 = (((codec_fourcc >> 24) & 0xFF) as u8) as char;

    format!("{}{}{}{}", c1, c2, c3, c4)
}

/// Detect color space based on codec name, channels, and bit depth
pub fn detect_color_space(codec_name: &str, channels: i32, _bit_depth: i32) -> String {
    // Codec-specific color space detection
    let codec_color_space = match codec_name {
        "avc1" | "H264" | "h264" => "YUV420",
        "hev1" | "hvc1" | "H265" | "hevc" => "YUV420",
        "mp4v" => "YUV420",
        _ => "",
    };

    // If we have codec-specific color space info, use it
    if !codec_color_space.is_empty() {
        return codec_color_space.to_string();
    }

    // Otherwise, determine based on channels
    match channels {
        1 => "Grayscale".to_string(),
        3 => "BGR".to_string(),
        4 => "BGRA".to_string(),
        _ => "Unknown".to_string(),
    }
}

/// Determine bit depth from OpenCV depth constants
pub fn determine_bit_depth(depth: i32) -> i32 {
    match depth {
        opencv::core::CV_8U => 8,
        opencv::core::CV_16U => 16,
        opencv::core::CV_32F => 32,
        _ => 8,
    }
}

/// Get color space info from image/frame analysis
pub fn analyze_color_info(frame: &opencv::core::Mat) -> (String, i32) {
    let channels = frame.channels();

    let color_space = match channels {
        1 => "Grayscale",
        3 => "BGR",
        4 => "BGRA",
        _ => "Unknown",
    };

    let bit_depth = determine_bit_depth(frame.depth());

    (color_space.to_string(), bit_depth)
}

/// Get human-readable codec name from fourcc string
pub fn get_codec_display_name(fourcc_str: &str) -> String {
    match fourcc_str.to_lowercase().as_str() {
        "avc1" | "h264" => "H.264/AVC".to_string(),
        "hev1" | "hvc1" | "hevc" => "H.265/HEVC".to_string(),
        "mp4v" => "MPEG-4 Part 2".to_string(),
        "vp8" | "vp80" => "VP8".to_string(),
        "vp9" | "vp90" => "VP9".to_string(),
        "av01" => "AV1".to_string(),
        "mjpg" => "Motion JPEG".to_string(),
        _ => fourcc_str.to_uppercase(),
    }
}
