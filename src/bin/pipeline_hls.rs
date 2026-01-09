//! Pipeline HLS Example
//!
//! This example demonstrates the Unified Media Pipeline for HLS conversion.
//! It showcases ALL features of the HLS module through the pipeline pattern:
//!
//! 1. Basic HLS conversion with default settings
//! 2. Custom segment duration and playlist name
//! 3. Combined metadata + HLS pipeline with custom profile/level
//! 4. Disable force keyframes option
//!
//! Run with: cargo run --bin pipeline_hls

use media_core::hls::pipeline::ConvertToHLS;
use media_core::metadata::pipeline::ExtractMetadata;
use media_core::pipeline::{MediaContext, Pipeline};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline HLS Example");
    println!("===========================================\n");

    let video_path = "data/test.mp4";
    let output_base = "output/pipeline_hls";

    // Check if test file exists
    if !Path::new(video_path).exists() {
        eprintln!("⚠️  Test video not found: {}", video_path);
        eprintln!("   Place a test video at 'data/test.mp4' to run this example.");
        return Ok(());
    }

    // Clean up previous output
    if Path::new(output_base).exists() {
        std::fs::remove_dir_all(output_base)?;
    }

    // ============================================
    // 1. Basic HLS Conversion
    // ============================================
    println!("🚀 1. Basic HLS Conversion (default settings)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(ConvertToHLS::new(format!("{}/basic", output_base)));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(hls) = &result.hls_result {
        println!("   ✅ Output: {}", hls.output_dir);
        println!("   Playlist: {}", hls.playlist_path);
        println!("   Segments: {}", hls.segment_count);
    }
    println!();

    // ============================================
    // 2. Custom Segment Duration
    // ============================================
    println!("🚀 2. Custom Segment Duration (10 seconds)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ConvertToHLS::new(format!("{}/custom_duration", output_base))
            .segment_duration(10)
            .playlist_name("stream.m3u8"),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(hls) = &result.hls_result {
        println!("   ✅ Output: {}", hls.output_dir);
        println!("   Playlist: {}", hls.playlist_path);
        println!("   Segments: {}", hls.segment_count);
    }
    println!();

    // ============================================
    // 3. Combined Pipeline: Metadata + HLS
    // ============================================
    println!("🚀 3. Combined Pipeline: Metadata + HLS");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new()
        .add_node(ExtractMetadata::new(false))
        .add_node(
            ConvertToHLS::new(format!("{}/combined", output_base))
                .segment_duration(5)
                .profile("main")
                .level("4.0"),
        );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(meta) = &result.metadata {
        println!("   📹 Video Info:");
        println!("      Resolution: {}", meta.resolution);
        println!("      FPS: {:?}", meta.fps);
    }

    if let Some(hls) = &result.hls_result {
        println!("   📦 HLS Output:");
        println!("      Directory: {}", hls.output_dir);
        println!("      Playlist: {}", hls.playlist_path);
        println!("      Segments: {}", hls.segment_count);
    }
    println!();

    // ============================================
    // 4. Disable Force Keyframes
    // ============================================
    println!("🚀 4. Disable Force Keyframes (faster encoding)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ConvertToHLS::new(format!("{}/no_keyframes", output_base))
            .segment_duration(5)
            .force_keyframes(false),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(hls) = &result.hls_result {
        println!("   ✅ Output: {}", hls.output_dir);
        println!("   Playlist: {}", hls.playlist_path);
        println!("   Segments: {}", hls.segment_count);
    }
    println!();

    println!("===========================================");
    println!("       ✅ All HLS Examples Completed!");
    println!("===========================================");

    Ok(())
}
