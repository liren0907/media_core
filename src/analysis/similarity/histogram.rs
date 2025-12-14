use opencv::{core::Mat, imgproc};

use super::analyzer::SimilarityMethodImpl;
use crate::analysis::types::AnalysisError;

pub struct HistogramMethod;

impl HistogramMethod {
    pub fn new() -> Result<Self, AnalysisError> {
        Ok(Self)
    }
}

impl SimilarityMethodImpl for HistogramMethod {
    fn compute_similarity(&mut self, img1: &Mat, img2: &Mat) -> Result<f64, AnalysisError> {
        // Convert both images to grayscale
        let mut gray1 = Mat::default();
        let mut gray2 = Mat::default();
        imgproc::cvt_color_def(img1, &mut gray1, imgproc::COLOR_BGR2GRAY)?;
        imgproc::cvt_color_def(img2, &mut gray2, imgproc::COLOR_BGR2GRAY)?;

        // Simple pixel-based comparison using mean absolute difference
        let mean1 = opencv::core::mean_def(&gray1)?;
        let mean2 = opencv::core::mean_def(&gray2)?;

        // Calculate similarity based on mean intensity difference
        let diff = (mean1[0] - mean2[0]).abs();
        let similarity = 1.0 - (diff / 255.0);

        Ok(similarity.max(0.0).min(1.0))
    }
}
