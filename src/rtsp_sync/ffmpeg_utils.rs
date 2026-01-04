//! FFmpeg utilities for RTSP sync module
//!
//! Provides helper functions for FFmpeg operations including
//! metadata extraction and recording.

use crate::rtsp_sync::types::{RtspSyncError, StreamMetadata};
use std::process::{Child, Command, Stdio};

/// FFmpeg utility functions
pub struct FFmpegUtils;

impl FFmpegUtils {
    /// Get stream metadata using ffprobe
    ///
    /// # Arguments
    /// * `url` - RTSP URL to probe
    ///
    /// # Returns
    /// StreamMetadata with resolution, fps, codec, bitrate info
    pub fn get_stream_metadata(url: &str) -> Result<StreamMetadata, RtspSyncError> {
        let mut command = Command::new("ffprobe");
        command.args([
            "-v",
            "quiet",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
            "-rtsp_transport",
            "tcp",
            "-i",
            url,
        ]);

        let output = command
            .output()
            .map_err(|e| RtspSyncError::FFmpegError(format!("Failed to execute ffprobe: {}", e)))?;

        let mut metadata = StreamMetadata {
            url: url.to_string(),
            status: "Unknown".to_string(),
            resolution: "Unknown".to_string(),
            fps: 0.0,
            codec: "Unknown".to_string(),
            bitrate: "Unknown".to_string(),
        };

        if output.status.success() {
            let json_output = String::from_utf8_lossy(&output.stdout);
            let json: serde_json::Value = serde_json::from_str(&json_output).map_err(|e| {
                RtspSyncError::FFmpegError(format!("Failed to parse ffprobe JSON: {}", e))
            })?;

            // Extract bitrate from format
            if let Some(format) = json.get("format") {
                if let Some(bit_rate) = format.get("bit_rate") {
                    let br = bit_rate
                        .as_str()
                        .unwrap_or("0")
                        .parse::<f64>()
                        .unwrap_or(0.0)
                        / 1024.0;
                    metadata.bitrate = format!("{:.2} Kbps", br);
                }
            }

            // Extract video stream info
            if let Some(streams) = json.get("streams") {
                if let Some(streams_array) = streams.as_array() {
                    for stream in streams_array.iter() {
                        if stream.get("codec_type").and_then(|t| t.as_str()) == Some("video") {
                            // Resolution
                            if let (Some(width), Some(height)) =
                                (stream.get("width"), stream.get("height"))
                            {
                                metadata.resolution = format!(
                                    "{}x{}",
                                    width.as_i64().unwrap_or(0),
                                    height.as_i64().unwrap_or(0)
                                );
                            }

                            // FPS
                            if let Some(r_frame_rate) = stream.get("r_frame_rate") {
                                if let Some(rate_str) = r_frame_rate.as_str() {
                                    if let Some((n, d)) = rate_str.split_once('/') {
                                        if let (Ok(num), Ok(den)) =
                                            (n.parse::<f64>(), d.parse::<f64>())
                                        {
                                            if den > 0.0 {
                                                metadata.fps = num / den;
                                            }
                                        }
                                    }
                                }
                            }

                            // Codec
                            if let Some(codec_name) = stream.get("codec_name") {
                                metadata.codec =
                                    codec_name.as_str().unwrap_or("Unknown").to_string();
                            }

                            break;
                        }
                    }
                }
            }

            metadata.status = "Connected".to_string();
            Ok(metadata)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            metadata.status = "Error".to_string();
            Err(RtspSyncError::FFmpegError(format!(
                "Failed to get metadata: {}",
                error
            )))
        }
    }

    /// Start FFmpeg recording process
    ///
    /// # Arguments
    /// * `url` - RTSP URL to record
    /// * `output_directory` - Directory to save recordings
    /// * `with_audio` - Whether to include audio
    ///
    /// # Returns
    /// Child process handle
    pub fn start_recording(
        url: &str,
        output_directory: &str,
        with_audio: bool,
    ) -> Result<Child, RtspSyncError> {
        let output_file = format!("{}/recording_%Y%m%d_%H%M%S.mp4", output_directory);

        let mut command = Command::new("ffmpeg");
        command
            .arg("-y")
            .arg("-loglevel")
            .arg("warning")
            .arg("-rtsp_transport")
            .arg("tcp")
            .arg("-use_wallclock_as_timestamps")
            .arg("1")
            .arg("-i")
            .arg(url)
            .arg("-c:v")
            .arg("copy")
            .arg("-f")
            .arg("segment")
            .arg("-segment_time")
            .arg("3600")
            .arg("-segment_format")
            .arg("mp4")
            .arg("-reset_timestamps")
            .arg("1")
            .arg("-strftime")
            .arg("1")
            .arg("-movflags")
            .arg("+faststart");

        if with_audio {
            command
                .arg("-c:a")
                .arg("aac")
                .arg("-b:a")
                .arg("64k")
                .arg("-aac_coder")
                .arg("twoloop");
        } else {
            command.arg("-an");
        }

        command.arg(&output_file);

        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| RtspSyncError::FFmpegError(format!("Failed to start FFmpeg: {}", e)))
    }

    /// Start HLS streaming for a single stream
    ///
    /// # Arguments
    /// * `url` - RTSP URL to stream
    /// * `output_dir` - HLS output directory
    /// * `stream_name` - Sanitized stream name for filenames
    /// * `segment_duration` - Duration of each segment
    /// * `playlist_size` - Number of segments to keep
    /// * `with_audio` - Whether to include audio
    /// * `sync_mode` - Whether to use sync flags (-copyts, etc.)
    pub fn start_hls_stream(
        url: &str,
        output_dir: &str,
        stream_name: &str,
        segment_duration: u64,
        playlist_size: u64,
        with_audio: bool,
        sync_mode: bool,
    ) -> Result<Child, RtspSyncError> {
        let segment_filename = format!("{}_segment_%03d.ts", stream_name);
        let playlist_filename = format!("{}_playlist.m3u8", stream_name);
        let segment_path = format!("{}/{}", output_dir, segment_filename);
        let playlist_path = format!("{}/{}", output_dir, playlist_filename);

        let mut command = Command::new("ffmpeg");
        command
            .arg("-y")
            .arg("-loglevel")
            .arg("warning")
            .arg("-rtsp_transport")
            .arg("tcp")
            .arg("-i")
            .arg(url)
            .arg("-c:v")
            .arg("copy");

        // Audio handling
        if with_audio {
            command.arg("-c:a").arg("copy");
        } else {
            command.arg("-an");
        }

        // Sync mode flags for aligned timestamps
        if sync_mode {
            command
                .arg("-fflags")
                .arg("+genpts")
                .arg("-copyts")
                .arg("-start_at_zero");
        }

        // HLS options
        command
            .arg("-f")
            .arg("hls")
            .arg("-hls_time")
            .arg(segment_duration.to_string())
            .arg("-hls_list_size")
            .arg(playlist_size.to_string())
            .arg("-hls_flags")
            .arg("delete_segments")
            .arg("-hls_segment_filename")
            .arg(&segment_path)
            .arg(&playlist_path);

        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| RtspSyncError::FFmpegError(format!("Failed to start HLS stream: {}", e)))
    }
}
