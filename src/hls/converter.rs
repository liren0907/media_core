use std::fs;
use std::process::{Command, Stdio};

use crate::hls::config::HLSVodConfig;
use crate::hls::types::HLSError;

/// HLS VOD Converter
///
/// Converts static video files (MP4, MOV, etc.) to HLS format
/// for Video On Demand streaming.
pub struct HLSConverter {
    config: HLSVodConfig,
}

impl HLSConverter {
    /// Create a new HLS converter with the given configuration
    pub fn new(config: HLSVodConfig) -> Self {
        Self { config }
    }

    /// Validate the configuration before conversion
    fn validate(&self) -> Result<(), HLSError> {
        // Check input file exists
        if !self.config.input_path.exists() {
            return Err(HLSError::InvalidInput(format!(
                "Input file does not exist: {}",
                self.config.input_path.display()
            )));
        }

        // Check input is a file
        if !self.config.input_path.is_file() {
            return Err(HLSError::InvalidInput(format!(
                "Input path is not a file: {}",
                self.config.input_path.display()
            )));
        }

        Ok(())
    }

    /// Convert the input video to HLS format
    ///
    /// # Returns
    /// - `Ok(())` if conversion succeeds
    /// - `Err(HLSError)` if validation or FFmpeg fails
    pub fn convert(&self) -> Result<(), HLSError> {
        self.validate()?;

        // Create output directory
        fs::create_dir_all(&self.config.output_dir)?;

        // Build playlist path
        let playlist_path = self.config.output_dir.join(&self.config.playlist_filename);

        // Build FFmpeg command
        let mut command = Command::new("ffmpeg");
        command
            .arg("-y") // Overwrite output
            .arg("-i")
            .arg(&self.config.input_path)
            .arg("-profile:v")
            .arg(&self.config.profile)
            .arg("-level")
            .arg(&self.config.level);

        // Add keyframe forcing if enabled
        if self.config.force_keyframes {
            command.arg("-force_key_frames").arg(format!(
                "expr:gte(t,n_forced*{})",
                self.config.segment_duration
            ));
        }

        // HLS options
        command
            .arg("-start_number")
            .arg("0")
            .arg("-hls_time")
            .arg(self.config.segment_duration.to_string())
            .arg("-hls_flags")
            .arg("independent_segments")
            .arg("-hls_list_size")
            .arg("0") // Keep all segments in playlist (VOD mode)
            .arg("-f")
            .arg("hls")
            .arg(&playlist_path);

        println!("🎬 Starting HLS conversion: {:?}", command);
        println!("   Input: {}", self.config.input_path.display());
        println!("   Output: {}", self.config.output_dir.display());

        // Execute FFmpeg
        let output = command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(HLSError::FFmpegError(format!(
                "FFmpeg failed with status {}: {}",
                output.status, stderr
            )));
        }

        println!("✅ HLS conversion complete!");
        println!("   Playlist: {}", playlist_path.display());

        Ok(())
    }
}
