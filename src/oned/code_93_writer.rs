/*
 * Copyright 2015 ZXing authors
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

use one_d_proc_derive::OneDWriter;

use crate::BarcodeFormat;

use super::{Code93Reader, OneDimensionalCodeWriter};

/**
 * This object renders a CODE93 code as a BitMatrix
 */
#[derive(OneDWriter)]
pub struct Code93Writer;

impl Default for Code93Writer {
    fn default() -> Self {
        Self {}
    }
}

impl OneDimensionalCodeWriter for Code93Writer {
    /**
     * @param contents barcode contents to encode. It should not be encoded for extended characters.
     * @return a {@code boolean[]} of horizontal pixels (false = white, true = black)
     */
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>, Exceptions> {
        let mut contents = Self::convertToExtended(contents)?;
        let length = contents.chars().count();
        if length > 80 {
            return Err(Exceptions::IllegalArgumentException(format!("Requested contents should be less than 80 digits long after converting to extended encoding, but got {}" , length)));
        }

        //length of code + 2 start/stop characters + 2 checksums, each of 9 bits, plus a termination bar
        let codeWidth = (contents.chars().count() + 2 + 2) * 9 + 1;

        let mut result = vec![false; codeWidth];

        //start character (*)
        let mut pos = Self::appendPattern(&mut result, 0, Code93Reader::ASTERISK_ENCODING as u32);

        for i in 0..length {
            // for (int i = 0; i < length; i++) {
            let Some(indexInString) = Code93Reader::ALPHABET_STRING.find(contents.chars().nth(i).unwrap()) else {panic!("alphabet")};
            pos += Self::appendPattern(
                &mut result,
                pos,
                Code93Reader::CHARACTER_ENCODINGS[indexInString],
            );
        }

        //add two checksums
        let check1 = Self::computeChecksumIndex(&contents, 20);
        pos += Self::appendPattern(&mut result, pos, Code93Reader::CHARACTER_ENCODINGS[check1]);

        //append the contents to reflect the first checksum added
        contents.push(Code93Reader::ALPHABET_STRING.chars().nth(check1).unwrap());

        let check2 = Self::computeChecksumIndex(&contents, 15);
        pos += Self::appendPattern(&mut result, pos, Code93Reader::CHARACTER_ENCODINGS[check2]);

        //end character (*)
        pos += Self::appendPattern(&mut result, pos, Code93Reader::ASTERISK_ENCODING as u32);

        //termination bar (single black bar)
        result[pos] = true;

        Ok(result)
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::CODE_93])
    }
}

impl Code93Writer {
    /**
     * @param target output to append to
     * @param pos start position
     * @param pattern pattern to append
     * @param startColor unused
     * @return 9
     * @deprecated without replacement; intended as an internal-only method
     */
    #[allow(dead_code)]
    #[deprecated]
    fn appendPatternWithPatternStart(
        target: &mut [bool],
        pos: usize,
        pattern: &[usize],
        _startColor: bool,
    ) -> u32 {
        let mut pos = pos;
        for bit in pattern {
            // for (int bit : pattern) {
            target[pos] = *bit != 0;
            pos += 1;
            // target[pos++] = bit != 0;
        }

        9
    }

    fn appendPattern(target: &mut [bool], pos: usize, a: u32) -> usize {
        for i in 0..9 {
            // for (int i = 0; i < 9; i++) {
            let temp = a & (1 << (8 - i));
            target[pos + i] = temp != 0;
        }

        9
    }

    fn computeChecksumIndex(contents: &str, maxWeight: u32) -> usize {
        let mut weight = 1_u32;
        let mut total = 0_u32;

        for i in (0..contents.chars().count()).rev() {
            // for (int i = contents.length() - 1; i >= 0; i--) {
            let Some(indexInString) = Code93Reader::ALPHABET_STRING.find(contents.chars().nth(i).unwrap()) else {panic!("not in the alphabet");};
            total += indexInString as u32 * weight;
            weight += 1;
            if weight > maxWeight {
                weight = 1;
            }
        }

        total as usize % 47
    }

