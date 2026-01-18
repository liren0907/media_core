//! Pipeline HLS Example
//!
//! This example demonstrates the Unified Media Pipeline for HLS conversion.
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

    // 1. Basic HLS Conversion
    // Converts video to HLS with default settings (5s segments, baseline profile).
    println!("🚀 1. Basic HLS Conversion");

    let pipeline = Pipeline::new().add_node(ConvertToHLS::new(format!("{}/basic", output_base)));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(hls) = &result.hls_result {
        println!(
            "   ✅ Segments={} | Playlist={} | Output={}",
            hls.segment_count, hls.playlist_path, hls.output_dir
        );
    }

    // 2. Custom Segment Duration
    // Sets 10-second segments with custom playlist name.
    println!("🚀 2. Custom Segment Duration (10s)");

    let pipeline = Pipeline::new().add_node(
        ConvertToHLS::new(format!("{}/custom_duration", output_base))
            .segment_duration(10)
            .playlist_name("stream.m3u8"),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(hls) = &result.hls_result {
        println!(
            "   ✅ Segments={} | Playlist={} | Output={}",
            hls.segment_count, hls.playlist_path, hls.output_dir
        );
    }

    // 3. Combined Pipeline: Metadata + HLS
    // Extracts metadata first, then converts with custom profile/level.
    println!("🚀 3. Combined Pipeline: Metadata + HLS");

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
        println!("   📹 Resolution={} | FPS={:?}", meta.resolution, meta.fps);
    }
    if let Some(hls) = &result.hls_result {
        println!(
            "   ✅ Segments={} | Playlist={} | Output={}",
            hls.segment_count, hls.playlist_path, hls.output_dir
        );
    }

    // 4. Disable Force Keyframes
    // Faster encoding but less precise segment boundaries.
    println!("🚀 4. Disable Force Keyframes");

    let pipeline = Pipeline::new().add_node(
        ConvertToHLS::new(format!("{}/no_keyframes", output_base))
            .segment_duration(5)
            .force_keyframes(false),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(hls) = &result.hls_result {
        println!(
            "   ✅ Segments={} | Playlist={} | Output={}",
            hls.segment_count, hls.playlist_path, hls.output_dir
        );
    }

    Ok(())
}
