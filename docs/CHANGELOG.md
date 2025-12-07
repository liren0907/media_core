# Changelog

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

