pub mod config;
pub mod motion;
pub mod similarity;
pub mod types;

pub use config::{AnalysisConfig, AnalysisMode, MotionConfig, SimilarityConfig};
pub use motion::MotionDetector;
pub use similarity::SimilarityAnalyzer;
pub use types::AnalysisError;
