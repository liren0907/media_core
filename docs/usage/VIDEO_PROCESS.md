# Video Process Module

Frame extraction utilities for video files with multiple extraction modes.

**Location:** `src/video_process/`

## Module Structure

```
video_process/
├── mod.rs              # Exports
├── frame_extraction.rs # FrameExtractor struct, ExtractionMode, SaveMode
├── wrappers.rs         # Convenience wrapper functions
├── helpers.rs          # Output path utilities
├── types.rs            # VideoProcessConfig types
└── image_display.rs    # Display utilities
```

## Quick Start

### Basic Frame Extraction

```rust
use media_core::video_process::{FrameExtractor, ExtractionMode, SaveMode};

let extractor = FrameExtractor::new("video.mp4", "output/")
    .with_interval(30)               // Extract every 30th frame
    .with_mode(ExtractionMode::OpenCVInterval)
    .with_save_mode(SaveMode::SingleDirectory);

extractor.extract()?;
```

### Wrapper Functions

```rust
use media_core::video_process::*;

// Extract with OpenCV interval
extract_frames_opencv_interval("video.mp4", "output/", 30)?;

// Extract all frames with FFmpeg
extract_all_frames_ffmpeg("video.mp4", "output/")?;

// Parallel extraction with Rayon
extract_all_frames_rayon("video.mp4", "output/")?;
```

---

## Types

### `ExtractionMode`

```rust
pub enum ExtractionMode {
    OpenCVSequential,  // Read ALL frames sequentially
    OpenCVInterval,    // Seek to every Nth frame (default)
    FFmpeg,            // FFmpeg for ALL frames
    FFmpegInterval,    // FFmpeg select filter for Nth frames
    Parallel,          // Rayon parallel extraction
}
```

### `SaveMode`

```rust
pub enum SaveMode {
    SingleDirectory,    // All frames in one folder
    MultipleDirectory,  // Subfolder per video (default)
}
```

### `FrameExtractor`

Builder-pattern struct for configuring extraction.

| Method | Description |
|--------|-------------|
| `new(video_path, output_dir)` | Create extractor |
| `with_interval(n)` | Set frame interval (default: 1) |
| `with_mode(mode)` | Set extraction mode |
| `with_save_mode(mode)` | Set save directory mode |
| `extract()` | Execute extraction |

**Getters:** `video_path()`, `output_dir()`, `frame_interval()`, `extraction_mode()`, `save_mode()`

---

## Wrapper Functions

| Function | Equivalent FrameExtractor |
|----------|---------------------------|
| `extract_all_frames_sequential()` | OpenCVSequential |
| `extract_all_frames_ffmpeg()` | FFmpeg |
| `extract_frames_ffmpeg_interval()` | FFmpegInterval + interval |
| `extract_frames_opencv_interval()` | OpenCVInterval + interval |
| `extract_all_frames_rayon()` | Parallel (interval=1) |
| `extract_frames()` | Generic with string params |

---

## Feature Overview

| Component | Feature | Defined In |
|-----------|---------|------------|
| **`FrameExtractor`** | Main extraction engine with builder pattern | `frame_extraction.rs` |
| **`ExtractionMode`** | 5 modes (OpenCV, FFmpeg, Parallel) | `frame_extraction.rs` |
| **`SaveMode`** | Single vs Multiple directory output | `frame_extraction.rs` |
| **Wrappers** | Convenience functions for common patterns | `wrappers.rs` |
