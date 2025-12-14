use media_core::hls::HLSVodConfig;
use std::error::Error;

use super::print_usage;

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
