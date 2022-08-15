use crate::{NotFoundException,ResultPoint};
use crate::common::{BitMatrix,BitMatrix};

// MathUtils.java
/**
 * General math-related and numeric utility functions.
 */
pub struct MathUtils {
}

impl MathUtils {

    fn new() -> Self {
        Self{}
    }

    /**
   * Ends up being a bit faster than {@link Math#round(float)}. This merely rounds its
   * argument to the nearest int, where x.5 rounds up to x+1. Semantics of this shortcut
   * differ slightly from {@link Math#round(float)} in that half rounds down for negative
   * values. -2.5 rounds to -3, not -2. For purposes here it makes no difference.
   *
   * @param d real value to round
   * @return nearest {@code int}
   */
    pub fn  round( d: f32) -> i32  {
        return (d + ( if d < 0.0f { -0.5f } else { 0.5f })) as i32;
    }

    /**
   * @param aX point A x coordinate
   * @param aY point A y coordinate
   * @param bX point B x coordinate
   * @param bY point B y coordinate
   * @return Euclidean distance between points A and B
   */
    pub fn  distance( a_x: f32,  a_y: f32,  b_x: f32,  b_y: f32) -> f32  {
         let x_diff: f64 = a_x - b_x;
         let y_diff: f64 = a_y - b_y;
        return Math::sqrt(x_diff * x_diff + y_diff * y_diff) as f32;
    }

    /**
   * @param aX point A x coordinate
   * @param aY point A y coordinate
   * @param bX point B x coordinate
   * @param bY point B y coordinate
   * @return Euclidean distance between points A and B
   */
    pub fn  distance( a_x: i32,  a_y: i32,  b_x: i32,  b_y: i32) -> f32  {
         let x_diff: f64 = a_x - b_x;
         let y_diff: f64 = a_y - b_y;
        return Math::sqrt(x_diff * x_diff + y_diff * y_diff) as f32;
    }

    /**
   * @param array values to sum
   * @return sum of values in array
   */
    pub fn  sum( array: &Vec<i32>) -> i32  {
         let mut count: i32 = 0;
        for   a in array {
            count += a;
        }
        return count;
    }
}

// MonochromeRectangleDetector.java
/**
 * <p>A somewhat generic detector that looks for a barcode-like rectangular region within an image.
 * It looks within a mostly white region of an image for a region of black and white, but mostly
 * black. It returns the four corners of the region, as best it can determine.</p>
 *
 * @author Sean Owen
 * @deprecated without replacement since 3.3.0
 */

const MAX_MODULES: i32 = 32;
#[deprecated]
pub struct MonochromeRectangleDetector {

     image: BitMatrix
}

impl MonochromeRectangleDetector {

    pub fn new( image: &BitMatrix) -> Self {
        Self{ image };
    }

    /**
   * <p>Detects a rectangular region of black and white -- mostly black -- with a region of mostly
   * white, in an image.</p>
   *
   * @return {@link ResultPoint}[] describing the corners of the rectangular region. The first and
   *  last points are opposed on the diagonal, as are the second and third. The first point will be
   *  the topmost point and the last, the bottommost. The second point will be leftmost and the
   *  third, the rightmost
   * @throws NotFoundException if no Data Matrix Code can be found
   */
    pub fn  detect(&self) -> /*  throws NotFoundException */Result<Vec<ResultPoint>, Rc<Exception>>   {
         let height: i32 = self.image.get_height();
         let width: i32 = self.image.get_width();
         let half_height: i32 = height / 2;
         let half_width: i32 = width / 2;
         let delta_y: i32 = Math::max(1, height / (MAX_MODULES * 8));
         let delta_x: i32 = Math::max(1, width / (MAX_MODULES * 8));
         let mut top: i32 = 0;
         let mut bottom: i32 = height;
         let mut left: i32 = 0;
         let mut right: i32 = width;
         let point_a: ResultPoint = self.find_corner_from_center(half_width, 0, left, right, half_height, -delta_y, top, bottom, half_width / 2);
        top = point_a.get_y() as i32 - 1;
         let point_b: ResultPoint = self.find_corner_from_center(half_width, -delta_x, left, right, half_height, 0, top, bottom, half_height / 2);
        left = point_b.get_x() as i32 - 1;
         let point_c: ResultPoint = self.find_corner_from_center(half_width, delta_x, left, right, half_height, 0, top, bottom, half_height / 2);
        right = point_c.get_x() as i32 + 1;
         let point_d: ResultPoint = self.find_corner_from_center(half_width, 0, left, right, half_height, delta_y, top, bottom, half_width / 2);
        bottom = point_d.get_y() as i32 + 1;
        // Go try to find point A again with better information -- might have been off at first.
        point_a = self.find_corner_from_center(half_width, 0, left, right, half_height, -delta_y, top, bottom, half_width / 4);
        return Ok(  vec![point_a, point_b, point_c, point_d, ]);
    }

