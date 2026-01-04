use opencv::{
    core::{Mat, NORM_MINMAX, Vector},
    imgproc,
};

use super::analyzer::SimilarityMethodImpl;
use crate::analysis::config::HistogramConfig;
use crate::analysis::types::AnalysisError;

/// Histogram-based image similarity using color histogram correlation
pub struct HistogramMethod {
    bins: i32,
}

impl HistogramMethod {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, AnalysisError> {
        Ok(Self {
            bins: HistogramConfig::default().bins,
        })
    }

    /// Create with custom configuration
    pub fn with_config(config: &HistogramConfig) -> Result<Self, AnalysisError> {
        Ok(Self { bins: config.bins })
    }

    /// Calculate normalized histogram for an image
    fn calculate_histogram(&self, image: &Mat) -> Result<Mat, AnalysisError> {
        // Convert to HSV for better color comparison
        let mut hsv = Mat::default();
        imgproc::cvt_color_def(image, &mut hsv, imgproc::COLOR_BGR2HSV)?;

        // Split channels
        let mut channels = Vector::<Mat>::new();
        opencv::core::split(&hsv, &mut channels)?;

        // Calculate histogram for H and S channels (ignore V for lighting invariance)
        let h_channel = channels.get(0)?;
        let s_channel = channels.get(1)?;

        let h_bins = self.bins;
        let s_bins = self.bins;
        let hist_size = [h_bins, s_bins];
        let h_ranges = [0f32, 180f32]; // H range in OpenCV is 0-180
        let s_ranges = [0f32, 256f32];
        let ranges = [h_ranges.as_slice(), s_ranges.as_slice()];
        let channels_arr = [0, 1];

        // Combine H and S channels for 2D histogram
        let mut combined = Vector::<Mat>::new();
        combined.push(h_channel);
        combined.push(s_channel);

        let mut hist = Mat::default();
        imgproc::calc_hist(
            &combined,
            &Vector::from_slice(&channels_arr),
            &Mat::default(),
            &mut hist,
            &Vector::from_slice(&hist_size),
            &Vector::from_slice(&[ranges[0][0], ranges[0][1], ranges[1][0], ranges[1][1]]),
            false,
        )?;

        // Normalize histogram
        let mut normalized_hist = Mat::default();
        opencv::core::normalize(
            &hist,
            &mut normalized_hist,
            0.0,
            1.0,
            NORM_MINMAX,
            -1,
            &Mat::default(),
        )?;

        Ok(normalized_hist)
    }
}

impl SimilarityMethodImpl for HistogramMethod {
    fn compute_similarity(&mut self, img1: &Mat, img2: &Mat) -> Result<f64, AnalysisError> {
        let hist1 = self.calculate_histogram(img1)?;
        let hist2 = self.calculate_histogram(img2)?;

        // Compare histograms using correlation method
        // Returns value between -1 and 1, where 1 means perfect match
        let similarity = imgproc::compare_hist(&hist1, &hist2, imgproc::HISTCMP_CORREL)?;

        // Normalize to 0-1 range (correlation can be negative)
        let normalized = (similarity + 1.0) / 2.0;

        Ok(normalized.max(0.0).min(1.0))
    }
}
