/*
 * Copyright 2013 ZXing authors
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

// package com.google.zxing.aztec.detector;

// import com.google.zxing.NotFoundException;
// import com.google.zxing.aztec.AztecDetectorRXingResult;
// import com.google.zxing.aztec.decoder.Decoder;
// import com.google.zxing.aztec.detector.Detector.Point;
// import com.google.zxing.aztec.encoder.AztecCode;
// import com.google.zxing.aztec.encoder.Encoder;
// import com.google.zxing.common.BitMatrix;
// import com.google.zxing.common.DecoderRXingResult;
// import org.junit.Assert;
// import org.junit.Test;

// import java.util.ArrayList;
// import java.util.Arrays;
// import java.util.Collection;
// import java.util.List;
// import java.util.Random;
// import java.util.TreeSet;

use rand::Rng;

use crate::{aztec::decoder, common::BitMatrix, exceptions::Exceptions, Point};

use super::{
    detector::{self, Detector},
    encoder::{self, AztecCode},
};

/**
 * Tests for the Detector
 *
 * @author Frank Yellin
 */

#[test]
fn test_error_in_parameter_locator_zero_zero() {
    // Layers=1, CodeWords=1.  So the parameter info and its Reed-Solomon info
    // will be completely zero!
    test_error_in_parameter_locator("X");
}

#[test]
fn test_error_in_parameter_locator_compact() {
    test_error_in_parameter_locator("This is an example Aztec symbol for Wikipedia.");
}

#[test]
fn test_error_in_parameter_locator_not_compact() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYabcdefghijklmnopqrstuvwxyz";
    test_error_in_parameter_locator(&format!("{alphabet}{alphabet}{alphabet}"));
}

#[test]
fn test_aztec_rxing_result_sample() {
    let matrix = BitMatrix::parse_strings(TEST_BARCODE, "X ", "  ").expect("string parse success");
    let r = Detector::new(&matrix).detect(false);
    assert!(r.is_ok());
    let r = r.expect("result already tested as ok");
    let res = decoder::decode(&r).expect("decode success");
    assert_eq!("Histórico", res.getText());
}

// Test that we can tolerate errors in the parameter locator bits
fn test_error_in_parameter_locator(data: &str) {
    let aztec =
        encoder::aztec_encoder::encode(data, 25, encoder::aztec_encoder::DEFAULT_AZTEC_LAYERS)
            .expect("encode should create");
    // dbg!(aztec.getMatrix().to_string());
    let mut random = rand::thread_rng(); //Random(aztec.getMatrix().hashCode());   // pseudo-random, but deterministic
    let layers = aztec.getLayers();
    let compact = aztec.isCompact();
    let orientation_points = get_orientation_points(&aztec);
    for isMirror in [false, true] {
        // for (boolean isMirror : new boolean[] { false, true }) {
        for matrix in get_rotations(aztec.getMatrix()) {
            // for (BitMatrix matrix : getRotations(aztec.getMatrix())) {
            // Systematically try every possible 1- and 2-bit error.
            for error1 in 0..orientation_points.len() {
                // for (int error1 = 0; error1 < orientationPoints.size(); error1++) {
                for error2 in error1..orientation_points.len() {
                    // for (int error2 = error1; error2 < orientationPoints.size(); error2++) {
                    let mut copy = if isMirror {
                        transpose(&matrix)
                    } else {
                        clone(&matrix)
                    };
                    copy.flip_coords(
                        (orientation_points.get(error1).unwrap().x as i32).unsigned_abs(),
                        (orientation_points.get(error1).unwrap().y as i32).unsigned_abs(),
                    );
                    if error2 > error1 {
                        // if error2 == error1, we only test a single error
                        copy.flip_coords(
                            (orientation_points.get(error2).unwrap().x as i32).unsigned_abs(),
                            (orientation_points.get(error2).unwrap().y as i32).unsigned_abs(),
                        );
                    }
                    // dbg!(copy.to_string());
                    // dbg!(make_larger(&copy, 3).to_string());
                    // The detector doesn't seem to work when matrix bits are only 1x1.  So magnify.
                    let r = Detector::new(&make_larger(&copy, 3)).detect(isMirror);
                    assert!(r.is_ok());
                    let r = r.expect("result already tested as ok");
                    assert_eq!(r.getNbLayers(), layers);
                    assert_eq!(r.isCompact(), compact);
                    let res = decoder::decode(&r).expect("decode should be ok");
                    assert_eq!(data, res.getText());
                }
            }
            // Try a few random three-bit errors;
            for _i in 0..5 {
                // for (int i = 0; i < 5; i++) {
                let mut copy = clone(&matrix);
                let mut errors = Vec::new();
                while errors.len() < 3 {
                    // Quick and dirty way of getting three distinct integers between 1 and n.
                    errors.push(random.gen_range(0..orientation_points.len()));
                }
                for error in errors {
                    // for (int error : errors) {
                    copy.flip_coords(
                        orientation_points.get(error).unwrap().x as u32,
                        orientation_points.get(error).unwrap().y as u32,
                    );
                    // copy.flip_coords(
                    //     orientation_points.get(error).unwrap().get_x().abs() as u32,
                    //     orientation_points.get(error).unwrap().get_y().abs() as u32,
                    // );
                }
                // try {
                if let Err(res) = detector::Detector::new(&make_larger(&copy, 3)).detect(false) {
                    if let Exceptions::NotFoundException(_msg) = res {
                        // all ok
                    } else {
                        panic!("Only Exceptions::NotFoundException allowed, got {res}");
                    }
                } else {
                    let r = Detector::new(&make_larger(&copy, 3)).detect(false);
                    assert!(r.is_ok());
                    let r = r.expect("result already tested as ok");
                    assert_eq!(r.getNbLayers(), layers);
                    assert_eq!(r.isCompact(), compact);
                    let res = decoder::decode(&r).expect("decode should be ok");
                    assert_eq!(data, res.getText());
                    //panic!("Should be unable to detect given value.");
                }
            }
        }
    }
}

