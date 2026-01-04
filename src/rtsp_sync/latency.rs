//! Latency monitoring for RTSP streams
//!
//! Monitors PTS (Presentation Time Stamp) and calculates latency
//! for multiple RTSP streams.

use crate::rtsp_sync::types::TimeInfo;
use chrono::Local;
use opencv::{prelude::*, videoio};
use prettytable::{Table, format, row};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Monitor RTSP stream latency and PTS drift
pub struct LatencyMonitor {
    rtsp_url_list: Vec<String>,
    latency_data: Arc<Mutex<HashMap<String, TimeInfo>>>,
    monitor_interval_ms: u64,
    display_pts: bool,
}

impl LatencyMonitor {
    /// Create a new latency monitor with default settings
    pub fn new(rtsp_url_list: Vec<String>) -> Self {
        LatencyMonitor {
            rtsp_url_list,
            latency_data: Arc::new(Mutex::new(HashMap::new())),
            monitor_interval_ms: 10000,
            display_pts: true,
        }
    }

    /// Create a new latency monitor with custom configuration
    pub fn with_config(
        rtsp_url_list: Vec<String>,
        monitor_interval_ms: u64,
        display_pts: bool,
    ) -> Self {
        LatencyMonitor {
            rtsp_url_list,
            latency_data: Arc::new(Mutex::new(HashMap::new())),
            monitor_interval_ms,
            display_pts,
        }
    }

    /// Run the latency monitor (blocking)
    pub fn run(&self) {
        println!("📊 Starting latency monitor...");
        println!("   Streams: {}", self.rtsp_url_list.len());
        println!("   Interval: {}ms", self.monitor_interval_ms);

        loop {
            self.monitor_latency();
            self.display_table();
            thread::sleep(Duration::from_millis(self.monitor_interval_ms));
        }
    }

    /// Run the latency monitor for a single iteration (non-blocking)
    pub fn run_once(&self) -> HashMap<String, TimeInfo> {
        self.monitor_latency();
        self.display_table();
        self.latency_data.lock().unwrap().clone()
    }

    /// Monitor latency for all streams
    fn monitor_latency(&self) {
        let mut data = self.latency_data.lock().unwrap();

        for url in &self.rtsp_url_list {
            match self.get_pts_and_local_time(url) {
                Ok(time_info) => {
                    data.insert(url.clone(), time_info);
                }
                Err(e) => {
                    eprintln!("❌ Failed to get PTS for {}: {}", url, e);
                    data.remove(url);
                }
            }
        }
    }

    /// Get PTS and local time for a stream
    fn get_pts_and_local_time(&self, url: &str) -> Result<TimeInfo, String> {
        // Open a temporary capture to inspect the latest frame
        let mut cap = videoio::VideoCapture::from_file(url, videoio::CAP_ANY as i32)
            .map_err(|e| format!("Failed to open stream: {}", e))?;

        if !cap.is_opened().map_err(|e| e.to_string())? {
            return Err("Stream not opened".to_string());
        }

        let mut frame = opencv::core::Mat::default();
        if !cap.read(&mut frame).map_err(|e| e.to_string())? {
            return Err("Failed to read frame".to_string());
        }

        // Get PTS (Presentation Time Stamp) in milliseconds
        let pts_msec = cap
            .get(videoio::CAP_PROP_POS_MSEC)
            .map_err(|e| e.to_string())?;

        // Current wall clock time
        let now = Local::now();
        let local_time_ms = now.timestamp_millis();

        // Convert PTS to milliseconds (integer)
        let pts_ms = pts_msec as i64;

        // Calculate latency (Wall Clock - PTS)
        // Note: This assumes PTS is somewhat synchronized with wall clock,
        // which depends on the RTSP source implementation.
        let latency_ms = local_time_ms - pts_ms;

        Ok(TimeInfo {
            stream_url: url.to_string(),
            pts: pts_ms,
            local_time: now,
            latency: latency_ms,
        })
    }

    /// Display latency data in a table
    fn display_table(&self) {
        let data = self.latency_data.lock().unwrap();

        if data.is_empty() {
            println!("⚠️  No stream data available");
            return;
        }

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        if self.display_pts {
            table.set_titles(row![
                b->"Stream URL",
                b->"PTS (s)",
                b->"Local Time (s)",
                b->"Latency (s)"
            ]);

            for (url, info) in data.iter() {
                let pts_secs = info.pts as f64 / 1000.0;
                let local_time_secs = info.local_time.timestamp_millis() as f64 / 1000.0;
                let latency_secs = info.latency as f64 / 1000.0;

                // Truncate URL for display
                let display_url = if url.len() > 50 {
                    format!("...{}", &url[url.len() - 47..])
                } else {
                    url.clone()
                };

                table.add_row(row![
                    display_url,
                    format!("{:.3}", pts_secs),
                    format!("{:.3}", local_time_secs),
                    format!("{:.3}", latency_secs)
                ]);
            }
        } else {
            table.set_titles(row![b->"Stream URL", b->"Latency (s)"]);

            for (url, info) in data.iter() {
                let latency_secs = info.latency as f64 / 1000.0;

                let display_url = if url.len() > 50 {
                    format!("...{}", &url[url.len() - 47..])
                } else {
                    url.clone()
                };

                table.add_row(row![display_url, format!("{:.3}", latency_secs)]);
            }
        }

        // Clear screen and print table (ANSI escape codes)
        print!("\x1B[2J\x1B[1;1H");
        println!("📊 RTSP Stream Latency Monitor");
        println!("   Last updated: {}", Local::now().format("%H:%M:%S"));
        println!();
        table.printstd();
    }

    /// Get current latency data
    pub fn get_latency_data(&self) -> HashMap<String, TimeInfo> {
        self.latency_data.lock().unwrap().clone()
    }
}
