# HLS VOD Conversion

Convert video files (MP4, MOV, AVI, MKV) to HLS format for Video On Demand streaming.

---

## Commands

### Direct Conversion

```bash
./media_core hls <input_video> <output_dir>
```

Converts a video file to HLS format using default settings:
- 5-second segments
- Baseline H.264 profile
- Level 3.0

**Output:**
- `playlist.m3u8` - Master playlist
- `playlist0.ts`, `playlist1.ts`, ... - Video segments

---

### Generate Default Config

```bash
./media_core config hls
```

Creates a template `hls_config.json` with default values:

```json
{
  "input_path": "",
  "output_dir": "hls_output",
  "segment_duration": 5,
  "playlist_filename": "playlist.m3u8",
  "force_keyframes": true,
  "profile": "baseline",
  "level": "3.0"
}
```

---

### Using Config File

```bash
./media_core hls --config hls_config.json
```

Loads all settings from a JSON config file for custom segment duration, H.264 profile/level, or batch processing.

---

## Config Options

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `input_path` | string | - | Input video file path |
| `output_dir` | string | `hls_output` | Output directory |
| `segment_duration` | u32 | `5` | Segment duration in seconds |
| `playlist_filename` | string | `playlist.m3u8` | Playlist file name |
| `force_keyframes` | bool | `true` | Force keyframes at segment boundaries |
| `profile` | string | `baseline` | H.264 profile (baseline/main/high) |
| `level` | string | `3.0` | H.264 level |

---

## Verify Output

```bash
ls ./hls_output/                      # Check generated files
ffplay ./hls_output/playlist.m3u8     # Play with ffplay
```

---

