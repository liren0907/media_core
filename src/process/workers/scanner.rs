use crate::process::config::VideoExtractionConfig;
use crate::process::types::ProcessError;
use path_clean::PathClean;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct VideoScanner;

impl VideoScanner {
    /// scan directories for video files
    pub fn scan(
        video_config: &VideoExtractionConfig,
    ) -> Result<HashMap<String, Vec<PathBuf>>, ProcessError> {
        let mut video_files_by_dir: HashMap<String, Vec<PathBuf>> = HashMap::new();

        for dir_path_str in &video_config.input_directories {
            let path = Path::new(dir_path_str);

            if path.is_file() {
                // Handle single file input
                if matches!(
                    path.extension().and_then(|s| s.to_str()),
                    Some("mp4" | "mov" | "avi" | "mkv")
                ) {
                    let parent = path.parent().unwrap_or_else(|| Path::new("."));
                    let parent_str = parent.to_string_lossy().to_string();

                    video_files_by_dir
                        .entry(parent_str)
                        .or_insert_with(Vec::new)
                        .push(path.clean());
                } else {
                    eprintln!(
                        "Warning: Input file {} is not a supported video format",
                        path.display()
                    );
                }
            } else if path.is_dir() {
                // Handle directory input
                let dir_path = path;
                let video_files: Vec<PathBuf> = fs::read_dir(dir_path)
                    .map_err(|e| {
                        ProcessError::IoError(format!(
                            "Failed to read directory {}: {}",
                            dir_path.display(),
                            e
                        ))
                    })?
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| {
                        let path = entry.path();
                        path.is_file()
                            && matches!(
                                path.extension().and_then(|s| s.to_str()),
                                Some("mp4" | "mov" | "avi" | "mkv")
                            )
                    })
                    .map(|entry| entry.path().clean())
                    .collect();

                if !video_files.is_empty() {
                    video_files_by_dir.insert(dir_path_str.to_string(), video_files);
                }
            } else {
                eprintln!(
                    "Warning: Input path does not exist or is inaccessible: {}",
                    path.display()
                );
            }
        }

        Ok(video_files_by_dir)
    }
}
