/*
 * Copyright 2008 ZXing authors
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

use crate::qrcode::encoder::{mask_util, ByteMatrix};

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author mysen@google.com (Chris Mysen) - ported from C++
 */

#[test]
fn testApplyMaskPenaltyRule1() {
    let mut matrix = ByteMatrix::new(4, 1);
    matrix.set(0, 0, 0);
    matrix.set(1, 0, 0);
    matrix.set(2, 0, 0);
    matrix.set(3, 0, 0);
    assert_eq!(0, mask_util::applyMaskPenaltyRule1(&matrix));
    // Horizontal.
    let mut matrix = ByteMatrix::new(6, 1);
    matrix.set(0, 0, 0);
    matrix.set(1, 0, 0);
    matrix.set(2, 0, 0);
    matrix.set(3, 0, 0);
    matrix.set(4, 0, 0);
    matrix.set(5, 0, 1);
    assert_eq!(3, mask_util::applyMaskPenaltyRule1(&matrix));
    matrix.set(5, 0, 0);
    assert_eq!(4, mask_util::applyMaskPenaltyRule1(&matrix));
    // Vertical.
    let mut matrix = ByteMatrix::new(1, 6);
    matrix.set(0, 0, 0);
    matrix.set(0, 1, 0);
    matrix.set(0, 2, 0);
    matrix.set(0, 3, 0);
    matrix.set(0, 4, 0);
    matrix.set(0, 5, 1);
    assert_eq!(3, mask_util::applyMaskPenaltyRule1(&matrix));
    matrix.set(0, 5, 0);
    assert_eq!(4, mask_util::applyMaskPenaltyRule1(&matrix));
}

#[test]
fn testApplyMaskPenaltyRule2() {
    let mut matrix = ByteMatrix::new(1, 1);
    matrix.set(0, 0, 0);
    assert_eq!(0, mask_util::applyMaskPenaltyRule2(&matrix));
    let mut matrix = ByteMatrix::new(2, 2);
    matrix.set(0, 0, 0);
    matrix.set(1, 0, 0);
    matrix.set(0, 1, 0);
    matrix.set(1, 1, 1);
    assert_eq!(0, mask_util::applyMaskPenaltyRule2(&matrix));
    let mut matrix = ByteMatrix::new(2, 2);
    matrix.set(0, 0, 0);
    matrix.set(1, 0, 0);
    matrix.set(0, 1, 0);
    matrix.set(1, 1, 0);
    assert_eq!(3, mask_util::applyMaskPenaltyRule2(&matrix));
    let mut matrix = ByteMatrix::new(3, 3);
    matrix.set(0, 0, 0);
    matrix.set(1, 0, 0);
    matrix.set(2, 0, 0);
    matrix.set(0, 1, 0);
    matrix.set(1, 1, 0);
    matrix.set(2, 1, 0);
    matrix.set(0, 2, 0);
    matrix.set(1, 2, 0);
    matrix.set(2, 2, 0);
    // Four instances of 2x2 blocks.
    assert_eq!(3 * 4, mask_util::applyMaskPenaltyRule2(&matrix));
}

#[test]
fn testApplyMaskPenaltyRule3() {
    // Horizontal 00001011101.
    let mut matrix = ByteMatrix::new(11, 1);
    matrix.set(0, 0, 0);
    matrix.set(1, 0, 0);
    matrix.set(2, 0, 0);
    matrix.set(3, 0, 0);
    matrix.set(4, 0, 1);
    matrix.set(5, 0, 0);
    matrix.set(6, 0, 1);
    matrix.set(7, 0, 1);
    matrix.set(8, 0, 1);
    matrix.set(9, 0, 0);
    matrix.set(10, 0, 1);
    assert_eq!(40, mask_util::applyMaskPenaltyRule3(&matrix));
    // Horizontal 10111010000.
    let mut matrix = ByteMatrix::new(11, 1);
    matrix.set(0, 0, 1);
    matrix.set(1, 0, 0);
    matrix.set(2, 0, 1);
    matrix.set(3, 0, 1);
    matrix.set(4, 0, 1);
    matrix.set(5, 0, 0);
    matrix.set(6, 0, 1);
    matrix.set(7, 0, 0);
    matrix.set(8, 0, 0);
    matrix.set(9, 0, 0);
    matrix.set(10, 0, 0);
    assert_eq!(40, mask_util::applyMaskPenaltyRule3(&matrix));
    // Horizontal 1011101.
    let mut matrix = ByteMatrix::new(7, 1);
    matrix.set(0, 0, 1);
    matrix.set(1, 0, 0);
    matrix.set(2, 0, 1);
    matrix.set(3, 0, 1);
    matrix.set(4, 0, 1);
    matrix.set(5, 0, 0);
    matrix.set(6, 0, 1);
    assert_eq!(0, mask_util::applyMaskPenaltyRule3(&matrix));
    // Vertical 00001011101.
    let mut matrix = ByteMatrix::new(1, 11);
    matrix.set(0, 0, 0);
    matrix.set(0, 1, 0);
    matrix.set(0, 2, 0);
    matrix.set(0, 3, 0);
    matrix.set(0, 4, 1);
    matrix.set(0, 5, 0);
    matrix.set(0, 6, 1);
    matrix.set(0, 7, 1);
    matrix.set(0, 8, 1);
    matrix.set(0, 9, 0);
    matrix.set(0, 10, 1);
    assert_eq!(40, mask_util::applyMaskPenaltyRule3(&matrix));
    // Vertical 10111010000.
    let mut matrix = ByteMatrix::new(1, 11);
    matrix.set(0, 0, 1);
    matrix.set(0, 1, 0);
    matrix.set(0, 2, 1);
    matrix.set(0, 3, 1);
    matrix.set(0, 4, 1);
    matrix.set(0, 5, 0);
    matrix.set(0, 6, 1);
    matrix.set(0, 7, 0);
    matrix.set(0, 8, 0);
    matrix.set(0, 9, 0);
    matrix.set(0, 10, 0);
    // Vertical 1011101.
    let mut matrix = ByteMatrix::new(1, 7);
    matrix.set(0, 0, 1);
    matrix.set(0, 1, 0);
    matrix.set(0, 2, 1);
    matrix.set(0, 3, 1);
    matrix.set(0, 4, 1);
    matrix.set(0, 5, 0);
    matrix.set(0, 6, 1);
    assert_eq!(0, mask_util::applyMaskPenaltyRule3(&matrix));
}

