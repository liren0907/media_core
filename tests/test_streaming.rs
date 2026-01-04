use media_core::streaming::{ExtractionMode, FrameData, SamplingStrategy, StreamExtractor};
use std::path::Path;

/// Example 1: Single Frame Mode (Custom strategy with single index)
#[test]
fn test_stream_extractor_single_frame() {
    println!("=== Test: StreamExtractor Single Frame ===");

    let video_path = "data/test.mp4";
    if !Path::new(video_path).exists() {
        println!("⚠️  Skipping: data/test.mp4 not found");
        return;
    }

    let mut extractor = StreamExtractor::new(video_path, None).expect("Failed to create extractor");
    extractor.set_strategy(SamplingStrategy::Custom(vec![0]));

    let frames = extractor.extract(None).expect("Extraction failed");

    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].index, 0);
    assert!(!frames[0].data.is_empty());

    println!("✅ Extracted single frame at index 0");
    println!("   Base64 length: {} bytes", frames[0].data.len());
    println!("=== Test Passed ===\n");
}

/// Example 2a: EveryNth strategy
#[test]
fn test_stream_extractor_every_nth() {
    println!("=== Test: StreamExtractor EveryNth ===");

    let video_path = "data/test.mp4";
    if !Path::new(video_path).exists() {
        println!("⚠️  Skipping: data/test.mp4 not found");
        return;
    }

    let mut extractor = StreamExtractor::new(video_path, Some(SamplingStrategy::EveryNth(50)))
        .expect("Failed to create extractor");

    let frames: Vec<FrameData> = extractor.extract(None).expect("Extraction failed");

    assert!(!frames.is_empty());
    // First frame should be index 0, second should be 50
    assert_eq!(frames[0].index, 0);
    if frames.len() > 1 {
        assert_eq!(frames[1].index, 50);
    }

    println!("✅ EveryNth(50): {} frames extracted", frames.len());
    println!("=== Test Passed ===\n");
}

/// Example 2b: FirstN strategy
#[test]
fn test_stream_extractor_first_n() {
    println!("=== Test: StreamExtractor FirstN ===");

    let video_path = "data/test.mp4";
    if !Path::new(video_path).exists() {
        println!("⚠️  Skipping: data/test.mp4 not found");
        return;
    }

    let mut extractor = StreamExtractor::new(video_path, None).expect("Failed to create extractor");
    extractor.set_strategy(SamplingStrategy::FirstN(5));

    let frames = extractor.extract(None).expect("Extraction failed");

    assert_eq!(frames.len(), 5);
    for (i, frame) in frames.iter().enumerate() {
        assert_eq!(frame.index, i);
    }

    println!("✅ FirstN(5): {} frames extracted", frames.len());
    println!("=== Test Passed ===\n");
}

/// Example 2c: Range strategy
#[test]
fn test_stream_extractor_range() {
    println!("=== Test: StreamExtractor Range ===");

    let video_path = "data/test.mp4";
    if !Path::new(video_path).exists() {
        println!("⚠️  Skipping: data/test.mp4 not found");
        return;
    }

    let mut extractor = StreamExtractor::new(video_path, None).expect("Failed to create extractor");
    extractor.set_strategy(SamplingStrategy::Range(0, 10));
    extractor.set_mode(ExtractionMode::Sequential);

    let frames = extractor.extract(None).expect("Extraction failed");

    assert_eq!(frames.len(), 10);
    assert_eq!(frames[0].index, 0);
    assert_eq!(frames[9].index, 9);

    println!("✅ Range(0, 10): {} frames extracted", frames.len());
    println!("=== Test Passed ===\n");
}

/// Example 2f: Scale factor comparison
#[test]
fn test_stream_extractor_scale_factor() {
    println!("=== Test: StreamExtractor Scale Factor ===");

    let video_path = "data/test.mp4";
    if !Path::new(video_path).exists() {
        println!("⚠️  Skipping: data/test.mp4 not found");
        return;
    }

    let mut extractor = StreamExtractor::new(video_path, Some(SamplingStrategy::FirstN(1)))
        .expect("Failed to create extractor");

    // Full size
    let frame_full = extractor.extract(None).expect("Extraction failed");
    let full_size = frame_full[0].data.len();

    // Half size
    let frame_half = extractor.extract(Some(0.5)).expect("Extraction failed");
    let half_size = frame_half[0].data.len();

    // Quarter size
    let frame_quarter = extractor.extract(Some(0.25)).expect("Extraction failed");
    let quarter_size = frame_quarter[0].data.len();

    // Smaller scale should produce smaller data
    assert!(half_size < full_size, "Half scale should be smaller");
    assert!(quarter_size < half_size, "Quarter scale should be smaller");

    println!("✅ Scale factor comparison:");
    println!("   Full (100%):    {} bytes", full_size);
    println!("   Half (50%):     {} bytes", half_size);
    println!("   Quarter (25%):  {} bytes", quarter_size);
    println!("=== Test Passed ===\n");
}
