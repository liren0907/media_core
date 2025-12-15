mod cli;

use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cli::print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "rtsp" => cli::run_rtsp_mode()?,
        "process" => {
            if args.len() < 3 {
                println!("Error: Process mode requires a config file path");
                println!("Usage: cargo run process <config_file_path>");
                return Ok(());
            }
            if args[2] == "extract" {
                if args.len() < 5 {
                    println!("Error: Extract mode requires input and output paths");
                    println!("Usage: cargo run process extract <input_file> <output_dir>");
                    return Ok(());
                }
                cli::run_extraction_mode(&args[3], &args[4])?;
            } else {
                cli::run_process_mode(&args[2])?;
            }
        }
        "hls" => {
            cli::run_hls_mode(&args[2..])?;
        }
        "analysis" => {
            cli::run_analysis_mode(&args[2..])?;
        }
        "config" => {
            if args.len() < 3 {
                println!(
                    "Error: Config mode requires a subcommand (e.g., 'rtsp', 'hls', 'analysis')"
                );
                println!("Usage: cargo run config <subcommand>");
                return Ok(());
            }
            cli::run_config_mode(&args[2])?;
        }
        "help" | "--help" | "-h" => cli::print_usage(),
        _ => {
            println!("Error: Unknown mode '{}'", args[1]);
            cli::print_usage();
        }
    }

    Ok(())
}
