/*
 * Copyright 2010 ZXing authors
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

use rxing_one_d_proc_derive::OneDWriter;

use crate::BarcodeFormat;

use super::{Code39Reader, OneDimensionalCodeWriter};

/**
 * This object renders a CODE39 code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */
#[derive(OneDWriter, Default)]
pub struct Code39Writer;

impl OneDimensionalCodeWriter for Code39Writer {
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>, Exceptions> {
        let mut contents = contents.to_owned();
        let mut length = contents.chars().count();
        if length > 80 {
            return Err(Exceptions::illegalArgument(format!(
                "Requested contents should be less than 80 digits long, but got {length}"
            )));
        }

        let mut i = 0;
        while i < length {
            // for i in 0..length {
            // for (int i = 0; i < length; i++) {
            if Code39Reader::ALPHABET_STRING
                .find(
                    contents
                        .chars()
                        .nth(i)
                        .ok_or(Exceptions::indexOutOfBoundsEmpty())?,
                )
                .is_none()
            {
                contents = Self::tryToConvertToExtendedMode(&contents)?;
                length = contents.chars().count();
                if length > 80 {
                    return Err(Exceptions::illegalArgument(format!("Requested contents should be less than 80 digits long, but got {length} (extended full ASCII mode)")));
                }
                break;
            }
            i += 1;
        }

        let mut widths = [0_usize; 9]; //new int[9];
        let codeWidth = 24 + 1 + (13 * length);
        let mut result = vec![false; codeWidth];
        Self::toIntArray(Code39Reader::ASTERISK_ENCODING, &mut widths);
        let mut pos = Self::appendPattern(&mut result, 0, &widths, true);
        let narrowWhite = [1_usize];
        pos += Self::appendPattern(&mut result, pos as usize, &narrowWhite, false);
        //append next character to byte matrix
        for i in 0..length {
            let Some(indexInString) = Code39Reader::ALPHABET_STRING.find(contents.chars().nth(i).ok_or(Exceptions::indexOutOfBoundsEmpty())?) else {
              continue;
            };
            Self::toIntArray(
                Code39Reader::CHARACTER_ENCODINGS[indexInString],
                &mut widths,
            );
            pos += Self::appendPattern(&mut result, pos as usize, &widths, true);
            pos += Self::appendPattern(&mut result, pos as usize, &narrowWhite, false);
        }
        Self::toIntArray(Code39Reader::ASTERISK_ENCODING, &mut widths);
        Self::appendPattern(&mut result, pos as usize, &widths, true);

        Ok(result)
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::CODE_39])
    }
}
impl Code39Writer {
    fn toIntArray(a: u32, toReturn: &mut [usize; 9]) {
        for (i, val) in toReturn.iter_mut().enumerate().take(9) {
            // for (int i = 0; i < 9; i++) {
            let temp = a & (1 << (8 - i));
            *val = if temp == 0 { 1 } else { 2 };
        }
    }

