use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SavingOption {
    Single,
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