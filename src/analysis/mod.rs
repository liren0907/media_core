pub mod config;
pub mod hashing;
pub mod motion;
pub mod similarity;
pub mod types;

pub mod pipeline;

pub use config::AnalysisConfig;
pub use config::AnalysisMode;
pub use config::FeatureMatchingConfig;
pub use config::HistogramConfig;
pub use config::MotionConfig;
pub use config::PerceptualHashConfig;
pub use config::ProcessMode;
pub use config::SimilarityConfig;
pub use config::SimilarityMethod;
pub use hashing::{Clusterer, ClusteringConfig, Hasher, SimilarityResult};
pub use motion::MotionDetector;
pub use similarity::{ProcessingStats, SimilarityAnalyzer};
pub use types::AnalysisError;
