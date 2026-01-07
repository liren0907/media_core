use crate::pipeline::{PipelineStep, PipelineError, MediaContext};
use crate::streaming::extractor::StreamExtractor;
use crate::streaming::strategy::SamplingStrategy;

/// A pipeline step that extracts frames using the streaming module.
pub struct ExtractFrames {
    strategy: SamplingStrategy,
}

impl ExtractFrames {
    pub fn new(strategy: SamplingStrategy) -> Self {
        Self { strategy }
    }
}

impl PipelineStep<MediaContext> for ExtractFrames {
    fn name(&self) -> &str {
        "ExtractFrames"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let path_str = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError("ExtractFrames step requires a File source".to_string())
        })?;

        // Initialize StreamExtractor
        let mut extractor = StreamExtractor::new(path_str, Some(self.strategy.clone()))
            .map_err(|e| PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: e,
            })?;

        // Extract frames
        let frames = extractor.extract(None)
            .map_err(|e| PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: e,
            })?;

        // Store in context
        context.extracted_frames = frames;
        
        Ok(())
    }
}

