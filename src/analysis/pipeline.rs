use crate::analysis::config::{MotionAlgorithm, MotionConfig};
use crate::analysis::motion::MotionDetector;
use crate::analysis::types::{AnalysisEvent, AnalysisReport};
use crate::pipeline::{MediaContext, PipelineError, PipelineStep};
use std::path::{Path, PathBuf};

/// A pipeline step that detects motion in the video.
pub struct DetectMotion {
    config: MotionConfig,
    output_dir: PathBuf,
}

impl DetectMotion {
    /// Create a new DetectMotion step with default configuration
    pub fn new() -> Self {
        Self {
            config: MotionConfig {
                algorithm: MotionAlgorithm::FrameDiff, // Default to FrameDiff (fastest)
                threshold: 25.0,
                min_area: 500,
                frame_skip: 0,
                roi: None,
                save_motion_clips: false,
            },
            output_dir: PathBuf::from("output/motion_debug"),
        }
    }

    /// Set the motion detection algorithm
    pub fn algorithm(mut self, algorithm: MotionAlgorithm) -> Self {
        self.config.algorithm = algorithm;
        self
    }

    /// Set the motion threshold
    pub fn threshold(mut self, threshold: f64) -> Self {
        self.config.threshold = threshold;
        self
    }

    /// Set the minimum area for motion detection
    pub fn min_area(mut self, min_area: i32) -> Self {
        self.config.min_area = min_area;
        self
    }

    /// Set the output directory for debug images (if any)
    pub fn output_dir<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.output_dir = path.as_ref().to_path_buf();
        self
    }
}

impl PipelineStep<MediaContext> for DetectMotion {
    fn name(&self) -> &str {
        "DetectMotion"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let path_str = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError(
                "DetectMotion step requires a File source".to_string(),
            )
        })?;
        let input_path = Path::new(path_str);

        // Initialize MotionDetector
        let mut detector =
            MotionDetector::new(self.config.clone()).map_err(|e| PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: format!("Failed to create MotionDetector: {}", e),
            })?;

        // Run motion detection
        // Note: usage of output_dir here depends on MotionDetector implementation
        // For now we pass it as required by the API
        let motion_segments = detector
            .process_video(input_path, &self.output_dir)
            .map_err(|e| PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: format!("Motion detection failed: {}", e),
            })?;

        // Convert segments to AnalysisReport
        let mut report = AnalysisReport::default();

        for (start, end) in motion_segments {
            report.motion_events.push(AnalysisEvent {
                start_frame: start,
                end_frame: end,
                event_type: "Motion".to_string(),
            });
        }

        // Store in context
        context.analysis = Some(report);

        Ok(())
    }
}

impl Default for DetectMotion {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SIMILARITY ANALYSIS STEPS
// ============================================================================

use crate::analysis::config::{ProcessMode, SimilarityConfig, SimilarityMethod};
use crate::analysis::similarity::SimilarityAnalyzer;
use crate::analysis::types::{ImageComparison, SimilarityGroup};

/// A pipeline step that groups similar images in a directory.
pub struct GroupSimilarImages {
    input_dir: PathBuf,
    output_dir: PathBuf,
    config: SimilarityConfig,
}

impl GroupSimilarImages {
    /// Create a new GroupSimilarImages step
    pub fn new<P: AsRef<Path>>(input_dir: P, output_dir: P) -> Self {
        Self {
            input_dir: input_dir.as_ref().to_path_buf(),
            output_dir: output_dir.as_ref().to_path_buf(),
            config: SimilarityConfig::default(),
        }
    }

    /// Set the similarity method (Histogram, PerceptualHash, FeatureMatching)
    pub fn method(mut self, method: SimilarityMethod) -> Self {
        self.config.method = method;
        self
    }

    /// Set the processing mode (Single or Parallel)
    pub fn process_mode(mut self, mode: ProcessMode) -> Self {
        self.config.process_mode = mode;
        self
    }

