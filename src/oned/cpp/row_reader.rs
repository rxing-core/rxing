use crate::common::cpp_essentials::{
    BarAndSpace, GetPatternRow, NormalizedPattern, PatternRow, PatternType, ToInt, UpdateMinMax,
};
use crate::common::Result;
use crate::qrcode::cpp_port::detector::AppendBit;
use crate::{common::cpp_essentials::PatternView, RXingResult};

use super::dxfilm_edge_reader::Clock;

/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

/*
Code39 : 1:2/3, 5+4+1 (0x3|2x1 wide) -> 12-15 mods, v1-? | ToNarrowWide(OMG 1) == *
Codabar: 1:2/3, 4+3+1 (1x1|1x2|3x0 wide) -> 9-13 mods, v1-? | ToNarrowWide(OMG 2) == ABCD
ITF    : 1:2/3, 5+5   (2x2 wide) -> mods, v6-?| .5, .38 == * | qz:10

Code93 : 1-4, 3+3 -> 9 mods  v1-? | round to 1-4 == *
Code128: 1-4, 3+3 -> 11 mods v1-? | .7, .25 == ABC | qz:10
UPC/EAN: 1-4, 2+2 -> 7 mods  f    | .7, .48 == *
  UPC-A: 11d 95m = 3 + 6*4 + 5 + 6*4 + 3 = 59 | qz:3
  EAN-13: 12d 95m
  UPC-E: 6d, 3 + 6*4 + 6 = 33
  EAN-8: 8d, 3 + 4*4 + 5 + 4*4 + 3 = 43

RSS14  : 1-8, finder: (15,2+3), symbol: (15/16,4+4) | .45, .2 (finder only), 14d
  code = 2xguard + 2xfinder + 4xsymbol = (96,23), stacked = 2x50 mods
RSSExp.:  v?-74d/?-41c
*/

type Pattern = PatternRow;
type Counter = PatternRow;
type Index = Vec<u32>;
type Alphabet = Vec<char>;

// pub trait DecodingState: Default {
//     // virtual ~DecodingState() = default;
// }

#[derive(Default, Debug, Clone)]
pub struct DecodingState {
    // DXO
    pub centerRow: u32,
    pub clocks: Vec<Clock>,
}

/**
* Encapsulates functionality and implementation that is common to all families
* of one-dimensional barcodes.
*/
pub trait RowReader {
    // type DS: DecodingState;

    fn decodePattern(
        &self,
        rowNumber: u32,
        next: &mut PatternView,
        state: &mut Option<DecodingState>,
    ) -> Result<RXingResult>;

    /**
     * Determines how closely a set of observed counts of runs of black/white values matches a given
     * target pattern. This is reported as the ratio of the total variance from the expected pattern
     * proportions across all pattern elements, to the length of the pattern.
     *
     * @param counters observed counters
     * @param pattern expected pattern
     * @param maxIndividualVariance The most any counter can differ before we give up
     * @return ratio of total variance between counters and pattern compared to total pattern size
     */
    fn PatternMatchVariance(
        counters: &Counter,
        pattern: &Pattern,
        length: usize,
        maxIndividualVariance: f32,
    ) -> f32 {
        let mut maxIndividualVariance = maxIndividualVariance;

        let total: PatternType = counters.sum(); //counters.into_iter().take(length).copied().reduce(|acc,e| {acc + e} ).unwrap_or_default().into(); //Reduce(counters, counters + length, 0);
        let patternLength: PatternType = pattern.sum(); //pattern.into().take(length).copied().reduce(|acc, e| {acc + e}).unwrap_or_default().into(); //Reduce(pattern, pattern + length, 0);
        if (total < patternLength) {
            // If we don't even have one pixel per unit of bar width, assume this is too small
            // to reliably match, so fail:
            return f32::MAX;
            // return std::numeric_limits<float>::max();
        }

        let unitBarWidth: f32 = total as f32 / patternLength as f32;
        maxIndividualVariance *= unitBarWidth;

        let mut totalVariance: f32 = 0.0;
        for x in 0..length {
            // for (size_t x = 0; x < length; ++x) {
            let variance: f32 = (counters[x] as f32 - pattern[x] as f32 * unitBarWidth).abs();
            if (variance > maxIndividualVariance) {
                return f32::MAX;
            }
            totalVariance += variance;
        }
        return totalVariance / total as f32;
    }

    fn PatternMatchVarianceNoLength(
        counters: &Counter,
        pattern: &Pattern,
        maxIndividualVariance: f32,
    ) -> f32 {
        assert!(counters.len() == pattern.len());
        return Self::PatternMatchVariance(
            counters,
            pattern,
            counters.len(),
            maxIndividualVariance,
        );
    }

    /**
    	* Attempts to decode a sequence of black/white lines into single
    	* digit.
    	*
    	* @param counters the counts of runs of observed black/white/black/... values
    	* @param patterns the list of patterns to compare the contents of counters to
    	* @param requireUnambiguousMatch the 'best match' must be better than all other matches
    	* @return The decoded digit index, -1 if no pattern matched
    	*/

