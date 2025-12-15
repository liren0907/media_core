use opencv::{core::Mat, imgcodecs, prelude::*};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::analysis::config::{SimilarityConfig, SimilarityMethod};
use crate::analysis::types::AnalysisError;

use super::feature_matching::FeatureMatchingMethod;
use super::histogram::HistogramMethod;
use super::perceptual_hash::PerceptualHashMethod;

pub trait SimilarityMethodImpl {
    fn compute_similarity(&mut self, img1: &Mat, img2: &Mat) -> Result<f64, AnalysisError>;
}

pub struct SimilarityAnalyzer {
    config: SimilarityConfig,
    method: Box<dyn SimilarityMethodImpl>,
}

impl SimilarityAnalyzer {
    pub fn new(config: SimilarityConfig) -> Result<Self, AnalysisError> {
        let method: Box<dyn SimilarityMethodImpl> = match config.method {
            SimilarityMethod::Histogram => Box::new(HistogramMethod::new()?),
            SimilarityMethod::FeatureMatching => Box::new(FeatureMatchingMethod::new()?),
            SimilarityMethod::PerceptualHash => Box::new(PerceptualHashMethod::new(
                config.resize_width,
                config.resize_height,
            )?),
        };

        Ok(Self { config, method })
    }

    pub fn compare_images(
        &mut self,
        img1_path: &Path,
        img2_path: &Path,
    ) -> Result<f64, AnalysisError> {
        let img1 = imgcodecs::imread(img1_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
        let img2 = imgcodecs::imread(img2_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;

        if img1.empty() || img2.empty() {
            return Err(AnalysisError::InvalidInput(
                "Failed to load images".to_string(),
            ));
        }

        self.method.compute_similarity(&img1, &img2)
    }

    pub fn group_similar_images(
        &mut self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> Result<HashMap<String, Vec<String>>, AnalysisError> {
        fs::create_dir_all(output_dir)?;

        // Collect image files
        let mut image_files: Vec<_> = fs::read_dir(input_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| {
                        let ext = ext.to_str().unwrap_or("").to_lowercase();
                        ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "bmp"
                    })
                    .unwrap_or(false)
            })
            .map(|e| e.path())
            .collect();

        image_files.sort();

        println!(
            "🖼️  Analyzing {} images for similarity...",
            image_files.len()
        );

        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        let mut assigned: Vec<bool> = vec![false; image_files.len()];
        let mut group_count = 0;

        // Load all images first
        let images: Vec<_> = image_files
            .iter()
            .map(|p| imgcodecs::imread(p.to_str().unwrap(), imgcodecs::IMREAD_COLOR))
            .collect();

        for i in 0..image_files.len() {
            if assigned[i] {
                continue;
            }

            let group_name = format!("group_{:03}", group_count);
            let mut group_members = vec![image_files[i].to_string_lossy().to_string()];
            assigned[i] = true;

            for j in (i + 1)..image_files.len() {
                if assigned[j] {
                    continue;
                }

                if let (Ok(img1), Ok(img2)) = (&images[i], &images[j]) {
                    match self.method.compute_similarity(img1, img2) {
                        Ok(similarity) => {
                            if similarity >= self.config.threshold {
                                group_members.push(image_files[j].to_string_lossy().to_string());
                                assigned[j] = true;
                            }
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to compare images: {}", e);
                        }
                    }
                }
            }

            if self.config.group_similar {
                // Create group directory and copy files
                let group_dir = output_dir.join(&group_name);
                fs::create_dir_all(&group_dir)?;

                for file_path in &group_members {
                    let src = Path::new(file_path);
                    if let Some(filename) = src.file_name() {
                        let dst = group_dir.join(filename);
                        let _ = fs::copy(src, dst);
                    }
                }
            }

            groups.insert(group_name, group_members);
            group_count += 1;
        }

        println!("✅ Grouping complete!");
        println!("   Total groups: {}", group_count);

        Ok(groups)
    }
}
