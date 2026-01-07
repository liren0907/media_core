//! Core Pipeline Infrastructure
//! 
//! This module provides the generic building blocks for the Pipeline Builder Pattern.
//! It is designed to be agnostic of the specific domain logic (metadata, analysis, etc.).

pub mod error;
pub mod traits;
pub mod builder;
pub mod context;
pub mod types;

pub use error::PipelineError;
pub use traits::{PipelineContext, PipelineStep};
pub use builder::Pipeline;
pub use context::MediaContext;
pub use types::MediaSource;
