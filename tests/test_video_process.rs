use media_core::video_process::{ExtractionMode, FrameExtractor, SaveMode};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

/// Test FrameExtractor with OpenCVInterval mode (Example 2 from binary)
#[test]
fn test_frame_extractor_opencv_interval() {
    println!("=== Test: FrameExtractor OpenCVInterval ===");

    let video_path = "data/test.mp4";
    if !Path::new(video_path).exists() {
        println!("⚠️  Skipping: data/test.mp4 not found");
        return;
    }

    let output_dir = tempdir().expect("Failed to create temp dir");
    let output_path = output_dir.path().to_str().unwrap();

    let extractor = FrameExtractor::new(video_path, output_path)
        .with_interval(100)
        .with_mode(ExtractionMode::OpenCVInterval)
        .with_save_mode(SaveMode::SingleDirectory);

    assert_eq!(extractor.video_path(), video_path);
    assert_eq!(extractor.frame_interval(), 100);
    assert_eq!(extractor.extraction_mode(), ExtractionMode::OpenCVInterval);
    assert_eq!(extractor.save_mode(), SaveMode::SingleDirectory);

    let result = extractor.extract();
    assert!(result.is_ok(), "Extraction failed: {:?}", result.err());

    // Verify frames were created in the output directory
    let frames: Vec<_> = fs::read_dir(output_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "jpg"))
        .collect();

    assert!(!frames.is_empty(), "No frames extracted");
    println!("✅ OpenCVInterval: {} frames extracted", frames.len());
    println!("=== Test Passed ===\n");
}

/// Test FrameExtractor with Parallel mode (Example 5 from binary)
#[test]
fn test_frame_extractor_parallel() {
    println!("=== Test: FrameExtractor Parallel ===");

    let video_path = "data/test.mp4";
    if !Path::new(video_path).exists() {
        println!("⚠️  Skipping: data/test.mp4 not found");
        return;
    }

    let output_dir = tempdir().expect("Failed to create temp dir");
    let output_path = output_dir.path().to_str().unwrap();

    let extractor = FrameExtractor::new(video_path, output_path)
        .with_interval(100)
        .with_mode(ExtractionMode::Parallel)
        .with_save_mode(SaveMode::SingleDirectory);

    assert_eq!(extractor.extraction_mode(), ExtractionMode::Parallel);
    assert_eq!(extractor.save_mode(), SaveMode::SingleDirectory);

    let result = extractor.extract();
    assert!(
        result.is_ok(),
        "Parallel extraction failed: {:?}",
        result.err()
    );

    // Verify frames in single directory
    let frames: Vec<_> = fs::read_dir(output_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "jpg"))
        .collect();

    assert!(!frames.is_empty(), "No frames extracted");
    println!("✅ Parallel + SingleDirectory: {} frames", frames.len());
    println!("=== Test Passed ===\n");
}

/// Test SaveMode::MultipleDirectory (Example 6 from binary)
#[test]
fn test_frame_extractor_multiple_directory() {
    println!("=== Test: FrameExtractor MultipleDirectory ===");

    let video_path = "data/test.mp4";
    if !Path::new(video_path).exists() {
        println!("⚠️  Skipping: data/test.mp4 not found");
        return;
    }

    let output_dir = tempdir().expect("Failed to create temp dir");
    let output_path = output_dir.path().to_str().unwrap();

    let extractor = FrameExtractor::new(video_path, output_path)
        .with_interval(100)
        .with_mode(ExtractionMode::OpenCVInterval)
        .with_save_mode(SaveMode::MultipleDirectory);

    assert_eq!(extractor.save_mode(), SaveMode::MultipleDirectory);

    let result = extractor.extract();
    assert!(result.is_ok(), "Extraction failed: {:?}", result.err());

    // Verify subdirectory was created (named after video)
    let subdirs: Vec<_> = fs::read_dir(output_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    assert!(!subdirs.is_empty(), "No subdirectory created");
    println!("✅ MultipleDirectory: subdirectory created");
    println!("=== Test Passed ===\n");
}
