//! HLS Converter Module Example
//!
//! This example demonstrates how to use the HLS converter programmatically.
//! Run with: cargo run --bin hls_converter_example

use media_core::hls::config::HLSVodConfig;
use media_core::hls::converter::HLSConverter;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       HLS Converter Module Example");
    println!("===========================================\n");

    let input_file = "data/test.mp4";
    let output_dir = "output/hls_example";

    // 1. Create Configuration
    let config = HLSVodConfig {
        input_path: Path::new(input_file).to_path_buf(),
        output_dir: Path::new(output_dir).to_path_buf(),
        segment_duration: 4,
        ..Default::default()
    };

    println!("Configuration:");
    println!("  Input:  {:?}", config.input_path);
    println!("  Output: {:?}", config.output_dir);
    println!("  Segment: {}s", config.segment_duration);

    // 2. Initialize Converter
    let converter = HLSConverter::new(config);

    // 3. Run Process
    println!("\nStarting conversion...");
    match converter.convert() {
        Ok(_) => {
            println!("✅ HLS conversion completed successfully!");
            println!("Output saved to: {}", output_dir);
        }
        Err(e) => {
            eprintln!("❌ HLS conversion failed: {}", e);
        }
    }

    Ok(())
}
