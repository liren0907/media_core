#[allow(unused_imports)]
use media_core::video_process::{
    ExtractionMode, FrameExtractor, SaveMode, extract_all_frames_ffmpeg,
    extract_all_frames_sequential, extract_frames, extract_frames_ffmpeg_interval,
    extract_frames_opencv_interval,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let video_path = "data/test.mp4";
    let output_dir = "output/video_process";

    // ============================================
    // FrameExtractor - Builder Pattern
    // Each mode is paired with its wrapper equivalent
    // ============================================
    if Path::new(video_path).exists() {
        std::fs::create_dir_all(output_dir)?;

        // OpenCVSequential mode
        // Extracts ALL frames sequentially using OpenCV
        let _extractor_sequential =
            FrameExtractor::new(video_path, output_dir).with_mode(ExtractionMode::OpenCVSequential);
        // _extractor_sequential.extract()?;
        // Equivalent wrapper: extract_all_frames_sequential(video_path, output_dir)?;

        // OpenCVInterval mode
        // Extracts every Nth frame using OpenCV seek
        let _extractor_interval = FrameExtractor::new(video_path, output_dir)
            .with_interval(30)
            .with_mode(ExtractionMode::OpenCVInterval);
        // _extractor_interval.extract()?;
        // Equivalent wrapper: extract_frames_opencv_interval(video_path, output_dir, 30)?;

        // FFmpeg mode
        // Extracts ALL frames using FFmpeg command
        let _extractor_ffmpeg =
            FrameExtractor::new(video_path, output_dir).with_mode(ExtractionMode::FFmpeg);
        // _extractor_ffmpeg.extract()?;
        // Equivalent wrapper: extract_all_frames_ffmpeg(video_path, output_dir)?;

        // FFmpegInterval mode
        // Extracts every Nth frame using FFmpeg select filter
        let _extractor_ffmpeg_interval = FrameExtractor::new(video_path, output_dir)
            .with_interval(30)
            .with_mode(ExtractionMode::FFmpegInterval);
        // _extractor_ffmpeg_interval.extract()?;
        // Equivalent wrapper: extract_frames_ffmpeg_interval(video_path, output_dir, 30)?;

        // Parallel mode with SingleDirectory
        // Extracts frames in parallel using Rayon, saves all to one folder
        let _extractor_parallel = FrameExtractor::new(video_path, output_dir)
            .with_interval(30)
            .with_mode(ExtractionMode::Parallel)
            .with_save_mode(SaveMode::SingleDirectory);
        // _extractor_parallel.extract()?;
        // Equivalent wrapper: extract_frames(video_path, output_dir, 30, "parallel", "single_directory")?;

        // OpenCVInterval with MultipleDirectory
        // Extracts frames and creates separate folder per video
        let _extractor_multi_dir = FrameExtractor::new(video_path, output_dir)
            .with_interval(30)
            .with_mode(ExtractionMode::OpenCVInterval)
            .with_save_mode(SaveMode::MultipleDirectory);
        // _extractor_multi_dir.extract()?;
        // Equivalent wrapper: extract_frames(video_path, output_dir, 30, "opencv_interval", "multiple_directory")?;

        // Parallel mode (all frames)
        // Extracts ALL frames in parallel using Rayon
        let _extractor_rayon = FrameExtractor::new(video_path, output_dir)
            .with_interval(1)
            .with_mode(ExtractionMode::Parallel);
        // _extractor_rayon.extract()?;
        // Equivalent wrapper: extract_all_frames_rayon(video_path, output_dir)?;
    }

    Ok(())
}
