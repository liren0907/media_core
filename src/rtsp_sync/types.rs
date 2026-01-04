//! Types for RTSP Sync module
//!
//! Contains data models for stream synchronization, latency monitoring,
//! and configuration.

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Operation mode for the RTSP sync processor
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Preview mode - display stream metadata without recording
    #[default]
    Preview,
    /// Latency mode - monitor PTS and latency
    Latency,
    /// Recording mode - record streams to files
    Recording,
    /// Sync mode - synchronized HLS streaming
    Sync,
}

/// Stream metadata information
#[derive(Debug, Clone, Default)]
pub struct StreamMetadata {
    /// RTSP URL of the stream
    pub url: String,
    /// Connection status
    pub status: String,
    /// Video resolution (e.g., "1920x1080")
    pub resolution: String,
    /// Frame rate
    pub fps: f64,
    /// Video codec (e.g., "h264")
    pub codec: String,
    /// Stream bitrate
    pub bitrate: String,
}

/// Time information for latency monitoring
#[derive(Debug, Clone)]
pub struct TimeInfo {
    /// RTSP URL of the stream
    pub stream_url: String,
    /// Presentation Time Stamp in milliseconds
    pub pts: i64,
    /// Local system time
    pub local_time: DateTime<Local>,
    /// Calculated latency in milliseconds
    pub latency: i64,
}

/// Log message for stream processing
#[derive(Debug, Clone)]
pub struct LogMessage {
    /// Timestamp of the log
    pub timestamp: String,
    /// Stream URL associated with the log
    pub stream_url: String,
    /// Log message content
    pub message: String,
    /// Whether this is an error
    pub is_error: bool,
}

/// HLS synchronization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HLSSyncConfig {
    /// Whether HLS is enabled
    pub enabled: bool,
    /// Root directory for HLS output
    pub root_directory: String,
    /// Duration of each segment in seconds
    pub segment_duration: u64,
    /// Number of segments to keep in playlist
    pub playlist_size: u64,
}

impl Default for HLSSyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            root_directory: "hls_streams".to_string(),
            segment_duration: 15,
            playlist_size: 10,
        }
    }
}

/// Latency monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMonitorConfig {
    /// Monitoring interval in milliseconds
    pub monitor_interval_ms: u64,
    /// Whether to display PTS information
    pub display_pts: bool,
}

impl Default for LatencyMonitorConfig {
    fn default() -> Self {
        Self {
            monitor_interval_ms: 5000,
            display_pts: true,
        }
    }
}

/// Main configuration for RTSP sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtspSyncConfig {
    /// Operation mode
    pub mode: Mode,
    /// List of RTSP URLs to process
    pub rtsp_url_list: Vec<String>,
    /// Output directory for recordings
    pub output_directory: String,
    /// Whether to show preview window
    pub show_preview: bool,
    /// Duration to save in seconds
    pub saved_time_duration: u64,
    /// Whether to include audio
    pub audio: bool,
    /// Whether to use custom FPS
    pub use_fps: bool,
    /// Custom FPS value
    pub fps: f64,
    /// HLS configuration
    #[serde(default)]
    pub hls: HLSSyncConfig,
    /// Latency monitor configuration
    #[serde(default)]
    pub latency_monitor: LatencyMonitorConfig,
}

impl Default for RtspSyncConfig {
    fn default() -> Self {
        Self {
            mode: Mode::Preview,
            rtsp_url_list: vec![
                "rtsp://username:password@camera1-ip:port/stream".to_string(),
                "rtsp://username:password@camera2-ip:port/stream".to_string(),
            ],
            output_directory: "rtsp_recordings".to_string(),
            show_preview: false,
            saved_time_duration: 300,
            audio: false,
            use_fps: false,
            fps: 30.0,
            hls: HLSSyncConfig::default(),
            latency_monitor: LatencyMonitorConfig::default(),
        }
    }
}

impl RtspSyncConfig {
    /// Load configuration from a JSON file
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let config_str = std::fs::read_to_string(path)?;
        serde_json::from_str(&config_str)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Save configuration to a JSON file
    pub fn to_file(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }

    /// Create required directories
    pub fn create_directories(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.output_directory)?;

        if self.hls.enabled {
            std::fs::create_dir_all(&self.hls.root_directory)?;
        }

        Ok(())
    }

    /// Generate a default configuration file
    pub fn generate_default_config(path: &str) -> std::io::Result<()> {
        let config = Self::default();
        config.to_file(path)
    }
}

/// Error types for RTSP sync operations
#[derive(Debug)]
pub enum RtspSyncError {
    /// I/O error
    IoError(String),
    /// FFmpeg process error
    FFmpegError(String),
    /// OpenCV error
    OpenCVError(String),
    /// Configuration error
    ConfigError(String),
}

impl std::fmt::Display for RtspSyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RtspSyncError::IoError(msg) => write!(f, "I/O Error: {}", msg),
            RtspSyncError::FFmpegError(msg) => write!(f, "FFmpeg Error: {}", msg),
            RtspSyncError::OpenCVError(msg) => write!(f, "OpenCV Error: {}", msg),
            RtspSyncError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
        }
    }
}

impl std::error::Error for RtspSyncError {}

impl From<std::io::Error> for RtspSyncError {
    fn from(err: std::io::Error) -> Self {
        RtspSyncError::IoError(err.to_string())
    }
}

impl From<opencv::Error> for RtspSyncError {
    fn from(err: opencv::Error) -> Self {
        RtspSyncError::OpenCVError(err.to_string())
    }
}
