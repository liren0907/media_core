use media_core::metadata::get_media_info;
use std::path::Path;

/// Test get_media_info with a real file
#[test]
fn test_get_media_info() {
    println!("=== Test: Get Media Info ===");

    // Setup path
    let input_video = Path::new("data/test.mp4");
    if !input_video.exists() {
        println!("⚠️ Skipping test: data/test.mp4 not found");
        return;
    }

    // Call get_media_info
    // We include thumbnail generation to test that path too
    let result = get_media_info(input_video.to_str().unwrap(), true);

    // Verify
    assert!(result.is_ok(), "get_media_info failed: {:?}", result.err());
    let metadata = result.unwrap();

    println!("Metadata retrieved successfully");

    // Basic assertions
    assert_eq!(
        metadata.media_type, "video",
        "Expected media_type to be 'video'"
    );

    println!(
        "Video Info: {}x{} @ {:?}fps, {:?}s",
        metadata.width, metadata.height, metadata.fps, metadata.duration_seconds
    );

    assert!(metadata.width > 0, "Invalid width");
    assert!(metadata.height > 0, "Invalid height");
    assert!(
        metadata.duration_seconds.unwrap_or(0.0) > 0.0,
        "Invalid duration"
    );

    if let Some(thumb) = &metadata.thumbnail {
        println!("Thumbnail generated, length: {}", thumb.len());
    } else {
        println!("No thumbnail generated");
    }

    println!("✅ get_media_info passed");
}
