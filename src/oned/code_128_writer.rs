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

use rxing_one_d_proc_derive::OneDWriter;

use crate::common::Result;
use crate::BarcodeFormat;

use super::{code_128_reader, OneDimensionalCodeWriter};

const CODE_START_A: usize = 103;
const CODE_START_B: usize = 104;
const CODE_START_C: usize = 105;
const CODE_CODE_A: usize = 101;
const CODE_CODE_B: usize = 100;
const CODE_CODE_C: usize = 99;
const CODE_STOP: usize = 106;

// Dummy characters used to specify control characters in input
const ESCAPE_FNC_1: char = '\u{00f1}';
const ESCAPE_FNC_2: char = '\u{00f2}';
const ESCAPE_FNC_3: char = '\u{00f3}';
const ESCAPE_FNC_4: char = '\u{00f4}';

const CODE_FNC_1: usize = 102; // Code A, Code B, Code C
const CODE_FNC_2: usize = 97; // Code A, Code B
const CODE_FNC_3: usize = 96; // Code A, Code B
const CODE_FNC_4_A: usize = 101; // Code A
const CODE_FNC_4_B: usize = 100; // Code B

// RXingResults of minimal lookahead for code C
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CType {
    Uncodable,
    OneDigit,
    TwoDigits,
    Fnc1,
}

/**
 * This object renders a CODE128 code as a {@link BitMatrix}.
 *
 * @author erik.barbara@gmail.com (Erik Barbara)
 */
#[derive(OneDWriter, Default)]
pub struct Code128Writer;

