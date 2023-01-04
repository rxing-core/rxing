#![allow(deprecated)]
/*
 * Copyright 2009 ZXing authors
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

use encoding::{Encoding, EncodingRef};
use java_rand;

use crate::pdf417::decoder::decoded_bit_stream_parser;
use crate::pdf417::encoder::{pdf_417_high_level_encoder_test_adapter, Compaction};
use crate::pdf417::PDF417RXingResultMetadata;

/**
 * Tests {@link DecodedBitStreamParser}.
 */

/**
 * Tests the first sample given in ISO/IEC 15438:2015(E) - Annex H.4
 */
#[test]
fn testStandardSample1() {
    let mut resultMetadata = PDF417RXingResultMetadata::default();
    let sampleCodes: [u32; 23] = [
        20, 928, 111, 100, 17, 53, 923, 1, 111, 104, 923, 3, 64, 416, 34, 923, 4, 258, 446, 67,
        // we should never reach these
        1000, 1000, 1000,
    ];

    decoded_bit_stream_parser::decodeMacroBlock(&sampleCodes, 2, &mut resultMetadata)
        .expect("decode");

    assert_eq!(0, resultMetadata.getSegmentIndex());
    assert_eq!("017053", resultMetadata.getFileId());
    assert!(!resultMetadata.isLastSegment());
    assert_eq!(4, resultMetadata.getSegmentCount());
    assert_eq!("CEN BE", resultMetadata.getSender());
    assert_eq!("ISO CH", resultMetadata.getAddressee());

    let optionalData = resultMetadata.getOptionalData();
    assert_eq!(
        1, optionalData[0],
        "first element of optional array should be the first field identifier"
    );
    assert_eq!(
        67,
        optionalData[optionalData.len() - 1],
        "last element of optional array should be the last codeword of the last field"
    );
}

/**
 * Tests the second given in ISO/IEC 15438:2015(E) - Annex H.4
 */
#[test]
fn testStandardSample2() {
    let mut resultMetadata = PDF417RXingResultMetadata::default();
    let sampleCodes: [u32; 14] = [
        11, 928, 111, 103, 17, 53, 923, 1, 111, 104, 922, // we should never reach these
        1000, 1000, 1000,
    ];

    decoded_bit_stream_parser::decodeMacroBlock(&sampleCodes, 2, &mut resultMetadata)
        .expect("decode");

    assert_eq!(3, resultMetadata.getSegmentIndex());
    assert_eq!("017053", resultMetadata.getFileId());
    assert!(resultMetadata.isLastSegment());
    assert_eq!(4, resultMetadata.getSegmentCount());
    assert!(resultMetadata.getAddressee().is_empty());
    assert!(resultMetadata.getSender().is_empty());

    let optionalData = resultMetadata.getOptionalData();
    assert_eq!(
        1, optionalData[0],
        "first element of optional array should be the first field identifier"
    );
    assert_eq!(
        104,
        optionalData[optionalData.len() - 1],
        "last element of optional array should be the last codeword of the last field"
    );
}

/**
 * Tests the example given in ISO/IEC 15438:2015(E) - Annex H.6
 */
#[test]
fn testStandardSample3() {
    let mut resultMetadata = PDF417RXingResultMetadata::default();
    let sampleCodes = [7_u32, 928, 111, 100, 100, 200, 300, 0]; // Final dummy ECC codeword required to avoid ArrayIndexOutOfBounds

    decoded_bit_stream_parser::decodeMacroBlock(&sampleCodes, 2, &mut resultMetadata)
        .expect("decode");

    assert_eq!(0, resultMetadata.getSegmentIndex());
    assert_eq!("100200300", resultMetadata.getFileId());
    assert!(!resultMetadata.isLastSegment());
    assert_eq!(-1, resultMetadata.getSegmentCount());
    assert!(resultMetadata.getAddressee().is_empty());
    assert!(resultMetadata.getSender().is_empty());
    assert!(resultMetadata.getOptionalData().is_empty());

    // Check that symbol containing no data except Macro is accepted (see note in Annex H.2)
    let decoderRXingResult = decoded_bit_stream_parser::decode(&sampleCodes, "0").expect("decode");
    assert_eq!("", decoderRXingResult.getText());
    assert!(decoderRXingResult.getOther().is_some());
}

