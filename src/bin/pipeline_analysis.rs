//! Pipeline Analysis Example
//!
//! This example demonstrates the Unified Media Pipeline for Analysis.
//! It showcases ALL features of the analysis module through the pipeline pattern:
//!
//! 1. Motion Detection (Video)
//! 2. Image Similarity Grouping (Directory)
//! 3. Perceptual Hash Comparison (Two Images)
//! 4. Algorithm Method Comparison (Histogram vs pHash vs Feature Matching)
//!
//! Run with: cargo run --bin pipeline_analysis

use media_core::analysis::config::SimilarityMethod;
use media_core::analysis::pipeline::{CompareImages, DetectMotion, GroupSimilarImages};
use media_core::metadata::pipeline::ExtractMetadata;
use media_core::pipeline::{MediaContext, Pipeline};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline Analysis Example");
    println!("===========================================\n");

    // ============================================
    // 1. Motion Detection (Video)
    // ============================================
    println!("--- 1. Motion Detection ---");

    let video_path = "data/test.mp4";

    if !Path::new(video_path).exists() {
        println!(
            "⚠️  Video not found: {}. Skipping Motion Detection.",
            video_path
        );
    } else {
        let pipeline = Pipeline::new()
            .add_node(ExtractMetadata::new(false))
            .add_node(
                DetectMotion::default()
                    .threshold(25.0)
                    .min_area(500)
                    .output_dir("output/pipeline_analysis/motion"),
            );

        let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
        let result = pipeline.execute(context)?;

        if let Some(meta) = result.metadata {
            println!("   Resolution: {}, FPS: {:?}", meta.resolution, meta.fps);
        }
        if let Some(report) = result.analysis {
            println!("   ✅ Motion events found: {}", report.motion_events.len());
            for (i, event) in report.motion_events.iter().take(3).enumerate() {
                println!(
                    "      {}. Frames {}-{}",
                    i + 1,
                    event.start_frame,
                    event.end_frame
                );
            }
        }
    }

    // ============================================
    // 2. Image Similarity Grouping
    // ============================================
    println!("\n--- 2. Image Similarity Grouping ---");

    let image_dir = Path::new("references/media-lake/data");
    let output_dir = Path::new("output/pipeline_analysis/similarity");

    if !image_dir.exists() {
        println!(
            "⚠️  Image directory not found: {}. Skipping.",
            image_dir.display()
        );
    } else {
        let pipeline = Pipeline::new().add_node(
            GroupSimilarImages::new(image_dir, output_dir)
                .method(SimilarityMethod::PerceptualHash)
                .phash_threshold(0.95)
                .min_category_size(2)
                .group_similar(true),
        );

        let context = MediaContext::from_file(Path::new("dummy").to_path_buf());
        let result = pipeline.execute(context)?;

        if let Some(report) = result.analysis {
            println!(
                "   ✅ Similarity groups found: {}",
                report.similarity_groups.len()
            );
            for group in report.similarity_groups.iter().take(3) {
                println!(
                    "      - {}: {} images",
                    group.group_name,
                    group.members.len()
                );
            }
        }
    }

    // ============================================
    // 3. Perceptual Hash Comparison (Two Images)
    // ============================================
    println!("\n--- 3. Perceptual Hash Comparison ---");

    let img1 = Path::new("references/media-lake/data/test_1.jpg");
    let img2 = Path::new("references/media-lake/data/A400001.jpg");

    if !img1.exists() || !img2.exists() {
        println!("⚠️  Test images not found. Skipping pHash comparison.");
    } else {
        println!("   Comparing: {} vs {}", img1.display(), img2.display());

        let pipeline = Pipeline::new().add_node(
            CompareImages::new(img1, img2)
                .method(SimilarityMethod::PerceptualHash)
                .threshold(0.95),
        );

        let context = MediaContext::from_file(Path::new("dummy").to_path_buf());
        let result = pipeline.execute(context)?;

        if let Some(report) = result.analysis {
            if let Some(cmp) = report.image_comparison {
                println!("   Similarity: {:.2}%", cmp.similarity_score * 100.0);
                println!(
                    "   Is Duplicate? {}",
                    if cmp.is_duplicate { "Yes" } else { "No" }
                );
            }
        }
    }

    // ============================================
    // 4. Algorithm Comparison (All Methods)
    // ============================================
    println!("\n--- 4. Algorithm Comparison ---");

    if !img1.exists() || !img2.exists() {
        println!("⚠️  Test images not found. Skipping algorithm comparison.");
    } else {
        let methods = vec![
            (SimilarityMethod::Histogram, "Histogram"),
            (SimilarityMethod::PerceptualHash, "pHash"),
            (SimilarityMethod::FeatureMatching, "FeatureMatching"),
        ];

        for (method, name) in methods {
            let pipeline = Pipeline::new().add_node(CompareImages::new(img1, img2).method(method));

            let context = MediaContext::from_file(Path::new("dummy").to_path_buf());
            let result = pipeline.execute(context)?;

            if let Some(report) = result.analysis {
                if let Some(cmp) = report.image_comparison {
                    println!("   {}: {:.2}%", name, cmp.similarity_score * 100.0);
                }
            }
        }
    }

    println!("\n===========================================");
    println!("       ✅ All Analysis Examples Completed!");
    println!("===========================================");

    Ok(())
}
