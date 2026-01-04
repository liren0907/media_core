//! Synchronizer for RTSP streams
//!
//! Provides functions to start synchronized HLS streaming
//! with wall-clock aligned segment boundaries.

use crate::rtsp_sync::types::RtspSyncError;
use chrono::Utc;
use regex::Regex;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

/// Start HLS streaming synchronized to wall clock time boundaries
///
/// This function waits until the next segment boundary (aligned to wall clock)
/// before starting all streams simultaneously. This ensures synchronized
/// playback across multiple camera feeds.
///
/// # Arguments
/// * `rtsp_urls` - List of RTSP stream URLs
/// * `output_dir` - Base output directory for HLS files
/// * `segment_duration` - Duration of each HLS segment in seconds
/// * `playlist_size` - Number of segments to keep in playlist
/// * `with_audio` - Whether to include audio in the stream
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(RtspSyncError)` on failure
pub fn start_ffmpeg_sync_hls(
    rtsp_urls: Vec<String>,
    output_dir: String,
    segment_duration: u64,
    playlist_size: u64,
    with_audio: bool,
) -> Result<(), RtspSyncError> {
    // 1. Calculate delay until the next segment boundary
    let now = Utc::now();
    let timestamp = now.timestamp();

    // We want to start at a multiple of segment_duration
    // e.g., if duration is 15s, start at :00, :15, :30, :45
    let remainder = timestamp % segment_duration as i64;
    let delay_secs = segment_duration as i64 - remainder;

    println!("⏱️  Synchronizing streams...");
    println!("   Current time: {}", now);
    println!("   Segment duration: {}s", segment_duration);
    println!("   Waiting {}s for synchronized start...", delay_secs);

    thread::sleep(Duration::from_secs(delay_secs as u64));

    let start_time = Utc::now();
    println!("🚀 Starting all streams at: {}", start_time);

    // Spawn FFmpeg processes with sync flags
    spawn_hls_processes(
        rtsp_urls,
        output_dir,
        segment_duration,
        playlist_size,
        with_audio,
        true, // sync mode
    )
}

/// Start HLS streaming without synchronization
///
/// Starts HLS streaming immediately without waiting for segment boundary.
///
/// # Arguments
/// * `rtsp_urls` - List of RTSP stream URLs
/// * `output_dir` - Base output directory for HLS files
/// * `segment_duration` - Duration of each HLS segment in seconds
/// * `playlist_size` - Number of segments to keep in playlist
/// * `with_audio` - Whether to include audio in the stream
pub fn start_ffmpeg_hls(
    rtsp_urls: Vec<String>,
    output_dir: String,
    segment_duration: u64,
    playlist_size: u64,
    with_audio: bool,
) -> Result<(), RtspSyncError> {
    println!("🎬 Starting HLS streaming (non-sync mode)...");

    spawn_hls_processes(
        rtsp_urls,
        output_dir,
        segment_duration,
        playlist_size,
        with_audio,
        false, // non-sync mode
    )
}

/// Spawn FFmpeg HLS processes for all URLs
fn spawn_hls_processes(
    rtsp_urls: Vec<String>,
    output_dir: String,
    segment_duration: u64,
    playlist_size: u64,
    with_audio: bool,
    sync_mode: bool,
) -> Result<(), RtspSyncError> {
    let mut handles = Vec::new();

    for url in rtsp_urls {
        let output_dir = output_dir.clone();

        let handle = thread::spawn(move || -> Result<(), RtspSyncError> {
            // Sanitize stream name from URL
            let re = Regex::new(r"[^\w-]").unwrap();
            let stream_name = re.replace_all(&url, "_").to_string();

            // Create per-stream directory
            let stream_dir = format!("{}/{}", output_dir, stream_name);
            std::fs::create_dir_all(&stream_dir)?;

            // Prepare output paths
            let segment_filename = format!("{}_segment_%03d.ts", stream_name);
            let playlist_filename = format!("{}_playlist.m3u8", stream_name);
            let segment_path = format!("{}/{}", stream_dir, segment_filename);
            let playlist_path = format!("{}/{}", stream_dir, playlist_filename);

            println!("   Starting stream: {}", url);
            println!("   Output: {}", stream_dir);

            let mut command = Command::new("ffmpeg");
            command
                .arg("-y")
                .arg("-loglevel")
                .arg("warning")
                .arg("-rtsp_transport")
                .arg("tcp")
                .arg("-i")
                .arg(&url)
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

            let mut child = command
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|e| {
                    RtspSyncError::FFmpegError(format!("Failed to start FFmpeg: {}", e))
                })?;

            // Wait for process
            let status = child
                .wait()
                .map_err(|e| RtspSyncError::FFmpegError(format!("FFmpeg process failed: {}", e)))?;

            if !status.success() {
                return Err(RtspSyncError::FFmpegError(format!(
                    "FFmpeg exited with status: {}",
                    status
                )));
            }

            Ok(())
        });

        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        match handle.join() {
            Ok(result) => {
                if let Err(e) = result {
                    eprintln!("❌ Stream error: {}", e);
                }
            }
            Err(_) => {
                eprintln!("❌ Thread panicked");
            }
        }
    }

    Ok(())
}

/// Start a single FFmpeg HLS stream and return the process handle
///
/// This is useful when you need to manage the process lifecycle manually.
pub fn spawn_single_hls_stream(
    url: &str,
    output_dir: &str,
    segment_duration: u64,
    playlist_size: u64,
    with_audio: bool,
    sync_mode: bool,
) -> Result<Child, RtspSyncError> {
    let re = Regex::new(r"[^\w-]").unwrap();
    let stream_name = re.replace_all(url, "_").to_string();

    let stream_dir = format!("{}/{}", output_dir, stream_name);
    std::fs::create_dir_all(&stream_dir)?;

    let segment_filename = format!("{}_segment_%03d.ts", stream_name);
    let playlist_filename = format!("{}_playlist.m3u8", stream_name);
    let segment_path = format!("{}/{}", stream_dir, segment_filename);
    let playlist_path = format!("{}/{}", stream_dir, playlist_filename);

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

    if with_audio {
        command.arg("-c:a").arg("copy");
    } else {
        command.arg("-an");
    }

    if sync_mode {
        command
            .arg("-fflags")
            .arg("+genpts")
            .arg("-copyts")
            .arg("-start_at_zero");
    }

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
        .map_err(|e| RtspSyncError::FFmpegError(format!("Failed to start FFmpeg: {}", e)))
}
