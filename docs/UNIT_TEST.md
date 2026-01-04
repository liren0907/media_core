# Unit Tests

## Table of Contents

- [Config Generation Tests](#config-generation-tests)
- [RTSP Stream Extraction Tests](#rtsp-stream-extraction-tests)
- [HLS Conversion Tests](#hls-conversion-tests)
- [Process Module Tests](#process-module-tests)
- [Metadata Module Tests](#metadata-module-tests)
- [Annotation Module Tests](#annotation-module-tests)
- [Benchmark Module Tests](#benchmark-module-tests)
- [Streaming Module Tests](#streaming-module-tests)

---

## Config Generation Tests

**File**: `tests/test_config_generation.rs`

### Tests

1. **`test_rtsp_config_generation`** - Generates and validates default RTSP config
2. **`test_process_config_generation`** - Generates and validates default process config

### Run
```bash
cargo test --test test_config_generation
```

---

## RTSP Stream Extraction Tests

**File**: `tests/test_rtsp_extraction.rs`

### Tests

1. **`test_rtsp_stream_extraction`** - FFmpeg mode (records `.mp4` to `output/`)
2. **`test_hls_streaming`** - HLS mode (generates `.m3u8` + `.ts` to `hls_test_output/`)

### Prerequisites
- FFmpeg installed

**Note**: Tests work without a real RTSP server (uses `run_once` timeout).

### Run All RTSP Tests
```bash
cargo test --test test_rtsp_extraction -- --nocapture
```

### Run Individual Tests
```bash
# FFmpeg mode
cargo test test_rtsp_stream_extraction -- --nocapture

# HLS mode
cargo test test_hls_streaming -- --nocapture
```

### Test Comparison

| Test | Mode | Output | Duration |
|------|------|--------|----------|
| `test_rtsp_stream_extraction` | FFmpeg | `output/camera_*/*.mp4` | ~36s |
| `test_hls_streaming` | HLS | `hls_test_output/*.m3u8, *.ts` | ~10s |

### Troubleshooting

**FFmpeg not found**: Install FFmpeg
```bash
brew install ffmpeg  # macOS
sudo apt-get install ffmpeg  # Ubuntu
```

**With real RTSP server** (optional):
```bash
cd data/rtsp
./run_mtx_server.sh              # Terminal 1
./run_rtsp-streaming.sh          # Terminal 2
cargo test --test test_rtsp_extraction -- --nocapture  # Terminal 3
```

---

## HLS Conversion Tests

**File**: `tests/test_hls_conversion.rs`

### Tests

1. **`test_hls_config_generation`** - Validates HLS config defaults and JSON serialization
2. **`test_hls_conversion`** - Converts `data/test.mp4` to HLS format

### Prerequisites
- FFmpeg installed
- Test video at `data/test.mp4`

### Run
```bash
cargo test --test test_hls_conversion -- --nocapture
```

### Test Comparison

| Test | Output | Duration |
|------|--------|----------|
| `test_hls_config_generation` | None | <1s |
| `test_hls_conversion` | `hls_test_output/*.m3u8, *.ts` | ~5s |

---

## Process Module Tests

**File**: `tests/test_process_module.rs`

### Tests

1.  **`test_process_config_generation`** - Validates Process config defaults and JSON serialization.
2.  **`test_video_extraction`** - **Integration Test**: Extracts frames from `data/test.mp4`.
    -   Mode: "skip" (Direct frame extraction, no video creation).
    -   Output: Temporary directory (cleaned up automatically).
    -   Verification: Checks that `.jpg` files are created.
3.  **`test_video_creation`** - **Integration Test**: Creates a time-lapse video from `data/test.mp4`.
    -   Mode: "direct" (Direct video creation).
    -   Output: Temporary directory.
    -   Verification: Checks that `.mp4` file is created and has size > 0.

### Prerequisites

-   Test video at `data/test.mp4`.

### Run

```bash
cargo test --test test_process_module -- --nocapture
```

### Test Comparison

| Test | Mode | Output | Duration |
| :--- | :--- | :--- | :--- |
| `test_video_extraction` | Skip (Direct) | `.jpg` frames | ~2s |
| `test_video_creation` | Direct | `.mp4` video | ~2s |

---

## Metadata Module Tests

**File**: `tests/test_metadata.rs`

### Tests

1. **`test_get_media_info`** - **Unit Test**: Validates metadata extraction.
    -   **Input**: `data/test.mp4`
    -   **Verification**: Checks that:
        -   Function returns `Ok`.
        -   `media_type` is "video".
        -   Basic fields (`width`, `height`, `duration_seconds`) are valid (> 0).

### Prerequisites

-   Test video at `data/test.mp4`.

### Run

```bash
cargo test --test test_metadata -- --nocapture
```

### Test Comparison

| Test | Output | Duration |
| :--- | :--- | :--- |
| `test_get_media_info` | Metadata logs | <1s |

---

## Annotation Module Tests

**File**: `tests/test_annotation.rs`

### Tests

1. **`test_single_frame_annotation`** - Example 1: Single image with filename overlay (TopLeft)
2. **`test_video_from_frames_filename`** - Example 2: Video from frames with filename annotation
3. **`test_video_from_frames_timestamp`** - Example 3: Video from frames with timestamp annotation (BottomLeft)
4. **`test_video_from_frames_custom_text`** - Example 4: Video from frames with custom watermark (TopRight)

### Prerequisites

- Test video at `data/test.mp4`
- FFmpeg installed

### Run

```bash
cargo test --test test_annotation -- --nocapture
```

### Test Comparison

| Test | Annotation Type | Output | Duration |
| :--- | :--- | :--- | :--- |
| `test_single_frame_annotation` | Filename | `.jpg` image | <1s |
| `test_video_from_frames_filename` | Filename | `.mp4` video | ~1s |
| `test_video_from_frames_timestamp` | Timestamp | `.mp4` video | ~1s |
| `test_video_from_frames_custom_text` | Custom | `.mp4` video | ~1s |

---

## Benchmark Module Tests

**File**: `tests/test_benchmark.rs`

### Tests

1. **`test_benchmark_single_function`** - Benchmark a simple function with multiple runs
2. **`test_benchmark_result_summary`** - Verify `print_summary()` output
3. **`test_benchmark_result_to_json`** - Export benchmark results to JSON file

### Prerequisites

- None (uses simple in-memory operations)

### Run

```bash
cargo test --test test_benchmark -- --nocapture
```

### Test Comparison

| Test | Output | Duration |
| :--- | :--- | :--- |
| `test_benchmark_single_function` | Console stats | <1s |
| `test_benchmark_result_summary` | Console summary | <1s |
| `test_benchmark_result_to_json` | `.json` file | <1s |

---

## Streaming Module Tests

**File**: `tests/test_streaming.rs`

### Tests

1. **`test_stream_extractor_single_frame`** - Extract single frame with Custom strategy
2. **`test_stream_extractor_every_nth`** - Extract with EveryNth(50) strategy
3. **`test_stream_extractor_first_n`** - Extract with FirstN(5) strategy
4. **`test_stream_extractor_range`** - Extract with Range(0, 10) strategy
5. **`test_stream_extractor_scale_factor`** - Compare scale factors (100%, 50%, 25%)

### Prerequisites

- Test video at `data/test.mp4`

### Run

```bash
cargo test --test test_streaming -- --nocapture
```

### Test Comparison

| Test | Strategy | Output | Duration |
| :--- | :--- | :--- | :--- |
| `test_stream_extractor_single_frame` | Custom | 1 frame | <1s |
| `test_stream_extractor_every_nth` | EveryNth(50) | ~39 frames | ~1s |
| `test_stream_extractor_first_n` | FirstN(5) | 5 frames | <1s |
| `test_stream_extractor_range` | Range(0,10) | 10 frames | <1s |
| `test_stream_extractor_scale_factor` | FirstN(1) | Size comparison | ~2s |
