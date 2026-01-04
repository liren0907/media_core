use media_core::annotation::{
    AnnotationConfig, AnnotationType, DataSource, FrameAnnotator, TextPosition, VideoOutputConfig,
};
use std::error::Error;
use std::path::Path;

pub fn run_annotation_mode(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let mut input = String::new();
    let mut output = String::new();
    let mut text_position = TextPosition::TopLeft;
    let mut annotation_type = AnnotationType::Filename;
    let mut source_fps = 30.0;
    let mut video_out = false;
    let video_filename = String::new();

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
            "--pos" => {
                if i + 1 < args.len() {
                    text_position = match args[i + 1].to_lowercase().as_str() {
                        "top-left" => TextPosition::TopLeft,
                        "top-right" => TextPosition::TopRight,
                        "bottom-left" => TextPosition::BottomLeft,
                        "bottom-right" => TextPosition::BottomRight,
                        "center" => TextPosition::Center,
                        _ => {
                            println!("Warning: Unknown position '{}', defaulting to TopLeft", args[i + 1]);
                            TextPosition::TopLeft
                        }
                    };
                    i += 1;
                }
            }
            "--type" => {
                if i + 1 < args.len() {
                    match args[i + 1].to_lowercase().as_str() {
                        "filename" => annotation_type = AnnotationType::Filename,
                        "timestamp" => annotation_type = AnnotationType::Timestamp,
                        "custom" => {
                            // Will be set when --text is parsed, or default empty
                        }
                        _ => println!("Warning: Unknown type '{}', usage: filename|timestamp|custom", args[i + 1]),
                    }
                    i += 1;
                }
            }
            "--text" => {
                if i + 1 < args.len() {
                    let text_val = args[i + 1].clone();
                    annotation_type = AnnotationType::Custom(text_val);
                    i += 1;
                }
            }
            "--fps" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<f64>() {
                        source_fps = val;
                    }
                    i += 1;
                }
            }
            "--video-out" => {
                video_out = true;
            }
            "--help" => {
                print_usage();
                return Ok(());
            }
            _ => {}
        }
        i += 1;
    }

    if input.is_empty() || output.is_empty() {
        println!("Error: Input and output paths are required.");
        print_usage();
        return Ok(());
    }

    // Determine data source type
    let data_source = if Path::new(&input).is_dir() {
        DataSource::FrameDir(input)
    } else {
        DataSource::Image(input)
    };

    // Configure Video Output if requested
    let video_encoding = if video_out {
        Some(VideoOutputConfig {
            fps: source_fps as i32,
            filename: video_filename, // Internal use in library often, or empty
        })
    } else {
        None
    };

    let config = AnnotationConfig {
        input: data_source,
        output_path: output,
        text_position,
        annotation_type,
        source_fps: Some(source_fps),
        video_encoding,
        ..Default::default()
    };

    println!("Starting annotation processing...");
    if let Err(e) = FrameAnnotator::new(config).process() {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)));
    }
    println!("Annotation completed successfully.");

    Ok(())
}

fn print_usage() {
    println!("Usage: cargo run annotation [options]");
    println!("Options:");
    println!("  -i, --input <path>      Input file (image) or directory (frames)");
    println!("  -o, --output <path>     Output file path (image or video)");
    println!("  --pos <position>        Text position (top-left, top-right, bottom-left, bottom-right, center)");
    println!("  --type <type>           Annotation type (filename, timestamp, custom)");
    println!("  --text <text>           Custom text content (implies --type custom)");
    println!("  --fps <value>           Source FPS for video generation (default: 30.0)");
    println!("  --video-out             Enable video output encoding from frames");
    println!("  --help                  Show this help message");
}

