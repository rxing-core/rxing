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

// package com.google.zxing.common;

// import org.junit.Assert;
// import org.junit.Test;

// import java.util.Arrays;

// /**
//  * @author Sean Owen
//  * @author dswitkin@google.com (Daniel Switkin)
//  */
// public final class BitMatrixTestCase extends Assert {

use super::BitMatrix;

static BIT_MATRIX_POINTS: [u32; 6] = [1, 2, 2, 0, 3, 1];

#[test]
fn test_get_set() {
    let mut matrix = BitMatrix::with_single_dimension(33);
    assert_eq!(33, matrix.getHeight());
    for y in 0..33 {
        // for (int y = 0; y < 33; y++) {
        for x in 0..33 {
            // for (int x = 0; x < 33; x++) {
            if y * x % 3 == 0 {
                matrix.set(x, y);
            }
        }
    }
    for y in 0..33 {
        // for (int y = 0; y < 33; y++) {
        for x in 0..33 {
            // for (int x = 0; x < 33; x++) {
            assert_eq!(y * x % 3 == 0, matrix.get(x, y));
        }
    }
}

#[test]
fn test_set_region() {
    let mut matrix = BitMatrix::with_single_dimension(5);
    matrix.setRegion(1, 1, 3, 3).expect("must set");
    for y in 0..5 {
        // for (int y = 0; y < 5; y++) {
        for x in 0..5 {
            // for (int x = 0; x < 5; x++) {
            assert_eq!(y >= 1 && y <= 3 && x >= 1 && x <= 3, matrix.get(x, y));
        }
    }
}

#[test]
fn test_enclosing() {
    let mut matrix = BitMatrix::with_single_dimension(5);
    assert!(matrix.getEnclosingRectangle().is_none());
    matrix.setRegion(1, 1, 1, 1).expect("must set");
    assert_eq!(vec![1, 1, 1, 1], matrix.getEnclosingRectangle().unwrap());
    matrix.setRegion(1, 1, 3, 2).expect("must set");
    assert_eq!(vec![1, 1, 3, 2], matrix.getEnclosingRectangle().unwrap());
    matrix.setRegion(0, 0, 5, 5).expect("must set");
    assert_eq!(vec![0, 0, 5, 5], matrix.getEnclosingRectangle().unwrap());
}

#[test]
fn test_on_bit() {
    let mut matrix = BitMatrix::with_single_dimension(5);
    assert!(matrix.getTopLeftOnBit().is_none());
    assert!(matrix.getBottomRightOnBit().is_none());
    matrix.setRegion(1, 1, 1, 1).expect("must set");
    assert_eq!(vec![1, 1], matrix.getTopLeftOnBit().unwrap());
    assert_eq!(vec![1, 1], matrix.getBottomRightOnBit().unwrap());
    matrix.setRegion(1, 1, 3, 2).expect("must set");
    assert_eq!(vec![1, 1], matrix.getTopLeftOnBit().unwrap());
    assert_eq!(vec![3, 2], matrix.getBottomRightOnBit().unwrap());
    matrix.setRegion(0, 0, 5, 5).expect("must set");
    assert_eq!(vec![0, 0], matrix.getTopLeftOnBit().unwrap());
    assert_eq!(vec![4, 4], matrix.getBottomRightOnBit().unwrap());
}

#[test]
fn test_rectangular_matrix() {
    let mut matrix = BitMatrix::new(75, 20).unwrap();
    assert_eq!(75, matrix.getWidth());
    assert_eq!(20, matrix.getHeight());
    matrix.set(10, 0);
    matrix.set(11, 1);
    matrix.set(50, 2);
    matrix.set(51, 3);
    matrix.flip_coords(74, 4);
    matrix.flip_coords(0, 5);

    // Should all be on
    assert!(matrix.get(10, 0));
    assert!(matrix.get(11, 1));
    assert!(matrix.get(50, 2));
    assert!(matrix.get(51, 3));
    assert!(matrix.get(74, 4));
    assert!(matrix.get(0, 5));

    // Flip a couple back off
    matrix.flip_coords(50, 2);
    matrix.flip_coords(51, 3);
    assert!(!matrix.get(50, 2));
    assert!(!matrix.get(51, 3));
}

#[test]
fn test_rectangular_set_region() {
    let mut matrix = BitMatrix::new(320, 240).unwrap();
    assert_eq!(320, matrix.getWidth());
    assert_eq!(240, matrix.getHeight());
    matrix.setRegion(105, 22, 80, 12).expect("must set");

    // Only bits in the region should be on
    for y in 0..240 {
        // for (int y = 0; y < 240; y++) {
        for x in 0..320 {
            // for (int x = 0; x < 320; x++) {
            assert_eq!(y >= 22 && y < 34 && x >= 105 && x < 185, matrix.get(x, y));
        }
    }
}

