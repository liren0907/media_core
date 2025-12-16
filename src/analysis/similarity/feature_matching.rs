use opencv::{
    core::{DMatch, KeyPoint, Mat, Vector},
    features2d::{self, Feature2DTrait, ORB, ORB_ScoreType},
    prelude::*,
};

use super::analyzer::SimilarityMethodImpl;
use crate::analysis::config::FeatureMatchingConfig;
use crate::analysis::types::AnalysisError;

/// ORB feature matching-based image similarity
pub struct FeatureMatchingMethod {
    orb: opencv::core::Ptr<ORB>,
    matcher: opencv::core::Ptr<features2d::BFMatcher>,
    good_match_percent: f64,
}

impl FeatureMatchingMethod {
    #[allow(dead_code)]
    pub fn new() -> Result<Self, AnalysisError> {
        Self::with_config(&FeatureMatchingConfig::default())
    }

    /// Create with custom configuration
    pub fn with_config(config: &FeatureMatchingConfig) -> Result<Self, AnalysisError> {
        // Create ORB with configurable max features using the default constructor
        let orb = ORB::create(
            config.max_features,
            1.2, // scale factor
            8,   // nlevels
            31,  // edge threshold
            0,   // first level
            2,   // WTA_K
            ORB_ScoreType::HARRIS_SCORE,
            31, // patch size
            20, // fast threshold
        )?;

        let matcher = features2d::BFMatcher::create(opencv::core::NORM_HAMMING, false)?;

        Ok(Self {
            orb,
            matcher,
            good_match_percent: config.good_match_percent,
        })
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

    /// Filter matches to keep only the best ones based on good_match_percent
    fn filter_good_matches(&self, matches: &Vector<DMatch>) -> Vector<DMatch> {
        if matches.is_empty() {
            return Vector::new();
        }

        // Convert to Vec for sorting
        let mut matches_vec: Vec<DMatch> = matches.iter().collect();

        // Sort by distance (lower is better)
        matches_vec.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Keep top percentage of matches
        let num_good = ((matches_vec.len() as f64) * self.good_match_percent).ceil() as usize;
        let num_good = num_good.max(1); // Keep at least 1 match

        let good_matches: Vector<DMatch> = matches_vec.into_iter().take(num_good).collect();

        good_matches
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

        // Filter to keep only good matches
        let good_matches = self.filter_good_matches(&matches);

        if good_matches.is_empty() {
            return Ok(0.0);
        }

        // Calculate similarity based on number of good matches relative to keypoints
        let total_features = (kp1.len() + kp2.len()) as f64 / 2.0;
        let match_count = good_matches.len() as f64;

        // Normalize to 0-1
        let similarity = (match_count / total_features).min(1.0);

        Ok(similarity)
    }
}
