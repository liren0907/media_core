use opencv::prelude::*;
use opencv::videoio::{self, VideoCapture};

pub fn is_camera_available(camera_id: i32) -> bool {
    if let Ok(camera) = VideoCapture::new(camera_id, videoio::CAP_ANY) {
        VideoCapture::is_opened(&camera).unwrap_or(false)
    } else {
        false
    }
}

pub fn list_available_cameras() -> Vec<i32> {
    (0..10).filter(|&id| is_camera_available(id)).collect()
}

pub fn default_camera_id() -> i32 {
    0
}

pub fn has_camera() -> bool {
    !list_available_cameras().is_empty()
}
