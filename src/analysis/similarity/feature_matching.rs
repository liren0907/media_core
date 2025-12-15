use opencv::{
    core::{DMatch, KeyPoint, Mat, Vector},
    features2d::{self, Feature2DTrait, ORB},
    prelude::*,
};

use super::analyzer::SimilarityMethodImpl;
use crate::analysis::types::AnalysisError;

pub struct FeatureMatchingMethod {
    orb: opencv::core::Ptr<ORB>,
    matcher: opencv::core::Ptr<features2d::BFMatcher>,
}

impl FeatureMatchingMethod {
    pub fn new() -> Result<Self, AnalysisError> {
        let orb = ORB::create_def()?;
        let matcher = features2d::BFMatcher::create_def()?;

        Ok(Self { orb, matcher })
    }

    fn detect_and_compute(&mut self, img: &Mat) -> Result<(Vector<KeyPoint>, Mat), AnalysisError> {
        let mut gray = Mat::default();
        opencv::imgproc::cvt_color_def(img, &mut gray, opencv::imgproc::COLOR_BGR2GRAY)?;

        let mut keypoints = Vector::new();
        let mut descriptors = Mat::default();
        let mask = Mat::default();

        self.orb
            .detect_and_compute(&gray, &mask, &mut keypoints, &mut descriptors, false)?;

        Ok((keypoints, descriptors))
    }
}

impl SimilarityMethodImpl for FeatureMatchingMethod {
    fn compute_similarity(&mut self, img1: &Mat, img2: &Mat) -> Result<f64, AnalysisError> {
        let (kp1, desc1) = self.detect_and_compute(img1)?;
        let (kp2, desc2) = self.detect_and_compute(img2)?;

        if desc1.empty() || desc2.empty() {
            return Ok(0.0);
        }

        let mut matches: Vector<DMatch> = Vector::new();
        self.matcher
            .train_match(&desc1, &desc2, &mut matches, &Mat::default())?;

        if matches.is_empty() {
            return Ok(0.0);
        }

        // Calculate similarity based on number of good matches
        let total_features = (kp1.len() + kp2.len()) as f64 / 2.0;
        let match_count = matches.len() as f64;

        // Normalize to 0-1
        let similarity = (match_count / total_features).min(1.0);

        Ok(similarity)
    }
}
