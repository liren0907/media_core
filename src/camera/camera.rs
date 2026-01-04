use opencv::{
    core::{Mat, Size},
    imgproc,
    prelude::*,
    videoio::{self, VideoCapture},
};

use crate::camera::helpers::mat_to_base64_jpeg;
use crate::camera::types::{CameraError, CameraFrame, CameraInfo, CameraResult};

pub struct Camera {
    camera_id: i32,
    capture: VideoCapture,
    info: CameraInfo,
}

impl Camera {
    pub fn new(camera_id: i32) -> CameraResult<Self> {
        let capture = VideoCapture::new(camera_id, videoio::CAP_ANY).map_err(|e| {
            CameraError::NotAvailable(format!("Failed to open camera {}: {}", camera_id, e))
        })?;

        if !VideoCapture::is_opened(&capture)
            .map_err(|e| CameraError::NotAvailable(e.to_string()))?
        {
            return Err(CameraError::NotAvailable(format!(
                "Cannot open camera {}",
                camera_id
            )));
        }

        let width = capture.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap_or(640.0) as i32;
        let height = capture.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap_or(480.0) as i32;
        let fps = capture.get(videoio::CAP_PROP_FPS).unwrap_or(30.0);
        let backend_id = capture.get(videoio::CAP_PROP_BACKEND).unwrap_or(0.0) as i32;

        let backend = match backend_id {
            videoio::CAP_AVFOUNDATION => "AVFoundation".to_string(),
            videoio::CAP_V4L2 => "V4L2".to_string(),
            videoio::CAP_DSHOW => "DirectShow".to_string(),
            videoio::CAP_FFMPEG => "FFmpeg".to_string(),
            _ => format!("Backend({})", backend_id),
        };

        let info = CameraInfo {
            camera_id,
            width,
            height,
            fps,
            backend,
        };

        Ok(Self {
            camera_id,
            capture,
            info,
        })
    }

    pub fn info(&self) -> &CameraInfo {
        &self.info
    }

    pub fn camera_id(&self) -> i32 {
        self.camera_id
    }

    fn warm_up(&mut self, frames: usize) {
        let mut frame = Mat::default();
        for _ in 0..frames {
            let _ = self.capture.read(&mut frame);
        }
    }

    pub fn capture_frame(&mut self) -> CameraResult<CameraFrame> {
        self.warm_up(5);

        let mut frame = Mat::default();
        if self
            .capture
            .read(&mut frame)
            .map_err(|e| CameraError::CaptureError(e.to_string()))?
            && !frame.empty()
        {
            let data = mat_to_base64_jpeg(&frame)?;
            Ok(CameraFrame { index: 0, data })
        } else {
            Err(CameraError::CaptureError(
                "Failed to capture frame".to_string(),
            ))
        }
    }

    pub fn capture_frames(
        &mut self,
        count: usize,
        scale_factor: Option<f64>,
    ) -> CameraResult<Vec<CameraFrame>> {
        if count == 0 {
            return Ok(Vec::new());
        }

        self.warm_up(5);

        let mut frames = Vec::with_capacity(count);
        let mut frame = Mat::default();

        for i in 0..count {
            if self
                .capture
                .read(&mut frame)
                .map_err(|e| CameraError::CaptureError(e.to_string()))?
                && !frame.empty()
            {
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
                        .map_err(|e| CameraError::CaptureError(e.to_string()))?;
                        resized
                    } else {
                        frame.clone()
                    }
                } else {
                    frame.clone()
                };

                let data = mat_to_base64_jpeg(&processed_frame)?;
                frames.push(CameraFrame { index: i, data });
            }
        }

        Ok(frames)
    }

    pub fn capture_interval(
        &mut self,
        interval_ms: u64,
        max_frames: usize,
    ) -> CameraResult<Vec<CameraFrame>> {
        use std::thread;
        use std::time::Duration;

        if max_frames == 0 {
            return Ok(Vec::new());
        }

        self.warm_up(5);

        let mut frames = Vec::with_capacity(max_frames);
        let mut frame = Mat::default();
        let interval = Duration::from_millis(interval_ms);

        for i in 0..max_frames {
            if self
                .capture
                .read(&mut frame)
                .map_err(|e| CameraError::CaptureError(e.to_string()))?
                && !frame.empty()
            {
                let data = mat_to_base64_jpeg(&frame)?;
                frames.push(CameraFrame { index: i, data });
            }

            if i < max_frames - 1 {
                thread::sleep(interval);
            }
        }

        Ok(frames)
    }

    pub fn set_resolution(&mut self, width: i32, height: i32) -> CameraResult<()> {
        self.capture
            .set(videoio::CAP_PROP_FRAME_WIDTH, width as f64)
            .map_err(|e| CameraError::CaptureError(e.to_string()))?;
        self.capture
            .set(videoio::CAP_PROP_FRAME_HEIGHT, height as f64)
            .map_err(|e| CameraError::CaptureError(e.to_string()))?;

        let actual_width = self
            .capture
            .get(videoio::CAP_PROP_FRAME_WIDTH)
            .unwrap_or(width as f64) as i32;
        let actual_height = self
            .capture
            .get(videoio::CAP_PROP_FRAME_HEIGHT)
            .unwrap_or(height as f64) as i32;

        self.info.width = actual_width;
        self.info.height = actual_height;

        Ok(())
    }

    pub fn capture_frame_with_resolution(
        &mut self,
        width: i32,
        height: i32,
    ) -> CameraResult<CameraFrame> {
        self.set_resolution(width, height)?;
        self.capture_frame()
    }
}
