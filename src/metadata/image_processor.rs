//! Image processing functions.

use opencv::{imgcodecs, prelude::MatTraitConst};
use std::fs;

use crate::metadata::{
    codec_analyzer::analyze_color_info,
    quality_analyzer::{assess_image_quality, calculate_aspect_ratio, estimate_image_memory_usage},
    thumbnail_generator::generate_image_thumbnail,
    types::MediaMetadata,
};

/// Process image file and extract metadata
pub fn process_image_file(
    media_path: &str,
    include_thumbnail: bool,
) -> Result<MediaMetadata, String> {
    // Read image using OpenCV
    let img = imgcodecs::imread(media_path, imgcodecs::IMREAD_UNCHANGED)
        .map_err(|e| format!("Cannot open image file: {}", e))?;

    if img.empty() {
        return Err("Cannot read image data".to_string());
    }

    // Get basic image properties
    let width = img.cols() as f64;
    let height = img.rows() as f64;
    let channels = img.channels();

    // Determine color space based on channels
    let color_space = match channels {
        1 => "Grayscale",
        3 => "BGR",
        4 => "BGRA",
        _ => "Unknown",
    };

    // Get color space and bit depth from frame analysis
    let (analyzed_color_space, bit_depth) = analyze_color_info(&img);

    let final_color_space = if analyzed_color_space != "Unknown" {
        analyzed_color_space
    } else {
        color_space.to_string()
    };

    // Get file size
    let file_size = fs::metadata(media_path).map(|m| m.len()).unwrap_or(0);

    // Calculate metrics
    let aspect_ratio = calculate_aspect_ratio(width, height);
    let total_pixels = width * height;
    let quality_score = assess_image_quality(total_pixels);

    // Determine image format
    let path_lower = media_path.to_lowercase();
    let format = if path_lower.ends_with(".jpg") || path_lower.ends_with(".jpeg") {
        "JPEG"
    } else if path_lower.ends_with(".png") {
        "PNG"
    } else if path_lower.ends_with(".bmp") {
        "BMP"
    } else if path_lower.ends_with(".tiff") || path_lower.ends_with(".tif") {
        "TIFF"
    } else if path_lower.ends_with(".webp") {
        "WebP"
    } else {
        "Unknown"
    };

    // Generate thumbnail if requested
    let thumbnail = if include_thumbnail {
        generate_image_thumbnail(&img, width, height).ok()
    } else {
        None
    };

    Ok(MediaMetadata {
        file_path: media_path.to_string(),
        file_size_bytes: file_size,
        file_size_mb: (file_size as f64) / (1024.0 * 1024.0),
        frame_count: None,
        fps: None,
        width: width as i32,
        height: height as i32,
        resolution: format!("{}x{}", width as i32, height as i32),
        duration_seconds: None,
        duration_formatted: None,
        codec_name: None,
        color_space: final_color_space,
        bit_depth,
        channels,
        format: format.to_string(),
        aspect_ratio,
        bitrate_mbps: None,
        total_pixels: total_pixels as i64,
        quality_category: quality_score,
        estimated_memory_mb: estimate_image_memory_usage(width, height, channels),
        media_type: "image".to_string(),
        thumbnail,
    })
}
