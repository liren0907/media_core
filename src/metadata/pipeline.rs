use crate::pipeline::{PipelineStep, PipelineError, MediaContext};
use crate::metadata::orchestrator::get_media_info;

/// A pipeline step that extracts metadata using the existing orchestrator logic.
/// This acts as an adapter to the legacy metadata module.
pub struct ExtractMetadata {
    include_thumbnail: bool,
}

impl ExtractMetadata {
    pub fn new(include_thumbnail: bool) -> Self {
        Self { include_thumbnail }
    }
}

impl PipelineStep<MediaContext> for ExtractMetadata {
    fn name(&self) -> &str {
        "ExtractMetadata"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let path_str = context.source.as_path_str().ok_or_else(|| {
            PipelineError::ConfigurationError("ExtractMetadata step requires a File source".to_string())
        })?;

        // Use the existing logic from metadata module
        // Note: This is efficient for now, but in the future we should break get_media_info 
        // into smaller pieces to use the context's shared VideoCapture.
        match get_media_info(path_str, self.include_thumbnail) {
            Ok(metadata) => {
                context.metadata = Some(metadata);
                Ok(())
            }
            Err(e) => Err(PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: e,
            }),
        }
    }
}

