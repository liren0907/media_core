use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraInfo {
    pub camera_id: i32,
    pub width: i32,
    pub height: i32,
    pub fps: f64,
    pub backend: String,
}

#[derive(Debug, Clone)]
pub struct CameraFrame {
    pub index: usize,
    pub data: String,
}

pub type CameraResult<T> = Result<T, CameraError>;

#[derive(Debug, Clone)]
pub enum CameraError {
    NotAvailable(String),
    CaptureError(String),
    EncodeError(String),
    InvalidParameter(String),
}

impl std::fmt::Display for CameraError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CameraError::NotAvailable(msg) => write!(f, "Camera not available: {}", msg),
            CameraError::CaptureError(msg) => write!(f, "Capture error: {}", msg),
            CameraError::EncodeError(msg) => write!(f, "Encode error: {}", msg),
            CameraError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

impl std::error::Error for CameraError {}

impl From<String> for CameraError {
    fn from(s: String) -> Self {
        CameraError::CaptureError(s)
    }
}
