use opencv::{
    core::{Mat, Size},
    imgcodecs, imgproc,
    prelude::*,
};

use crate::analysis::types::AnalysisError;

use super::types::SimilarityResult;

/// Stateless-ish perceptual hashing (aHash) helper.
///
/// - Input: image file path
/// - Output: hash bytes (`Vec<u8>`)
pub struct Hasher {
    hash_size: i32,
}

impl Default for Hasher {
    fn default() -> Self {
        Self { hash_size: 8 }
    }
}

impl Hasher {
    pub fn new(hash_size: i32) -> Self {
        Self { hash_size }
    }

    /// Compute perceptual hash (aHash) from an image file path.
    pub fn compute_phash(&self, path: &str) -> Result<Vec<u8>, AnalysisError> {
        let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)?;
        if img.empty() {
            return Err(AnalysisError::InvalidInput(format!(
                "Failed to load image: {}",
                path
            )));
        }
        self.compute_phash_from_mat(&img)
    }

    /// Internal hashing logic from a `Mat`.
    pub fn compute_phash_from_mat(&self, img: &Mat) -> Result<Vec<u8>, AnalysisError> {
        // 1) grayscale
        let mut gray = Mat::default();
        imgproc::cvt_color_def(img, &mut gray, imgproc::COLOR_BGR2GRAY)?;

        // 2) resize to hash_size x hash_size
        let mut resized = Mat::default();
        imgproc::resize_def(&gray, &mut resized, Size::new(self.hash_size, self.hash_size))?;

        // 3) convert to f64 for mean
        let mut float_img = Mat::default();
        resized.convert_to_def(&mut float_img, opencv::core::CV_64F)?;

        // 4) mean
        let mean = opencv::core::mean_def(&float_img)?;
        let avg = mean[0];

        // 5) bits -> bytes
        let total_pixels = self.hash_size * self.hash_size;
        let num_bytes = ((total_pixels + 7) / 8) as usize;
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

        if bit_count > 0 {
            hash.push(current_byte);
        }

        Ok(hash)
    }

    /// Hamming distance between two hashes.
    pub fn hamming_distance(hash1: &[u8], hash2: &[u8]) -> u32 {
        hash1
            .iter()
            .zip(hash2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum()
    }

    /// Similarity score in [0.0, 1.0].
    pub fn calculate_similarity(&self, hash1: &[u8], hash2: &[u8]) -> f64 {
        let distance = Self::hamming_distance(hash1, hash2);
        let max_distance = (self.hash_size * self.hash_size) as u32;

        if max_distance == 0 {
            return 0.0;
        }

        1.0 - (distance as f64 / max_distance as f64)
    }

    pub fn compare(&self, hash1: &[u8], hash2: &[u8], threshold: f64) -> SimilarityResult {
        let score = self.calculate_similarity(hash1, hash2);
        SimilarityResult {
            is_similar: score >= threshold,
            similarity_score: score,
        }
    }
}


