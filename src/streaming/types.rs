//! Type definitions for streaming module.

#[derive(Debug, Clone)]
pub struct FrameData {
    pub index: usize,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct StreamProgress {
    pub current: usize,
    pub total: usize,
    pub message: String,
}
