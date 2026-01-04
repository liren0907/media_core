//! Stream processor for RTSP sync module
//!
//! Provides centralized orchestration for stream processing
//! based on different operation modes.

use crate::rtsp_sync::ffmpeg_utils::FFmpegUtils;
use crate::rtsp_sync::latency::LatencyMonitor;
use crate::rtsp_sync::synchronizer::{start_ffmpeg_hls, start_ffmpeg_sync_hls};
use crate::rtsp_sync::types::{LogMessage, Mode, RtspSyncConfig, RtspSyncError, StreamMetadata};
use chrono::Local;
use prettytable::{Table, format, row};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

/// Stream processor for orchestrating RTSP stream operations
pub struct StreamProcessor {
    config: Arc<RtspSyncConfig>,
    log_messages: Arc<Mutex<Vec<LogMessage>>>,
}

impl StreamProcessor {
    /// Create a new stream processor
    pub fn new(config: RtspSyncConfig) -> Self {
        StreamProcessor {
            config: Arc::new(config),
            log_messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get reference to the configuration
    pub fn config(&self) -> &RtspSyncConfig {
        &self.config
    }

    /// Process streams based on the configured mode
    pub fn process_streams(&self) -> Result<(), RtspSyncError> {
        // Create required directories
        self.config.create_directories()?;

        // Start HLS if enabled
        if self.config.hls.enabled {
            let hls_result = match self.config.mode {
                Mode::Sync => start_ffmpeg_sync_hls(
                    self.config.rtsp_url_list.clone(),
                    self.config.hls.root_directory.clone(),
                    self.config.hls.segment_duration,
                    self.config.hls.playlist_size,
                    self.config.audio,
                ),
                _ => start_ffmpeg_hls(
                    self.config.rtsp_url_list.clone(),
                    self.config.hls.root_directory.clone(),
                    self.config.hls.segment_duration,
                    self.config.hls.playlist_size,
                    self.config.audio,
                ),
            };

            if let Err(e) = hls_result {
                Self::log_error(
                    &self.log_messages,
                    "HLS",
                    &format!("Failed to start HLS: {}", e),
                );
                return Err(e);
            }
            Self::log_info(
                &self.log_messages,
                "HLS",
                "HLS streaming started successfully",
            );
        }

        // Handle specific mode operations
        match self.config.mode {
            Mode::Preview => {
                println!(
                    "📺 Starting preview mode for {} streams",
                    self.config.rtsp_url_list.len()
                );
                for url in &self.config.rtsp_url_list {
                    match FFmpegUtils::get_stream_metadata(url) {
                        Ok(metadata) => {
                            self.display_stream_metadata(&metadata);
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to get metadata for {}: {}", url, e);
                        }
                    }
                }
            }
            Mode::Latency => {
                println!("📊 Running in latency monitoring mode...");
                let monitor = LatencyMonitor::with_config(
                    self.config.rtsp_url_list.clone(),
                    self.config.latency_monitor.monitor_interval_ms,
                    self.config.latency_monitor.display_pts,
                );
                monitor.run();
            }
            Mode::Recording => {
                println!(
                    "🎬 Starting recording mode for {} streams",
                    self.config.rtsp_url_list.len()
                );
                let mut handles = vec![];
                for url in &self.config.rtsp_url_list {
                    handles.push(self.spawn_stream_handler(url.clone()));
                }
                self.monitor_streams(&handles);
            }
            Mode::Sync => {
                // Sync mode is handled by HLS initialization above
                println!("🔄 Sync mode - HLS streaming with synchronized timestamps");
            }
        }

        Ok(())
    }

    /// Spawn a thread to handle a single stream
    fn spawn_stream_handler(&self, url: String) -> JoinHandle<()> {
        let config = Arc::clone(&self.config);
        let log_messages = Arc::clone(&self.log_messages);

        thread::spawn(move || {
            // Create a unique directory for this stream recording
            let sanitized_url = url
                .replace("://", "_")
                .replace('/', "_")
                .replace('@', "_")
                .replace(':', "_");
            let stream_dir = format!("{}/{}", config.output_directory, sanitized_url);

            if let Err(e) = std::fs::create_dir_all(&stream_dir) {
                Self::log_error(
                    &log_messages,
                    &url,
                    &format!("Failed to create directory: {}", e),
                );
                return;
            }

            Self::log_info(&log_messages, &url, "Starting stream recording...");

            match FFmpegUtils::start_recording(&url, &stream_dir, config.audio) {
                Ok(mut child) => {
                    Self::log_info(&log_messages, &url, "Recording started successfully");
                    if let Err(e) = child.wait() {
                        Self::log_error(
                            &log_messages,
                            &url,
                            &format!("Recording process failed: {}", e),
                        );
                    }
                }
                Err(e) => {
                    Self::log_error(
                        &log_messages,
                        &url,
                        &format!("Failed to start recording: {}", e),
                    );
                }
            }
        })
    }

    /// Monitor stream handler threads
    fn monitor_streams(&self, handles: &[JoinHandle<()>]) {
        let mut last_log_count = 0;

        while handles.iter().any(|h| !h.is_finished()) {
            self.display_new_logs(&mut last_log_count);
            thread::sleep(std::time::Duration::from_millis(100));
        }

        // Display any remaining logs after all threads are done
        self.display_new_logs(&mut last_log_count);
    }

    /// Display new log messages since last check
    fn display_new_logs(&self, last_log_count: &mut usize) {
        let logs = self.log_messages.lock().unwrap();
        while *last_log_count < logs.len() {
            let log = &logs[*last_log_count];
            if log.is_error {
                eprintln!("[{}] {} - {}", log.timestamp, log.stream_url, log.message);
            } else {
                println!("[{}] {} - {}", log.timestamp, log.stream_url, log.message);
            }
            *last_log_count += 1;
        }
    }

    /// Display stream metadata in a formatted way
    fn display_stream_metadata(&self, metadata: &StreamMetadata) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        table.add_row(row!["Property", "Value"]);
        table.add_row(row!["URL", &metadata.url]);
        table.add_row(row!["Status", &metadata.status]);
        table.add_row(row!["Resolution", &metadata.resolution]);
        table.add_row(row!["FPS", format!("{:.2}", metadata.fps)]);
        table.add_row(row!["Codec", &metadata.codec]);
        table.add_row(row!["Bitrate", &metadata.bitrate]);

        println!();
        table.printstd();
    }

    /// Log an info message
    fn log_info(log_messages: &Arc<Mutex<Vec<LogMessage>>>, url: &str, message: impl Into<String>) {
        let msg = LogMessage {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            stream_url: url.to_string(),
            message: message.into(),
            is_error: false,
        };
        log_messages.lock().unwrap().push(msg);
    }

    /// Log an error message
    fn log_error(
        log_messages: &Arc<Mutex<Vec<LogMessage>>>,
        url: &str,
        message: impl Into<String>,
    ) {
        let msg = LogMessage {
            timestamp: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            stream_url: url.to_string(),
            message: message.into(),
            is_error: true,
        };
        log_messages.lock().unwrap().push(msg);
    }
}
