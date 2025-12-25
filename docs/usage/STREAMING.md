# Streaming Module

Frame extraction and streaming utilities for video files.

**Location:** `src/video_processing/streaming/`

## Module Structure

```
streaming/
├── mod.rs        # Exports
├── types.rs      # FrameData, SamplingStrategy, StreamProgress, StreamResult
├── extractor.rs  # extract_frame, extract_frames_interval
├── helpers.rs    # get_stream_info, get_video_capture
└── sampler.rs    # stream_frames, stream_frames_sampled
```

## Functions

### `stream_frames`

Extract frames from video with basic options.

```rust
pub fn stream_frames(
    video_path: &str,
    skip: usize,
    max: usize,
    scale_factor: Option<f64>,
) -> StreamResult<Vec<FrameData>>
```

| Parameter | Type | Description |
|-----------|------|-------------|
| `video_path` | `&str` | Path to video file |
| `skip` | `usize` | Number of frames to skip from start |
| `max` | `usize` | Maximum frames to extract |
| `scale_factor` | `Option<f64>` | Resize factor (e.g., 0.5 = half size) |

**Example:**
```rust
use media_core::video_processing::{stream_frames, FrameData};

let frames = stream_frames("video.mp4", 30, 10, Some(0.5))?;
for frame in frames {
    println!("Frame {}: {} bytes", frame.index, frame.data.len());
}
```

---

### `stream_frames_sampled`

Extract frames using smart sampling strategies.

```rust
pub fn stream_frames_sampled(
    video_path: &str,
    sampling_strategy: SamplingStrategy,
) -> StreamResult<Vec<FrameData>>
```

**Sampling Strategies:**

| Strategy | Description |
|----------|-------------|
| `EveryNth(n)` | Every Nth frame |
| `FirstN(n)` | First N frames only |
| `Range(start, end)` | Frame range [start, end) |
| `KeyFrames` | Every 30th frame |
| `Custom(Vec<usize>)` | Specific frame indices |

**Example:**
```rust
use media_core::video_processing::{stream_frames_sampled, SamplingStrategy};

// Every 10th frame
let frames = stream_frames_sampled("video.mp4", SamplingStrategy::EveryNth(10))?;

// Frames 100-200
let frames = stream_frames_sampled("video.mp4", SamplingStrategy::Range(100, 200))?;

// Specific frames
let frames = stream_frames_sampled("video.mp4", SamplingStrategy::Custom(vec![0, 50, 100, 150]))?;
```

---

### `get_stream_info`

Get video information before streaming.

```rust
pub fn get_stream_info(video_path: &str) -> StreamResult<StreamProgress>
```

**Example:**
```rust
use media_core::video_processing::get_stream_info;

let info = get_stream_info("video.mp4")?;
println!("{}", info.message);
// Output: "Video: 1920x1080 @ 30.00 FPS, 1800 frames"
```

---

### `extract_frame`

Extract a single frame at specific index.

```rust
pub fn extract_frame(video_path: &str, frame_index: usize) -> StreamResult<FrameData>
```

**Example:**
```rust
use media_core::video_processing::extract_frame;

let frame = extract_frame("video.mp4", 100)?;
println!("Frame index: {}", frame.index);
```

---

### `extract_frames_interval`

Extract frames at regular intervals.

```rust
pub fn extract_frames_interval(
    video_path: &str,
    interval: usize,
    max_frames: Option<usize>,
) -> StreamResult<Vec<FrameData>>
```

**Example:**
```rust
use media_core::video_processing::extract_frames_interval;

let frames = extract_frames_interval("video.mp4", 30, Some(10))?;
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

### `StreamProgress`
```rust
pub struct StreamProgress {
    pub current: usize,
    pub total: usize,
    pub message: String,
}
```

### `SamplingStrategy`
```rust
pub enum SamplingStrategy {
    EveryNth(usize),
    FirstN(usize),
    Range(usize, usize),
    KeyFrames,
    Custom(Vec<usize>),
}
```

### `StreamResult<T>`
```rust
pub type StreamResult<T> = Result<T, String>;
```

## Feature Overview

| Function | Feature | Defined In |
| :--- | :--- | :--- |
| **`get_stream_info`** | Fast metadata extraction (dims, fps, count) | `helpers.rs` |
| **`extract_frame`** | Single frame extraction by index | `extractor.rs` |
| **`extract_frames_interval`** | Regular interval extraction | `extractor.rs` |
| **`stream_frames`** | Sequential streaming | `sampler.rs` |
| **`stream_frames_sampled`** | Complex sampling strategies | `sampler.rs` |
