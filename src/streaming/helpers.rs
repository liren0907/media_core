use base64::{engine::general_purpose, Engine};
use opencv::{
    core::{Mat, Vector},
    imgcodecs,
    prelude::*,
    videoio::{self, VideoCapture},
};

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

pub fn mat_to_base64_jpeg(frame: &Mat) -> Result<String, String> {
    let mut buf: Vector<u8> = Vector::new();
    let params: Vector<i32> = Vector::new();

    imgcodecs::imencode(".jpg", frame, &mut buf, &params)
        .map_err(|e| format!("Failed to encode frame: {}", e))?;

    Ok(general_purpose::STANDARD.encode(buf.to_vec()))
}
