use media_core::process::{
    ProcessConfig, VideoExtractionConfig, create_video_processor, generate_default_config,
};
use std::fs;
use std::io::Read;
use std::path::Path;
use tempfile::tempdir;

/// Test programmatic config generation
#[test]
fn test_process_config_generation() {
    println!("=== Test: Process Config Generation ===");

    // Create a temporary directory
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("process_config.json");
    let file_path_str = file_path.to_str().expect("Path conversion failed");

    // Generate
    generate_default_config(file_path_str).expect("Failed to generate default config");
    assert!(file_path.exists(), "Config file not created");

    // Verify content
    let mut file = fs::File::open(&file_path).expect("Failed to open config");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read config");

    let config: ProcessConfig = serde_json::from_str(&content).expect("Deserialization failed");

    // Validate defaults
    assert_eq!(config.input_path, "input");
    assert!(config.video_config.is_some());
    let video_config = config.video_config.unwrap();
    assert_eq!(video_config.output_fps, 30);

    println!("✅ Config generation and deserialization pass");
}

/// Test full video extraction flow (SKIP mode)
#[test]
fn test_video_extraction() {
    println!("=== Test: Video Extraction (Skip Mode) ===");

    // Setup paths
    let input_video = Path::new("data/test.mp4");
    if !input_video.exists() {
        println!("⚠️ Skipping test: data/test.mp4 not found");
        return;
    }

    let output_root = tempdir().expect("Failed to create temp output dir");
    let output_path_str = output_root.path().to_str().unwrap();

    // Create Config for "Skip" Mode (Direct Extraction)
    // We manually construct the internal config just like run_extraction_mode does
    // But since run_extraction_mode is in CLI, we'll simulate the config file approach
    // to test the whole VideoProcessor flow.

    // We create a temp file for the config
    let config_dir = tempdir().expect("Failed to create temp config dir");
    let config_file_path = config_dir.path().join("test_extract_config.json");

    let video_config = VideoExtractionConfig {
        input_directories: vec![input_video.to_str().unwrap().to_string()],
        output_directory: output_path_str.to_string(),
        output_prefix: "test_extract".to_string(),
        num_threads: Some(1),
        output_fps: 30,
        frame_interval: 100, // Extract every 100th frame for faster testing
        extraction_mode: "opencv".to_string(),
        create_summary_per_thread: Some(false),
        video_creation_mode: Some("skip".to_string()), // Test the new mode
        processing_mode: Some("sequential".to_string()),
        hardware_acceleration: Default::default(),
    };

    let config_json = serde_json::to_string(&video_config).unwrap();
    fs::write(&config_file_path, config_json).expect("Failed to write config file");

    // Run Processor
    let mut processor = create_video_processor().expect("Failed to create processor");
    processor
        .run_video_extraction(config_file_path.to_str().unwrap())
        .expect("Video extraction failed");

    // Verify Output
    // Expected structure: <output_root>/extract_<parent_dir_name>_frames/
    // Since input is data/test.mp4, parent is 'data', so tag is 'data'
    // expected folder: extract_data_frames

    let expected_dir = output_root.path().join("test_extract_data_frames");
    assert!(
        expected_dir.exists(),
        "Output directory 'test_extract_data_frames' was not created"
    );

    // Check for frames
    let frames: Vec<_> = fs::read_dir(&expected_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "jpg"))
        .collect();

    assert!(!frames.is_empty(), "No frames were extracted");
    println!("✅ Extracted {} frames to {:?}", frames.len(), expected_dir);
}

/// Test video creation flow (Time-lapse mode)
#[test]
fn test_video_creation() {
    println!("=== Test: Video Creation (Time-lapse Mode) ===");

    // Setup paths
    let input_video = Path::new("data/test.mp4");
    if !input_video.exists() {
        println!("⚠️ Skipping test: data/test.mp4 not found");
        return;
    }

    let output_root = tempdir().expect("Failed to create temp output dir");
    let output_path_str = output_root.path().to_str().unwrap();

    // Create Config for "Video Creation"
    let config_dir = tempdir().expect("Failed to create temp config dir");
    let config_file_path = config_dir.path().join("test_create_config.json");

    let video_config = VideoExtractionConfig {
        input_directories: vec![input_video.to_str().unwrap().to_string()],
        output_directory: output_path_str.to_string(),
        output_prefix: "test_create".to_string(),
        num_threads: Some(1),
        output_fps: 30,
        frame_interval: 100, // Sparse sampling for speed
        extraction_mode: "opencv".to_string(),
        create_summary_per_thread: Some(false),
        video_creation_mode: Some("direct".to_string()), // Test creation mode
        processing_mode: Some("sequential".to_string()),
        hardware_acceleration: Default::default(),
    };

    let config_json = serde_json::to_string(&video_config).unwrap();
    fs::write(&config_file_path, config_json).expect("Failed to write config file");

    // Run Processor
    let mut processor = create_video_processor().expect("Failed to create processor");
    processor
        .run_video_extraction(config_file_path.to_str().unwrap())
        .expect("Video creation process failed");

    // Verify Output
    // Expected file: <output_root>/test_create_<tag>.mp4
    // Tag for data/test.mp4 is 'data'
    let expected_video = output_root.path().join("test_create_data.mp4");

    assert!(
        expected_video.exists(),
        "Output video 'test_create_data.mp4' was not created"
    );
    assert!(
        expected_video.metadata().unwrap().len() > 0,
        "Output video is empty"
    );

    println!("✅ Created video file at {:?}", expected_video);
}
