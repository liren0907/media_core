use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum TextPosition {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl TextPosition {
    pub fn as_str(&self) -> &str {
        match self {
            TextPosition::TopLeft => "top_left",
            TextPosition::TopRight => "top_right",
            TextPosition::BottomLeft => "bottom_left",
            TextPosition::BottomRight => "bottom_right",
            TextPosition::Center => "center",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "top_right" => TextPosition::TopRight,
            "bottom_left" => TextPosition::BottomLeft,
            "bottom_right" => TextPosition::BottomRight,
            "center" => TextPosition::Center,
            _ => TextPosition::TopLeft,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum AnnotationType {
    #[default]
    Filename,
    Timestamp,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoOutputConfig {
    pub fps: i32,
    pub filename: String,
}

impl Default for VideoOutputConfig {
    fn default() -> Self {
        Self {
            fps: 30,
            filename: "annotated_output.mp4".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    Image(String),
    FrameDir(String),
}

impl Default for DataSource {
    fn default() -> Self {
        DataSource::FrameDir("./output".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationConfig {
    pub input: DataSource,
    pub output_path: String,
    pub text_position: TextPosition,
    pub annotation_type: AnnotationType,
    pub source_fps: Option<f64>,
    pub video_encoding: Option<VideoOutputConfig>,
}

impl Default for AnnotationConfig {
    fn default() -> Self {
        Self {
            input: DataSource::default(),
            output_path: "output.mp4".to_string(),
            text_position: TextPosition::TopLeft,
            annotation_type: AnnotationType::Filename,
            source_fps: Some(30.0),
            video_encoding: Some(VideoOutputConfig::default()),
        }
    }
}

pub fn format_timestamp(frame_index: usize, fps: f64) -> String {
    let total_seconds = frame_index as f64 / fps;
    let hours = (total_seconds / 3600.0) as u32;
    let minutes = ((total_seconds % 3600.0) / 60.0) as u32;
    let seconds = (total_seconds % 60.0) as u32;
    let millis = ((total_seconds % 1.0) * 1000.0) as u32;
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}
