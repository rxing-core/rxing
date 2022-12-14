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

use std::{fmt::Display, hash::Hash};

pub(crate) trait DataCharacterTrait: Display + Eq + PartialEq + Hash {
    fn getValue(&self) -> u32;

    fn getChecksumPortion(&self) -> u32;
}

/**
 * Encapsulates a since character value in an RSS barcode, including its checksum information.
 */
#[derive(Hash, PartialEq, Eq, Debug)]
pub struct DataCharacter {
    value: u32,
    checksumPortion: u32,
}
impl DataCharacterTrait for DataCharacter {
    fn getValue(&self) -> u32 {
        self.value
    }

    fn getChecksumPortion(&self) -> u32 {
        self.checksumPortion
    }
}
impl DataCharacter {
    pub fn new(value: u32, checksumPortion: u32) -> Self {
        Self {
            value,
            checksumPortion,
        }
    }
}
impl Display for DataCharacter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.value, self.checksumPortion)
    }
}
