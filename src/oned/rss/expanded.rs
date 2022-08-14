use crate::{BarcodeFormat,DecodeHintType,FormatException,NotFoundException,RXingResult,ResultMetadataType,ResultPoint};
use crate::common::BitArray;
use crate::oned::rss::{DataCharacter,FinderPattern,AbstractRSSReader,RSSUtils};
use crate::oned::rss::expanded::decoders::AbstractExpandedDecoder;


// NEW FILE: bit_array_builder.rs
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

// NEW FILE: expanded_pair.rs
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

// NEW FILE: expanded_row.rs
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

// NEW FILE: r_s_s_expanded_reader.rs
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

 const SYMBOL_WIDEST: vec![Vec<i32>; 5] = vec![7, 5, 4, 3, 1, ]
;

 const EVEN_TOTAL_SUBSET: vec![Vec<i32>; 5] = vec![4, 20, 52, 104, 204, ]
;

 const GSUM: vec![Vec<i32>; 5] = vec![0, 348, 1388, 2948, 3988, ]
;

 const FINDER_PATTERNS: vec![vec![Vec<Vec<i32>>; 4]; 6] = vec![// A
vec![1, 8, 4, 1, ]
, // B
vec![3, 6, 4, 1, ]
, // C
vec![3, 4, 6, 1, ]
, // D
vec![3, 2, 8, 1, ]
, // E
vec![2, 6, 5, 1, ]
, // F
vec![2, 2, 9, 1, ]
, ]
;

 const WEIGHTS: vec![vec![Vec<Vec<i32>>; 8]; 23] = vec![vec![1, 3, 9, 27, 81, 32, 96, 77, ]
, vec![20, 60, 180, 118, 143, 7, 21, 63, ]
, vec![189, 145, 13, 39, 117, 140, 209, 205, ]
, vec![193, 157, 49, 147, 19, 57, 171, 91, ]
, vec![62, 186, 136, 197, 169, 85, 44, 132, ]
, vec![185, 133, 188, 142, 4, 12, 36, 108, ]
, vec![113, 128, 173, 97, 80, 29, 87, 50, ]
, vec![150, 28, 84, 41, 123, 158, 52, 156, ]
, vec![46, 138, 203, 187, 139, 206, 196, 166, ]
, vec![76, 17, 51, 153, 37, 111, 122, 155, ]
, vec![43, 129, 176, 106, 107, 110, 119, 146, ]
, vec![16, 48, 144, 10, 30, 90, 59, 177, ]
, vec![109, 116, 137, 200, 178, 112, 125, 164, ]
, vec![70, 210, 208, 202, 184, 130, 179, 115, ]
, vec![134, 191, 151, 31, 93, 68, 204, 190, ]
, vec![148, 22, 66, 198, 172, 94, 71, 2, ]
, vec![6, 18, 54, 162, 64, 192, 154, 40, ]
, vec![120, 149, 25, 75, 14, 42, 126, 167, ]
, vec![79, 26, 78, 23, 69, 207, 199, 175, ]
, vec![103, 98, 83, 38, 114, 131, 182, 124, ]
, vec![161, 61, 183, 127, 170, 88, 53, 159, ]
, vec![55, 165, 73, 8, 24, 72, 5, 15, ]
, vec![45, 135, 194, 160, 58, 174, 100, 89, ]
, ]
;

 const FINDER_PAT_A: i32 = 0;

 const FINDER_PAT_B: i32 = 1;

 const FINDER_PAT_C: i32 = 2;

 const FINDER_PAT_D: i32 = 3;

 const FINDER_PAT_E: i32 = 4;

 const FINDER_PAT_F: i32 = 5;

 const FINDER_PATTERN_SEQUENCES: vec![vec![Vec<Vec<i32>>; 11]; 10] = vec![vec![FINDER_PAT_A, FINDER_PAT_A, ]
, vec![FINDER_PAT_A, FINDER_PAT_B, FINDER_PAT_B, ]
, vec![FINDER_PAT_A, FINDER_PAT_C, FINDER_PAT_B, FINDER_PAT_D, ]
, vec![FINDER_PAT_A, FINDER_PAT_E, FINDER_PAT_B, FINDER_PAT_D, FINDER_PAT_C, ]
, vec![FINDER_PAT_A, FINDER_PAT_E, FINDER_PAT_B, FINDER_PAT_D, FINDER_PAT_D, FINDER_PAT_F, ]
, vec![FINDER_PAT_A, FINDER_PAT_E, FINDER_PAT_B, FINDER_PAT_D, FINDER_PAT_E, FINDER_PAT_F, FINDER_PAT_F, ]
, vec![FINDER_PAT_A, FINDER_PAT_A, FINDER_PAT_B, FINDER_PAT_B, FINDER_PAT_C, FINDER_PAT_C, FINDER_PAT_D, FINDER_PAT_D, ]
, vec![FINDER_PAT_A, FINDER_PAT_A, FINDER_PAT_B, FINDER_PAT_B, FINDER_PAT_C, FINDER_PAT_C, FINDER_PAT_D, FINDER_PAT_E, FINDER_PAT_E, ]
, vec![FINDER_PAT_A, FINDER_PAT_A, FINDER_PAT_B, FINDER_PAT_B, FINDER_PAT_C, FINDER_PAT_C, FINDER_PAT_D, FINDER_PAT_E, FINDER_PAT_F, FINDER_PAT_F, ]
, vec![FINDER_PAT_A, FINDER_PAT_A, FINDER_PAT_B, FINDER_PAT_B, FINDER_PAT_C, FINDER_PAT_D, FINDER_PAT_D, FINDER_PAT_E, FINDER_PAT_E, FINDER_PAT_F, FINDER_PAT_F, ]
, ]
;

 const MAX_PAIRS: i32 = 11;