impl OneDimensionalCodeWriter for Code128Writer {
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>> {
        self.encode_oned_with_hints(contents, &HashMap::new())
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<crate::BarcodeFormat>> {
        Some(vec![BarcodeFormat::CODE_128])
    }

    fn encode_oned_with_hints(
        &self,
        contents: &str,
        hints: &crate::EncodingHintDictionary,
    ) -> Result<Vec<bool>> {
        let forcedCodeSet = check(contents, hints)?;

        let hasCompactionHint = matches!(
            hints.get(&EncodeHintType::CODE128_COMPACT),
            Some(EncodeHintValue::Code128Compact(true))
        );
        // let hasCompactionHint = if let Some(EncodeHintValue::Code128Compact(compat)) =
        //     hints.get(&EncodeHintType::CODE128_COMPACT)
        // {
        //     *compat
        // } else {
        //     false
        // };
        // let hasCompactionHint = hints != null && hints.containsKey(EncodeHintType::CODE128_COMPACT) &&
        //     Boolean.parseBoolean(hints.get(EncodeHintType::CODE128_COMPACT).toString());

        if hasCompactionHint {
            MinimalEncoder::encode(contents)
        } else {
            encodeFast(contents, forcedCodeSet)
        }
    }
}

fn check(contents: &str, hints: &crate::EncodingHintDictionary) -> Result<i32> {
    let length = contents.chars().count();
    // Check length
    if !(1..=80).contains(&length) {
        return Err(Exceptions::illegal_argument_with(format!(
            "Contents length should be between 1 and 80 characters, but got {length}"
        )));
    }

    // Check for forced code set hint.
    let mut forcedCodeSet = -1_i32;
    if hints.contains_key(&EncodeHintType::FORCE_CODE_SET) {
        let Some(EncodeHintValue::ForceCodeSet(codeSetHint)) = hints.get(&EncodeHintType::FORCE_CODE_SET) else { return Err(Exceptions::ILLEGAL_STATE) };
        match codeSetHint.as_str() {
            "A" => forcedCodeSet = CODE_CODE_A as i32,
            "B" => forcedCodeSet = CODE_CODE_B as i32,
            "C" => forcedCodeSet = CODE_CODE_C as i32,
            _ => {
                return Err(Exceptions::illegal_argument_with(format!(
                    "Unsupported code set hint: {codeSetHint}"
                )))
            }
        }
    }

    // Check content
    for ch in contents.chars() {
        let c = ch as u32;
        // for (int i = 0; i < length; i++) {
        //   char c = contents.charAt(i);
        // check for non ascii characters that are not special GS1 characters
        match ch {
            // special function characters
            ESCAPE_FNC_1 | ESCAPE_FNC_2 | ESCAPE_FNC_3 | ESCAPE_FNC_4 => {}
            // non ascii characters
            _ => {
                if c > 127 {
                    // no full Latin-1 character set available at the moment
                    // shift and manual code change are not supported
                    return Err(Exceptions::illegal_argument_with(format!(
                        "Bad character in input: ASCII value={c}"
                    )));
                }
            }
        }
        // check characters for compatibility with forced code set
        const CODE_CODE_A_I32: i32 = CODE_CODE_A as i32;
        const CODE_CODE_B_I32: i32 = CODE_CODE_B as i32;
        const CODE_CODE_C_I32: i32 = CODE_CODE_C as i32;
        match forcedCodeSet {
            CODE_CODE_A_I32 =>
            // allows no ascii above 95 (no lower caps, no special symbols)
            {
                if c > 95 && c <= 127 {
                    return Err(Exceptions::illegal_argument_with(format!(
                        "Bad character in input for forced code set A: ASCII value={c}"
                    )));
                }
            }
            CODE_CODE_B_I32 =>
            // allows no ascii below 32 (terminal symbols)
            {
                if c <= 32 {
                    return Err(Exceptions::illegal_argument_with(format!(
                        "Bad character in input for forced code set B: ASCII value={c}"
                    )));
                }
            }
            CODE_CODE_C_I32 =>
            // allows only numbers and no FNC 2/3/4
            {
                if c < 48
                    || (c > 57 && c <= 127)
                    || ch == ESCAPE_FNC_2
                    || ch == ESCAPE_FNC_3
                    || ch == ESCAPE_FNC_4
                {
                    return Err(Exceptions::illegal_argument_with(format!(
                        "Bad character in input for forced code set C: ASCII value={c}"
                    )));
                }
            }
            _ => {}
        }
    }
    Ok(forcedCodeSet)
}

fn encodeFast(contents: &str, forcedCodeSet: i32) -> Result<Vec<bool>> {
    let length = contents.chars().count();

    let mut patterns: Vec<Vec<usize>> = Vec::new(); //new ArrayList<>(); // temporary storage for patterns
    let mut checkSum = 0;
    let mut checkWeight = 1;
    let mut codeSet = 0; // selected code (CODE_CODE_B or CODE_CODE_C)
    let mut position = 0; // position in contents

    while position < length {
        //Select code to use
        let newCodeSet = if forcedCodeSet == -1 {
            chooseCode(contents, position, codeSet).ok_or(Exceptions::ILLEGAL_STATE)?
        } else {
            forcedCodeSet as usize // THIS IS RISKY
        };

        //Get the pattern index
        let mut patternIndex: isize;
        if newCodeSet == codeSet {
            // Encode the current character
            // First handle escapes
            match contents
                .chars()
                .nth(position)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
            {
                ESCAPE_FNC_1 => patternIndex = CODE_FNC_1 as isize,
                ESCAPE_FNC_2 => patternIndex = CODE_FNC_2 as isize,
                ESCAPE_FNC_3 => patternIndex = CODE_FNC_3 as isize,
                ESCAPE_FNC_4 => {
                    if codeSet == CODE_CODE_A {
                        patternIndex = CODE_FNC_4_A as isize;
                    } else {
                        patternIndex = CODE_FNC_4_B as isize;
                    }
                }
                _ =>
                // Then handle normal characters otherwise
                {
                    match codeSet {
                        CODE_CODE_A => {
                            patternIndex = contents
                                .chars()
                                .nth(position)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                                as isize
                                - ' ' as isize;
                            if patternIndex < 0 {
                                // everything below a space character comes behind the underscore in the code patterns table
                                patternIndex += '`' as isize;
                            }
                        }
                        CODE_CODE_B => {
                            patternIndex = contents
                                .chars()
                                .nth(position)
                                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                                as isize
                                - ' ' as isize
                        }
                        _ => {
                            // CODE_CODE_C
                            if position + 1 == length {
                                // this is the last character, but the encoding is C, which always encodes two characers
                                return Err(Exceptions::illegal_argument_with(
                                    "Bad number of characters for digit only encoding.",
                                ));
                            }
                            let s: String = contents
                                .char_indices()
                                .skip(position)
                                .take(2)
                                .map(|(_u, c)| c)
                                .collect();
                            patternIndex = s.parse::<isize>().map_err(|e| {
                                Exceptions::parse_with(format!("issue parsing {s}: {e}"))
                            })?;
                            position += 1;
                        } // Also incremented below
                    }
                }
            }
            position += 1;
        } else {
            // Should we change the current code?
            // Do we have a code set?
            if codeSet == 0 {
                // No, we don't have a code set
                match newCodeSet {
                    CODE_CODE_A => patternIndex = CODE_START_A as isize,
                    CODE_CODE_B => patternIndex = CODE_START_B as isize,
                    _ => patternIndex = CODE_START_C as isize,
                }
            } else {
                // Yes, we have a code set
                patternIndex = newCodeSet as isize;
            }
            codeSet = newCodeSet;
        }

        // Get the pattern
        patterns.push(
            code_128_reader::CODE_PATTERNS[patternIndex as usize]
                .iter()
                .map(|x| *x as usize)
                .collect(),
        );

        // Compute checksum
        checkSum += patternIndex * checkWeight;
        if position != 0 {
            checkWeight += 1;
        }
    }

    Ok(produceRXingResult(&mut patterns, checkSum as usize))
}

fn produceRXingResult(patterns: &mut Vec<Vec<usize>>, checkSum: usize) -> Vec<bool> {
    // Compute and append checksum
    let mut checkSum = checkSum;
    checkSum %= 103;
    patterns.push(
        code_128_reader::CODE_PATTERNS[checkSum]
            .iter()
            .map(|x| *x as usize)
            .collect(),
    );

    // Append stop code
    patterns.push(
        code_128_reader::CODE_PATTERNS[CODE_STOP]
            .iter()
            .map(|x| *x as usize)
            .collect(),
    );

    // Compute code width
    let mut codeWidth = 0_usize;
    for pattern in &mut *patterns {
        codeWidth += pattern.iter().sum::<usize>();
    }

    // Compute result
    let mut result = vec![false; codeWidth];
    let mut pos = 0;
    for pattern in patterns {
        // for (int[] pattern : patterns) {
        pos += Code128Writer::appendPattern(&mut result, pos, pattern, true) as usize;
    }

    result
}

fn findCType(value: &str, start: usize) -> Option<CType> {
    let last = value.chars().count();
    if start >= last {
        return Some(CType::Uncodable);
    }
    let c = value.chars().nth(start)?;
    if c == ESCAPE_FNC_1 {
        return Some(CType::Fnc1);
    }
    if !c.is_ascii_digit() {
        return Some(CType::Uncodable);
    }
    if start + 1 >= last {
        return Some(CType::OneDigit);
    }
    let c = value.chars().nth(start + 1)?;
    if !c.is_ascii_digit() {
        return Some(CType::OneDigit);
    }
    Some(CType::TwoDigits)
}

fn chooseCode(value: &str, start: usize, oldCode: usize) -> Option<usize> {
    let mut lookahead = findCType(value, start)?;
    if lookahead == CType::OneDigit {
        if oldCode == CODE_CODE_A {
            return Some(CODE_CODE_A);
        }
        return Some(CODE_CODE_B);
    }
    if lookahead == CType::Uncodable {
        if start < value.chars().count() {
            let c = value.chars().nth(start)?;
            if c < ' '
                || (oldCode == CODE_CODE_A && (c < '`' || (c >= ESCAPE_FNC_1 && c <= ESCAPE_FNC_4)))
            {
                // can continue in code A, encodes ASCII 0 to 95 or FNC1 to FNC4
                return Some(CODE_CODE_A);
            }
        }
        return Some(CODE_CODE_B); // no choice
    }
    if oldCode == CODE_CODE_A && lookahead == CType::Fnc1 {
        return Some(CODE_CODE_A);
    }
    if oldCode == CODE_CODE_C {
        // can continue in code C
        return Some(CODE_CODE_C);
    }
    if oldCode == CODE_CODE_B {
        if lookahead == CType::Fnc1 {
            return Some(CODE_CODE_B); // can continue in code B
        }
        // Seen two consecutive digits, see what follows
        lookahead = findCType(value, start + 2)?;
        if lookahead == CType::Uncodable || lookahead == CType::OneDigit {
            return Some(CODE_CODE_B); // not worth switching now
        }
        if lookahead == CType::Fnc1 {
            // two digits, then FNC_1...
            lookahead = findCType(value, start + 3)?;
            if lookahead == CType::TwoDigits {
                // then two more digits, switch
                return Some(CODE_CODE_C);
            } else {
                return Some(CODE_CODE_B); // otherwise not worth switching
            }
        }
        // At this point, there are at least 4 consecutive digits.
        // Look ahead to choose whether to switch now or on the next round.
        let mut index = start + 4;
        let mut lookahead = findCType(value, index)?;
        while lookahead == CType::TwoDigits {
            // while (lookahead = findCType(value, index)) == CType::TWO_DIGITS {
            index += 2;
            lookahead = findCType(value, index)?;
        }
        if lookahead == CType::OneDigit {
            // odd number of digits, switch later
            return Some(CODE_CODE_B);
        }
        return Some(CODE_CODE_C); // even number of digits, switch now
    }
    // Here oldCode == 0, which means we are choosing the initial code
    if lookahead == CType::Fnc1 {
        // ignore FNC_1
        lookahead = findCType(value, start + 1)?;
    }
    if lookahead == CType::TwoDigits {
        // at least two digits, start in code C
        return Some(CODE_CODE_C);
    }
    Some(CODE_CODE_B)
}

/**
 * Encodes minimally using Divide-And-Conquer with Memoization
 **/
// struct MinimalEncoder {
//    memoizedCost:Vec<Vec<u32>>,
//    minPath:Vec<Vec<Latch>>,
// }
mod MinimalEncoder {
    use crate::{common::Result, oned::code_128_reader, Exceptions};

