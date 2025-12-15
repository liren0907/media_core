# Changelog

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

