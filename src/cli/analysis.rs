use media_core::analysis::{AnalysisConfig, AnalysisMode, MotionDetector, SimilarityAnalyzer};
use std::error::Error;
use std::path::PathBuf;

pub fn run_analysis_mode(args: &[String]) -> Result<(), Box<dyn Error>> {
    println!("🔍 Starting Analysis Mode...");

    if args.is_empty() {
        eprintln!("❌ Error: Missing arguments for analysis mode");
        println!();
        println!("Usage:");
        println!("    cargo run analysis --config <config_file>");
        println!("    cargo run analysis motion <video_file> <output_dir>");
        println!("    cargo run analysis similarity <image_dir> <output_dir>");
        return Ok(());
    }

    let (config, mode_override) = if args[0] == "--config" {
        if args.len() < 2 {
            eprintln!("❌ Error: Missing config file path");
            return Ok(());
        }
        println!("📄 Loading config from: {}", args[1]);
        (AnalysisConfig::from_file(&args[1])?, None)
    } else {
        let mode = args[0].as_str();
        if args.len() < 3 {
            eprintln!("❌ Error: Missing input or output path");
            return Ok(());
        }
        let input = PathBuf::from(&args[1]);
        let output = PathBuf::from(&args[2]);

        let mut config = AnalysisConfig::default();
        config.input_path = input;
        config.output_dir = output;

        (config, Some(mode.to_string()))
    };

    let mode = mode_override
        .map(|m| match m.as_str() {
            "motion" => AnalysisMode::Motion,
            "similarity" => AnalysisMode::Similarity,
            _ => config.mode.clone(),
        })
        .unwrap_or(config.mode.clone());

    match mode {
        AnalysisMode::Motion => {
            println!("🎬 Running Motion Detection...");
            let mut detector = MotionDetector::new(config.motion.clone())?;
            let segments = detector.process_video(&config.input_path, &config.output_dir)?;
            println!("✅ Found {} motion segments", segments.len());
        }
        AnalysisMode::Similarity => {
            println!("🖼️  Running Image Similarity Analysis...");
            let mut analyzer = SimilarityAnalyzer::new(config.similarity.clone())?;
            let groups = analyzer.group_similar_images(&config.input_path, &config.output_dir)?;
            println!("✅ Created {} image groups", groups.len());
        }
    }

    Ok(())
}
