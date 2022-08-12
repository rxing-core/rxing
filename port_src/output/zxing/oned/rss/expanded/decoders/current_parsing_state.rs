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
 */
struct CurrentParsingState {

     let mut position: i32;

     let mut encoding: State;
}

impl CurrentParsingState {

    enum State {

        NUMERIC(), ALPHA(), ISO_IEC_646()
    }

    fn new() -> CurrentParsingState {
        let .position = 0;
        let .encoding = State::NUMERIC;
    }

    fn  get_position(&self) -> i32  {
        return self.position;
    }

    fn  set_position(&self,  position: i32)   {
        self.position = position;
    }

    fn  increment_position(&self,  delta: i32)   {
        self.position += delta;
    }

    fn  is_alpha(&self) -> bool  {
        return self.encoding == State::ALPHA;
    }

    fn  is_numeric(&self) -> bool  {
        return self.encoding == State::NUMERIC;
    }

    fn  is_iso_iec646(&self) -> bool  {
        return self.encoding == State::ISO_IEC_646;
    }

    fn  set_numeric(&self)   {
        self.encoding = State::NUMERIC;
    }

    fn  set_alpha(&self)   {
        self.encoding = State::ALPHA;
    }

    fn  set_iso_iec646(&self)   {
        self.encoding = State::ISO_IEC_646;
    }
}

