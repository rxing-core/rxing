/*
 * Copyright 2006 Jeremias Maerki.
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
// package com::google::zxing::datamatrix::encoder;

/**
 * Error Correction Code for ECC200.
 */

/**
   * Lookup table which factors to use for which number of error correction codewords.
   * See FACTORS.
   */
 const FACTOR_SETS: vec![Vec<i32>; 16] = vec![5, 7, 10, 11, 12, 14, 18, 20, 24, 28, 36, 42, 48, 56, 62, 68, ]
;

/**
   * Precomputed polynomial factors for ECC 200.
   */
 const FACTORS: vec![vec![Vec<Vec<i32>>; 68]; 16] = vec![vec![228, 48, 15, 111, 62, ]
, vec![23, 68, 144, 134, 240, 92, 254, ]
, vec![28, 24, 185, 166, 223, 248, 116, 255, 110, 61, ]
, vec![175, 138, 205, 12, 194, 168, 39, 245, 60, 97, 120, ]
, vec![41, 153, 158, 91, 61, 42, 142, 213, 97, 178, 100, 242, ]
, vec![156, 97, 192, 252, 95, 9, 157, 119, 138, 45, 18, 186, 83, 185, ]
, vec![83, 195, 100, 39, 188, 75, 66, 61, 241, 213, 109, 129, 94, 254, 225, 48, 90, 188, ]
, vec![15, 195, 244, 9, 233, 71, 168, 2, 188, 160, 153, 145, 253, 79, 108, 82, 27, 174, 186, 172, ]
, vec![52, 190, 88, 205, 109, 39, 176, 21, 155, 197, 251, 223, 155, 21, 5, 172, 254, 124, 12, 181, 184, 96, 50, 193, ]
, vec![211, 231, 43, 97, 71, 96, 103, 174, 37, 151, 170, 53, 75, 34, 249, 121, 17, 138, 110, 213, 141, 136, 120, 151, 233, 168, 93, 255, ]
, vec![245, 127, 242, 218, 130, 250, 162, 181, 102, 120, 84, 179, 220, 251, 80, 182, 229, 18, 2, 4, 68, 33, 101, 137, 95, 119, 115, 44, 175, 184, 59, 25, 225, 98, 81, 112, ]
, vec![77, 193, 137, 31, 19, 38, 22, 153, 247, 105, 122, 2, 245, 133, 242, 8, 175, 95, 100, 9, 167, 105, 214, 111, 57, 121, 21, 1, 253, 57, 54, 101, 248, 202, 69, 50, 150, 177, 226, 5, 9, 5, ]
, vec![245, 132, 172, 223, 96, 32, 117, 22, 238, 133, 238, 231, 205, 188, 237, 87, 191, 106, 16, 147, 118, 23, 37, 90, 170, 205, 131, 88, 120, 100, 66, 138, 186, 240, 82, 44, 176, 87, 187, 147, 160, 175, 69, 213, 92, 253, 225, 19, ]
, vec![175, 9, 223, 238, 12, 17, 220, 208, 100, 29, 175, 170, 230, 192, 215, 235, 150, 159, 36, 223, 38, 200, 132, 54, 228, 146, 218, 234, 117, 203, 29, 232, 144, 238, 22, 150, 201, 117, 62, 207, 164, 13, 137, 245, 127, 67, 247, 28, 155, 43, 203, 107, 233, 53, 143, 46, ]
, vec![242, 93, 169, 50, 144, 210, 39, 118, 202, 188, 201, 189, 143, 108, 196, 37, 185, 112, 134, 230, 245, 63, 197, 190, 250, 106, 185, 221, 175, 64, 114, 71, 161, 44, 147, 6, 27, 218, 51, 63, 87, 10, 40, 130, 188, 17, 163, 31, 176, 170, 4, 107, 232, 7, 94, 166, 224, 124, 86, 47, 11, 204, ]
, vec![220, 228, 173, 89, 251, 149, 159, 56, 89, 33, 147, 244, 154, 36, 73, 127, 213, 136, 248, 180, 234, 197, 158, 177, 68, 122, 93, 213, 15, 160, 227, 236, 66, 139, 153, 185, 202, 167, 179, 25, 220, 232, 96, 210, 231, 136, 223, 239, 181, 241, 59, 52, 172, 25, 49, 232, 211, 189, 64, 54, 108, 153, 132, 63, 96, 103, 82, 186, ]
, ]
;

 const MODULO_VALUE: i32 = 0x12D;

 const LOG: Vec<i32>;

 const ALOG: Vec<i32>;
pub struct ErrorCorrection {
}

impl ErrorCorrection {

