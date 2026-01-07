use crate::pipeline::{MediaContext, PipelineError, PipelineStep};
use crate::streaming::extractor::{ExtractionMode, StreamExtractor};
use crate::streaming::strategy::SamplingStrategy;

/// A pipeline step that extracts frames using the streaming module.
///
/// Supports all features of StreamExtractor:
/// - Sampling strategies: EveryNth, FirstN, Range, KeyFrames, Custom
/// - Scale factor for resizing frames
/// - Extraction modes: Random (default) or Sequential
///
/// # Example
/// ```rust
/// let step = ExtractFrames::new(SamplingStrategy::EveryNth(30))
///     .with_scale(0.5)
///     .with_mode(ExtractionMode::Sequential);
/// ```
pub struct ExtractFrames {
    strategy: SamplingStrategy,
    scale_factor: Option<f64>,
    extraction_mode: Option<ExtractionMode>,
}

impl ExtractFrames {
    /// Create a new ExtractFrames step with the given sampling strategy.
    pub fn new(strategy: SamplingStrategy) -> Self {
        Self {
            strategy,
            scale_factor: None,
            extraction_mode: None,
        }
    }

    /// Set the scale factor for frame resizing (0.0-1.0).
    ///
    /// - 1.0 = full size (default)
    /// - 0.5 = half size
    /// - 0.25 = quarter size
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale_factor = Some(scale);
        self
    }

    /// Set the extraction mode.
    ///
    /// - `ExtractionMode::Random` (default): Efficient for sparse sampling
    /// - `ExtractionMode::Sequential`: Better for dense/continuous reading
    pub fn with_mode(mut self, mode: ExtractionMode) -> Self {
        self.extraction_mode = Some(mode);
        self
    }
}

impl PipelineStep<MediaContext> for ExtractFrames {
    fn name(&self) -> &str {
        "ExtractFrames"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let path_str = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError(
                "ExtractFrames step requires a File source".to_string(),
            )
        })?;

        // Initialize StreamExtractor
        let mut extractor =
            StreamExtractor::new(path_str, Some(self.strategy.clone())).map_err(|e| {
                PipelineError::StepFailed {
                    step_name: self.name().to_string(),
                    error: e,
                }
            })?;

        // Apply extraction mode if specified
        if let Some(mode) = self.extraction_mode {
            extractor.set_mode(mode);
        }

        // Extract frames with optional scale factor
        let frames =
            extractor
                .extract(self.scale_factor)
                .map_err(|e| PipelineError::StepFailed {
                    step_name: self.name().to_string(),
                    error: e,
                })?;

        // Store in context
        context.extracted_frames = frames;

        Ok(())
    }
}
