use opencv::{core::Mat, prelude::*, videoio};

use crate::streaming::helpers::{get_video_capture, mat_to_base64_jpeg};
use crate::streaming::sampler::stream_frames_sampled;
use crate::streaming::strategy::SamplingStrategy;
use crate::streaming::types::FrameData;

pub fn extract_frame(video_path: &str, frame_index: usize) -> Result<FrameData, String> {
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
) -> Result<Vec<FrameData>, String> {
    let strategy = SamplingStrategy::EveryNth(interval);
    let mut frames = stream_frames_sampled(video_path, strategy)?;

    if let Some(max) = max_frames {
        frames.truncate(max);
    }

    Ok(frames)
}
