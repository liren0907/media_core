use crate::annotation::annotator::FrameAnnotator;
use crate::annotation::types::{
    AnnotationConfig, AnnotationType, DataSource, TextPosition, VideoOutputConfig,
};
use crate::pipeline::{MediaContext, PipelineError, PipelineStep};
use std::path::{Path, PathBuf};

// ============================================================================
// ANNOTATION RESULT TYPES
// ============================================================================

/// Result of an annotation operation
#[derive(Debug, Clone, Default)]
pub struct AnnotationResult {
    pub output_path: String,
    pub annotation_type: String,
    pub success: bool,
}

// ============================================================================
// ANNOTATE FRAME STEP
// ============================================================================

/// A pipeline step that annotates a single image with text overlay.
pub struct AnnotateFrame {
    input_path: Option<PathBuf>,
    output_path: PathBuf,
    annotation_type: AnnotationType,
    text_position: TextPosition,
}

impl AnnotateFrame {
    /// Create a new AnnotateFrame step with a specific input file.
    pub fn new<P: AsRef<Path>>(input: P, output: P) -> Self {
        Self {
            input_path: Some(input.as_ref().to_path_buf()),
            output_path: output.as_ref().to_path_buf(),
            annotation_type: AnnotationType::Filename,
            text_position: TextPosition::TopLeft,
        }
    }

    /// Create a new AnnotateFrame step that uses the pipeline context's source as input.
    pub fn with_context_source<P: AsRef<Path>>(output: P) -> Self {
        Self {
            input_path: None,
            output_path: output.as_ref().to_path_buf(),
            annotation_type: AnnotationType::Filename,
            text_position: TextPosition::TopLeft,
        }
    }

    /// Set the annotation type (Filename, Timestamp, Custom)
    pub fn annotation_type(mut self, annotation_type: AnnotationType) -> Self {
        self.annotation_type = annotation_type;
        self
    }

    /// Set the text position
    pub fn position(mut self, position: TextPosition) -> Self {
        self.text_position = position;
        self
    }
}

impl PipelineStep<MediaContext> for AnnotateFrame {
    fn name(&self) -> &str {
        "AnnotateFrame"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        // Resolve input path: use step's path if available, otherwise try context source
        let input_path = if let Some(path) = &self.input_path {
            path.clone()
        } else {
            use crate::pipeline::types::MediaSource;
            match &context.source {
                MediaSource::File(path) => path.clone(),
                _ => {
                    return Err(PipelineError::ConfigurationError(
                        "AnnotateFrame requires a file source in the context if no input path is provided in the step".to_string(),
                    ))
                }
            }
        };

        let config = AnnotationConfig {
            input: DataSource::Image(input_path.to_string_lossy().to_string()),
            output_path: self.output_path.to_string_lossy().to_string(),
            text_position: self.text_position.clone(),
            annotation_type: self.annotation_type.clone(),
            source_fps: None,
            video_encoding: None,
        };

        let annotator = FrameAnnotator::new(config);
        annotator.process().map_err(|e| PipelineError::StepFailed {
            step_name: self.name().to_string(),
            error: format!("Frame annotation failed: {}", e),
        })?;

        // Store result in context
        let result = AnnotationResult {
            output_path: self.output_path.to_string_lossy().to_string(),
            annotation_type: format!("{:?}", self.annotation_type),
            success: true,
        };
        context.annotation_result = Some(result);

        Ok(())
    }
}

// ============================================================================
// ANNOTATE VIDEO STEP
// ============================================================================

/// A pipeline step that creates an annotated video from a directory of frames.
pub struct AnnotateVideo {
    frames_dir: Option<PathBuf>,
    output_path: PathBuf,
    annotation_type: AnnotationType,
    text_position: TextPosition,
    fps: i32,
    source_fps: f64,
}

impl AnnotateVideo {
    /// Create a new AnnotateVideo step with a specific frames directory.
    pub fn new<P: AsRef<Path>>(frames_dir: P, output: P) -> Self {
        Self {
            frames_dir: Some(frames_dir.as_ref().to_path_buf()),
            output_path: output.as_ref().to_path_buf(),
            annotation_type: AnnotationType::Filename,
            text_position: TextPosition::TopLeft,
            fps: 30,
            source_fps: 30.0,
        }
    }

    /// Create a new AnnotateVideo step that uses the pipeline context's source as the frames directory.
    pub fn with_context_source<P: AsRef<Path>>(output: P) -> Self {
        Self {
            frames_dir: None,
            output_path: output.as_ref().to_path_buf(),
            annotation_type: AnnotationType::Filename,
            text_position: TextPosition::TopLeft,
            fps: 30,
            source_fps: 30.0,
        }
    }

    /// Set the annotation type (Filename, Timestamp, Custom)
    pub fn annotation_type(mut self, annotation_type: AnnotationType) -> Self {
        self.annotation_type = annotation_type;
        self
    }

    /// Set the text position
    pub fn position(mut self, position: TextPosition) -> Self {
        self.text_position = position;
        self
    }

    /// Set the output video FPS
    pub fn fps(mut self, fps: i32) -> Self {
        self.fps = fps;
        self
    }

    /// Set the source frames FPS (used for timestamp calculation)
    pub fn source_fps(mut self, fps: f64) -> Self {
        self.source_fps = fps;
        self
    }
}

impl PipelineStep<MediaContext> for AnnotateVideo {
    fn name(&self) -> &str {
        "AnnotateVideo"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        // Resolve input path: use step's path if available, otherwise try context source
        let input_path = if let Some(path) = &self.frames_dir {
            path.clone()
        } else {
            use crate::pipeline::types::MediaSource;
            match &context.source {
                MediaSource::File(path) => path.clone(),
                _ => {
                    return Err(PipelineError::ConfigurationError(
                        "AnnotateVideo requires a file source (directory or video file) in the context if no input path is provided in the step".to_string(),
                    ))
                }
            }
        };

        // Determine Data Source Type
        let data_source = if input_path.is_dir() {
            DataSource::FrameDir(input_path.to_string_lossy().to_string())
        } else if input_path.is_file() {
            DataSource::Video(input_path.to_string_lossy().to_string())
        } else {
            // Fallback/Error if path doesn't exist yet (maybe it's a future output?)
            // For now, let's assume if extension is mp4/avi/mov it is video, else Dir?
            // Or safer: check existence. If not exists, standard pipeline error.
            if !input_path.exists() {
                return Err(PipelineError::ConfigurationError(format!(
                    "Input path does not exist: {:?}",
                    input_path
                )));
            }
            // Default fallback
            DataSource::Video(input_path.to_string_lossy().to_string())
        };

        let config = AnnotationConfig {
            input: data_source,
            output_path: self.output_path.to_string_lossy().to_string(),
            text_position: self.text_position.clone(),
            annotation_type: self.annotation_type.clone(),
            source_fps: Some(self.source_fps),
            video_encoding: Some(VideoOutputConfig {
                fps: self.fps,
                filename: String::new(),
            }),
        };

        let annotator = FrameAnnotator::new(config);
        annotator.process().map_err(|e| PipelineError::StepFailed {
            step_name: self.name().to_string(),
            error: format!("Video annotation failed: {}", e),
        })?;

        // Store result in context
        let result = AnnotationResult {
            output_path: self.output_path.to_string_lossy().to_string(),
            annotation_type: format!("{:?}", self.annotation_type),
            success: true,
        };
        context.annotation_result = Some(result);

        Ok(())
    }
}
