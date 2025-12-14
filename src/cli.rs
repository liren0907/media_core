use media_core::hls::{HLSConverter, HLSVodConfig};
use media_core::process::create_video_processor;
use media_core::{CaptureConfig, RTSPCapture, SavingOption};
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::thread;

pub fn print_usage() {
    println!("Media Core - RTSP Stream Extractor & Video Processor");
    println!();
    println!("USAGE:");
    println!("    cargo run <MODE> [OPTIONS]");
    println!();
    println!("MODES:");
    println!("    rtsp                              Run RTSP stream capture mode");
    println!("    process <config_file>             Run video processing mode");
    println!("    hls <input_file> <output_dir>     Convert video to HLS VOD format");
    println!("    hls --config <config_file>        Convert using JSON config file");
    println!("    config <subcommand>               Generate configuration files");
    println!("    help                              Show this help message");
    println!();
    println!("EXAMPLES:");
    println!(
        "    cargo run rtsp                           # Capture RTSP streams using config.json"
    );
    println!("    cargo run process video_config.json     # Process videos using video config");
    println!("    cargo run hls video.mp4 hls_output/     # Convert MP4 to HLS");
    println!("    cargo run hls --config hls_config.json  # Convert using config file");
    println!("    cargo run config rtsp                    # Generate default RTSP config");
    println!("    cargo run help                           # Show help");
}

/// Run configuration generation mode
pub fn run_config_mode(subcommand: &str) -> Result<(), Box<dyn Error>> {
    match subcommand {
        "rtsp" => {
            println!("⚙️  Generating default RTSP configuration...");
            media_core::rtsp::generate_default_config("config.json")?;
            println!("✅ Generated 'config.json' successfully!");
            Ok(())
        }
        "process" => {
            println!("⚙️  Generating default Video Processing configuration...");
            media_core::process::generate_default_config("process_config.json")?;
            println!("✅ Generated 'process_config.json' successfully!");
            Ok(())
        }
        "hls" => {
            println!("⚙️  Generating default HLS VOD configuration...");
            let default_config = HLSVodConfig::default();
            let json = serde_json::to_string_pretty(&default_config)?;
            std::fs::write("hls_config.json", json)?;
            println!("✅ Generated 'hls_config.json' successfully!");
            Ok(())
        }
        _ => {
            println!("Error: Unknown config subcommand '{}'", subcommand);
            print_usage();
            Ok(())
        }
    }
}

/// Run RTSP stream capture mode (original functionality)
pub fn run_rtsp_mode() -> Result<(), Box<dyn Error>> {
    println!("🎥 Starting RTSP Stream Capture Mode...");

    // Load configuration from file
    let config_file = File::open("config.json")?;
    let reader = BufReader::new(config_file);
    let config: CaptureConfig = serde_json::from_reader(reader)?;

    let mut handles = vec![];

    let (urls_to_process, show_preview_for_list) = match config.saving_option {
        SavingOption::Single => (vec![config.rtsp_url.clone()], config.show_preview),
        SavingOption::List => (config.rtsp_url_list.clone(), false),
        SavingOption::Both => {
            let mut urls = vec![config.rtsp_url.clone()];
            urls.extend(config.rtsp_url_list.clone());
            (urls, false)
        }
    };

    println!("📡 Processing {} RTSP stream(s)...", urls_to_process.len());

    for url in urls_to_process {
        let output_dir = config.output_directory.clone();
        // For 'Both' and 'List', show_preview is false for all streams.
        // For 'Single', it depends on the config.
        let show_preview = if config.rtsp_url == url {
            show_preview_for_list
        } else {
            false
        };
        let segment_duration = config.saved_time_duration;
        let use_fps = config.use_fps;
        let fps = config.fps;
        let hls_config = Some(config.hls.clone());

        let handle = thread::spawn(move || {
            match RTSPCapture::new(
                url.clone(),
                output_dir,
                show_preview,
                segment_duration,
                use_fps,
                fps,
                hls_config,
                false, // run_once: false for production use
            ) {
                Ok(mut capture) => {
                    println!("📹 Processing stream: {}", url);
                    if let Err(e) = capture.process_stream() {
                        eprintln!("❌ Error processing stream {}: {:?}", url, e);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to create RTSP capture for {}: {:?}", url, e);
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("✅ RTSP stream capture completed!");
    Ok(())
}

/// Run video processing mode (new Process module functionality)
pub fn run_process_mode(config_path: &str) -> Result<(), Box<dyn Error>> {
    println!("🎬 Starting Video Processing Mode...");
    println!("📄 Using config file: {}", config_path);

    // Create a video processor
    let mut processor = create_video_processor()?;

    // Run video extraction with the provided config
    match processor.run_video_extraction(config_path) {
        Ok(_) => {
            println!("✅ Video processing completed successfully!");

            // Print processing statistics
            let stats = processor.get_stats();
            println!("📊 Processing Statistics:");
            println!("   • Files processed: {}", stats.files_processed);
            println!("   • Files failed: {}", stats.files_failed);
            println!("   • Success rate: {:.2}%", stats.success_rate());
            println!("   • Processing time: {:?}", stats.processing_time);

            if !stats.errors.is_empty() {
                println!("⚠️  Errors encountered:");
                for error in &stats.errors {
                    println!("   • {}", error);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Video processing failed: {}", e);
            return Err(Box::new(e));
        }
    }

    Ok(())
}

/// Run HLS VOD conversion mode
pub fn run_hls_mode(args: &[String]) -> Result<(), Box<dyn Error>> {
    println!("🎬 Starting HLS VOD Conversion Mode...");

    if args.is_empty() {
        eprintln!("❌ Error: Missing arguments for HLS mode");
        println!();
        println!("Usage:");
        println!("    cargo run hls <input_file> <output_dir>");
        println!("    cargo run hls --config <config_file>");
        return Ok(());
    }

    let config = if args[0] == "--config" {
        // Load from config file
        if args.len() < 2 {
            eprintln!("❌ Error: Missing config file path");
            return Ok(());
        }
        println!("📄 Loading config from: {}", args[1]);
        HLSVodConfig::from_file(&args[1])?
    } else {
        // Direct arguments: <input_file> <output_dir>
        if args.len() < 2 {
            eprintln!("❌ Error: Missing output directory");
            println!("Usage: cargo run hls <input_file> <output_dir>");
            return Ok(());
        }
        let input_path = PathBuf::from(&args[0]);
        let output_dir = PathBuf::from(&args[1]);
        HLSVodConfig::new(input_path, output_dir)
    };

    println!("📥 Input: {}", config.input_path.display());
    println!("📤 Output: {}", config.output_dir.display());
    println!("⏱️  Segment Duration: {}s", config.segment_duration);

    let converter = HLSConverter::new(config);
    match converter.convert() {
        Ok(()) => {
            println!("✅ HLS conversion completed successfully!");
        }
        Err(e) => {
            eprintln!("❌ HLS conversion failed: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
