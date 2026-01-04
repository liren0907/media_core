pub mod annotator;
pub mod overlay;
pub mod types;

pub use annotator::FrameAnnotator;
pub use overlay::{
    add_text_overlay, add_text_overlay_with_position, annotate_frame, annotate_frame_with_position,
};
pub use types::{
    AnnotationConfig, AnnotationType, DataSource, TextPosition, VideoOutputConfig, format_timestamp,
};
