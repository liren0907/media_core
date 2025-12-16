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

/// Processing mode for similarity analysis
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProcessMode {
    #[default]
    Single,
    Parallel,
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

/// Configuration for histogram-based similarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramConfig {
    /// Number of histogram bins (default: 64)
    pub bins: i32,
    /// Similarity threshold for histogram method (0.0-1.0)
    pub similarity_threshold: f64,
}

impl Default for HistogramConfig {
    fn default() -> Self {
        Self {
            bins: 64,
            similarity_threshold: 0.8,
        }
    }
}

/// Configuration for ORB feature matching-based similarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureMatchingConfig {
    /// Maximum number of ORB features to detect (default: 500)
    pub max_features: i32,
    /// Percentage of best matches to keep (0.0-1.0, default: 0.15)
    pub good_match_percent: f64,
    /// Similarity threshold for feature matching method (0.0-1.0)
    pub similarity_threshold: f64,
}

impl Default for FeatureMatchingConfig {
    fn default() -> Self {
        Self {
            max_features: 500,
            good_match_percent: 0.15,
            similarity_threshold: 0.2,
        }
    }
}

/// Configuration for perceptual hash-based similarity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptualHashConfig {
    /// Size of the hash grid (default: 8, produces 64-bit hash)
    pub hash_size: i32,
    /// Similarity threshold for perceptual hash method (0.0-1.0)
    pub similarity_threshold: f64,
}

impl Default for PerceptualHashConfig {
    fn default() -> Self {
        Self {
            hash_size: 8,
            similarity_threshold: 0.9,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityConfig {
    /// Similarity comparison method
    pub method: SimilarityMethod,
    /// Processing mode: single-threaded or parallel
    pub process_mode: ProcessMode,
    /// Number of parallel workers (only used when process_mode is Parallel)
    pub parallel_num: usize,
    /// Global similarity threshold (0.0-1.0, used if method-specific not set)
    pub threshold: f64,
    /// Width to resize images before comparison
    pub resize_width: i32,
    /// Height to resize images before comparison
    pub resize_height: i32,
    /// Whether to group similar images into directories
    pub group_similar: bool,
    /// Minimum number of images required in a category (smaller groups are discarded)
    pub min_category_size: i32,
    /// Histogram-specific configuration
    pub histogram: HistogramConfig,
    /// Feature matching-specific configuration
    pub feature_matching: FeatureMatchingConfig,
    /// Perceptual hash-specific configuration
    pub perceptual_hash: PerceptualHashConfig,
}

impl Default for SimilarityConfig {
    fn default() -> Self {
        Self {
            method: SimilarityMethod::default(),
            process_mode: ProcessMode::default(),
            parallel_num: num_cpus::get(),
            threshold: 0.9,
            resize_width: 256,
            resize_height: 256,
            group_similar: true,
            min_category_size: 1,
            histogram: HistogramConfig::default(),
            feature_matching: FeatureMatchingConfig::default(),
            perceptual_hash: PerceptualHashConfig::default(),
        }
    }
}

impl SimilarityConfig {
    /// Get the effective threshold for the current method
    pub fn get_effective_threshold(&self) -> f64 {
        match self.method {
            SimilarityMethod::Histogram => self.histogram.similarity_threshold,
            SimilarityMethod::FeatureMatching => self.feature_matching.similarity_threshold,
            SimilarityMethod::PerceptualHash => self.perceptual_hash.similarity_threshold,
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