    fn DecodeDigit(
        counters: &Counter,
        patterns: Vec<Pattern>,
        maxAvgVariance: f32,
        maxIndividualVariance: f32,
        requireUnambiguousMatch: Option<bool>,
    ) -> i32 {
        let requireUnambiguousMatch = requireUnambiguousMatch.unwrap_or(true);
        let mut bestVariance: f32 = maxAvgVariance; // worst variance we'll accept
        const INVALID_MATCH: i32 = -1;
        let mut bestMatch = INVALID_MATCH;
        for i in 0..patterns.len() {
            // for (int i = 0; i < Size(patterns); i++) {
            let variance: f32 =
                Self::PatternMatchVarianceNoLength(counters, &patterns[i], maxIndividualVariance);
            if (variance < bestVariance) {
                bestVariance = variance;
                bestMatch = i as i32;
            } else if (requireUnambiguousMatch && variance == bestVariance) {
                // if we find a second 'best match' with the same variance, we can not reliably report to have a suitable match
                bestMatch = INVALID_MATCH;
            }
        }
        return bestMatch;
    }

    /**
     * @brief NarrowWideThreshold calculates width thresholds to separate narrow and wide bars and spaces.
     *
     * This is useful for codes like Codabar, Code39 and ITF which distinguish between narrow and wide
     * bars/spaces. Where wide ones are between 2 and 3 times as wide as the narrow ones.
     *
     * @param view containing one character
     * @return threshold value for bars and spaces
     */
    fn NarrowWideThreshold(view: &PatternView) -> BarAndSpace<i32> {
        let mut m: BarAndSpace<i32> = BarAndSpace::new(view[0] as i32, view[1] as i32);
        let mut M: BarAndSpace<i32> = m.clone();
        for i in 0..view.size() {
            // for (int i = 2; i < view.size(); ++i)
            UpdateMinMax(&mut m[i], &mut M[i], view[i] as i32);
        }

        let mut res = BarAndSpace::default();
        for i in 0..2 {
            // for (int i = 0; i < 2; ++i) {
            // check that
            //  a) wide <= 4 * narrow
            //  b) bars and spaces are not more than a factor of 2 (or 3 for the max) apart from each other
            if (M[i] > 4 * (m[i] + 1) || M[i] > 3 * M[i + 1] || m[i] > 2 * (m[i + 1] + 1)) {
                return BarAndSpace::default();
            }
            // the threshold is the average of min and max but at least 1.5 * min
            res[i] = std::cmp::max((m[i] + M[i]) / 2, m[i] * 3 / 2);
        }

        return res;
    }

    /**
     * @brief ToNarrowWidePattern takes a PatternView, calculates a NarrowWideThreshold and returns int where a '0' bit
     * means narrow and a '1' bit means 'wide'.
     */
    fn NarrowWideBitPattern(view: &PatternView) -> i32 {
        let threshold = Self::NarrowWideThreshold(view);
        if (!threshold.isValid()) {
            return -1;
        }

        let mut pattern: i32 = 0;
        for i in 0..view.size() {
            // for (int i = 0; i < view.size(); ++i) {
            if (view[i] as i32 > threshold[i] * 2) {
                return -1;
            }
            AppendBit(&mut pattern, view[i] as i32 > threshold[i]);
        }

        return pattern;
    }

    /**
     * @brief each bar/space is 1-4 modules wide, we have N bars/spaces, they are SUM modules wide in total
     */
    fn OneToFourBitPattern<const LEN: usize, const SUM: usize>(view: &PatternView) -> Option<u32> {
        // TODO: make sure none of the elements in the normalized pattern exceeds 4
        ToInt(&NormalizedPattern::<LEN, SUM>(view).ok()?.map(|x| x as u32))
        //  ToInt(NormalizedPattern::<LEN, SUM>(view).unwrap_or_default()).unwrap_or(-1)
    }

    /**
     * @brief Lookup the pattern in the table and return the character in alphabet at the same index.
     * @returns 0 if pattern is not found. Used to be -1 but that fails on systems where char is unsigned.
     */

    fn LookupBitPattern(pattern: u32, table: &Index, alphabet: &Alphabet) -> char {
        if let Some(i) = table.iter().position(|e| *e == pattern) {
            alphabet[i]
        } else {
            char::from(0)
        }
        // let i :i32 = IndexOf(table, pattern);
        // return if i == -1  {0} else {alphabet[i]};
    }

    fn DecodeNarrowWidePattern(view: &PatternView, table: &Index, alphabet: &Alphabet) -> char {
        return Self::LookupBitPattern(Self::NarrowWideBitPattern(view) as u32, table, alphabet);
    }
}

fn DecodeSingleRow<Range, RR>(reader: &RR, range: &[Range]) -> Result<RXingResult>
where
    Range: Into<PatternType> + Copy + Default + From<Range>,
    RR: RowReader,
{
    let mut row = PatternRow::default();
    GetPatternRow(range, &mut row);
    let mut view = PatternView::new(&row);

    let state = DecodingState::default();

    // std::unique_ptr<RowReader::DecodingState> state;
    reader.decodePattern(0, &mut view, &mut Some(state))
}
