use media_core::pipeline::{MediaContext, Pipeline};
use media_core::process::{ProcessFiles, ProcessingMode, generate_default_config};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline Process Example");
    println!("===========================================\n");

    let video_path = "data/test.mp4";
    let output_base = "output/pipeline_process";

    std::fs::create_dir_all(output_base)?;

    // 1. Single File Processing
    // This example processes a single video file (data/test.mp4) and saves the result to the output directory.
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

    // 2. Directory Processing Mode
    // This example processes all files in a directory (simulated here using the same single file as context for demo purposes)
    // and saves them to the output directory. It also demonstrates the 'overwrite' flag.
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

    // 3. Batch Processing Mode
    // This example simulates batch processing where multiple files are processed as a group.
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

    // 4. Generate Default Config File (Utility)
    // This utility function generates a default JSON configuration file for the process module.
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
