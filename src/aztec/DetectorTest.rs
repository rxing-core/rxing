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

use crate::{aztec::decoder, common::BitMatrix, exceptions::Exceptions};

use super::{
    detector::{self, Detector, Point},
    encoder::{self, AztecCode},
};

/**
 * Tests for the Detector
 *
 * @author Frank Yellin
 */

#[test]
fn testErrorInParameterLocatorZeroZero() {
    // Layers=1, CodeWords=1.  So the parameter info and its Reed-Solomon info
    // will be completely zero!
    testErrorInParameterLocator("X");
}

#[test]
fn testErrorInParameterLocatorCompact() {
    testErrorInParameterLocator("This is an example Aztec symbol for Wikipedia.");
}

#[test]
fn testErrorInParameterLocatorNotCompact() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYabcdefghijklmnopqrstuvwxyz";
    testErrorInParameterLocator(&format!("{}{}{}", alphabet, alphabet, alphabet));
}

// Test that we can tolerate errors in the parameter locator bits
fn testErrorInParameterLocator(data: &str) {
    let aztec = encoder::encoder::encode(data, 25, encoder::encoder::DEFAULT_AZTEC_LAYERS)
        .expect("encode should create");
    let random = rand::thread_rng(); //Random(aztec.getMatrix().hashCode());   // pseudo-random, but deterministic
    let layers = aztec.getLayers();
    let compact = aztec.isCompact();
    let orientationPoints = getOrientationPoints(&aztec);
    for isMirror in [false, true] {
        // for (boolean isMirror : new boolean[] { false, true }) {
        for matrix in getRotations(aztec.getMatrix()) {
            // for (BitMatrix matrix : getRotations(aztec.getMatrix())) {
            // Systematically try every possible 1- and 2-bit error.
            for error1 in 0..orientationPoints.size() {
                // for (int error1 = 0; error1 < orientationPoints.size(); error1++) {
                for error2 in error1..orientationPoints.size() {
                    // for (int error2 = error1; error2 < orientationPoints.size(); error2++) {
                    let copy = if isMirror {
                        transpose(&matrix)
                    } else {
                        clone(&matrix)
                    };
                    copy.flip(
                        orientationPoints.get(error1).getX(),
                        orientationPoints.get(error1).getY(),
                    );
                    if error2 > error1 {
                        // if error2 == error1, we only test a single error
                        copy.flip(
                            orientationPoints.get(error2).getX(),
                            orientationPoints.get(error2).getY(),
                        );
                    }
                    // The detector doesn't seem to work when matrix bits are only 1x1.  So magnify.
                    let r = Detector::new(makeLarger(&copy, 3)).detect(isMirror);
                    assert!(r.is_ok());
                    let r = r.expect("result already tested as ok");
                    assert_eq!(r.getNbLayers(), layers);
                    assert_eq!(r.isCompact(), compact);
                    let res = decoder::decode(&r).expect("decode should be ok");
                    assert_eq!(data, res.getText());
                }
            }
            // Try a few random three-bit errors;
            for i in 0..5 {
                // for (int i = 0; i < 5; i++) {
                let copy = clone(&matrix);
                let errors = Vec::new();
                while errors.size() < 3 {
                    // Quick and dirty way of getting three distinct integers between 1 and n.
                    errors.push(random.nextInt(orientationPoints.size()));
                }
                for error in errors {
                    // for (int error : errors) {
                    copy.flip(
                        orientationPoints.get(error).getX(),
                        orientationPoints.get(error).getY(),
                    );
                }
                // try {
                if let Err(res) = detector::Detector::new(makeLarger(&copy, 3)).detect(false) {
                    if let Exceptions::NotFoundException(msg) = res {
                        // all ok
                    } else {
                        panic!("Should not reach here");
                    }
                } else {
                    panic!("Should not reach here");
                }
                //   // new Detector(makeLarger(copy, 3)).detect(false);
                //   fail("Should not reach here");
                // } catch (NotFoundException expected) {
                //   // continue
                // }
            }
        }
    }
}

// Zooms a bit matrix so that each bit is factor x factor
fn makeLarger(input: &BitMatrix, factor: u32) -> BitMatrix {
    let width = input.getWidth();
    let output = BitMatrix::with_single_dimension(width * factor);
    for inputY in 0..width {
        // for (int inputY = 0; inputY < width; inputY++) {
        for inputX in 0..width {
            // for (int inputX = 0; inputX < width; inputX++) {
            if input.get(inputX, inputY) {
                output.setRegion(inputX * factor, inputY * factor, factor, factor);
            }
        }
    }
    return output;
}

// Returns a list of the four rotations of the BitMatrix.
fn getRotations(matrix0: &BitMatrix) -> Vec<BitMatrix> {
    let matrix90 = rotateRight(matrix0);
    let matrix180 = rotateRight(&matrix90);
    let matrix270 = rotateRight(&matrix180);
    vec![*matrix0, matrix90, matrix180, matrix270]
}

// Rotates a square BitMatrix to the right by 90 degrees
fn rotateRight(input: &BitMatrix) -> BitMatrix {
    let width = input.getWidth();
    let result = BitMatrix::with_single_dimension(width);
    for x in 0..width {
        // for (int x = 0; x < width; x++) {
        for y in 0..width {
            // for (int y = 0; y < width; y++) {
            if (input.get(x, y)) {
                result.set(y, width - x - 1);
            }
        }
    }
    return result;
}

// Returns the transpose of a bit matrix, which is equivalent to rotating the
// matrix to the right, and then flipping it left-to-right
fn transpose(input: &BitMatrix) -> BitMatrix {
    let width = input.getWidth();
    let result = BitMatrix::with_single_dimension(width);
    for x in 0..width {
        // for (int x = 0; x < width; x++) {
        for y in 0..width {
            // for (int y = 0; y < width; y++) {
            if (input.get(x, y)) {
                result.set(y, x);
            }
        }
    }
    return result;
}

fn clone(input: &BitMatrix) -> BitMatrix {
    let width = input.getWidth();
    let result = BitMatrix::with_single_dimension(width);
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

fn getOrientationPoints(code: &AztecCode) -> Vec<Point> {
    let center = code.getMatrix().getWidth() / 2;
    let offset = if code.isCompact() { 5 } else { 7 };
    let result = Vec::new();
    let mut xSign = -1;
    while xSign <= 1 {
        // for (int xSign = -1; xSign <= 1; xSign += 2) {
        let mut ySign = -1;
        while ySign <= 1 {
            // for (int ySign = -1; ySign <= 1; ySign += 2) {
            result.add(Point::new(center + xSign * offset, center + ySign * offset));
            result.add(Point::new(
                center + xSign * (offset - 1),
                center + ySign * offset,
            ));
            result.add(Point::new(
                center + xSign * offset,
                center + ySign * (offset - 1),
            ));

            ySign += 2;
        }
        xSign += 2;
    }
    return result;
}
