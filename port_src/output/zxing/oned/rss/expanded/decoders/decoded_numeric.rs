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
// package com::google::zxing::oned::rss::expanded::decoders;

/**
 * @author Pablo OrduÃ±a, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */

 const FNC1: i32 = 10;
struct DecodedNumeric {
    super: DecodedObject;

     let first_digit: i32;

     let second_digit: i32;
}

impl DecodedNumeric {

    fn new( new_position: i32,  first_digit: i32,  second_digit: i32) -> DecodedNumeric throws FormatException {
        super(new_position);
        if first_digit < 0 || first_digit > 10 || second_digit < 0 || second_digit > 10 {
            throw FormatException::get_format_instance();
        }
        let .firstDigit = first_digit;
        let .secondDigit = second_digit;
    }

    fn  get_first_digit(&self) -> i32  {
        return self.firstDigit;
    }

    fn  get_second_digit(&self) -> i32  {
        return self.secondDigit;
    }

    fn  get_value(&self) -> i32  {
        return self.firstDigit * 10 + self.secondDigit;
    }

    fn  is_first_digit_f_n_c1(&self) -> bool  {
        return self.firstDigit == FNC1;
    }

    fn  is_second_digit_f_n_c1(&self) -> bool  {
        return self.secondDigit == FNC1;
    }
}

