use media_core::{CaptureConfig, RTSPCapture, SavingOption, HLSConfig};
use std::path::Path;

#[test]
fn test_rtsp_stream_extraction() {
    let test_config = CaptureConfig {
        rtsp_url: "rtsp://localhost:8554/mystream".to_string(),
        rtsp_url_list: vec![],
        output_directory: "output".to_string(),
        show_preview: false,
        saving_option: SavingOption::Single,
        saved_time_duration: 30,
        use_fps: false,
        fps: 30.0,
        hls: HLSConfig::default(),
    };

    let mut capture = RTSPCapture::new(
        test_config.rtsp_url.clone(),
        test_config.output_directory.clone(),
        test_config.show_preview,
        test_config.saved_time_duration,
        test_config.use_fps,
        test_config.fps,
        None, // No HLS for this test
        true, // run_once: true for testing
    )
    .expect("Failed to create RTSP capture");

    capture.process_stream().expect("Stream processing failed");

    let output_path = Path::new("output");
    assert!(output_path.exists(), "Output directory should exist");

    let camera_dirs: Vec<_> = std::fs::read_dir(output_path)
        .expect("Failed to read output directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    assert!(
        !camera_dirs.is_empty(),
        "Should have at least one camera directory"
    );

    let camera_dir = &camera_dirs[0].path();
    let video_files: Vec<_> = std::fs::read_dir(camera_dir)
        .expect("Failed to read camera directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "mp4"))
        .collect();

    assert!(
        !video_files.is_empty(),
        "Should have at least one video segment"
    );

    let first_video = &video_files[0];
    let filename_osstr = first_video.file_name();
    let filename = filename_osstr.to_string_lossy();
    assert!(
        filename.starts_with("segment_"),
        "Filename should start with 'segment_'"
    );
    assert!(
        filename.ends_with(".mp4"),
        "Filename should end with '.mp4'"
    );

    println!(
        "✅ Test passed! Created {} video file(s)",
        video_files.len()
    );
}

#[test]
fn test_hls_streaming() {
    // Create HLS configuration with enabled flag
    let hls_config = HLSConfig {
        enabled: true,
        output_directory: "hls_test_output".to_string(),
        segment_duration: 2,    // Short segments for testing
        playlist_size: 3,       // Small playlist for testing
    };

    // Create RTSPCapture with HLS enabled
    let mut capture = RTSPCapture::new(
        "rtsp://localhost:8554/mystream".to_string(),
        "output".to_string(), // Won't be used (HLS has its own directory)
        false,                // show_preview
        30,                   // segment_duration_secs
        false,                // use_custom_fps
        30.0,                 // custom_fps
        Some(hls_config.clone()), // HLS config
        true,                 // run_once: true for testing
    )
    .expect("Failed to create RTSP capture");

    // Process stream (should trigger HLS mode)
    capture.process_stream().expect("Stream processing failed");

    // Verify HLS output directory exists
    let hls_output_path = Path::new("hls_test_output");
    assert!(
        hls_output_path.exists(),
        "HLS output directory should exist"
    );

    // Verify playlist.m3u8 exists
    let playlist_path = hls_output_path.join("playlist.m3u8");
    assert!(
        playlist_path.exists(),
        "playlist.m3u8 should exist in HLS output directory"
    );

    // Verify .ts segment files exist
    let ts_files: Vec<_> = std::fs::read_dir(hls_output_path)
        .expect("Failed to read HLS output directory")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext == "ts")
        })
        .collect();

    assert!(
        !ts_files.is_empty(),
        "Should have at least one .ts segment file"
    );

    // Verify .ts filename format
    let first_ts = &ts_files[0];
    let filename_osstr = first_ts.file_name();
    let filename = filename_osstr.to_string_lossy();
    assert!(
        filename.ends_with(".ts"),
        "TS filename should end with '.ts'"
    );

    println!(
        "✅ HLS Test passed! Created playlist.m3u8 and {} .ts segment(s)",
        ts_files.len()
    );
}
