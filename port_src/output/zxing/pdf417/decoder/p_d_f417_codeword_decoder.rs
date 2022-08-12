/*
 * Copyright 2013 ZXing authors
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
// package com::google::zxing::pdf417::decoder;

/**
 * @author Guenther Grau
 * @author creatale GmbH (christoph.schulz@creatale.de)
 */

 const RATIOS_TABLE: [[f32; PDF417Common.BARS_IN_MODULE]; PDF417Common.SYMBOL_TABLE.len()] = [[0.0; PDF417Common.BARS_IN_MODULE]; PDF417Common.SYMBOL_TABLE.len()];
struct PDF417CodewordDecoder {
}

impl PDF417CodewordDecoder {

    static {
        // Pre-computes the symbol ratio table.
         {
             let mut i: i32 = 0;
            while i < PDF417Common.SYMBOL_TABLE.len() {
                {
                     let current_symbol: i32 = PDF417Common.SYMBOL_TABLE[i];
                     let current_bit: i32 = current_symbol & 0x1;
                     {
                         let mut j: i32 = 0;
                        while j < PDF417Common.BARS_IN_MODULE {
                            {
                                 let mut size: f32 = 0.0f;
                                while (current_symbol & 0x1) == current_bit {
                                    size += 1.0f;
                                    current_symbol >>= 1;
                                }
                                current_bit = current_symbol & 0x1;
                                RATIOS_TABLE[i][PDF417Common.BARS_IN_MODULE - j - 1] = size / PDF417Common.MODULES_IN_CODEWORD;
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

    }

    fn new() -> PDF417CodewordDecoder {
    }

    fn  get_decoded_value( module_bit_count: &Vec<i32>) -> i32  {
         let decoded_value: i32 = ::get_decoded_codeword_value(&::sample_bit_counts(&module_bit_count));
        if decoded_value != -1 {
            return decoded_value;
        }
        return ::get_closest_decoded_value(&module_bit_count);
    }

    fn  sample_bit_counts( module_bit_count: &Vec<i32>) -> Vec<i32>  {
         let bit_count_sum: f32 = MathUtils::sum(&module_bit_count);
         let mut result: [i32; PDF417Common.BARS_IN_MODULE] = [0; PDF417Common.BARS_IN_MODULE];
         let bit_count_index: i32 = 0;
         let sum_previous_bits: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < PDF417Common.MODULES_IN_CODEWORD {
                {
                     let sample_index: f32 = bit_count_sum / (2.0 * PDF417Common.MODULES_IN_CODEWORD) + (i * bit_count_sum) / PDF417Common.MODULES_IN_CODEWORD;
                    if sum_previous_bits + module_bit_count[bit_count_index] <= sample_index {
                        sum_previous_bits += module_bit_count[bit_count_index];
                        bit_count_index += 1;
                    }
                    result[bit_count_index] += 1;
                }
                i += 1;
             }
         }

        return result;
    }

    fn  get_decoded_codeword_value( module_bit_count: &Vec<i32>) -> i32  {
         let decoded_value: i32 = ::get_bit_value(&module_bit_count);
        return  if PDF417Common::get_codeword(decoded_value) == -1 { -1 } else { decoded_value };
    }

    fn  get_bit_value( module_bit_count: &Vec<i32>) -> i32  {
         let mut result: i64 = 0;
         {
             let mut i: i32 = 0;
            while i < module_bit_count.len() {
                {
                     {
                         let mut bit: i32 = 0;
                        while bit < module_bit_count[i] {
                            {
                                result = (result << 1) | ( if i % 2 == 0 { 1 } else { 0 });
                            }
                            bit += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        return result as i32;
    }

    fn  get_closest_decoded_value( module_bit_count: &Vec<i32>) -> i32  {
         let bit_count_sum: i32 = MathUtils::sum(&module_bit_count);
         let bit_count_ratios: [f32; PDF417Common.BARS_IN_MODULE] = [0.0; PDF417Common.BARS_IN_MODULE];
        if bit_count_sum > 1 {
             {
                 let mut i: i32 = 0;
                while i < bit_count_ratios.len() {
                    {
                        bit_count_ratios[i] = module_bit_count[i] / bit_count_sum as f32;
                    }
                    i += 1;
                 }
             }

        }
         let best_match_error: f32 = Float::MAX_VALUE;
         let best_match: i32 = -1;
         {
             let mut j: i32 = 0;
            while j < RATIOS_TABLE.len() {
                {
                     let mut error: f32 = 0.0f;
                     let ratio_table_row: Vec<f32> = RATIOS_TABLE[j];
                     {
                         let mut k: i32 = 0;
                        while k < PDF417Common.BARS_IN_MODULE {
                            {
                                 let diff: f32 = ratio_table_row[k] - bit_count_ratios[k];
                                error += diff * diff;
                                if error >= best_match_error {
                                    break;
                                }
                            }
                            k += 1;
                         }
                     }

                    if error < best_match_error {
                        best_match_error = error;
                        best_match = PDF417Common.SYMBOL_TABLE[j];
                    }
                }
                j += 1;
             }
         }

        return best_match;
    }
}

