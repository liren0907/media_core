//! Analysis Module Example
//!
//! This example demonstrates how to use the Analysis module programmatically.
//! It covers:
//! 1. Motion Detection
//! 2. Image Similarity (General)
//! 3. Perceptual Hash Comparison (Media Lake Style)
//! 4. Comprehensive Algorithm Comparison (All Methods)
//!
//! Run with: cargo run --bin analysis

use media_core::analysis::config::{
    MotionConfig, PerceptualHashConfig, ProcessMode, SimilarityConfig, SimilarityMethod,
};
use media_core::analysis::motion::MotionDetector;
use media_core::analysis::similarity::SimilarityAnalyzer;
use std::path::Path;

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
            if !video_path.exists() {
                println!(
                    "⚠️  Video file not found: {}. Skipping Motion Detection.",
                    video_path.display()
                );
            } else {
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

    if !image_dir.exists() {
        println!(
            "⚠️  Image directory not found: {}. Skipping General Image Similarity.",
            image_dir.display()
        );
    } else {
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
    }

    // ============================================
    // 3. Perceptual Hash Comparison (Media Lake Style)
    // ============================================
    // This demonstrates the same functionality as the `media_lake` module
    // but using only the `analysis` module's SimilarityAnalyzer.
    println!("\n--- 3. Perceptual Hash Comparison (Media Lake Style) ---");

    let img1_path = Path::new("references/media-lake/data/test_1.jpg");
    let img2_path = Path::new("references/media-lake/data/A400001.jpg");

    // Check if test images exist
    if !img1_path.exists() || !img2_path.exists() {
        eprintln!("⚠️  Test images not found. Skipping pHash comparison.");
        eprintln!("   Expected: references/media-lake/data/test_1.jpg");
        eprintln!("   Expected: references/media-lake/data/A400001.jpg");
    } else {
        println!("Comparing two images using Perceptual Hash:");
        println!("  Image A: {}", img1_path.display());
        println!("  Image B: {}", img2_path.display());

        // Configure for Perceptual Hash with custom threshold
        let phash_config = SimilarityConfig {
            method: SimilarityMethod::PerceptualHash,
            perceptual_hash: PerceptualHashConfig {
                hash_size: 8,               // 8x8 pHash (64-bit)
                similarity_threshold: 0.95, // 95% similarity threshold
            },
            ..Default::default()
        };

        match SimilarityAnalyzer::new(phash_config) {
            Ok(mut analyzer) => match analyzer.compare_images(img1_path, img2_path) {
                Ok(similarity) => {
                    let threshold = 0.95;
                    let is_duplicate = similarity >= threshold;

                    println!("\n📊 Results:");
                    println!("   Similarity Score: {:.2}%", similarity * 100.0);
                    println!("   Threshold:        {:.2}%", threshold * 100.0);
                    println!(
                        "   Is Duplicate?     {}",
                        if is_duplicate { "Yes" } else { "No" }
                    );

                    if is_duplicate {
                        println!("\n✅ Conclusion: These images are visually identical.");
                    } else {
                        println!("\n❌ Conclusion: These images are different.");
                    }
                }
                Err(e) => eprintln!("❌ Comparison failed: {}", e),
            },
            Err(e) => eprintln!("❌ Failed to initialize SimilarityAnalyzer: {}", e),
        }
    }

    // ============================================
    // 4. Comprehensive Algorithm Comparison (All Methods)
    // ============================================
    println!("\n========================================================");
    println!("--- 4. Comprehensive Algorithm Comparison (All Methods) ---");
    println!("========================================================");

    // We will run the clustering on the same input directory using ALL available methods
    // to see how results differ.

    let comparison_input_dir = Path::new("references/media-lake/data");
    let comparison_base_output = output_dir.join("algorithm_comparison");

    println!(
        "Running comprehensive comparison on: {}",
        comparison_input_dir.display()
    );
    println!(
        "Output base directory: {}",
        comparison_base_output.display()
    );

    // Define the methods to test
    let methods = vec![
        (SimilarityMethod::Histogram, "histogram"),
        (SimilarityMethod::PerceptualHash, "phash"),
        (SimilarityMethod::FeatureMatching, "feature_matching"),
    ];

    for (method, name) in methods {
        println!("\n🔹 Running Method: {}", name.to_uppercase());

        let method_output_dir = comparison_base_output.join(name);
        println!("   Output Directory: {}", method_output_dir.display());

        // Create a config specific to this method
        let mut config = SimilarityConfig {
            method: method,
            process_mode: ProcessMode::Single,
            group_similar: true,
            min_category_size: 2, // Only show interesting groups
            ..Default::default()
        };

        // Tweak thresholds per method for demonstration
        match name {
            "histogram" => config.histogram.similarity_threshold = 0.8,
            "phash" => config.perceptual_hash.similarity_threshold = 0.95,
            "feature_matching" => config.feature_matching.similarity_threshold = 0.3, // Features need lower threshold usually
            _ => {}
        }

        match SimilarityAnalyzer::new(config) {
            Ok(mut analyzer) => {
                match analyzer.group_similar_images(comparison_input_dir, &method_output_dir) {
                    Ok(groups) => {
                        println!("   ✅ Complete. Found {} groups.", groups.len());
                        if !groups.is_empty() {
                            println!("   First few groups found:");
                            for (group_name, members) in groups.iter().take(3) {
                                println!("      - {}: {} images", group_name, members.len());
                            }
                        } else {
                            println!("      (No groups found with this method/threshold)");
                        }
                    }
                    Err(e) => eprintln!("   ❌ Failed: {}", e),
                }
            }
            Err(e) => eprintln!("   ❌ Failed to initialize analyzer: {}", e),
        }
    }

    println!("\n✅ Comprehensive comparison complete.");
    println!(
        "   Check {} to inspect detailed results.",
        comparison_base_output.display()
    );

    Ok(())
}
