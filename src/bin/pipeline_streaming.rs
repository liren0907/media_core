//! Pipeline Streaming Example
//!
//! This example demonstrates the Unified Media Pipeline where:
//! 1. Metadata is extracted first.
//! 2. Frames are extracted using a sampling strategy.
//! 3. All data is accessible via the shared MediaContext.

use media_core::pipeline::{Pipeline, MediaContext};
use media_core::metadata::pipeline::ExtractMetadata;
use media_core::streaming::pipeline::ExtractFrames;
use media_core::streaming::SamplingStrategy;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline Streaming Example");
    println!("===========================================\n");

    let video_path = "data/test.mp4";

    // Check if test file exists
    if !Path::new(video_path).exists() {
        eprintln!("⚠️  Test video not found: {}", video_path);
        return Ok(());
    }

    // 1. Configure the Pipeline
    //    We chain multiple steps together. Each step modifies the Context.
    println!("🔧 Building Pipeline...");
    let pipeline = Pipeline::new()
        // Step A: Get Metadata (Resolution, FPS, Duration)
        .add_step(ExtractMetadata::new(false)) // false = no thumbnail
        // Step B: Extract Frames (Every 30th frame -> approx 1 fps for 30fps video)
        .add_step(ExtractFrames::new(SamplingStrategy::EveryNth(30)));

    // 2. Initialize Context
    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    println!("📂 Input Source: {:?}", context.source);

    // 3. Execute
    println!("🚀 Executing Pipeline...");
    let result_context = pipeline.execute(context)?;

    // 4. Inspect Results from the Context
    println!("\n✅ Pipeline Completed Successfully!");
    
    // --- Inspect Metadata ---
    if let Some(meta) = result_context.metadata {
        println!("\n📊 [Metadata Result]");
        println!("   Resolution: {}", meta.resolution);
        println!("   FPS:        {:?}", meta.fps.unwrap_or(0.0));
        println!("   Duration:   {} sec", meta.duration_seconds.unwrap_or(0.0));
        println!("   Codec:      {:?}", meta.codec_name.unwrap_or_default());
    } else {
        println!("\n⚠️  No Metadata extracted!");
    }

    // --- Inspect Extracted Frames ---
    let frames = result_context.extracted_frames;
    println!("\n🎞️  [Streaming Result]");
    println!("   Extracted Frames: {}", frames.len());

    if !frames.is_empty() {
        println!("   Sample Frames:");
        for (i, frame) in frames.iter().take(5).enumerate() {
            println!(
                "     {}. Frame Index: {} (Data size: {} bytes)", 
                i + 1, 
                frame.index, 
                frame.data.len()
            );
        }
        if frames.len() > 5 {
            println!("     ... and {} more.", frames.len() - 5);
        }
    }

    Ok(())
}

