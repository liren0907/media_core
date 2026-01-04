# Annotation Module

Text overlay and watermark utilities for images and videos.

**Location:** `src/annotation/`

## Module Structure

```
annotation/
├── mod.rs        # Exports
├── annotator.rs  # FrameAnnotator struct (main processor)
├── overlay.rs    # Low-level text overlay functions
└── types.rs      # AnnotationConfig, TextPosition, AnnotationType
```

## Quick Start

### Single Image Annotation

```rust
use media_core::annotation::{
    AnnotationConfig, AnnotationType, DataSource, FrameAnnotator, TextPosition,
};

let config = AnnotationConfig {
    input: DataSource::Image("frame.jpg".to_string()),
    output_path: "annotated_frame.jpg".to_string(),
    text_position: TextPosition::TopLeft,
    annotation_type: AnnotationType::Filename,
    ..Default::default()
};

FrameAnnotator::new(config).process()?;
```

### Video from Frames

```rust
use media_core::annotation::{
    AnnotationConfig, AnnotationType, DataSource, FrameAnnotator, 
    TextPosition, VideoOutputConfig,
};

let config = AnnotationConfig {
    input: DataSource::FrameDir("./frames".to_string()),
    output_path: "output_video.mp4".to_string(),
    text_position: TextPosition::BottomLeft,
    annotation_type: AnnotationType::Timestamp,
    source_fps: Some(30.0),
    video_encoding: Some(VideoOutputConfig {
        fps: 30,
        filename: "".to_string(),
    }),
};

FrameAnnotator::new(config).process()?;
```

---

## Types

### `AnnotationConfig`

Main configuration struct.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `input` | `DataSource` | `FrameDir("./output")` | Input source |
| `output_path` | `String` | `"output.mp4"` | Output file path |
| `text_position` | `TextPosition` | `TopLeft` | Text overlay position |
| `annotation_type` | `AnnotationType` | `Filename` | What text to overlay |
| `source_fps` | `Option<f64>` | `Some(30.0)` | Source video FPS |
| `video_encoding` | `Option<VideoOutputConfig>` | `Some(...)` | Video output settings |

### `DataSource`

```rust
pub enum DataSource {
    Image(String),     // Single image file path
    FrameDir(String),  // Directory containing .jpg frames
}
```

### `AnnotationType`

```rust
pub enum AnnotationType {
    Filename,           // Use source filename as text
    Timestamp,          // Generate HH:MM:SS.mmm from frame index
    Custom(String),     // Custom watermark text
}
```

### `TextPosition`

```rust
pub enum TextPosition {
    TopLeft,      // Default
    TopRight,
    BottomLeft,
    BottomRight,
}
```

### `VideoOutputConfig`

```rust
pub struct VideoOutputConfig {
    pub fps: i32,         // Output video FPS (default: 30)
    pub filename: String, // Not used (kept for compatibility)
}
```

---

## Feature Overview

| Component | Feature | Defined In |
|-----------|---------|------------|
| **`FrameAnnotator`** | Main processor for annotation workflow | `annotator.rs` |
| **`AnnotationConfig`** | Configuration for input/output/style | `types.rs` |
| **`DataSource`** | Image vs FrameDir input selection | `types.rs` |
| **`AnnotationType`** | Filename, Timestamp, or Custom text | `types.rs` |
| **`TextPosition`** | Corner placement for overlay | `types.rs` |
