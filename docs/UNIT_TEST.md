# Unit Tests

This document outlines the unit and integration tests available in the project.

## Table of Contents

- [Config Generation Tests](#config-generation-tests)
- [RTSP Stream Extraction Tests](#rtsp-stream-extraction-tests)

---

## Config Generation Tests

### Overview
These tests verify that the application can correctly generate default configuration files for both RTSP capture and Video Processing modes.

### Test File
`tests/test_config_generation.rs`

### Tests Included
1.  **`test_rtsp_config_generation`**:
    -   Generates a default `config.json`.
    -   Verifies the file exists.
    -   Deserializes the content to ensure it matches the `CaptureConfig` structure.
2.  **`test_process_config_generation`**:
    -   Generates a default `process_config.json`.
    -   Verifies the file exists.
    -   Deserializes the content to ensure it matches the `ProcessConfig` structure.

### Running the Tests
```bash
cargo test --test test_config_generation
```

---

## RTSP Stream Extraction Tests

### Overview
A single unit test `test_rtsp_stream_extraction` verifies that the RTSP stream extractor can connect, record a segment, and exit cleanly.

### Test File
`tests/test_rtsp_extraction.rs`

### Prerequisites
1.  **MediaMTX Server**: Must be running.
2.  **RTSP Stream**: A stream must be active at `rtsp://localhost:8554/mystream`.

### Running the Test
```bash
cargo test test_rtsp_stream_extraction -- --nocapture
```

### What it Does
1.  Connects to `rtsp://localhost:8554/mystream`.
2.  Records video segments to the `output/` directory.
3.  Runs in **single-run mode** (stops after ~30 seconds).
4.  Verifies:
    *   Output directory creation.
    *   Video file existence (`.mp4`).
    *   Correct filename format.

### Troubleshooting
*   **Permission Denied**: Ensure `run_mtx_server.sh` and `run_rtsp-streaming.sh` are executable.
*   **FFmpeg Error**: Ensure FFmpeg is installed and accessible.
