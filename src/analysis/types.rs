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