#[test]
fn test_get_row() {
    let mut matrix = BitMatrix::new(102, 5).unwrap();
    for x in 0..102 {
        // for (int x = 0; x < 102; x++) {
        if (x & 0x03) == 0 {
            matrix.set(x, 2);
        }
    }

    // Should allocate
    let array = matrix.getRow(2);
    assert_eq!(102, array.getSize());

    // Should reallocate
    // let mut array2 = BitArray::with_size(60);
    let array2 = matrix.getRow(2);
    assert_eq!(102, array2.getSize());

    // Should use provided object, with original BitArray size
    // let mut array3 = BitArray::with_size(200);
    let array3 = matrix.getRow(2);
    assert_eq!(200, array3.getSize());

    for x in 0..102 {
        // for (int x = 0; x < 102; x++) {
        let on = (x & 0x03) == 0;
        assert_eq!(on, array.get(x));
        assert_eq!(on, array2.get(x));
        assert_eq!(on, array3.get(x));
    }
}

#[test]
fn test_rotate90_simple() {
    let mut matrix = BitMatrix::new(3, 3).unwrap();
    matrix.set(0, 0);
    matrix.set(0, 1);
    matrix.set(1, 2);
    matrix.set(2, 1);

    matrix.rotate90();

    assert!(matrix.get(0, 2));
    assert!(matrix.get(1, 2));
    assert!(matrix.get(2, 1));
    assert!(matrix.get(1, 0));
}

#[test]
fn test_rotate180_simple() {
    let mut matrix = BitMatrix::new(3, 3).unwrap();
    matrix.set(0, 0);
    matrix.set(0, 1);
    matrix.set(1, 2);
    matrix.set(2, 1);

    matrix.rotate180();

    assert!(matrix.get(2, 2));
    assert!(matrix.get(2, 1));
    assert!(matrix.get(1, 0));
    assert!(matrix.get(0, 1));
}

#[test]
fn test_rotate180_case() {
    test_rotate_180(7, 4);
    test_rotate_180(7, 5);
    test_rotate_180(8, 4);
    test_rotate_180(8, 5);
}

#[test]
fn test_parse() {
    let emptyMatrix = BitMatrix::new(3, 3).unwrap();
    let mut fullMatrix = BitMatrix::new(3, 3).unwrap();
    fullMatrix.setRegion(0, 0, 3, 3).expect("must set");
    let mut centerMatrix = BitMatrix::new(3, 3).unwrap();
    centerMatrix.setRegion(1, 1, 1, 1).expect("must set");
    let emptyMatrix24 = BitMatrix::new(2, 4).unwrap();

    assert_eq!(
        emptyMatrix,
        BitMatrix::parse_strings("   \n   \n   \n", "x", " ").unwrap()
    );
    assert_eq!(
        emptyMatrix,
        BitMatrix::parse_strings("   \n   \r\r\n   \n\r", "x", " ").unwrap()
    );
    assert_eq!(
        emptyMatrix,
        BitMatrix::parse_strings("   \n   \n   ", "x", " ").unwrap()
    );

    assert_eq!(
        fullMatrix,
        BitMatrix::parse_strings("xxx\nxxx\nxxx\n", "x", " ").unwrap()
    );

    assert_eq!(
        centerMatrix,
        BitMatrix::parse_strings("   \n x \n   \n", "x", " ").unwrap()
    );
    assert_eq!(
        centerMatrix,
        BitMatrix::parse_strings("      \n  x   \n      \n", "x ", "  ").unwrap()
    );

    assert!(BitMatrix::parse_strings("   \n xy\n   \n", "x", " ").is_err());

    assert_eq!(
        emptyMatrix24,
        BitMatrix::parse_strings("  \n  \n  \n  \n", "x", " ").unwrap()
    );

    assert_eq!(
        centerMatrix,
        BitMatrix::parse_strings(&centerMatrix.toString("x", "."), "x", ".").unwrap()
    );
}

#[test]
fn test_parse_boolean() {
    let emptyMatrix = BitMatrix::new(3, 3).unwrap();
    let mut fullMatrix = BitMatrix::new(3, 3).unwrap();
    fullMatrix.setRegion(0, 0, 3, 3).expect("must set");
    let mut centerMatrix = BitMatrix::new(3, 3).unwrap();
    centerMatrix.setRegion(1, 1, 1, 1).expect("must set");
    let _emptyMatrix24 = BitMatrix::new(2, 4).unwrap();

    let mut matrix = vec![vec![false; 3]; 3];
    // boolean[][] matrix = new boolean[3][3];
    assert_eq!(emptyMatrix, BitMatrix::parse_bools(&matrix));
    matrix[1][1] = true;
    assert_eq!(centerMatrix, BitMatrix::parse_bools(&matrix));
    for arr in matrix.iter_mut() {
        // for (boolean[] arr : matrix) {
        arr[..].clone_from_slice(&[true, true, true])
    }
    assert_eq!(fullMatrix, BitMatrix::parse_bools(&matrix));
}

