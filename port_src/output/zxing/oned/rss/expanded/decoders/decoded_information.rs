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
struct DecodedInformation {
    super: DecodedObject;

     let new_string: String;

     let remaining_value: i32;

     let mut remaining: bool;
}

impl DecodedInformation {

    fn new( new_position: i32,  new_string: &String) -> DecodedInformation {
        super(new_position);
        let .newString = new_string;
        let .remaining = false;
        let .remainingValue = 0;
    }

    fn new( new_position: i32,  new_string: &String,  remaining_value: i32) -> DecodedInformation {
        super(new_position);
        let .remaining = true;
        let .remainingValue = remaining_value;
        let .newString = new_string;
    }

    fn  get_new_string(&self) -> String  {
        return self.newString;
    }

    fn  is_remaining(&self) -> bool  {
        return self.remaining;
    }

    fn  get_remaining_value(&self) -> i32  {
        return self.remainingValue;
    }
}

