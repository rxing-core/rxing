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

use crate::{
    common::{
        reedsolomon::{get_predefined_genericgf, PredefinedGenericGF, ReedSolomonDecoder},
        BitMatrix, DecoderRXingResult,
    },
    Exceptions,
};

use super::{decoded_bit_stream_parser, BitMatrixParser, DataBlock};

/**
 * <p>The main class which implements Data Matrix Code decoding -- as opposed to locating and extracting
 * the Data Matrix Code from an image.</p>
 *
 * @author bbrown@google.com (Brian Brown)
 */
pub struct Decoder(ReedSolomonDecoder);

impl Decoder {
    pub fn new() -> Self {
        Self(ReedSolomonDecoder::new(get_predefined_genericgf(
            PredefinedGenericGF::DataMatrixField256,
        )))
        // rsDecoder = new ReedSolomonDecoder(GenericGF.DATA_MATRIX_FIELD_256);
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
    pub fn decode(&self, bits: &BitMatrix) -> Result<DecoderRXingResult, Exceptions> {
        self.perform_decode(bits, false)
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
    pub fn decode_bools(&self, image: &Vec<Vec<bool>>) -> Result<DecoderRXingResult, Exceptions> {
        self.perform_decode(&BitMatrix::parse_bools(image),false)
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
     fn perform_decode(&self, bits: &BitMatrix, fix259: bool) -> Result<DecoderRXingResult, Exceptions> {
        // Construct a parser and read version, error-correction level
        let mut parser = BitMatrixParser::new(bits)?;

        // Read codewords
        let codewords = parser.readCodewords()?;

        let version = parser.getVersion();

        // Separate into data blocks
        let dataBlocks = DataBlock::getDataBlocks(&codewords, version, fix259)?;

        // Count total number of data bytes
        let totalBytes = dataBlocks
            .iter()
            .fold(0, |acc, db| acc + db.getNumDataCodewords());

        let mut resultBytes = vec![0u8; totalBytes as usize];

        let dataBlocksCount = dataBlocks.len();
        // Error-correct and copy data blocks together into a stream of bytes
        for j in 0..dataBlocksCount {
            // for (int j = 0; j < dataBlocksCount; j++) {
            let dataBlock = &dataBlocks[j];
            let mut codewordBytes = dataBlock.getCodewords().to_vec();
            let numDataCodewords = dataBlock.getNumDataCodewords() as usize;
            let errors_corrected = self.correctErrors(&mut codewordBytes, numDataCodewords as u32);
            if errors_corrected.is_err() && !fix259 {
                return self.perform_decode(bits, true);
            }else if errors_corrected.is_err() {
                return Err(errors_corrected.err().unwrap())
            }
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
    fn correctErrors(
        &self,
        codewordBytes: &mut [u8],
        numDataCodewords: u32,
    ) -> Result<(), Exceptions> {
        let _numCodewords = codewordBytes.len();
        // First read into an array of ints
        // let codewordsInts = vec![0i32;numCodewords];
        // for i in 0..numCodewords {
        // // for (int i = 0; i < numCodewords; i++) {
        //   codewordsInts[i] = codewordBytes[i];
        // }
        let mut codewordsInts: Vec<i32> = codewordBytes.iter().map(|x| *x as i32).collect();

        //try {
        self.0.decode(
            &mut codewordsInts,
            codewordBytes.len() as i32 - numDataCodewords as i32,
        )?;
        //} catch (ReedSolomonException ignored) {
        //throw ChecksumException.getChecksumInstance();
        //}
        // Copy back into array of bytes -- only need to worry about the bytes that were data
        // We don't care about errors in the error-correction codewords
        for i in 0..numDataCodewords as usize {
            // for (int i = 0; i < numDataCodewords; i++) {
            codewordBytes[i] = codewordsInts[i] as u8;
        }
        // codewordsInts.into_iter().take(numDataCodewords as usize).map(|x| x as u8).collect::<Vec<u8>>()
        Ok(())
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
