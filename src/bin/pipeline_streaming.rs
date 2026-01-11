//! Pipeline Streaming Example
//!
//! This example demonstrates the Unified Media Pipeline for streaming extraction.
//! It showcases ALL features of the streaming module through the pipeline pattern:
//!
//! 1. Multiple sampling strategies (EveryNth, FirstN, Range, KeyFrames, Custom)
//! 2. Scale factor support for frame resizing
//! 3. ExtractionMode switching (Sequential/Random)
//! 4. Metadata extraction integration
//!
//! Run with: cargo run --bin pipeline_streaming

use media_core::metadata::pipeline::ExtractMetadata;
use media_core::pipeline::{MediaContext, Pipeline};
use media_core::streaming::pipeline::ExtractFrames;
use media_core::streaming::{ExtractionMode, SamplingStrategy};
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

    // ============================================
    // 1. Basic Pipeline: Metadata + EveryNth
    // ============================================
    println!("🚀 1. Basic Pipeline: Metadata + EveryNth(30)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new()
        .add_node(ExtractMetadata::new(false))
        .add_node(ExtractFrames::new(SamplingStrategy::EveryNth(30)));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    print_results(&result, "EveryNth(30)");

    // ============================================
    // 2. FirstN Strategy
    // ============================================
    println!("� 2. FirstN Strategy: First 10 frames");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(ExtractFrames::new(SamplingStrategy::FirstN(10)));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    print_results(&result, "FirstN(10)");

    // ============================================
    // 3. Range Strategy
    // ============================================
    println!("🚀 3. Range Strategy: Frames 0-20");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(ExtractFrames::new(SamplingStrategy::Range(0, 20)));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    print_results(&result, "Range(0, 20)");

    // ============================================
    // 4. KeyFrames Strategy
    // ============================================
    println!("🚀 4. KeyFrames Strategy: Extract only keyframes");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(ExtractFrames::new(SamplingStrategy::KeyFrames));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    print_results(&result, "KeyFrames");

    // ============================================
    // 5. Custom Strategy
    // ============================================
    println!("🚀 5. Custom Strategy: Specific frame indices [0, 50, 100]");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(ExtractFrames::new(SamplingStrategy::Custom(vec![
        0, 50, 100,
    ])));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    print_results(&result, "Custom([0, 50, 100])");

    // ============================================
    // 6. Scale Factor Comparison
    // ============================================
    println!("🚀 6. Scale Factor Comparison");
    println!("----------------------------------------------");

    // Full size (no scale)
    let pipeline = Pipeline::new().add_node(ExtractFrames::new(SamplingStrategy::FirstN(1)));
    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let full = pipeline.execute(context)?;
    let full_size = full
        .extracted_frames
        .first()
        .map(|f| f.data.len())
        .unwrap_or(0);

    // Half size (0.5)
    let pipeline =
        Pipeline::new().add_node(ExtractFrames::new(SamplingStrategy::FirstN(1)).with_scale(0.5));
    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let half = pipeline.execute(context)?;
    let half_size = half
        .extracted_frames
        .first()
        .map(|f| f.data.len())
        .unwrap_or(0);

    // Quarter size (0.25)
    let pipeline =
        Pipeline::new().add_node(ExtractFrames::new(SamplingStrategy::FirstN(1)).with_scale(0.25));
    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let quarter = pipeline.execute(context)?;
    let quarter_size = quarter
        .extracted_frames
        .first()
        .map(|f| f.data.len())
        .unwrap_or(0);

    println!("   � Scale Factor Results:");
    println!("     - No scale (100%): ~{} bytes", full_size);
    println!("     - Scale 0.5 (50%): ~{} bytes", half_size);
    println!("     - Scale 0.25 (25%): ~{} bytes", quarter_size);
    println!();

    // ============================================
    // 7. ExtractionMode: Sequential
    // ============================================
    println!("🚀 7. ExtractionMode: Sequential (for dense reading)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ExtractFrames::new(SamplingStrategy::EveryNth(10))
            .with_mode(ExtractionMode::Sequential)
            .with_scale(0.5),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    println!("   Mode: Sequential");
    println!("   Strategy: EveryNth(10)");
    println!("   Scale: 0.5");
    print_results(&result, "Sequential + EveryNth(10) + Scale 0.5");

    // ============================================
    // 8. Single Frame Mode (like extract_frame)
    // ============================================
    println!("🚀 8. Single Frame Mode: Extract frame at index 100");
    println!("----------------------------------------------");

    let pipeline =
        Pipeline::new().add_node(ExtractFrames::new(SamplingStrategy::Custom(vec![100])));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(frame) = result.extracted_frames.first() {
        println!("   ✅ Extracted single frame at index: {}", frame.index);
        println!("   Base64 length: {} bytes", frame.data.len());
    } else {
        println!("   ❌ Frame 100 not found!");
    }
    println!();

    println!("===========================================");
    println!("       ✅ All Pipeline Examples Completed!");
    println!("===========================================");

    Ok(())
}

/// Helper function to print extraction results
fn print_results(context: &MediaContext, strategy_name: &str) {
    let frames = &context.extracted_frames;
    println!("   Strategy: {}", strategy_name);
    println!("   Extracted: {} frames", frames.len());

    if !frames.is_empty() {
        // Show first few frame indices
        let indices: Vec<_> = frames.iter().take(5).map(|f| f.index).collect();
        print!("   Sample indices: {:?}", indices);
        if frames.len() > 5 {
            print!(" ... (+{} more)", frames.len() - 5);
        }
        println!();
    }
    println!();
}
