/*
 * Copyright 2010 ZXing authors
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
// package com::google::zxing::oned;

/**
 * This object renders a CODE128 code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */

 const CODE_START_A: i32 = 103;

 const CODE_START_B: i32 = 104;

 const CODE_START_C: i32 = 105;

 const CODE_CODE_A: i32 = 101;

 const CODE_CODE_B: i32 = 100;

 const CODE_CODE_C: i32 = 99;

 const CODE_STOP: i32 = 106;

// Dummy characters used to specify control characters in input
 const ESCAPE_FNC_1: char = 'ñ';

 const ESCAPE_FNC_2: char = 'ò';

 const ESCAPE_FNC_3: char = 'ó';

 const ESCAPE_FNC_4: char = 'ô';

// Code A, Code B, Code C
 const CODE_FNC_1: i32 = 102;

// Code A, Code B
 const CODE_FNC_2: i32 = 97;

// Code A, Code B
 const CODE_FNC_3: i32 = 96;

// Code A
 const CODE_FNC_4_A: i32 = 101;

// Code B
 const CODE_FNC_4_B: i32 = 100;
pub struct Code128Writer {
    super: OneDimensionalCodeWriter;
}

impl Code128Writer {

    // Results of minimal lookahead for code C
    enum CType {

        UNCODABLE(), ONE_DIGIT(), TWO_DIGITS(), FNC_1()
    }

    pub fn  get_supported_write_formats(&self) -> Collection<BarcodeFormat>  {
        return Collections::singleton(BarcodeFormat::CODE_128);
    }

    pub fn  encode(&self,  contents: &String) -> Vec<bool>  {
        return self.encode(&contents, null);
    }

    pub fn  encode(&self,  contents: &String,  hints: &Map<EncodeHintType, ?>) -> Vec<bool>  {
         let forced_code_set: i32 = ::check(&contents, &hints);
         let has_compaction_hint: bool = hints != null && hints.contains_key(EncodeHintType::CODE128_COMPACT) && Boolean::parse_boolean(&hints.get(EncodeHintType::CODE128_COMPACT).to_string());
        return  if has_compaction_hint { MinimalEncoder::new().encode(&contents) } else { ::encode_fast(&contents, forced_code_set) };
    }

