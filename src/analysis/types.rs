use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AnalysisError {
    IoError(String),
    OpenCVError(String),
    ConfigError(String),
    InvalidInput(String),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisError::IoError(msg) => write!(f, "I/O Error: {}", msg),
            AnalysisError::OpenCVError(msg) => write!(f, "OpenCV Error: {}", msg),
            AnalysisError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
            AnalysisError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
        }
    }
}

impl Error for AnalysisError {}

impl From<std::io::Error> for AnalysisError {
    fn from(err: std::io::Error) -> Self {
        AnalysisError::IoError(err.to_string())
    }
}

impl From<opencv::Error> for AnalysisError {
    fn from(err: opencv::Error) -> Self {
        AnalysisError::OpenCVError(err.message)
    }
}

/// Represents a detected event in the video
#[derive(Debug, Clone)]
pub struct AnalysisEvent {
    pub start_frame: i32,
    pub end_frame: i32,
    pub event_type: String,
}

/// Represents a group of similar images
#[derive(Debug, Clone)]
pub struct SimilarityGroup {
    pub group_name: String,
    pub members: Vec<String>,
}

/// Represents the result of comparing two images
#[derive(Debug, Clone)]
pub struct ImageComparison {
    pub image1: String,
    pub image2: String,
    pub similarity_score: f64,
    pub is_duplicate: bool,
}

/// Consolidated report of all analysis performed on the media
#[derive(Debug, Clone, Default)]
pub struct AnalysisReport {
    pub motion_events: Vec<AnalysisEvent>,
    pub similarity_groups: Vec<SimilarityGroup>,
    pub image_comparison: Option<ImageComparison>,
}
