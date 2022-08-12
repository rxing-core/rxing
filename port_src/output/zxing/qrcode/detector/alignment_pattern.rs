/*
 * Copyright 2007 ZXing authors
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
// package com::google::zxing::qrcode::detector;

/**
 * <p>Encapsulates an alignment pattern, which are the smaller square patterns found in
 * all but the simplest QR Codes.</p>
 *
 * @author Sean Owen
 */
pub struct AlignmentPattern {
    super: ResultPoint;

     let estimated_module_size: f32;
}

impl AlignmentPattern {

    fn new( pos_x: f32,  pos_y: f32,  estimated_module_size: f32) -> AlignmentPattern {
        super(pos_x, pos_y);
        let .estimatedModuleSize = estimated_module_size;
    }

    /**
   * <p>Determines if this alignment pattern "about equals" an alignment pattern at the stated
   * position and size -- meaning, it is at nearly the same center with nearly the same size.</p>
   */
    fn  about_equals(&self,  module_size: f32,  i: f32,  j: f32) -> bool  {
        if Math::abs(i - get_y()) <= module_size && Math::abs(j - get_x()) <= module_size {
             let module_size_diff: f32 = Math::abs(module_size - self.estimated_module_size);
            return module_size_diff <= 1.0f || module_size_diff <= self.estimated_module_size;
        }
        return false;
    }

    /**
   * Combines this object's current estimate of a finder pattern position and module size
   * with a new estimate. It returns a new {@code FinderPattern} containing an average of the two.
   */
    fn  combine_estimate(&self,  i: f32,  j: f32,  new_module_size: f32) -> AlignmentPattern  {
         let combined_x: f32 = (get_x() + j) / 2.0f;
         let combined_y: f32 = (get_y() + i) / 2.0f;
         let combined_module_size: f32 = (self.estimated_module_size + new_module_size) / 2.0f;
        return AlignmentPattern::new(combined_x, combined_y, combined_module_size);
    }
}

