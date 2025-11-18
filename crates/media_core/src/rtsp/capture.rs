use opencv::{videoio, Result};
use std::process::Child;
use std::time::{Duration, Instant};

pub struct RTSPCapture {
    pub url: String,
    pub output_dir: String,
    pub show_preview: bool,
    pub capture: Option<videoio::VideoCapture>,
    pub writer: Option<videoio::VideoWriter>,
    pub ffmpeg_process: Option<Child>,
    pub current_file_start: Instant,
    pub segment_duration: Duration,
    pub use_custom_fps: bool,
    pub custom_fps: f64,
}

impl RTSPCapture {
    pub fn new(
        url: String,
        output_dir: String,
        show_preview: bool,
        segment_duration_secs: u64,
        use_custom_fps: bool,
        custom_fps: f64,
    ) -> Result<Self> {
        Ok(Self {
            url,
            output_dir,
            show_preview,
            capture: None,
            writer: None,
            ffmpeg_process: None,
            current_file_start: Instant::now(),
            segment_duration: Duration::from_secs(segment_duration_secs),
            use_custom_fps,
            custom_fps,
        })
    }

    pub fn process_stream(&mut self) -> Result<()> {
        if self.use_custom_fps {
            self.start_opencv_recording()?;
            self.process_stream_opencv()
        } else {
            self.start_ffmpeg_recording().map_err(|e| {
                opencv::Error::new(opencv::core::StsError, &format!("Failed to start FFmpeg: {}", e))
            })?;
            self.process_stream_ffmpeg()
        }
    }
}