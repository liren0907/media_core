#[allow(unused_imports)]
use media_core::benchmark::{Benchmark, Report};
use media_core::video_process::{ExtractionMode, FrameExtractor};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let video_path = "data/test.mp4";
    let output_dir = "output/benchmark";

    // ============================================
    // Benchmark - FrameExtractor Example
    // Wraps actual media_core function for timing
    // ============================================
    if Path::new(video_path).exists() {
        std::fs::create_dir_all(output_dir)?;

        let result = Benchmark::new("FrameExtractor Parallel Mode")
            .runs(3)
            .run(|| {
                FrameExtractor::new(video_path, output_dir)
                    .with_interval(30)
                    .with_mode(ExtractionMode::Parallel)
                    .extract()
            })?;

        result.print_summary();
        // result.to_markdown("benchmark_report.md")?;
        // result.to_json("benchmark_report.json")?;
    } else {
        println!("Video not found: {}", video_path);
        println!("Skipping benchmark.");
    }

    Ok(())
}
