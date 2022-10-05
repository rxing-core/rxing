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

use crate::common::BitMatrix;

use super::DataMask;

/**
 * @author Sean Owen
 */

type MaskCondition = fn(u32, u32) -> bool;

#[test]
fn testMask0() {
    testMaskAcrossDimensions(DataMask::DATA_MASK_000, |i, j| ((i + j) % 2 == 0));
    testMaskAcrossDimensionsU8(0, |i, j| ((i + j) % 2 == 0));

}

#[test]
fn testMask1() {
    testMaskAcrossDimensions(DataMask::DATA_MASK_001, |i, j| i % 2 == 0);
    testMaskAcrossDimensionsU8(1, |i, j| i % 2 == 0);

}

#[test]
fn testMask2() {
    testMaskAcrossDimensions(DataMask::DATA_MASK_010, |i, j| j % 3 == 0);
    testMaskAcrossDimensionsU8(2, |i, j| j % 3 == 0);

}

#[test]
fn testMask3() {
    testMaskAcrossDimensions(DataMask::DATA_MASK_011, |i, j| (i + j) % 3 == 0);
    testMaskAcrossDimensionsU8(3, |i, j| (i + j) % 3 == 0);
}

#[test]
fn testMask4() {
    testMaskAcrossDimensions(DataMask::DATA_MASK_100, |i, j| (i / 2 + j / 3) % 2 == 0);
    testMaskAcrossDimensionsU8(4, |i, j| (i / 2 + j / 3) % 2 == 0);
}

#[test]
fn testMask5() {
    testMaskAcrossDimensions(DataMask::DATA_MASK_101, |i, j| {
        (i * j) % 2 + (i * j) % 3 == 0
    });
    testMaskAcrossDimensionsU8(5, |i, j| {
        (i * j) % 2 + (i * j) % 3 == 0
    });
}

#[test]
fn testMask6() {
    testMaskAcrossDimensions(DataMask::DATA_MASK_110, |i, j| {
        ((i * j) % 2 + (i * j) % 3) % 2 == 0
    });
    testMaskAcrossDimensionsU8(6, |i, j| {
        ((i * j) % 2 + (i * j) % 3) % 2 == 0
    });
}

#[test]
fn testMask7() {
    testMaskAcrossDimensions(DataMask::DATA_MASK_111, |i, j| {
        ((i + j) % 2 + (i * j) % 3) % 2 == 0
    });
    testMaskAcrossDimensionsU8(7, |i, j| {
        ((i + j) % 2 + (i * j) % 3) % 2 == 0
    });
}

fn testMaskAcrossDimensionsU8(mask: u8, condition: MaskCondition) {
    testMaskAcrossDimensions(mask.try_into().unwrap(), condition)
}

fn testMaskAcrossDimensions(mask: DataMask, condition: MaskCondition) {
    // let mask = DataMask.values()[reference];
    for version in 1..=40 {
        // for (int version = 1; version <= 40; version++) {
        let dimension = 17 + 4 * version;
        testMask(mask, dimension, condition);
    }
}

fn testMask(mask: DataMask, dimension: u32, condition: MaskCondition) {
    let mut bits = BitMatrix::with_single_dimension(dimension);
    mask.unmaskBitMatrix(&mut bits, dimension);
    for i in 0..dimension {
        // for (int i = 0; i < dimension; i++) {
        for j in 0..dimension {
            // for (int j = 0; j < dimension; j++) {
            assert_eq!(condition(i, j), bits.get(j, i), "({},{})", i, j);
        }
    }
}

// @FunctionalInterface
// private interface MaskCondition {
//   boolean isMasked(int i, int j);
// }