    fn convertToExtended(contents: &str) -> Result<String, Exceptions> {
        let length = contents.chars().count();
        let mut extendedContent = String::with_capacity(length * 2);
        for character in contents.chars() {
            // for (int i = 0; i < length; i++) {
            //   char character = contents.charAt(i);
            // ($)=a, (%)=b, (/)=c, (+)=d. see Code93Reader.ALPHABET_STRING
            if character as u32 == 0 {
                // NUL: (%)U
                extendedContent.push_str("bU");
            } else if character as u32 <= 26 {
                // SOH - SUB: ($)A - ($)Z
                extendedContent.push('a');
                extendedContent.push(char::from_u32('A' as u32 + character as u32 - 1).unwrap());
            } else if character as u32 <= 31 {
                // ESC - US: (%)A - (%)E
                extendedContent.push('b');
                extendedContent.push(char::from_u32('A' as u32 + character as u32 - 27).unwrap());
            } else if character == ' ' || character == '$' || character == '%' || character == '+' {
                // space $ % +
                extendedContent.push(character);
            } else if character <= ',' {
                // ! " # & ' ( ) * ,: (/)A - (/)L
                extendedContent.push('c');
                extendedContent
                    .push(char::from_u32('A' as u32 + character as u32 - '!' as u32).unwrap());
            } else if character <= '9' {
                extendedContent.push(character);
            } else if character == ':' {
                // :: (/)Z
                extendedContent.push_str("cZ");
            } else if character <= '?' {
                // ; - ?: (%)F - (%)J
                extendedContent.push('b');
                extendedContent
                    .push(char::from_u32('F' as u32 + character as u32 - ';' as u32).unwrap());
            } else if character == '@' {
                // @: (%)V
                extendedContent.push_str("bV");
            } else if character <= 'Z' {
                // A - Z
                extendedContent.push(character);
            } else if character <= '_' {
                // [ - _: (%)K - (%)O
                extendedContent.push('b');
                extendedContent
                    .push(char::from_u32('K' as u32 + character as u32 - '[' as u32).unwrap());
            } else if character == '`' {
                // `: (%)W
                extendedContent.push_str("bW");
            } else if character <= 'z' {
                // a - z: (*)A - (*)Z
                extendedContent.push('d');
                extendedContent
                    .push(char::from_u32('A' as u32 + character as u32 - 'a' as u32).unwrap());
            } else if character as u32 <= 127 {
                // { - DEL: (%)P - (%)T
                extendedContent.push('b');
                extendedContent
                    .push(char::from_u32('P' as u32 + character as u32 - '{' as u32).unwrap());
            } else {
                return Err(Exceptions::IllegalArgumentException(format!(
                    "Requested content contains a non-encodable character: '{}'",
                    character
                )));
            }
        }

        Ok(extendedContent)
    }
}

/**
 * Tests {@link Code93Writer}.
 */
#[cfg(test)]
mod Code93WriterTestCase {
    use crate::{common::BitMatrixTestCase, oned::Code93Writer, BarcodeFormat, Writer};

    #[test]
    fn testEncode() {
        doTest(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            "000001010111101101010001101001001101000101100101001100100101100010101011010001011001\
001011000101001101001000110101010110001010011001010001101001011001000101101101101001\
101100101101011001101001101100101101100110101011011001011001101001101101001110101000\
101001010010001010001001010000101001010001001001001001000101010100001000100101000010\
10100111010101000010101011110100000",
        );

        doTest("\u{0000}\u{0001}\u{001a}\u{001b}\u{001f} $%+!,09:;@AZ[_`az{\u{007f}",
           &format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", 
           "00000" , "101011110" ,
           "111011010" , "110010110" , "100100110" , "110101000" ,  // bU aA
           "100100110" , "100111010" , "111011010" , "110101000" ,  // aZ bA
           "111011010" , "110010010" , "111010010" , "111001010" ,  // bE space $
           "110101110" , "101110110" , "111010110" , "110101000" ,  // % + cA
           "111010110" , "101011000" , "100010100" , "100001010" ,  // cL 0 9
           "111010110" , "100111010" , "111011010" , "110001010" ,  // cZ bF
           "111011010" , "110011010" , "110101000" , "100111010" ,  // bV A Z
           "111011010" , "100011010" , "111011010" , "100101100" ,  // bK bO
           "111011010" , "101101100" , "100110010" , "110101000" ,  // bW dA
           "100110010" , "100111010" , "111011010" , "100010110" ,  // dZ bP
           "111011010" , "110100110" ,  // bT
           "110100010" , "110101100" ,  // checksum: 12 28
           "101011110" , "100000"));
    }

    fn doTest(input: &str, expected: &str) {
        let result = Code93Writer::default()
            .encode(input, &BarcodeFormat::CODE_93, 0, 0)
            .expect("encode");
        assert_eq!(expected, BitMatrixTestCase::matrix_to_string(&result));
    }

    #[test]
    fn testConvertToExtended() {
        // non-extended chars are not changed.
        let src = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%";
        let dst = Code93Writer::convertToExtended(src).expect("convert");
        assert_eq!(src, dst);
    }
}
