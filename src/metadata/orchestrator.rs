//! High-level orchestration functions for media metadata processing.

use crate::metadata::{
    image_processor::process_image_file, types::MediaMetadata, video_processor::process_video_file,
};

/// Unified media information extractor
/// Handles both video and image files with appropriate metadata extraction
pub fn get_media_info(media_path: &str, include_thumbnail: bool) -> Result<MediaMetadata, String> {
    let path_lower = media_path.to_lowercase();

    let is_video = path_lower.ends_with(".mp4")
        || path_lower.ends_with(".avi")
        || path_lower.ends_with(".mov")
        || path_lower.ends_with(".mkv")
        || path_lower.ends_with(".webm")
        || path_lower.ends_with(".flv")
        || path_lower.ends_with(".wmv");

    let is_image = path_lower.ends_with(".jpg")
        || path_lower.ends_with(".jpeg")
        || path_lower.ends_with(".png")
        || path_lower.ends_with(".bmp")
        || path_lower.ends_with(".tiff")
        || path_lower.ends_with(".tif")
        || path_lower.ends_with(".webp");

    if is_video {
        process_video_file(media_path, include_thumbnail)
    } else if is_image {
        process_image_file(media_path, include_thumbnail)
    } else {
        Err("Unsupported media format. Supported: MP4, AVI, MOV, MKV, WebM, JPG, PNG, BMP, TIFF, WebP".to_string())
    }
}

/// Get media info as JSON string
pub fn get_media_info_json(media_path: &str, include_thumbnail: bool) -> Result<String, String> {
    let metadata = get_media_info(media_path, include_thumbnail)?;
    serde_json::to_string_pretty(&metadata)
        .map_err(|e| format!("Failed to serialize metadata: {}", e))
}

/// Check if a file is a supported media type
pub fn is_supported_media(media_path: &str) -> bool {
    let path_lower = media_path.to_lowercase();

    let is_video = path_lower.ends_with(".mp4")
        || path_lower.ends_with(".avi")
        || path_lower.ends_with(".mov")
        || path_lower.ends_with(".mkv")
        || path_lower.ends_with(".webm")
        || path_lower.ends_with(".flv")
        || path_lower.ends_with(".wmv");

    let is_image = path_lower.ends_with(".jpg")
        || path_lower.ends_with(".jpeg")
        || path_lower.ends_with(".png")
        || path_lower.ends_with(".bmp")
        || path_lower.ends_with(".tiff")
        || path_lower.ends_with(".tif")
        || path_lower.ends_with(".webp");

    is_video || is_image
}

/// Get media type string
pub fn get_media_type(media_path: &str) -> Option<&'static str> {
    let path_lower = media_path.to_lowercase();

    if path_lower.ends_with(".mp4")
        || path_lower.ends_with(".avi")
        || path_lower.ends_with(".mov")
        || path_lower.ends_with(".mkv")
        || path_lower.ends_with(".webm")
        || path_lower.ends_with(".flv")
        || path_lower.ends_with(".wmv")
    {
        Some("video")
    } else if path_lower.ends_with(".jpg")
        || path_lower.ends_with(".jpeg")
        || path_lower.ends_with(".png")
        || path_lower.ends_with(".bmp")
        || path_lower.ends_with(".tiff")
        || path_lower.ends_with(".tif")
        || path_lower.ends_with(".webp")
    {
        Some("image")
    } else {
        None
    }
}
