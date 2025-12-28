# Streaming Module

Frame extraction and streaming utilities for video files, centered around the unified `StreamExtractor`.

**Location:** `src/streaming/`

## Module Structure

```
streaming/
├── mod.rs        # Exports
├── extractor.rs  # StreamExtractor struct (The core engine)
├── strategy.rs   # enum SamplingStrategy
├── types.rs      # struct FrameData, struct StreamProgress
└── helpers.rs    # Low-level utilities (OpenCV integration)
```

## Functions & Structs

### `StreamExtractor`

The primary tool for extracting frames. It supports both **Seek** (sparse sampling) and **Sequential** (continuous reading) modes.

```rust
// 1. Initialize (Optional strategy, defaults to All Frames)
let mut extractor = StreamExtractor::new("video.mp4", None)?;

// 2. Configure (Optional)
extractor.set_mode(ExtractionMode::Seek); // or Sequential
extractor.set_strategy(SamplingStrategy::EveryNth(10));

// 3. Extract
let frames: Vec<FrameData> = extractor.extract(Some(0.5))?; // 0.5 scale factor
```

**Key Methods:**
*   `new(path, option<strategy>)`: Create a new stateful extractor.
*   `set_strategy(strategy)`: Change what frames to extract.
*   `set_mode(mode)`: Switch between `Seek` (jump) and `Sequential` (scan).
*   `extract(scale_factor)`: Perform the extraction.

---

### `SamplingStrategy`

Defines **which** frames to extract.

```rust
pub enum SamplingStrategy {
    EveryNth(usize),      // 1, 11, 21...
    FirstN(usize),        // 0, 1, 2... N
    Range(usize, usize),  // Start..End
    KeyFrames,            // Keyframes typically every 30
    Custom(Vec<usize>),   // Explicit list [0, 100, 500]
}
```

---

### `ExtractionMode`

Defines **how** to read frames.

*   `Seek` (Default): Jumps to specific frames. Best for sparse sampling (e.g., EveryNth(100)).
*   `Sequential`: Reads frames one by one. Best for continuous blocks (e.g., Range(0, 100)).

---

### Low-Level Helpers

#### `get_stream_info`
Quickly get video metadata without extracting frames.
```rust
let info = get_stream_info("video.mp4")?;
// info.total, info.message, etc.
```

#### `get_video_capture`
Get raw OpenCV `VideoCapture` object.
```rust
let cap = get_video_capture("video.mp4")?;
```

#### `mat_to_base64_jpeg`
Convert OpenCV Mat to Base64 string.
```rust
let b64 = mat_to_base64_jpeg(&frame)?;
```

---

## Types

### `FrameData`
```rust
pub struct FrameData {
    pub index: usize,
    pub data: String,  // Base64 encoded JPEG
}
```

## Feature Overview

| Component | Feature | Defined In |
| :--- | :--- | :--- |
| **`StreamExtractor`** | The unified engine for all extraction needs | `extractor.rs` |
| **`SamplingStrategy`** | Defines filtered subsets of frames | `strategy.rs` |
| **`ExtractionMode`** | Optimizes read performance (Seek vs Sequential) | `extractor.rs` |
| **`get_stream_info`** | Fast metadata extraction | `helpers.rs` |
