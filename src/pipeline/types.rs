use std::path::PathBuf;

/// Defines the source of the media to be processed.
#[derive(Debug, Clone)]
pub enum MediaSource {
    /// A local file path (video, image, or audio)
    File(PathBuf),
    /// A network stream URL (e.g., RTSP, RTMP, HTTP)
    Stream(String),
    /// A local camera device ID
    Camera(i32),
}

impl MediaSource {
    /// Helper to get the path as a string if it is a file
    pub fn as_path_str(&self) -> Option<&str> {
        match self {
            MediaSource::File(p) => p.to_str(),
            _ => None,
        }
    }

    /// Helper to get the camera ID if it is a camera
    pub fn as_camera_id(&self) -> Option<i32> {
        match self {
            MediaSource::Camera(id) => Some(*id),
            _ => None,
        }
    }
}

