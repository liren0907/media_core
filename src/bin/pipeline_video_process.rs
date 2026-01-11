use media_core::pipeline::{MediaContext, Pipeline};
use media_core::video_process::{ExtractFramesToDisk, ExtractionMode, VideoProcessResult};
use std::path::Path;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline Video Process Example");
    println!("===========================================\n");

    let video_path = "data/test.mp4";
    let output_base = "output/pipeline_video_process";

    let print_summary = |vp: &VideoProcessResult, elapsed: Duration| {
        println!(
            "   ✅ Mode={} | Frames={} | Time={}ms | Output={}",
            vp.extraction_mode,
            vp.frames_extracted,
            elapsed.as_millis(),
            vp.output_dir
        );
    };

    // 1. Basic Frame Extraction (OpenCV Interval)
    // Extracts frames every 30 frames using the default OpenCV backend (Interval Mode).
    // NOTE: This uses the default 'MultipleDirectory' save mode (nested folders).
    // You can also use .save_mode(SaveMode::SingleDirectory) to save all frames in one flat folder.
    println!("🚀 1. Basic Frame Extraction (OpenCV Interval)");

    let pipeline = Pipeline::new()
        .add_node(ExtractFramesToDisk::new(format!("{}/basic", output_base)).interval(30));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let start = Instant::now();
    let result = pipeline.execute(context)?;
    let elapsed = start.elapsed();

    if let Some(vp) = &result.video_process_result {
        print_summary(vp, elapsed);
    }

    // 2. Parallel Extraction Mode
    // Uses multiple threads to extract frames in parallel, significantly speeding up processing for large videos.
    // Save Mode:
    // - Default: MultipleDirectory (frames saved under output/<video_stem>/...)
    // - Option: use .save_mode(SaveMode::SingleDirectory) to save all frames in one flat folder
    println!("🚀 2. Parallel Extraction Mode");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/parallel", output_base))
            .interval(30)
            .mode(ExtractionMode::Parallel),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let start = Instant::now();
    let result = pipeline.execute(context)?;
    let elapsed = start.elapsed();

    if let Some(vp) = &result.video_process_result {
        print_summary(vp, elapsed);
    }

    // 3. FFmpeg Interval Extraction
    // Uses the FFmpeg backend to extract frames at a specific interval, offering robust format support.
    println!("🚀 3. FFmpeg Interval Extraction");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/ffmpeg", output_base))
            .interval(30)
            .mode(ExtractionMode::FFmpegInterval),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let start = Instant::now();
    let result = pipeline.execute(context)?;
    let elapsed = start.elapsed();

    if let Some(vp) = &result.video_process_result {
        print_summary(vp, elapsed);
    }

    // 4. OpenCV Sequential Mode (ALL frames)
    // Extracts every single frame from the video sequentially using OpenCV.
    println!("🚀 4. OpenCV Sequential Mode (ALL frames)");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/opencv_sequential", output_base))
            .interval(1) // Every frame
            .mode(ExtractionMode::OpenCVSequential),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let start = Instant::now();
    let result = pipeline.execute(context)?;
    let elapsed = start.elapsed();

    if let Some(vp) = &result.video_process_result {
        print_summary(vp, elapsed);
    }

    // 5. FFmpeg Mode (ALL frames)
    // Extracts every single frame from the video using FFmpeg.
    println!("🚀 5. FFmpeg Mode (ALL frames)");

    let pipeline = Pipeline::new().add_node(
        ExtractFramesToDisk::new(format!("{}/ffmpeg_all", output_base))
            .mode(ExtractionMode::FFmpeg),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let start = Instant::now();
    let result = pipeline.execute(context)?;
    let elapsed = start.elapsed();

    if let Some(vp) = &result.video_process_result {
        print_summary(vp, elapsed);
    }

    println!("===========================================");
    println!("       ✅ All Video Process Examples Completed!");
    println!("===========================================");

    Ok(())
}
