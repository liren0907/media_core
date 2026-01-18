# Unified Pipeline Usage Guide

This guide describes how to use the **Unified Media Pipeline** to process media through a modular, context-aware workflow.

## 1. Core Concepts

The pipeline operates on a **Blackboard Pattern**:
1.  **Pipeline**: A sequence of processing nodes.
2.  **Context (`MediaContext`)**: A shared state passed between nodes. It holds the input source, metadata, analysis results, and extracted frames.
3.  **Nodes**: Modular steps (e.g., `ExtractMetadata`, `ExtractFrames`) that read from and write to the Context.

### Basic Structure

```rust
use media_core::pipeline::{Pipeline, MediaContext};

// 1. Build the Pipeline
let pipeline = Pipeline::new()
    .add_node(StepOne::new())
    .add_node(StepTwo::new());

// 2. Initialize Context
let context = MediaContext::from_file(Path::new("video.mp4").to_path_buf());

// 3. Execute
let result_context = pipeline.execute(context)?;
```

---

## 2. Streaming Pipieline

The Streaming module allows you to extract frames from a video using various sampling strategies.

### Node: `ExtractFrames`

**Import:**
```rust
use media_core::streaming::pipeline::ExtractFrames;
use media_core::streaming::{SamplingStrategy, ExtractionMode};
```

**Configuration:**
- **Strategy**: Define which frames to pick (`EveryNth`, `FirstN`, etc.).
- **Scale**: Resize frames (e.g., `0.5` for 50% size).
- **Mode**: `Random` (default) or `Sequential` (optimized for dense sampling).

### Examples

#### Basic Extraction (Every 30th frame)
```rust
.add_node(ExtractFrames::new(SamplingStrategy::EveryNth(30)))
```

#### First N Frames with Resizing
Extract the first 10 frames and resize them to 50% (half width/height).
```rust
.add_node(
    ExtractFrames::new(SamplingStrategy::FirstN(10))
        .with_scale(0.5)
)
```

#### Dense Sequential Extraction
Efficiently read a range of frames (e.g., 0 to 100) using sequential reading mode.
```rust
.add_node(
    ExtractFrames::new(SamplingStrategy::Range(0, 100))
        .with_mode(ExtractionMode::Sequential)
)
```

#### Single Frame Extraction
Extract a specific frame by index (e.g., frame 100).
```rust
.add_node(ExtractFrames::new(SamplingStrategy::Custom(vec![100])))
```

**Accessing Results:**
```rust
let frames = context.extracted_frames; // Vec<FrameData>
println!("Extracted {} frames", frames.len());
```

---

## 3. Analysis Pipeline

The Analysis module enables the pipeline to "understand" the media content, such as extracting metadata or detecting motion.

### Node: `ExtractMetadata`

Extracts resolution, FPS, duration, and codec information.

**Import:**
```rust
use media_core::metadata::pipeline::ExtractMetadata;
```

**Usage:**
```rust
// include_thumbnail: false
.add_node(ExtractMetadata::new(false))
```

**Accessing Results:**
```rust
if let Some(meta) = context.metadata {
    println!("Resolution: {}", meta.resolution);
    println!("FPS: {:?}", meta.fps);
}
```

### Node: `DetectMotion`

Detects motion segments in the video.

**Import:**
```rust
use media_core::analysis::pipeline::DetectMotion;
```

**Configuration:**
- `.threshold(f64)`: Sensitivity (default: 25.0).
- `.min_area(i32)`: Minimum motion area to trigger event (default: 500).
- `.output_dir(path)`: Directory for debug output.

**Usage:**
```rust
.add_node(
    DetectMotion::default()
        .threshold(25.0)
        .min_area(500)
        .output_dir("output/motion_debug")
)
```

**Accessing Results:**
```rust
if let Some(report) = context.analysis {
    for event in report.motion_events {
        println!("Motion detected: Frames {}-{}", event.start_frame, event.end_frame);
    }
}
```

---

## 4. Full Example: Analysis & Streaming

Combine modules to first analyze the video and then extract key frames (conceptually).

```rust
let pipeline = Pipeline::new()
    // 1. Get Metadata
    .add_node(ExtractMetadata::new(false))
    
    // 2. Detect Motion (Populates context.analysis)
    .add_node(DetectMotion::default())
    
    // 3. Extract some frames for preview
    .add_node(ExtractFrames::new(SamplingStrategy::EveryNth(30)));

let result = pipeline.execute(context)?;
```
