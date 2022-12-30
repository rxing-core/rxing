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

/**
 * Tests {@link Code128Writer}.
 */

const FNC1: &str = "11110101110";
const FNC2: &str = "11110101000";
const FNC3: &str = "10111100010";
const FNC4A: &str = "11101011110";
const FNC4B: &str = "10111101110";
const START_CODE_A: &str = "11010000100";
const START_CODE_B: &str = "11010010000";
const START_CODE_C: &str = "11010011100";
const SWITCH_CODE_A: &str = "11101011110";
const SWITCH_CODE_B: &str = "10111101110";
const QUIET_SPACE: &str = "00000";
const STOP: &str = "1100011101011";
const LF: &str = "10000110010";

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{
    common::{BitArray, BitMatrix, BitMatrixTestCase},
    oned::{Code128Reader, OneDReader},
    BarcodeFormat, EncodeHintType, EncodeHintValue, EncodingHintDictionary, Exceptions, Writer,
};

use super::Code128Writer;
lazy_static! {
    static ref WRITER: Code128Writer = Code128Writer::default();
}

#[test]
fn testEncodeWithFunc3() {
    let toEncode = "\u{00f3}123";
    let expected = format!(
        "{}{}{}{}{}{}{}{}{}",
        QUIET_SPACE,
        START_CODE_B,
        FNC3,
        // "1"            "2"             "3"            check digit 51
        "10011100110",
        "11001110010",
        "11001011100",
        "11101000110",
        STOP,
        QUIET_SPACE
    );

    let result = encode(toEncode, false, "123").expect("encode");

    let actual = BitMatrixTestCase::matrix_to_string(&result);
    assert_eq!(expected, actual);

    let width = result.getWidth();
    let result = encode(toEncode, true, "123").expect("encode");

    assert_eq!(width, result.getWidth());
}

#[test]
fn testEncodeWithFunc2() {
    let toEncode = "\u{00f2}123";
    let expected = format!(
        "{}{}{}{}{}{}{}{}{}",
        QUIET_SPACE,
        START_CODE_B,
        FNC2,
        // "1"            "2"             "3"             check digit 56
        "10011100110",
        "11001110010",
        "11001011100",
        "11100010110",
        STOP,
        QUIET_SPACE
    );

    let result = encode(toEncode, false, "123").expect("encode");

    let actual = BitMatrixTestCase::matrix_to_string(&result);
    assert_eq!(expected, actual);

    let width = result.getWidth();
    let result = encode(toEncode, true, "123").expect("encode");

    assert_eq!(width, result.getWidth());
}

#[test]
fn testEncodeWithFunc1() {
    let toEncode = "\u{00f1}123";
    let expected = format!(
        "{}{}{}{}{}{}{}{}{}",
        QUIET_SPACE,
        START_CODE_C,
        FNC1,
        // "12"                           "3"            check digit 92
        "10110011100",
        SWITCH_CODE_B,
        "11001011100",
        "10101111000",
        STOP,
        QUIET_SPACE
    );

    let result = encode(toEncode, false, "123").expect("encode");

    let actual = BitMatrixTestCase::matrix_to_string(&result);
    assert_eq!(expected, actual);

    let width = result.getWidth();
    let result = encode(toEncode, true, "123").expect("encode");

    assert_eq!(width, result.getWidth());
}

#[test]
fn testRoundtrip() {
    let toEncode = concat!("\u{00f1}", "10958", "\u{00f1}", "17160526");
    let expected = "1095817160526";

    let encRXingResult = encode(toEncode, false, expected).expect("encode");

    let width = encRXingResult.getWidth();
    let encRXingResult = encode(toEncode, true, expected).expect("encode");
    //Compact encoding has one latch less and encodes as STARTA,FNC1,1,CODEC,09,58,FNC1,17,16,05,26
    assert_eq!(width, encRXingResult.getWidth() + 11);
}

#[test]
fn testLongCompact() {
    //test longest possible input
    let toEncode =
        "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    encode(toEncode, true, toEncode).expect("encode");
}

