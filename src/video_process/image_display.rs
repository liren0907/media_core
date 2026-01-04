use opencv::{highgui, imgcodecs, prelude::*};
use std::error::Error;

pub fn display_image(file_path: &str) -> Result<(), Box<dyn Error>> {
    let img = imgcodecs::imread(file_path, imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        return Err("Failed to load image".into());
    }
    highgui::named_window("VIEWER", highgui::WINDOW_AUTOSIZE)?;
    highgui::imshow("VIEWER", &img)?;
    highgui::wait_key(0)?;
    Ok(())
}
