use media_core::annotation::{
    AnnotationConfig, AnnotationType, DataSource, FrameAnnotator, TextPosition, VideoOutputConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let output_dir = "output/annotation";
    std::fs::create_dir_all(output_dir)?;

    // ============================================
    // Example 1: Single Frame Annotation
    // ============================================

    let sample_image = "output/video_process/test/frame_0.jpg";
    let config = AnnotationConfig {
        input: DataSource::Image(sample_image.to_string()),
        output_path: format!("{}/annotated_frame.jpg", output_dir),
        text_position: TextPosition::TopLeft,
        annotation_type: AnnotationType::Filename,
        ..Default::default()
    };

    FrameAnnotator::new(config).process()?;
    println!("Example 1: Single frame annotation completed");

    // ============================================
    // Example 2: Video from Frames (Filename)
    // ============================================

    let frames_dir = "output/video_process/test";
    let config = AnnotationConfig {
        input: DataSource::FrameDir(frames_dir.to_string()),
        output_path: format!("{}/annotated_filename.mp4", output_dir),
        text_position: TextPosition::TopLeft,
        annotation_type: AnnotationType::Filename,
        source_fps: Some(30.0),
        video_encoding: Some(VideoOutputConfig {
            fps: 30,
            filename: "".to_string(),
        }),
    };

    FrameAnnotator::new(config).process()?;
    println!("Example 2: Created video with filename annotation");

    // ============================================
    // Example 3: Video from Frames (Timestamp)
    // ============================================

    let config = AnnotationConfig {
        input: DataSource::FrameDir(frames_dir.to_string()),
        output_path: format!("{}/annotated_timestamp.mp4", output_dir),
        text_position: TextPosition::BottomLeft,
        annotation_type: AnnotationType::Timestamp,
        source_fps: Some(30.0),
        video_encoding: Some(VideoOutputConfig {
            fps: 30,
            filename: "".to_string(),
        }),
    };

    FrameAnnotator::new(config).process()?;
    println!("Example 3: Created video with timestamp annotation");

    // ============================================
    // Example 4: Video from Frames (Custom Text)
    // ============================================

    let config = AnnotationConfig {
        input: DataSource::FrameDir(frames_dir.to_string()),
        output_path: format!("{}/annotated_custom.mp4", output_dir),
        text_position: TextPosition::TopRight,
        annotation_type: AnnotationType::Custom("Watermark".to_string()),
        source_fps: Some(30.0),
        video_encoding: Some(VideoOutputConfig {
            fps: 30,
            filename: "".to_string(),
        }),
    };

    FrameAnnotator::new(config).process()?;
    println!("Example 4: Created video with custom text annotation");

    println!("\nAnnotation examples completed!");
    Ok(())
}
