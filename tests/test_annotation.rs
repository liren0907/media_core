use media_core::annotation::{
    AnnotationConfig, AnnotationType, DataSource, FrameAnnotator, TextPosition, VideoOutputConfig,
};
use opencv::{core::Mat, imgcodecs, prelude::*};
use std::path::Path;
use tempfile::tempdir;

// Helper: Ensure test image exists
fn ensure_test_image() -> bool {
    let input_image = Path::new("data/test.jpg");
    if !input_image.exists() {
        let test_video = Path::new("data/test.mp4");
        if test_video.exists() {
            if let Ok(mut cap) =
                opencv::videoio::VideoCapture::from_file("data/test.mp4", opencv::videoio::CAP_ANY)
            {
                let mut frame = Mat::default();
                if cap.read(&mut frame).unwrap_or(false) && !frame.empty() {
                    let _ =
                        imgcodecs::imwrite("data/test.jpg", &frame, &opencv::core::Vector::new());
                }
            }
        }
    }
    input_image.exists()
}

// Helper: Create test frames in a temp directory (avoids race conditions)
fn create_test_frames() -> Option<tempfile::TempDir> {
    let test_video = Path::new("data/test.mp4");
    if !test_video.exists() {
        return None;
    }

    let temp_dir = tempdir().ok()?;
    let frames_path = temp_dir.path();

    if let Ok(mut cap) =
        opencv::videoio::VideoCapture::from_file("data/test.mp4", opencv::videoio::CAP_ANY)
    {
        for i in 0..10 {
            let mut frame = Mat::default();
            if cap.read(&mut frame).unwrap_or(false) && !frame.empty() {
                let path = frames_path.join(format!("frame_{:04}.jpg", i));
                let _ = imgcodecs::imwrite(
                    path.to_str().unwrap(),
                    &frame,
                    &opencv::core::Vector::new(),
                );
            }
        }
    }
    Some(temp_dir)
}

// ============================================================================
// Unit Tests Aligned to bin/annotation.rs Examples
// ============================================================================

/// Example 1: Single Frame Annotation (Filename, TopLeft)
#[test]
fn test_single_frame_annotation() {
    println!("=== Test: Example 1 - Single Frame Annotation ===");

    if !ensure_test_image() {
        println!("⚠️  Skipping: data/test.jpg not found");
        return;
    }

    let output_dir = tempdir().expect("Failed to create temp dir");
    let output_path = output_dir.path().join("annotated_frame.jpg");

    let config = AnnotationConfig {
        input: DataSource::Image("data/test.jpg".to_string()),
        output_path: output_path.to_str().unwrap().to_string(),
        text_position: TextPosition::TopLeft,
        annotation_type: AnnotationType::Filename,
        ..Default::default()
    };

    let result = FrameAnnotator::new(config).process();

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    assert!(output_path.exists());
    println!("✅ Single frame annotation completed");
    println!("=== Test Passed ===\n");
}

/// Example 2: Video from Frames (Filename, TopLeft)
#[test]
fn test_video_from_frames_filename() {
    println!("=== Test: Example 2 - Video from Frames (Filename) ===");

    let frames_dir = match create_test_frames() {
        Some(dir) => dir,
        None => {
            println!("⚠️  Skipping: data/test.mp4 not found");
            return;
        }
    };

    let output_dir = tempdir().expect("Failed to create temp dir");
    let output_path = output_dir.path().join("annotated_filename.mp4");

    let config = AnnotationConfig {
        input: DataSource::FrameDir(frames_dir.path().to_str().unwrap().to_string()),
        output_path: output_path.to_str().unwrap().to_string(),
        text_position: TextPosition::TopLeft,
        annotation_type: AnnotationType::Filename,
        source_fps: Some(30.0),
        video_encoding: Some(VideoOutputConfig {
            fps: 30,
            filename: "".to_string(),
        }),
    };

    let result = FrameAnnotator::new(config).process();

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    assert!(output_path.exists());
    println!("✅ Created video with filename annotation");
    println!("=== Test Passed ===\n");
}

/// Example 3: Video from Frames (Timestamp, BottomLeft)
#[test]
fn test_video_from_frames_timestamp() {
    println!("=== Test: Example 3 - Video from Frames (Timestamp) ===");

    let frames_dir = match create_test_frames() {
        Some(dir) => dir,
        None => {
            println!("⚠️  Skipping: data/test.mp4 not found");
            return;
        }
    };

    let output_dir = tempdir().expect("Failed to create temp dir");
    let output_path = output_dir.path().join("annotated_timestamp.mp4");

    let config = AnnotationConfig {
        input: DataSource::FrameDir(frames_dir.path().to_str().unwrap().to_string()),
        output_path: output_path.to_str().unwrap().to_string(),
        text_position: TextPosition::BottomLeft,
        annotation_type: AnnotationType::Timestamp,
        source_fps: Some(30.0),
        video_encoding: Some(VideoOutputConfig {
            fps: 30,
            filename: "".to_string(),
        }),
    };

    let result = FrameAnnotator::new(config).process();

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    assert!(output_path.exists());
    println!("✅ Created video with timestamp annotation");
    println!("=== Test Passed ===\n");
}

/// Example 4: Video from Frames (Custom Text, TopRight)
#[test]
fn test_video_from_frames_custom_text() {
    println!("=== Test: Example 4 - Video from Frames (Custom Text) ===");

    let frames_dir = match create_test_frames() {
        Some(dir) => dir,
        None => {
            println!("⚠️  Skipping: data/test.mp4 not found");
            return;
        }
    };

    let output_dir = tempdir().expect("Failed to create temp dir");
    let output_path = output_dir.path().join("annotated_custom.mp4");

    let config = AnnotationConfig {
        input: DataSource::FrameDir(frames_dir.path().to_str().unwrap().to_string()),
        output_path: output_path.to_str().unwrap().to_string(),
        text_position: TextPosition::TopRight,
        annotation_type: AnnotationType::Custom("Watermark".to_string()),
        source_fps: Some(30.0),
        video_encoding: Some(VideoOutputConfig {
            fps: 30,
            filename: "".to_string(),
        }),
    };

    let result = FrameAnnotator::new(config).process();

    assert!(result.is_ok(), "Failed: {:?}", result.err());
    assert!(output_path.exists());
    println!("✅ Created video with custom text annotation");
    println!("=== Test Passed ===\n");
}