#[test]
fn testSampleWithFilename() {
    let sampleCodes = [
        23_u32, 477, 928, 111, 100, 0, 252, 21, 86, 923, 0, 815, 251, 133, 12, 148, 537, 593, 599,
        923, 1, 111, 102, 98, 311, 355, 522, 920, 779, 40, 628, 33, 749, 267, 506, 213, 928, 465,
        248, 493, 72, 780, 699, 780, 493, 755, 84, 198, 628, 368, 156, 198, 809, 19, 113,
    ];
    let mut resultMetadata = PDF417RXingResultMetadata::default();

    decoded_bit_stream_parser::decodeMacroBlock(&sampleCodes, 3, &mut resultMetadata)
        .expect("decode");

    assert_eq!(0, resultMetadata.getSegmentIndex());
    assert_eq!("000252021086", resultMetadata.getFileId());
    assert!(!resultMetadata.isLastSegment());
    assert_eq!(2, resultMetadata.getSegmentCount());
    assert!(resultMetadata.getAddressee().is_empty());
    assert!(resultMetadata.getSender().is_empty());
    assert_eq!("filename.txt", resultMetadata.getFileName());
}

#[test]
fn testSampleWithNumericValues() {
    let sampleCodes = [
        25_u32, 477, 928, 111, 100, 0, 252, 21, 86, 923, 2, 2, 0, 1, 0, 0, 0, 923, 5, 130, 923, 6,
        1, 500, 13, 0,
    ];
    let mut resultMetadata = PDF417RXingResultMetadata::default();

    decoded_bit_stream_parser::decodeMacroBlock(&sampleCodes, 3, &mut resultMetadata)
        .expect("decode");

    assert_eq!(0, resultMetadata.getSegmentIndex());
    assert_eq!("000252021086", resultMetadata.getFileId());
    assert!(!resultMetadata.isLastSegment());

    assert_eq!(180980729000000, resultMetadata.getTimestamp());
    assert_eq!(30, resultMetadata.getFileSize());
    assert_eq!(260013, resultMetadata.getChecksum());
}

#[test]
fn testSampleWithMacroTerminatorOnly() {
    let sampleCodes = [7_u32, 477, 928, 222, 198, 0, 922];
    let mut resultMetadata = PDF417RXingResultMetadata::default();

    decoded_bit_stream_parser::decodeMacroBlock(&sampleCodes, 3, &mut resultMetadata)
        .expect("decode");

    assert_eq!(99998, resultMetadata.getSegmentIndex());
    assert_eq!("000", resultMetadata.getFileId());
    assert!(resultMetadata.isLastSegment());
    assert_eq!(-1, resultMetadata.getSegmentCount());
    assert!(resultMetadata.getOptionalData().is_empty());
}

#[test]
#[should_panic]
fn testSampleWithBadSequenceIndexMacro() {
    let sampleCodes = [3_u32, 928, 222, 0];
    let mut resultMetadata = PDF417RXingResultMetadata::default();
    decoded_bit_stream_parser::decodeMacroBlock(&sampleCodes, 2, &mut resultMetadata)
        .expect("decode");
}

#[test]
#[should_panic]
fn testSampleWithNoFileIdMacro() {
    let sampleCodes = [4_u32, 928, 222, 198, 0];
    let mut resultMetadata = PDF417RXingResultMetadata::default();
    decoded_bit_stream_parser::decodeMacroBlock(&sampleCodes, 2, &mut resultMetadata)
        .expect("decode");
}

#[test]
#[should_panic]
fn testSampleWithNoDataNoMacro() {
    let sampleCodes = [3_u32, 899, 899, 0];
    decoded_bit_stream_parser::decode(&sampleCodes, "0").expect("decode");
}

#[test]
fn testUppercase() {
    //encodeDecode("", 0);
    performEncodeTest('A', &[3, 4, 5, 6, 4, 4, 5, 5]);
}

#[test]
fn testNumeric() {
    performEncodeTest(
        '1',
        &[
            2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10,
        ],
    );
}

#[test]
fn testByte() {
    performEncodeTest('\u{00c4}', &[3, 4, 5, 6, 7, 7, 8]);
}

