use std::fmt::Display;

use super::{DataCharacter, DataCharacterTrait, FinderPattern};

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
#[derive(Hash, Eq, PartialEq)]
pub struct Pair {
    finderPattern: FinderPattern,
    count: u32,
    internal_data_character: DataCharacter,
}

impl DataCharacterTrait for Pair {
    fn getValue(&self) -> u32 {
        self.internal_data_character.getValue()
    }

    fn getChecksumPortion(&self) -> u32 {
        self.internal_data_character.getChecksumPortion()
    }
}

impl Pair {
    pub fn new(value: u32, checksumPortion: u32, finderPattern: FinderPattern) -> Self {
        Self {
            finderPattern,
            count: 0,
            internal_data_character: DataCharacter::new(value, checksumPortion),
        }
    }

    pub fn getFinderPattern(&self) -> &FinderPattern {
        &self.finderPattern
    }

    pub fn getCount(&self) -> u32 {
        self.count
    }

    pub fn incrementCount(&mut self) {
        self.count += 1;
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.internal_data_character.fmt(f)
    }
}
