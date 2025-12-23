# Metadata Module

## Usage

### `get_media_info`

The core function for fetching metadata as a Rust struct.

```rust
pub fn get_media_info(media_path: &str, include_thumbnail: bool) -> Result<MediaMetadata, String>
```

**Parameters:**
- `media_path`: Absolute path to the media file.
- `include_thumbnail`: Boolean flag to generate a thumbnail.

**Returns:**
- `Result<MediaMetadata, String>`: Struct containing all extracted metadata or an error message.

### `get_media_info_json`

A wrapper that returns the metadata as a pretty-printed JSON string.

```rust
pub fn get_media_info_json(media_path: &str, include_thumbnail: bool) -> Result<String, String>
```

## Data Structures

### `MediaMetadata`

The main struct holding the extracted information.

| Field | Type | Description |
|-------|------|-------------|
| `file_path` | `String` | Absolute path to the file. |
| `file_size_bytes` | `u64` | File size in bytes. |
| `resolution` | `String` | Resolution string (e.g., "1920x1080"). |
| `codec_name` | `Option<String>` | Name of the video codec (e.g., "h264"). |
| `duration_seconds` | `Option<f64>` | Duration in seconds. |
| `quality_category` | `String` | Estimated quality (e.g., "1080p Full HD"). |

## Usage Examples

### Basic Usage

```rust
use media_core::metadata::orchestrator::get_media_info;

fn main() {
    let path = "/path/to/video.mp4";
    match get_media_info(path, true) {
        Ok(info) => println!("Resolution: {}", info.resolution),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### JSON Output

```rust
use media_core::metadata::orchestrator::get_media_info_json;

fn main() {
    let json = get_media_info_json("/path/to/image.jpg", false).unwrap();
    println!("{}", json);
}
```