// Zooms a bit matrix so that each bit is factor x factor
fn make_larger(input: &BitMatrix, factor: u32) -> BitMatrix {
    let width = input.getWidth();
    let mut output = BitMatrix::with_single_dimension(width * factor).expect("new");
    for inputY in 0..width {
        // for (int inputY = 0; inputY < width; inputY++) {
        for inputX in 0..width {
            // for (int inputX = 0; inputX < width; inputX++) {
            if input.get(inputX, inputY) {
                output
                    .setRegion(inputX * factor, inputY * factor, factor, factor)
                    .expect("region set should be ok");
            }
        }
    }
    output
}

// Returns a list of the four rotations of the BitMatrix.
fn get_rotations(matrix0: &BitMatrix) -> Vec<BitMatrix> {
    let matrix90 = rotate_right(matrix0);
    let matrix180 = rotate_right(&matrix90);
    let matrix270 = rotate_right(&matrix180);
    vec![matrix0.clone(), matrix90, matrix180, matrix270]
    // vec![matrix0.clone()]
    // vec![matrix90]
    // vec![matrix180]
    // vec![matrix270]
    // vec![matrix0.clone(), matrix90, matrix270, matrix180 ]
}

// Rotates a square BitMatrix to the right by 90 degrees
fn rotate_right(input: &BitMatrix) -> BitMatrix {
    let width = input.getWidth();
    let mut result = BitMatrix::with_single_dimension(width).expect("new");
    for x in 0..width {
        // for (int x = 0; x < width; x++) {
        for y in 0..width {
            // for (int y = 0; y < width; y++) {
            if input.get(x, y) {
                result.set(y, width - x - 1);
            }
        }
    }
    result
}

// Returns the transpose of a bit matrix, which is equivalent to rotating the
// matrix to the right, and then flipping it left-to-right
fn transpose(input: &BitMatrix) -> BitMatrix {
    let width = input.getWidth();
    let mut result = BitMatrix::with_single_dimension(width).expect("new");
    for x in 0..width {
        // for (int x = 0; x < width; x++) {
        for y in 0..width {
            // for (int y = 0; y < width; y++) {
            if input.get(x, y) {
                result.set(y, x);
            }
        }
    }
    result
}

fn clone(input: &BitMatrix) -> BitMatrix {
    let width = input.getWidth();
    let mut result = BitMatrix::with_single_dimension(width).expect("new");
    for x in 0..width {
        // for (int x = 0; x < width; x++) {
        for y in 0..width {
            // for (int y = 0; y < width; y++) {
            if input.get(x, y) {
                result.set(x, y);
            }
        }
    }
    result
}

fn get_orientation_points(code: &AztecCode) -> Vec<Point> {
    let center = code.getMatrix().getWidth() as i32 / 2;
    let offset = if code.isCompact() { 5 } else { 7 };
    let mut result = Vec::new();
    let mut xSign: i32 = -1;
    while xSign <= 1 {
        // for (int xSign = -1; xSign <= 1; xSign += 2) {
        let mut ySign: i32 = -1;
        while ySign <= 1 {
            // for (int ySign = -1; ySign <= 1; ySign += 2) {
            result.push(Point::from((
                center + xSign * offset,
                center + ySign * offset,
            )));
            result.push(Point::from((
                center + xSign * (offset - 1),
                center + ySign * offset,
            )));
            result.push(Point::from((
                center + xSign * offset,
                center + ySign * (offset - 1),
            )));

            ySign += 2;
        }
        xSign += 2;
    }
    result
}

