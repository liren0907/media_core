//! Pipeline Video Process Example
//!
//! This example demonstrates the Unified Media Pipeline for Video Processing.
//! It showcases ALL features of the video_process module through the pipeline pattern:
//!
//! 1. Basic frame extraction with default settings
//! 2. Custom interval and parallel extraction mode
//! 3. FFmpeg-based extraction
//! 4. Single directory save mode
//!
//! Run with: cargo run --bin pipeline_video_process

use media_core::pipeline::{MediaContext, Pipeline};
use media_core::video_process::{ExtractFramesToDisk, ExtractionMode, SaveMode};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline Video Process Example");
    println!("===========================================\n");

    let video_path = "data/test.mp4";
    let output_base = "output/pipeline_video_process";

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
    // 1. Basic Frame Extraction (OpenCV Interval)
    // ============================================
    println!("🚀 1. Basic Frame Extraction (OpenCV Interval)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new()
        .add_node(ExtractFramesToDisk::new(format!("{}/basic", output_base)).interval(30));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(vp) = &result.video_process_result {
        println!("   ✅ Output: {}", vp.output_dir);
        println!("   Mode: {}", vp.extraction_mode);
        println!("   Frames: {}", vp.frames_extracted);
    }
    println!();

    // ============================================
    // 2. Parallel Extraction Mode
    // ============================================
    println!("🚀 2. Parallel Extraction Mode");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/parallel", output_base))
            .interval(30)
            .mode(ExtractionMode::Parallel),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(vp) = &result.video_process_result {
        println!("   ✅ Output: {}", vp.output_dir);
        println!("   Mode: {}", vp.extraction_mode);
        println!("   Frames: {}", vp.frames_extracted);
    }
    println!();

    // ============================================
    // 3. FFmpeg Interval Extraction
    // ============================================
    println!("🚀 3. FFmpeg Interval Extraction");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/ffmpeg", output_base))
            .interval(30)
            .mode(ExtractionMode::FFmpegInterval),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(vp) = &result.video_process_result {
        println!("   ✅ Output: {}", vp.output_dir);
        println!("   Mode: {}", vp.extraction_mode);
        println!("   Frames: {}", vp.frames_extracted);
    }
    println!();

    // ============================================
    // 4. Single Directory Save Mode
    // ============================================
    println!("🚀 4. Single Directory Save Mode");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/single_dir", output_base))
            .interval(30)
            .mode(ExtractionMode::OpenCVInterval)
            .save_mode(SaveMode::SingleDirectory),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(vp) = &result.video_process_result {
        println!("   ✅ Output: {}", vp.output_dir);
        println!("   Mode: {}", vp.extraction_mode);
        println!("   Save Mode: {}", vp.save_mode);
        println!("   Frames: {}", vp.frames_extracted);
    }
    println!();

    // ============================================
    // 5. OpenCV Sequential Mode (ALL frames)
    // ============================================
    println!("🚀 5. OpenCV Sequential Mode (ALL frames)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/opencv_sequential", output_base))
            .interval(1) // Every frame
            .mode(ExtractionMode::OpenCVSequential),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(vp) = &result.video_process_result {
        println!("   ✅ Output: {}", vp.output_dir);
        println!("   Mode: {}", vp.extraction_mode);
        println!("   Frames: {}", vp.frames_extracted);
    }
    println!();

    // ============================================
    // 6. FFmpeg Mode (ALL frames)
    // ============================================
    println!("🚀 6. FFmpeg Mode (ALL frames)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/ffmpeg_all", output_base))
            .mode(ExtractionMode::FFmpeg),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(vp) = &result.video_process_result {
        println!("   ✅ Output: {}", vp.output_dir);
        println!("   Mode: {}", vp.extraction_mode);
        println!("   Frames: {}", vp.frames_extracted);
    }
    println!();

    // ============================================
    // 7. Multiple Directory Save Mode
    // ============================================
    println!("🚀 7. Multiple Directory Save Mode");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/multi_dir", output_base))
            .interval(30)
            .mode(ExtractionMode::Parallel)
            .save_mode(SaveMode::MultipleDirectory),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(vp) = &result.video_process_result {
        println!("   ✅ Output: {}", vp.output_dir);
        println!("   Mode: {}", vp.extraction_mode);
        println!("   Save Mode: {}", vp.save_mode);
        println!("   Frames: {}", vp.frames_extracted);
    }
    println!();

    println!("===========================================");
    println!("       ✅ All Video Process Examples Completed!");
    println!("===========================================");

    Ok(())
}
