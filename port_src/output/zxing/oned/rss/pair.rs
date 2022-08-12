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

struct Pair {
    super: DataCharacter;

     let finder_pattern: FinderPattern;

     let mut count: i32;
}

impl Pair {

    fn new( value: i32,  checksum_portion: i32,  finder_pattern: &FinderPattern) -> Pair {
        super(value, checksum_portion);
        let .finderPattern = finder_pattern;
    }

    fn  get_finder_pattern(&self) -> FinderPattern  {
        return self.finder_pattern;
    }

    fn  get_count(&self) -> i32  {
        return self.count;
    }

    fn  increment_count(&self)   {
        self.count += 1;
    }
}

