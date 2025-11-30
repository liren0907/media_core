use media_core::{CaptureConfig, RTSPCapture, SavingOption};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🚀 Starting Media Core App...");

    // 1. Create a configuration programmatically
    let config = CaptureConfig {
        rtsp_url: "rtsp://localhost:8554/mystream".to_string(),
        rtsp_url_list: vec![],
        output_directory: "app_media".to_string(),
        show_preview: false,
        saving_option: SavingOption::Single,
        saved_time_duration: 60, // Record for 60 seconds
        use_fps: false,
        fps: 30.0,
    };

    println!("📋 Configuration:");
    println!("   URL: {}", config.rtsp_url);
    println!("   Output: {}", config.output_directory);

    // 2. Initialize the RTSP Capture
    println!("\n🔄 Initializing RTSP Capture...");

    match RTSPCapture::new(
        config.rtsp_url.clone(),
        config.output_directory.clone(),
        config.show_preview,
        config.saved_time_duration,
        config.use_fps,
        config.fps,
        true, // run_once: true for this example
    ) {
        Ok(mut capture) => {
            println!("✅ Capture initialized successfully!");
            println!("▶️  Starting stream processing...");

            if let Err(e) = capture.process_stream() {
                eprintln!("⚠️  Stream processing ended: {}", e);
            } else {
                println!("✅ Stream processing completed!");
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize capture: {}", e);
        }
    }

    println!("\n✨ App finished!");
    Ok(())
}
