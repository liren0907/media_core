//! Helper functions for video capture and frame processing.

use crate::streaming::types::StreamProgress;
use base64::{Engine, engine::general_purpose};
use opencv::{
    core::{Mat, Vector},
    imgcodecs,
    prelude::*,
    videoio::{self, VideoCapture},
};

/// Get basic video stream information.
pub fn get_stream_info(video_path: &str) -> Result<StreamProgress, String> {
    let cam = get_video_capture(video_path)?;

    let frame_count = cam.get(videoio::CAP_PROP_FRAME_COUNT).unwrap_or(0.0) as usize;
    let fps = cam.get(videoio::CAP_PROP_FPS).unwrap_or(0.0);
    let width = cam.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap_or(0.0) as i32;
    let height = cam.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap_or(0.0) as i32;

    Ok(StreamProgress {
        current: 0,
        total: frame_count,
        message: format!(
            "Video: {}x{} @ {:.2} FPS, {} frames",
            width, height, fps, frame_count
        ),
    })
}

/// Open a video file and return a VideoCapture object.
pub fn get_video_capture(video_path: &str) -> Result<VideoCapture, String> {
    let cam = VideoCapture::from_file(video_path, videoio::CAP_ANY)
        .map_err(|e| format!("Failed to open video file: {}", e))?;

    let opened = VideoCapture::is_opened(&cam)
        .map_err(|e| format!("Failed to check if video is opened: {}", e))?;

    if !opened {
        return Err(format!("Cannot open video stream for path: {}", video_path));
    }

    Ok(cam)
}

/// Convert an OpenCV Mat to a base64-encoded JPEG string.
pub fn mat_to_base64_jpeg(frame: &Mat) -> Result<String, String> {
    let mut buf: Vector<u8> = Vector::new();
    let params: Vector<i32> = Vector::new();

    imgcodecs::imencode(".jpg", frame, &mut buf, &params)
        .map_err(|e| format!("Failed to encode frame: {}", e))?;

    Ok(general_purpose::STANDARD.encode(buf.to_vec()))
}
