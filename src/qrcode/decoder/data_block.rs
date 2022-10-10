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

use crate::Exceptions;

use super::{VersionRef, ErrorCorrectionLevel};

/**
 * <p>Encapsulates a block of data within a QR Code. QR Codes may split their data into
 * multiple blocks, each of which is a unit of data and error-correction codewords. Each
 * is represented by an instance of this class.</p>
 *
 * @author Sean Owen
 */
pub struct DataBlock {

  numDataCodewords:u32,
   codewords:Vec<u8>,
}

impl DataBlock {

  fn new( numDataCodewords:u32,  codewords:Vec<u8>) -> Self{
    Self{
        numDataCodewords,
        codewords,
    }
  }

  /**
   * <p>When QR Codes use multiple data blocks, they are actually interleaved.
   * That is, the first byte of data block 1 to n is written, then the second bytes, and so on. This
   * method will separate the data into original blocks.</p>
   *
   * @param rawCodewords bytes as read directly from the QR Code
   * @param version version of the QR Code
   * @param ecLevel error-correction level of the QR Code
   * @return DataBlocks containing original bytes, "de-interleaved" from representation in the
   *         QR Code
   */
  pub fn getDataBlocks( rawCodewords:&[u8],
                                    version:VersionRef,
                                    ecLevel:ErrorCorrectionLevel) -> Result<Vec<Self>,Exceptions> {

    if rawCodewords.len() as u32 != version.getTotalCodewords() {
      return Err(Exceptions::IllegalArgumentException("".to_owned()))
    }

    // Figure out the number and size of data blocks used by this version and
    // error correction level
    let ecBlocks = version.getECBlocksForLevel(ecLevel);

    // First count the total number of data blocks
    let mut totalBlocks = 0;
    let ecBlockArray = ecBlocks.getECBlocks();
    for ecBlock in ecBlockArray {
    // for (Version.ECB ecBlock : ecBlockArray) {
      totalBlocks += ecBlock.getCount();
    }

    // Now establish DataBlocks of the appropriate size and number of data codewords
     let mut result = Vec::new();
    let mut numRXingResultBlocks = 0;
    for ecBlock in ecBlockArray {
    // for (Version.ECB ecBlock : ecBlockArray) {
      for _i in 0..ecBlock.getCount() {
      // for (int i = 0; i < ecBlock.getCount(); i++) {
        let numDataCodewords = ecBlock.getDataCodewords();
        let numBlockCodewords = ecBlocks.getECCodewordsPerBlock() + numDataCodewords;
        // result[numRXingResultBlocks] =  DataBlock::new(numDataCodewords, vec![0u8;numBlockCodewords as usize]);
        result.push(  DataBlock::new(numDataCodewords, vec![0u8;numBlockCodewords as usize]));
        numRXingResultBlocks += 1;
      }
    }

    // All blocks have the same amount of data, except that the last n
    // (where n may be 0) have 1 more byte. Figure out where these start.
    let shorterBlocksTotalCodewords = result[0].codewords.len();
    let mut longerBlocksStartAt = result.len() - 1;
    while (longerBlocksStartAt >= 0) {
      let numCodewords = result[longerBlocksStartAt].codewords.len();
      if (numCodewords == shorterBlocksTotalCodewords) {
        break;
      }
      longerBlocksStartAt-=1;
    }
    longerBlocksStartAt+=1;

    let shorterBlocksNumDataCodewords = shorterBlocksTotalCodewords - ecBlocks.getECCodewordsPerBlock() as usize;
    // The last elements of result may be 1 element longer;
    // first fill out as many elements as all of them have
    let mut rawCodewordsOffset = 0;
    for i in 0..shorterBlocksNumDataCodewords {
    // for (int i = 0; i < shorterBlocksNumDataCodewords; i++) {
      for j in 0..numRXingResultBlocks {
      // for (int j = 0; j < numRXingResultBlocks; j++) {
        result[j].codewords[i] = rawCodewords[rawCodewordsOffset];
        rawCodewordsOffset += 1;
      }
    }
    // Fill out the last data block in the longer ones
    for j in longerBlocksStartAt..numRXingResultBlocks{
    // for (int j = longerBlocksStartAt; j < numRXingResultBlocks; j++) {
      result[j].codewords[shorterBlocksNumDataCodewords] = rawCodewords[rawCodewordsOffset];
      rawCodewordsOffset += 1;
    }
    // Now add in error correction blocks
    let max = result[0].codewords.len();
    for i in shorterBlocksNumDataCodewords..max {
    // for (int i = shorterBlocksNumDataCodewords; i < max; i++) {
      for j in 0..numRXingResultBlocks {
      // for (int j = 0; j < numRXingResultBlocks; j++) {
        let iOffset = if j < longerBlocksStartAt  {i} else {i + 1};
        result[j].codewords[iOffset] = rawCodewords[rawCodewordsOffset];
        rawCodewordsOffset += 1;
      }
    }
    Ok(result)
  }

  pub fn getNumDataCodewords(&self) -> u32{
    self. numDataCodewords
  }

  pub fn getCodewords(&self) -> &[u8] {
    &self.codewords
  }

}
