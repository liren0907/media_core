//! Pipeline Annotation Example
//!
//! This example demonstrates the Unified Media Pipeline for Annotation.
//! It showcases ALL features of the annotation module through the pipeline pattern:
//!
//! 1. Single Frame Annotation (Filename)
//! 2. Video from Frames (Filename)
//! 3. Video from Frames (Timestamp)
//! 4. Video from Frames (Custom Text)
//!
//! Run with: cargo run --bin pipeline_annotation

use media_core::annotation::pipeline::{AnnotateFrame, AnnotateVideo};
use media_core::annotation::{AnnotationType, TextPosition};
use media_core::pipeline::{MediaContext, Pipeline};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline Annotation Example");
    println!("===========================================\n");

    let output_dir = "output/pipeline_annotation";
    std::fs::create_dir_all(output_dir)?;

    // ============================================
    // 1. Single Frame Annotation (Filename)
    // ============================================
    println!("--- 1. Single Frame Annotation ---");

    let sample_image = "output/video_process/test/frame_0.jpg";

    if !Path::new(sample_image).exists() {
        println!("⚠️  Sample image not found: {}. Skipping.", sample_image);
    } else {
        let pipeline = Pipeline::new().add_node(
            AnnotateFrame::new(sample_image, &format!("{}/annotated_frame.jpg", output_dir))
                .annotation_type(AnnotationType::Filename)
                .position(TextPosition::TopLeft),
        );

        let context = MediaContext::from_file(Path::new("dummy").to_path_buf());
        let result = pipeline.execute(context)?;

        if let Some(ann) = result.annotation_result {
            println!("   ✅ Output: {}", ann.output_path);
        }
    }

    // ============================================
    // 2. Video from Frames (Filename)
    // ============================================
    println!("\n--- 2. Video from Frames (Filename) ---");

    let frames_dir = "output/video_process/test";

    if !Path::new(frames_dir).exists() {
        println!("⚠️  Frames directory not found: {}. Skipping.", frames_dir);
    } else {
        let pipeline = Pipeline::new().add_node(
            AnnotateVideo::new(
                frames_dir,
                &format!("{}/annotated_filename.mp4", output_dir),
            )
            .annotation_type(AnnotationType::Filename)
            .position(TextPosition::TopLeft)
            .fps(30),
        );

        let context = MediaContext::from_file(Path::new("dummy").to_path_buf());
        let result = pipeline.execute(context)?;

        if let Some(ann) = result.annotation_result {
            println!("   ✅ Output: {}", ann.output_path);
        }
    }

    // ============================================
    // 3. Video from Frames (Timestamp)
    // ============================================
    println!("\n--- 3. Video from Frames (Timestamp) ---");

    if !Path::new(frames_dir).exists() {
        println!("⚠️  Frames directory not found. Skipping.");
    } else {
        let pipeline = Pipeline::new().add_node(
            AnnotateVideo::new(
                frames_dir,
                &format!("{}/annotated_timestamp.mp4", output_dir),
            )
            .annotation_type(AnnotationType::Timestamp)
            .position(TextPosition::BottomLeft)
            .fps(30)
            .source_fps(30.0),
        );

        let context = MediaContext::from_file(Path::new("dummy").to_path_buf());
        let result = pipeline.execute(context)?;

        if let Some(ann) = result.annotation_result {
            println!("   ✅ Output: {}", ann.output_path);
        }
    }

    // ============================================
    // 4. Video from Frames (Custom Text)
    // ============================================
    println!("\n--- 4. Video from Frames (Custom Text) ---");

    if !Path::new(frames_dir).exists() {
        println!("⚠️  Frames directory not found. Skipping.");
    } else {
        let pipeline = Pipeline::new().add_node(
            AnnotateVideo::new(frames_dir, &format!("{}/annotated_custom.mp4", output_dir))
                .annotation_type(AnnotationType::Custom("Watermark".to_string()))
                .position(TextPosition::TopRight)
                .fps(30),
        );

        let context = MediaContext::from_file(Path::new("dummy").to_path_buf());
        let result = pipeline.execute(context)?;

        if let Some(ann) = result.annotation_result {
            println!("   ✅ Output: {}", ann.output_path);
        }
    }

    println!("\n===========================================");
    println!("       ✅ All Annotation Examples Completed!");
    println!("===========================================");

    Ok(())
}
