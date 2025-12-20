//! Video processing functions.

use opencv::{core::Mat, prelude::*, videoio};
use std::fs;

use crate::metadata::helpers::get_video_capture;
use crate::metadata::{
    codec_analyzer::{analyze_color_info, detect_codec_name, detect_color_space},
    quality_analyzer::{
        assess_video_quality, calculate_aspect_ratio, calculate_bitrate,
        estimate_video_memory_usage,
    },
    thumbnail_generator::generate_video_thumbnail,
    types::MediaMetadata,
};

/// Process video file and extract metadata
pub fn process_video_file(
    media_path: &str,
    include_thumbnail: bool,
) -> Result<MediaMetadata, String> {
    let mut cap = get_video_capture(media_path)?;

    // Get basic video properties
    let frame_count = cap.get(videoio::CAP_PROP_FRAME_COUNT).unwrap_or(0.0);
    let fps = cap.get(videoio::CAP_PROP_FPS).unwrap_or(0.0);
    let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap_or(0.0);
    let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap_or(0.0);
    let codec = cap.get(videoio::CAP_PROP_FOURCC).unwrap_or(0.0) as i32;

    let codec_str = detect_codec_name(codec);

    // Extract color space and bit depth information
    let mut frame = Mat::default();
    let channels_count = if cap.read(&mut frame).unwrap_or(false) && !frame.empty() {
        frame.channels()
    } else {
        3 // Default to 3 channels
    };

    let (_color_space, bit_depth) = if !frame.empty() {
        analyze_color_info(&frame)
    } else {
        ("Unknown".to_string(), 8)
    };

    let final_color_space = detect_color_space(&codec_str, channels_count, bit_depth);

    // Get thumbnail if requested
    let thumbnail = if include_thumbnail && !frame.empty() {
        generate_video_thumbnail(&frame, width, height).ok()
    } else {
        None
    };

    // Get file size
    let file_size = fs::metadata(media_path).map(|m| m.len()).unwrap_or(0);

    // Calculate metrics
    let duration_seconds = if fps > 0.0 { frame_count / fps } else { 0.0 };
    let aspect_ratio = calculate_aspect_ratio(width, height);
    let bitrate_mbps = calculate_bitrate(file_size, duration_seconds);
    let total_pixels = width * height;
    let quality_score = assess_video_quality(total_pixels);

    let duration_formatted = format!(
        "{}:{:02}:{:02}",
        (duration_seconds as i32) / 3600,
        ((duration_seconds as i32) % 3600) / 60,
        (duration_seconds as i32) % 60
    );

    // Determine format from extension
    let path_lower = media_path.to_lowercase();
    let format = if path_lower.ends_with(".mp4") {
        "MP4"
    } else if path_lower.ends_with(".avi") {
        "AVI"
    } else if path_lower.ends_with(".mov") {
        "MOV"
    } else if path_lower.ends_with(".mkv") {
        "MKV"
    } else if path_lower.ends_with(".webm") {
        "WebM"
    } else {
        "Unknown"
    };

    Ok(MediaMetadata {
        file_path: media_path.to_string(),
        file_size_bytes: file_size,
        file_size_mb: (file_size as f64) / (1024.0 * 1024.0),
        frame_count: Some(frame_count as i32),
        fps: Some(fps),
        width: width as i32,
        height: height as i32,
        resolution: format!("{}x{}", width as i32, height as i32),
        duration_seconds: Some(duration_seconds),
        duration_formatted: Some(duration_formatted),
        codec_name: Some(codec_str),
        color_space: final_color_space,
        bit_depth,
        channels: channels_count,
        format: format.to_string(),
        aspect_ratio,
        bitrate_mbps: Some(bitrate_mbps),
        total_pixels: total_pixels as i64,
        quality_category: quality_score,
        estimated_memory_mb: estimate_video_memory_usage(width, height, frame_count),
        media_type: "video".to_string(),
        thumbnail,
    })
}

/// Extract the first frame as base64 JPEG
pub fn get_first_frame(media_path: &str) -> Result<String, String> {
    let mut cam = get_video_capture(media_path)?;

    let mut frame = Mat::default();
    if !cam.read(&mut frame).map_err(|e| e.to_string())? || frame.empty() {
        return Err("Could not read the first frame".to_string());
    }

    crate::metadata::helpers::mat_to_base64_jpeg(&frame)
}

/// Get video duration in seconds
pub fn get_video_duration(media_path: &str) -> Result<f64, String> {
    let cap = get_video_capture(media_path)?;

    let fps = cap.get(videoio::CAP_PROP_FPS).unwrap_or(0.0);
    let frame_count = cap.get(videoio::CAP_PROP_FRAME_COUNT).unwrap_or(0.0);

    if fps > 0.0 {
        Ok(frame_count / fps)
    } else {
        Err("Could not determine video duration".to_string())
    }
}
