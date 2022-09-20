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
 */
// package com.google.zxing.aztec.decoder;

// import com.google.zxing.aztec.encoder.EncoderTest;

// import com.google.zxing.FormatException;
// import com.google.zxing.RXingResultPoint;
// import com.google.zxing.aztec.AztecDetectorRXingResult;
// import com.google.zxing.common.BitArray;
// import com.google.zxing.common.BitMatrix;
// import com.google.zxing.common.DecoderRXingResult;
// import org.junit.Test;
// import org.junit.Assert;

use crate::{
    aztec::shared_test_methods::{stripSpace, toBitArray, toBooleanArray},
    common::BitMatrix,
    RXingResultPoint,
};

use super::{decoder, AztecDetectorResult::AztecDetectorRXingResult};

/**
 * Tests {@link Decoder}.
 */

const NO_POINTS: &[RXingResultPoint] = &[RXingResultPoint { x: 0.0, y: 0.0 }; 0];

#[test]
fn test_high_level_decode() {
    // no ECI codes
    test_high_level_decode_string(
        "A. b.",
        // 'A'  P/S   '. ' L/L    b    D/L    '.'
        "...X. ..... ...XX XXX.. ...XX XXXX. XX.X",
    );

    // initial ECI code 26 (switch to UTF-8)
    test_high_level_decode_string(
        "Ça",
        // P/S FLG(n) 2  '2'  '6'  B/S   2     0xc3     0x87     L/L   'a'
        "..... ..... .X. .X.. X... XXXXX ...X. XX....XX X....XXX XXX.. ...X.",
    );

    // initial character without ECI (must be interpreted as ISO_8859_1)
    // followed by ECI code 26 (= UTF-8) and UTF-8 text
    test_high_level_decode_string(
        "±Ça",
        // B/S 1     0xb1     P/S   FLG(n) 2  '2'  '6'  B/S   2     0xc3     0x87     L/L   'a'
        "XXXXX ....X X.XX...X ..... ..... .X. .X.. X... XXXXX ...X. XX....XX X....XXX XXX.. ...X.",
    );

    // GS1 data
    test_high_level_decode_string(
        "101233742",
        // P/S FLG(n) 0  D/L   1    0    1    2    3    P/S  FLG(n) 0  3    7    4    2
        "..... ..... ... XXXX. ..XX ..X. ..XX .X.. .X.X .... ..... ... .X.X X..X .XX. .X..",
    );
}

fn test_high_level_decode_string(expectedString: &str, b: &str) {
    let bits = toBitArray(&stripSpace(b));
    assert_eq!(
        expectedString,
        decoder::highLevelDecode(&toBooleanArray(&bits)).expect("highLevelDecode Failed"),
        "highLevelDecode() failed for input bits: {}",
        b
    );
}

#[test]
fn test_aztec_rxing_result() {
    let matrix = BitMatrix::parse_strings(
        r"X X X X X     X X X       X X X     X X X     
X X X     X X X     X X X X     X X X     X X 
  X   X X       X   X   X X X X     X     X X 
  X   X X     X X     X     X   X       X   X 
  X X   X X         X               X X     X 
  X X   X X X X X X X X X X X X X X X     X   
  X X X X X                       X   X X X   
  X   X   X   X X X X X X X X X   X X X   X X 
  X   X X X   X               X   X X       X 
  X X   X X   X   X X X X X   X   X X X X   X 
  X X   X X   X   X       X   X   X   X X X   
  X   X   X   X   X   X   X   X   X   X   X   
  X X X   X   X   X       X   X   X X   X X   
  X X X X X   X   X X X X X   X   X X X   X X 
X X   X X X   X               X   X   X X   X 
  X       X   X X X X X X X X X   X   X     X 
  X X   X X                       X X   X X   
  X X X   X X X X X X X X X X X X X X   X X   
X     X     X     X X   X X               X X 
X   X X X X X   X X X X X     X   X   X     X 
X X X   X X X X           X X X       X     X 
X X     X X X     X X X X     X X X     X X   
    X X X     X X X       X X X     X X X X   
",
        "X ",
        "  ",
    )
    .expect("Bitmatrix should init");
    let r = AztecDetectorRXingResult::new(matrix, NO_POINTS.to_vec(), false, 30, 2);
    let result = decoder::decode(&r).expect("decoder should init");
    assert_eq!("88888TTTTTTTTTTTTTTTTTTTTTTTTTTTTTT", result.getText());
    assert_eq!(
        &vec![
            -11i8 as u8,
            85,
            85,
            117,
            107,
            90,
            -42i8 as u8,
            -75i8 as u8,
            -83i8 as u8,
            107,
            90,
            -42i8 as u8,
            -75i8 as u8,
            -83i8 as u8,
            107,
            90,
            -42i8 as u8,
            -75i8 as u8,
            -83i8 as u8,
            107,
            90,
            -42i8 as u8,
            -80i8 as u8
        ],
        result.getRawBytes()
    );
    assert_eq!(180, result.getNumBits());
}