    /**
   * Attempts to locate a corner of the barcode by scanning up, down, left or right from a center
   * point which should be within the barcode.
   *
   * @param centerX center's x component (horizontal)
   * @param deltaX same as deltaY but change in x per step instead
   * @param left minimum value of x
   * @param right maximum value of x
   * @param centerY center's y component (vertical)
   * @param deltaY change in y per step. If scanning up this is negative; down, positive;
   *  left or right, 0
   * @param top minimum value of y to search through (meaningless when di == 0)
   * @param bottom maximum value of y
   * @param maxWhiteRun maximum run of white pixels that can still be considered to be within
   *  the barcode
   * @return a {@link ResultPoint} encapsulating the corner that was found
   * @throws NotFoundException if such a point cannot be found
   */
    fn  find_corner_from_center(&self,  center_x: i32,  delta_x: i32,  left: i32,  right: i32,  center_y: i32,  delta_y: i32,  top: i32,  bottom: i32,  max_white_run: i32) -> Result<ResultPoint, NotFoundException>   {
         let last_range: Vec<i32> = null;
         {
             let mut y: i32 = center_y;
              let mut x: i32 = center_x;
            while y < bottom && y >= top && x < right && x >= left {
                {
                     let mut range: Vec<i32>;
                    if delta_x == 0 {
                        // horizontal slices, up and down
                        range = self.black_white_range(y, max_white_run, left, right, true);
                    } else {
                        // vertical slices, left and right
                        range = self.black_white_range(x, max_white_run, top, bottom, false);
                    }
                    if range == null {
                        if last_range == null {
                            return Err( NotFoundException::get_not_found_instance());
                        }
                        // lastRange was found
                        if delta_x == 0 {
                             let last_y: i32 = y - delta_y;
                            if last_range[0] < center_x {
                                if last_range[1] > center_x {
                                    // straddle, choose one or the other based on direction
                                    return Ok(ResultPoint::new(last_range[ if delta_y > 0 { 0 } else { 1 }], last_y));
                                }
                                return Ok(ResultPoint::new(last_range[0], last_y));
                            } else {
                                return Ok(ResultPoint::new(last_range[1], last_y));
                            }
                        } else {
                             let last_x: i32 = x - delta_x;
                            if last_range[0] < center_y {
                                if last_range[1] > center_y {
                                    return Ok(ResultPoint::new(last_x, last_range[ if delta_x < 0 { 0 } else { 1 }]));
                                }
                                return Ok(ResultPoint::new(last_x, last_range[0]));
                            } else {
                                return Ok(ResultPoint::new(last_x, last_range[1]));
                            }
                        }
                    }
                    last_range = range;
                }
                y += delta_y;
                x += delta_x;
             }
         }

        return Err( NotFoundException::get_not_found_instance());
    }

