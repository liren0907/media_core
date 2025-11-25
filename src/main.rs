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
            cli::run_process_mode(&args[2])?;
        }
        "help" | "--help" | "-h" => cli::print_usage(),
        _ => {
            println!("Error: Unknown mode '{}'", args[1]);
            cli::print_usage();
        }
    }

    Ok(())
}
