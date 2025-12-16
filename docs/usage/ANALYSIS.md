# Analysis Module Documentation

## Usage: Motion Detection

### Direct Usage (Default Algorithm)

```bash
./media_core analysis motion <video_file> <output_dir>
```

Analyzes a video file for motion using **Frame Difference** (default). Returns motion segments as `(start_frame, end_frame)` pairs.

**Example:**
```bash
./media_core analysis motion video.mp4 motion_output/
```

> **Note:** To use algorithms other than `framediff`, you must use the **Configuration File** method.

### Using Config File (Advanced)

```bash
./media_core analysis --config analysis_config.json
```

Allows selecting different algorithms (`mog2`, `knn`, `opticalflow`) and tuning parameters like threshold and minimum area.

**Config Example for Motion:**

```json
{
  "mode": "motion",
  "input_path": "video.mp4",
  "output_dir": "output/",
  "motion": {
    "algorithm": "mog2",
    "threshold": 25.0,
    "min_area": 500,
    "frame_skip": 1,
    "save_motion_clips": true
  }
}
```

### Supported Algorithms

| Algorithm | Description |
|-----------|-------------|
| `framediff` | Frame-to-frame difference |
| `mog2` | MOG2 background subtraction |
| `knn` | KNN background subtraction |
| `opticalflow` | Farneback optical flow |

### Configuration Reference

- `algorithm`: One of `"framediff"`, `"mog2"`, `"knn"`, `"opticalflow"`.
- `threshold`: Sensitivity (default: 25.0). Lower is more sensitive.
- `min_area`: Minimum pixel area to consider as motion (default: 500). Filters noise.
- `frame_skip`: Frames to skip between checks (default: 1). Higher = faster but less precise.
- `save_motion_clips`: Boolean (default: true). Whether to save clips (currently placeholder).

---

## Usage: Image Similarity

### Direct Usage

```bash
./media_core analysis similarity <image_dir> <output_dir>
```

Compares all images in a directory and groups similar ones together using **Histogram comparison** (default).

> **Note:** To use methods other than `histogram` (e.g., `featurematching`), you must use the **Configuration File** method.

**Example:**
```bash
./media_core analysis similarity ./frames/ ./grouped/
```

**Output:**
- `grouped/group_000/`
- `grouped/group_001/`
- ...

### Using Config File (Advanced)

```bash
./media_core analysis --config analysis_config.json
```

Allows selecting different methods (`featurematching`, `perceptualhash`) and tuning thresholds.

**Config Example for Similarity:**

```json
{
  "mode": "similarity",
  "input_path": "frames/",
  "output_dir": "grouped/",
  "similarity": {
    "method": "featurematching",
    "process_mode": "parallel",
    "parallel_num": 4,
    "threshold": 0.9,
    "resize_width": 256,
    "resize_height": 256,
    "group_similar": true,
    "min_category_size": 2,
    "histogram": {
      "bins": 64,
      "similarity_threshold": 0.8
    },
    "feature_matching": {
      "max_features": 500,
      "good_match_percent": 0.15,
      "similarity_threshold": 0.2
    },
    "perceptual_hash": {
      "hash_size": 8,
      "similarity_threshold": 0.9
    }
  }
}
```

### Supported Methods

| Method | Description |
|--------|-------------|
| `histogram` | 2D HSV color histogram correlation |
| `featurematching` | ORB keypoint matching with distance filtering |
| `perceptualhash` | Average hash (aHash) with configurable size |

---

## Configuration Reference (Similarity)

### Global Options

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `method` | string | `histogram` | Similarity method: `histogram`, `featurematching`, `perceptualhash` |
| `process_mode` | string | `single` | Processing mode: `single` or `parallel` |
| `parallel_num` | int | CPU count | Number of parallel workers |
| `threshold` | float | 0.9 | Global similarity threshold (0.0-1.0) |
| `resize_width` | int | 256 | Width to resize images before comparison |
| `resize_height` | int | 256 | Height to resize images before comparison |
| `group_similar` | bool | true | Whether to copy files into group directories |
| `min_category_size` | int | 1 | Minimum images per category (smaller groups discarded) |

### Histogram Options

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `bins` | int | 64 | Number of histogram bins per channel |
| `similarity_threshold` | float | 0.8 | Method-specific threshold |

### Feature Matching Options

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `max_features` | int | 500 | Maximum ORB features to detect |
| `good_match_percent` | float | 0.15 | Keep top N% of matches by distance |
| `similarity_threshold` | float | 0.2 | Method-specific threshold |

### Perceptual Hash Options

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `hash_size` | int | 8 | Hash grid size (8 = 64-bit hash) |
| `similarity_threshold` | float | 0.9 | Method-specific threshold |

---
