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
    "threshold": 0.9,
    "resize_width": 256,
    "resize_height": 256,
    "group_similar": true
  }
}
```

### Supported Methods

| Method | Description |
|--------|-------------|
| `histogram` | Mean intensity comparison |
| `featurematching` | ORB keypoint matching |
| `perceptualhash` | Average hash (aHash) |

### Configuration Reference

- `method`: One of `"histogram"`, `"featurematching"`, `"perceptualhash"`.
- `threshold`: Similarity range (0.0 - 1.0).
- `resize_width`/`height`: Internal size for comparison (default 256x256).
- `group_similar`: `Boolean` (default: true). Whether to physically move/copy files into group folders.


