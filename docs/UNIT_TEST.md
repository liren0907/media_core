# RTSP Unit Test

## Overview
A single unit test `test_rtsp_stream_extraction` verifies that the RTSP stream extractor can connect, record a segment, and exit cleanly.

## Prerequisites
1.  **MediaMTX Server**: Must be running.
2.  **RTSP Stream**: A stream must be active at `rtsp://localhost:8554/mystream`.

## Running the Test

```bash
cargo test test_rtsp_stream_extraction -- --nocapture
```

## What it Does
1.  Connects to `rtsp://localhost:8554/mystream`.
2.  Records video segments to the `output/` directory.
3.  Runs in **single-run mode** (stops after ~30 seconds).
4.  Verifies:
    *   Output directory creation.
    *   Video file existence (`.mp4`).
    *   Correct filename format.

## Troubleshooting
*   **Permission Denied**: Ensure `run_mtx_server.sh` and `run_rtsp-streaming.sh` are executable.
*   **FFmpeg Error**: Ensure FFmpeg is installed and accessible.
