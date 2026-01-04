use base64::{Engine, engine::general_purpose};
use opencv::{
    core::{Mat, Vector},
    imgcodecs,
    prelude::*,
};

use crate::camera::types::CameraError;

pub fn mat_to_base64_jpeg(frame: &Mat) -> Result<String, CameraError> {
    let mut buf: Vector<u8> = Vector::new();
    let params: Vector<i32> = Vector::new();

    imgcodecs::imencode(".jpg", frame, &mut buf, &params)
        .map_err(|e| CameraError::EncodeError(format!("Failed to encode frame: {}", e)))?;

    Ok(general_purpose::STANDARD.encode(buf.to_vec()))
}

#[allow(dead_code)]
pub fn mat_to_base64_png(frame: &Mat) -> Result<String, CameraError> {
    let mut buf: Vector<u8> = Vector::new();
    let params: Vector<i32> = Vector::new();

    imgcodecs::imencode(".png", frame, &mut buf, &params)
        .map_err(|e| CameraError::EncodeError(format!("Failed to encode frame: {}", e)))?;

    Ok(general_purpose::STANDARD.encode(buf.to_vec()))
}
