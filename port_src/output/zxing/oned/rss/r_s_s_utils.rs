/*
 * Copyright 2009 ZXing authors
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
// package com::google::zxing::oned::rss;

/** Adapted from listings in ISO/IEC 24724 Appendix B and Appendix G. */
pub struct RSSUtils {
}

impl RSSUtils {

    fn new() -> RSSUtils {
    }

    pub fn  get_r_s_svalue( widths: &Vec<i32>,  max_width: i32,  no_narrow: bool) -> i32  {
         let mut n: i32 = 0;
        for  let width: i32 in widths {
            n += width;
        }
         let mut val: i32 = 0;
         let narrow_mask: i32 = 0;
         let elements: i32 = widths.len();
         {
             let mut bar: i32 = 0;
            while bar < elements - 1 {
                {
                     let elm_width: i32;
                     {
                        elm_width = 1;
                        narrow_mask |= 1 << bar;
                        while elm_width < widths[bar] {
                            {
                                 let sub_val: i32 = ::combins(n - elm_width - 1, elements - bar - 2);
                                if no_narrow && (narrow_mask == 0) && (n - elm_width - (elements - bar - 1) >= elements - bar - 1) {
                                    sub_val -= ::combins(n - elm_width - (elements - bar), elements - bar - 2);
                                }
                                if elements - bar - 1 > 1 {
                                     let less_val: i32 = 0;
                                     {
                                         let mxw_element: i32 = n - elm_width - (elements - bar - 2);
                                        while mxw_element > max_width {
                                            {
                                                less_val += ::combins(n - elm_width - mxw_element - 1, elements - bar - 3);
                                            }
                                            mxw_element -= 1;
                                         }
                                     }

                                    sub_val -= less_val * (elements - 1 - bar);
                                } else if n - elm_width > max_width {
                                    sub_val -= 1;
                                }
                                val += sub_val;
                            }
                            elm_width += 1;
                            narrow_mask &= ~(1 << bar);
                         }
                     }

                    n -= elm_width;
                }
                bar += 1;
             }
         }

        return val;
    }

    fn  combins( n: i32,  r: i32) -> i32  {
         let max_denom: i32;
         let min_denom: i32;
        if n - r > r {
            min_denom = r;
            max_denom = n - r;
        } else {
            min_denom = n - r;
            max_denom = r;
        }
         let mut val: i32 = 1;
         let mut j: i32 = 1;
         {
             let mut i: i32 = n;
            while i > max_denom {
                {
                    val *= i;
                    if j <= min_denom {
                        val /= j;
                        j += 1;
                    }
                }
                i -= 1;
             }
         }

        while j <= min_denom {
            val /= j;
            j += 1;
        }
        return val;
    }
}

