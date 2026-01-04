use media_core::metadata::orchestrator::get_media_info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "data/test.mp4";
    println!("Analyzing: {}", input_file);

    let metadata = get_media_info(input_file, true)?;
    let json = serde_json::to_string_pretty(&metadata)?;
    println!("{}", json);

    Ok(())
}
