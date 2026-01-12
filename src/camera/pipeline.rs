use crate::camera::helpers::mat_to_base64_jpeg;
use crate::pipeline::error::PipelineError;
use crate::pipeline::traits::PipelineStep;
use crate::pipeline::MediaContext;
use crate::streaming::FrameData;
use opencv::prelude::*;

/// Pipeline step to capture a single frame from the camera
pub struct CaptureFrame;

impl PipelineStep<MediaContext> for CaptureFrame {
    fn name(&self) -> &str {
        "CaptureFrame"
    }

    fn execute(&self, context: &mut MediaContext) -> Result<(), PipelineError> {
        let mut frame = Mat::default();

        // Scope the lock so we don't hold the context borrow while processing
        {
            // Get access to the shared VideoCapture resource
            // This will lazily initialize the camera if it hasn't been opened yet
            let mut cap_guard = context.get_opencv_capture()?;

            if !cap_guard
                .read(&mut frame)
                .map_err(|e| PipelineError::StepFailed {
                    step_name: self.name().to_string(),
                    error: format!("Failed to read frame: {}", e),
                })?
            {
                return Err(PipelineError::StepFailed {
                    step_name: self.name().to_string(),
                    error: "Failed to read frame (camera disconnected?)".to_string(),
                });
            }
        }

        if frame.empty() {
            return Err(PipelineError::StepFailed {
                step_name: self.name().to_string(),
                error: "Captured frame is empty".to_string(),
            });
        }

        // Encode to base64
        let b64 = mat_to_base64_jpeg(&frame).map_err(|e| PipelineError::StepFailed {
            step_name: self.name().to_string(),
            error: format!("Encoding error: {}", e),
        })?;

        // Store in context
        let index = context.extracted_frames.len();
        context.extracted_frames.push(FrameData { index, data: b64 });

        Ok(())
    }
}
