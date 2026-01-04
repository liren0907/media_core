//! RTSP Capture Module Example
//!
//! This example demonstrates how to use the RTSP capture module programmatically.
//! Run with: cargo run --bin rtsp

use media_core::rtsp::capture::RTSPCapture;
use std::env;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       RTSP Capture Module Example");
    println!("===========================================\n");

    // ============================================
    // 📝 CONFIGURATION
    // ============================================
    let rtsp_url =
        env::var("RTSP_URL").unwrap_or_else(|_| "rtsp://127.0.0.1:8554/stream".to_string());

    // ============================================
    // 🎬 1. Single Camera - FFmpeg Mode (Default)
    // ============================================
    // FFmpeg mode uses stream copy (no re-encoding) for low CPU usage.
    // This is the recommended mode for most use cases.
    println!("🎬 1. Single Camera - FFmpeg Mode (Default)");
    println!("--------------------------------------------");

    let output_dir = "output/rtsp_ffmpeg";
    std::fs::create_dir_all(output_dir)?;

    let mut capture = RTSPCapture::new(
        rtsp_url.clone(),
        output_dir.to_string(),
        false, // show_preview
        10,    // segment_duration (seconds)
        false, // use_custom_fps = false → FFmpeg mode
        30.0,  // custom_fps (ignored in FFmpeg mode)
        None,  // No HLS config
        true,  // run_once = true for example
    )?;

    println!("   URL: {}", rtsp_url);
    println!("   Output: {}", output_dir);
    println!("   Mode: FFmpeg (stream copy)");

    match capture.process_stream() {
        Ok(_) => println!("   ✅ FFmpeg capture completed\n"),
        Err(e) => println!(
            "   ⚠️  FFmpeg capture error (expected if no stream): {}\n",
            e
        ),
    }

    // ============================================
    // 🎬 2. Multi-Camera - Parallel Capture
    // ============================================
    // Demonstrates capturing from multiple RTSP streams simultaneously.
    // Each camera runs in its own thread.
    println!("🎬 2. Multi-Camera - Parallel Capture");
    println!("--------------------------------------");

    let rtsp_urls = vec![
        env::var("RTSP_URL_1").unwrap_or_else(|_| "rtsp://127.0.0.1:8554/stream1".to_string()),
        env::var("RTSP_URL_2").unwrap_or_else(|_| "rtsp://127.0.0.1:8554/stream2".to_string()),
    ];

    let output_dir_multi = "output/rtsp_multi";
    std::fs::create_dir_all(output_dir_multi)?;

    println!("   Cameras: {} streams", rtsp_urls.len());
    for (i, url) in rtsp_urls.iter().enumerate() {
        println!("   - Camera {}: {}", i + 1, url);
    }
    println!("   Output: {}", output_dir_multi);

    let handles: Vec<_> = rtsp_urls
        .into_iter()
        .enumerate()
        .map(|(idx, url)| {
            let output = output_dir_multi.to_string();
            thread::spawn(move || {
                let result = RTSPCapture::new(
                    url.clone(),
                    output,
                    false, // show_preview
                    10,    // segment_duration
                    false, // FFmpeg mode
                    30.0,
                    None,
                    true, // run_once
                );

                match result {
                    Ok(mut capture) => {
                        if let Err(e) = capture.process_stream() {
                            eprintln!("   Camera {} error: {}", idx + 1, e);
                        }
                    }
                    Err(e) => eprintln!("   Camera {} init error: {}", idx + 1, e),
                }
            })
        })
        .collect();

    // Wait for all camera threads to complete
    for handle in handles {
        let _ = handle.join();
    }
    println!("   ✅ Multi-camera capture completed\n");

    // ============================================
    // 🎬 3. Single Camera - OpenCV Mode (Custom FPS)
    // ============================================
    // OpenCV mode allows custom FPS and optional live preview.
    // Uses frame-by-frame processing (higher CPU than FFmpeg).
    println!("🎬 3. Single Camera - OpenCV Mode (Custom FPS)");
    println!("-----------------------------------------------");

    let output_dir_opencv = "output/rtsp_opencv";
    std::fs::create_dir_all(output_dir_opencv)?;

    let custom_fps = 15.0; // Custom frame rate

    let mut capture_opencv = RTSPCapture::new(
        rtsp_url.clone(),
        output_dir_opencv.to_string(),
        false,      // show_preview (set to true for live window)
        10,         // segment_duration
        true,       // use_custom_fps = true → OpenCV mode
        custom_fps, // custom FPS value
        None,
        true, // run_once
    )?;

    println!("   URL: {}", rtsp_url);
    println!("   Output: {}", output_dir_opencv);
    println!("   Mode: OpenCV (custom FPS)");
    println!("   FPS: {}", custom_fps);

    match capture_opencv.process_stream() {
        Ok(_) => println!("   ✅ OpenCV capture completed\n"),
        Err(e) => println!(
            "   ⚠️  OpenCV capture error (expected if no stream): {}\n",
            e
        ),
    }

    // ============================================
    // 📋 Summary
    // ============================================
    println!("===========================================");
    println!("       ✅ All examples completed!");
    println!("===========================================");
    println!("\nNotes:");
    println!("  - Set RTSP_URL env var for single camera examples");
    println!("  - Set RTSP_URL_1, RTSP_URL_2 for multi-camera example");
    println!("  - Set run_once=false in code for continuous capture");
    println!("  - Set show_preview=true in OpenCV mode for live window");

    Ok(())
}
