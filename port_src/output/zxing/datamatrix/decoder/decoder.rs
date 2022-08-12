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
// package com::google::zxing::datamatrix::decoder;

/**
 * <p>The main class which implements Data Matrix Code decoding -- as opposed to locating and extracting
 * the Data Matrix Code from an image.</p>
 *
 * @author bbrown@google.com (Brian Brown)
 */
pub struct Decoder {

     let rs_decoder: ReedSolomonDecoder;
}

impl Decoder {

    pub fn new() -> Decoder {
        rs_decoder = ReedSolomonDecoder::new(GenericGF::DATA_MATRIX_FIELD_256);
    }

    /**
   * <p>Convenience method that can decode a Data Matrix Code represented as a 2D array of booleans.
   * "true" is taken to mean a black module.</p>
   *
   * @param image booleans representing white/black Data Matrix Code modules
   * @return text and bytes encoded within the Data Matrix Code
   * @throws FormatException if the Data Matrix Code cannot be decoded
   * @throws ChecksumException if error correction fails
   */
    pub fn  decode(&self,  image: &Vec<Vec<bool>>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
        return Ok(self.decode(&BitMatrix::parse(&image)));
    }

    /**
   * <p>Decodes a Data Matrix Code represented as a {@link BitMatrix}. A 1 or "true" is taken
   * to mean a black module.</p>
   *
   * @param bits booleans representing white/black Data Matrix Code modules
   * @return text and bytes encoded within the Data Matrix Code
   * @throws FormatException if the Data Matrix Code cannot be decoded
   * @throws ChecksumException if error correction fails
   */
    pub fn  decode(&self,  bits: &BitMatrix) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
        // Construct a parser and read version, error-correction level
         let parser: BitMatrixParser = BitMatrixParser::new(bits);
         let version: Version = parser.get_version();
        // Read codewords
         let codewords: Vec<i8> = parser.read_codewords();
        // Separate into data blocks
         let data_blocks: Vec<DataBlock> = DataBlock::get_data_blocks(&codewords, version);
        // Count total number of data bytes
         let total_bytes: i32 = 0;
        for  let db: DataBlock in data_blocks {
            total_bytes += db.get_num_data_codewords();
        }
         let result_bytes: [i8; total_bytes] = [0; total_bytes];
         let data_blocks_count: i32 = data_blocks.len();
        // Error-correct and copy data blocks together into a stream of bytes
         {
             let mut j: i32 = 0;
            while j < data_blocks_count {
                {
                     let data_block: DataBlock = data_blocks[j];
                     let codeword_bytes: Vec<i8> = data_block.get_codewords();
                     let num_data_codewords: i32 = data_block.get_num_data_codewords();
                    self.correct_errors(&codeword_bytes, num_data_codewords);
                     {
                         let mut i: i32 = 0;
                        while i < num_data_codewords {
                            {
                                // De-interlace data blocks.
                                result_bytes[i * data_blocks_count + j] = codeword_bytes[i];
                            }
                            i += 1;
                         }
                     }

                }
                j += 1;
             }
         }

        // Decode the contents of that stream of bytes
        return Ok(DecodedBitStreamParser::decode(&result_bytes));
    }

    /**
   * <p>Given data and error-correction codewords received, possibly corrupted by errors, attempts to
   * correct the errors in-place using Reed-Solomon error correction.</p>
   *
   * @param codewordBytes data and error correction codewords
   * @param numDataCodewords number of codewords that are data bytes
   * @throws ChecksumException if error correction fails
   */
    fn  correct_errors(&self,  codeword_bytes: &Vec<i8>,  num_data_codewords: i32)  -> /*  throws ChecksumException */Result<Void, Rc<Exception>>   {
         let num_codewords: i32 = codeword_bytes.len();
        // First read into an array of ints
         let codewords_ints: [i32; num_codewords] = [0; num_codewords];
         {
             let mut i: i32 = 0;
            while i < num_codewords {
                {
                    codewords_ints[i] = codeword_bytes[i] & 0xFF;
                }
                i += 1;
             }
         }

        let tryResult1 = 0;
        'try1: loop {
        {
            self.rs_decoder.decode(&codewords_ints, codeword_bytes.len() - num_data_codewords);
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
            while i < num_data_codewords {
                {
                    codeword_bytes[i] = codewords_ints[i] as i8;
                }
                i += 1;
             }
         }

    }
}

