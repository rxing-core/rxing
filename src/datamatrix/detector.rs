use crate::common::detector::WhiteRectangleDetector;
use crate::common::{BitMatrix, DetectorResult, GridSampler};
use crate::{NotFoundException, ResultPoint};

// Detector.java
/**
 * <p>Encapsulates logic that can detect a Data Matrix Code in an image, even if the Data Matrix Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 */
pub struct Detector {
    image: BitMatrix,

    rectangle_detector: WhiteRectangleDetector,
}

impl Detector {
    pub fn new(image: &BitMatrix) -> Result<Self, NotFoundException> {
        let d: Self;
        d.image = image;
        d.rectangle_detector = WhiteRectangleDetector::new(image, None, None, None);

        Ok(d)
    }

    /**
     * <p>Detects a Data Matrix Code in an image.</p>
     *
     * @return {@link DetectorResult} encapsulating results of detecting a Data Matrix Code
     * @throws NotFoundException if no Data Matrix Code can be found
     */
    pub fn detect(&self) -> Result<DetectorResult, NotFoundException> {
        let corner_points: Vec<ResultPoint> = self.rectangle_detector.detect();
        let mut points: Vec<ResultPoint> = self.detect_solid1(&corner_points);
        points = self.detect_solid2(points?);
        points[3] = self.correct_top_right(points?);
        if points[3] == null {
            return Err(NotFoundException::get_not_found_instance());
        }
        points = self.shift_to_module_center(points?);
        let top_left: ResultPoint = points[0];
        let bottom_left: ResultPoint = points[1];
        let bottom_right: ResultPoint = points[2];
        let top_right: ResultPoint = points[3];
        let dimension_top: i32 = self.transitions_between(&top_left, &top_right) + 1;
        let dimension_right: i32 = self.transitions_between(&bottom_right, &top_right) + 1;
        if (dimension_top & 0x01) == 1 {
            dimension_top += 1;
        }
        if (dimension_right & 0x01) == 1 {
            dimension_right += 1;
        }
        if 4 * dimension_top < 6 * dimension_right && 4 * dimension_right < 6 * dimension_top {
            // The matrix is square
            dimension_top = dimension_right = Math::max(dimension_top, dimension_right);
        }
        let bits: BitMatrix = ::sample_grid(
            self.image,
            top_left,
            bottom_left,
            bottom_right,
            top_right,
            dimension_top,
            dimension_right,
        );
        return Ok(DetectorResult::new(
            bits,
            vec![top_left, bottom_left, bottom_right, top_right],
        ));
    }

    fn shift_point(point: &ResultPoint, to: &ResultPoint, div: i32) -> ResultPoint {
        let x: f32 = (to.get_x() - point.get_x()) / (div + 1);
        let y: f32 = (to.get_y() - point.get_y()) / (div + 1);
        return ResultPoint::new(point.get_x() + x, point.get_y() + y);
    }

    fn move_away(point: &ResultPoint, from_x: f32, from_y: f32) -> ResultPoint {
        let mut x: f32 = point.get_x();
        let mut y: f32 = point.get_y();
        if x < from_x {
            x -= 1.0;
        } else {
            x += 1.0;
        }
        if y < from_y {
            y -= 1.0;
        } else {
            y += 1.0;
        }
        return ResultPoint::new(x, y);
    }

    /**
     * Detect a solid side which has minimum transition.
     */
    fn detect_solid1(&self, corner_points: &Vec<ResultPoint>) -> Vec<ResultPoint> {
        // 0  2
        // 1  3
        let point_a: ResultPoint = corner_points[0];
        let point_b: ResultPoint = corner_points[1];
        let point_c: ResultPoint = corner_points[3];
        let point_d: ResultPoint = corner_points[2];
        let tr_a_b: i32 = self.transitions_between(&point_a, &point_b);
        let tr_b_c: i32 = self.transitions_between(&point_b, &point_c);
        let tr_c_d: i32 = self.transitions_between(&point_c, &point_d);
        let tr_d_a: i32 = self.transitions_between(&point_d, &point_a);
        // 0..3
        // :  :
        // 1--2
        let mut min: i32 = tr_a_b;
        let mut points: vec![Vec<ResultPoint>; 4] = vec![point_d, point_a, point_b, point_c];
        if min > tr_b_c {
            min = tr_b_c;
            points[0] = point_a;
            points[1] = point_b;
            points[2] = point_c;
            points[3] = point_d;
        }
        if min > tr_c_d {
            min = tr_c_d;
            points[0] = point_b;
            points[1] = point_c;
            points[2] = point_d;
            points[3] = point_a;
        }
        if min > tr_d_a {
            points[0] = point_c;
            points[1] = point_d;
            points[2] = point_a;
            points[3] = point_b;
        }
        return points;
    }

