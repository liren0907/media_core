use super::CaptureConfig;
use serde_json;
use std::fs::File;
use std::io::Write;

/// Generate a default configuration file for RTSP capture
pub fn generate_default_config(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = CaptureConfig::default();
    let json = serde_json::to_string_pretty(&config)?;

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
