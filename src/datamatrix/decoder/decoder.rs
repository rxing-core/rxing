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

use crate::{common::{reedsolomon::{ReedSolomonDecoder, get_predefined_genericgf, PredefinedGenericGF}, DecoderRXingResult, BitMatrix}, Exceptions};

use super::{DataBlock, BitMatrixParser, decoded_bit_stream_parser};


/**
 * <p>The main class which implements Data Matrix Code decoding -- as opposed to locating and extracting
 * the Data Matrix Code from an image.</p>
 *
 * @author bbrown@google.com (Brian Brown)
 */
pub struct Decoder(ReedSolomonDecoder);

impl Decoder {

  pub fn new() -> Self {
    
    Self(ReedSolomonDecoder::new(get_predefined_genericgf(PredefinedGenericGF::DataMatrixField256)))
    // rsDecoder = new ReedSolomonDecoder(GenericGF.DATA_MATRIX_FIELD_256);
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
  pub fn decode_bools(&self,  image:&Vec<Vec<bool>>) -> Result<DecoderRXingResult,Exceptions> {
     self.decode(&BitMatrix::parse_bools(&image))
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
  pub fn decode(&self,  bits:&BitMatrix) -> Result<DecoderRXingResult,Exceptions> {

    // Construct a parser and read version, error-correction level
    let parser =  BitMatrixParser::new(bits)?;
    let version = parser.getVersion();


    // Read codewords
    let codewords = parser.readCodewords()?;
    // Separate into data blocks
    let dataBlocks = DataBlock::getDataBlocks(&codewords, version)?;

    // Count total number of data bytes
    let totalBytes = 0;
    for  db in dataBlocks {
      totalBytes += db.getNumDataCodewords();
    }
    let resultBytes = vec![0u8;totalBytes as usize];

    let dataBlocksCount = dataBlocks.len();
    // Error-correct and copy data blocks together into a stream of bytes
    for j in 0..dataBlocksCount {
    // for (int j = 0; j < dataBlocksCount; j++) {
      let dataBlock = dataBlocks[j];
      let codewordBytes = dataBlock.getCodewords();
      let numDataCodewords = dataBlock.getNumDataCodewords() as usize;
      self.correctErrors(codewordBytes, numDataCodewords as u32);
      for i in 0..numDataCodewords {
      // for (int i = 0; i < numDataCodewords; i++) {
        // De-interlace data blocks.
        resultBytes[i * dataBlocksCount + j] = codewordBytes[i];
      }
    }

    // Decode the contents of that stream of bytes
    decoded_bit_stream_parser::decode(&resultBytes)
  }

  /**
   * <p>Given data and error-correction codewords received, possibly corrupted by errors, attempts to
   * correct the errors in-place using Reed-Solomon error correction.</p>
   *
   * @param codewordBytes data and error correction codewords
   * @param numDataCodewords number of codewords that are data bytes
   * @throws ChecksumException if error correction fails
   */
  fn correctErrors(&self,  codewordBytes:&[u8],  numDataCodewords:u32) -> Result<(),Exceptions> {
    let numCodewords = codewordBytes.len();
    // First read into an array of ints
    // let codewordsInts = vec![0i32;numCodewords];
    // for i in 0..numCodewords {
    // // for (int i = 0; i < numCodewords; i++) {
    //   codewordsInts[i] = codewordBytes[i];
    // }
    let codewordsInts : Vec<i32> = codewordBytes.iter().map(|x| *x as i32).collect();

    //try {
       self.0.decode(&mut codewordsInts, codewordBytes.len() as i32- numDataCodewords as i32)?;
    //} catch (ReedSolomonException ignored) {
      //throw ChecksumException.getChecksumInstance();
    //}
    // Copy back into array of bytes -- only need to worry about the bytes that were data
    // We don't care about errors in the error-correction codewords
    for i in 0..numDataCodewords as usize {
    // for (int i = 0; i < numDataCodewords; i++) {
      codewordBytes[i] =  codewordsInts[i] as u8;
    }
    // codewordsInts.into_iter().take(numDataCodewords as usize).map(|x| x as u8).collect::<Vec<u8>>()
    Ok(())
  }

}
