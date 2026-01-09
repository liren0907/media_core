//! Pipeline Process Example
//!
//! This example demonstrates the Unified Media Pipeline for File Processing.
//! It showcases the process module through the pipeline pattern:
//!
//! 1. Single file processing
//! 2. Directory processing mode
//! 3. Batch processing mode
//! 4. Generate default config file (utility)
//!
//! Run with: cargo run --bin pipeline_process

use media_core::pipeline::{MediaContext, Pipeline};
use media_core::process::{ProcessFiles, ProcessingMode, generate_default_config};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline Process Example");
    println!("===========================================\n");

    let video_path = "data/test.mp4";
    let output_base = "output/pipeline_process";

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
    std::fs::create_dir_all(output_base)?;

    // ============================================
    // 1. Single File Processing
    // ============================================
    println!("🚀 1. Single File Processing");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ProcessFiles::new(format!("{}/single", output_base)).mode(ProcessingMode::SingleFile),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(pr) = &result.process_result {
        println!("   ✅ Output: {}", pr.output_dir);
        println!("   Mode: {}", pr.processing_mode);
        println!("   Files Processed: {}", pr.files_processed);
        println!("   Total Size: {} bytes", pr.total_size_bytes);
    }
    println!();

    // ============================================
    // 2. Directory Processing Mode
    // ============================================
    println!("🚀 2. Directory Processing Mode");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ProcessFiles::new(format!("{}/directory", output_base))
            .mode(ProcessingMode::DirectoryProcess)
            .overwrite(true),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(pr) = &result.process_result {
        println!("   ✅ Output: {}", pr.output_dir);
        println!("   Mode: {}", pr.processing_mode);
        println!("   Files Processed: {}", pr.files_processed);
        println!("   Success: {}", pr.success);
    }
    println!();

    // ============================================
    // 3. Batch Processing Mode
    // ============================================
    println!("🚀 3. Batch Processing Mode");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        ProcessFiles::new(format!("{}/batch", output_base)).mode(ProcessingMode::BatchFiles),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(pr) = &result.process_result {
        println!("   ✅ Output: {}", pr.output_dir);
        println!("   Mode: {}", pr.processing_mode);
        println!("   Files Processed: {}", pr.files_processed);
    }
    println!();

    // ============================================
    // 4. Generate Default Config File (Utility)
    // ============================================
    println!("📄 4. Generate Default Config File");
    println!("----------------------------------------------");

    let config_path = format!("{}/default_config.json", output_base);
    match generate_default_config(&config_path) {
        Ok(_) => println!("   ✅ Config saved to: {}", config_path),
        Err(e) => println!("   ❌ Failed: {}", e),
    }
    println!();

    println!("===========================================");
    println!("       ✅ All Process Examples Completed!");
    println!("===========================================");

    Ok(())
}
