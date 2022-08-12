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
 * <p>Encapsulates information about finder patterns in an image, including the location of
 * the three finder patterns, and their estimated module size.</p>
 *
 * @author Sean Owen
 */
pub struct FinderPatternInfo {

     let bottom_left: FinderPattern;

     let top_left: FinderPattern;

     let top_right: FinderPattern;
}

impl FinderPatternInfo {

    pub fn new( pattern_centers: &Vec<FinderPattern>) -> FinderPatternInfo {
        let .bottomLeft = pattern_centers[0];
        let .topLeft = pattern_centers[1];
        let .topRight = pattern_centers[2];
    }

    pub fn  get_bottom_left(&self) -> FinderPattern  {
        return self.bottom_left;
    }

    pub fn  get_top_left(&self) -> FinderPattern  {
        return self.top_left;
    }

    pub fn  get_top_right(&self) -> FinderPattern  {
        return self.top_right;
    }
}

