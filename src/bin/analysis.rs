//! Analysis Module Example
//!
//! This example demonstrates how to use the Analysis module programmatically.
//! It covers:
//! 1. Motion Detection
//! 2. Image Similarity
//!
//! Run with: cargo run --bin analysis_example

use media_core::analysis::config::{MotionConfig, SimilarityConfig, ProcessMode};
use media_core::analysis::motion::MotionDetector;
use media_core::analysis::similarity::SimilarityAnalyzer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Analysis Module Example");
    println!("===========================================\n");

    let video_path = std::path::PathBuf::from("data/test.mp4");
    let output_dir = std::path::PathBuf::from("output/analysis_example");

    // Ensure output directory exists
    std::fs::create_dir_all(&output_dir)?;

    // ============================================
    // 1. Motion Detection
    // ============================================
    println!("--- 1. Motion Detection ---");

    let motion_config = MotionConfig::default();

    // MotionDetector::new returns a Result
    match MotionDetector::new(motion_config) {
        Ok(mut detector) => {
            println!("Starting motion detection on: {}", video_path.display());
            println!("This may take a while depending on video length...");

            match detector.process_video(&video_path, &output_dir) {
                Ok(events) => {
                    println!("✅ Motion detection complete.");
                    println!("Found {} motion events.", events.len());
                    for (i, event) in events.iter().take(5).enumerate() {
                        println!(
                            "  Event {}: Frame {} - Frame {} (Duration: {} frames)",
                            i + 1,
                            event.0,
                            event.1,
                            event.1 - event.0
                        );
                    }
                    if events.len() > 5 {
                        println!("  ... and {} more.", events.len() - 5);
                    }
                }
                Err(e) => eprintln!("❌ Motion detection failed: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Failed to initialize MotionDetector: {}", e),
    }

    // ============================================
    // 2. Image Similarity (Conceptual)
    // ============================================
    println!("\n--- 2. Image Similarity ---");
    println!("Initializing Image Similarity Analyzer...");
    
    // Analyzer usually takes a directory of images
    let image_dir = std::path::PathBuf::from("data/images");
    let sim_config = SimilarityConfig {
        process_mode: ProcessMode::Single,
        ..Default::default()
    };

    // SimilarityAnalyzer::new returns a Result
    match SimilarityAnalyzer::new(sim_config) {
        Ok(mut analyzer) => match analyzer.group_similar_images(&image_dir, &output_dir) {
            Ok(groups) => {
                println!("✅ Similarity analysis complete.");
                println!("Found {} similarity groups.", groups.len());
            }
            Err(e) => eprintln!("❌ Similarity analysis failed: {}", e),
        },
        Err(e) => eprintln!("❌ Failed to initialize SimilarityAnalyzer: {}", e),
    }

    Ok(())
}
