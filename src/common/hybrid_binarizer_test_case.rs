/*
 * Copyright 2026 RXing authors
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

use super::test_utils::MockLuminanceSource;
use super::HybridBinarizer;
use crate::Binarizer;

#[test]
fn test_hybrid_binarizer_small_image() {
    // Small image should fall back to GlobalHistogramBinarizer
    let width = 10;
    let height = 10;
    let mut luminances = vec![0; width * height];
    // Create a simple black and white pattern
    for y in 0..height {
        for x in 0..width {
            if x < 5 {
                luminances[y * width + x] = 0; // Black
            } else {
                luminances[y * width + x] = 255; // White
            }
        }
    }

    let source = MockLuminanceSource::new(width, height, luminances);
    let binarizer = HybridBinarizer::new(source);
    let matrix = binarizer.get_black_matrix().unwrap();

    assert_eq!(width as u32, matrix.getWidth());
    assert_eq!(height as u32, matrix.getHeight());

    for y in 0..height {
        for x in 0..width {
            if x < 5 {
                assert!(matrix.get(x as u32, y as u32));
            } else {
                assert!(!matrix.get(x as u32, y as u32));
            }
        }
    }
}

#[test]
fn test_hybrid_binarizer_large_image() {
    // Large image uses local thresholding
    let width = 40; // HybridBinarizer::MINIMUM_DIMENSION is 40
    let height = 40;
    let mut luminances = vec![0; width * height];

    // Create a pattern with a gradient to test local thresholding
    for y in 0..height {
        for x in 0..width {
            if x < 20 {
                luminances[y * width + x] = 50; // Dark grey
            } else {
                luminances[y * width + x] = 200; // Light grey
            }
        }
    }

    let source = MockLuminanceSource::new(width, height, luminances);
    let binarizer = HybridBinarizer::new(source);
    let matrix = binarizer.get_black_matrix().unwrap();

    assert_eq!(width as u32, matrix.getWidth());
    assert_eq!(height as u32, matrix.getHeight());

    for y in 0..height {
        for x in 0..width {
            if x < 20 {
                assert!(
                    matrix.get(x as u32, y as u32),
                    "Bit at ({}, {}) should be set",
                    x,
                    y
                );
            } else {
                assert!(
                    !matrix.get(x as u32, y as u32),
                    "Bit at ({}, {}) should NOT be set",
                    x,
                    y
                );
            }
        }
    }
}
