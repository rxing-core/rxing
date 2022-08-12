/*
 * Copyright (C) 2010 ZXing authors
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
// package com::google::zxing::oned::rss::expanded;

/**
 * @author Pablo OrduÃ±a, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
struct BitArrayBuilder {
}

impl BitArrayBuilder {

    fn new() -> BitArrayBuilder {
    }

    fn  build_bit_array( pairs: &List<ExpandedPair>) -> BitArray  {
         let char_number: i32 = (pairs.size() * 2) - 1;
        if pairs.get(pairs.size() - 1).get_right_char() == null {
            char_number -= 1;
        }
         let size: i32 = 12 * char_number;
         let binary: BitArray = BitArray::new(size);
         let acc_pos: i32 = 0;
         let first_pair: ExpandedPair = pairs.get(0);
         let first_value: i32 = first_pair.get_right_char().get_value();
         {
             let mut i: i32 = 11;
            while i >= 0 {
                {
                    if (first_value & (1 << i)) != 0 {
                        binary.set(acc_pos);
                    }
                    acc_pos += 1;
                }
                i -= 1;
             }
         }

         {
             let mut i: i32 = 1;
            while i < pairs.size() {
                {
                     let current_pair: ExpandedPair = pairs.get(i);
                     let left_value: i32 = current_pair.get_left_char().get_value();
                     {
                         let mut j: i32 = 11;
                        while j >= 0 {
                            {
                                if (left_value & (1 << j)) != 0 {
                                    binary.set(acc_pos);
                                }
                                acc_pos += 1;
                            }
                            j -= 1;
                         }
                     }

                    if current_pair.get_right_char() != null {
                         let right_value: i32 = current_pair.get_right_char().get_value();
                         {
                             let mut j: i32 = 11;
                            while j >= 0 {
                                {
                                    if (right_value & (1 << j)) != 0 {
                                        binary.set(acc_pos);
                                    }
                                    acc_pos += 1;
                                }
                                j -= 1;
                             }
                         }

                    }
                }
                i += 1;
             }
         }

        return binary;
    }
}

