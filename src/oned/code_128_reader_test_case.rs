/*
 * Copyright 2026 ZXing authors
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

/**
 * Tests {@link Code128Reader}.
 *
 * Ported from zxing/zxing#2151. rxing does not report reader initialization
 * (FNC3), so the READER_INIT assertions are omitted.
 */
use crate::{
    BarcodeFormat, DecodeHintValue, DecodeHints, EncodeHintValue, EncodeHints, RXingResult,
    RXingResultMetadataType, RXingResultMetadataValue, Writer,
    oned::{Code128Reader, Code128Writer, OneDReader},
};

const INPUTS: [[&str; 3]; 3] = [
    ["@C", "?A", "@@"],
    ["AB", "As", "A?"],
    ["1045", "1142", "1042"],
];
const CHECKSUMS: [u8; 3] = [102, 97, 96];

#[test]
fn testFunctionCodeChecksumsText() {
    for (inputs, codeSet) in INPUTS.iter().zip(["A", "B", "C"]) {
        for (input, checksum) in inputs.iter().zip(CHECKSUMS) {
            let result = decode(input, Some(codeSet), true);
            assert_eq!(checksum, checksumOf(&result));
            assert_eq!(*input, result.getText());
        }
    }
}

#[test]
fn testFunctionCodeChecksumsMetadata() {
    for (inputs, codeSet) in INPUTS.iter().zip(["A", "B", "C"]) {
        for (input, checksum) in inputs.iter().zip(CHECKSUMS) {
            let result = decode(input, Some(codeSet), false);
            assert_eq!(checksum, checksumOf(&result));
            assert_eq!("]C0", symbologyIdentifier(&result));
        }
    }
}

#[test]
fn testGS1WithFunctionCodeChecksum() {
    let input = "01012345678901281726010110BATCH070";
    let result = decode(&format!("\u{00f1}{input}"), None, true);
    assert_eq!(102, checksumOf(&result));
    assert_eq!(format!("]C1{input}"), result.getText());
    assert_eq!("]C1", symbologyIdentifier(&result));
}

#[test]
fn testFunctionCodesInData() {
    let fnc1 = decode("\u{00f1}AB", Some("B"), true);
    assert_eq!("]C1AB", fnc1.getText());
    assert_eq!("]C1", symbologyIdentifier(&fnc1));
    let fnc2 = decode("\u{00f2}AB", Some("B"), false);
    assert_eq!("AB", fnc2.getText());
    assert_eq!("]C4", symbologyIdentifier(&fnc2));
    let fnc3 = decode("\u{00f3}AB", Some("B"), false);
    assert_eq!("AB", fnc3.getText());
}

#[test]
fn testFunctionCodeChecksumsKeepMetadata() {
    let fnc1Checksum = decode("\u{00f3}V", Some("B"), false);
    assert_eq!(102, checksumOf(&fnc1Checksum));
    assert_eq!("V", fnc1Checksum.getText());
    assert_eq!("]C0", symbologyIdentifier(&fnc1Checksum));
    let fnc2Checksum = decode("\u{00f1}@+", Some("B"), true);
    assert_eq!(97, checksumOf(&fnc2Checksum));
    assert_eq!("]C1@+", fnc2Checksum.getText());
    assert_eq!("]C1", symbologyIdentifier(&fnc2Checksum));
}

#[test]
fn testReaderInitializationWithoutText() {
    // The checksum is the only printable codeword, so no text remains.
    let result = decode("\u{00f3}\u{00f3}", Some("B"), false);
    assert_eq!("", result.getText());
}

#[test]
fn testExtendedCharacterWithFunctionCodeChecksum() {
    // FNC4 + 'T' decodes to 'Ô', two bytes in UTF-8, and the checksum is 102.
    let result = decode("\u{00f4}T", Some("B"), false);
    assert_eq!(102, checksumOf(&result));
    assert_eq!("Ô", result.getText());
    assert_eq!("]C0", symbologyIdentifier(&result));
}

fn decode(input: &str, codeSet: Option<&str>, assumeGS1: bool) -> RXingResult {
    let mut encodeHints = EncodeHints::default();
    if let Some(codeSet) = codeSet {
        encodeHints = encodeHints.with(EncodeHintValue::ForceCodeSet(codeSet.to_owned()));
    }
    let matrix = Code128Writer::default()
        .encode_with_hints(input, &BarcodeFormat::CODE_128, 0, 0, &encodeHints)
        .expect("encode");
    let mut decodeHints = DecodeHints::default();
    if assumeGS1 {
        decodeHints = decodeHints.with(DecodeHintValue::AssumeGs1(true));
    }
    Code128Reader
        .decode_row(0, &matrix.getRow(0), &decodeHints)
        .expect("decode")
}

fn checksumOf(result: &RXingResult) -> u8 {
    let raw = result.getRawBytes();
    raw[raw.len() - 2]
}

fn symbologyIdentifier(result: &RXingResult) -> &str {
    match result
        .getRXingResultMetadata()
        .get(&RXingResultMetadataType::SYMBOLOGY_IDENTIFIER)
    {
        Some(RXingResultMetadataValue::SymbologyIdentifier(id)) => id,
        other => panic!("unexpected symbology identifier: {other:?}"),
    }
}
