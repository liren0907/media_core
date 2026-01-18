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

    // Example 1. Single Frame Annotation
    println!("--- 1. Single Frame Annotation ---");
    let sample_image = "data/test.jpg";
    let pipeline = Pipeline::new().add_node(
        AnnotateFrame::with_context_source(&format!("{}/annotated_frame.jpg", output_dir))
            .annotation_type(AnnotationType::Filename)
            .position(TextPosition::TopLeft),
    );
    let context = MediaContext::from_file(Path::new(sample_image).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(ann) = result.annotation_result {
        println!("   ✅ Output: {}", ann.output_path);
    }

    // Example 2. Video from Video File (Filename)
    println!("\n--- 2. Video from Video File (Filename) ---");
    let video_path = "data/test.mp4";
    let pipeline = Pipeline::new().add_node(
        AnnotateVideo::with_context_source(&format!("{}/annotated_video_filename.mp4", output_dir))
            .annotation_type(AnnotationType::Filename)
            .position(TextPosition::TopLeft)
            .fps(30),
    );
    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(ann) = result.annotation_result {
        println!("   ✅ Output: {}", ann.output_path);
    }

    // Example 3. Video from Video File (Timestamp)
    println!("\n--- 3. Video from Video File (Timestamp) ---");
    let pipeline = Pipeline::new().add_node(
        AnnotateVideo::with_context_source(&format!(
            "{}/annotated_video_timestamp.mp4",
            output_dir
        ))
        .annotation_type(AnnotationType::Timestamp)
        .position(TextPosition::BottomLeft)
        .fps(30)
        .source_fps(30.0),
    );
    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(ann) = result.annotation_result {
        println!("   ✅ Output: {}", ann.output_path);
    }

    // Example 4. Video from Video File (Custom Text)
    println!("\n--- 4. Video from Video File (Custom Text) ---");
    let pipeline = Pipeline::new().add_node(
        AnnotateVideo::with_context_source(&format!("{}/annotated_video_custom.mp4", output_dir))
            .annotation_type(AnnotationType::Custom("Watermark".to_string()))
            .position(TextPosition::TopRight)
            .fps(30),
    );
    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(ann) = result.annotation_result {
        println!("   ✅ Output: {}", ann.output_path);
    }

    Ok(())
}
