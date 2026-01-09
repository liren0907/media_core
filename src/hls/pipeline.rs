//! HLS Pipeline Steps
//!
//! This module provides pipeline steps for HLS (HTTP Live Streaming) conversion.

use crate::hls::config::HLSVodConfig;
use crate::hls::converter::HLSConverter;
use crate::pipeline::{MediaContext, PipelineError, PipelineStep};
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// HLS RESULT TYPE
// ============================================================================

/// Result of an HLS conversion operation
#[derive(Debug, Clone, Default)]
pub struct HLSResult {
    /// Output directory containing HLS files
    pub output_dir: String,
    /// Path to the generated playlist file
    pub playlist_path: String,
    /// Number of segment files created
    pub segment_count: usize,
    /// Whether conversion was successful
    pub success: bool,
}

// ============================================================================
// CONVERT TO HLS STEP
// ============================================================================

/// A pipeline step that converts a video file to HLS format.
///
/// Wraps `HLSConverter` for use in the pipeline pattern.
///
/// # Example
/// ```rust
/// let step = ConvertToHLS::new("output/hls")
///     .segment_duration(10)
///     .playlist_name("stream.m3u8");
/// ```
pub struct ConvertToHLS {
    output_dir: PathBuf,
    segment_duration: u32,
    playlist_filename: String,
    force_keyframes: bool,
    profile: String,
    level: String,
}

impl ConvertToHLS {
    /// Create a new ConvertToHLS step with the specified output directory.
    pub fn new<P: AsRef<Path>>(output_dir: P) -> Self {
        Self {
            output_dir: output_dir.as_ref().to_path_buf(),
            segment_duration: 5,
            playlist_filename: "playlist.m3u8".to_string(),
            force_keyframes: true,
            profile: "baseline".to_string(),
            level: "3.0".to_string(),
        }
    }

    /// Set the segment duration in seconds (default: 5)
    pub fn segment_duration(mut self, seconds: u32) -> Self {
        self.segment_duration = seconds;
        self
    }

    /// Set the playlist filename (default: "playlist.m3u8")
    pub fn playlist_name(mut self, name: &str) -> Self {
        self.playlist_filename = name.to_string();
        self
    }

    /// Enable or disable keyframe forcing at segment boundaries (default: true)
    pub fn force_keyframes(mut self, enabled: bool) -> Self {
        self.force_keyframes = enabled;
        self
    }

    /// Set the H.264 profile (default: "baseline")
    pub fn profile(mut self, profile: &str) -> Self {
        self.profile = profile.to_string();
        self
    }

    /// Set the H.264 level (default: "3.0")
    pub fn level(mut self, level: &str) -> Self {
        self.level = level.to_string();
        self
    }
}

impl PipelineStep<MediaContext> for ConvertToHLS {
    fn name(&self) -> &str {
        "ConvertToHLS"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        // Get input path from context
        let input_path = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError(
                "ConvertToHLS step requires a File source".to_string(),
            )
        })?;

        // Build HLS configuration
        let config = HLSVodConfig {
            input_path: PathBuf::from(input_path),
            output_dir: self.output_dir.clone(),
            segment_duration: self.segment_duration,
            playlist_filename: self.playlist_filename.clone(),
            force_keyframes: self.force_keyframes,
            profile: self.profile.clone(),
            level: self.level.clone(),
        };

        // Run conversion
        let converter = HLSConverter::new(config);
        converter.convert().map_err(|e| PipelineError::StepFailed {
            step_name: self.name().to_string(),
            error: format!("HLS conversion failed: {}", e),
        })?;

        // Count segment files
        let segment_count = fs::read_dir(&self.output_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension().map(|ext| ext == "ts").unwrap_or(false))
                    .count()
            })
            .unwrap_or(0);

        // Store result in context
        let playlist_path = self.output_dir.join(&self.playlist_filename);
        context.hls_result = Some(HLSResult {
            output_dir: self.output_dir.to_string_lossy().to_string(),
            playlist_path: playlist_path.to_string_lossy().to_string(),
            segment_count,
            success: true,
        });

        Ok(())
    }
}
