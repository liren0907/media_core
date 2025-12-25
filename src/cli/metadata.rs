//! CLI handler for metadata extraction mode.

use media_core::metadata::{get_media_info, get_media_info_json};
use std::error::Error;

/// Run metadata extraction mode
pub fn run_metadata_mode(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        println!("Error: Metadata mode requires a media file path");
        println!("Usage: media_core metadata <file_path> [--json]");
        return Ok(());
    }

    let file_path = &args[0];
    let use_json = args.iter().any(|arg| arg == "--json");

    // Check if file exists
    if !std::path::Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path).into());
    }

    if use_json {
        // Output as JSON
        match get_media_info_json(file_path, false) {
            Ok(json) => println!("{}", json),
            Err(e) => return Err(format!("Failed to extract metadata: {}", e).into()),
        }
    } else {
        // Output as formatted text
        match get_media_info(file_path, false) {
            Ok(metadata) => {
                println!("═══════════════════════════════════════════════════════════");
                println!("                     MEDIA METADATA                        ");
                println!("═══════════════════════════════════════════════════════════");
                println!();
                println!("📁 File Information");
                println!("   Path:        {}", metadata.file_path);
                println!(
                    "   Size:        {:.2} MB ({} bytes)",
                    metadata.file_size_mb, metadata.file_size_bytes
                );
                println!("   Format:      {}", metadata.format);
                println!("   Media Type:  {}", metadata.media_type);
                println!();
                println!("🎬 Media Properties");
                println!("   Resolution:  {}", metadata.resolution);
                println!("   Dimensions:  {}x{}", metadata.width, metadata.height);
                println!("   Aspect Ratio: {:.2}", metadata.aspect_ratio);

                if let Some(duration) = metadata.duration_formatted.as_ref() {
                    println!("   Duration:    {}", duration);
                }
                if let Some(fps) = metadata.fps {
                    println!("   FPS:         {:.2}", fps);
                }
                if let Some(frames) = metadata.frame_count {
                    println!("   Frame Count: {}", frames);
                }
                if let Some(codec) = metadata.codec_name.as_ref() {
                    println!("   Codec:       {}", codec);
                }
                if let Some(bitrate) = metadata.bitrate_mbps {
                    println!("   Bitrate:     {:.2} Mbps", bitrate);
                }

                println!();
                println!("🎨 Color Information");
                println!("   Color Space: {}", metadata.color_space);
                println!("   Bit Depth:   {}", metadata.bit_depth);
                println!("   Channels:    {}", metadata.channels);
                println!();
                println!("📊 Quality Metrics");
                println!("   Quality:     {}", metadata.quality_category);
                println!("   Total Pixels: {}", metadata.total_pixels);
                println!("   Est. Memory: {:.2} MB", metadata.estimated_memory_mb);
                println!();
                println!("═══════════════════════════════════════════════════════════");
            }
            Err(e) => return Err(format!("Failed to extract metadata: {}", e).into()),
        }
    }

    Ok(())
}
