//! Quality assessment and metrics calculation for media files.

/// Quality assessment thresholds (in total pixels)
pub mod quality_thresholds {
    pub const ULTRA_HIGH: f64 = 8_294_400.0; // 3840x2160 (4K)
    pub const FULL_HD: f64 = 2_073_600.0; // 1920x1080
    pub const HD: f64 = 921_600.0; // 1280x720
    pub const SD: f64 = 307_200.0; // 640x480
}

/// Video quality assessment based on total pixel count
pub fn assess_video_quality(total_pixels: f64) -> String {
    if total_pixels >= quality_thresholds::ULTRA_HIGH {
        "4K".to_string()
    } else if total_pixels >= quality_thresholds::FULL_HD {
        "Full HD".to_string()
    } else if total_pixels >= quality_thresholds::HD {
        "HD".to_string()
    } else if total_pixels >= quality_thresholds::SD {
        "SD".to_string()
    } else {
        "Low".to_string()
    }
}

/// Image quality assessment based on total pixel count
pub fn assess_image_quality(total_pixels: f64) -> String {
    if total_pixels >= quality_thresholds::ULTRA_HIGH {
        "Ultra High".to_string()
    } else if total_pixels >= quality_thresholds::FULL_HD {
        "High".to_string()
    } else if total_pixels >= quality_thresholds::HD {
        "Medium".to_string()
    } else if total_pixels >= quality_thresholds::SD {
        "Low".to_string()
    } else {
        "Very Low".to_string()
    }
}

/// Calculate bitrate in Mbps for video files
pub fn calculate_bitrate(file_size_bytes: u64, duration_seconds: f64) -> f64 {
    if duration_seconds > 0.0 {
        (file_size_bytes as f64 * 8.0) / (duration_seconds * 1_000_000.0)
    } else {
        0.0
    }
}

/// Estimate memory usage for video files (RGB assumption: 3 bytes per pixel)
pub fn estimate_video_memory_usage(width: f64, height: f64, frame_count: f64) -> f64 {
    let bytes_per_frame = width * height * 3.0;
    let total_bytes = bytes_per_frame * frame_count;
    total_bytes / (1024.0 * 1024.0)
}

/// Estimate memory usage for image files
pub fn estimate_image_memory_usage(width: f64, height: f64, channels: i32) -> f64 {
    let total_bytes = width * height * channels as f64;
    total_bytes / (1024.0 * 1024.0)
}

/// Calculate aspect ratio from dimensions
pub fn calculate_aspect_ratio(width: f64, height: f64) -> f64 {
    if height > 0.0 { width / height } else { 0.0 }
}

/// Get quality score as a numerical value (0.0 to 1.0)
pub fn get_quality_score(total_pixels: f64) -> f64 {
    use quality_thresholds::*;

    if total_pixels >= ULTRA_HIGH {
        1.0
    } else if total_pixels >= FULL_HD {
        0.8
    } else if total_pixels >= HD {
        0.6
    } else if total_pixels >= SD {
        0.4
    } else {
        0.2
    }
}

/// Get standard aspect ratio name
pub fn get_aspect_ratio_name(ratio: f64) -> String {
    const TOLERANCE: f64 = 0.05;

    if (ratio - 16.0 / 9.0).abs() < TOLERANCE {
        "16:9".to_string()
    } else if (ratio - 4.0 / 3.0).abs() < TOLERANCE {
        "4:3".to_string()
    } else if (ratio - 21.0 / 9.0).abs() < TOLERANCE {
        "21:9".to_string()
    } else if (ratio - 1.0).abs() < TOLERANCE {
        "1:1".to_string()
    } else if (ratio - 9.0 / 16.0).abs() < TOLERANCE {
        "9:16".to_string()
    } else {
        format!("{:.2}:1", ratio)
    }
}
