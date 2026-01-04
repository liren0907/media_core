use crate::video_process::frame_extraction::{ExtractionMode, FrameExtractor, SaveMode};

pub fn extract_frames(
    video_path: &str,
    output_dir: &str,
    frame_interval: usize,
    extraction_mode: &str,
    save_mode: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mode = match extraction_mode {
        "parallel" => ExtractionMode::Parallel,
        "ffmpeg" | "ffmpeg_interval" => ExtractionMode::FFmpegInterval,
        _ => ExtractionMode::OpenCVInterval,
    };

    let save = match save_mode {
        "single_directory" => SaveMode::SingleDirectory,
        _ => SaveMode::MultipleDirectory,
    };

    FrameExtractor::new(video_path, output_dir)
        .with_interval(frame_interval)
        .with_mode(mode)
        .with_save_mode(save)
        .extract()
}

pub fn extract_all_frames_sequential(
    filename: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    FrameExtractor::new(filename, output_dir)
        .with_mode(ExtractionMode::OpenCVSequential)
        .extract()
}

pub fn extract_all_frames_ffmpeg(
    filename: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    FrameExtractor::new(filename, output_dir)
        .with_mode(ExtractionMode::FFmpeg)
        .extract()
}

pub fn extract_frames_ffmpeg_interval(
    filename: &str,
    output_dir: &str,
    frame_interval: usize,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    FrameExtractor::new(filename, output_dir)
        .with_interval(frame_interval)
        .with_mode(ExtractionMode::FFmpegInterval)
        .extract()
}

pub fn extract_frames_opencv_interval(
    filename: &str,
    output_dir: &str,
    frame_interval: usize,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    FrameExtractor::new(filename, output_dir)
        .with_interval(frame_interval)
        .with_mode(ExtractionMode::OpenCVInterval)
        .extract()
}

pub fn extract_all_frames_rayon(
    filename: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    FrameExtractor::new(filename, output_dir)
        .with_interval(1)
        .with_mode(ExtractionMode::Parallel)
        .extract()
}
