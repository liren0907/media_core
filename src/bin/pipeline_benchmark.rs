//! Pipeline Benchmark Example
//!
//! This example demonstrates the Unified Media Pipeline for Benchmarking.
//! It showcases ALL features of the benchmark module through the pipeline pattern:
//!
//! 1. Benchmark context access
//! 2. Benchmark metadata extraction with custom runs
//! 3. Benchmark frame extraction with custom frame count
//! 4. Print comprehensive benchmark summary
//!
//! Run with: cargo run --bin pipeline_benchmark

use media_core::benchmark::pipeline::{
    BenchmarkContext, BenchmarkFrameExtraction, BenchmarkMetadataExtraction,
};
use media_core::pipeline::{MediaContext, Pipeline};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("       Pipeline Benchmark Example");
    println!("===========================================\n");

    let video_path = "data/test.mp4";

    // Check if test file exists
    if !Path::new(video_path).exists() {
        eprintln!("⚠️  Test video not found: {}", video_path);
        eprintln!("   Place a test video at 'data/test.mp4' to run this example.");
        return Ok(());
    }

    // ============================================
    // 1. Benchmark Context Access
    // ============================================
    println!("🚀 1. Benchmark Context Access");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(BenchmarkContext::new("Context Access").runs(5));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(bench) = &result.benchmark_result {
        for r in &bench.results {
            println!(
                "   ✅ {}: Avg {:?}, Min {:?}, Max {:?}",
                r.name, r.average, r.min, r.max
            );
        }
    }
    println!();

    // ============================================
    // 2. Benchmark Metadata Extraction
    // ============================================
    println!("🚀 2. Benchmark Metadata Extraction");
    println!("----------------------------------------------");

    let pipeline =
        Pipeline::new().add_node(BenchmarkMetadataExtraction::new("Metadata Extraction").runs(3));

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(bench) = &result.benchmark_result {
        for r in &bench.results {
            println!(
                "   ✅ {}: Avg {:?}, Min {:?}, Max {:?}",
                r.name, r.average, r.min, r.max
            );
        }
    }
    println!();

    // ============================================
    // 3. Benchmark Frame Extraction
    // ============================================
    println!("🚀 3. Benchmark Frame Extraction (10 frames)");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new().add_node(
        BenchmarkFrameExtraction::new("Frame Extraction (10)")
            .runs(3)
            .frame_count(10),
    );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(bench) = &result.benchmark_result {
        for r in &bench.results {
            println!(
                "   ✅ {}: Avg {:?}, Min {:?}, Max {:?}",
                r.name, r.average, r.min, r.max
            );
        }
    }
    println!();

    // ============================================
    // 4. Combined Pipeline Benchmark
    // ============================================
    println!("🚀 4. Combined Pipeline Benchmark");
    println!("----------------------------------------------");

    let pipeline = Pipeline::new()
        .add_node(BenchmarkContext::new("1. Context Access").runs(3))
        .add_node(BenchmarkMetadataExtraction::new("2. Metadata").runs(2))
        .add_node(
            BenchmarkFrameExtraction::new("3. Frames (5)")
                .runs(2)
                .frame_count(5),
        );

    let context = MediaContext::from_file(Path::new(video_path).to_path_buf());
    let result = pipeline.execute(context)?;

    if let Some(bench) = &result.benchmark_result {
        bench.print_summary();
    }

    println!("===========================================");
    println!("       ✅ All Benchmark Examples Completed!");
    println!("===========================================");

    Ok(())
}
