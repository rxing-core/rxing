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

use crate::qrcode::{
    decoder::{ErrorCorrectionLevel, Mode, Version},
    encoder::ByteMatrix,
};

use super::QRCode;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author mysen@google.com (Chris Mysen) - ported from C++
 */

#[test]
fn test() {
    let mut qrCode = QRCode::new();

    // First, test simple setters and getters.
    // We use numbers of version 7-H.
    qrCode.setMode(Mode::BYTE);
    qrCode.setECLevel(ErrorCorrectionLevel::H);
    qrCode.setVersion(Version::getVersionForNumber(7).expect("must exist"));
    qrCode.setMaskPattern(3);

    assert_eq!(&Mode::BYTE, qrCode.getMode().as_ref().unwrap());
    assert_eq!(ErrorCorrectionLevel::H, qrCode.getECLevel().unwrap());
    assert_eq!(7, qrCode.getVersion().as_ref().unwrap().getVersionNumber());
    assert_eq!(3, qrCode.getMaskPattern());

    // Prepare the matrix.
    let mut matrix = ByteMatrix::new(45, 45);
    // Just set bogus zero/one values.
    for y in 0..45 {
        // for (int y = 0; y < 45; ++y) {
        for x in 0..45 {
            // for (int x = 0; x < 45; ++x) {
            matrix.set(x, y, ((y + x) % 2) as u8);
        }
    }

    // Set the matrix.
    qrCode.setMatrix(matrix.clone());
    assert_eq!(&matrix, qrCode.getMatrix().as_ref().unwrap());
}

#[test]
fn testToString1() {
    let qrCode = QRCode::new();
    let expected =
        "<<\n mode: null\n ecLevel: null\n version: null\n maskPattern: -1\n matrix: null\n>>\n";
    assert_eq!(expected, qrCode.to_string());
}

#[test]
fn testToString2() {
    let mut qrCode = QRCode::new();
    qrCode.setMode(Mode::BYTE);
    qrCode.setECLevel(ErrorCorrectionLevel::H);
    qrCode.setVersion(Version::getVersionForNumber(1).expect("predefined value must exist"));
    qrCode.setMaskPattern(3);
    let mut matrix = ByteMatrix::new(21, 21);
    for y in 0..21 {
        // for (int y = 0; y < 21; ++y) {
        for x in 0..21 {
            // for (int x = 0; x < 21; ++x) {
            matrix.set(x, y, ((y + x) % 2) as u8);
        }
    }
    qrCode.setMatrix(matrix);
    let expected = "<<\n \
 mode: BYTE\n \
 ecLevel: H\n \
 version: 1\n \
 maskPattern: 3\n \
 matrix:\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n \
 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1\n \
 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0 1 0\n\
>>\n";
    assert_eq!(expected, qrCode.to_string());
}

#[test]
fn testIsValidMaskPattern() {
    assert!(!QRCode::isValidMaskPattern(-1));
    assert!(QRCode::isValidMaskPattern(0));
    assert!(QRCode::isValidMaskPattern(7));
    assert!(!QRCode::isValidMaskPattern(8));
}