const TEST_BARCODE: &str = r"                    X X X X X                     X X X X X           X X X X X X X X X X                                         X X X X X           
                    X X X X X                     X X X X X           X X X X X X X X X X                                         X X X X X           
                    X X X X X                     X X X X X           X X X X X X X X X X                                         X X X X X           
                    X X X X X                     X X X X X           X X X X X X X X X X                                         X X X X X           
                    X X X X X                     X X X X X           X X X X X X X X X X                                         X X X X X           
          X X X X X X X X X X X X X X X                     X X X X X           X X X X X X X X X X X X X X X                               X X X X X 
          X X X X X X X X X X X X X X X                     X X X X X           X X X X X X X X X X X X X X X                               X X X X X 
          X X X X X X X X X X X X X X X                     X X X X X           X X X X X X X X X X X X X X X                               X X X X X 
          X X X X X X X X X X X X X X X                     X X X X X           X X X X X X X X X X X X X X X                               X X X X X 
          X X X X X X X X X X X X X X X                     X X X X X           X X X X X X X X X X X X X X X                               X X X X X 
                    X X X X X X X X X X                                         X X X X X           X X X X X           X X X X X                     
                    X X X X X X X X X X                                         X X X X X           X X X X X           X X X X X                     
                    X X X X X X X X X X                                         X X X X X           X X X X X           X X X X X                     
                    X X X X X X X X X X                                         X X X X X           X X X X X           X X X X X                     
                    X X X X X X X X X X                                         X X X X X           X X X X X           X X X X X                     
X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X                     
X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X                     
X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X                     
X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X                     
X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X                     
X X X X X X X X X X           X X X X X                                                                       X X X X X                               
X X X X X X X X X X           X X X X X                                                                       X X X X X                               
X X X X X X X X X X           X X X X X                                                                       X X X X X                               
X X X X X X X X X X           X X X X X                                                                       X X X X X                               
X X X X X X X X X X           X X X X X                                                                       X X X X X                               
                    X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X           X X X X X           
                    X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X           X X X X X           
                    X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X           X X X X X           
                    X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X           X X X X X           
                    X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X           X X X X X           
          X X X X X           X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X           X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X           X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X           X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X           X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X X X X X X X X X X X           X X X X X           X X X X X           X X X X X           X X X X X X X X X X X X X X X X X X X X 
          X X X X X X X X X X X X X X X           X X X X X           X X X X X           X X X X X           X X X X X X X X X X X X X X X X X X X X 
          X X X X X X X X X X X X X X X           X X X X X           X X X X X           X X X X X           X X X X X X X X X X X X X X X X X X X X 
          X X X X X X X X X X X X X X X           X X X X X           X X X X X           X X X X X           X X X X X X X X X X X X X X X X X X X X 
          X X X X X X X X X X X X X X X           X X X X X           X X X X X           X X X X X           X X X X X X X X X X X X X X X X X X X X 
          X X X X X X X X X X X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X X X X X X X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X X X X X X X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X X X X X X X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X X X X X X X X X X X           X X X X X                               X X X X X           X X X X X                               
          X X X X X X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X X X X X X                     
          X X X X X X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X X X X X X                     
          X X X X X X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X X X X X X                     
          X X X X X X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X X X X X X                     
          X X X X X X X X X X X X X X X           X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X X X X X X                     
                    X X X X X X X X X X                                                                       X X X X X X X X X X X X X X X           
                    X X X X X X X X X X                                                                       X X X X X X X X X X X X X X X           
                    X X X X X X X X X X                                                                       X X X X X X X X X X X X X X X           
                    X X X X X X X X X X                                                                       X X X X X X X X X X X X X X X           
                    X X X X X X X X X X                                                                       X X X X X X X X X X X X X X X           
X X X X X                     X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X 
X X X X X                     X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X 
X X X X X                     X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X 
X X X X X                     X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X 
X X X X X                     X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X X           X X X X X 
X X X X X                                         X X X X X                     X X X X X           X X X X X                                         
X X X X X                                         X X X X X                     X X X X X           X X X X X                                         
X X X X X                                         X X X X X                     X X X X X           X X X X X                                         
X X X X X                                         X X X X X                     X X X X X           X X X X X                                         
X X X X X                                         X X X X X                     X X X X X           X X X X X                                         
X X X X X X X X X X                               X X X X X X X X X X X X X X X           X X X X X           X X X X X                     X X X X X 
X X X X X X X X X X                               X X X X X X X X X X X X X X X           X X X X X           X X X X X                     X X X X X 
X X X X X X X X X X                               X X X X X X X X X X X X X X X           X X X X X           X X X X X                     X X X X X 
X X X X X X X X X X                               X X X X X X X X X X X X X X X           X X X X X           X X X X X                     X X X X X 
X X X X X X X X X X                               X X X X X X X X X X X X X X X           X X X X X           X X X X X                     X X X X X 
X X X X X X X X X X X X X X X X X X X X                     X X X X X X X X X X           X X X X X X X X X X           X X X X X           X X X X X 
X X X X X X X X X X X X X X X X X X X X                     X X X X X X X X X X           X X X X X X X X X X           X X X X X           X X X X X 
X X X X X X X X X X X X X X X X X X X X                     X X X X X X X X X X           X X X X X X X X X X           X X X X X           X X X X X 
X X X X X X X X X X X X X X X X X X X X                     X X X X X X X X X X           X X X X X X X X X X           X X X X X           X X X X X 
X X X X X X X X X X X X X X X X X X X X                     X X X X X X X X X X           X X X X X X X X X X           X X X X X           X X X X X 
";
