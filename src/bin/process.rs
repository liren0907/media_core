//! Process Module Example
//!
//! This example demonstrates how to use the process module functions.
//! Run with: cargo run --bin process

use media_core::process::{
    HardwareAccelConfig, ProcessConfig, ProcessingMode, ProcessingOptions, ProcessingStats,
    VideoExtractionConfig, VideoProcessor, create_processor_with_mode, generate_default_config,
};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ============================================
    // 📝 CONFIGURATION - Modify these paths to test
    // ============================================
    let video_path = "data/test.mp4";
    let output_dir = "output/process_demo";

    println!("===========================================");
    println!("         Process Module Example");
    println!("===========================================\n");

    // ============================================
    // 1. ProcessConfig - Basic Configuration
    // ============================================
    println!("📋 1. ProcessConfig (Basic Configuration)");
    println!("------------------------------------------");

    let config = ProcessConfig::default();
    println!("   Input path:  {}", config.input_path);
    println!("   Output path: {}", config.output_path);
    println!("   Mode: {:?}", config.processing_mode);
    println!(
        "   Supported formats: {} types",
        config.supported_formats.len()
    );
    println!();

    // ============================================
    // 2. ProcessingOptions - Processing Settings
    // ============================================
    println!("⚙️  2. ProcessingOptions");
    println!("------------------------");

    let options = ProcessingOptions::default();
    println!("   enable_validation:       {}", options.enable_validation);
    println!("   verbose_logging:         {}", options.verbose_logging);
    println!(
        "   create_output_directory: {}",
        options.create_output_directory
    );
    println!("   overwrite_existing:      {}", options.overwrite_existing);
    println!("   max_file_size_mb:        {:?}", options.max_file_size_mb);
    println!(
        "   parallel_processing:     {}",
        options.parallel_processing
    );
    println!();

    // ============================================
    // 3. VideoExtractionConfig - Video Settings
    // ============================================
    println!("🎬 3. VideoExtractionConfig");
    println!("----------------------------");

    let video_config = VideoExtractionConfig::default();
    println!("   output_fps:          {}", video_config.output_fps);
    println!("   frame_interval:      {}", video_config.frame_interval);
    println!("   extraction_mode:     {}", video_config.extraction_mode);
    println!(
        "   video_creation_mode: {:?}",
        video_config.video_creation_mode
    );
    println!("   processing_mode:     {:?}", video_config.processing_mode);
    println!();

    // ============================================
    // 4. HardwareAccelConfig - GPU Acceleration
    // ============================================
    println!("🚀 4. HardwareAccelConfig");
    println!("-------------------------");

    let hw_config = HardwareAccelConfig::default();
    println!("   enabled:          {}", hw_config.enabled);
    println!("   mode:             {}", hw_config.mode);
    println!("   fallback_to_cpu:  {}", hw_config.fallback_to_cpu);
    println!("   prefer_backends:  {:?}", hw_config.prefer_backends);
    println!();

    // ============================================
    // 5. Create Processor with Mode
    // ============================================
    println!("🔧 5. Create Processor");
    println!("----------------------");

    match create_processor_with_mode(
        "input".to_string(),
        "output".to_string(),
        ProcessingMode::DirectoryProcess,
    ) {
        Ok(processor) => {
            println!("   ✅ Processor created successfully");
            println!(
                "   Supported formats: {}",
                processor.get_supported_formats().len()
            );
            println!("   Config input:  {}", processor.get_config().input_path);
            println!("   Config output: {}", processor.get_config().output_path);
        }
        Err(e) => {
            println!("   ⚠️  Expected error (paths don't exist): {}", e);
        }
    }
    println!();

    // ============================================
    // 6. Generate Default Config File
    // ============================================
    println!("📄 6. Generate Config File");
    println!("--------------------------");

    let config_output_path = format!("{}/generated_config.json", output_dir);
    fs::create_dir_all(output_dir)?;

    match generate_default_config(&config_output_path) {
        Ok(_) => println!("   ✅ Config saved to: {}", config_output_path),
        Err(e) => println!("   ❌ Failed: {}", e),
    }
    println!();

    // ============================================
    // 7. VideoProcessor - Frame Extraction
    // ============================================
    println!("🎥 7. VideoProcessor (Frame Extraction)");
    println!("---------------------------------------");

    // Check if test video exists
    if !Path::new(video_path).exists() {
        println!("   ⚠️  Test video not found: {}", video_path);
        println!(
            "   Place a video file at '{}' to test extraction.",
            video_path
        );
        println!();
    } else {
        // Create extraction config
        let extraction_config = VideoExtractionConfig {
            input_directories: vec![video_path.to_string()],
            output_directory: format!("{}/frames", output_dir),
            output_prefix: "demo".to_string(),
            num_threads: Some(2),
            output_fps: 30,
            frame_interval: 30, // Extract every 30th frame
            extraction_mode: "opencv".to_string(),
            create_summary_per_thread: Some(false),
            video_creation_mode: Some("skip".to_string()), // Extract only, no video
            processing_mode: Some("sequential".to_string()),
            hardware_acceleration: HardwareAccelConfig::default(),
        };

        // Write config to temp file
        let temp_config = format!("{}/temp_config.json", output_dir);
        let config_json = serde_json::to_string_pretty(&extraction_config)?;
        fs::write(&temp_config, &config_json)?;

        println!("   Video: {}", video_path);
        println!("   Frame interval: {}", extraction_config.frame_interval);
        println!("   Output: {}/frames", output_dir);
        println!();

        // Run extraction
        let mut stats = ProcessingStats::new();
        match VideoProcessor::run_video_extraction(&temp_config, &mut stats) {
            Ok(_) => {
                stats.finalize();
                println!("   ✅ Extraction complete!");
                println!("   Files processed: {}", stats.files_processed);
                println!("   Processing time: {:?}", stats.processing_time);
            }
            Err(e) => {
                println!("   ❌ Extraction failed: {}", e);
            }
        }

        // Cleanup temp config
        let _ = fs::remove_file(&temp_config);
    }
    println!();

    // ============================================
    // 8. ProcessingStats - Metrics
    // ============================================
    println!("📊 8. ProcessingStats");
    println!("---------------------");

    let mut stats = ProcessingStats::new();
    stats.add_processed_file(1024 * 1024); // 1MB
    stats.add_processed_file(2048 * 1024); // 2MB
    stats.add_failed_file("Example error".to_string());
    stats.finalize();

    println!("   Files processed: {}", stats.files_processed);
    println!("   Files failed:    {}", stats.files_failed);
    println!("   Total size:      {} bytes", stats.total_size_processed);
    println!("   Success rate:    {:.1}%", stats.success_rate());
    println!("   Errors: {:?}", stats.errors);
    println!();

    println!("===========================================");
    println!("         ✅ All examples completed!");
    println!("===========================================");

    Ok(())
}
