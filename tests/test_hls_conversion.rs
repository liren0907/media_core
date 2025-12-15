use media_core::hls::{HLSConverter, HLSVodConfig};
use std::fs;
use std::path::PathBuf;

/// Test HLS config generation and default values
#[test]
fn test_hls_config_generation() {
    println!("=== Test: HLS Config Generation ===");

    // Test default config
    let default_config = HLSVodConfig::default();
    println!("Default config: {:?}", default_config);

    assert_eq!(default_config.segment_duration, 5);
    assert_eq!(default_config.playlist_filename, "playlist.m3u8");
    assert_eq!(default_config.force_keyframes, true);
    assert_eq!(default_config.profile, "baseline");
    assert_eq!(default_config.level, "3.0");
    println!("✅ Default values verified");

    // Test new() constructor
    let input = PathBuf::from("test_video.mp4");
    let output = PathBuf::from("test_output");
    let config = HLSVodConfig::new(input.clone(), output.clone());

    assert_eq!(config.input_path, input);
    assert_eq!(config.output_dir, output);
    assert_eq!(config.segment_duration, 5); // Should use default
    println!("✅ new() constructor verified");

    // Test JSON serialization
    let json = serde_json::to_string_pretty(&default_config).unwrap();
    println!("Serialized JSON:\n{}", json);
    assert!(json.contains("segment_duration"));
    assert!(json.contains("playlist_filename"));
    println!("✅ JSON serialization verified");

    println!("=== Test Passed ===\n");
}

/// Test HLS conversion with a real video file
#[test]
fn test_hls_conversion() {
    println!("=== Test: HLS Conversion ===");

    let input_path = PathBuf::from("data/test.mp4");
    let output_dir = PathBuf::from("hls_test_output");

    // Check if test video exists
    if !input_path.exists() {
        println!(
            "⚠️  Skipping test: test video not found at {}",
            input_path.display()
        );
        println!("   Place a test video at 'data/test.mp4' to run this test");
        return;
    }

    // Clean up previous test output
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).unwrap();
    }

    // Create config
    let config = HLSVodConfig::new(input_path.clone(), output_dir.clone());
    println!("Configuration:");
    println!("  Input:    {}", config.input_path.display());
    println!("  Output:   {}", config.output_dir.display());
    println!("  Segment:  {}s", config.segment_duration);
    println!("  Playlist: {}", config.playlist_filename);

    // Run conversion
    let converter = HLSConverter::new(config.clone());
    let result = converter.convert();

    assert!(result.is_ok(), "HLS conversion failed: {:?}", result.err());
    println!("✅ Conversion completed");

    // Verify output files
    let playlist_path = output_dir.join(&config.playlist_filename);
    assert!(playlist_path.exists(), "Playlist file not created");
    println!("✅ Playlist file exists: {}", playlist_path.display());

    // Check for .ts segment files
    let ts_files: Vec<_> = fs::read_dir(&output_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "ts").unwrap_or(false))
        .collect();

    assert!(!ts_files.is_empty(), "No .ts segment files created");
    println!("✅ Created {} segment files", ts_files.len());

    // List output files
    println!("\nOutput files:");
    for entry in fs::read_dir(&output_dir).unwrap() {
        let entry = entry.unwrap();
        println!("  - {}", entry.file_name().to_string_lossy());
    }

    println!("=== Test Passed ===\n");
}
