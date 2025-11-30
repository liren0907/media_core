use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum SavingOption {
    Single,
    #[default]
    List,
    Both,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureConfig {
    pub rtsp_url: String,
    pub rtsp_url_list: Vec<String>,
    pub output_directory: String,
    pub show_preview: bool,
    pub saving_option: SavingOption,
    pub saved_time_duration: u64,
    pub use_fps: bool,
    pub fps: f64,
}

impl Default for CaptureConfig {
    fn default() -> Self {
        Self {
            rtsp_url: "rtsp://username:password@camera-ip:port/stream".to_string(),
            rtsp_url_list: vec![
                "rtsp://username:password@camera1-ip:port/stream".to_string(),
                "rtsp://username:password@camera2-ip:port/stream".to_string(),
            ],
            output_directory: "media".to_string(),
            show_preview: false,
            saving_option: SavingOption::List,
            saved_time_duration: 300,
            use_fps: false,
            fps: 30.0,
        }
    }
}