#[test]
fn testShift() {
    //compare fast to compact
    let toEncode = "a\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\na\n";
    let result = encode(toEncode, false, toEncode).expect("encode");

    let width = result.getWidth();
    let result = encode(toEncode, true, toEncode).expect("encode");

    //big difference since the fast algoritm doesn't make use of SHIFT
    assert_eq!(width, result.getWidth() + 253);
}

#[test]
fn testDigitMixCompaction() {
    //compare fast to compact
    let toEncode = "A1A12A123A1234A12345AA1AA12AA123AA1234AA1235";
    let result = encode(toEncode, false, toEncode).expect("encode");

    let width = result.getWidth();
    let result = encode(toEncode, true, toEncode).expect("encode");

    //very good, no difference
    assert_eq!(width, result.getWidth());
}

#[test]
fn testCompaction1() {
    //compare fast to compact
    let toEncode = "AAAAAAAAAAA12AAAAAAAAA";
    let result = encode(toEncode, false, toEncode).expect("encode");

    let width = result.getWidth();
    let result = encode(toEncode, true, toEncode).expect("encode");

    //very good, no difference
    assert_eq!(width, result.getWidth());
}

#[test]
fn testCompaction2() {
    //compare fast to compact
    let toEncode = "AAAAAAAAAAA1212aaaaaaaaa";
    let result = encode(toEncode, false, toEncode).expect("encode");

    let width = result.getWidth();
    let result = encode(toEncode, true, toEncode).expect("encode");

    //very good, no difference
    assert_eq!(width, result.getWidth());
}

#[test]
fn testEncodeWithFunc4() {
    let toEncode = concat!("\u{00f4}", "123");
    let expected = format!(
        "{}{}{}{}{}{}{}{}{}",
        QUIET_SPACE,
        START_CODE_B,
        FNC4B,
        // "1"            "2"             "3"            check digit 59
        "10011100110",
        "11001110010",
        "11001011100",
        "11100011010",
        STOP,
        QUIET_SPACE
    );

    let result = encode(toEncode, false, "").expect("encode");

    let actual = BitMatrixTestCase::matrix_to_string(&result);
    assert_eq!(expected, actual);

    let width = result.getWidth();
    let result = encode(toEncode, true, "").expect("encode");
    assert_eq!(width, result.getWidth());
}

#[test]
fn testEncodeWithFncsAndNumberInCodesetA() {
    let toEncode = concat!("\n", "\u{00f1}", "\u{00f4}", "1", "\n");

    let expected = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        QUIET_SPACE,
        START_CODE_A,
        LF,
        FNC1,
        FNC4A,
        "10011100110",
        LF,
        "10101111000",
        STOP,
        QUIET_SPACE
    );

    let result = encode(toEncode, false, "").expect("encode");

    let actual = BitMatrixTestCase::matrix_to_string(&result);

    assert_eq!(expected, actual);

    let width = result.getWidth();
    let result = encode(toEncode, true, "").expect("encode");
    assert_eq!(width, result.getWidth());
}

#[test]
fn testEncodeSwitchBetweenCodesetsAAndB() {
    // start with A switch to B and back to A
    testEncode(
        "\0ABab\u{0010}",
        &format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}",
            QUIET_SPACE,
            START_CODE_A,
            // "\0"            "A"             "B"             Switch to B     "a"             "b"
            "10100001100",
            "10100011000",
            "10001011000",
            SWITCH_CODE_B,
            "10010110000",
            "10010000110",
            // Switch to A    "\u0010"        check digit
            SWITCH_CODE_A,
            "10100111100",
            "11001110100",
            STOP,
            QUIET_SPACE
        ),
    );

    // start with B switch to A and back to B
    // the compact encoder encodes this shorter as STARTB,a,b,SHIFT,NUL,a,b
    testEncode(
        "ab\0ab",
        &format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}",
            QUIET_SPACE,
            START_CODE_B,
            //  "a"             "b"            Switch to A     "\0"           Switch to B
            "10010110000",
            "10010000110",
            SWITCH_CODE_A,
            "10100001100",
            SWITCH_CODE_B,
            //  "a"             "b"            check digit
            "10010110000",
            "10010000110",
            "11010001110",
            STOP,
            QUIET_SPACE
        ),
    );
}

