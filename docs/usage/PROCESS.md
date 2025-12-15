# Process Module Documentation

The `process` module provides a powerful engine for batch media processing, frame extraction, and video creation.

---

## Commands

### 1. Direct Frame Extraction (Quick Start)

Extract all frames from a video or directory of videos without creating a new video file. This requires NO configuration file.

```bash
cargo run process extract <input_path> <output_dir>
```

**Example:**
```bash
cargo run process extract ./data/my_video.mp4 ./frames_output
```

**Output:**
-   `frames_output/extract_my_video_frames/video000_frame000001.jpg`
-   ...

**Key Features:**
-   **Direct Extraction**: Skips temporary files and video re-encoding.
-   **Persistent Output**: Frames are saved and not deleted.
-   **Flexible Input**: Works with single `.mp4` files or entire directories.

---

### 2. Generate Configuration

For advanced usage (filtering, resizing, custom hardware alignment), you need a configuration file. Generate a default template:

```bash
cargo run config process
```

This creates a `process_config.json` file in your current directory with default settings.

---

### 3. Standard Processing (Advanced)

Run the processor using your generated or customized configuration file.

```bash
cargo run process <config_file_path>
```

**Example:**
```bash
cargo run process ./process_config.json
```

---

## Configuration Reference

The `process_config.json` file controls all aspects of the processing pipeline.

```json
{
  "input_path": "./data",
  "output_path": "./output",
  "processing_mode": "parallel",
  "processing_options": {
      "backup_original": false
  },
  "video_config": {
    "input_directories": ["./data", "./other_videos"],
    "output_directory": "./output",
    "output_prefix": "processed",
    "output_fps": 30,
    "frame_interval": 1,
    "extraction_mode": "opencv",
    "video_creation_mode": "temp_frames",
    "hardware_acceleration": {
      "enabled": true,
      "backend": "videotoolbox",
      "device_id": 0
    }
  }
}
```

### Configuration Options

*   **`input_path`**: Main input directory path.
*   **`output_path`**: Main output directory path.
*   **`processing_mode`**: `"parallel"` (fastest) or `"sequential"` (low memory).
*   **`video_config`**: Settings for video processing:
    *   **`input_directories`**: List of folders OR file paths to process.
    *   **`output_directory`**: Destination for final files.
    *   **`output_prefix`**: Naming prefix for generated files.
    *   **`output_fps`**: Framerate for created videos (default 30).
    *   **`frame_interval`**: Process every Nth frame (1 = all frames, 10 = every 10th).
    *   **`extraction_mode`**: `"opencv"` (faster) or `"ffmpeg"` (more compatible).
    *   **`video_creation_mode`**:
        *   `"direct"`: Creates video on-the-fly (Low memory usage).
        *   `"temp_frames"`: Extracts all frames first, then encodes (Standard).
        *   `"skip"`: Extracts frames only, creates no video.
    *   **`hardware_acceleration`**:
        *   `enabled`: Set to `true` to use GPU.
        *   `backend`: `"videotoolbox"` (Mac), `"cuda"` (Nvidia), or `"any"`.
