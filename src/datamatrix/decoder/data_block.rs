/*
 * Copyright 2008 ZXing authors
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

use crate::common::Result;
use crate::Exceptions;

use super::Version;

/**
 * <p>Encapsulates a block of data within a Data Matrix Code. Data Matrix Codes may split their data into
 * multiple blocks, each of which is a unit of data and error-correction codewords. Each
 * is represented by an instance of this class.</p>
 *
 * @author bbrown@google.com (Brian Brown)
 */
pub struct DataBlock {
    numDataCodewords: u32,
    codewords: Vec<u8>,
}

impl DataBlock {
    pub fn new(numDataCodewords: u32, codewords: Vec<u8>) -> Self {
        Self {
            numDataCodewords,
            codewords,
        }
    }

    /**
     * <p>When Data Matrix Codes use multiple data blocks, they actually interleave the bytes of each of them.
     * That is, the first byte of data block 1 to n is written, then the second bytes, and so on. This
     * method will separate the data into original blocks.</p>
     *
     * @param rawCodewords bytes as read directly from the Data Matrix Code
     * @param version version of the Data Matrix Code
     * @return DataBlocks containing original bytes, "de-interleaved" from representation in the
     *         Data Matrix Code
     */
    pub fn getDataBlocks(
        rawCodewords: &[u8],
        version: &Version,
        fix259: bool,
    ) -> Result<Vec<DataBlock>> {
        // Figure out the number and size of data blocks used by this version
        let ecBlocks = version.getECBlocks();

        // First count the total number of data blocks
        let ecBlockArray = ecBlocks.getECBlocks();
        let totalBlocks = ecBlockArray
            .iter()
            .fold(0, |acc, ecBlock| acc + ecBlock.getCount() as usize);

        // Now establish DataBlocks of the appropriate size and number of data codewords
        let mut result = Vec::with_capacity(totalBlocks);
        let mut numRXingResultBlocks = 0;
        for ecBlock in ecBlockArray {
            for _i in 0..ecBlock.getCount() {
                // for (int i = 0; i < ecBlock.getCount(); i++) {
                let numDataCodewords = ecBlock.getDataCodewords() as usize;
                let numBlockCodewords = ecBlocks.getECCodewords() as usize + numDataCodewords;
                // result[numRXingResultBlocks++] = new DataBlock(numDataCodewords, new byte[numBlockCodewords]);
                result.push(DataBlock::new(
                    numDataCodewords as u32,
                    vec![0; numBlockCodewords],
                ));
                numRXingResultBlocks += 1;
            }
        }

        // All blocks have the same amount of data, except that the last n
        // (where n may be 0) have 1 less byte. Figure out where these start.
        // TODO(bbrown): There is only one case where there is a difference for Data Matrix for size 144
        let longerBlocksTotalCodewords = result[0].codewords.len();
        //int shorterBlocksTotalCodewords = longerBlocksTotalCodewords - 1;

        let longerBlocksNumDataCodewords =
            longerBlocksTotalCodewords - ecBlocks.getECCodewords() as usize;
        let shorterBlocksNumDataCodewords = longerBlocksNumDataCodewords - 1;
        // The last elements of result may be 1 element shorter for 144 matrix
        // first fill out as many elements as all of them have minus 1
        let mut rawCodewordsOffset = 0;
        for i in 0..shorterBlocksNumDataCodewords {
            // for (int i = 0; i < shorterBlocksNumDataCodewords; i++) {

            for res in result.iter_mut().take(numRXingResultBlocks) {
                // for j in 0..numRXingResultBlocks {
                // for (int j = 0; j < numRXingResultBlocks; j++) {
                res.codewords[i] = rawCodewords[rawCodewordsOffset];
                rawCodewordsOffset += 1;
            }
        }

        // Fill out the last data block in the longer ones
        let specialVersion = version.getVersionNumber() == 24;
        let numLongerBlocks = if specialVersion {
            8
        } else {
            numRXingResultBlocks
        };
        for res in result.iter_mut().take(numLongerBlocks) {
            // for j in 0..numLongerBlocks {
            // for (int j = 0; j < numLongerBlocks; j++) {
            res.codewords[longerBlocksNumDataCodewords - 1] = rawCodewords[rawCodewordsOffset];
            rawCodewordsOffset += 1;
        }

        // Now add in error correction blocks
        let max = result[0].codewords.len();
        for i in longerBlocksNumDataCodewords..max {
            // for (int i = longerBlocksNumDataCodewords; i < max; i++) {
            for j in 0..numRXingResultBlocks {
                // for (int j = 0; j < numRXingResultBlocks; j++) {
                let jOffset = if specialVersion && fix259 {
                    (j + 8) % numRXingResultBlocks
                } else {
                    j
                };
                let iOffset = if specialVersion && jOffset > 7 {
                    i - 1
                } else {
                    i
                };
                result[jOffset].codewords[iOffset] = rawCodewords[rawCodewordsOffset];
                rawCodewordsOffset += 1;
            }
        }

        if rawCodewordsOffset != rawCodewords.len() {
            return Err(Exceptions::illegalArgument);
        }

        Ok(result)
    }

    pub fn getNumDataCodewords(&self) -> u32 {
        self.numDataCodewords
    }

    pub fn getCodewords(&self) -> &[u8] {
        &self.codewords
    }
}
