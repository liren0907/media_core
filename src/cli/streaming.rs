use media_core::streaming::{ExtractionMode, SamplingStrategy, StreamExtractor};
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn run_streaming_mode(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let mut input = String::new();
    let mut output = "output/streaming".to_string();
    let mut strategy = SamplingStrategy::EveryNth(30); // Default
    let mut scale_factor = None;
    let mut mode = ExtractionMode::Seek; // Default for simpler one-shot CLI

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
            "--strategy" => {
                if i + 1 < args.len() {
                    let strategy_str = args[i + 1].to_lowercase();
                    i += 1;
                    
                    if strategy_str == "keyframes" {
                        strategy = SamplingStrategy::KeyFrames;
                    } else if strategy_str == "interval" {
                        if i < args.len() {
                            if let Ok(val) = args[i].parse::<usize>() {
                                strategy = SamplingStrategy::EveryNth(val);
                                i += 1;
                            } else {
                                println!("Error: --strategy interval requires a number");
                                return Ok(());
                            }
                        }
                    } else if strategy_str == "first" {
                         if i < args.len() {
                            if let Ok(val) = args[i].parse::<usize>() {
                                strategy = SamplingStrategy::FirstN(val);
                                i += 1;
                            } else {
                                println!("Error: --strategy first requires a number");
                                return Ok(());
                            }
                        }
                    } else if strategy_str == "range" {
                         if i + 1 < args.len() {
                            let start = args[i].parse::<usize>().unwrap_or(0);
                            let end = args[i+1].parse::<usize>().unwrap_or(0);
                            strategy = SamplingStrategy::Range(start, end);
                            i += 2;
                        } else {
                            println!("Error: --strategy range requires two numbers (start end)");
                            return Ok(());
                        }
                    } else {
                        println!("Error: Unknown strategy '{}'", strategy_str);
                        return Ok(());
                    }
                }
            }
            "--scale" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<f64>() {
                        scale_factor = Some(val);
                    }
                    i += 1;
                }
            }
            "--sequential" => {
                mode = ExtractionMode::Sequential;
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

    fs::create_dir_all(&output)?;

    println!("Starting streaming extraction...");
    println!("Input: {}", input);
    println!("Strategy: {:?}", strategy);
    if let Some(scale) = scale_factor {
        println!("Scale Factor: {}", scale);
    }

    let mut extractor = StreamExtractor::new(&input, Some(strategy))
        .map_err(|e| format!("Failed to create extractor: {}", e))?;
    
    extractor.set_mode(mode);

    let frames = extractor.extract(scale_factor)
        .map_err(|e| format!("Extraction failed: {}", e))?;

    println!("Extracted {} frames.", frames.len());

    // Save frames to disk as demonstration
    for (_idx, frame) in frames.iter().enumerate() {
        let file_path = format!("{}/frame_{:04}.jpg", output, frame.index);
        // data is Vec<u8> (likely encoded image bytes if using opencv imencode, or raw? 
        // Based on bin/streaming.rs it says "Base64 length" which implies encoded or just raw bytes.
        // Looking at src/streaming/extractor.rs would confirm, but usually these are encoded bytes ready for file.
        // Let's assume they are bytes we can write directly.
        fs::write(&file_path, &frame.data)?;
    }
    println!("Frames saved to {}", output);

    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run streaming [options]");
    println!("Options:");
    println!("  -i, --input <path>      Input video file path (required)");
    println!("  -o, --output <path>     Output directory (default: output/streaming)");
    println!("  --strategy <type> [args] Sampling strategy:");
    println!("      interval <n>        Every Nth frame");
    println!("      keyframes           All keyframes");
    println!("      first <n>           First N frames");
    println!("      range <start> <end> Frames in range [start, end]");
    println!("  --scale <factor>        Resize frames (0.0 - 1.0)");
    println!("  --sequential            Use sequential reading mode (slower but more compatible)");
    println!("  --help                  Show this help message");
}

