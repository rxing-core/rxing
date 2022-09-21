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

use crate::common::BitMatrix;

/**
 * Tests for the Detector
 *
 * @author Frank Yellin
 */

  #[test]
  fn testErrorInParameterLocatorZeroZero()  {
    // Layers=1, CodeWords=1.  So the parameter info and its Reed-Solomon info
    // will be completely zero!
    testErrorInParameterLocator("X");
  }

  #[test]
  fn testErrorInParameterLocatorCompact() {
    testErrorInParameterLocator("This is an example Aztec symbol for Wikipedia.");
  }

  #[test]
  fn testErrorInParameterLocatorNotCompact()  {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYabcdefghijklmnopqrstuvwxyz";
    testErrorInParameterLocator(&format!("{}{}{}",alphabet , alphabet , alphabet));
  }

  // Test that we can tolerate errors in the parameter locator bits
  fn testErrorInParameterLocator( data:&str)  {
    let aztec = Encoder.encode(data, 25, Encoder.DEFAULT_AZTEC_LAYERS);
    let random = new Random(aztec.getMatrix().hashCode());   // pseudo-random, but deterministic
    let layers = aztec.getLayers();
    let compact = aztec.isCompact();
    let orientationPoints = getOrientationPoints(aztec);
    for (boolean isMirror : new boolean[] { false, true }) {
      for (BitMatrix matrix : getRotations(aztec.getMatrix())) {
        // Systematically try every possible 1- and 2-bit error.
        for (int error1 = 0; error1 < orientationPoints.size(); error1++) {
          for (int error2 = error1; error2 < orientationPoints.size(); error2++) {
            BitMatrix copy = isMirror ? transpose(matrix) : clone(matrix);
            copy.flip(orientationPoints.get(error1).getX(), orientationPoints.get(error1).getY());
            if (error2 > error1) {
              // if error2 == error1, we only test a single error
              copy.flip(orientationPoints.get(error2).getX(), orientationPoints.get(error2).getY());
            }
            // The detector doesn't seem to work when matrix bits are only 1x1.  So magnify.
            AztecDetectorRXingResult r = new Detector(makeLarger(copy, 3)).detect(isMirror);
            assertNotNull(r);
            assertEquals(r.getNbLayers(), layers);
            assertEquals(r.isCompact(), compact);
            DecoderRXingResult res = new Decoder().decode(r);
            assertEquals(data, res.getText());
          }
        }
        // Try a few random three-bit errors;
        for (int i = 0; i < 5; i++) {
          BitMatrix copy = clone(matrix);
          Collection<Integer> errors = new TreeSet<>();
          while (errors.size() < 3) {
            // Quick and dirty way of getting three distinct integers between 1 and n.
            errors.add(random.nextInt(orientationPoints.size()));
          }
          for (int error : errors) {
            copy.flip(orientationPoints.get(error).getX(), orientationPoints.get(error).getY());
          }
          try {
            new Detector(makeLarger(copy, 3)).detect(false);
            fail("Should not reach here");
          } catch (NotFoundException expected) {
            // continue
          }
        }
      }
    }
  }

  // Zooms a bit matrix so that each bit is factor x factor
  fn  makeLarger( input:&BitMatrix,  factor:u32) -> BitMatrix{
    let width = input.getWidth();
    let output =  BitMatrix::new(width * factor);
    for (int inputY = 0; inputY < width; inputY++) {
      for (int inputX = 0; inputX < width; inputX++) {
        if (input.get(inputX, inputY)) {
          output.setRegion(inputX * factor, inputY * factor, factor, factor);
        }
      }
    }
    return output;
  }

  // Returns a list of the four rotations of the BitMatrix.
  fn  getRotations( matrix0:&BitMatrix)-> Vec<BitMatrix> {
    let matrix90 = rotateRight(matrix0);
    let matrix180 = rotateRight(matrix90);
    let matrix270 = rotateRight(matrix180);
    return Arrays.asList(matrix0, matrix90, matrix180, matrix270);
  }

  // Rotates a square BitMatrix to the right by 90 degrees
  fn  rotateRight( input:&BitMatrix) -> BitMatrix{
    let width = input.getWidth();
    let result = new BitMatrix(width);
    for (int x = 0; x < width; x++) {
      for (int y = 0; y < width; y++) {
        if (input.get(x,y)) {
          result.set(y, width - x - 1);
        }
      }
    }
    return result;
  }

  // Returns the transpose of a bit matrix, which is equivalent to rotating the
  // matrix to the right, and then flipping it left-to-right
   fn transpose( input:&BitMatrix) -> BitMatrix {
    let width = input.getWidth();
    let result = new BitMatrix(width);
    for (int x = 0; x < width; x++) {
      for (int y = 0; y < width; y++) {
        if (input.get(x, y)) {
          result.set(y, x);
        }
      }
    }
    return result;
  }

  fn clone( input:&BitMatrix) -> BitMatrix  {
    let width = input.getWidth();
    let result = new BitMatrix(width);
    for (int x = 0; x < width; x++) {
      for (int y = 0; y < width; y++) {
        if (input.get(x,y)) {
          result.set(x,y);
        }
      }
    }
    return result;
  }

  fn  getOrientationPoints( code:&AztecCode) -> Vec<Point> {
    let center = code.getMatrix().getWidth() / 2;
    let offset = code.isCompact() ? 5 : 7;
    let result = new ArrayList<>();
    for (int xSign = -1; xSign <= 1; xSign += 2) {
      for (int ySign = -1; ySign <= 1; ySign += 2) {
        result.add(new Point(center + xSign * offset, center + ySign * offset));
        result.add(new Point(center + xSign * (offset - 1), center + ySign * offset));
        result.add(new Point(center + xSign * offset, center + ySign * (offset - 1)));
      }
    }
    return result;
  }
