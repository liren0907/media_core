//! Thumbnail generation utilities for media files.

use opencv::{
    core::{Mat, Size},
    imgproc,
    prelude::MatTraitConst,
};

use crate::metadata::helpers::mat_to_base64_jpeg;

/// Generate thumbnail from video frame (max 320px)
pub fn generate_video_thumbnail(frame: &Mat, _width: f64, _height: f64) -> Result<String, String> {
    generate_thumbnail_with_max_size(frame, 320.0).or_else(|_| {
        // Fallback: return original frame if resize fails
        mat_to_base64_jpeg(frame)
    })
}

/// Generate thumbnail from image (max 320px)
pub fn generate_image_thumbnail(image: &Mat, _width: f64, _height: f64) -> Result<String, String> {
    generate_thumbnail_with_max_size(image, 320.0).or_else(|_| {
        // Fallback: return original image if resize fails
        mat_to_base64_jpeg(image)
    })
}

/// Generate thumbnail with custom maximum dimension
pub fn generate_thumbnail_with_max_size(frame: &Mat, max_dimension: f64) -> Result<String, String> {
    let width = frame.cols() as f64;
    let height = frame.rows() as f64;

    if width <= 0.0 || height <= 0.0 {
        return Err("Invalid frame dimensions".to_string());
    }

    // Calculate scale to fit within max_dimension
    let scale = max_dimension / width.max(height);

    // Don't upscale if already smaller
    if scale >= 1.0 {
        return mat_to_base64_jpeg(frame);
    }

    let thumb_width = (width * scale) as i32;
    let thumb_height = (height * scale) as i32;

    let mut thumb = Mat::default();
    let new_size = Size::new(thumb_width, thumb_height);

    imgproc::resize(frame, &mut thumb, new_size, 0.0, 0.0, imgproc::INTER_AREA)
        .map_err(|e| format!("Failed to resize for thumbnail: {}", e))?;

    mat_to_base64_jpeg(&thumb)
}

/// Generate multiple thumbnails at different sizes
pub fn generate_thumbnails(frame: &Mat, sizes: &[f64]) -> Vec<Result<String, String>> {
    sizes
        .iter()
        .map(|&size| generate_thumbnail_with_max_size(frame, size))
        .collect()
}