fn testEncode(toEncode: &str, expected: &str) {
    let result = encode(toEncode, false, toEncode).expect("encode");
    let actual = BitMatrixTestCase::matrix_to_string(&result);
    assert_eq!(expected, actual, "{}", toEncode);

    let width = result.getWidth();
    let result = encode(toEncode, true, toEncode).expect("encode");
    assert!(result.getWidth() <= width);
}

#[test]
#[should_panic]
fn testEncodeWithForcedCodeSetFailureCodeSetABadCharacter() {
    // Lower case characters should not be accepted when the code set is forced to A.
    let toEncode = "ASDFx0123";

    let mut hints = HashMap::new(); //new EnumMap<>(EncodeHintType.class);
    hints.insert(
        EncodeHintType::FORCE_CODE_SET,
        EncodeHintValue::ForceCodeSet("A".to_string()),
    );
    WRITER
        .encode_with_hints(toEncode, &BarcodeFormat::CODE_128, 0, 0, &hints)
        .expect("encode");
}

#[test]
#[should_panic]
fn testEncodeWithForcedCodeSetFailureCodeSetBBadCharacter() {
    let toEncode = "ASdf\00123"; // \0 (ascii value 0)
                                 // Characters with ASCII value below 32 should not be accepted when the code set is forced to B.

    let mut hints = HashMap::new(); //new EnumMap<>(EncodeHintType.class);
    hints.insert(
        EncodeHintType::FORCE_CODE_SET,
        EncodeHintValue::ForceCodeSet("B".to_string()),
    );
    // let  hints = new EnumMap<>(EncodeHintType.class);
    // hints.put(EncodeHintType.FORCE_CODE_SET, "B");
    WRITER
        .encode_with_hints(toEncode, &BarcodeFormat::CODE_128, 0, 0, &hints)
        .expect("encode");
}

#[test]
#[should_panic]
fn testEncodeWithForcedCodeSetFailureCodeSetCBadCharactersNonNum() {
    let toEncode = "123a5678";
    // Non-digit characters should not be accepted when the code set is forced to C.

    let mut hints = HashMap::new(); //new EnumMap<>(EncodeHintType.class);
    hints.insert(
        EncodeHintType::FORCE_CODE_SET,
        EncodeHintValue::ForceCodeSet("C".to_string()),
    );
    // let  hints = new EnumMap<>(EncodeHintType.class);
    // hints.put(EncodeHintType.FORCE_CODE_SET, "C");
    WRITER
        .encode_with_hints(toEncode, &BarcodeFormat::CODE_128, 0, 0, &hints)
        .expect("encode");
}

#[test]
#[should_panic]
fn testEncodeWithForcedCodeSetFailureCodeSetCBadCharactersFncCode() {
    let toEncode = "123\u{00f2}a678";
    // Function codes other than 1 should not be accepted when the code set is forced to C.

    let mut hints = HashMap::new(); //new EnumMap<>(EncodeHintType.class);
    hints.insert(
        EncodeHintType::FORCE_CODE_SET,
        EncodeHintValue::ForceCodeSet("C".to_string()),
    );
    WRITER
        .encode_with_hints(toEncode, &BarcodeFormat::CODE_128, 0, 0, &hints)
        .expect("encode");
}

#[test]
#[should_panic]
fn testEncodeWithForcedCodeSetFailureCodeSetCWrongAmountOfDigits() {
    let toEncode = "123456789";
    // An uneven amount of digits should not be accepted when the code set is forced to C.

    let mut hints = HashMap::new(); //new EnumMap<>(EncodeHintType.class);
    hints.insert(
        EncodeHintType::FORCE_CODE_SET,
        EncodeHintValue::ForceCodeSet("C".to_string()),
    );
    WRITER
        .encode_with_hints(toEncode, &BarcodeFormat::CODE_128, 0, 0, &hints)
        .expect("encode");
}

