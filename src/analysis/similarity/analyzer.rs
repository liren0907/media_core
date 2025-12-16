use opencv::{core::Mat, imgcodecs, prelude::*};
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::analysis::config::{ProcessMode, SimilarityConfig, SimilarityMethod};
use crate::analysis::types::AnalysisError;

use super::feature_matching::FeatureMatchingMethod;
use super::histogram::HistogramMethod;
use super::perceptual_hash::PerceptualHashMethod;

pub trait SimilarityMethodImpl {
    fn compute_similarity(&mut self, img1: &Mat, img2: &Mat) -> Result<f64, AnalysisError>;
}

/// Processing statistics for similarity analysis
#[derive(Debug, Clone, Default)]
pub struct ProcessingStats {
    pub total_images: usize,
    pub total_comparisons: usize,
    pub total_groups: usize,
    pub processing_time_secs: f64,
    pub images_per_second: f64,
}

impl ProcessingStats {
    pub fn print_summary(&self) {
        println!("📊 Processing Statistics:");
        println!("   Total images: {}", self.total_images);
        println!("   Total comparisons: {}", self.total_comparisons);
        println!("   Total groups: {}", self.total_groups);
        println!("   Processing time: {:.2}s", self.processing_time_secs);
        println!("   Speed: {:.2} images/sec", self.images_per_second);
    }
}

pub struct SimilarityAnalyzer {
    config: SimilarityConfig,
    method: Box<dyn SimilarityMethodImpl>,
    stats: ProcessingStats,
}

impl SimilarityAnalyzer {
    pub fn new(config: SimilarityConfig) -> Result<Self, AnalysisError> {
        let method: Box<dyn SimilarityMethodImpl> = match config.method {
            SimilarityMethod::Histogram => {
                Box::new(HistogramMethod::with_config(&config.histogram)?)
            }
            SimilarityMethod::FeatureMatching => Box::new(FeatureMatchingMethod::with_config(
                &config.feature_matching,
            )?),
            SimilarityMethod::PerceptualHash => {
                Box::new(PerceptualHashMethod::with_config(&config.perceptual_hash)?)
            }
        };

        Ok(Self {
            config,
            method,
            stats: ProcessingStats::default(),
        })
    }

    /// Get the current processing statistics
    pub fn get_stats(&self) -> &ProcessingStats {
        &self.stats
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

    /// Collect image files from a directory
    fn collect_image_files(&self, input_dir: &Path) -> Result<Vec<PathBuf>, AnalysisError> {
        let mut image_files: Vec<_> = fs::read_dir(input_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| {
                        let ext = ext.to_str().unwrap_or("").to_lowercase();
                        ext == "jpg"
                            || ext == "jpeg"
                            || ext == "png"
                            || ext == "bmp"
                            || ext == "webp"
                    })
                    .unwrap_or(false)
            })
            .map(|e| e.path())
            .collect();

        image_files.sort();
        Ok(image_files)
    }

    /// Load images sequentially
    fn load_images_sequential(&self, image_files: &[PathBuf]) -> Vec<Result<Mat, opencv::Error>> {
        image_files
            .iter()
            .map(|p| imgcodecs::imread(p.to_str().unwrap(), imgcodecs::IMREAD_COLOR))
            .collect()
    }

    /// Load images in parallel using rayon
    fn load_images_parallel(&self, image_files: &[PathBuf]) -> Vec<Result<Mat, opencv::Error>> {
        image_files
            .par_iter()
            .map(|p| imgcodecs::imread(p.to_str().unwrap(), imgcodecs::IMREAD_COLOR))
            .collect()
    }

    pub fn group_similar_images(
        &mut self,
        input_dir: &Path,
        output_dir: &Path,
    ) -> Result<HashMap<String, Vec<String>>, AnalysisError> {
        let start_time = Instant::now();

        fs::create_dir_all(output_dir)?;

        // Collect image files
        let image_files = self.collect_image_files(input_dir)?;

        println!(
            "🖼️  Analyzing {} images for similarity...",
            image_files.len()
        );
        println!("   Method: {:?}", self.config.method);
        println!("   Processing mode: {:?}", self.config.process_mode);
        println!("   Threshold: {:.2}", self.config.get_effective_threshold());

        // Load images based on processing mode
        let images = if self.config.process_mode == ProcessMode::Parallel {
            println!(
                "   Loading images in parallel ({} workers)...",
                self.config.parallel_num
            );
            self.load_images_parallel(&image_files)
        } else {
            println!("   Loading images sequentially...");
            self.load_images_sequential(&image_files)
        };

        let threshold = self.config.get_effective_threshold();
        let mut groups: HashMap<String, Vec<String>> = HashMap::new();
        let mut assigned: Vec<bool> = vec![false; image_files.len()];
        let mut group_count = 0;
        let mut total_comparisons = 0;

        // Grouping phase
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
                    total_comparisons += 1;
                    match self.method.compute_similarity(img1, img2) {
                        Ok(similarity) => {
                            if similarity >= threshold {
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

            // Apply min_category_size filter
            if (group_members.len() as i32) < self.config.min_category_size {
                // Skip groups smaller than minimum size
                continue;
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

        let elapsed = start_time.elapsed().as_secs_f64();

        // Update statistics
        self.stats = ProcessingStats {
            total_images: image_files.len(),
            total_comparisons,
            total_groups: group_count,
            processing_time_secs: elapsed,
            images_per_second: image_files.len() as f64 / elapsed,
        };

        println!("✅ Grouping complete!");
        self.stats.print_summary();

        Ok(groups)
    }
}