    /// Set the minimum category size (groups smaller than this are discarded)
    pub fn min_category_size(mut self, size: i32) -> Self {
        self.config.min_category_size = size;
        self
    }

    /// Set whether to physically group similar images into directories
    pub fn group_similar(mut self, group: bool) -> Self {
        self.config.group_similar = group;
        self
    }

    /// Set the similarity threshold (0.0-1.0) for histogram method
    pub fn histogram_threshold(mut self, threshold: f64) -> Self {
        self.config.histogram.similarity_threshold = threshold;
        self
    }

    /// Set the similarity threshold (0.0-1.0) for perceptual hash method
    pub fn phash_threshold(mut self, threshold: f64) -> Self {
        self.config.perceptual_hash.similarity_threshold = threshold;
        self
    }

    /// Set the similarity threshold (0.0-1.0) for feature matching method
    pub fn feature_threshold(mut self, threshold: f64) -> Self {
        self.config.feature_matching.similarity_threshold = threshold;
        self
    }
}

impl PipelineStep<MediaContext> for GroupSimilarImages {
    fn name(&self) -> &str {
        "GroupSimilarImages"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let mut analyzer = SimilarityAnalyzer::new(self.config.clone()).map_err(|e| {
            PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: format!("Failed to create SimilarityAnalyzer: {}", e),
            }
        })?;

        let groups = analyzer
            .group_similar_images(&self.input_dir, &self.output_dir)
            .map_err(|e| PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: format!("Similarity grouping failed: {}", e),
            })?;

        // Convert to SimilarityGroup structs
        let similarity_groups: Vec<SimilarityGroup> = groups
            .into_iter()
            .map(|(name, members)| SimilarityGroup {
                group_name: name,
                members,
            })
            .collect();

        // Store in context (append to existing report or create new)
        if let Some(ref mut report) = context.analysis {
            report.similarity_groups = similarity_groups;
        } else {
            let mut report = AnalysisReport::default();
            report.similarity_groups = similarity_groups;
            context.analysis = Some(report);
        }

        Ok(())
    }
}

/// A pipeline step that compares two specific images.
pub struct CompareImages {
    image1: PathBuf,
    image2: PathBuf,
    config: SimilarityConfig,
    threshold: f64,
}

impl CompareImages {
    /// Create a new CompareImages step
    pub fn new<P: AsRef<Path>>(image1: P, image2: P) -> Self {
        Self {
            image1: image1.as_ref().to_path_buf(),
            image2: image2.as_ref().to_path_buf(),
            config: SimilarityConfig {
                method: SimilarityMethod::PerceptualHash, // Default to pHash
                ..Default::default()
            },
            threshold: 0.95, // 95% default threshold
        }
    }

    /// Set the similarity method
    pub fn method(mut self, method: SimilarityMethod) -> Self {
        self.config.method = method;
        self
    }

    /// Set the duplicate threshold (0.0-1.0)
    pub fn threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold;
        self
    }
}

impl PipelineStep<MediaContext> for CompareImages {
    fn name(&self) -> &str {
        "CompareImages"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let mut analyzer = SimilarityAnalyzer::new(self.config.clone()).map_err(|e| {
            PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: format!("Failed to create SimilarityAnalyzer: {}", e),
            }
        })?;

        let similarity = analyzer
            .compare_images(&self.image1, &self.image2)
            .map_err(|e| PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: format!("Image comparison failed: {}", e),
            })?;

        let comparison = ImageComparison {
            image1: self.image1.to_string_lossy().to_string(),
            image2: self.image2.to_string_lossy().to_string(),
            similarity_score: similarity,
            is_duplicate: similarity >= self.threshold,
        };

        // Store in context
        if let Some(ref mut report) = context.analysis {
            report.image_comparison = Some(comparison);
        } else {
            let mut report = AnalysisReport::default();
            report.image_comparison = Some(comparison);
            context.analysis = Some(report);
        }

        Ok(())
    }
}