#[test]
fn testUppercaseLowercaseMix1() {
    encodeDecodeWithLength("aA", 4);
    encodeDecodeWithLength("aAa", 5);
    encodeDecodeWithLength("Aa", 4);
    encodeDecodeWithLength("Aaa", 5);
    encodeDecodeWithLength("AaA", 5);
    encodeDecodeWithLength("AaaA", 6);
    encodeDecodeWithLength("Aaaa", 6);
    encodeDecodeWithLength("AaAaA", 5);
    encodeDecodeWithLength("AaaAaaA", 6);
    encodeDecodeWithLength("AaaAAaaA", 7);
}

#[test]
fn testPunctuation() {
    performEncodeTest(';', &[3, 4, 5, 6, 6, 7, 8]);
    encodeDecodeWithLength(";;;;;;;;;;;;;;;;", 17);
}

#[test]
fn testUppercaseLowercaseMix2() {
    performPermutationTest(&['A', 'a'], 10, 8972);
}

#[test]
fn testUppercaseNumericMix() {
    performPermutationTest(&['A', '1'], 14, 192510);
}

#[test]
fn testUppercaseMixedMix() {
    performPermutationTest(&['A', '1', ' ', ';'], 7, 106060);
}

#[test]
fn testUppercasePunctuationMix() {
    performPermutationTest(&['A', ';'], 10, 8967);
}

#[test]
fn testUppercaseByteMix() {
    performPermutationTest(&['A', '\u{00c4}'], 10, 11222);
}

#[test]
fn testLowercaseByteMix() {
    performPermutationTest(&['a', '\u{00c4}'], 10, 11233);
}

#[test]
fn testUppercaseLowercaseNumericMix() {
    performPermutationTest(&['A', 'a', '1'], 7, 15491);
}

#[test]
fn testUppercaseLowercasePunctuationMix() {
    performPermutationTest(&['A', 'a', ';'], 7, 15491);
}

#[test]
fn testUppercaseLowercaseByteMix() {
    performPermutationTest(&['A', 'a', '\u{00c4}'], 7, 17288);
}

#[test]
fn testLowercasePunctuationByteMix() {
    performPermutationTest(&['a', ';', '\u{00c4}'], 7, 17427);
}

#[test]
fn testUppercaseLowercaseNumericPunctuationMix() {
    performPermutationTest(&['A', 'a', '1', ';'], 7, 120479);
}

#[test]
fn testBinaryData() {
    let mut bytes = [0_u8; 500];
    // let random = rand::thread_rng();
    let mut random = java_rand::Random::new(0);
    let mut total = 0;
    for _i in 0..10000 {
        // for (int i = 0; i < 10000; i++) {
        // let bytes = gen_500_random_bytes();
        random.next_bytes(&mut bytes);

        total += encodeDecode(
            &encoding::all::ISO_8859_1
                .decode(&bytes, encoding::DecoderTrap::Strict)
                .expect("decode bytes"),
        );
    }
    assert_eq!(4190044, total);
}

// fn gen_500_random_bytes() -> [u8;500] {
//   let mut bytes = [0_u8;500];
//   let mut random = rand::thread_rng();
//   for i in 0..500 {
//     bytes[i] = random.gen();
//   }
//   bytes
// }

#[test]
fn testECIEnglishHiragana() {
    //multi ECI UTF-8, UTF-16 and ISO-8859-1
    performECITest(
        &['a', '1', '\u{3040}'],
        &mut [20.0, 1.0, 10.0],
        105825,
        110914,
    );
}

#[test]
fn testECIEnglishKatakana() {
    //multi ECI UTF-8, UTF-16 and ISO-8859-1
    performECITest(
        &['a', '1', '\u{30a0}'],
        &mut [20.0, 1.0, 10.0],
        109177,
        110914,
    );
}

#[test]
fn testECIEnglishHalfWidthKatakana() {
    //single ECI
    performECITest(
        &['a', '1', '\u{ff80}'],
        &mut [20.0, 1.0, 10.0],
        80617,
        110914,
    );
}

#[test]
fn testECIEnglishChinese() {
    //single ECI
    performECITest(
        &['a', '1', '\u{4e00}'],
        &mut [20.0, 1.0, 10.0],
        95797,
        110914,
    );
}

