use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for HLS VOD (Video On Demand) conversion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HLSVodConfig {
    /// Path to the input video file
    pub input_path: PathBuf,
    /// Directory where HLS output will be saved
    pub output_dir: PathBuf,
    /// Duration of each HLS segment in seconds (default: 5)
    pub segment_duration: u32,
    /// Name of the playlist file (default: "playlist.m3u8")
    pub playlist_filename: String,
    /// Force keyframes at segment boundaries for precise cuts
    pub force_keyframes: bool,
    /// H.264 profile: "baseline", "main", or "high"
    pub profile: String,
    /// H.264 level: "3.0", "4.0", "4.1", etc.
    pub level: String,
}

impl Default for HLSVodConfig {
    fn default() -> Self {
        Self {
            input_path: PathBuf::new(),
            output_dir: PathBuf::from("hls_output"),
            segment_duration: 5,
            playlist_filename: "playlist.m3u8".to_string(),
            force_keyframes: true,
            profile: "baseline".to_string(),
            level: "3.0".to_string(),
        }
    }
}

impl HLSVodConfig {
    /// Create a new config with input and output paths
    pub fn new(input_path: PathBuf, output_dir: PathBuf) -> Self {
        Self {
            input_path,
            output_dir,
            ..Default::default()
        }
    }

    /// Load config from a JSON file
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
