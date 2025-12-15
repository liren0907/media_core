use opencv::{
    core::{Mat, Rect, Vector, absdiff},
    imgproc,
    prelude::*,
};

use super::detector::MotionAlgorithmImpl;
use crate::analysis::types::AnalysisError;

pub struct FrameDiffDetector {
    prev_frame: Option<Mat>,
    threshold: f64,
}

impl FrameDiffDetector {
    pub fn new(threshold: f64) -> Result<Self, AnalysisError> {
        Ok(Self {
            prev_frame: None,
            threshold,
        })
    }
}

impl MotionAlgorithmImpl for FrameDiffDetector {
    fn process_frame(&mut self, frame: &Mat) -> Result<Mat, AnalysisError> {
        let mut gray = Mat::default();
        imgproc::cvt_color_def(frame, &mut gray, imgproc::COLOR_BGR2GRAY)?;

        let mask = if let Some(ref prev) = self.prev_frame {
            let mut diff = Mat::default();
            absdiff(prev, &gray, &mut diff)?;

            let mut thresh = Mat::default();
            imgproc::threshold(
                &diff,
                &mut thresh,
                self.threshold,
                255.0,
                imgproc::THRESH_BINARY,
            )?;

            let kernel = imgproc::get_structuring_element_def(
                imgproc::MORPH_ELLIPSE,
                opencv::core::Size::new(5, 5),
            )?;

            let mut cleaned = Mat::default();
            imgproc::dilate_def(&thresh, &mut cleaned, &kernel)?;

            cleaned
        } else {
            Mat::zeros(gray.rows(), gray.cols(), opencv::core::CV_8UC1)?.to_mat()?
        };

        self.prev_frame = Some(gray);
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
