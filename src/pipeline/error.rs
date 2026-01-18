use std::fmt;

/// Standard error type for pipeline execution
#[derive(Debug)]
pub enum PipelineError {
    /// A step failed, and it was marked as critical
    StepFailed {
        step_name: String,
        error: String, // Kept generic to wrap any inner error
    },
    /// The pipeline was misconfigured
    ConfigurationError(String),
    /// A resource required by a step was missing from the context
    MissingResource(String),
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::StepFailed { step_name, error } => {
                write!(f, "Pipeline step '{}' failed: {}", step_name, error)
            }
            PipelineError::ConfigurationError(msg) => {
                write!(f, "Pipeline configuration error: {}", msg)
            }
            PipelineError::MissingResource(res) => {
                write!(f, "Missing required resource in context: {}", res)
            }
        }
    }
}

impl std::error::Error for PipelineError {}