    fn tryToConvertToExtendedMode(contents: &str) -> Result<String, Exceptions> {
        // let length = contents.chars().count();
        let mut extendedContent = String::new(); //new StringBuilder();
        for character in contents.chars() {
            // for (int i = 0; i < length; i++) {
            //   char character = contents.charAt(i);
            match character {
                '\u{0000}' => extendedContent.push_str("%U"),
                ' ' | '-' | '.' => extendedContent.push(character),
                '@' => extendedContent.push_str("%V"),

                '`' => extendedContent.push_str("%W"),

                _ => {
                    if (character as u32) <= 26 {
                        extendedContent.push('$');
                        extendedContent.push(
                            char::from_u32('A' as u32 + (character as u32 - 1))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else if character < ' ' {
                        extendedContent.push('%');
                        extendedContent.push(
                            char::from_u32('A' as u32 + (character as u32 - 27))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else if character <= ',' || character == '/' || character == ':' {
                        extendedContent.push('/');
                        extendedContent.push(
                            char::from_u32('A' as u32 + (character as u32 - 33))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else if character <= '9' {
                        extendedContent.push(
                            char::from_u32('0' as u32 + (character as u32 - 48))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else if character <= '?' {
                        extendedContent.push('%');
                        extendedContent.push(
                            char::from_u32('F' as u32 + (character as u32 - 59))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else if character <= 'Z' {
                        extendedContent.push(
                            char::from_u32('A' as u32 + (character as u32 - 65))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else if character <= '_' {
                        extendedContent.push('%');
                        extendedContent.push(
                            char::from_u32('K' as u32 + (character as u32 - 91))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else if character <= 'z' {
                        extendedContent.push('+');
                        extendedContent.push(
                            char::from_u32('A' as u32 + (character as u32 - 97))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else if character as u32 <= 127 {
                        extendedContent.push('%');
                        extendedContent.push(
                            char::from_u32('P' as u32 + (character as u32 - 123))
                                .ok_or(Exceptions::parseEmpty())?,
                        );
                    } else {
                        return Err(Exceptions::illegalArgument(format!(
                            "Requested content contains a non-encodable character: '{character}'"
                        )));
                    }
                }
            }
        }

        Ok(extendedContent)
    }
}

#[cfg(test)]

/**
 * Tests {@link Code39Writer}.
 */
mod Code39WriterTestCase {
    use crate::{common::bit_matrix_test_case, oned::Code39Writer, BarcodeFormat, Writer};

    #[test]
    fn testEncode() {
        doTest(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789",
            "000001001011011010110101001011010110100101101101101001010101011001011011010110010101\
011011001010101010011011011010100110101011010011010101011001101011010101001101011010\
100110110110101001010101101001101101011010010101101101001010101011001101101010110010\
101101011001010101101100101100101010110100110101011011001101010101001011010110110010\
110101010011011010101010011011010110100101011010110010101101101100101010101001101011\
01101001101010101100110101010100101101101101001011010101100101101010010110110100000",
        );

        // extended mode blocks
        doTest("\u{0000}\u{0001}\u{0002}\u{0003}\u{0004}\u{0005}\u{0006}\u{0007}\u{008}\t\n\u{000b}\u{000c}\r\u{000e}\u{000f}\u{0010}\u{0011}\u{0012}\u{0013}\u{0014}\u{0015}\u{0016}\u{0017}\u{0018}\u{0019}\u{001a}\u{001b}\u{001c}\u{001d}\u{001e}\u{001f}",
           "000001001011011010101001001001011001010101101001001001010110101001011010010010010101\
011010010110100100100101011011010010101001001001010101011001011010010010010101101011\
001010100100100101010110110010101001001001010101010011011010010010010101101010011010\
100100100101010110100110101001001001010101011001101010010010010101101010100110100100\
100101010110101001101001001001010110110101001010010010010101010110100110100100100101\
011010110100101001001001010101101101001010010010010101010101100110100100100101011010\
101100101001001001010101101011001010010010010101010110110010100100100101011001010101\
101001001001010100110101011010010010010101100110101010100100100101010010110101101001\
001001010110010110101010010010010101001101101010101001001001011010100101101010010010\
010101101001011010100100100101101101001010101001001001010101100101101010010010010110\
101100101010010110110100000");

        doTest(
            " !\"#$%&'()*+,-./0123456789:;<=>?",
            "000001001011011010100110101101010010010100101101010010110100100101001010110100101101\
001001010010110110100101010010010100101010110010110100100101001011010110010101001001\
010010101101100101010010010100101010100110110100100101001011010100110101001001010010\
101101001101010010010100101010110011010100100101001011010101001101001001010010101101\
010011010010101101101100101011010100100101001011010110100101010011011010110100101011\
010110010101101101100101010101001101011011010011010101011001101010101001011011011010\
010110101011001011010100100101001010011011010101010010010010101101100101010100100100\
101010100110110101001001001011010100110101010010010010101101001101010100100100101010\
11001101010010110110100000",
        );

        doTest(
            "@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_",
            "0000010010110110101010010010010100110101011011010100101101011010010110110110100101010\
101100101101101011001010101101100101010101001101101101010011010101101001101010101100\
110101101010100110101101010011011011010100101010110100110110101101001010110110100101\
010101100110110101011001010110101100101010110110010110010101011010011010101101100110\
101010100101101011011001011010101001101101010101001001001011010101001101010010010010\
101101010011010100100100101101101010010101001001001010101101001101010010010010110101\
101001010010110110100000",
        );

        doTest(
            "`abcdefghijklmnopqrstuvwxyz{|}~",
            "000001001011011010101001001001011001101010101001010010010110101001011010010100100101\
011010010110100101001001011011010010101001010010010101011001011010010100100101101011\
001010100101001001010110110010101001010010010101010011011010010100100101101010011010\
100101001001010110100110101001010010010101011001101010010100100101101010100110100101\
001001010110101001101001010010010110110101001010010100100101010110100110100101001001\
011010110100101001010010010101101101001010010100100101010101100110100101001001011010\
101100101001010010010101101011001010010100100101010110110010100101001001011001010101\
101001010010010100110101011010010100100101100110101010100101001001010010110101101001\
010010010110010110101010010100100101001101101010101001001001010110110100101010010010\
010101010110011010100100100101101010110010101001001001010110101100101010010010010101\
011011001010010110110100000",
        );
    }

    fn doTest(input: &str, expected: &str) {
        let result = Code39Writer::default()
            .encode(input, &BarcodeFormat::CODE_39, 0, 0)
            .expect("must encode");
        assert_eq!(
            expected,
            bit_matrix_test_case::matrix_to_string(&result),
            "{input}"
        );
    }
}
