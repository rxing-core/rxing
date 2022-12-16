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

use std::fmt::Display;

use super::ExpandedPair;

/**
 * One row of an RSS Expanded Stacked symbol, consisting of 1+ expanded pairs.
 */
#[derive(Hash, Clone)]
pub struct ExpandedRow {
    pairs: Vec<ExpandedPair>,
    rowNumber: u32,
}
impl ExpandedRow {
    pub fn new(pairs: Vec<ExpandedPair>, rowNumber: u32) -> Self {
        Self { pairs, rowNumber }
    }

    pub fn getPairs(&self) -> &[ExpandedPair] {
        &self.pairs
    }

    #[cfg(test)]
    pub(crate) fn getPairsMut(&mut self) -> &mut [ExpandedPair] {
        &mut self.pairs
    }

    pub fn getRowNumber(&self) -> u32 {
        self.rowNumber
    }

    pub fn isEquivalent(&self, otherPairs: &[ExpandedPair]) -> bool {
        self.pairs == otherPairs
    }
}

impl PartialEq for ExpandedRow {
    /**
     * Two rows are equal if they contain the same pairs in the same order.
     */
    fn eq(&self, other: &Self) -> bool {
        self.pairs == other.pairs
    }
}

impl Display for ExpandedRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        for p in &self.pairs {
            write!(f, "{}", p)?;
        }
        write!(f, " }}") //{:?} }} " , self.pairs )
    }
}
