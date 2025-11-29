use media_core::process::ProcessConfig;
use media_core::CaptureConfig;
use std::fs::File;
use std::io::Read;
use tempfile::tempdir;

#[test]
fn test_rtsp_config_generation() {
    // Create a temporary directory
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("config.json");
    let file_path_str = file_path
        .to_str()
        .expect("Failed to convert path to string");

    // Generate the default RTSP config
    media_core::rtsp::generate_default_config(file_path_str)
        .expect("Failed to generate default RTSP config");

    // Verify the file exists
    assert!(file_path.exists(), "Config file was not created");

    // Read and parse the file to ensure it's valid JSON and matches the struct
    let mut file = File::open(&file_path).expect("Failed to open config file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read config file");

    let config: CaptureConfig =
        serde_json::from_str(&content).expect("Failed to deserialize RTSP config");

    // Basic validation of default values (optional, but good for sanity)
    assert_eq!(config.output_directory, "media");
    assert_eq!(config.fps, 30.0);
}

#[test]
fn test_process_config_generation() {
    // Create a temporary directory
    let dir = tempdir().expect("Failed to create temp dir");
    let file_path = dir.path().join("process_config.json");
    let file_path_str = file_path
        .to_str()
        .expect("Failed to convert path to string");

    // Generate the default Process config
    media_core::process::generate_default_config(file_path_str)
        .expect("Failed to generate default Process config");

    // Verify the file exists
    assert!(file_path.exists(), "Process config file was not created");

    // Read and parse the file to ensure it's valid JSON and matches the struct
    let mut file = File::open(&file_path).expect("Failed to open process config file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed to read process config file");

    let config: ProcessConfig =
        serde_json::from_str(&content).expect("Failed to deserialize Process config");

    // Basic validation of default values
    assert_eq!(config.input_path, "input");
    assert_eq!(config.output_path, "output");
}