    static {
        //Create log and antilog table
        LOG = : [i32; 256] = [0; 256];
        ALOG = : [i32; 255] = [0; 255];
         let mut p: i32 = 1;
         {
             let mut i: i32 = 0;
            while i < 255 {
                {
                    ALOG[i] = p;
                    LOG[p] = i;
                    p *= 2;
                    if p >= 256 {
                        p ^= MODULO_VALUE;
                    }
                }
                i += 1;
             }
         }

    }

    fn new() -> ErrorCorrection {
    }

    /**
   * Creates the ECC200 error correction for an encoded message.
   *
   * @param codewords  the codewords
   * @param symbolInfo information about the symbol to be encoded
   * @return the codewords with interleaved error correction.
   */
    pub fn  encode_e_c_c200( codewords: &String,  symbol_info: &SymbolInfo) -> String  {
        if codewords.length() != symbol_info.get_data_capacity() {
            throw IllegalArgumentException::new("The number of codewords does not match the selected symbol");
        }
         let sb: StringBuilder = StringBuilder::new(symbol_info.get_data_capacity() + symbol_info.get_error_codewords());
        sb.append(&codewords);
         let block_count: i32 = symbol_info.get_interleaved_block_count();
        if block_count == 1 {
             let ecc: String = ::create_e_c_c_block(&codewords, &symbol_info.get_error_codewords());
            sb.append(&ecc);
        } else {
            sb.set_length(&sb.capacity());
             let data_sizes: [i32; block_count] = [0; block_count];
             let error_sizes: [i32; block_count] = [0; block_count];
             {
                 let mut i: i32 = 0;
                while i < block_count {
                    {
                        data_sizes[i] = symbol_info.get_data_length_for_interleaved_block(i + 1);
                        error_sizes[i] = symbol_info.get_error_length_for_interleaved_block(i + 1);
                    }
                    i += 1;
                 }
             }

             {
                 let mut block: i32 = 0;
                while block < block_count {
                    {
                         let temp: StringBuilder = StringBuilder::new(data_sizes[block]);
                         {
                             let mut d: i32 = block;
                            while d < symbol_info.get_data_capacity() {
                                {
                                    temp.append(&codewords.char_at(d));
                                }
                                d += block_count;
                             }
                         }

                         let ecc: String = ::create_e_c_c_block(&temp.to_string(), error_sizes[block]);
                         let mut pos: i32 = 0;
                         {
                             let mut e: i32 = block;
                            while e < error_sizes[block] * block_count {
                                {
                                    sb.set_char_at(symbol_info.get_data_capacity() + e, &ecc.char_at(pos += 1 !!!check!!! post increment));
                                }
                                e += block_count;
                             }
                         }

                    }
                    block += 1;
                 }
             }

        }
        return sb.to_string();
    }

    fn  create_e_c_c_block( codewords: &CharSequence,  num_e_c_words: i32) -> String  {
         let mut table: i32 = -1;
         {
             let mut i: i32 = 0;
            while i < FACTOR_SETS.len() {
                {
                    if FACTOR_SETS[i] == num_e_c_words {
                        table = i;
                        break;
                    }
                }
                i += 1;
             }
         }

        if table < 0 {
            throw IllegalArgumentException::new(format!("Illegal number of error correction codewords specified: {}", num_e_c_words));
        }
         let poly: Vec<i32> = FACTORS[table];
         let mut ecc: [Option<char>; num_e_c_words] = [None; num_e_c_words];
         {
             let mut i: i32 = 0;
            while i < num_e_c_words {
                {
                    ecc[i] = 0;
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 0;
            while i < codewords.length() {
                {
                     let m: i32 = ecc[num_e_c_words - 1] ^ codewords.char_at(i);
                     {
                         let mut k: i32 = num_e_c_words - 1;
                        while k > 0 {
                            {
                                if m != 0 && poly[k] != 0 {
                                    ecc[k] = (ecc[k - 1] ^ ALOG[(LOG[m] + LOG[poly[k]]) % 255]) as char;
                                } else {
                                    ecc[k] = ecc[k - 1];
                                }
                            }
                            k -= 1;
                         }
                     }

                    if m != 0 && poly[0] != 0 {
                        ecc[0] = ALOG[(LOG[m] + LOG[poly[0]]) % 255] as char;
                    } else {
                        ecc[0] = 0;
                    }
                }
                i += 1;
             }
         }

         let ecc_reversed: [Option<char>; num_e_c_words] = [None; num_e_c_words];
         {
             let mut i: i32 = 0;
            while i < num_e_c_words {
                {
                    ecc_reversed[i] = ecc[num_e_c_words - i - 1];
                }
                i += 1;
             }
         }

        return String::value_of(&ecc_reversed);
    }
}

