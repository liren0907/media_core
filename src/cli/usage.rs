pub fn print_usage() {
    println!("Media Core - RTSP Stream Extractor & Video Processor");
    println!();
    println!("USAGE:");
    println!("    ./media_core <MODE> [OPTIONS]");
    println!();
    println!("MODES:");
    println!("    rtsp                              Run RTSP stream capture mode");
    println!("    process <config_file>             Run video processing mode");
    println!("    hls <input_file> <output_dir>     Convert video to HLS VOD format");
    println!("    hls --config <config_file>        Convert using JSON config file");
    println!("    analysis motion <video> <output>  Run motion detection");
    println!("    analysis similarity <dir> <out>   Run image similarity analysis");
    println!("    config <subcommand>               Generate configuration files");
    println!("    help                              Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("    ./media_core rtsp                           # Capture RTSP streams");
    println!("    ./media_core process video_config.json      # Process videos");
    println!("    ./media_core hls video.mp4 hls_output/      # Convert MP4 to HLS");
    println!("    ./media_core hls --config hls_config.json   # Convert using config");
    println!("    ./media_core analysis motion video.mp4 out/ # Detect motion");
    println!("    ./media_core config rtsp                    # Generate RTSP config");
    println!("    ./media_core help                           # Show help");
}