    fn  check( contents: &String,  hints: &Map<EncodeHintType, ?>) -> i32  {
         let length: i32 = contents.length();
        // Check length
        if length < 1 || length > 80 {
            throw IllegalArgumentException::new(format!("Contents length should be between 1 and 80 characters, but got {}", length));
        }
        // Check for forced code set hint.
         let forced_code_set: i32 = -1;
        if hints != null && hints.contains_key(EncodeHintType::FORCE_CODE_SET) {
             let code_set_hint: String = hints.get(EncodeHintType::FORCE_CODE_SET).to_string();
            match code_set_hint {
                  "A" => 
                     {
                        forced_code_set = CODE_CODE_A;
                        break;
                    }
                  "B" => 
                     {
                        forced_code_set = CODE_CODE_B;
                        break;
                    }
                  "C" => 
                     {
                        forced_code_set = CODE_CODE_C;
                        break;
                    }
                _ => 
                     {
                        throw IllegalArgumentException::new(format!("Unsupported code set hint: {}", code_set_hint));
                    }
            }
        }
        // Check content
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let c: char = contents.char_at(i);
                    // check for non ascii characters that are not special GS1 characters
                    match c {
                        // special function characters
                          ESCAPE_FNC_1 => 
                             {
                            }
                          ESCAPE_FNC_2 => 
                             {
                            }
                          ESCAPE_FNC_3 => 
                             {
                            }
                          ESCAPE_FNC_4 => 
                             {
                                break;
                            }
                        // non ascii characters
                        _ => 
                             {
                                if c > 127 {
                                    // shift and manual code change are not supported
                                    throw IllegalArgumentException::new(format!("Bad character in input: ASCII value={}", c as i32));
                                }
                            }
                    }
                    // check characters for compatibility with forced code set
                    match forced_code_set {
                          CODE_CODE_A => 
                             {
                                // allows no ascii above 95 (no lower caps, no special symbols)
                                if c > 95 && c <= 127 {
                                    throw IllegalArgumentException::new(format!("Bad character in input for forced code set A: ASCII value={}", c as i32));
                                }
                                break;
                            }
                          CODE_CODE_B => 
                             {
                                // allows no ascii below 32 (terminal symbols)
                                if c <= 32 {
                                    throw IllegalArgumentException::new(format!("Bad character in input for forced code set B: ASCII value={}", c as i32));
                                }
                                break;
                            }
                          CODE_CODE_C => 
                             {
                                // allows only numbers and no FNC 2/3/4
                                if c < 48 || (c > 57 && c <= 127) || c == ESCAPE_FNC_2 || c == ESCAPE_FNC_3 || c == ESCAPE_FNC_4 {
                                    throw IllegalArgumentException::new(format!("Bad character in input for forced code set C: ASCII value={}", c as i32));
                                }
                                break;
                            }
                    }
                }
                i += 1;
             }
         }

        return forced_code_set;
    }

    fn  encode_fast( contents: &String,  forced_code_set: i32) -> Vec<bool>  {
         let length: i32 = contents.length();
        // temporary storage for patterns
         let patterns: Collection<Vec<i32>> = ArrayList<>::new();
         let check_sum: i32 = 0;
         let check_weight: i32 = 1;
        // selected code (CODE_CODE_B or CODE_CODE_C)
         let code_set: i32 = 0;
        // position in contents
         let mut position: i32 = 0;
        while position < length {
            //Select code to use
             let new_code_set: i32;
            if forced_code_set == -1 {
                new_code_set = ::choose_code(&contents, position, code_set);
            } else {
                new_code_set = forced_code_set;
            }
            //Get the pattern index
             let pattern_index: i32;
            if new_code_set == code_set {
                // First handle escapes
                match contents.char_at(position) {
                      ESCAPE_FNC_1 => 
                         {
                            pattern_index = CODE_FNC_1;
                            break;
                        }
                      ESCAPE_FNC_2 => 
                         {
                            pattern_index = CODE_FNC_2;
                            break;
                        }
                      ESCAPE_FNC_3 => 
                         {
                            pattern_index = CODE_FNC_3;
                            break;
                        }
                      ESCAPE_FNC_4 => 
                         {
                            if code_set == CODE_CODE_A {
                                pattern_index = CODE_FNC_4_A;
                            } else {
                                pattern_index = CODE_FNC_4_B;
                            }
                            break;
                        }
                    _ => 
                         {
                            // Then handle normal characters otherwise
                            match code_set {
                                  CODE_CODE_A => 
                                     {
                                        pattern_index = contents.char_at(position) - ' ';
                                        if pattern_index < 0 {
                                            // everything below a space character comes behind the underscore in the code patterns table
                                            pattern_index += '`';
                                        }
                                        break;
                                    }
                                  CODE_CODE_B => 
                                     {
                                        pattern_index = contents.char_at(position) - ' ';
                                        break;
                                    }
                                _ => 
                                     {
                                        // CODE_CODE_C
                                        if position + 1 == length {
                                            // this is the last character, but the encoding is C, which always encodes two characers
                                            throw IllegalArgumentException::new("Bad number of characters for digit only encoding.");
                                        }
                                        pattern_index = Integer::parse_int(&contents.substring(position, position + 2));
                                        // Also incremented below
                                        position += 1;
                                        break;
                                    }
                            }
                        }
                }
                position += 1;
            } else {
                // Do we have a code set?
                if code_set == 0 {
                    // No, we don't have a code set
                    match new_code_set {
                          CODE_CODE_A => 
                             {
                                pattern_index = CODE_START_A;
                                break;
                            }
                          CODE_CODE_B => 
                             {
                                pattern_index = CODE_START_B;
                                break;
                            }
                        _ => 
                             {
                                pattern_index = CODE_START_C;
                                break;
                            }
                    }
                } else {
                    // Yes, we have a code set
                    pattern_index = new_code_set;
                }
                code_set = new_code_set;
            }
            // Get the pattern
            patterns.add(Code128Reader::CODE_PATTERNS[pattern_index]);
            // Compute checksum
            check_sum += pattern_index * check_weight;
            if position != 0 {
                check_weight += 1;
            }
        }
        return ::produce_result(&patterns, check_sum);
    }

    fn  produce_result( patterns: &Collection<Vec<i32>>,  check_sum: i32) -> Vec<bool>  {
        // Compute and append checksum
        check_sum %= 103;
        patterns.add(Code128Reader::CODE_PATTERNS[check_sum]);
        // Append stop code
        patterns.add(Code128Reader::CODE_PATTERNS[CODE_STOP]);
        // Compute code width
         let code_width: i32 = 0;
        for  let pattern: Vec<i32> in patterns {
            for  let width: i32 in pattern {
                code_width += width;
            }
        }
        // Compute result
         let result: [bool; code_width] = [false; code_width];
         let mut pos: i32 = 0;
        for  let pattern: Vec<i32> in patterns {
            pos += append_pattern(&result, pos, &pattern, true);
        }
        return result;
    }

    fn  find_c_type( value: &CharSequence,  start: i32) -> CType  {
         let last: i32 = value.length();
        if start >= last {
            return CType.UNCODABLE;
        }
         let mut c: char = value.char_at(start);
        if c == ESCAPE_FNC_1 {
            return CType.FNC_1;
        }
        if c < '0' || c > '9' {
            return CType.UNCODABLE;
        }
        if start + 1 >= last {
            return CType.ONE_DIGIT;
        }
        c = value.char_at(start + 1);
        if c < '0' || c > '9' {
            return CType.ONE_DIGIT;
        }
        return CType.TWO_DIGITS;
    }

    fn  choose_code( value: &CharSequence,  start: i32,  old_code: i32) -> i32  {
         let mut lookahead: CType = ::find_c_type(&value, start);
        if lookahead == CType.ONE_DIGIT {
            if old_code == CODE_CODE_A {
                return CODE_CODE_A;
            }
            return CODE_CODE_B;
        }
        if lookahead == CType.UNCODABLE {
            if start < value.length() {
                 let c: char = value.char_at(start);
                if c < ' ' || (old_code == CODE_CODE_A && (c < '`' || (c >= ESCAPE_FNC_1 && c <= ESCAPE_FNC_4))) {
                    // can continue in code A, encodes ASCII 0 to 95 or FNC1 to FNC4
                    return CODE_CODE_A;
                }
            }
            // no choice
            return CODE_CODE_B;
        }
        if old_code == CODE_CODE_A && lookahead == CType.FNC_1 {
            return CODE_CODE_A;
        }
        if old_code == CODE_CODE_C {
            // can continue in code C
            return CODE_CODE_C;
        }
        if old_code == CODE_CODE_B {
            if lookahead == CType.FNC_1 {
                // can continue in code B
                return CODE_CODE_B;
            }
            // Seen two consecutive digits, see what follows
            lookahead = ::find_c_type(&value, start + 2);
            if lookahead == CType.UNCODABLE || lookahead == CType.ONE_DIGIT {
                // not worth switching now
                return CODE_CODE_B;
            }
            if lookahead == CType.FNC_1 {
                // two digits, then FNC_1...
                lookahead = ::find_c_type(&value, start + 3);
                if lookahead == CType.TWO_DIGITS {
                    // then two more digits, switch
                    return CODE_CODE_C;
                } else {
                    // otherwise not worth switching
                    return CODE_CODE_B;
                }
            }
            // At this point, there are at least 4 consecutive digits.
            // Look ahead to choose whether to switch now or on the next round.
             let mut index: i32 = start + 4;
            while (lookahead = ::find_c_type(&value, index)) == CType.TWO_DIGITS {
                index += 2;
            }
            if lookahead == CType.ONE_DIGIT {
                // odd number of digits, switch later
                return CODE_CODE_B;
            }
            // even number of digits, switch now
            return CODE_CODE_C;
        }
        // Here oldCode == 0, which means we are choosing the initial code
        if lookahead == CType.FNC_1 {
            // ignore FNC_1
            lookahead = ::find_c_type(&value, start + 1);
        }
        if lookahead == CType.TWO_DIGITS {
            // at least two digits, start in code C
            return CODE_CODE_C;
        }
        return CODE_CODE_B;
    }

    /** 
   * Encodes minimally using Divide-And-Conquer with Memoization
   **/

     const A: &'static str = format!(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_ 	\n\rÿ");

     const B: &'static str = format!(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~ÿ");

     const CODE_SHIFT: i32 = 98;
    struct MinimalEncoder {

         let memoized_cost: Vec<Vec<i32>>;

         let min_path: Vec<Vec<Latch>>;
    }
    
    impl MinimalEncoder {

        enum Charset {

            A(), B(), C(), NONE()
        }

        enum Latch {

            A(), B(), C(), SHIFT(), NONE()
        }

        fn  encode(&self,  contents: &String) -> Vec<bool>  {
            self.memoized_cost = : [[i32; contents.length()]; 4] = [[0; contents.length()]; 4];
            self.min_path = : [[Option<Latch>; contents.length()]; 4] = [[None; contents.length()]; 4];
            self.encode(&contents, Charset::NONE, 0);
             let patterns: Collection<Vec<i32>> = ArrayList<>::new();
             let check_sum : vec![i32; 1] = vec![0, ]
            ;
             let check_weight : vec![i32; 1] = vec![1, ]
            ;
             let length: i32 = contents.length();
             let mut charset: Charset = Charset::NONE;
             {
                 let mut i: i32 = 0;
                while i < length {
                    {
                         let latch: Latch = self.min_path[charset.ordinal()][i];
                        match latch {
                              A => 
                                 {
                                    charset = Charset::A;
                                    ::add_pattern(&patterns,  if i == 0 { CODE_START_A } else { CODE_CODE_A }, &check_sum, &check_weight, i);
                                    break;
                                }
                              B => 
                                 {
                                    charset = Charset::B;
                                    ::add_pattern(&patterns,  if i == 0 { CODE_START_B } else { CODE_CODE_B }, &check_sum, &check_weight, i);
                                    break;
                                }
                              C => 
                                 {
                                    charset = Charset::C;
                                    ::add_pattern(&patterns,  if i == 0 { CODE_START_C } else { CODE_CODE_C }, &check_sum, &check_weight, i);
                                    break;
                                }
                              SHIFT => 
                                 {
                                    ::add_pattern(&patterns, CODE_SHIFT, &check_sum, &check_weight, i);
                                    break;
                                }
                        }
                        if charset == Charset::C {
                            if contents.char_at(i) == ESCAPE_FNC_1 {
                                ::add_pattern(&patterns, CODE_FNC_1, &check_sum, &check_weight, i);
                            } else {
                                ::add_pattern(&patterns, &Integer::parse_int(&contents.substring(i, i + 2)), &check_sum, &check_weight, i);
                                //the algorithm never leads to a single trailing digit in character set C
                                assert!( i + 1 < length);
                                if i + 1 < length {
                                    i += 1;
                                }
                            }
                        } else {
                            // charset A or B
                             let pattern_index: i32;
                            match contents.char_at(i) {
                                  ESCAPE_FNC_1 => 
                                     {
                                        pattern_index = CODE_FNC_1;
                                        break;
                                    }
                                  ESCAPE_FNC_2 => 
                                     {
                                        pattern_index = CODE_FNC_2;
                                        break;
                                    }
                                  ESCAPE_FNC_3 => 
                                     {
                                        pattern_index = CODE_FNC_3;
                                        break;
                                    }
                                  ESCAPE_FNC_4 => 
                                     {
                                        if (charset == Charset::A && latch != Latch::SHIFT) || (charset == Charset::B && latch == Latch::SHIFT) {
                                            pattern_index = CODE_FNC_4_A;
                                        } else {
                                            pattern_index = CODE_FNC_4_B;
                                        }
                                        break;
                                    }
                                _ => 
                                     {
                                        pattern_index = contents.char_at(i) - ' ';
                                    }
                            }
                            if (charset == Charset::A && latch != Latch::SHIFT) || (charset == Charset::B && latch == Latch::SHIFT) {
                                if pattern_index < 0 {
                                    pattern_index += '`';
                                }
                            }
                            ::add_pattern(&patterns, pattern_index, &check_sum, &check_weight, i);
                        }
                    }
                    i += 1;
                 }
             }

            self.memoized_cost = null;
            self.min_path = null;
            return ::produce_result(&patterns, check_sum[0]);
        }

        fn  add_pattern( patterns: &Collection<Vec<i32>>,  pattern_index: i32,  check_sum: &Vec<i32>,  check_weight: &Vec<i32>,  position: i32)   {
            patterns.add(Code128Reader::CODE_PATTERNS[pattern_index]);
            if position != 0 {
                check_weight[0] += 1;
            }
            check_sum[0] += pattern_index * check_weight[0];
        }

        fn  is_digit( c: char) -> bool  {
            return c >= '0' && c <= '9';
        }

        fn  can_encode(&self,  contents: &CharSequence,  charset: &Charset,  position: i32) -> bool  {
             let c: char = contents.char_at(position);
            match charset {
                  A => 
                     {
                        return c == ESCAPE_FNC_1 || c == ESCAPE_FNC_2 || c == ESCAPE_FNC_3 || c == ESCAPE_FNC_4 || A::index_of(c) >= 0;
                    }
                  B => 
                     {
                        return c == ESCAPE_FNC_1 || c == ESCAPE_FNC_2 || c == ESCAPE_FNC_3 || c == ESCAPE_FNC_4 || B::index_of(c) >= 0;
                    }
                  C => 
                     {
                        return c == ESCAPE_FNC_1 || (position + 1 < contents.length() && ::is_digit(c) && ::is_digit(&contents.char_at(position + 1)));
                    }
                _ => 
                     {
                        return false;
                    }
            }
        }

        /**
     * Encode the string starting at position position starting with the character set charset
     **/
        fn  encode(&self,  contents: &CharSequence,  charset: &Charset,  position: i32) -> i32  {
            assert!( position < contents.length());
             let m_cost: i32 = self.memoized_cost[charset.ordinal()][position];
            if m_cost > 0 {
                return m_cost;
            }
             let min_cost: i32 = Integer::MAX_VALUE;
             let min_latch: Latch = Latch::NONE;
             let at_end: bool = position + 1 >= contents.length();
             let sets : vec![Charset; 2] = vec![Charset::A, Charset::B, ]
            ;
             {
                 let mut i: i32 = 0;
                while i <= 1 {
                    {
                        if self.can_encode(&contents, sets[i], position) {
                             let mut cost: i32 = 1;
                             let mut latch: Latch = Latch::NONE;
                            if charset != sets[i] {
                                cost += 1;
                                latch = Latch::value_of(&sets[i].to_string());
                            }
                            if !at_end {
                                cost += self.encode(&contents, sets[i], position + 1);
                            }
                            if cost < min_cost {
                                min_cost = cost;
                                min_latch = latch;
                            }
                            cost = 1;
                            if charset == sets[(i + 1) % 2] {
                                cost += 1;
                                latch = Latch::SHIFT;
                                if !at_end {
                                    cost += self.encode(&contents, charset, position + 1);
                                }
                                if cost < min_cost {
                                    min_cost = cost;
                                    min_latch = latch;
                                }
                            }
                        }
                    }
                    i += 1;
                 }
             }

            if self.can_encode(&contents, Charset::C, position) {
                 let mut cost: i32 = 1;
                 let mut latch: Latch = Latch::NONE;
                if charset != Charset::C {
                    cost += 1;
                    latch = Latch::C;
                }
                 let advance: i32 =  if contents.char_at(position) == ESCAPE_FNC_1 { 1 } else { 2 };
                if position + advance < contents.length() {
                    cost += self.encode(&contents, Charset::C, position + advance);
                }
                if cost < min_cost {
                    min_cost = cost;
                    min_latch = latch;
                }
            }
            if min_cost == Integer::MAX_VALUE {
                throw IllegalArgumentException::new(format!("Bad character in input: ASCII value={}", contents.char_at(position) as i32));
            }
            self.memoized_cost[charset.ordinal()][position] = min_cost;
            self.min_path[charset.ordinal()][position] = min_latch;
            return min_cost;
        }
    }

}