    /**
   * Computes the start and end of a region of pixels, either horizontally or vertically, that could
   * be part of a Data Matrix barcode.
   *
   * @param fixedDimension if scanning horizontally, this is the row (the fixed vertical location)
   *  where we are scanning. If scanning vertically it's the column, the fixed horizontal location
   * @param maxWhiteRun largest run of white pixels that can still be considered part of the
   *  barcode region
   * @param minDim minimum pixel location, horizontally or vertically, to consider
   * @param maxDim maximum pixel location, horizontally or vertically, to consider
   * @param horizontal if true, we're scanning left-right, instead of up-down
   * @return int[] with start and end of found range, or null if no such range is found
   *  (e.g. only white was found)
   */
    fn  black_white_range(&self,  fixed_dimension: i32,  max_white_run: i32,  min_dim: i32,  max_dim: i32,  horizontal: bool) -> Vec<i32>  {
         let center: i32 = (min_dim + max_dim) / 2;
        // Scan left/up first
         let mut start: i32 = center;
        while start >= min_dim {
            if  if horizontal { self.image.get(start, fixed_dimension) } else { self.image.get(fixed_dimension, start) } {
                start -= 1;
            } else {
                 let white_run_start: i32 = start;
                loop { {
                    start -= 1;
                }if !(start >= min_dim && !( if horizontal { self.image.get(start, fixed_dimension) } else { self.image.get(fixed_dimension, start) })) {
                    break;
                } }
                 let white_run_size: i32 = white_run_start - start;
                if start < min_dim || white_run_size > max_white_run {
                    start = white_run_start;
                    break;
                }
            }
        }
        start += 1;
        // Then try right/down
         let mut end: i32 = center;
        while end < max_dim {
            if  if horizontal { self.image.get(end, fixed_dimension) } else { self.image.get(fixed_dimension, end) } {
                end += 1;
            } else {
                 let white_run_start: i32 = end;
                loop { {
                    end += 1;
                }if !(end < max_dim && !( if horizontal { self.image.get(end, fixed_dimension) } else { self.image.get(fixed_dimension, end) })) {break;}}
                 let white_run_size: i32 = end - white_run_start;
                if end >= max_dim || white_run_size > max_white_run {
                    end = white_run_start;
                    break;
                }
            }
        }
        end -= 1;
        return  if end > start {   vec![start, end, ]
         } else { null };
    }
}

// WhiteRectangleDetector.java
/**
 * <p>
 * Detects a candidate barcode-like rectangular region within an image. It
 * starts around the center of the image, increases the size of the candidate
 * region until it finds a white rectangular region. By keeping track of the
 * last black points it encountered, it determines the corners of the barcode.
 * </p>
 *
 * @author David Olivier
 */

const INIT_SIZE: i32 = 10;

const CORR: i32 = 1;
pub struct WhiteRectangleDetector {

     image: BitMatrix,

     height: i32,

     width: i32,

     left_init: i32,

     right_init: i32,

     down_init: i32,

     up_init: i32
}

impl WhiteRectangleDetector {

   pub fn new( image: &BitMatrix) -> Result<Self, NotFoundException> {
       this(image, INIT_SIZE, image.get_width() / 2, image.get_height() / 2);
   }

   /**
  * @param image barcode image to find a rectangle in
  * @param initSize initial size of search area around center
  * @param x x position of search center
  * @param y y position of search center
  * @throws NotFoundException if image is too small to accommodate {@code initSize}
  */
   pub fn new( image: &BitMatrix,  init_size: i32,  x: i32,  y: i32) -> Result<Self,NotFoundException>  {
    let mut new_wrd : Self;   
    new_wrd .image = image;
    new_wrd.height = image.get_height();
    new_wrd.width = image.get_width();
        let halfsize: i32 = init_size / 2;
        new_wrd.left_init = x - halfsize;
        new_wrd.right_init = x + halfsize;
        new_wrd.up_init = y - halfsize;
        new_wrd.down_init = y + halfsize;
       if up_init < 0 || left_init < 0 || down_init >= height || right_init >= width {
           return Err( NotFoundException::get_not_found_instance());
       }
       Ok(new_wrd)
   }