#[test]
fn test_aztec_rxing_result_eci() {
    let matrix = BitMatrix::parse_strings(
        r"      X     X X X   X           X     
    X X   X X   X X X X X X X   X     
    X X                         X   X 
  X X X X X X X X X X X X X X X X X   
      X                       X       
      X   X X X X X X X X X   X   X   
  X X X   X               X   X X X   
  X   X   X   X X X X X   X   X X X   
      X   X   X       X   X   X X X   
  X   X   X   X   X   X   X   X   X   
X   X X   X   X       X   X   X     X 
  X X X   X   X X X X X   X   X X     
      X   X               X   X X   X 
      X   X X X X X X X X X   X   X X 
  X   X                       X       
X X   X X X X X X X X X X X X X X X   
X X     X   X         X X X       X X 
  X   X   X   X X X X X     X X   X   
X     X       X X   X X X       X     ",
        "X ",
        "  ",
    )
    .expect("string parse success");
    let r = AztecDetectorRXingResult::new(matrix, NO_POINTS.to_vec(), false, 15, 1);
    let result = decoder::decode(&r).expect("decode success");
    assert_eq!("Français", result.getText());
}

#[test]
#[should_panic]
fn test_decode_too_many_errors() {
    let matrix = BitMatrix::parse_strings(
        r"
X X . X . . . X X . . . X . . X X X . X . X X X X X . 
X X . . X X . . . . . X X . . . X X . . . X . X . . X 
X . . . X X . . X X X . X X . X X X X . X X . . X . . 
. . . . X . X X . . X X . X X . X . X X X X . X . . X 
X X X . . X X X X X . . . . . X X . . . X . X . X . X 
X X . . . . . . . . X . . . X . X X X . X . . X . . . 
X X . . X . . . . . X X . . . . . X . . . . X . . X X 
. . . X . X . X . . . . . X X X X X X . . . . . . X X 
X . . . X . X X X X X X . . X X X . X . X X X X X X . 
X . . X X X . X X X X X X X X X X X X X . . . X . X X 
. . . . X X . . . X . . . . . . . X X . . . X X . X . 
. . . X X X . . X X . X X X X X . X . . X . . . . . . 
X . . . . X . X . X . X . . . X . X . X X . X X . X X 
X . X . . X . X . X . X . X . X . X . . . . . X . X X 
X . X X X . . X . X . X . . . X . X . X X X . . . X X 
X X X X X X X X . X . X X X X X . X . X . X . X X X . 
. . . . . . . X . X . . . . . . . X X X X . . . X X X 
X X . . X . . X . X X X X X X X X X X X X X . . X . X 
X X X . X X X X . . X X X X . . X . . . . X . . X X X 
. . . . X . X X X . . . . X X X X . . X X X X . . . . 
. . X . . X . X . . . X . X X . X X . X . . . X . X . 
X X . . X . . X X X X X X X . . X . X X X X X X X . . 
X . X X . . X X . . . . . X . . . . . . X X . X X X . 
X . . X X . . X X . X . X . . . . X . X . . X . . X . 
X . X . X . . X . X X X X X X X X . X X X X . . X X . 
X X X X . . . X . . X X X . X X . . X . . . . X X X . 
X X . X . X . . . X . X . . . . X X . X . . X X . . . ",
        "X ",
        ". ",
    )
    .expect("parse string failed");
    let r = AztecDetectorRXingResult::new(matrix, NO_POINTS.to_vec(), true, 16, 4);
    assert!(decoder::decode(&r).is_ok());
}

