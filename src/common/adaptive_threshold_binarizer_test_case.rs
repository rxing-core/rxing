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

use super::AdaptiveThresholdBinarizer;
use super::test_utils::MockLuminanceSource;
use crate::Binarizer;

#[test]
fn test_adaptive_threshold_binarizer() {
    let width = 40;
    let height = 40;
    let mut luminances = vec![0; width * height];

    // Create a pattern with alternating black and white bars
    for y in 0..height {
        for x in 0..width {
            if (x / 5) % 2 == 0 {
                luminances[y * width + x] = 0; // Black
            } else {
                luminances[y * width + x] = 255; // White
            }
        }
    }

    let source = MockLuminanceSource::new(width, height, luminances);
    // Radius of 10 for adaptive threshold
    let binarizer = AdaptiveThresholdBinarizer::new(source, 10);
    let matrix = binarizer.get_black_matrix().unwrap();

    assert_eq!(width as u32, matrix.getWidth());
    assert_eq!(height as u32, matrix.getHeight());

    // Check points in the middle of bars to avoid boundary issues
    assert!(matrix.get(2, 2)); // First bar (black)
    assert!(!matrix.get(7, 7)); // Second bar (white)
    assert!(matrix.get(12, 12)); // Third bar (black)
    assert!(!matrix.get(17, 17)); // Fourth bar (white)
}
