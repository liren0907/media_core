use media_core::video_process::{ExtractionMode, FrameExtractor, SaveMode};
use std::error::Error;
use std::path::Path;

pub fn run_video_process_mode(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let mut input = String::new();
    let mut output = "output/video_process".to_string();
    let mut mode = ExtractionMode::Parallel;
    let mut save_mode = SaveMode::SingleDirectory;
    let mut interval = 1;

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
            "-m" | "--mode" => {
                if i + 1 < args.len() {
                    mode = match args[i + 1].to_lowercase().as_str() {
                        "parallel" => ExtractionMode::Parallel,
                        "ffmpeg" => ExtractionMode::FFmpeg,
                        "sequential" | "opencv_sequential" => ExtractionMode::OpenCVSequential,
                        "interval" | "opencv_interval" => ExtractionMode::OpenCVInterval,
                        "ffmpeg_interval" => ExtractionMode::FFmpegInterval,
                        _ => {
                            println!("Warning: Unknown mode '{}', defaulting to Parallel", args[i + 1]);
                            ExtractionMode::Parallel
                        }
                    };
                    i += 1;
                }
            }
            "--save-mode" => {
                if i + 1 < args.len() {
                    save_mode = match args[i + 1].to_lowercase().as_str() {
                        "single" | "single_directory" => SaveMode::SingleDirectory,
                        "multi" | "multiple_directory" => SaveMode::MultipleDirectory,
                        _ => {
                            println!("Warning: Unknown save mode '{}', defaulting to SingleDirectory", args[i + 1]);
                            SaveMode::SingleDirectory
                        }
                    };
                    i += 1;
                }
            }
            "--interval" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<usize>() {
                        interval = val;
                    }
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

    // Ensure output directory exists
    std::fs::create_dir_all(&output)?;

    println!("Starting video process...");
    println!("Input: {}", input);
    println!("Output: {}", output);
    println!("Mode: {:?}", mode);
    println!("Save Mode: {:?}", save_mode);
    println!("Interval: {}", interval);

    if let Err(e) = FrameExtractor::new(&input, &output)
        .with_interval(interval)
        .with_mode(mode)
        .with_save_mode(save_mode)
        .extract()
    {
        return Err(e as Box<dyn Error>);
    }

    println!("Video processing completed successfully.");

    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run video-process [options]");
    println!("Options:");
    println!("  -i, --input <path>      Input video file path (required)");
    println!("  -o, --output <path>     Output directory (default: output/video_process)");
    println!("  -m, --mode <mode>       Extraction mode:");
    println!("      parallel            Multi-threaded extraction (default)");
    println!("      ffmpeg              Using FFmpeg command");
    println!("      sequential          Sequential OpenCV reading");
    println!("      interval            OpenCV seek-based interval");
    println!("      ffmpeg_interval     FFmpeg select filter interval");
    println!("  --save-mode <mode>      Directory structure:");
    println!("      single              All frames in one directory (default)");
    println!("      multi               Create sub-folder for video");
    println!("  --interval <n>          Extract every Nth frame (default: 1)");
    println!("  --help                  Show this help message");
}

