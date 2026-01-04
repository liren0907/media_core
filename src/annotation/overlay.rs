use opencv::{
    core::{Point, Scalar},
    imgproc::{self, HersheyFonts, LineTypes},
    prelude::*,
};

use crate::annotation::types::TextPosition;

pub fn add_text_overlay(
    frame: &mut Mat,
    text: &str,
    position: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    add_text_overlay_with_position(frame, text, TextPosition::from_str(position))
}

pub fn add_text_overlay_with_position(
    frame: &mut Mat,
    text: &str,
    position: TextPosition,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let height = frame.rows() as i32;
    let width = frame.cols() as i32;
    let font = HersheyFonts::FONT_HERSHEY_SIMPLEX as i32;
    let font_scale = 1.0;
    let thickness = 2;
    let color = Scalar::new(255.0, 255.0, 255.0, 0.0);
    let line_type = LineTypes::LINE_AA as i32;
    let text_size = imgproc::get_text_size(text, font, font_scale, thickness, &mut 0)?;

    let point = match position {
        TextPosition::TopLeft => Point::new(10, 30),
        TextPosition::TopRight => Point::new(width - text_size.width - 10, 30),
        TextPosition::BottomLeft => Point::new(10, height - 20),
        TextPosition::BottomRight => Point::new(width - text_size.width - 10, height - 20),
    };

    imgproc::put_text(
        frame, text, point, font, font_scale, color, thickness, line_type, false,
    )?;

    Ok(())
}

pub fn annotate_frame(
    frame: &mut Mat,
    filename: &str,
    position: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    add_text_overlay(frame, filename, position)
}

pub fn annotate_frame_with_position(
    frame: &mut Mat,
    filename: &str,
    position: TextPosition,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    add_text_overlay_with_position(frame, filename, position)
}
