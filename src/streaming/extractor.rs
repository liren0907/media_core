use opencv::{core::Mat, prelude::*, videoio};

use crate::streaming::helpers::{get_video_capture, mat_to_base64_jpeg};
use crate::streaming::types::{FrameData, StreamProgress, StreamResult};

pub fn get_stream_info(video_path: &str) -> StreamResult<StreamProgress> {
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

pub fn extract_frame(video_path: &str, frame_index: usize) -> StreamResult<FrameData> {
    let mut cam = get_video_capture(video_path)?;

    cam.set(videoio::CAP_PROP_POS_FRAMES, frame_index as f64)
        .map_err(|e| format!("Failed to seek to frame {}: {}", frame_index, e))?;

    let mut frame = Mat::default();
    if cam.read(&mut frame).unwrap_or(false) && !frame.empty() {
        let frame_data = mat_to_base64_jpeg(&frame)?;
        Ok(FrameData {
            index: frame_index,
            data: frame_data,
        })
    } else {
        Err(format!("Failed to read frame at index {}", frame_index))
    }
}

pub fn extract_frames_interval(
    video_path: &str,
    interval: usize,
    max_frames: Option<usize>,
) -> StreamResult<Vec<FrameData>> {
    use crate::streaming::sampler::stream_frames_sampled;
    use crate::streaming::types::SamplingStrategy;

    let strategy = SamplingStrategy::EveryNth(interval);
    let mut frames = stream_frames_sampled(video_path, strategy)?;

    if let Some(max) = max_frames {
        frames.truncate(max);
    }

    Ok(frames)
}
