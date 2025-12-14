# Analysis Module Documentation

> **Last Updated:** 2025-12-15

---

## Overview

The `analysis` module provides motion detection and image similarity analysis.

| Submodule | Purpose |
|-----------|---------|
| `motion/` | Detect motion in video files |
| `similarity/` | Group similar images |

---

## Files

```
src/analysis/
├── mod.rs           # Module exports
├── types.rs         # AnalysisError enum
├── config.rs        # AnalysisConfig, MotionConfig, SimilarityConfig
├── motion/
│   ├── mod.rs
│   ├── detector.rs      # MotionDetector
│   ├── frame_diff.rs    # Frame differencing algorithm
│   ├── mog2.rs          # MOG2 background subtraction
│   ├── knn.rs           # KNN background subtraction
│   └── optical_flow.rs  # Farneback optical flow
└── similarity/
    ├── mod.rs
    ├── analyzer.rs          # SimilarityAnalyzer
    ├── histogram.rs         # Mean-based comparison
    ├── feature_matching.rs  # ORB feature matching
    └── perceptual_hash.rs   # Average hash algorithm
```

---

## Usage

### Motion Detection

```bash
./media_core analysis motion <video_file> <output_dir>
```

Analyzes a video file for motion using frame differencing (default). Returns motion segments as `(start_frame, end_frame)` pairs.

**Example:**
```bash
./media_core analysis motion video.mp4 motion_output/
```

### Image Similarity

```bash
./media_core analysis similarity <image_dir> <output_dir>
```

Compares all images in a directory and groups similar ones together.

**Example:**
```bash
./media_core analysis similarity ./frames/ ./grouped/
```

Output:
- `group_000/` - First group of similar images
- `group_001/` - Second group, etc.

### Using Config File

```bash
./media_core analysis --config analysis_config.json
```

---

## Motion Detection Algorithms

| Algorithm | Description | When to Use |
|-----------|-------------|-------------|
| `framediff` | Frame-to-frame difference | Static camera, indoor |
| `mog2` | MOG2 background subtraction | Dynamic background, outdoor |
| `knn` | KNN background subtraction | Complex scenes, shadows |
| `opticalflow` | Farneback optical flow | Moving camera, velocity estimation |

---

## Image Similarity Methods

| Method | Description | Best For |
|--------|-------------|----------|
| `histogram` | Mean intensity comparison | Fast, global similarity |
| `featurematching` | ORB keypoint matching | Object recognition |
| `perceptualhash` | Average hash (aHash) | Duplicate detection |

---

## Config Options

```json
{
  "mode": "motion",
  "input_path": "video.mp4",
  "output_dir": "output/",
  "motion": {
    "algorithm": "framediff",
    "threshold": 25.0,
    "min_area": 500,
    "frame_skip": 1,
    "save_motion_clips": true
  },
  "similarity": {
    "method": "histogram",
    "threshold": 0.9,
    "resize_width": 256,
    "resize_height": 256,
    "group_similar": true
  }
}
```

---

## Dependencies

- **OpenCV** (already in Cargo.toml)

---

## Known Limitations

1. Motion detection returns segment ranges but does not export video clips (planned feature).
