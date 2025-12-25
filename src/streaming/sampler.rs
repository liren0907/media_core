use opencv::{
    core::{Mat, Size},
    imgproc,
    prelude::*,
    videoio,
};

use crate::streaming::helpers::{get_video_capture, mat_to_base64_jpeg};
use crate::streaming::strategy::SamplingStrategy;
use crate::streaming::types::FrameData;

pub fn stream_frames(
    video_path: &str,
    skip: usize,
    max: usize,
    scale_factor: Option<f64>,
) -> Result<Vec<FrameData>, String> {
    let mut cam = get_video_capture(video_path)?;
    let mut frames = Vec::new();
    let mut frame = Mat::default();
    let mut frame_index = 0;
    let mut frames_sent = 0;

    while cam.read(&mut frame).unwrap_or(false) {
        if frame.empty() {
            break;
        }

        if frame_index >= skip {
            let processed_frame = if let Some(factor) = scale_factor {
                if factor != 1.0 {
                    let mut resized = Mat::default();
                    let new_size = Size::new(
                        (frame.cols() as f64 * factor) as i32,
                        (frame.rows() as f64 * factor) as i32,
                    );
                    imgproc::resize(
                        &frame,
                        &mut resized,
                        new_size,
                        0.0,
                        0.0,
                        imgproc::INTER_AREA,
                    )
                    .map_err(|e| e.to_string())?;
                    resized
                } else {
                    frame.clone()
                }
            } else {
                frame.clone()
            };

            let frame_data = mat_to_base64_jpeg(&processed_frame)?;
            frames.push(FrameData {
                index: frame_index,
                data: frame_data,
            });

            frames_sent += 1;
            if frames_sent >= max {
                break;
            }
        }
        frame_index += 1;
    }

    Ok(frames)
}

pub fn stream_frames_sampled(
    video_path: &str,
    sampling_strategy: SamplingStrategy,
) -> Result<Vec<FrameData>, String> {
    let mut cam = get_video_capture(video_path)?;

    let total_frames = cam.get(videoio::CAP_PROP_FRAME_COUNT).unwrap_or(0.0) as usize;
    let frame_indices = sampling_strategy.get_frame_indices(total_frames);

    let mut frames = Vec::new();
    let mut frame = Mat::default();

    for frame_index in frame_indices {
        cam.set(videoio::CAP_PROP_POS_FRAMES, frame_index as f64)
            .map_err(|e| format!("Failed to seek to frame {}: {}", frame_index, e))?;

        if cam.read(&mut frame).unwrap_or(false) && !frame.empty() {
            let frame_data = mat_to_base64_jpeg(&frame)?;
            frames.push(FrameData {
                index: frame_index,
                data: frame_data,
            });
        }
    }

    Ok(frames)
}