#[test]
fn testEncodeWithForcedCodeSetFailureCodeSetA() {
    let toEncode = "AB123";
    //                          would default to B   "A"             "B"             "1"
    let expected = format!(
        "{}{}{}{}{}{}{}{}{}{}",
        QUIET_SPACE,
        START_CODE_A,
        "10100011000",
        "10001011000",
        "10011100110",
        // "2"             "3"           check digit 10
        "11001110010",
        "11001011100",
        "11001000100",
        STOP,
        QUIET_SPACE
    );

    //     let  hints = new EnumMap<>(EncodeHintType.class);
    // hints.put(EncodeHintType.FORCE_CODE_SET, "A");
    let mut hints = HashMap::new(); //new EnumMap<>(EncodeHintType.class);
    hints.insert(
        EncodeHintType::FORCE_CODE_SET,
        EncodeHintValue::ForceCodeSet("A".to_string()),
    );
    let result = WRITER
        .encode_with_hints(toEncode, &BarcodeFormat::CODE_128, 0, 0, &hints)
        .expect("encode");

    let actual = BitMatrixTestCase::matrix_to_string(&result);
    assert_eq!(expected, actual);
}

#[test]
fn testEncodeWithForcedCodeSetFailureCodeSetB() {
    let toEncode = "1234";
    //                          would default to C   "1"             "2"             "3"
    let expected = format!(
        "{}{}{}{}{}{}{}{}{}",
        QUIET_SPACE,
        START_CODE_B,
        "10011100110",
        "11001110010",
        "11001011100",
        // "4"           check digit 88
        "11001001110",
        "11110010010",
        STOP,
        QUIET_SPACE
    );

    //     let  hints = new EnumMap<>(EncodeHintType.class);
    // hints.put(EncodeHintType.FORCE_CODE_SET, "B");
    let mut hints = HashMap::new(); //new EnumMap<>(EncodeHintType.class);
    hints.insert(
        EncodeHintType::FORCE_CODE_SET,
        EncodeHintValue::ForceCodeSet("B".to_string()),
    );
    let result = WRITER
        .encode_with_hints(toEncode, &BarcodeFormat::CODE_128, 0, 0, &hints)
        .expect("encode");

    let actual = BitMatrixTestCase::matrix_to_string(&result);
    assert_eq!(expected, actual);
}

fn encode(toEncode: &str, compact: bool, expectedLoopback: &str) -> Result<BitMatrix, Exceptions> {
    let mut reader = Code128Reader::default();

    let mut hints: EncodingHintDictionary = HashMap::new();
    if compact {
        hints.insert(
            EncodeHintType::CODE128_COMPACT,
            EncodeHintValue::Code128Compact(true),
        );
    }
    let encRXingResult =
        WRITER.encode_with_hints(toEncode, &BarcodeFormat::CODE_128, 0, 0, &hints)?;
    if !expectedLoopback.is_empty() {
        let row = encRXingResult.getRow(0, BitArray::new());
        let rtRXingResult = reader.decodeRow(0, &row, &HashMap::new())?;
        let actual = rtRXingResult.getText();
        assert_eq!(expectedLoopback, actual);
    }
    if compact {
        //check that what is encoded compactly yields the same on loopback as what was encoded fast.
        let row = encRXingResult.getRow(0, BitArray::new());
        let rtRXingResult = reader.decodeRow(0, &row, &HashMap::new())?;
        let actual = rtRXingResult.getText();
        let encRXingResultFast = WRITER.encode(toEncode, &BarcodeFormat::CODE_128, 0, 0)?;
        let row = encRXingResultFast.getRow(0, BitArray::new());
        let rtRXingResult = reader.decodeRow(0, &row, &HashMap::new())?;
        assert_eq!(rtRXingResult.getText(), actual);
    }
    Ok(encRXingResult)
}
