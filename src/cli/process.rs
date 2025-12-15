use media_core::process::create_video_processor;
use std::error::Error;

pub fn run_process_mode(config_path: &str) -> Result<(), Box<dyn Error>> {
    println!("🎬 Starting Video Processing Mode...");
    println!("📄 Using config file: {}", config_path);

    let mut processor = create_video_processor()?;

    match processor.run_video_extraction(config_path) {
        Ok(_) => {
            println!("✅ Video processing completed successfully!");

            let stats = processor.get_stats();
            println!("📊 Processing Statistics:");
            println!("   • Files processed: {}", stats.files_processed);
            println!("   • Files failed: {}", stats.files_failed);
            println!("   • Success rate: {:.2}%", stats.success_rate());
            println!("   • Processing time: {:?}", stats.processing_time);

            if !stats.errors.is_empty() {
                println!("⚠️  Errors encountered:");
                for error in &stats.errors {
                    println!("   • {}", error);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Video processing failed: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}

pub fn run_extraction_mode(input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    use media_core::process::{HardwareAccelConfig, VideoExtractionConfig};
    use std::io::Write;
    use tempfile::NamedTempFile;

    println!("🎬 Starting Extraction Mode (Direct)...");
    println!("   • Input: {}", input_path);
    println!("   • Output: {}", output_path);

    // Create a temporary config file based on CLI args
    let config = VideoExtractionConfig {
        input_directories: vec![input_path.to_string()],
        output_directory: output_path.to_string(),
        output_prefix: "extract".to_string(),
        num_threads: Some(4),
        output_fps: 30,                        // Irrelevant for extraction only
        frame_interval: 1,                     // Extract every frame by default
        extraction_mode: "opencv".to_string(), // Default to fast opencv
        create_summary_per_thread: Some(true),
        video_creation_mode: Some("skip".to_string()), // CRITICAL: Skip video creation
        processing_mode: Some("sequential".to_string()),
        hardware_acceleration: HardwareAccelConfig::default(),
    };

    let mut temp_file = NamedTempFile::new()?;
    let config_json = serde_json::to_string_pretty(&config)?;
    write!(temp_file, "{}", config_json)?;

    // Create processor and run
    let mut processor = create_video_processor()?;
    let temp_path = temp_file.path().to_str().ok_or("Invalid temp path")?;

    match processor.run_video_extraction(temp_path) {
        Ok(_) => {
            println!("✅ Extraction completed successfully!");
        }
        Err(e) => {
            eprintln!("❌ Extraction failed: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}
