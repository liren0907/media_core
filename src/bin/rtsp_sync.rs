//! RTSP Sync Module Example
//!
//! This example demonstrates how to use the RTSP sync module programmatically.
//! Run with: cargo run --bin rtsp_sync

use media_core::rtsp_sync::{
    FFmpegUtils, HLSSyncConfig, LatencyMonitor, LatencyMonitorConfig, Mode, RtspSyncConfig,
    StreamProcessor,
};
use std::env;
use std::process::Child;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       RTSP Sync Module Example");
    println!("===========================================\n");

    // ============================================
    // 📝 CONFIGURATION
    // ============================================
    let rtsp_urls = vec![
        env::var("RTSP_URL_1").unwrap_or_else(|_| "rtsp://127.0.0.1:8554/stream1".to_string()),
        env::var("RTSP_URL_2").unwrap_or_else(|_| "rtsp://127.0.0.1:8554/stream2".to_string()),
    ];

    // ============================================
    // 🎬 1. Preview Mode - Get Stream Metadata
    // ============================================
    // Preview mode retrieves and displays stream metadata
    // without recording anything.
    println!("🎬 1. Preview Mode - Stream Metadata");
    println!("-------------------------------------");

    for url in &rtsp_urls {
        println!("   Probing: {}", url);
        match FFmpegUtils::get_stream_metadata(url) {
            Ok(metadata) => {
                println!("   ✅ Status: {}", metadata.status);
                println!("      Resolution: {}", metadata.resolution);
                println!("      FPS: {:.2}", metadata.fps);
                println!("      Codec: {}", metadata.codec);
                println!("      Bitrate: {}", metadata.bitrate);
            }
            Err(e) => {
                println!("   ⚠️  Error (expected if no stream): {}", e);
            }
        }
        println!();
    }

    // ============================================
    // 📊 2. Latency Monitoring
    // ============================================
    // Latency monitor checks PTS drift across streams.
    println!("📊 2. Latency Monitoring (single check)");
    println!("----------------------------------------");

    let monitor = LatencyMonitor::with_config(
        rtsp_urls.clone(),
        5000, // monitor interval ms
        true, // display PTS
    );

    // Run a single check instead of continuous loop
    let latency_data = monitor.run_once();
    println!("   Checked {} streams", latency_data.len());
    for (url, info) in latency_data.iter() {
        println!("   - {}: latency={:.3}s", url, info.latency as f64 / 1000.0);
    }
    println!();

    // ============================================
    // 🎥 3. StreamProcessor with Config
    // ============================================
    // StreamProcessor orchestrates all modes based on config.
    println!("🎥 3. StreamProcessor - Preview Mode");
    println!("-------------------------------------");

    let config = RtspSyncConfig {
        mode: Mode::Preview,
        rtsp_url_list: rtsp_urls.clone(),
        output_directory: "output/rtsp_sync".to_string(),
        show_preview: false,
        saved_time_duration: 60,
        audio: false,
        use_fps: false,
        fps: 30.0,
        hls: HLSSyncConfig {
            enabled: false,
            root_directory: "output/hls_sync".to_string(),
            segment_duration: 15,
            playlist_size: 10,
        },
        latency_monitor: LatencyMonitorConfig {
            monitor_interval_ms: 5000,
            display_pts: true,
        },
    };

    let processor = StreamProcessor::new(config);
    match processor.process_streams() {
        Ok(_) => println!("   ✅ Preview mode completed\n"),
        Err(e) => println!("   ⚠️  Preview error: {}\n", e),
    }

    // ============================================
    // 🎬 4. Recording Mode (Time-Limited Demo)
    // ============================================
    // Recording mode captures streams to MP4 files.
    // This demo runs for 10 seconds then stops.
    println!("🎬 4. Recording Mode (10-second demo)");
    println!("--------------------------------------");

    let demo_duration_secs = 10;
    println!(
        "   Starting recording for {} seconds...",
        demo_duration_secs
    );
    println!("   Output directory: output/rtsp_sync/");

    // Start FFmpeg recording processes for each stream
    let mut recording_processes: Vec<Child> = vec![];

    for (i, url) in rtsp_urls.iter().enumerate() {
        let stream_dir = format!("output/rtsp_sync/stream_{}", i + 1);
        std::fs::create_dir_all(&stream_dir)?;

        println!("   📹 Stream {}: {}", i + 1, url);
        match FFmpegUtils::start_recording(url, &stream_dir, false) {
            Ok(child) => {
                recording_processes.push(child);
                println!("      ✅ Recording started");
            }
            Err(e) => {
                println!("      ⚠️  Failed (expected if no stream): {}", e);
            }
        }
    }

    // Wait for demo duration
    if !recording_processes.is_empty() {
        println!("\n   ⏱️  Recording for {} seconds...", demo_duration_secs);
        thread::sleep(Duration::from_secs(demo_duration_secs));

        // Stop all recording processes
        println!("   🛑 Stopping recordings...");
        for mut child in recording_processes {
            let _ = child.kill();
            let _ = child.wait();
        }
        println!("   ✅ Recording demo completed\n");
    } else {
        println!("   ⚠️  No streams were recorded (no active RTSP streams)\n");
    }

    // ============================================
    // 🔄 5. Synchronized HLS Mode
    // ============================================
    // Demonstrates config for synchronized HLS streaming.
    println!("🔄 5. Sync HLS Mode (Config Only)");
    println!("----------------------------------");

    let sync_config = RtspSyncConfig {
        mode: Mode::Sync,
        rtsp_url_list: rtsp_urls.clone(),
        output_directory: "output/rtsp_sync".to_string(),
        show_preview: false,
        saved_time_duration: 300,
        audio: false,
        use_fps: false,
        fps: 30.0,
        hls: HLSSyncConfig {
            enabled: true,
            root_directory: "output/hls_sync".to_string(),
            segment_duration: 15,
            playlist_size: 10,
        },
        latency_monitor: LatencyMonitorConfig::default(),
    };

    println!("   Mode: {:?}", sync_config.mode);
    println!("   HLS Enabled: {}", sync_config.hls.enabled);
    println!("   Segment Duration: {}s", sync_config.hls.segment_duration);
    println!("   Playlist Size: {}", sync_config.hls.playlist_size);
    println!("   Output: {}", sync_config.hls.root_directory);
    println!();
    println!("   ℹ️  To run sync mode, uncomment the processor.process_streams() call below.");
    println!("   ℹ️  This requires active RTSP streams.");

    // Uncomment to actually run sync mode:
    // let processor = StreamProcessor::new(sync_config);
    // processor.process_streams()?;

    // ============================================
    // 💾 6. Save/Load Configuration
    // ============================================
    println!("\n💾 6. Configuration File Operations");
    println!("------------------------------------");

    let config_path = "output/rtsp_sync_config.json";
    std::fs::create_dir_all("output")?;

    // Save config
    sync_config.to_file(config_path)?;
    println!("   ✅ Saved config to: {}", config_path);

    // Load config
    let loaded_config = RtspSyncConfig::from_file(config_path)?;
    println!(
        "   ✅ Loaded config: mode={:?}, streams={}",
        loaded_config.mode,
        loaded_config.rtsp_url_list.len()
    );
    println!();

    // ============================================
    // 📋 Summary
    // ============================================
    println!("===========================================");
    println!("       ✅ All examples completed!");
    println!("===========================================");
    println!("\nAvailable Modes:");
    println!("  - Preview:   Display stream metadata");
    println!("  - Latency:   Monitor PTS and latency");
    println!("  - Recording: Record streams to files");
    println!("  - Sync:      Synchronized HLS streaming");
    println!("\nUsage:");
    println!("  1. Set RTSP_URL_1, RTSP_URL_2 environment variables");
    println!("  2. Create a config file or use RtspSyncConfig::default()");
    println!("  3. Use StreamProcessor::new(config).process_streams()");

    Ok(())
}