pub struct RSSExpandedReader {
    super: AbstractRSSReader;

     let pairs: List<ExpandedPair> = ArrayList<>::new(MAX_PAIRS);

     let rows: List<ExpandedRow> = ArrayList<>::new();

     let start_end: [i32; 2] = [0; 2];

     let start_from_even: bool;
}

impl RSSExpandedReader {

    pub fn  decode_row(&self,  row_number: i32,  row: &BitArray,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        // Rows can start with even pattern in case in prev rows there where odd number of patters.
        // So lets try twice
        self.pairs.clear();
        self.startFromEven = false;
        let tryResult1 = 0;
        'try1: loop {
        {
            return Ok(::construct_result(&self.decode_row2pairs(row_number, row)));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &NotFoundException) {
            }  0 => break
        }

        self.pairs.clear();
        self.startFromEven = true;
        return Ok(::construct_result(&self.decode_row2pairs(row_number, row)));
    }

    pub fn  reset(&self)   {
        self.pairs.clear();
        self.rows.clear();
    }

    // Not private for testing
    fn  decode_row2pairs(&self,  row_number: i32,  row: &BitArray) -> /*  throws NotFoundException */Result<List<ExpandedPair>, Rc<Exception>>   {
         let mut done: bool = false;
        while !done {
            let tryResult1 = 0;
            'try1: loop {
            {
                self.pairs.add(&self.retrieve_next_pair(row, self.pairs, row_number));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( nfe: &NotFoundException) {
                    if self.pairs.is_empty() {
                        throw nfe;
                    }
                    done = true;
                }  0 => break
            }

        }
        // TODO: verify sequence of finder patterns as in checkPairSequence()
        if self.check_checksum() {
            return Ok(self.pairs);
        }
         let try_stacked_decode: bool = !self.rows.is_empty();
        // TODO: deal with reversed rows
        self.store_row(row_number);
        if try_stacked_decode {
            // When the image is 180-rotated, then rows are sorted in wrong direction.
            // Try twice with both the directions.
             let mut ps: List<ExpandedPair> = self.check_rows(false);
            if ps != null {
                return Ok(ps);
            }
            ps = self.check_rows(true);
            if ps != null {
                return Ok(ps);
            }
        }
        throw NotFoundException::get_not_found_instance();
    }

    fn  check_rows(&self,  reverse: bool) -> List<ExpandedPair>  {
        // Stacked barcode can have up to 11 rows, so 25 seems reasonable enough
        if self.rows.size() > 25 {
            // We will never have a chance to get result, so clear it
            self.rows.clear();
            return Ok(null);
        }
        self.pairs.clear();
        if reverse {
            Collections::reverse(self.rows);
        }
         let mut ps: List<ExpandedPair> = null;
        let tryResult1 = 0;
        'try1: loop {
        {
            ps = self.check_rows(ArrayList<>::new(), 0);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &NotFoundException) {
            }  0 => break
        }

        if reverse {
            Collections::reverse(self.rows);
        }
        return Ok(ps);
    }

    // Try to construct a valid rows sequence
    // Recursion is used to implement backtracking
    fn  check_rows(&self,  collected_rows: &List<ExpandedRow>,  current_row: i32) -> /*  throws NotFoundException */Result<List<ExpandedPair>, Rc<Exception>>   {
         {
             let mut i: i32 = current_row;
            while i < self.rows.size() {
                {
                     let row: ExpandedRow = self.rows.get(i);
                    self.pairs.clear();
                    for  let collected_row: ExpandedRow in collected_rows {
                        self.pairs.add_all(&collected_row.get_pairs());
                    }
                    self.pairs.add_all(&row.get_pairs());
                    if ::is_valid_sequence(self.pairs) {
                        if self.check_checksum() {
                            return Ok(self.pairs);
                        }
                         let rs: List<ExpandedRow> = ArrayList<>::new(&collected_rows);
                        rs.add(row);
                        let tryResult1 = 0;
                        'try1: loop {
                        {
                            // Recursion: try to add more rows
                            return Ok(self.check_rows(&rs, i + 1));
                        }
                        break 'try1
                        }
                        match tryResult1 {
                             catch ( e: &NotFoundException) {
                            }  0 => break
                        }

                    }
                }
                i += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    // Whether the pairs form a valid find pattern sequence,
    // either complete or a prefix
    fn  is_valid_sequence( pairs: &List<ExpandedPair>) -> bool  {
        for  let sequence: Vec<i32> in FINDER_PATTERN_SEQUENCES {
            if pairs.size() <= sequence.len() {
                 let mut stop: bool = true;
                 {
                     let mut j: i32 = 0;
                    while j < pairs.size() {
                        {
                            if pairs.get(j).get_finder_pattern().get_value() != sequence[j] {
                                stop = false;
                                break;
                            }
                        }
                        j += 1;
                     }
                 }

                if stop {
                    return true;
                }
            }
        }
        return false;
    }

    fn  store_row(&self,  row_number: i32)   {
        // Discard if duplicate above or below; otherwise insert in order by row number.
         let insert_pos: i32 = 0;
         let prev_is_same: bool = false;
         let next_is_same: bool = false;
        while insert_pos < self.rows.size() {
             let erow: ExpandedRow = self.rows.get(insert_pos);
            if erow.get_row_number() > row_number {
                next_is_same = erow.is_equivalent(self.pairs);
                break;
            }
            prev_is_same = erow.is_equivalent(self.pairs);
            insert_pos += 1;
        }
        if next_is_same || prev_is_same {
            return;
        }
        // Check whether the row is part of an already detected row
        if ::is_partial_row(self.pairs, self.rows) {
            return;
        }
        self.rows.add(insert_pos, ExpandedRow::new(self.pairs, row_number));
        ::remove_partial_rows(self.pairs, self.rows);
    }

    // Remove all the rows that contains only specified pairs
    fn  remove_partial_rows( pairs: &Collection<ExpandedPair>,  rows: &Collection<ExpandedRow>)   {
         {
             let iterator: Iterator<ExpandedRow> = rows.iterator();
            while iterator.has_next(){
                 let r: ExpandedRow = iterator.next();
                if r.get_pairs().size() != pairs.size() {
                     let all_found: bool = true;
                    for  let p: ExpandedPair in r.get_pairs() {
                        if !pairs.contains(p) {
                            all_found = false;
                            break;
                        }
                    }
                    if all_found {
                        // 'pairs' contains all the pairs from the row 'r'
                        iterator.remove();
                    }
                }
            }
         }

    }

    // Returns true when one of the rows already contains all the pairs
    fn  is_partial_row( pairs: &Iterable<ExpandedPair>,  rows: &Iterable<ExpandedRow>) -> bool  {
        for  let r: ExpandedRow in rows {
             let all_found: bool = true;
            for  let p: ExpandedPair in pairs {
                 let mut found: bool = false;
                for  let pp: ExpandedPair in r.get_pairs() {
                    if p.equals(pp) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    all_found = false;
                    break;
                }
            }
            if all_found {
                // the row 'r' contain all the pairs from 'pairs'
                return true;
            }
        }
        return false;
    }

    // Only used for unit testing
    fn  get_rows(&self) -> List<ExpandedRow>  {
        return self.rows;
    }

    // Not private for unit testing
    fn  construct_result( pairs: &List<ExpandedPair>) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
         let binary: BitArray = BitArrayBuilder::build_bit_array(&pairs);
         let decoder: AbstractExpandedDecoder = AbstractExpandedDecoder::create_decoder(binary);
         let resulting_string: String = decoder.parse_information();
         let first_points: Vec<ResultPoint> = pairs.get(0).get_finder_pattern().get_result_points();
         let last_points: Vec<ResultPoint> = pairs.get(pairs.size() - 1).get_finder_pattern().get_result_points();
         let result: Result = Result::new(&resulting_string, null,  : vec![ResultPoint; 4] = vec![first_points[0], first_points[1], last_points[0], last_points[1], ]
        , BarcodeFormat::RSS_EXPANDED);
        result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, "]e0");
        return Ok(result);
    }

    fn  check_checksum(&self) -> bool  {
         let first_pair: ExpandedPair = self.pairs.get(0);
         let check_character: DataCharacter = first_pair.get_left_char();
         let first_character: DataCharacter = first_pair.get_right_char();
        if first_character == null {
            return false;
        }
         let mut checksum: i32 = first_character.get_checksum_portion();
         let mut s: i32 = 2;
         {
             let mut i: i32 = 1;
            while i < self.pairs.size() {
                {
                     let current_pair: ExpandedPair = self.pairs.get(i);
                    checksum += current_pair.get_left_char().get_checksum_portion();
                    s += 1;
                     let current_right_char: DataCharacter = current_pair.get_right_char();
                    if current_right_char != null {
                        checksum += current_right_char.get_checksum_portion();
                        s += 1;
                    }
                }
                i += 1;
             }
         }

        checksum %= 211;
         let check_character_value: i32 = 211 * (s - 4) + checksum;
        return check_character_value == check_character.get_value();
    }

    fn  get_next_second_bar( row: &BitArray,  initial_pos: i32) -> i32  {
         let current_pos: i32;
        if row.get(initial_pos) {
            current_pos = row.get_next_unset(initial_pos);
            current_pos = row.get_next_set(current_pos);
        } else {
            current_pos = row.get_next_set(initial_pos);
            current_pos = row.get_next_unset(current_pos);
        }
        return current_pos;
    }

    // not private for testing
    fn  retrieve_next_pair(&self,  row: &BitArray,  previous_pairs: &List<ExpandedPair>,  row_number: i32) -> /*  throws NotFoundException */Result<ExpandedPair, Rc<Exception>>   {
         let is_odd_pattern: bool = previous_pairs.size() % 2 == 0;
        if self.start_from_even {
            is_odd_pattern = !is_odd_pattern;
        }
         let mut pattern: FinderPattern;
         let keep_finding: bool = true;
         let forced_offset: i32 = -1;
        loop { {
            self.find_next_pair(row, &previous_pairs, forced_offset);
            pattern = self.parse_found_finder_pattern(row, row_number, is_odd_pattern);
            if pattern == null {
                forced_offset = ::get_next_second_bar(row, self.startEnd[0]);
            } else {
                keep_finding = false;
            }
        }if !(keep_finding) break;}
        // When stacked symbol is split over multiple rows, there's no way to guess if this pair can be last or not.
        // boolean mayBeLast = checkPairSequence(previousPairs, pattern);
         let left_char: DataCharacter = self.decode_data_character(row, pattern, is_odd_pattern, true);
        if !previous_pairs.is_empty() && previous_pairs.get(previous_pairs.size() - 1).must_be_last() {
            throw NotFoundException::get_not_found_instance();
        }
         let right_char: DataCharacter;
        let tryResult1 = 0;
        'try1: loop {
        {
            right_char = self.decode_data_character(row, pattern, is_odd_pattern, false);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &NotFoundException) {
                right_char = null;
            }  0 => break
        }

        return Ok(ExpandedPair::new(left_char, right_char, pattern));
    }

    fn  find_next_pair(&self,  row: &BitArray,  previous_pairs: &List<ExpandedPair>,  forced_offset: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.get_decode_finder_counters();
        counters[0] = 0;
        counters[1] = 0;
        counters[2] = 0;
        counters[3] = 0;
         let width: i32 = row.get_size();
         let row_offset: i32;
        if forced_offset >= 0 {
            row_offset = forced_offset;
        } else if previous_pairs.is_empty() {
            row_offset = 0;
        } else {
             let last_pair: ExpandedPair = previous_pairs.get(previous_pairs.size() - 1);
            row_offset = last_pair.get_finder_pattern().get_start_end()[1];
        }
         let searching_even_pair: bool = previous_pairs.size() % 2 != 0;
        if self.start_from_even {
            searching_even_pair = !searching_even_pair;
        }
         let is_white: bool = false;
        while row_offset < width {
            is_white = !row.get(row_offset);
            if !is_white {
                break;
            }
            row_offset += 1;
        }
         let counter_position: i32 = 0;
         let pattern_start: i32 = row_offset;
         {
             let mut x: i32 = row_offset;
            while x < width {
                {
                    if row.get(x) != is_white {
                        counters[counter_position] += 1;
                    } else {
                        if counter_position == 3 {
                            if searching_even_pair {
                                ::reverse_counters(&counters);
                            }
                            if is_finder_pattern(&counters) {
                                self.startEnd[0] = pattern_start;
                                self.startEnd[1] = x;
                                return;
                            }
                            if searching_even_pair {
                                ::reverse_counters(&counters);
                            }
                            pattern_start += counters[0] + counters[1];
                            counters[0] = counters[2];
                            counters[1] = counters[3];
                            counters[2] = 0;
                            counters[3] = 0;
                            counter_position -= 1;
                        } else {
                            counter_position += 1;
                        }
                        counters[counter_position] = 1;
                        is_white = !is_white;
                    }
                }
                x += 1;
             }
         }

        throw NotFoundException::get_not_found_instance();
    }

    fn  reverse_counters( counters: &Vec<i32>)   {
         let mut length: i32 = counters.len();
         {
             let mut i: i32 = 0;
            while i < length / 2 {
                {
                     let tmp: i32 = counters[i];
                    counters[i] = counters[length - i - 1];
                    counters[length - i - 1] = tmp;
                }
                i += 1;
             }
         }

    }

    fn  parse_found_finder_pattern(&self,  row: &BitArray,  row_number: i32,  odd_pattern: bool) -> FinderPattern  {
        // Actually we found elements 2-5.
         let first_counter: i32;
         let mut start: i32;
         let mut end: i32;
        if odd_pattern {
            // If pattern number is odd, we need to locate element 1 *before* the current block.
             let first_element_start: i32 = self.startEnd[0] - 1;
            // Locate element 1
            while first_element_start >= 0 && !row.get(first_element_start) {
                first_element_start -= 1;
            }
            first_element_start += 1;
            first_counter = self.startEnd[0] - first_element_start;
            start = first_element_start;
            end = self.startEnd[1];
        } else {
            // If pattern number is even, the pattern is reversed, so we need to locate element 1 *after* the current block.
            start = self.startEnd[0];
            end = row.get_next_unset(self.startEnd[1] + 1);
            first_counter = end - self.startEnd[1];
        }
        // Make 'counters' hold 1-4
         let mut counters: Vec<i32> = self.get_decode_finder_counters();
        System::arraycopy(&counters, 0, &counters, 1, counters.len() - 1);
        counters[0] = first_counter;
         let mut value: i32;
        let tryResult1 = 0;
        'try1: loop {
        {
            value = parse_finder_value(&counters, &FINDER_PATTERNS);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ignored: &NotFoundException) {
                return null;
            }  0 => break
        }

        return FinderPattern::new(value,  : vec![i32; 2] = vec![start, end, ]
        , start, end, row_number);
    }

    fn  decode_data_character(&self,  row: &BitArray,  pattern: &FinderPattern,  is_odd_pattern: bool,  left_char: bool) -> /*  throws NotFoundException */Result<DataCharacter, Rc<Exception>>   {
         let mut counters: Vec<i32> = self.get_data_character_counters();
        Arrays::fill(&counters, 0);
        if left_char {
            record_pattern_in_reverse(row, pattern.get_start_end()[0], &counters);
        } else {
            record_pattern(row, pattern.get_start_end()[1], &counters);
            // reverse it
             {
                 let mut i: i32 = 0, let mut j: i32 = counters.len() - 1;
                while i < j {
                    {
                         let temp: i32 = counters[i];
                        counters[i] = counters[j];
                        counters[j] = temp;
                    }
                    i += 1;
                    j -= 1;
                 }
             }

        }
        //counters[] has the pixels of the module
        //left and right data characters have all the same length
         let num_modules: i32 = 17;
         let element_width: f32 = MathUtils::sum(&counters) / num_modules as f32;
        // Sanity check: element width for pattern and the character should match
         let expected_element_width: f32 = (pattern.get_start_end()[1] - pattern.get_start_end()[0]) / 15.0f;
        if Math::abs(element_width - expected_element_width) / expected_element_width > 0.3f {
            throw NotFoundException::get_not_found_instance();
        }
         let odd_counts: Vec<i32> = self.get_odd_counts();
         let even_counts: Vec<i32> = self.get_even_counts();
         let odd_rounding_errors: Vec<f32> = self.get_odd_rounding_errors();
         let even_rounding_errors: Vec<f32> = self.get_even_rounding_errors();
         {
             let mut i: i32 = 0;
            while i < counters.len() {
                {
                     let value: f32 = 1.0f * counters[i] / element_width;
                    // Round
                     let mut count: i32 = (value + 0.5f) as i32;
                    if count < 1 {
                        if value < 0.3f {
                            throw NotFoundException::get_not_found_instance();
                        }
                        count = 1;
                    } else if count > 8 {
                        if value > 8.7f {
                            throw NotFoundException::get_not_found_instance();
                        }
                        count = 8;
                    }
                     let mut offset: i32 = i / 2;
                    if (i & 0x01) == 0 {
                        odd_counts[offset] = count;
                        odd_rounding_errors[offset] = value - count;
                    } else {
                        even_counts[offset] = count;
                        even_rounding_errors[offset] = value - count;
                    }
                }
                i += 1;
             }
         }

        self.adjust_odd_even_counts(num_modules);
         let weight_row_number: i32 = 4 * pattern.get_value() + ( if is_odd_pattern { 0 } else { 2 }) + ( if left_char { 0 } else { 1 }) - 1;
         let odd_sum: i32 = 0;
         let odd_checksum_portion: i32 = 0;
         {
             let mut i: i32 = odd_counts.len() - 1;
            while i >= 0 {
                {
                    if ::is_not_a1left(pattern, is_odd_pattern, left_char) {
                         let weight: i32 = WEIGHTS[weight_row_number][2 * i];
                        odd_checksum_portion += odd_counts[i] * weight;
                    }
                    odd_sum += odd_counts[i];
                }
                i -= 1;
             }
         }

         let even_checksum_portion: i32 = 0;
         {
             let mut i: i32 = even_counts.len() - 1;
            while i >= 0 {
                {
                    if ::is_not_a1left(pattern, is_odd_pattern, left_char) {
                         let weight: i32 = WEIGHTS[weight_row_number][2 * i + 1];
                        even_checksum_portion += even_counts[i] * weight;
                    }
                }
                i -= 1;
             }
         }

         let checksum_portion: i32 = odd_checksum_portion + even_checksum_portion;
        if (odd_sum & 0x01) != 0 || odd_sum > 13 || odd_sum < 4 {
            throw NotFoundException::get_not_found_instance();
        }
         let group: i32 = (13 - odd_sum) / 2;
         let odd_widest: i32 = SYMBOL_WIDEST[group];
         let even_widest: i32 = 9 - odd_widest;
         let v_odd: i32 = RSSUtils::get_r_s_svalue(&odd_counts, odd_widest, true);
         let v_even: i32 = RSSUtils::get_r_s_svalue(&even_counts, even_widest, false);
         let t_even: i32 = EVEN_TOTAL_SUBSET[group];
         let g_sum: i32 = GSUM[group];
         let value: i32 = v_odd * t_even + v_even + g_sum;
        return Ok(DataCharacter::new(value, checksum_portion));
    }

    fn  is_not_a1left( pattern: &FinderPattern,  is_odd_pattern: bool,  left_char: bool) -> bool  {
        // A1: pattern.getValue is 0 (A), and it's an oddPattern, and it is a left char
        return !(pattern.get_value() == 0 && is_odd_pattern && left_char);
    }

    fn  adjust_odd_even_counts(&self,  num_modules: i32)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         let odd_sum: i32 = MathUtils::sum(&self.get_odd_counts());
         let even_sum: i32 = MathUtils::sum(&self.get_even_counts());
         let increment_odd: bool = false;
         let decrement_odd: bool = false;
        if odd_sum > 13 {
            decrement_odd = true;
        } else if odd_sum < 4 {
            increment_odd = true;
        }
         let increment_even: bool = false;
         let decrement_even: bool = false;
        if even_sum > 13 {
            decrement_even = true;
        } else if even_sum < 4 {
            increment_even = true;
        }
         let mismatch: i32 = odd_sum + even_sum - num_modules;
         let odd_parity_bad: bool = (odd_sum & 0x01) == 1;
         let even_parity_bad: bool = (even_sum & 0x01) == 0;
        match mismatch {
              1 => 
                 {
                    if odd_parity_bad {
                        if even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        decrement_odd = true;
                    } else {
                        if !even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        decrement_even = true;
                    }
                    break;
                }
              -1 => 
                 {
                    if odd_parity_bad {
                        if even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        increment_odd = true;
                    } else {
                        if !even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        increment_even = true;
                    }
                    break;
                }
              0 => 
                 {
                    if odd_parity_bad {
                        if !even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                        // Both bad
                        if odd_sum < even_sum {
                            increment_odd = true;
                            decrement_even = true;
                        } else {
                            decrement_odd = true;
                            increment_even = true;
                        }
                    } else {
                        if even_parity_bad {
                            throw NotFoundException::get_not_found_instance();
                        }
                    // Nothing to do!
                    }
                    break;
                }
            _ => 
                 {
                    throw NotFoundException::get_not_found_instance();
                }
        }
        if increment_odd {
            if decrement_odd {
                throw NotFoundException::get_not_found_instance();
            }
            increment(&self.get_odd_counts(), &self.get_odd_rounding_errors());
        }
        if decrement_odd {
            decrement(&self.get_odd_counts(), &self.get_odd_rounding_errors());
        }
        if increment_even {
            if decrement_even {
                throw NotFoundException::get_not_found_instance();
            }
            increment(&self.get_even_counts(), &self.get_odd_rounding_errors());
        }
        if decrement_even {
            decrement(&self.get_even_counts(), &self.get_even_rounding_errors());
        }
    }
}

