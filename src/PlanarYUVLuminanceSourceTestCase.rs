/*
 * Copyright 2014 ZXing authors
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
//  */

// package com.google.zxing;

// import org.junit.Assert;
// import org.junit.Test;

// /**
//  * Tests {@link PlanarYUVLuminanceSource}.
//  */
// public final class PlanarYUVLuminanceSourceTestCase extends Assert {

use crate::{LuminanceSource, PlanarYUVLuminanceSource};

static YUV: [u8; 36] = [
    // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 0, -1, -1, -2, -3, -5, -8, -13, -21, -34, -55, -89,
    128, 129, 129, 130, 131, 133, 136, 141, 149, 162, 183, 217, 128, 127, 127, 126, 125, 123, 120,
    115, 107, 94, 73, 39, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
];
static COLS: usize = 6;
static ROWS: usize = 4;
static Y: [u8; 24] = [
    128, 129, 129, 130, 131, 133, 136, 141, 149, 162, 183, 217, 128, 127, 127, 126, 125, 123, 120,
    115, 107, 94, 73, 39,
];

#[test]
fn test_no_crop() {
    let source = PlanarYUVLuminanceSource::new_with_all(
        YUV.to_vec(),
        COLS,
        ROWS,
        0,
        0,
        COLS,
        ROWS,
        false,
        false,
    )
    .unwrap();
    assert_equals(&Y, 0, &source.get_matrix(), 0, Y.len());
    for r in 0..ROWS {
        // for (int r = 0; r < ROWS; r++) {
        assert_equals(&Y, r * COLS, &source.get_row(r), 0, COLS);
    }
}

#[test]
fn test_crop() {
    let source = PlanarYUVLuminanceSource::new_with_all(
        YUV.to_vec(),
        COLS,
        ROWS,
        1,
        1,
        COLS - 2,
        ROWS - 2,
        false,
        false,
    )
    .unwrap();
    assert!(source.is_crop_supported());
    let cropMatrix = source.get_matrix();
    for r in 0..ROWS - 2 {
        // for (int r = 0; r < ROWS - 2; r++) {
        assert_equals(
            &Y,
            (r + 1) * COLS + 1,
            &cropMatrix,
            r * (COLS - 2),
            COLS - 2,
        );
    }
    for r in 0..ROWS - 2 {
        // for (int r = 0; r < ROWS - 2; r++) {
        assert_equals(&Y, (r + 1) * COLS + 1, &source.get_row(r), 0, COLS - 2);
    }
}

#[test]
fn test_thumbnail() {
    let source = PlanarYUVLuminanceSource::new_with_all(
        YUV.to_vec(),
        COLS,
        ROWS,
        0,
        0,
        COLS,
        ROWS,
        false,
        false,
    )
    .unwrap();
    let c_vec = vec![
        ((0x00FF0000 << 8) + 0x00808080) as u8,
        ((0x00FF0000 << 8) + 0x00818181) as u8,
        ((0x00FF0000 << 8) + 0x00838383) as u8,
        ((0x00FF0000 << 8) + 0x00808080) as u8,
        ((0x00FF0000 << 8) + 0x007F7F7F) as u8,
        ((0x00FF0000 << 8) + 0x007D7D7D) as u8,
    ];
    assert_eq!(c_vec, source.renderThumbnail());
}

fn assert_equals(
    expected: &[u8],
    expectedFrom: usize,
    actual: &[u8],
    actualFrom: usize,
    length: usize,
) {
    for i in 0..length {
        // for (int i = 0; i < length; i++) {
        assert_eq!(expected[expectedFrom + i], actual[actualFrom + i]);
    }
}

// }
