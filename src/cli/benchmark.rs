use media_core::benchmark::{Benchmark, Report};
use media_core::video_process::{ExtractionMode, FrameExtractor};
use std::error::Error;
use std::path::Path;

pub fn run_benchmark_mode(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let mut input = String::new();
    let mut output = "output/benchmark".to_string();
    let mut runs = 3;
    let mut mode_str = "parallel".to_string();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-i" | "--input" => {
                if i + 1 < args.len() {
                    input = args[i + 1].clone();
                    i += 1;
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output = args[i + 1].clone();
                    i += 1;
                }
            }
            "-r" | "--runs" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<usize>() {
                        runs = val;
                    }
                    i += 1;
                }
            }
            "-m" | "--mode" => {
                if i + 1 < args.len() {
                    mode_str = args[i + 1].clone();
                    i += 1;
                }
            }
            "--help" => {
                print_usage();
                return Ok(());
            }
            _ => {}
        }
        i += 1;
    }

    if input.is_empty() {
        println!("Error: Input video path is required.");
        print_usage();
        return Ok(());
    }

    if !Path::new(&input).exists() {
        println!("Error: Input file '{}' does not exist.", input);
        return Ok(());
    }

    std::fs::create_dir_all(&output)?;

    println!("Starting benchmark with {} runs...", runs);
    println!("Input: {}", input);
    println!("Output: {}", output);
    println!("Mode: {}", mode_str);

    let extraction_mode = match mode_str.to_lowercase().as_str() {
        "parallel" => ExtractionMode::Parallel,
        "ffmpeg" => ExtractionMode::FFmpeg,
        "sequential" | "opencv_sequential" => ExtractionMode::OpenCVSequential,
        "interval" | "opencv_interval" => ExtractionMode::OpenCVInterval,
        _ => {
            println!("Warning: Unknown mode '{}', defaulting to Parallel", mode_str);
            ExtractionMode::Parallel
        }
    };

    let benchmark_name = format!("FrameExtractor {} Mode", mode_str);
    
    // Clone values for the closure
    let input_path = input.clone();
    let output_path = output.clone();

    // Use a RefCell or similar if we needed to mutate outside, but here we just run logic
    let result = Benchmark::new(&benchmark_name)
        .runs(runs)
        .run(|| {
            FrameExtractor::new(&input_path, &output_path)
                .with_interval(30) // Standardize on interval 30 for benchmark
                .with_mode(extraction_mode.clone())
                .extract()
        })
        .map_err(|e| e as Box<dyn Error>)?;

    result.print_summary();
    
    // Optional: Save report
    let report_path = format!("{}/benchmark_report.json", output);
    result.to_json(&report_path)?;
    println!("Report saved to: {}", report_path);

    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run benchmark [options]");
    println!("Options:");
    println!("  -i, --input <path>      Input video file path (required)");
    println!("  -o, --output <path>     Output directory (default: output/benchmark)");
    println!("  -r, --runs <number>     Number of runs (default: 3)");
    println!("  -m, --mode <mode>       Extraction mode (parallel, ffmpeg, sequential, interval)");
    println!("  --help                  Show this help message");
}

