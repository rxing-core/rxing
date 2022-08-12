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
 * <p>The main class which implements QR Code decoding -- as opposed to locating and extracting
 * the QR Code from an image.</p>
 *
 * @author Sean Owen
 */
pub struct Decoder {

     let rs_decoder: ReedSolomonDecoder;
}

impl Decoder {

    pub fn new() -> Decoder {
        rs_decoder = ReedSolomonDecoder::new(GenericGF::QR_CODE_FIELD_256);
    }

    pub fn  decode(&self,  image: &Vec<Vec<bool>>) -> /*  throws ChecksumException, FormatException */Result<DecoderResult, Rc<Exception>>   {
        return Ok(self.decode(&image, null));
    }

    /**
   * <p>Convenience method that can decode a QR Code represented as a 2D array of booleans.
   * "true" is taken to mean a black module.</p>
   *
   * @param image booleans representing white/black QR Code modules
   * @param hints decoding hints that should be used to influence decoding
   * @return text and bytes encoded within the QR Code
   * @throws FormatException if the QR Code cannot be decoded
   * @throws ChecksumException if error correction fails
   */
    pub fn  decode(&self,  image: &Vec<Vec<bool>>,  hints: &Map<DecodeHintType, ?>) -> /*  throws ChecksumException, FormatException */Result<DecoderResult, Rc<Exception>>   {
        return Ok(self.decode(&BitMatrix::parse(&image), &hints));
    }

    pub fn  decode(&self,  bits: &BitMatrix) -> /*  throws ChecksumException, FormatException */Result<DecoderResult, Rc<Exception>>   {
        return Ok(self.decode(bits, null));
    }

    /**
   * <p>Decodes a QR Code represented as a {@link BitMatrix}. A 1 or "true" is taken to mean a black module.</p>
   *
   * @param bits booleans representing white/black QR Code modules
   * @param hints decoding hints that should be used to influence decoding
   * @return text and bytes encoded within the QR Code
   * @throws FormatException if the QR Code cannot be decoded
   * @throws ChecksumException if error correction fails
   */
    pub fn  decode(&self,  bits: &BitMatrix,  hints: &Map<DecodeHintType, ?>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
        // Construct a parser and read version, error-correction level
         let parser: BitMatrixParser = BitMatrixParser::new(bits);
         let mut fe: FormatException = null;
         let mut ce: ChecksumException = null;
        let tryResult1 = 0;
        'try1: loop {
        {
            return Ok(self.decode(parser, &hints));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &FormatException) {
                fe = e;
            } catch ( e: &ChecksumException) {
                ce = e;
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
            // Revert the bit matrix
            parser.remask();
            // Will be attempting a mirrored reading of the version and format info.
            parser.set_mirror(true);
            // Preemptively read the version.
            parser.read_version();
            // Preemptively read the format information.
            parser.read_format_information();
            /*
       * Since we're here, this means we have successfully detected some kind
       * of version and format information when mirrored. This is a good sign,
       * that the QR code may be mirrored, and we should try once more with a
       * mirrored content.
       */
            // Prepare for a mirrored reading.
            parser.mirror();
             let result: DecoderResult = self.decode(parser, &hints);
            // Success! Notify the caller that the code was mirrored.
            result.set_other(QRCodeDecoderMetaData::new(true));
            return Ok(result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &FormatExceptionChecksumException | ) {
                if fe != null {
                    throw fe;
                }
                throw ce;
            }  0 => break
        }

    }

    fn  decode(&self,  parser: &BitMatrixParser,  hints: &Map<DecodeHintType, ?>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
         let version: Version = parser.read_version();
         let ec_level: ErrorCorrectionLevel = parser.read_format_information().get_error_correction_level();
        // Read codewords
         let codewords: Vec<i8> = parser.read_codewords();
        // Separate into data blocks
         let data_blocks: Vec<DataBlock> = DataBlock::get_data_blocks(&codewords, version, ec_level);
        // Count total number of data bytes
         let total_bytes: i32 = 0;
        for  let data_block: DataBlock in data_blocks {
            total_bytes += data_block.get_num_data_codewords();
        }
         let result_bytes: [i8; total_bytes] = [0; total_bytes];
         let result_offset: i32 = 0;
        // Error-correct and copy data blocks together into a stream of bytes
        for  let data_block: DataBlock in data_blocks {
             let codeword_bytes: Vec<i8> = data_block.get_codewords();
             let num_data_codewords: i32 = data_block.get_num_data_codewords();
            self.correct_errors(&codeword_bytes, num_data_codewords);
             {
                 let mut i: i32 = 0;
                while i < num_data_codewords {
                    {
                        result_bytes[result_offset += 1 !!!check!!! post increment] = codeword_bytes[i];
                    }
                    i += 1;
                 }
             }

        }
        // Decode the contents of that stream of bytes
        return Ok(DecodedBitStreamParser::decode(&result_bytes, version, ec_level, &hints));
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

