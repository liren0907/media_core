use crate::rtsp::capture::RTSPCapture;
use chrono::Local;
use opencv::{prelude::*, videoio, Result};
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

impl RTSPCapture {
    pub fn start_opencv_recording(&mut self) -> Result<()> {
        let mut capture = videoio::VideoCapture::from_file(&self.url, videoio::CAP_FFMPEG)?;
        if !capture.is_opened()? {
            return Err(opencv::Error::new(opencv::core::StsError, "Failed to open RTSP stream"));
        }

        let stream_fps = capture.get(videoio::CAP_PROP_FPS)?;
        let actual_fps = if stream_fps <= 0.0 {
            if self.use_custom_fps {
                self.custom_fps
            } else {
                30.0
            }
        } else {
            stream_fps
        };
        println!("Stream FPS: {}", actual_fps);
        let _ = capture.set(videoio::CAP_PROP_CONVERT_RGB, 1.0);
        self.capture = Some(capture);
        Ok(())
    }

    pub fn process_stream_opencv(&mut self) -> Result<()> {
        let window = if self.show_preview {
            let window_name = format!("RTSP Stream - {}", self.url);
            opencv::highgui::named_window(&window_name, opencv::highgui::WINDOW_AUTOSIZE)?;
            Some(window_name)
        } else {
            None
        };

        let mut frame = Mat::default();
        self.create_new_video_file()?;

        loop {
            let current_time = Instant::now();
            let segment_elapsed = current_time.duration_since(self.current_file_start);
            if segment_elapsed >= self.segment_duration {
                self.create_new_video_file()?;
                continue;
            }
            if let Some(capture) = &mut self.capture {
                let frame_read = capture.read(&mut frame)?;
                if frame_read && !frame.empty() {
                    if let Some(writer) = &mut self.writer {
                        writer.write(&frame)?;
                    }
                    if let Some(window_name) = &window {
                        opencv::highgui::imshow(window_name, &frame)?;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
            if self.show_preview {
                let key = opencv::highgui::wait_key(1)?;
                if key == 27 {
                    break;
                }
            } else {
                thread::sleep(Duration::from_millis(10));
            }
        }
        if let Some(window_name) = &window {
            opencv::highgui::destroy_window(window_name)?;
        }
        Ok(())
    }

    pub fn create_new_video_file(&mut self) -> Result<()> {
        if let Some(mut writer) = self.writer.take() {
            writer.release()?;
        }
        let camera_dir = PathBuf::from(&self.output_dir).join(format!(
            "camera_{}",
            self.url
                .replace("://", "_")
                .replace("/", "_")
                .replace(":", "_")
        ));
        fs::create_dir_all(&camera_dir).map_err(|e| {
            opencv::Error::new(opencv::core::StsError, &format!("Failed to create directory: {}", e))
        })?;
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let file_name = camera_dir.join(format!("segment_{}.mp4", timestamp));
        if let Some(capture) = &self.capture {
            let frame_width = capture.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
            let frame_height = capture.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;
            let stream_fps = capture.get(videoio::CAP_PROP_FPS)?;
            let fps = if self.use_custom_fps {
                self.custom_fps
            } else if stream_fps > 0.0 {
                stream_fps
            } else {
                30.0
            };
            let fourcc = videoio::VideoWriter::fourcc('m', 'p', '4', 'v')?;
            let writer = videoio::VideoWriter::new(
                file_name.to_str().unwrap(),
                fourcc,
                fps,
                (frame_width, frame_height).into(),
                true,
            )?;
            if !writer.is_opened()? {
                return Err(opencv::Error::new(opencv::core::StsError, "Failed to create video writer"));
            }
            self.writer = Some(writer);
            self.current_file_start = Instant::now();
        }
        Ok(())
    }
}