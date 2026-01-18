use crate::pipeline::traits::{PipelineContext, PipelineStep};
use crate::pipeline::error::PipelineError;

/// The Linear Executor for a sequence of steps.
pub struct Pipeline<T: PipelineContext> {
    steps: Vec<Box<dyn PipelineStep<T>>>,
}

impl<T: PipelineContext> Pipeline<T> {
    /// Create a new empty pipeline
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    /// Add a node to the end of the pipeline
    pub fn add_node<S: PipelineStep<T> + 'static>(mut self, step: S) -> Self {
        self.steps.push(Box::new(step));
        self
    }

    /// Add a generic boxed node (useful for dynamic construction)
    pub fn add_boxed_node(mut self, step: Box<dyn PipelineStep<T>>) -> Self {
        self.steps.push(step);
        self
    }

    /// Execute the pipeline on the given context
    pub fn execute(&self, mut context: T) -> Result<T, PipelineError> {
        for step in &self.steps {
            // println!("Executing step: {}", step.name()); // Optional debug log

            match step.execute(&mut context) {
                Ok(_) => continue,
                Err(e) => {
                    if step.is_critical() {
                        // Critical failure - abort
                        return Err(e);
                    } else {
                        // Non-critical failure - log and continue
                        // For now we just print to stderr, later integrate with proper logger
                        eprintln!("Warning: Non-critical step '{}' failed: {}", step.name(), e);
                    }
                }
            }
        }
        
        // Return the fully populated context
        Ok(context)
    }
}

impl<T: PipelineContext> Default for Pipeline<T> {
    fn default() -> Self {
        Self::new()
    }
}

