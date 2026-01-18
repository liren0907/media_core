use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::analysis::types::AnalysisError;

use super::hasher::Hasher;

#[derive(Debug, Clone)]
pub struct ClusteringConfig {
    pub hash_size: i32,
    pub similarity_threshold: f64,
    pub min_group_size: usize,
}

impl Default for ClusteringConfig {
    fn default() -> Self {
        Self {
            hash_size: 8,
            similarity_threshold: 0.95,
            min_group_size: 2,
        }
    }
}

/// PHash-only directory clusterer (analysis_process style).
///
/// - Computes hashes in parallel (does not keep images in RAM)
/// - Returns groups as `PathBuf` lists (no copying / side effects)
pub struct Clusterer {
    config: ClusteringConfig,
    hasher: Hasher,
}

impl Clusterer {
    pub fn new(config: ClusteringConfig) -> Self {
        Self {
            hasher: Hasher::new(config.hash_size),
            config,
        }
    }

    pub fn cluster_directory(
        &self,
        input_dir: &Path,
    ) -> Result<HashMap<String, Vec<PathBuf>>, AnalysisError> {
        let files = self.collect_images(input_dir)?;

        // 1) compute hashes (parallel), dropping Mats immediately
        let entries: Vec<(PathBuf, Option<Vec<u8>>)> = files
            .par_iter()
            .map(|path| {
                let path_str = path.to_string_lossy();
                match self.hasher.compute_phash(&path_str) {
                    Ok(hash) => (path.clone(), Some(hash)),
                    Err(_) => (path.clone(), None),
                }
            })
            .collect();

        let valid_entries: Vec<(PathBuf, Vec<u8>)> = entries
            .into_iter()
            .filter_map(|(p, h)| h.map(|hash| (p, hash)))
            .collect();

        // 2) grouping (O(N^2))
        let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();
        let mut assigned = vec![false; valid_entries.len()];
        let mut group_count = 0;

        for i in 0..valid_entries.len() {
            if assigned[i] {
                continue;
            }

            let (path_i, hash_i) = &valid_entries[i];
            let group_name = format!("group_{:04}", group_count);
            let mut members = vec![path_i.clone()];
            assigned[i] = true;

            for j in (i + 1)..valid_entries.len() {
                if assigned[j] {
                    continue;
                }

                let (path_j, hash_j) = &valid_entries[j];
                let similarity = self.hasher.calculate_similarity(hash_i, hash_j);
                if similarity >= self.config.similarity_threshold {
                    members.push(path_j.clone());
                    assigned[j] = true;
                }
            }

            if members.len() >= self.config.min_group_size {
                groups.insert(group_name, members);
                group_count += 1;
            }
        }

        Ok(groups)
    }

    fn collect_images(&self, dir: &Path) -> Result<Vec<PathBuf>, AnalysisError> {
        let mut files = Vec::new();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg" | "png" | "webp") {
                        files.push(path);
                    }
                }
            }
        }
        Ok(files)
    }
}


