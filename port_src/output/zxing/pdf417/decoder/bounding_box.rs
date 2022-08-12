/*
 * Copyright 2013 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::google::zxing::pdf417::decoder;

/**
 * @author Guenther Grau
 */
struct BoundingBox {

     let mut image: BitMatrix;

     let top_left: ResultPoint;

     let bottom_left: ResultPoint;

     let top_right: ResultPoint;

     let bottom_right: ResultPoint;

     let min_x: i32;

     let max_x: i32;

     let min_y: i32;

     let max_y: i32;
}

impl BoundingBox {

    fn new( image: &BitMatrix,  top_left: &ResultPoint,  bottom_left: &ResultPoint,  top_right: &ResultPoint,  bottom_right: &ResultPoint) -> BoundingBox throws NotFoundException {
         let left_unspecified: bool = top_left == null || bottom_left == null;
         let right_unspecified: bool = top_right == null || bottom_right == null;
        if left_unspecified && right_unspecified {
            throw NotFoundException::get_not_found_instance();
        }
        if left_unspecified {
            top_left = ResultPoint::new(0, &top_right.get_y());
            bottom_left = ResultPoint::new(0, &bottom_right.get_y());
        } else if right_unspecified {
            top_right = ResultPoint::new(image.get_width() - 1, &top_left.get_y());
            bottom_right = ResultPoint::new(image.get_width() - 1, &bottom_left.get_y());
        }
        let .image = image;
        let .topLeft = top_left;
        let .bottomLeft = bottom_left;
        let .topRight = top_right;
        let .bottomRight = bottom_right;
        let .minX = Math::min(&top_left.get_x(), &bottom_left.get_x()) as i32;
        let .maxX = Math::max(&top_right.get_x(), &bottom_right.get_x()) as i32;
        let .minY = Math::min(&top_left.get_y(), &top_right.get_y()) as i32;
        let .maxY = Math::max(&bottom_left.get_y(), &bottom_right.get_y()) as i32;
    }

    fn new( bounding_box: &BoundingBox) -> BoundingBox {
        let .image = bounding_box.image;
        let .topLeft = bounding_box.topLeft;
        let .bottomLeft = bounding_box.bottomLeft;
        let .topRight = bounding_box.topRight;
        let .bottomRight = bounding_box.bottomRight;
        let .minX = bounding_box.minX;
        let .maxX = bounding_box.maxX;
        let .minY = bounding_box.minY;
        let .maxY = bounding_box.maxY;
    }

    fn  merge( left_box: &BoundingBox,  right_box: &BoundingBox) -> /*  throws NotFoundException */Result<BoundingBox, Rc<Exception>>   {
        if left_box == null {
            return Ok(right_box);
        }
        if right_box == null {
            return Ok(left_box);
        }
        return Ok(BoundingBox::new(left_box.image, left_box.topLeft, left_box.bottomLeft, right_box.topRight, right_box.bottomRight));
    }

    fn  add_missing_rows(&self,  missing_start_rows: i32,  missing_end_rows: i32,  is_left: bool) -> /*  throws NotFoundException */Result<BoundingBox, Rc<Exception>>   {
         let new_top_left: ResultPoint = self.top_left;
         let new_bottom_left: ResultPoint = self.bottom_left;
         let new_top_right: ResultPoint = self.top_right;
         let new_bottom_right: ResultPoint = self.bottom_right;
        if missing_start_rows > 0 {
             let top: ResultPoint =  if is_left { self.top_left } else { self.top_right };
             let new_min_y: i32 = top.get_y() as i32 - missing_start_rows;
            if new_min_y < 0 {
                new_min_y = 0;
            }
             let new_top: ResultPoint = ResultPoint::new(&top.get_x(), new_min_y);
            if is_left {
                new_top_left = new_top;
            } else {
                new_top_right = new_top;
            }
        }
        if missing_end_rows > 0 {
             let bottom: ResultPoint =  if is_left { self.bottom_left } else { self.bottom_right };
             let new_max_y: i32 = bottom.get_y() as i32 + missing_end_rows;
            if new_max_y >= self.image.get_height() {
                new_max_y = self.image.get_height() - 1;
            }
             let new_bottom: ResultPoint = ResultPoint::new(&bottom.get_x(), new_max_y);
            if is_left {
                new_bottom_left = new_bottom;
            } else {
                new_bottom_right = new_bottom;
            }
        }
        return Ok(BoundingBox::new(self.image, new_top_left, new_bottom_left, new_top_right, new_bottom_right));
    }

    fn  get_min_x(&self) -> i32  {
        return self.min_x;
    }

    fn  get_max_x(&self) -> i32  {
        return self.max_x;
    }

    fn  get_min_y(&self) -> i32  {
        return self.min_y;
    }

    fn  get_max_y(&self) -> i32  {
        return self.max_y;
    }

    fn  get_top_left(&self) -> ResultPoint  {
        return self.top_left;
    }

    fn  get_top_right(&self) -> ResultPoint  {
        return self.top_right;
    }

    fn  get_bottom_left(&self) -> ResultPoint  {
        return self.bottom_left;
    }

    fn  get_bottom_right(&self) -> ResultPoint  {
        return self.bottom_right;
    }
}