    use super::{
        produceRXingResult, CODE_CODE_A, CODE_CODE_B, CODE_CODE_C, CODE_FNC_1, CODE_FNC_2,
        CODE_FNC_3, CODE_FNC_4_A, CODE_FNC_4_B, CODE_START_A, CODE_START_B, CODE_START_C,
        ESCAPE_FNC_1, ESCAPE_FNC_2, ESCAPE_FNC_3, ESCAPE_FNC_4,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Charset {
        A,
        B,
        C,
        None,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Latch {
        A,
        B,
        C,
        Shift,
        None,
    }

    const A : &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_\u{0000}\u{0001}\u{0002}/
\u{0003}\u{0004}\u{0005}\u{0006}\u{0007}\u{0008}\u{0009}\n\u{000B}\u{000C}\r\u{000E}\u{000F}\u{0010}\u{0011}/
\u{0012}\u{0013}\u{0014}\u{0015}\u{0016}\u{0017}\u{0018}\u{0019}\u{001A}\u{001B}\u{001C}\u{001D}\u{001E}\u{001F}/
\u{00FF}";
    const B: &str =
        " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqr\
stuvwxyz{|}~\u{007F}\u{00FF}";

    const CODE_SHIFT: usize = 98;

    pub fn encode(contents: &str) -> Result<Vec<bool>> {
        let length = contents.chars().count();
        let mut memoizedCost = vec![vec![0_u32; length]; 4]; //new int[4][contents.length()];
        let mut minPath = vec![vec![Latch::None; length]; 4]; //new Latch[4][contents.length()];

        encode_with_start_position(contents, Charset::None, 0, &mut memoizedCost, &mut minPath)?;

        let mut patterns: Vec<Vec<usize>> = Vec::new(); //new ArrayList<>();
        let mut checkSum = vec![0_usize]; //new int[] {0};
        let mut checkWeight = vec![1]; //new int[] {1};
        let mut charset = Charset::None;
        let mut i = 0;
        while i < length {
            // for i in 0..length {
            // for (int i = 0; i < length; i++) {
            let latch = minPath[charset.ordinal()][i];
            match latch {
                Latch::A => {
                    charset = Charset::A;
                    addPattern(
                        &mut patterns,
                        if i == 0 { CODE_START_A } else { CODE_CODE_A },
                        &mut checkSum,
                        &mut checkWeight,
                        i,
                    );
                }
                Latch::B => {
                    charset = Charset::B;
                    addPattern(
                        &mut patterns,
                        if i == 0 { CODE_START_B } else { CODE_CODE_B },
                        &mut checkSum,
                        &mut checkWeight,
                        i,
                    );
                }
                Latch::C => {
                    charset = Charset::C;
                    addPattern(
                        &mut patterns,
                        if i == 0 { CODE_START_C } else { CODE_CODE_C },
                        &mut checkSum,
                        &mut checkWeight,
                        i,
                    );
                }
                Latch::Shift => addPattern(
                    &mut patterns,
                    CODE_SHIFT,
                    &mut checkSum,
                    &mut checkWeight,
                    i,
                ),
                Latch::None => { /* skip */ }
            }
            if charset == Charset::C {
                if contents
                    .chars()
                    .nth(i)
                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                    == ESCAPE_FNC_1
                {
                    addPattern(
                        &mut patterns,
                        CODE_FNC_1,
                        &mut checkSum,
                        &mut checkWeight,
                        i,
                    );
                } else {
                    let s: String = contents
                        .char_indices()
                        .skip(i)
                        .take(2)
                        .map(|(_u, c)| c)
                        .collect();
                    addPattern(
                        &mut patterns,
                        s.parse::<usize>().map_err(|e| {
                            Exceptions::parse_with(format!("unable to parse {s} {e}"))
                        })?,
                        &mut checkSum,
                        &mut checkWeight,
                        i,
                    );
                    assert!(i + 1 < length); //the algorithm never leads to a single trailing digit in character set C
                    if i + 1 < length {
                        i += 1;
                    }
                }
            } else {
                // charset A or B
                let mut patternIndex = match contents
                    .chars()
                    .nth(i)
                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                {
                    ESCAPE_FNC_1 => CODE_FNC_1 as isize,
                    ESCAPE_FNC_2 => CODE_FNC_2 as isize,
                    ESCAPE_FNC_3 => CODE_FNC_3 as isize,
                    ESCAPE_FNC_4 => {
                        if (charset == Charset::A && latch != Latch::Shift)
                            || (charset == Charset::B && latch == Latch::Shift)
                        {
                            CODE_FNC_4_A as isize
                        } else {
                            CODE_FNC_4_B as isize
                        }
                    }
                    _ => {
                        contents
                            .chars()
                            .nth(i)
                            .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                            as isize
                            - ' ' as isize
                    }
                };
                if ((charset == Charset::A && latch != Latch::Shift)
                    || (charset == Charset::B && latch == Latch::Shift))
                    && patternIndex < 0
                {
                    patternIndex += '`' as isize;
                }
                addPattern(
                    &mut patterns,
                    patternIndex as usize,
                    &mut checkSum,
                    &mut checkWeight,
                    i,
                );
            }

            i += 1;
        }
        // memoizedCost.clear();
        // minPath.clear();

        Ok(produceRXingResult(&mut patterns, checkSum[0]))
    }

    fn addPattern(
        patterns: &mut Vec<Vec<usize>>,
        patternIndex: usize,
        checkSum: &mut [usize],
        checkWeight: &mut [u32],
        position: usize,
    ) {
        patterns.push(
            code_128_reader::CODE_PATTERNS[patternIndex]
                .iter()
                .map(|x| *x as usize)
                .collect(),
        );
        if position != 0 {
            checkWeight[0] += 1;
        }
        checkSum[0] += patternIndex * checkWeight[0] as usize;
    }

    fn isDigit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn canEncode(contents: &str, charset: Charset, position: usize) -> bool {
        let Some(c) = contents.chars().nth(position) else {
            return false;
        };
        match charset {
            Charset::A => {
                c == ESCAPE_FNC_1
                    || c == ESCAPE_FNC_2
                    || c == ESCAPE_FNC_3
                    || c == ESCAPE_FNC_4
                    || A.find(c).is_some()
            }
            Charset::B => {
                c == ESCAPE_FNC_1
                    || c == ESCAPE_FNC_2
                    || c == ESCAPE_FNC_3
                    || c == ESCAPE_FNC_4
                    || B.find(c).is_some()
            }
            Charset::C => {
                let Some(c_p_1) = contents.chars().nth(position + 1) else {
                    return false;
                };
                c == ESCAPE_FNC_1
                    || (position + 1 < contents.chars().count() && isDigit(c) && isDigit(c_p_1))
            }
            _ => false,
        }
    }

    /**
     * Encode the string starting at position position starting with the character set charset
     **/
    fn encode_with_start_position(
        contents: &str,
        charset: Charset,
        position: usize,
        memoizedCost: &mut Vec<Vec<u32>>,
        minPath: &mut Vec<Vec<Latch>>,
    ) -> Result<u32> {
        if position >= contents.chars().count() {
            return Err(Exceptions::ILLEGAL_STATE);
        }
        let mCost = memoizedCost[charset.ordinal()][position];
        if mCost > 0 {
            return Ok(mCost);
        }

        let mut minCost = u32::MAX;
        let mut minLatch = Latch::None;
        let atEnd = position + 1 >= contents.chars().count();

        let sets = [Charset::A, Charset::B];
        for i in 0..=1 {
            // for (int i = 0; i <= 1; i++) {
            if canEncode(contents, sets[i], position) {
                let mut cost = 1;
                let mut latch = Latch::None;
                if charset != sets[i] {
                    cost += 1;
                    latch = sets[i].into();
                }
                if !atEnd {
                    cost += encode_with_start_position(
                        contents,
                        sets[i],
                        position + 1,
                        memoizedCost,
                        minPath,
                    )?;
                }
                if cost < minCost {
                    minCost = cost;
                    minLatch = latch;
                }
                cost = 1;
                if charset == sets[(i + 1) % 2] {
                    cost += 1;
                    latch = Latch::Shift;
                    if !atEnd {
                        cost += encode_with_start_position(
                            contents,
                            charset,
                            position + 1,
                            memoizedCost,
                            minPath,
                        )?;
                    }
                    if cost < minCost {
                        minCost = cost;
                        minLatch = latch;
                    }
                }
            }
        }
        if canEncode(contents, Charset::C, position) {
            let mut cost = 1;
            let mut latch = Latch::None;
            if charset != Charset::C {
                cost += 1;
                latch = Latch::C;
            }
            let advance = if contents.chars().nth(position).unwrap_or_default() == ESCAPE_FNC_1 {
                1
            } else {
                2
            };
            if position + advance < contents.chars().count() {
                cost += encode_with_start_position(
                    contents,
                    Charset::C,
                    position + advance,
                    memoizedCost,
                    minPath,
                )?;
            }
            if cost < minCost {
                minCost = cost;
                minLatch = latch;
            }
        }
        if minCost == u32::MAX {
            return Err(Exceptions::illegal_argument_with(format!(
                "Bad character in input: ASCII value={}",
                contents.chars().nth(position).unwrap_or('x')
            )));
            // throw new IllegalArgumentException("Bad character in input: ASCII value=" + (int) contents.charAt(position));
        }
        memoizedCost[charset.ordinal()][position] = minCost;
        minPath[charset.ordinal()][position] = minLatch;
        Ok(minCost)
    }

    trait HasOrdinal {
        fn ordinal(&self) -> usize;
    }

    impl HasOrdinal for Charset {
        fn ordinal(&self) -> usize {
            match self {
                Charset::A => 0,
                Charset::B => 1,
                Charset::C => 2,
                Charset::None => 3,
            }
        }
    }
    impl HasOrdinal for Latch {
        fn ordinal(&self) -> usize {
            match self {
                Latch::A => 0,
                Latch::B => 1,
                Latch::C => 2,
                Latch::Shift => 3,
                Latch::None => 4,
            }
        }
    }
    impl From<Charset> for Latch {
        fn from(cs: Charset) -> Self {
            match cs {
                Charset::A => Latch::A,
                Charset::B => Latch::B,
                Charset::C => Latch::C,
                Charset::None => Latch::None,
            }
        }
    }
}