#[test]
fn testECIGermanCyrillic() {
    //single ECI since the German Umlaut is in ISO-8859-1
    performECITest(
        &['a', '1', '\u{00c4}', '\u{042f}'],
        &mut [20.0, 1.0, 1.0, 10.0],
        80755,
        96007,
    );
}

#[test]
fn testECIEnglishCzechCyrillic1() {
    //multi ECI between ISO-8859-2 and ISO-8859-5
    performECITest(
        &['a', '1', '\u{010c}', '\u{042f}'],
        &mut [10.0, 1.0, 10.0, 10.0],
        102824,
        124525,
    );
}

#[test]
fn testECIEnglishCzechCyrillic2() {
    //multi ECI between ISO-8859-2 and ISO-8859-5
    performECITest(
        &['a', '1', '\u{010c}', '\u{042f}'],
        &mut [40.0, 1.0, 10.0, 10.0],
        81321,
        88236,
    );
}

#[test]
fn testECIEnglishArabicCyrillic() {
    //multi ECI between UTF-8 (ISO-8859-6 is excluded in CharacterSetECI) and ISO-8859-5
    performECITest(
        &['a', '1', '\u{0620}', '\u{042f}'],
        &mut [10.0, 1.0, 10.0, 10.0],
        118510,
        124525,
    );
}

#[test]
fn testBinaryMultiECI() {
    //Test the cases described in 5.5.5.3 "ECI and Byte Compaction mode using latch 924 and 901"
    performDecodeTest(&[5, 927, 4, 913, 200], "\u{010c}");
    performDecodeTest(&[9, 927, 4, 913, 200, 927, 7, 913, 207], "\u{010c}\u{042f}");
    performDecodeTest(&[9, 927, 4, 901, 200, 927, 7, 901, 207], "\u{010c}\u{042f}");
    performDecodeTest(&[8, 927, 4, 901, 200, 927, 7, 207], "\u{010c}\u{042f}");
    performDecodeTest(
        &[14, 927, 4, 901, 200, 927, 7, 207, 927, 4, 200, 927, 7, 207],
        "\u{010c}\u{042f}\u{010c}\u{042f}",
    );
    performDecodeTest(&[16, 927, 4, 924, 336, 432, 197, 51, 300, 927, 7, 348, 231, 311, 858, 567],
        "\u{010c}\u{010c}\u{010c}\u{010c}\u{010c}\u{010c}\u{042f}\u{042f}\u{042f}\u{042f}\u{042f}\u{042f}");
}

fn encodeDecodeWithLength(input: &str, expectedLength: u32) {
    assert_eq!(expectedLength, encodeDecode(input));
}

fn encodeDecode(input: &str) -> u32 {
    encodeDecodeWithAll(input, None, false, true)
}

fn encodeDecodeWithAll(
    input: &str,
    charset: Option<EncodingRef>,
    autoECI: bool,
    decode: bool,
) -> u32 {
    let s = pdf_417_high_level_encoder_test_adapter::encodeHighLevel(
        input,
        Compaction::AUTO,
        charset,
        autoECI,
    )
    .expect("encode");
    if decode {
        let mut codewords = vec![0_u32; s.chars().count() + 1];
        codewords[0] = codewords.len() as u32;
        for (i, codeword) in codewords.iter_mut().enumerate().skip(1) {
            // for (int i = 1; i < codewords.length; i++) {
            *codeword = s.chars().nth(i - 1).unwrap() as u32;
        }
        performDecodeTest(&codewords, input);
    }
    s.chars().count() as u32 + 1
}

fn getEndIndex(length: u32, chars: &[char]) -> u32 {
    let decimalLength: f64 = (chars.len() as f64).log10(); //Math.log10(chars.length);
    10_f64.powf(decimalLength * length as f64).ceil() as u32
    // (decimalLength*length as f64).powi(10).ceil() as u32
    // (decimalLength*length as f64).powi(10).ceil() as u32
    //  Math.ceil(Math.pow(10, decimalLength * length))
}

