use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub video_encoder: String,
    pub audio_encoder: String,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FFprobeOutput {
    streams: Vec<StreamInfo>,
}

#[derive(Debug, Deserialize)]
struct StreamInfo {
    codec_type: Option<String>,
    codec_name: Option<String>,
}

pub fn probe_video_codecs(video_path: &str) -> Result<ProbeResult, String> {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_streams")
        .arg(video_path)
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}. Is FFmpeg installed?", e))?;

    if !output.status.success() {
        return Err(format!(
            "ffprobe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);

    let probe_data: FFprobeOutput = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse ffprobe output: {}", e))?;

    let mut video_codec: Option<String> = None;
    let mut audio_codec: Option<String> = None;

    for stream in &probe_data.streams {
        match stream.codec_type.as_deref() {
            Some("video") if video_codec.is_none() => {
                video_codec = stream.codec_name.clone();
            }
            Some("audio") if audio_codec.is_none() => {
                audio_codec = stream.codec_name.clone();
            }
            _ => {}
        }
    }

    let video_encoder = map_video_codec_to_encoder(video_codec.as_deref());
    let audio_encoder = map_audio_codec_to_encoder(audio_codec.as_deref());

    Ok(ProbeResult {
        video_encoder,
        audio_encoder,
        video_codec,
        audio_codec,
    })
}

fn map_video_codec_to_encoder(codec: Option<&str>) -> String {
    match codec {
        Some("h264") | Some("avc1") => "libx264".to_string(),
        Some("hevc") | Some("h265") | Some("hev1") | Some("hvc1") => "libx265".to_string(),
        Some("vp8") => "libvpx".to_string(),
        Some("vp9") => "libvpx-vp9".to_string(),
        Some("av1") => "libaom-av1".to_string(),
        Some("mpeg4") | Some("mp4v") => "mpeg4".to_string(),
        Some("mjpeg") => "mjpeg".to_string(),
        _ => "libx264".to_string(),
    }
}

fn map_audio_codec_to_encoder(codec: Option<&str>) -> String {
    match codec {
        Some("aac") => "aac".to_string(),
        Some("mp3") => "libmp3lame".to_string(),
        Some("opus") => "libopus".to_string(),
        Some("vorbis") => "libvorbis".to_string(),
        Some("flac") => "flac".to_string(),
        Some("pcm_s16le") | Some("pcm_s24le") | Some("pcm_s32le") => "pcm_s16le".to_string(),
        _ => "aac".to_string(),
    }
}

pub fn is_ffprobe_available() -> bool {
    Command::new("ffprobe")
        .arg("-version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn get_stream_info(video_path: &str) -> Result<Vec<StreamDetails>, String> {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("quiet")
        .arg("-print_format")
        .arg("json")
        .arg("-show_streams")
        .arg(video_path)
        .output()
        .map_err(|e| format!("Failed to execute ffprobe: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "ffprobe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let probe_data: FFprobeDetailedOutput = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse ffprobe output: {}", e))?;

    Ok(probe_data.streams)
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamDetails {
    pub index: Option<i32>,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub codec_type: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub sample_rate: Option<String>,
    pub channels: Option<i32>,
    pub bit_rate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FFprobeDetailedOutput {
    streams: Vec<StreamDetails>,
}
