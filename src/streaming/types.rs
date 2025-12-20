#[derive(Debug, Clone)]
pub enum SamplingStrategy {
    EveryNth(usize),
    FirstN(usize),
    Range(usize, usize),
    KeyFrames,
    Custom(Vec<usize>),
}

impl SamplingStrategy {
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

pub type StreamResult<T> = Result<T, String>;

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
