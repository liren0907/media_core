use crate::rtsp::capture::RTSPCapture;
use opencv::Result;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

impl RTSPCapture {
    pub fn start_ffmpeg_recording(&mut self) -> std::io::Result<()> {
        let camera_dir = PathBuf::from(&self.output_dir).join(format!(
            "camera_{}",
            self.url
                .replace("://", "_")
                .replace("/", "_")
                .replace(":", "_")
        ));
        fs::create_dir_all(&camera_dir)?;

        let output_pattern = camera_dir
            .join("segment_%Y%m%d_%H%M%S.mp4")
            .to_str()
            .unwrap()
            .to_string();

        let mut command = Command::new("ffmpeg");
        command.args([
            "-y",
            "-loglevel",
            "error",
            "-rtsp_transport",
            "tcp",
            "-use_wallclock_as_timestamps",
            "1",
            "-i",
            &self.url,
            "-c:v",
            "copy",
            "-an",
            "-f",
            "segment",
            "-segment_time",
            &self.segment_duration.as_secs().to_string(),
            "-segment_format",
            "mp4",
            "-reset_timestamps",
            "1",
            "-segment_format_options",
            "movflags=+faststart+frag_keyframe+empty_moov+default_base_moof",
            "-segment_time_delta",
            "0.05",
            "-strftime",
            "1",
            "-reconnect_at_eof",
            "1",
            "-reconnect_streamed",
            "1",
            "-reconnect_delay_max",
            "120",
            &output_pattern,
        ]);

        println!("Starting FFmpeg with command: {:?}", command);

        let process = command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        self.ffmpeg_process = Some(process);
        Ok(())
    }

    pub fn process_stream_ffmpeg(&mut self) -> Result<()> {
        let mut consecutive_failures = 0;
        let max_failures = 3;

        loop {
            if self.ffmpeg_process.is_none() {
                match self.start_ffmpeg_recording() {
                    Ok(_) => {
                        println!("Successfully started FFmpeg process for {}", self.url);
                        consecutive_failures = 0;
                    }
                    Err(e) => {
                        eprintln!("Failed to start FFmpeg for {}: {}", self.url, e);
                        consecutive_failures += 1;
                        if consecutive_failures >= max_failures {
                            thread::sleep(Duration::from_secs(10));
                        } else {
                            thread::sleep(Duration::from_secs(1));
                        }
                        continue;
                    }
                }
            }

            if let Some(process) = &mut self.ffmpeg_process {
                match process.try_wait() {
                    Ok(Some(status)) => {
                        println!(
                            "FFmpeg process for {} ended with status: {}",
                            self.url, status
                        );
                        if !status.success() {
                            eprintln!("FFmpeg process failed for {}, restarting...", self.url);
                            consecutive_failures += 1;
                        }
                        self.ffmpeg_process = None;

                        if consecutive_failures >= max_failures {
                            thread::sleep(Duration::from_secs(10));
                        } else {
                            thread::sleep(Duration::from_secs(1));
                        }
                    }
                    Ok(None) => {
                        consecutive_failures = 0;
                        thread::sleep(Duration::from_secs(1));
                    }
                    Err(e) => {
                        eprintln!("Error checking FFmpeg process for {}: {}", self.url, e);
                        self.ffmpeg_process = None;
                        consecutive_failures += 1;
                        if consecutive_failures >= max_failures {
                            thread::sleep(Duration::from_secs(10));
                        } else {
                            thread::sleep(Duration::from_secs(1));
                        }
                    }
                }
            }
        }
    }
}