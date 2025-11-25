# TODO Roadmap and Restructuring Plan

## Workspace Structure (current and targets)
- `Cargo.toml` (workspace; members include `crates/media_core`)
- `crates/`
  - `media_core/` (library crate; package `media-core`; modules: `rtsp`, `process`)
  - `media_rtsp/` (future crate: RTSP helpers and adapters; package `media-rtsp`)
  - `media_process/` (future crate: offline processing pipeline; package `media-process`)
  - `media_hls/` (future crate: HLS playlist and segment management; package `media-hls`)
  - `media_metrics/` (future crate: metrics and health endpoints; package `media-metrics`)
- `src/` (binary crate; CLI that uses the library)
- `examples/` (root examples showing library usage)
- `docker/` (container build files)
- `docs/` (project documentation)

## Feature To-Do
- Reliability: graceful shutdown; exponential backoff; FFmpeg stderr diagnostics; disk quota/retention
- Observability: structured logging; Prometheus metrics; per-stream health endpoint
- Configuration: CLI `--config`; schema validation; env interpolation for secrets
- RTSP: audio toggle; camera IDs; optional HLS output; segment tuning
- Processing: HW-accelerated decoding; codec consistency; robust extraction; sidecar indexes/thumbnails
- Performance: thread pool sizing; I/O efficiency (faststart, reduced logging)
- Testing (future): RTSP integration tests; golden assets; config validation
- Security: credential masking; optional encrypted configs/tokens
- Developer experience: `clap` CLI; preflight checks for FFmpeg/OpenCV

