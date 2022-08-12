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
 * <p>Encapsulates a block of data within a QR Code. QR Codes may split their data into
 * multiple blocks, each of which is a unit of data and error-correction codewords. Each
 * is represented by an instance of this class.</p>
 *
 * @author Sean Owen
 */
struct DataBlock {

     let num_data_codewords: i32;

     let mut codewords: Vec<i8>;
}

impl DataBlock {

    fn new( num_data_codewords: i32,  codewords: &Vec<i8>) -> DataBlock {
        let .numDataCodewords = num_data_codewords;
        let .codewords = codewords;
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
    fn  get_data_blocks( raw_codewords: &Vec<i8>,  version: &Version,  ec_level: &ErrorCorrectionLevel) -> Vec<DataBlock>  {
        if raw_codewords.len() != version.get_total_codewords() {
            throw IllegalArgumentException::new();
        }
        // Figure out the number and size of data blocks used by this version and
        // error correction level
         let ec_blocks: Version.ECBlocks = version.get_e_c_blocks_for_level(ec_level);
        // First count the total number of data blocks
         let total_blocks: i32 = 0;
         let ec_block_array: Vec<Version.ECB> = ec_blocks.get_e_c_blocks();
        for  let ec_block: Version.ECB in ec_block_array {
            total_blocks += ec_block.get_count();
        }
        // Now establish DataBlocks of the appropriate size and number of data codewords
         let mut result: [Option<DataBlock>; total_blocks] = [None; total_blocks];
         let num_result_blocks: i32 = 0;
        for  let ec_block: Version.ECB in ec_block_array {
             {
                 let mut i: i32 = 0;
                while i < ec_block.get_count() {
                    {
                         let num_data_codewords: i32 = ec_block.get_data_codewords();
                         let num_block_codewords: i32 = ec_blocks.get_e_c_codewords_per_block() + num_data_codewords;
                        result[num_result_blocks += 1 !!!check!!! post increment] = DataBlock::new(num_data_codewords, : [i8; num_block_codewords] = [0; num_block_codewords]);
                    }
                    i += 1;
                 }
             }

        }
        // All blocks have the same amount of data, except that the last n
        // (where n may be 0) have 1 more byte. Figure out where these start.
         let shorter_blocks_total_codewords: i32 = result[0].codewords.len();
         let longer_blocks_start_at: i32 = result.len() - 1;
        while longer_blocks_start_at >= 0 {
             let num_codewords: i32 = result[longer_blocks_start_at].codewords.len();
            if num_codewords == shorter_blocks_total_codewords {
                break;
            }
            longer_blocks_start_at -= 1;
        }
        longer_blocks_start_at += 1;
         let shorter_blocks_num_data_codewords: i32 = shorter_blocks_total_codewords - ec_blocks.get_e_c_codewords_per_block();
        // The last elements of result may be 1 element longer;
        // first fill out as many elements as all of them have
         let raw_codewords_offset: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < shorter_blocks_num_data_codewords {
                {
                     {
                         let mut j: i32 = 0;
                        while j < num_result_blocks {
                            {
                                result[j].codewords[i] = raw_codewords[raw_codewords_offset += 1 !!!check!!! post increment];
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        // Fill out the last data block in the longer ones
         {
             let mut j: i32 = longer_blocks_start_at;
            while j < num_result_blocks {
                {
                    result[j].codewords[shorter_blocks_num_data_codewords] = raw_codewords[raw_codewords_offset += 1 !!!check!!! post increment];
                }
                j += 1;
             }
         }

        // Now add in error correction blocks
         let max: i32 = result[0].codewords.len();
         {
             let mut i: i32 = shorter_blocks_num_data_codewords;
            while i < max {
                {
                     {
                         let mut j: i32 = 0;
                        while j < num_result_blocks {
                            {
                                 let i_offset: i32 =  if j < longer_blocks_start_at { i } else { i + 1 };
                                result[j].codewords[i_offset] = raw_codewords[raw_codewords_offset += 1 !!!check!!! post increment];
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        return result;
    }

    fn  get_num_data_codewords(&self) -> i32  {
        return self.num_data_codewords;
    }

    fn  get_codewords(&self) -> Vec<i8>  {
        return self.codewords;
    }
}

