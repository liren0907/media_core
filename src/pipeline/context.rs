use crate::analysis::types::AnalysisReport;
use crate::annotation::pipeline::AnnotationResult;
use crate::hls::pipeline::HLSResult;
use crate::metadata::types::MediaMetadata;
use crate::pipeline::error::PipelineError;
use crate::pipeline::traits::PipelineContext;
use crate::pipeline::types::MediaSource;
use crate::streaming::FrameData;
use opencv::prelude::*;
use opencv::videoio::{CAP_ANY, VideoCapture};
use std::any::Any;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

/// The unified context for the Media Core Pipeline.
///
/// This struct acts as the "Blackboard" where all data and resources are shared
/// between different processing steps.
pub struct MediaContext {
    /// The source input (File or Stream)
    pub source: MediaSource,

    /// Accumulated Metadata (from Metadata Module)
    pub metadata: Option<MediaMetadata>,

    /// Analysis Results (from Analysis Module)
    pub analysis: Option<AnalysisReport>,

    /// Annotation Results (from Annotation Module)
    pub annotation_result: Option<AnnotationResult>,

    /// Extracted Frames Buffer (from Streaming Module)
    pub extracted_frames: Vec<FrameData>,

    /// HLS Conversion Results (from HLS Module)
    pub hls_result: Option<HLSResult>,

    /// Internal resource cache for lazy loading.
    /// We use Arc<Mutex<...>> to ensure thread safety (Sync) as required by PipelineContext.
    resources: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl PipelineContext for MediaContext {}

impl MediaContext {
    /// Create a new context from a file path
    pub fn from_file(path: PathBuf) -> Self {
        Self {
            source: MediaSource::File(path),
            metadata: None,
            analysis: None,
            annotation_result: None,
            extracted_frames: Vec::new(),
            hls_result: None,
            resources: HashMap::new(),
        }
    }

    /// Create a new context from a stream URL
    pub fn from_stream(url: String) -> Self {
        Self {
            source: MediaSource::Stream(url),
            metadata: None,
            analysis: None,
            annotation_result: None,
            extracted_frames: Vec::new(),
            hls_result: None,
            resources: HashMap::new(),
        }
    }

    /// Get (or lazily initialize) the OpenCV VideoCapture resource.
    ///
    /// This ensures we only open the file once, even if multiple steps need it.
    /// Returns a Generic Guard to the VideoCapture.
    pub fn get_opencv_capture(
        &mut self,
    ) -> Result<std::sync::MutexGuard<'_, VideoCapture>, PipelineError> {
        const KEY: &str = "opencv_capture";

        if !self.resources.contains_key(KEY) {
            // Initialize the resource
            let path_str = match &self.source {
                MediaSource::File(p) => p.to_str().ok_or(PipelineError::ConfigurationError(
                    "Invalid file path".to_string(),
                ))?,
                MediaSource::Stream(s) => s.as_str(),
            };

            let cap = VideoCapture::from_file(path_str, CAP_ANY).map_err(|e| {
                PipelineError::StepFailed {
                    step_name: "ResourceInit".to_string(),
                    error: format!("Failed to open OpenCV capture: {}", e),
                }
            })?;

            if !cap.is_opened().unwrap_or(false) {
                return Err(PipelineError::StepFailed {
                    step_name: "ResourceInit".to_string(),
                    error: format!("OpenCV capture is not opened for: {}", path_str),
                });
            }

            // Wrap in Mutex to satisfy Sync requirements of PipelineContext
            let sync_resource = Mutex::new(cap);
            self.resources
                .insert(KEY.to_string(), Box::new(sync_resource));
        }

        // Retrieve, downcast to Mutex<VideoCapture>, and lock
        let mutex = self
            .resources
            .get(KEY)
            .and_then(|boxed| boxed.downcast_ref::<Mutex<VideoCapture>>())
            .ok_or_else(|| PipelineError::MissingResource(KEY.to_string()))?;

        mutex.lock().map_err(|_| PipelineError::StepFailed {
            step_name: "ResourceAccess".to_string(),
            error: "Failed to lock OpenCV capture mutex".to_string(),
        })
    }
}
