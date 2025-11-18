# TODO Roadmap and Restructuring Plan

## Immediate Priority: Library-Based Workspace
- Rename the project to `media-core` as the primary library crate.
- Introduce a top-level `crates/` directory and move current core code into `crates/media_core/`.
- Keep the root package as a consumer/example: its `src/` demonstrates how to import and use the `media-core` library.
- Plan for multiple future crates in `crates/` for video, image, and stream pre/post-processing.
- Maintain backward compatibility for existing imports (e.g., `use media_core::...`).

## Proposed Directory Structure (target)
- `Cargo.toml` (workspace)
- `crates/`
  - `media_core/` (library crate; current core modules)
  - `media_rtsp/` (future crate: RTSP helpers and adapters)
  - `media_process/` (future crate: offline processing pipeline)
  - `media_hls/` (future crate: HLS playlist and segment management)
  - `media_metrics/` (future crate: metrics and health endpoints)
- `rtsp_stream_extractor/` (binary crate; CLI that uses the library)
- `examples/` (root examples showing library usage)
- `docker/` (container build files)

## Migration Plan (no code changes yet)
- Move `media_core/` to `crates/media_core/` and set `package.name = "media-core"`.
- Update workspace members in root `Cargo.toml` to include `crates/media_core` and the binary package.
- Adjust imports in the binary to reference the new crate path.
- Preserve config file semantics (`config.json`, `config_process.json`).
- Add top-level documentation explaining crate boundaries and usage examples.

## Feature Roadmap

### Reliability
- Graceful shutdown for FFmpeg loop (clean exit, resource release).
- Exponential backoff with jitter for restarts; cap max consecutive failures.
- Parse FFmpeg stderr for actionable diagnostics (auth, network, codec).
- Disk quota and retention: auto-delete oldest segments per camera; low-space safeguard.

### Observability
- Structured logging with configurable levels; redact credentials in logs.
- Metrics export (Prometheus): segments written, bytes, errors, restarts, processing time.
- Lightweight health endpoint for per-stream status and disk usage.

### Configuration
- CLI flags for `--config` and validation prior to run.
- Schema validation and strong defaults for `CaptureConfig` and `VideoExtractionConfig`.
- Environment variable interpolation for secrets; optional `.env` support.

### RTSP Recording Features
- Honor `audio` field: toggle `-an` off when audio is enabled.
- Camera-friendly IDs and directory names (not sanitized URLs).
- Optional HLS output: `-f hls` segments with playlist management; simple preview server.
- Segment rotation tuning: overlap/keyframe alignment; configurable `segment_time_delta`.

### Video Processing Pipeline
- Hardware-accelerated decoding selection with clear logs and safe fallback.
- Codec consistency across outputs; configurable output codec per platform.
- Robust frame extraction with OpenCV/FFmpeg fallback and auto-resize to common frame size.
- Sidecar JSON indexes, thumbnails/contact sheets for quick review.

### Performance
- Global thread pool sizing and admission control for multi-camera scaling.
- I/O efficiency: faststart flags, reduced per-frame logging.

### Testing
- Integration tests with an RTSP server to validate reconnection and segmentation.
- Golden assets for OpenCV direct and FFmpeg concat paths; config validation tests.

### Security
- Credential masking in logs and error messages.
- Optional encrypted configs or token-based access for production.

### Developer Experience
- `clap`-based CLI: `--mode`, `--config`, `--dry-run`, `--preview`.
- Preflight checks for FFmpeg/OpenCV backends; actionable error hints.

## Phased Implementation
- Phase 1: Workspace restructuring and crate renaming (`media-core`), keep root as consumer/example.
- Phase 2: Logging redaction, graceful shutdown, improved backoff.
- Phase 3: Metrics and health endpoint.
- Phase 4: HLS export option and audio toggle.
- Phase 5: Pipeline codec alignment and robust extraction fallbacks.
- Phase 6: Test harness and CI integration.

## Non-Goals (initial)
- No immediate changes to public APIs beyond crate path adjustments.
- No forced audio inclusion by default; audio remains opt-in.

## Notes
- All restructuring will be done incrementally to keep the binary runnable throughout.
- Backward compatibility will be maintained where practical to ease adoption.