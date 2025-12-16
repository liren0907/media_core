use opencv::{
    core::{Mat, Size},
    imgproc,
    prelude::*,
};

use super::analyzer::SimilarityMethodImpl;
use crate::analysis::config::PerceptualHashConfig;
use crate::analysis::types::AnalysisError;

/// Perceptual hash (aHash) based image similarity
pub struct PerceptualHashMethod {
    hash_size: i32,
}

impl PerceptualHashMethod {
    #[allow(dead_code)]
    pub fn new(_resize_width: i32, _resize_height: i32) -> Result<Self, AnalysisError> {
        Ok(Self {
            hash_size: PerceptualHashConfig::default().hash_size,
        })
    }

    /// Create with custom configuration
    pub fn with_config(config: &PerceptualHashConfig) -> Result<Self, AnalysisError> {
        Ok(Self {
            hash_size: config.hash_size,
        })
    }

    fn compute_phash(&self, img: &Mat) -> Result<Vec<u8>, AnalysisError> {
        // Convert to grayscale
        let mut gray = Mat::default();
        imgproc::cvt_color_def(img, &mut gray, imgproc::COLOR_BGR2GRAY)?;

        // Resize to hash_size x hash_size
        let mut resized = Mat::default();
        imgproc::resize_def(
            &gray,
            &mut resized,
            Size::new(self.hash_size, self.hash_size),
        )?;

        // Convert to f64 for mean calculation
        let mut float_img = Mat::default();
        resized.convert_to_def(&mut float_img, opencv::core::CV_64F)?;

        // Calculate mean
        let mean = opencv::core::mean_def(&float_img)?;
        let avg = mean[0];

        // Generate hash: each bit is 1 if pixel > mean, 0 otherwise
        let total_bits = self.hash_size * self.hash_size;
        let num_bytes = ((total_bits + 7) / 8) as usize;
        let mut hash = Vec::with_capacity(num_bytes);
        let mut current_byte: u8 = 0;
        let mut bit_count = 0;

        for row in 0..self.hash_size {
            for col in 0..self.hash_size {
                let pixel = *float_img.at_2d::<f64>(row, col)?;
                if pixel > avg {
                    current_byte |= 1 << (7 - bit_count);
                }
                bit_count += 1;
                if bit_count == 8 {
                    hash.push(current_byte);
                    current_byte = 0;
                    bit_count = 0;
                }
            }
        }

        // Push remaining bits if any
        if bit_count > 0 {
            hash.push(current_byte);
        }

        Ok(hash)
    }

    fn hamming_distance(&self, hash1: &[u8], hash2: &[u8]) -> u32 {
        hash1
            .iter()
            .zip(hash2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum()
    }
}

impl SimilarityMethodImpl for PerceptualHashMethod {
    fn compute_similarity(&mut self, img1: &Mat, img2: &Mat) -> Result<f64, AnalysisError> {
        let hash1 = self.compute_phash(img1)?;
        let hash2 = self.compute_phash(img2)?;

        let distance = self.hamming_distance(&hash1, &hash2);
        let max_distance = (self.hash_size * self.hash_size) as u32;

        // Convert distance to similarity (0-1, where 1 is identical)
        let similarity = 1.0 - (distance as f64 / max_distance as f64);

        Ok(similarity)
    }
}
