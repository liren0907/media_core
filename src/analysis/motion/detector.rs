use opencv::{
    core::{Mat, Rect},
    prelude::*,
    videoio::{CAP_PROP_FPS, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH, VideoCapture},
};
use std::fs;
use std::path::Path;

use crate::analysis::config::{MotionAlgorithm, MotionConfig, RegionOfInterest};
use crate::analysis::types::AnalysisError;

use super::frame_diff::FrameDiffDetector;
use super::knn::KnnDetector;
use super::mog2::Mog2Detector;
use super::optical_flow::OpticalFlowDetector;

pub trait MotionAlgorithmImpl {
    // Renamed from apply() to avoid collision with OpenCV's BackgroundSubtractor::apply()
    fn process_frame(&mut self, frame: &Mat) -> Result<Mat, AnalysisError>;
    fn detect_motion(&self, mask: &Mat, min_area: i32) -> Result<Vec<Rect>, AnalysisError>;
}

pub struct MotionDetector {
    config: MotionConfig,
    algorithm: Box<dyn MotionAlgorithmImpl>,
}

impl MotionDetector {
    pub fn new(config: MotionConfig) -> Result<Self, AnalysisError> {
        let algorithm: Box<dyn MotionAlgorithmImpl> = match config.algorithm {
            MotionAlgorithm::FrameDiff => Box::new(FrameDiffDetector::new(config.threshold)?),
            MotionAlgorithm::Mog2 => Box::new(Mog2Detector::new()?),
            MotionAlgorithm::Knn => Box::new(KnnDetector::new()?),
            MotionAlgorithm::OpticalFlow => Box::new(OpticalFlowDetector::new()?),
        };

        Ok(Self { config, algorithm })
    }

    pub fn process_video(
        &mut self,
        input_path: &Path,
        output_dir: &Path,
    ) -> Result<Vec<(i32, i32)>, AnalysisError> {
        fs::create_dir_all(output_dir)?;

        let mut cap =
            VideoCapture::from_file(input_path.to_str().unwrap(), opencv::videoio::CAP_ANY)?;

        let fps = cap.get(CAP_PROP_FPS)? as f64;
        let width = cap.get(CAP_PROP_FRAME_WIDTH)? as i32;
        let height = cap.get(CAP_PROP_FRAME_HEIGHT)? as i32;

        let mut motion_segments: Vec<(i32, i32)> = Vec::new();
        let mut frame_count = 0;
        let mut motion_start: Option<i32> = None;
        let mut frames_without_motion = 0;
        let motion_gap_threshold = (fps * 2.0) as i32;

        println!("🎬 Processing video for motion detection...");
        println!("   Resolution: {}x{}, FPS: {:.2}", width, height, fps);

        loop {
            let mut frame = Mat::default();
            if !cap.read(&mut frame)? || frame.empty() {
                break;
            }

            if frame_count % (self.config.frame_skip + 1) != 0 {
                frame_count += 1;
                continue;
            }

            let roi_frame = if let Some(ref roi) = self.config.roi {
                self.apply_roi(&frame, roi)?
            } else {
                frame.try_clone()?
            };

            let mask = self.algorithm.process_frame(&roi_frame)?;
            let motion_rects = self.algorithm.detect_motion(&mask, self.config.min_area)?;

            if !motion_rects.is_empty() {
                if motion_start.is_none() {
                    motion_start = Some(frame_count);
                }
                frames_without_motion = 0;
            } else {
                frames_without_motion += 1;
                if let Some(start) = motion_start {
                    if frames_without_motion > motion_gap_threshold {
                        motion_segments.push((start, frame_count - frames_without_motion));
                        motion_start = None;
                    }
                }
            }

            frame_count += 1;
        }

        if let Some(start) = motion_start {
            motion_segments.push((start, frame_count));
        }

        println!("✅ Motion detection complete!");
        println!("   Total frames: {}", frame_count);
        println!("   Motion segments found: {}", motion_segments.len());

        Ok(motion_segments)
    }

    fn apply_roi(&self, frame: &Mat, roi: &RegionOfInterest) -> Result<Mat, AnalysisError> {
        let rect = Rect::new(roi.x, roi.y, roi.width, roi.height);
        let roi_mat = Mat::roi(frame, rect)?;
        Ok(roi_mat.try_clone()?)
    }
}
