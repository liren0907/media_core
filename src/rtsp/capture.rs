use crate::rtsp::types::HLSConfig;
use opencv::{Result, videoio};
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
    pub hls_config: Option<HLSConfig>,
    pub run_once: bool,
}

impl RTSPCapture {
    pub fn new(
        url: String,
        output_dir: String,
        show_preview: bool,
        segment_duration_secs: u64,
        use_custom_fps: bool,
        custom_fps: f64,
        hls_config: Option<HLSConfig>,
        run_once: bool,
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
            hls_config,
            run_once,
        })
    }

    pub fn process_stream(&mut self) -> Result<()> {
        // Priority 1: Check HLS mode first
        if let Some(ref config) = self.hls_config {
            if config.enabled {
                self.start_hls_streaming().map_err(|e| {
                    opencv::Error::new(
                        opencv::core::StsError,
                        &format!("Failed to start HLS streaming: {}", e),
                    )
                })?;
                return self.process_stream_hls();
            }
        }

        // Priority 2: OpenCV mode with custom FPS
        if self.use_custom_fps {
            self.start_opencv_recording()?;
            self.process_stream_opencv()
        }
        // Priority 3: FFmpeg mode (default)
        else {
            self.start_ffmpeg_recording().map_err(|e| {
                opencv::Error::new(
                    opencv::core::StsError,
                    &format!("Failed to start FFmpeg: {}", e),
                )
            })?;
            self.process_stream_ffmpeg()
        }
    }
}