   /**
  * <p>
  * Detects a candidate barcode-like rectangular region within an image. It
  * starts around the center of the image, increases the size of the candidate
  * region until it finds a white rectangular region.
  * </p>
  *
  * @return {@link ResultPoint}[] describing the corners of the rectangular
  *         region. The first and last points are opposed on the diagonal, as
  *         are the second and third. The first point will be the topmost
  *         point and the last, the bottommost. The second point will be
  *         leftmost and the third, the rightmost
  * @throws NotFoundException if no Data Matrix Code can be found
  */
   pub fn  detect(&self) -> Result<Vec<ResultPoint>, NotFoundException>   {
        let mut left: i32 = self.left_init;
        let mut right: i32 = self.right_init;
        let mut up: i32 = self.up_init;
        let mut down: i32 = self.down_init;
        let size_exceeded: bool = false;
        let a_black_point_found_on_border: bool = true;
        let at_least_one_black_point_found_on_right: bool = false;
        let at_least_one_black_point_found_on_bottom: bool = false;
        let at_least_one_black_point_found_on_left: bool = false;
        let at_least_one_black_point_found_on_top: bool = false;
       while a_black_point_found_on_border {
           a_black_point_found_on_border = false;
           // .....
           // .   |
           // .....
            let right_border_not_white: bool = true;
           while (right_border_not_white || !at_least_one_black_point_found_on_right) && right < self.width {
               right_border_not_white = self.contains_black_point(up, down, right, false);
               if right_border_not_white {
                   right += 1;
                   a_black_point_found_on_border = true;
                   at_least_one_black_point_found_on_right = true;
               } else if !at_least_one_black_point_found_on_right {
                   right += 1;
               }
           }
           if right >= self.width {
               size_exceeded = true;
               break;
           }
           // .....
           // .   .
           // .___.
            let bottom_border_not_white: bool = true;
           while (bottom_border_not_white || !at_least_one_black_point_found_on_bottom) && down < self.height {
               bottom_border_not_white = self.contains_black_point(left, right, down, true);
               if bottom_border_not_white {
                   down += 1;
                   a_black_point_found_on_border = true;
                   at_least_one_black_point_found_on_bottom = true;
               } else if !at_least_one_black_point_found_on_bottom {
                   down += 1;
               }
           }
           if down >= self.height {
               size_exceeded = true;
               break;
           }
           // .....
           // |   .
           // .....
            let left_border_not_white: bool = true;
           while (left_border_not_white || !at_least_one_black_point_found_on_left) && left >= 0 {
               left_border_not_white = self.contains_black_point(up, down, left, false);
               if left_border_not_white {
                   left -= 1;
                   a_black_point_found_on_border = true;
                   at_least_one_black_point_found_on_left = true;
               } else if !at_least_one_black_point_found_on_left {
                   left -= 1;
               }
           }
           if left < 0 {
               size_exceeded = true;
               break;
           }
           // .___.
           // .   .
           // .....
            let top_border_not_white: bool = true;
           while (top_border_not_white || !at_least_one_black_point_found_on_top) && up >= 0 {
               top_border_not_white = self.contains_black_point(left, right, up, true);
               if top_border_not_white {
                   up -= 1;
                   a_black_point_found_on_border = true;
                   at_least_one_black_point_found_on_top = true;
               } else if !at_least_one_black_point_found_on_top {
                   up -= 1;
               }
           }
           if up < 0 {
               size_exceeded = true;
               break;
           }
       }
       if !size_exceeded {
            let max_size: i32 = right - left;
            let mut z: ResultPoint = null;
            {
                let mut i: i32 = 1;
               while z == null && i < max_size {
                   {
                       z = self.get_black_point_on_segment(left, down - i, left + i, down);
                   }
                   i += 1;
                }
            }

           if z == null {
               return Err( NotFoundException::get_not_found_instance());
           }
            let mut t: ResultPoint = null;
           //go down right
            {
                let mut i: i32 = 1;
               while t == null && i < max_size {
                   {
                       t = self.get_black_point_on_segment(left, up + i, left + i, up);
                   }
                   i += 1;
                }
            }

           if t == null {
               return Err( NotFoundException::get_not_found_instance());
           }
            let mut x: ResultPoint = null;
           //go down left
            {
                let mut i: i32 = 1;
               while x == null && i < max_size {
                   {
                       x = self.get_black_point_on_segment(right, up + i, right - i, up);
                   }
                   i += 1;
                }
            }

           if x == null {
               return Err( NotFoundException::get_not_found_instance());
           }
            let mut y: ResultPoint = null;
           //go up left
            {
                let mut i: i32 = 1;
               while y == null && i < max_size {
                   {
                       y = self.get_black_point_on_segment(right, down - i, right - i, down);
                   }
                   i += 1;
                }
            }

           if y == null {
               return Err( NotFoundException::get_not_found_instance());
           }
           return Ok(self.center_edges(y, z, x, t));
       } else {
           return Err( NotFoundException::get_not_found_instance());
       }
   }

