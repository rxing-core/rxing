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
// package com::google::zxing::pdf417::detector;

/**
 * @author Guenther Grau
 */
pub struct PDF417DetectorResult {

     let bits: BitMatrix;

     let points: List<Vec<ResultPoint>>;

     let rotation: i32;
}

impl PDF417DetectorResult {

    pub fn new( bits: &BitMatrix,  points: &List<Vec<ResultPoint>>,  rotation: i32) -> PDF417DetectorResult {
        let .bits = bits;
        let .points = points;
        let .rotation = rotation;
    }

    pub fn new( bits: &BitMatrix,  points: &List<Vec<ResultPoint>>) -> PDF417DetectorResult {
        this(bits, &points, 0);
    }

    pub fn  get_bits(&self) -> BitMatrix  {
        return self.bits;
    }

    pub fn  get_points(&self) -> List<Vec<ResultPoint>>  {
        return self.points;
    }

    pub fn  get_rotation(&self) -> i32  {
        return self.rotation;
    }
}

