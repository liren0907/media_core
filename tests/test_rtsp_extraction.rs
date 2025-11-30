use media_core::{CaptureConfig, RTSPCapture, SavingOption};
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
    };

    let mut capture = RTSPCapture::new(
        test_config.rtsp_url.clone(),
        test_config.output_directory.clone(),
        test_config.show_preview,
        test_config.saved_time_duration,
        test_config.use_fps,
        test_config.fps,
        true,
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
