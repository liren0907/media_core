use crate::pipeline::error::PipelineError;

/// The Context trait that holds state for a pipeline execution.
/// 
/// Modules implementing a pipeline must define a struct that implements this trait.
/// It acts as the shared workspace.
pub trait PipelineContext: Send + Sync {
    // Currently marker trait, can be expanded later for standardized logging/metrics
}

/// A single unit of work in the pipeline.
/// 
/// T: The concrete Context type this step operates on.
pub trait PipelineStep<T: PipelineContext>: Send + Sync {
    /// Unique name for debugging/logging
    fn name(&self) -> &str;

    /// The core logic of this step.
    /// 
    /// Implementations should read from `context` and write results back to `context`.
    fn execute(&self, context: &mut T) -> Result<(), PipelineError>;

    /// Policy: Should a failure in this step stop the entire pipeline?
    /// 
    /// Default: true (Critical).
    /// If false, the error is logged (via generic logging) and execution continues.
    fn is_critical(&self) -> bool {
        true
    }
}

