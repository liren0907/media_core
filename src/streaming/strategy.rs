//! Sampling strategy definitions and implementations for frame extraction.

/// Defines various strategies for sampling frames from a video stream.
#[derive(Debug, Clone)]
pub enum SamplingStrategy {
    /// Sample every Nth frame
    EveryNth(usize),
    /// Sample only the first N frames
    FirstN(usize),
    /// Sample frames within a specific range [start, end)
    Range(usize, usize),
    /// Sample key frames (every 30th frame)
    KeyFrames,
    /// Sample specific frame indices
    Custom(Vec<usize>),
}

impl SamplingStrategy {
    /// Calculate the frame indices to extract based on the sampling strategy.
    pub fn get_frame_indices(&self, total_frames: usize) -> Vec<usize> {
        match self {
            SamplingStrategy::EveryNth(n) => (0..total_frames).step_by(*n.max(&1)).collect(),
            SamplingStrategy::FirstN(n) => (0..*n.min(&total_frames)).collect(),
            SamplingStrategy::Range(start, end) => {
                let start = *start.min(&total_frames);
                let end = *end.min(&total_frames);
                (start..end).collect()
            }
            SamplingStrategy::KeyFrames => (0..total_frames).step_by(30).collect(),
            SamplingStrategy::Custom(indices) => indices
                .iter()
                .filter(|&&i| i < total_frames)
                .copied()
                .collect(),
        }
    }
}
