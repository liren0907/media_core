use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AnalysisMode {
    #[default]
    Motion,
    Similarity,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MotionAlgorithm {
    #[default]
    FrameDiff,
    Mog2,
    Knn,
    OpticalFlow,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SimilarityMethod {
    #[default]
    Histogram,
    FeatureMatching,
    PerceptualHash,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionConfig {
    pub algorithm: MotionAlgorithm,
    pub threshold: f64,
    pub min_area: i32,
    pub frame_skip: i32,
    pub roi: Option<RegionOfInterest>,
    pub save_motion_clips: bool,
}

impl Default for MotionConfig {
    fn default() -> Self {
        Self {
            algorithm: MotionAlgorithm::default(),
            threshold: 25.0,
            min_area: 500,
            frame_skip: 1,
            roi: None,
            save_motion_clips: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionOfInterest {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityConfig {
    pub method: SimilarityMethod,
    pub threshold: f64,
    pub resize_width: i32,
    pub resize_height: i32,
    pub group_similar: bool,
}

impl Default for SimilarityConfig {
    fn default() -> Self {
        Self {
            method: SimilarityMethod::default(),
            threshold: 0.9,
            resize_width: 256,
            resize_height: 256,
            group_similar: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub mode: AnalysisMode,
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
    pub motion: MotionConfig,
    pub similarity: SimilarityConfig,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            mode: AnalysisMode::default(),
            input_path: PathBuf::new(),
            output_dir: PathBuf::from("analysis_output"),
            motion: MotionConfig::default(),
            similarity: SimilarityConfig::default(),
        }
    }
}

impl AnalysisConfig {
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}