fn generatePermutation(index: u32, length: u32, chars: &[char]) -> String {
    let N = chars.len();
    // let baseNNumber = Integer.toString(index, N);
    let mut baseNNumber = int_to_string(index, N);
    while baseNNumber.chars().count() < length as usize {
        baseNNumber.insert(0, '0');
        // baseNNumber = "0" + baseNNumber;
    }
    let mut prefix = String::from("");
    for ch in baseNNumber.chars() {
        prefix.push(chars[(ch as isize - '0' as isize) as usize]);
    }
    // for i in 0..baseNNumber.chars().count() {
    // // for (int i = 0; i < baseNNumber.length(); i++) {
    //   prefix += chars[baseNNumber.charAt(i) - '0'];
    // }

    prefix
}

fn performPermutationTest(chars: &[char], length: u32, expectedTotal: u32) {
    let endIndex = getEndIndex(length, chars);
    let mut total = 0;
    for i in 0..endIndex {
        // for (int i = 0; i < endIndex; i++) {
        total += encodeDecode(&generatePermutation(i, length, chars));
    }
    assert_eq!(expectedTotal, total);
}

fn performEncodeTest(c: char, expectedLengths: &[u32]) {
    for (i, epected_length) in expectedLengths.iter().enumerate() {
        let sb = vec![c; i + 1].into_iter().collect::<String>();
        encodeDecodeWithLength(&sb, *epected_length);
    }
}

fn performDecodeTest(codewords: &[u32], expectedRXingResult: &str) {
    let result = decoded_bit_stream_parser::decode(codewords, "0").expect("decode");
    assert_eq!(expectedRXingResult, result.getText());
}

fn performECITest(
    chars: &[char],
    weights: &mut [f32],
    expectedMinLength: u32,
    expectedUTFLength: u32,
) {
    let mut random = java_rand::Random::new(0);
    let mut minLength = 0;
    let mut utfLength = 0;
    for _i in 0..1000 {
        // for (int i = 0; i < 1000; i++) {
        let s = generateText(&mut random, 100, chars, weights);
        minLength += encodeDecodeWithAll(&s, None, true, true);
        utfLength += encodeDecodeWithAll(&s, Some(encoding::all::UTF_8), false, true);
    }
    assert_eq!(expectedMinLength, minLength);
    assert_eq!(expectedUTFLength, utfLength);
}

fn generateText(
    random: &mut java_rand::Random,
    maxWidth: u32,
    chars: &[char],
    weights: &mut [f32],
) -> String {
    let mut result = String::new(); //new StringBuilder();
    let maxWordWidth = 7;
    let mut total = 0.0;
    // for (int i = 0; i < weights.length; i++) {
    //   total += weights[i];
    // }
    total += weights.iter().sum::<f32>();

    for weight in weights.iter_mut() {
        // for (int i = 0; i < weights.length; i++) {
        *weight /= total;
    }
    let mut cnt = 0;
    loop {
        let mut maxValue = 0.0;
        let mut maxIndex = 0;
        for (j, weight) in weights.iter().enumerate() {
            // for (int j = 0; j < weights.length; j++) {
            let value = random.next_f32() * *weight;
            if value > maxValue {
                maxValue = value;
                maxIndex = j;
            }
        }
        let wordLength = maxWordWidth as f32 * random.next_f32();
        if wordLength > 0.0 && result.chars().count() > 0 {
            result.push(' ');
        }
        for j in 0..wordLength.ceil() as u32 {
            // for (int j = 0; j < wordLength; j++) {
            let mut c = chars[maxIndex];
            if j == 0 && ('a'..='z').contains(&c) && random.next_bool() {
                c = char::from_u32(c as u32 - 'a' as u32 + 'A' as u32).unwrap();
            }
            result.push(c);
        }
        if cnt % 2 != 0 && random.next_bool() {
            result.push('.');
        }
        cnt += 1;

        if result.chars().count() >= (maxWidth as isize - maxWordWidth as isize) as usize {
            break;
        }
    } //while (result.length() < maxWidth - maxWordWidth);

    result
}

fn int_to_string(x: u32, radix: usize) -> String {
    // Handle the special case of 0
    if x == 0 {
        return "0".to_string();
    }

    // Build the string by repeatedly dividing the number by the radix and
    // adding the remainder to the beginning of the string
    let mut s = String::new();
    let mut x = x as usize;
    while x > 0 {
        let remainder = (x % radix) as u8;
        s = (remainder).to_string() + &s;
        x /= radix;
    }

    s
}
