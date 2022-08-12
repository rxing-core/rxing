/*
 * Copyright 2010 ZXing authors
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
// package com::google::zxing::oned;

/**
 * This object renders a ITF code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */

 const START_PATTERN: vec![Vec<i32>; 4] = vec![1, 1, 1, 1, ]
;

 const END_PATTERN: vec![Vec<i32>; 3] = vec![3, 1, 1, ]
;

// Pixel width of a 3x wide line
 const W: i32 = 3;

// Pixed width of a narrow line
 const N: i32 = 1;

// See ITFReader.PATTERNS
 const PATTERNS: vec![vec![Vec<Vec<i32>>; 5]; 10] = vec![// 0
vec![N, N, W, W, N, ]
, // 1
vec![W, N, N, N, W, ]
, // 2
vec![N, W, N, N, W, ]
, // 3
vec![W, W, N, N, N, ]
, // 4
vec![N, N, W, N, W, ]
, // 5
vec![W, N, W, N, N, ]
, // 6
vec![N, W, W, N, N, ]
, // 7
vec![N, N, N, W, W, ]
, // 8
vec![W, N, N, W, N, ]
, // 9
vec![N, W, N, W, N, ]
, ]
;
pub struct ITFWriter {
    super: OneDimensionalCodeWriter;
}

impl ITFWriter {

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::ITF);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
         let length: i32 = contents.length();
        if length % 2 != 0 {
            throw IllegalArgumentException::new("The length of the input should be even");
        }
        if length > 80 {
            throw IllegalArgumentException::new(format!("Requested contents should be less than 80 digits long, but got {}", length));
        }
        check_numeric(&contents);
         let result: [bool; 9 + 9 * length] = [false; 9 + 9 * length];
         let mut pos: i32 = append_pattern(&result, 0, &START_PATTERN, true);
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let one: i32 = Character::digit(&contents.char_at(i), 10);
                     let two: i32 = Character::digit(&contents.char_at(i + 1), 10);
                     let mut encoding: [i32; 10] = [0; 10];
                     {
                         let mut j: i32 = 0;
                        while j < 5 {
                            {
                                encoding[2 * j] = PATTERNS[one][j];
                                encoding[2 * j + 1] = PATTERNS[two][j];
                            }
                            j += 1;
                         }
                     }

                    pos += append_pattern(&result, pos, &encoding, true);
                }
                i += 2;
             }
         }

        append_pattern(&result, pos, &END_PATTERN, true);
        return result;
    }
}

