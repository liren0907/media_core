# Unit Tests

## Table of Contents

- [Config Generation Tests](#config-generation-tests)
- [RTSP Stream Extraction Tests](#rtsp-stream-extraction-tests)

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