   fn  get_black_point_on_segment(&self,  a_x: f32,  a_y: f32,  b_x: f32,  b_y: f32) -> ResultPoint  {
        let dist: i32 = MathUtils::round(&MathUtils::distance(a_x, a_y, b_x, b_y));
        let x_step: f32 = (b_x - a_x) / dist;
        let y_step: f32 = (b_y - a_y) / dist;
        {
            let mut i: i32 = 0;
           while i < dist {
               {
                    let x: i32 = MathUtils::round(a_x + i * x_step);
                    let y: i32 = MathUtils::round(a_y + i * y_step);
                   if self.image.get(x, y) {
                       return ResultPoint::new(x, y);
                   }
               }
               i += 1;
            }
        }

       return null;
   }

   /**
  * recenters the points of a constant distance towards the center
  *
  * @param y bottom most point
  * @param z left most point
  * @param x right most point
  * @param t top most point
  * @return {@link ResultPoint}[] describing the corners of the rectangular
  *         region. The first and last points are opposed on the diagonal, as
  *         are the second and third. The first point will be the topmost
  *         point and the last, the bottommost. The second point will be
  *         leftmost and the third, the rightmost
  */
   fn  center_edges(&self,  y: &ResultPoint,  z: &ResultPoint,  x: &ResultPoint,  t: &ResultPoint) -> Vec<ResultPoint>  {
       //
       //       t            t
       //  z                      x
       //        x    OR    z
       //   y                    y
       //
        let yi: f32 = y.get_x();
        let yj: f32 = y.get_y();
        let zi: f32 = z.get_x();
        let zj: f32 = z.get_y();
        let xi: f32 = x.get_x();
        let xj: f32 = x.get_y();
        let ti: f32 = t.get_x();
        let tj: f32 = t.get_y();
       if yi < self.width / 2.0f {
           return   vec![ResultPoint::new(ti - CORR, tj + CORR), ResultPoint::new(zi + CORR, zj + CORR), ResultPoint::new(xi - CORR, xj - CORR), ResultPoint::new(yi + CORR, yj - CORR), ]
           ;
       } else {
           return   vec![ResultPoint::new(ti + CORR, tj + CORR), ResultPoint::new(zi + CORR, zj - CORR), ResultPoint::new(xi - CORR, xj + CORR), ResultPoint::new(yi - CORR, yj - CORR), ]
           ;
       }
   }

   /**
  * Determines whether a segment contains a black point
  *
  * @param a          min value of the scanned coordinate
  * @param b          max value of the scanned coordinate
  * @param fixed      value of fixed coordinate
  * @param horizontal set to true if scan must be horizontal, false if vertical
  * @return true if a black point has been found, else false.
  */
   fn  contains_black_point(&self,  a: i32,  b: i32,  fixed: i32,  horizontal: bool) -> bool  {
       if horizontal {
            {
                let mut x: i32 = a;
               while x <= b {
                   {
                       if self.image.get(x, fixed) {
                           return true;
                       }
                   }
                   x += 1;
                }
            }

       } else {
            {
                let mut y: i32 = a;
               while y <= b {
                   {
                       if self.image.get(fixed, y) {
                           return true;
                       }
                   }
                   y += 1;
                }
            }

       }
       return false;
   }
}

