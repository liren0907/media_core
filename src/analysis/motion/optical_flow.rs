use opencv::{
    core::{Mat, Rect, Vector},
    imgproc,
    prelude::*,
    video,
};

use super::detector::MotionAlgorithmImpl;
use crate::analysis::types::AnalysisError;

pub struct OpticalFlowDetector {
    prev_gray: Option<Mat>,
    flow_threshold: f64,
}

impl OpticalFlowDetector {
    pub fn new() -> Result<Self, AnalysisError> {
        Ok(Self {
            prev_gray: None,
            flow_threshold: 2.0,
        })
    }
}

impl MotionAlgorithmImpl for OpticalFlowDetector {
    fn process_frame(&mut self, frame: &Mat) -> Result<Mat, AnalysisError> {
        let mut gray = Mat::default();
        imgproc::cvt_color_def(frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;

        let mask = if let Some(ref prev) = self.prev_gray {
            let mut flow = Mat::default();
            video::calc_optical_flow_farneback(prev, &gray, &mut flow, 0.5, 3, 15, 3, 5, 1.2, 0)?;

            let mut magnitude = Mat::default();
            let mut angle = Mat::default();

            let mut flow_parts: Vector<Mat> = Vector::new();
            opencv::core::split(&flow, &mut flow_parts)?;

            let flow_x = flow_parts.get(0)?;
            let flow_y = flow_parts.get(1)?;

            opencv::core::cart_to_polar_def(&flow_x, &flow_y, &mut magnitude, &mut angle)?;

            let mut thresh = Mat::default();
            imgproc::threshold(
                &magnitude,
                &mut thresh,
                self.flow_threshold,
                255.0,
                imgproc::THRESH_BINARY,
            )?;

            let mut mask_u8 = Mat::default();
            thresh.convert_to_def(&mut mask_u8, opencv::core::CV_8UC1)?;

            mask_u8
        } else {
            Mat::zeros(gray.rows(), gray.cols(), opencv::core::CV_8UC1)?.to_mat()?
        };

        self.prev_gray = Some(gray);
        Ok(mask)
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
