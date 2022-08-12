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

 const MAX_NEARBY_DISTANCE: i32 = 5;
struct DetectionResultColumn {

     let bounding_box: BoundingBox;

     let mut codewords: Vec<Codeword>;
}

impl DetectionResultColumn {

    fn new( bounding_box: &BoundingBox) -> DetectionResultColumn {
        let .boundingBox = BoundingBox::new(bounding_box);
        codewords = : [Option<Codeword>; bounding_box.get_max_y() - bounding_box.get_min_y() + 1] = [None; bounding_box.get_max_y() - bounding_box.get_min_y() + 1];
    }

    fn  get_codeword_nearby(&self,  image_row: i32) -> Codeword  {
         let mut codeword: Codeword = self.get_codeword(image_row);
        if codeword != null {
            return codeword;
        }
         {
             let mut i: i32 = 1;
            while i < MAX_NEARBY_DISTANCE {
                {
                     let near_image_row: i32 = self.image_row_to_codeword_index(image_row) - i;
                    if near_image_row >= 0 {
                        codeword = self.codewords[near_image_row];
                        if codeword != null {
                            return codeword;
                        }
                    }
                    near_image_row = self.image_row_to_codeword_index(image_row) + i;
                    if near_image_row < self.codewords.len() {
                        codeword = self.codewords[near_image_row];
                        if codeword != null {
                            return codeword;
                        }
                    }
                }
                i += 1;
             }
         }

        return null;
    }

    fn  image_row_to_codeword_index(&self,  image_row: i32) -> i32  {
        return image_row - self.bounding_box.get_min_y();
    }

    fn  set_codeword(&self,  image_row: i32,  codeword: &Codeword)   {
        self.codewords[self.image_row_to_codeword_index(image_row)] = codeword;
    }

    fn  get_codeword(&self,  image_row: i32) -> Codeword  {
        return self.codewords[self.image_row_to_codeword_index(image_row)];
    }

    fn  get_bounding_box(&self) -> BoundingBox  {
        return self.bounding_box;
    }

    fn  get_codewords(&self) -> Vec<Codeword>  {
        return self.codewords;
    }

    pub fn  to_string(&self) -> String  {
        let tryResult1 = 0;
        'try1: loop {
        ( let formatter: Formatter = Formatter::new()) {
             let mut row: i32 = 0;
            for  let codeword: Codeword in self.codewords {
                if codeword == null {
                    formatter.format("%3d:    |   %n", row += 1 !!!check!!! post increment);
                    continue;
                }
                formatter.format("%3d: %3d|%3d%n", row += 1 !!!check!!! post increment, &codeword.get_row_number(), &codeword.get_value());
            }
            return formatter.to_string();
        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }

    }
}

