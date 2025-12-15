use media_core::hls::{HLSConverter, HLSVodConfig};
use std::error::Error;
use std::path::PathBuf;

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
        if args.len() < 2 {
            eprintln!("❌ Error: Missing config file path");
            return Ok(());
        }
        println!("📄 Loading config from: {}", args[1]);
        HLSVodConfig::from_file(&args[1])?
    } else {
        if args.len() < 2 {
            eprintln!("❌ Error: Missing output directory");
            println!("Usage: cargo run hls <input_file> <output_dir>");
            return Ok(());
        }
        let input_path = PathBuf::from(&args[0]);
        let output_dir = PathBuf::from(&args[1]);
        HLSVodConfig::new(input_path, output_dir)
    };

    println!();
    println!("Configuration:");
    println!("  Input:           {}", config.input_path.display());
    println!("  Output:          {}", config.output_dir.display());
    println!("  Segment:         {}s", config.segment_duration);
    println!("  Playlist:        {}", config.playlist_filename);
    println!("  Force Keyframes: {}", config.force_keyframes);
    println!("  Profile:         {}", config.profile);
    println!("  Level:           {}", config.level);
    println!();

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
