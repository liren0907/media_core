//! Streaming Module Example
//!
//! This example demonstrates how to use the streaming module functions.
//! Run with: cargo run --bin streaming

use media_core::streaming::{FrameData, SamplingStrategy, StreamExtractor};

fn main() -> Result<(), String> {
    // ============================================
    // 📝 CONFIGURATION - Modify this path to test
    // ============================================
    let video_path = "data/test.mp4";

    println!("===========================================");
    println!("       Streaming Module Example");
    println!("===========================================\n");

    // ============================================
    // 1. StreamExtractor (Single Frame Mode)
    // ============================================
    // Demonstrates how to use StreamExtractor to get a specific single frame
    // (Replacing the old extract_frame function)
    println!("🖼️  1. StreamExtractor (Single Frame Mode)");
    println!("------------------------");
    let mut extractor = StreamExtractor::new(video_path, None)?;
    // Use Custom strategy with a single index for one-shot extraction
    extractor.set_strategy(SamplingStrategy::Custom(vec![100]));

    let frames = extractor.extract(None)?;
    if let Some(frame) = frames.first() {
        println!("   Extracted frame at index: {}", frame.index);
        println!("   Base64 length: {} bytes", frame.data.len());
    } else {
        println!("   ❌ Frame 100 not found!");
    }
    println!();

    // ============================================
    // 2. StreamExtractor - Struct-based extraction
    // ============================================
    // This struct combines sampling strategies with scale factor support.
    // It allows reusing the same VideoCapture for multiple extractions.
    //
    // Methods:
    //   - StreamExtractor::new(video_path, strategy_opt) -> Result<Self, String>
    //   - .video_path() -> &str
    //   - .total_frames() -> usize
    //   - .set_strategy(strategy)
    //   - .set_mode(mode)
    //   - .extract(scale_factor) -> Result<Vec<FrameData>, String>
    println!("🚀 2. StreamExtractor");
    println!("----------------------");

    let mut extractor = StreamExtractor::new(video_path, Some(SamplingStrategy::EveryNth(50)))?;

    // Demonstrate switching to Sequential mode (like stream_frames)
    // This is useful when you want to read many frames continuously
    extractor.set_mode(media_core::streaming::ExtractionMode::Sequential);
    println!("   Mode: Sequential");

    println!("   Video: {}", extractor.video_path());
    println!("   Total frames: {}", extractor.total_frames());
    println!();

    // 2a. EveryNth with scale
    // Strategy is already set to EveryNth(50) from construction
    let frames: Vec<FrameData> = extractor.extract(Some(0.5))?;
    println!("   EveryNth(50) + scale 0.5: {} frames", frames.len());

    // 2b. FirstN with scale
    extractor.set_strategy(SamplingStrategy::FirstN(10));
    let frames: Vec<FrameData> = extractor.extract(Some(0.5))?;
    println!("   FirstN(10) + scale 0.5: {} frames", frames.len());

    // 2c. Range with scale
    extractor.set_strategy(SamplingStrategy::Range(0, 10));
    let frames: Vec<FrameData> = extractor.extract(Some(0.25))?;
    println!("   Range(0, 10) + scale 0.25: {} frames", frames.len());

    // 2d. KeyFrames with scale
    extractor.set_strategy(SamplingStrategy::KeyFrames);
    let frames: Vec<FrameData> = extractor.extract(Some(0.5))?;
    println!("   KeyFrames + scale 0.5: {} frames", frames.len());

    // 2e. Custom with no scale
    extractor.set_strategy(SamplingStrategy::Custom(vec![0, 100, 200]));
    let frames: Vec<FrameData> = extractor.extract(None)?;
    println!("   Custom([0,100,200]) + no scale: {} frames", frames.len());
    println!();

    // 2f. Scale factor comparison - showing size difference
    println!("   📐 Scale Factor Comparison:");
    extractor.set_strategy(SamplingStrategy::FirstN(1));
    let frame_full = extractor.extract(None)?;
    let frame_half = extractor.extract(Some(0.5))?;
    let frame_quarter = extractor.extract(Some(0.25))?;
    println!(
        "     - No scale (100%): ~{} bytes",
        frame_full.first().map(|f| f.data.len()).unwrap_or(0)
    );
    println!(
        "     - Scale 0.5 (50%): ~{} bytes",
        frame_half.first().map(|f| f.data.len()).unwrap_or(0)
    );
    println!(
        "     - Scale 0.25 (25%): ~{} bytes",
        frame_quarter.first().map(|f| f.data.len()).unwrap_or(0)
    );
    println!();

    println!("===========================================");
    println!("       ✅ All examples completed!");
    println!("===========================================");

    Ok(())
}
