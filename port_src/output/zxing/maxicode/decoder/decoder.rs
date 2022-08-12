/*
 * Copyright 2011 ZXing authors
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
// package com::google::zxing::maxicode::decoder;

/**
 * <p>The main class which implements MaxiCode decoding -- as opposed to locating and extracting
 * the MaxiCode from an image.</p>
 *
 * @author Manuel Kasten
 */

 const ALL: i32 = 0;

 const EVEN: i32 = 1;

 const ODD: i32 = 2;
pub struct Decoder {

     let rs_decoder: ReedSolomonDecoder;
}

impl Decoder {

    pub fn new() -> Decoder {
        rs_decoder = ReedSolomonDecoder::new(GenericGF::MAXICODE_FIELD_64);
    }

    pub fn  decode(&self,  bits: &BitMatrix) -> /*  throws ChecksumException, FormatException */Result<DecoderResult, Rc<Exception>>   {
        return Ok(self.decode(bits, null));
    }

    pub fn  decode(&self,  bits: &BitMatrix,  hints: &Map<DecodeHintType, ?>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
         let parser: BitMatrixParser = BitMatrixParser::new(bits);
         let codewords: Vec<i8> = parser.read_codewords();
        self.correct_errors(&codewords, 0, 10, 10, ALL);
         let mode: i32 = codewords[0] & 0x0F;
         let mut datawords: Vec<i8>;
        match mode {
              2 => 
                 {
                }
              3 => 
                 {
                }
              4 => 
                 {
                    self.correct_errors(&codewords, 20, 84, 40, EVEN);
                    self.correct_errors(&codewords, 20, 84, 40, ODD);
                    datawords = : [i8; 94] = [0; 94];
                    break;
                }
              5 => 
                 {
                    self.correct_errors(&codewords, 20, 68, 56, EVEN);
                    self.correct_errors(&codewords, 20, 68, 56, ODD);
                    datawords = : [i8; 78] = [0; 78];
                    break;
                }
            _ => 
                 {
                    throw FormatException::get_format_instance();
                }
        }
        System::arraycopy(&codewords, 0, &datawords, 0, 10);
        System::arraycopy(&codewords, 20, &datawords, 10, datawords.len() - 10);
        return Ok(DecodedBitStreamParser::decode(&datawords, mode));
    }

    fn  correct_errors(&self,  codeword_bytes: &Vec<i8>,  start: i32,  data_codewords: i32,  ec_codewords: i32,  mode: i32)  -> /*  throws ChecksumException */Result<Void, Rc<Exception>>   {
         let codewords: i32 = data_codewords + ec_codewords;
        // in EVEN or ODD mode only half the codewords
         let mut divisor: i32 =  if mode == ALL { 1 } else { 2 };
        // First read into an array of ints
         let codewords_ints: [i32; codewords / divisor] = [0; codewords / divisor];
         {
             let mut i: i32 = 0;
            while i < codewords {
                {
                    if (mode == ALL) || (i % 2 == (mode - 1)) {
                        codewords_ints[i / divisor] = codeword_bytes[i + start] & 0xFF;
                    }
                }
                i += 1;
             }
         }

        let tryResult1 = 0;
        'try1: loop {
        {
            self.rs_decoder.decode(&codewords_ints, ec_codewords / divisor);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &ReedSolomonException) {
                throw ChecksumException::get_checksum_instance();
            }  0 => break
        }

        // We don't care about errors in the error-correction codewords
         {
             let mut i: i32 = 0;
            while i < data_codewords {
                {
                    if (mode == ALL) || (i % 2 == (mode - 1)) {
                        codeword_bytes[i + start] = codewords_ints[i / divisor] as i8;
                    }
                }
                i += 1;
             }
         }

    }
}