    /**
     * Detect a second solid side next to first solid side.
     */
    fn detect_solid2(&self, points: &Vec<ResultPoint>) -> Vec<ResultPoint> {
        // A..D
        // :  :
        // B--C
        let point_a: ResultPoint = points[0];
        let point_b: ResultPoint = points[1];
        let point_c: ResultPoint = points[2];
        let point_d: ResultPoint = points[3];
        // Transition detection on the edge is not stable.
        // To safely detect, shift the points to the module center.
        let tr: i32 = self.transitions_between(&point_a, &point_d);
        let point_bs: ResultPoint = ::shift_point(point_b, point_c, (tr + 1) * 4);
        let point_cs: ResultPoint = ::shift_point(point_c, point_b, (tr + 1) * 4);
        let tr_b_a: i32 = self.transitions_between(&point_bs, &point_a);
        let tr_c_d: i32 = self.transitions_between(&point_cs, &point_d);
        // 1--2
        if tr_b_a < tr_c_d {
            // solid sides: A-B-C
            points[0] = point_a;
            points[1] = point_b;
            points[2] = point_c;
            points[3] = point_d;
        } else {
            // solid sides: B-C-D
            points[0] = point_b;
            points[1] = point_c;
            points[2] = point_d;
            points[3] = point_a;
        }
        return points;
    }

    /**
     * Calculates the corner position of the white top right module.
     */
    fn correct_top_right(&self, points: &Vec<ResultPoint>) -> ResultPoint {
        // A..D
        // |  :
        // B--C
        let point_a: ResultPoint = points[0];
        let point_b: ResultPoint = points[1];
        let point_c: ResultPoint = points[2];
        let point_d: ResultPoint = points[3];
        // shift points for safe transition detection.
        let tr_top: i32 = self.transitions_between(&point_a, &point_d);
        let tr_right: i32 = self.transitions_between(&point_b, &point_d);
        let point_as: ResultPoint = ::shift_point(point_a, point_b, (tr_right + 1) * 4);
        let point_cs: ResultPoint = ::shift_point(point_c, point_b, (tr_top + 1) * 4);
        tr_top = self.transitions_between(&point_as, &point_d);
        tr_right = self.transitions_between(&point_cs, &point_d);
        let candidate1: ResultPoint = ResultPoint::new(
            point_d.get_x() + (point_c.get_x() - point_b.get_x()) / (tr_top + 1),
            point_d.get_y() + (point_c.get_y() - point_b.get_y()) / (tr_top + 1),
        );
        let candidate2: ResultPoint = ResultPoint::new(
            point_d.get_x() + (point_a.get_x() - point_b.get_x()) / (tr_right + 1),
            point_d.get_y() + (point_a.get_y() - point_b.get_y()) / (tr_right + 1),
        );
        if !self.is_valid(&candidate1) {
            if self.is_valid(&candidate2) {
                return candidate2;
            }
            return null;
        }
        if !self.is_valid(&candidate2) {
            return candidate1;
        }
        let sumc1: i32 = self.transitions_between(&point_as, &candidate1)
            + self.transitions_between(&point_cs, &candidate1);
        let sumc2: i32 = self.transitions_between(&point_as, &candidate2)
            + self.transitions_between(&point_cs, &candidate2);
        if sumc1 > sumc2 {
            return candidate1;
        } else {
            return candidate2;
        }
    }

