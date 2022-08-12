/*
 * Copyright 2007 ZXing authors
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>See ISO 18004:2006, 6.4.1, Tables 2 and 3. This enum encapsulates the various modes in which
 * data can be encoded to bits in the QR code standard.</p>
 *
 * @author Sean Owen
 */
pub enum Mode {

    // Not really a mode...
    TERMINATOR( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x00), NUMERIC( : vec![i32; 3] = vec![10, 12, 14, ]
    , 0x01), ALPHANUMERIC( : vec![i32; 3] = vec![9, 11, 13, ]
    , 0x02), // Not supported
    STRUCTURED_APPEND( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x03), BYTE( : vec![i32; 3] = vec![8, 16, 16, ]
    , 0x04), // character counts don't apply
    ECI( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x07), KANJI( : vec![i32; 3] = vec![8, 10, 12, ]
    , 0x08), FNC1_FIRST_POSITION( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x05), FNC1_SECOND_POSITION( : vec![i32; 3] = vec![0, 0, 0, ]
    , 0x09), /** See GBT 18284-2000; "Hanzi" is a transliteration of this mode name. */
    HANZI( : vec![i32; 3] = vec![8, 10, 12, ]
    , 0x0D);

     let character_count_bits_for_versions: Vec<i32>;

     let bits: i32;

    fn new( character_count_bits_for_versions: &Vec<i32>,  bits: i32) -> Mode {
        let .characterCountBitsForVersions = character_count_bits_for_versions;
        let .bits = bits;
    }

    /**
   * @param bits four bits encoding a QR Code data mode
   * @return Mode encoded by these bits
   * @throws IllegalArgumentException if bits do not correspond to a known mode
   */
    pub fn  for_bits( bits: i32) -> Mode  {
        match bits {
              0x0 => 
                 {
                    return TERMINATOR;
                }
              0x1 => 
                 {
                    return NUMERIC;
                }
              0x2 => 
                 {
                    return ALPHANUMERIC;
                }
              0x3 => 
                 {
                    return STRUCTURED_APPEND;
                }
              0x4 => 
                 {
                    return BYTE;
                }
              0x5 => 
                 {
                    return FNC1_FIRST_POSITION;
                }
              0x7 => 
                 {
                    return ECI;
                }
              0x8 => 
                 {
                    return KANJI;
                }
              0x9 => 
                 {
                    return FNC1_SECOND_POSITION;
                }
              0xD => 
                 {
                    // 0xD is defined in GBT 18284-2000, may not be supported in foreign country
                    return HANZI;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new();
                }
        }
    }

    /**
   * @param version version in question
   * @return number of bits used, in this QR Code symbol {@link Version}, to encode the
   *         count of characters that will follow encoded in this Mode
   */
    pub fn  get_character_count_bits(&self,  version: &Version) -> i32  {
         let number: i32 = version.get_version_number();
         let mut offset: i32;
        if number <= 9 {
            offset = 0;
        } else if number <= 26 {
            offset = 1;
        } else {
            offset = 2;
        }
        return self.character_count_bits_for_versions[offset];
    }

    pub fn  get_bits(&self) -> i32  {
        return self.bits;
    }
}