#[test]
fn testApplyMaskPenaltyRule4() {
    // Dark cell ratio = 0%
    let mut matrix = ByteMatrix::new(1, 1);
    matrix.set(0, 0, 0);
    assert_eq!(100, mask_util::applyMaskPenaltyRule4(&matrix));
    // Dark cell ratio = 5%
    let mut matrix = ByteMatrix::new(2, 1);
    matrix.set(0, 0, 0);
    matrix.set(0, 0, 1);
    assert_eq!(0, mask_util::applyMaskPenaltyRule4(&matrix));
    // Dark cell ratio = 66.67%
    let mut matrix = ByteMatrix::new(6, 1);
    matrix.set(0, 0, 0);
    matrix.set(1, 0, 1);
    matrix.set(2, 0, 1);
    matrix.set(3, 0, 1);
    matrix.set(4, 0, 1);
    matrix.set(5, 0, 0);
    assert_eq!(30, mask_util::applyMaskPenaltyRule4(&matrix));
}

fn testGetDataMaskBitInternal(maskPattern: u32, expected: &Vec<Vec<u32>>) -> bool {
    for x in 0..6 {
        // for (int x = 0; x < 6; ++x) {
        for y in 0..6 {
            // for (int y = 0; y < 6; ++y) {
            if (expected[y][x] == 1)
                != mask_util::getDataMaskBit(maskPattern, x as u32, y as u32)
                    .expect("should never fail")
            {
                return false;
            }
        }
    }
    return true;
}

// See mask patterns on the page 43 of JISX0510:2004.
#[test]
fn testGetDataMaskBit() {
    let mask0 = vec![
        vec![1, 0, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 0, 1],
        vec![1, 0, 1, 0, 1, 0],
        vec![0, 1, 0, 1, 0, 1],
    ];
    assert!(testGetDataMaskBitInternal(0, &mask0));
    let mask1 = vec![
        vec![1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0],
    ];
    assert!(testGetDataMaskBitInternal(1, &mask1));
    let mask2 = vec![
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 1, 0, 0],
    ];
    assert!(testGetDataMaskBitInternal(2, &mask2));
    let mask3 = vec![
        vec![1, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 1],
        vec![0, 1, 0, 0, 1, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 1],
        vec![0, 1, 0, 0, 1, 0],
    ];
    assert!(testGetDataMaskBitInternal(3, &mask3));
    let mask4 = vec![
        vec![1, 1, 1, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 1],
        vec![0, 0, 0, 1, 1, 1],
        vec![1, 1, 1, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
    ];
    assert!(testGetDataMaskBitInternal(4, &mask4));
    let mask5 = vec![
        vec![1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 0, 1, 0, 1, 0],
        vec![1, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 0],
    ];
    assert!(testGetDataMaskBitInternal(5, &mask5));
    let mask6 = vec![
        vec![1, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 1, 1, 0],
        vec![1, 0, 1, 0, 1, 0],
        vec![1, 0, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 1, 1],
    ];
    assert!(testGetDataMaskBitInternal(6, &mask6));
    let mask7 = vec![
        vec![1, 0, 1, 0, 1, 0],
        vec![0, 0, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 1, 1],
        vec![0, 1, 0, 1, 0, 1],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 0],
    ];
    assert!(testGetDataMaskBitInternal(7, &mask7));
}
