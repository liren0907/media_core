use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub fps: i32,
    pub filename: String,
    pub filename_label: FilenameLabel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilenameLabel {
    pub position: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoProcessConfig {
    pub mode: String,
    pub input_directory: String,
    pub output_directory: String,
    pub frame_interval: usize,
    pub extraction_mode: String,
    pub save_mode: String,
    pub output_video: VideoConfig,
    pub benchmark: bool,
}

impl Default for VideoProcessConfig {
    fn default() -> Self {
        Self {
            mode: "batch".to_string(),
            input_directory: "./input".to_string(),
            output_directory: "./output".to_string(),
            frame_interval: 30,
            extraction_mode: "opencv_interval".to_string(),
            save_mode: "multiple_directory".to_string(),
            output_video: VideoConfig {
                fps: 30,
                filename: "output.mp4".to_string(),
                filename_label: FilenameLabel {
                    position: "top_left".to_string(),
                },
            },
            benchmark: false,
        }
    }
}

impl Default for FilenameLabel {
    fn default() -> Self {
        Self {
            position: "top_left".to_string(),
        }
    }
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            fps: 30,
            filename: "output.mp4".to_string(),
            filename_label: FilenameLabel::default(),
        }
    }
}
