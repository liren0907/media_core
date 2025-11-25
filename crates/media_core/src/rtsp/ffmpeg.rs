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
        let mut args = vec![
            "-y".to_string(),
            "-loglevel".to_string(),
            "error".to_string(),
            "-rtsp_transport".to_string(),
            "tcp".to_string(),
            "-use_wallclock_as_timestamps".to_string(),
            "1".to_string(),
            "-i".to_string(),
            self.url.clone(),
            "-c:v".to_string(),
            "copy".to_string(),
            "-an".to_string(),
            "-f".to_string(),
            "segment".to_string(),
            "-segment_time".to_string(),
            self.segment_duration.as_secs().to_string(),
            "-segment_format".to_string(),
            "mp4".to_string(),
            "-reset_timestamps".to_string(),
            "1".to_string(),
            "-segment_format_options".to_string(),
            "movflags=+faststart+frag_keyframe+empty_moov+default_base_moof".to_string(),
            "-segment_time_delta".to_string(),
            "0.05".to_string(),
            "-strftime".to_string(),
            "1".to_string(),
            "-reconnect_at_eof".to_string(),
            "1".to_string(),
            "-reconnect_streamed".to_string(),
            "1".to_string(),
            "-reconnect_delay_max".to_string(),
            "120".to_string(),
        ];

        if self.run_once {
            // Run for slightly longer than one segment to ensure it finishes
            args.push("-t".to_string());
            args.push((self.segment_duration.as_secs() + 5).to_string());
        }

        args.push(output_pattern);

        command.args(&args);

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
                            if self.run_once {
                                return Err(opencv::Error::new(
                                    opencv::core::StsError,
                                    "Failed to start FFmpeg in run_once mode",
                                ));
                            }
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
                        } else if self.run_once {
                            println!("FFmpeg process finished successfully in run_once mode.");
                            return Ok(());
                        }

                        self.ffmpeg_process = None;

                        if self.run_once && !status.success() {
                            return Err(opencv::Error::new(
                                opencv::core::StsError,
                                "FFmpeg process failed in run_once mode",
                            ));
                        }

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
                        if self.run_once {
                            return Err(opencv::Error::new(
                                opencv::core::StsError,
                                &format!("Error checking FFmpeg process: {}", e),
                            ));
                        }
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
