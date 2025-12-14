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
