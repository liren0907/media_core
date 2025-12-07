use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum SavingOption {
    Single,
    #[default]
    List,
    Both,
}

/// HLS (HTTP Live Streaming) configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HLSConfig {
    pub enabled: bool,
    pub output_directory: String,
    pub segment_duration: u32,
    pub playlist_size: u32,
}

impl Default for HLSConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            output_directory: "hls_output".to_string(),
            segment_duration: 10,
            playlist_size: 5,
        }
    }
}

/// Main RTSP stream configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamConfig {
    pub rtsp_url: String,
    pub rtsp_url_list: Vec<String>,
    pub output_directory: String,
    pub show_preview: bool,
    pub saving_option: SavingOption,
    pub saved_time_duration: u64,
    pub use_fps: bool,
    pub fps: f64,
    pub hls: HLSConfig,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            rtsp_url: "rtsp://username:password@camera-ip:port/stream".to_string(),
            rtsp_url_list: vec![
                "rtsp://username:password@camera1-ip:port/stream".to_string(),
                "rtsp://username:password@camera2-ip:port/stream".to_string(),
            ],
            output_directory: "media".to_string(),
            show_preview: false,
            saving_option: SavingOption::List,
            saved_time_duration: 300,
            use_fps: false,
            fps: 30.0,
            hls: HLSConfig::default(),
        }
    }
}

/// Backward compatibility alias
pub type CaptureConfig = StreamConfig;