#[test]
fn test_unset() {
    let emptyMatrix = BitMatrix::new(3, 3).unwrap();
    let mut matrix = emptyMatrix.clone();
    matrix.set(1, 1);
    assert_ne!(emptyMatrix, matrix);
    matrix.unset(1, 1);
    assert_eq!(emptyMatrix, matrix);
    matrix.unset(1, 1);
    assert_eq!(emptyMatrix, matrix);
}

#[test]
fn test_xor_case() {
    let emptyMatrix = BitMatrix::new(3, 3).unwrap();
    let mut fullMatrix = BitMatrix::new(3, 3).unwrap();
    fullMatrix.setRegion(0, 0, 3, 3).expect("must set");
    let mut centerMatrix = BitMatrix::new(3, 3).unwrap();
    centerMatrix.setRegion(1, 1, 1, 1).expect("must set");
    let mut invertedCenterMatrix = fullMatrix.clone();
    invertedCenterMatrix.unset(1, 1);
    let badMatrix = BitMatrix::new(4, 4).unwrap();

    test_XOR(&emptyMatrix, &emptyMatrix, &emptyMatrix);
    test_XOR(&emptyMatrix, &centerMatrix, &centerMatrix);
    test_XOR(&emptyMatrix, &fullMatrix, &fullMatrix);

    test_XOR(&centerMatrix, &emptyMatrix, &centerMatrix);
    test_XOR(&centerMatrix, &centerMatrix, &emptyMatrix);
    test_XOR(&centerMatrix, &fullMatrix, &invertedCenterMatrix);

    test_XOR(&invertedCenterMatrix, &emptyMatrix, &invertedCenterMatrix);
    test_XOR(&invertedCenterMatrix, &centerMatrix, &fullMatrix);
    test_XOR(&invertedCenterMatrix, &fullMatrix, &centerMatrix);

    test_XOR(&fullMatrix, &emptyMatrix, &fullMatrix);
    test_XOR(&fullMatrix, &centerMatrix, &invertedCenterMatrix);
    test_XOR(&fullMatrix, &fullMatrix, &emptyMatrix);

    assert!(emptyMatrix.clone().xor(&badMatrix).is_err());
    // try {
    //   emptyMatrix.clone().xor(badMatrix);
    //   fail();
    // } catch (IllegalArgumentException ex) {
    //   // good
    // }

    assert!(badMatrix.clone().xor(&emptyMatrix).is_err());
    // try {
    //   badMatrix.clone().xor(emptyMatrix);
    //   fail();
    // } catch (IllegalArgumentException ex) {
    //   // good
    // }
}

pub fn matrix_to_string(result: &BitMatrix) -> String {
    assert_eq!(1, result.getHeight());
    let mut builder = String::with_capacity(result.getWidth().try_into().unwrap());
    for i in 0..result.getWidth() {
        // for (int i = 0; i < result.getWidth(); i++) {
        builder.push(if result.get(i, 0) { '1' } else { '0' });
    }
    return builder;
}

fn test_XOR(dataMatrix: &BitMatrix, flipMatrix: &BitMatrix, expectedMatrix: &BitMatrix) {
    let mut matrix = dataMatrix.clone();
    matrix.xor(flipMatrix).expect("must set");
    assert_eq!(*expectedMatrix, matrix);
}

fn test_rotate_180(width: u32, height: u32) {
    let mut input = get_input(width, height);
    input.rotate180();
    let expected = get_expected(width, height);

    for y in 0..height {
        // for (int y = 0; y < height; y++) {
        for x in 0..width {
            // for (int x = 0; x < width; x++) {
            assert_eq!(expected.get(x, y), input.get(x, y), "({},{})", x, y);
        }
    }
}

fn get_expected(width: u32, height: u32) -> BitMatrix {
    let mut result = BitMatrix::new(width, height).unwrap();
    let mut i = 0;
    while i < BIT_MATRIX_POINTS.len() {
        // for (int i = 0; i < BIT_MATRIX_POINTS.length; i += 2) {
        result.set(
            width - 1 - BIT_MATRIX_POINTS[i],
            height - 1 - BIT_MATRIX_POINTS[i + 1],
        );
        i += 2;
    }
    return result;
}

fn get_input(width: u32, height: u32) -> BitMatrix {
    let mut result = BitMatrix::new(width, height).unwrap();
    let mut i = 0;
    while i < BIT_MATRIX_POINTS.len() {
        // for (int i = 0; i < BIT_MATRIX_POINTS.length; i += 2) {
        result.set(BIT_MATRIX_POINTS[i], BIT_MATRIX_POINTS[i + 1]);
        i += 2;
    }
    return result;
}

// }
