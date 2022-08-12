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
 */
struct ExpandedPair {

     let left_char: DataCharacter;

     let right_char: DataCharacter;

     let finder_pattern: FinderPattern;
}

impl ExpandedPair {

    fn new( left_char: &DataCharacter,  right_char: &DataCharacter,  finder_pattern: &FinderPattern) -> ExpandedPair {
        let .leftChar = left_char;
        let .rightChar = right_char;
        let .finderPattern = finder_pattern;
    }

    fn  get_left_char(&self) -> DataCharacter  {
        return self.leftChar;
    }

    fn  get_right_char(&self) -> DataCharacter  {
        return self.rightChar;
    }

    fn  get_finder_pattern(&self) -> FinderPattern  {
        return self.finderPattern;
    }

    fn  must_be_last(&self) -> bool  {
        return self.rightChar == null;
    }

    pub fn  to_string(&self) -> String  {
        return format!("[ {} , {} : {} ]", self.left_char, self.right_char, ( if self.finder_pattern == null { "null" } else { self.finder_pattern.get_value() }));
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof ExpandedPair) {
            return false;
        }
         let that: ExpandedPair = o as ExpandedPair;
        return Objects::equals(self.left_char, that.leftChar) && Objects::equals(self.right_char, that.rightChar) && Objects::equals(self.finder_pattern, that.finderPattern);
    }

    pub fn  hash_code(&self) -> i32  {
        return Objects::hash_code(self.left_char) ^ Objects::hash_code(self.right_char) ^ Objects::hash_code(self.finder_pattern);
    }
}

