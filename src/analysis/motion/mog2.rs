use opencv::{
    core::{Mat, Rect, Vector},
    imgproc,
    prelude::*,
    video::{self, BackgroundSubtractorTrait},
};

use super::detector::MotionAlgorithmImpl;
use crate::analysis::types::AnalysisError;

pub struct Mog2Detector {
    bg_subtractor: opencv::core::Ptr<video::BackgroundSubtractorMOG2>,
}

impl Mog2Detector {
    pub fn new() -> Result<Self, AnalysisError> {
        let bg_subtractor = video::create_background_subtractor_mog2_def()?;
        Ok(Self { bg_subtractor })
    }
}

impl MotionAlgorithmImpl for Mog2Detector {
    fn process_frame(&mut self, frame: &Mat) -> Result<Mat, AnalysisError> {
        let mut mask = Mat::default();
        // The apply method is on the base BackgroundSubtractorTrait
        BackgroundSubtractorTrait::apply(&mut self.bg_subtractor, frame, &mut mask, -1.0)?;

        // Remove shadows (gray pixels become white)
        let mut binary = Mat::default();
        imgproc::threshold(&mask, &mut binary, 200.0, 255.0, imgproc::THRESH_BINARY)?;

        Ok(binary)
    }

    fn detect_motion(&self, mask: &Mat, min_area: i32) -> Result<Vec<Rect>, AnalysisError> {
        let mut mask_clone = mask.try_clone()?;
        let mut contours: Vector<Vector<opencv::core::Point>> = Vector::new();
        imgproc::find_contours_def(
            &mut mask_clone,
            &mut contours,
            imgproc::RETR_EXTERNAL,
            imgproc::CHAIN_APPROX_SIMPLE,
        )?;

        let mut rects = Vec::new();
        for i in 0..contours.len() {
            let contour = contours.get(i)?;
            let area = imgproc::contour_area_def(&contour)?;
            if area > min_area as f64 {
                let rect = imgproc::bounding_rect(&contour)?;
                rects.push(rect);
            }
        }

        Ok(rects)
    }
}
