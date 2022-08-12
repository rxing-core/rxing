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
 * One row of an RSS Expanded Stacked symbol, consisting of 1+ expanded pairs.
 */
struct ExpandedRow {

     let pairs: List<ExpandedPair>;

     let row_number: i32;
}

impl ExpandedRow {

    fn new( pairs: &List<ExpandedPair>,  row_number: i32) -> ExpandedRow {
        let .pairs = ArrayList<>::new(&pairs);
        let .rowNumber = row_number;
    }

    fn  get_pairs(&self) -> List<ExpandedPair>  {
        return self.pairs;
    }

    fn  get_row_number(&self) -> i32  {
        return self.rowNumber;
    }

    fn  is_equivalent(&self,  other_pairs: &List<ExpandedPair>) -> bool  {
        return self.pairs.equals(&other_pairs);
    }

    pub fn  to_string(&self) -> String  {
        return format!("{ {} }", self.pairs);
    }

    /**
   * Two rows are equal if they contain the same pairs in the same order.
   */
    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof ExpandedRow) {
            return false;
        }
         let that: ExpandedRow = o as ExpandedRow;
        return self.pairs.equals(that.pairs);
    }

    pub fn  hash_code(&self) -> i32  {
        return self.pairs.hash_code();
    }
}

