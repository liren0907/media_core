use opencv::{
    prelude::*,
    videoio::{self, CAP_PROP_FPS, CAP_PROP_FRAME_COUNT, VideoCapture},
};
use std::fs;
use std::path::Path;

pub fn get_output_path(
    base_dir: &str,
    video_path: &str,
    frame_number: usize,
    save_mode: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let video_name = Path::new(video_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    match save_mode {
        "single_directory" => {
            fs::create_dir_all(base_dir)?;
            Ok(format!("{}/{}_{}.jpg", base_dir, video_name, frame_number))
        }
        _ => {
            let video_output_dir = Path::new(base_dir).join(video_name);
            fs::create_dir_all(&video_output_dir)?;
            Ok(format!(
                "{}/frame_{}.jpg",
                video_output_dir.to_str().unwrap(),
                frame_number
            ))
        }
    }
}

pub fn get_video_duration(filename: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let cap = VideoCapture::from_file(filename, videoio::CAP_ANY.into())?;
    let fps = cap.get(CAP_PROP_FPS)? as f64;
    let frame_count = cap.get(CAP_PROP_FRAME_COUNT)? as f64;
    let duration_seconds = frame_count / fps;
    Ok(duration_seconds)
}

pub fn get_video_info(
    filename: &str,
) -> Result<(f64, String, String, (i32, i32), f64, f64), Box<dyn std::error::Error>> {
    let cap = VideoCapture::from_file(filename, videoio::CAP_ANY.into())?;
    let fps = cap.get(CAP_PROP_FPS)? as f64;
    let frame_count = cap.get(CAP_PROP_FRAME_COUNT)? as f64;
    let duration_seconds = frame_count / fps;

    let codec = cap.get(videoio::CAP_PROP_FOURCC)? as i32;
    let codec_str = format!(
        "{}{}{}{}",
        (codec & 0xFF) as u8 as char,
        ((codec >> 8) & 0xFF) as u8 as char,
        ((codec >> 16) & 0xFF) as u8 as char,
        ((codec >> 24) & 0xFF) as u8 as char
    );

    let codec_name = match codec_str.as_str() {
        "avc1" | "h264" => "H264",
        "hev1" | "hvc1" => "H265",
        "mp4v" => "MPEG-4 Part 2",
        "mp4a" => "MPEG-4 AAC",
        _ => "Unknown",
    }
    .to_string();

    let width = cap.get(videoio::CAP_PROP_FRAME_WIDTH)? as i32;
    let height = cap.get(videoio::CAP_PROP_FRAME_HEIGHT)? as i32;

    Ok((
        duration_seconds,
        codec_name,
        codec_str,
        (width, height),
        frame_count,
        fps,
    ))
}
