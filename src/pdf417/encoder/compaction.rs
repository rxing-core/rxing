/*
 * Copyright 2011 ZXing authors
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

use crate::Exceptions;

/**
 * Represents possible PDF417 barcode compaction types.
 */
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Compaction {
    AUTO = 0,
    TEXT = 1,
    BYTE = 2,
    NUMERIC = 3,
}

impl TryFrom<&String> for Compaction {
    type Error = Exceptions;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if let Ok(num_val) = value.parse::<u8>() {
            match num_val {
                0 => return Ok(Compaction::AUTO),
                1 => return Ok(Compaction::TEXT),
                2 => return Ok(Compaction::BYTE),
                3 => return Ok(Compaction::NUMERIC),
                _ => {}
            }
        }
        Err(Exceptions::format_with(format!(
            "Compaction must be 0-3 (inclusivie). Found: {value}"
        )))
    }
}