#[test]
#[should_panic]
fn test_decode_too_many_errors2() {
    let matrix = BitMatrix::parse_strings(
        r"
. X X . . X . X X . . . X . . X X X . . . X X . X X . 
X X . X X . . X . . . X X . . . X X . X X X . X . X X 
. . . . X . . . X X X . X X . X X X X . X X . . X . . 
X . X X . . X . . . X X . X X . X . X X . . . . . X . 
X X . X . . X . X X . . . . . X X . . . . . X . . . X 
X . . X . . . . . . X . . . X . X X X X X X X . . . X 
X . . X X . . X . . X X . . . . . X . . . . . X X X . 
. . X X X X . X . . . . . X X X X X X . . . . . . X X 
X . . . X . X X X X X X . . X X X . X . X X X X X X . 
X . . X X X . X X X X X X X X X X X X X . . . X . X X 
. . . . X X . . . X . . . . . . . X X . . . X X . X . 
. . . X X X . . X X . X X X X X . X . . X . . . . . . 
X . . . . X . X . X . X . . . X . X . X X . X X . X X 
X . X . . X . X . X . X . X . X . X . . . . . X . X X 
X . X X X . . X . X . X . . . X . X . X X X . . . X X 
X X X X X X X X . X . X X X X X . X . X . X . X X X . 
. . . . . . . X . X . . . . . . . X X X X . . . X X X 
X X . . X . . X . X X X X X X X X X X X X X . . X . X 
X X X . X X X X . . X X X X . . X . . . . X . . X X X 
. . X X X X X . X . . . . X X X X . . X X X . X . X . 
. . X X . X . X . . . X . X X . X X . . . . X X . . . 
X . . . X . X . X X X X X X . . X . X X X X X . X . . 
. X . . . X X X . . . . . X . . . . . X X X X X . X . 
X . . X . X X X X . X . X . . . . X . X X . X . . X . 
X . . . X X . X . X X X X X X X X . X X X X . . X X . 
. X X X X . . X . . X X X . X X . . X . . . . X X X . 
X X . . . X X . . X . X . . . . X X . X . . X . X . X ",
        "X ",
        ". ",
    )
    .expect("String Parse failed");
    let r = AztecDetectorRXingResult::new(matrix, NO_POINTS.to_vec(), true, 16, 4);
    assert!(decoder::decode(&r).is_ok());
}

#[test]
fn test_raw_bytes() {
    let bool0 = vec![false; 0];
    let bool1 = vec![true];
    let bool7 = vec![true, false, true, false, true, false, true];
    let bool8 = vec![true, false, true, false, true, false, true, false];
    let bool9 = vec![true, false, true, false, true, false, true, false, true];
    let bool16 = vec![
        false, true, true, false, false, false, true, true, true, true, false, false, false, false,
        false, true,
    ];
    let byte0 = vec![0u8; 0];
    let byte1 = vec![-128i8 as u8];
    let byte7 = vec![-86i8 as u8];
    let byte8 = vec![-86i8 as u8];
    let byte9 = vec![-86i8 as u8, -128i8 as u8];
    let byte16 = vec![99, -63i8 as u8];

    assert_eq!(byte0, decoder::convertBoolArrayToByteArray(&bool0));
    assert_eq!(byte1, decoder::convertBoolArrayToByteArray(&bool1));
    assert_eq!(byte7, decoder::convertBoolArrayToByteArray(&bool7));
    assert_eq!(byte8, decoder::convertBoolArrayToByteArray(&bool8));
    assert_eq!(byte9, decoder::convertBoolArrayToByteArray(&bool9));
    assert_eq!(byte16, decoder::convertBoolArrayToByteArray(&bool16));
}