    /**
     * Shift the edge points to the module center.
     */
    fn shift_to_module_center(&self, points: &Vec<ResultPoint>) -> Vec<ResultPoint> {
        // A..D
        // |  :
        // B--C
        let point_a: ResultPoint = points[0];
        let point_b: ResultPoint = points[1];
        let point_c: ResultPoint = points[2];
        let point_d: ResultPoint = points[3];
        // calculate pseudo dimensions
        let dim_h: i32 = self.transitions_between(&point_a, &point_d) + 1;
        let dim_v: i32 = self.transitions_between(&point_c, &point_d) + 1;
        // shift points for safe dimension detection
        let point_as: ResultPoint = ::shift_point(point_a, point_b, dim_v * 4);
        let point_cs: ResultPoint = ::shift_point(point_c, point_b, dim_h * 4);
        //  calculate more precise dimensions
        dim_h = self.transitions_between(&point_as, &point_d) + 1;
        dim_v = self.transitions_between(&point_cs, &point_d) + 1;
        if (dim_h & 0x01) == 1 {
            dim_h += 1;
        }
        if (dim_v & 0x01) == 1 {
            dim_v += 1;
        }
        // WhiteRectangleDetector returns points inside of the rectangle.
        // I want points on the edges.
        let center_x: f32 =
            (point_a.get_x() + point_b.get_x() + point_c.get_x() + point_d.get_x()) / 4;
        let center_y: f32 =
            (point_a.get_y() + point_b.get_y() + point_c.get_y() + point_d.get_y()) / 4;
        point_a = ::move_away(point_a, center_x, center_y);
        point_b = ::move_away(point_b, center_x, center_y);
        point_c = ::move_away(point_c, center_x, center_y);
        point_d = ::move_away(point_d, center_x, center_y);
        let point_bs: ResultPoint;
        let point_ds: ResultPoint;
        // shift points to the center of each modules
        point_as = ::shift_point(point_a, point_b, dim_v * 4);
        point_as = ::shift_point(point_as, point_d, dim_h * 4);
        point_bs = ::shift_point(point_b, point_a, dim_v * 4);
        point_bs = ::shift_point(point_bs, point_c, dim_h * 4);
        point_cs = ::shift_point(point_c, point_d, dim_v * 4);
        point_cs = ::shift_point(point_cs, point_b, dim_h * 4);
        point_ds = ::shift_point(point_d, point_c, dim_v * 4);
        point_ds = ::shift_point(point_ds, point_a, dim_h * 4);
        return vec![point_as, point_bs, point_cs, point_ds];
    }

    fn is_valid(&self, p: &ResultPoint) -> bool {
        return p.get_x() >= 0
            && p.get_x() <= self.image.get_width() - 1
            && p.get_y() > 0
            && p.get_y() <= self.image.get_height() - 1;
    }

    fn sample_grid(
        image: &BitMatrix,
        top_left: &ResultPoint,
        bottom_left: &ResultPoint,
        bottom_right: &ResultPoint,
        top_right: &ResultPoint,
        dimension_x: i32,
        dimension_y: i32,
    ) -> Result<BitMatrix, Rc<Exception>> {
        let sampler: GridSampler = GridSampler::get_instance();
        return Ok(sampler.sample_grid(
            image,
            dimension_x,
            dimension_y,
            0.5f32,
            0.5f32,
            dimension_x - 0.5f32,
            0.5f32,
            dimension_x - 0.5f32,
            dimension_y - 0.5f32,
            0.5f32,
            dimension_y - 0.5f32,
            &top_left.get_x(),
            &top_left.get_y(),
            &top_right.get_x(),
            &top_right.get_y(),
            &bottom_right.get_x(),
            &bottom_right.get_y(),
            &bottom_left.get_x(),
            &bottom_left.get_y(),
        ));
    }

    /**
     * Counts the number of black/white transitions between two points, using something like Bresenham's algorithm.
     */
    fn transitions_between(&self, from: &ResultPoint, to: &ResultPoint) -> i32 {
        // See QR Code Detector, sizeOfBlackWhiteBlackRun()
        let from_x: i32 = from.get_x() as i32;
        let from_y: i32 = from.get_y() as i32;
        let to_x: i32 = to.get_x() as i32;
        let to_y: i32 = Math::min(self.image.get_height() - 1, to.get_y() as i32);
        let steep: bool = Math::abs(to_y - from_y) > Math::abs(to_x - from_x);
        if steep {
            let mut temp: i32 = from_x;
            from_x = from_y;
            from_y = temp;
            temp = to_x;
            to_x = to_y;
            to_y = temp;
        }
        let dx: i32 = Math::abs(to_x - from_x);
        let dy: i32 = Math::abs(to_y - from_y);
        let mut error: i32 = -dx / 2;
        let ystep: i32 = if from_y < to_y { 1 } else { -1 };
        let xstep: i32 = if from_x < to_x { 1 } else { -1 };
        let mut transitions: i32 = 0;
        let in_black: bool = self.image.get(
            if steep { from_y } else { from_x },
            if steep { from_x } else { from_y },
        );
        {
            let mut x: i32 = from_x;
            let mut y: i32 = from_y;
            while x != to_x {
                {
                    let is_black: bool = self
                        .image
                        .get(if steep { y } else { x }, if steep { x } else { y });
                    if is_black != in_black {
                        transitions += 1;
                        in_black = is_black;
                    }
                    error += dy;
                    if error > 0 {
                        if y == to_y {
                            break;
                        }
                        y += ystep;
                        error -= dx;
                    }
                }
                x += xstep;
            }
        }

        return transitions;
    }
}
