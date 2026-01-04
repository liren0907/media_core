# Changelog

## [0.3.1] - 2026-01-04

### Added
- **CLI Enhancements**: Integrated standalone tools into the main CLI application:
  - `annotation`: Image and video annotation command.
  - `benchmark`: Performance benchmarking command.
  - `streaming`: Frame extraction stream command.
  - `video-process`: Granular video processing command with multiple extraction modes.

### Changed
- **CLI**: Expanded command-line interface to support granular video processing options (`--mode`, `--interval`, `--save-mode`).
- **Annotation**: Added `Center` text alignment support.
- **Refactor**: Improved type safety and error handling across CLI modules.

## [0.3.0] - 2026-01-04

### Added
- **Annotation Module**: Text overlay and watermark for images/videos with `FrameAnnotator`.
- **Benchmark Module**: Performance timing utilities with `Benchmark` and `BenchmarkSuite`.
- **RTSP Sync Module**: Synchronized multi-stream capture with latency monitoring.
- **Video Process Module**: Frame extraction with `FrameExtractor` (5 modes, parallel support).
- **Unit Tests**: Added tests for annotation, benchmark, streaming, and video_process modules.
- **Documentation**: Usage guides for annotation, benchmark, and video_process modules.

### Changed
- **Streaming Module**: Optimized `StreamExtractor` with Seek/Sequential modes.
- **Metadata Module**: Added FFprobe-based codec analysis.

---

## [0.2.0] - 2025-12-15

### Added
- **Analysis Module**: Introduced comprehensive media analysis capabilities.
- **HLS Module**: Added support for HTTP Live Streaming (HLS) generation and handling.

### Changed
- **CLI Module**: Refactored the command-line interface module for better structure and maintainability.
- **Process Module**: Modified `process` module to improve code organization (Workers & Handlers) and performance.


## [0.1.0] - 2025-12-07
### Added
- Initial release of `media_core` with RTSP capture support:
  - FFmpeg-based segmented recording (default).
  - OpenCV-based capture with optional custom FPS and preview.
  - Optional HLS output (playlist + TS segments).
- Configuration generation helpers for RTSP and process modes.
- Video processing module with extraction and basic copy pipelines.
- Example app (`media_core_app`) demonstrating RTSP capture.
- Integration and config generation tests.

