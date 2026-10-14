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

use crate::{
    Exceptions, RXingResult,
    common::{BitArray, Result},
};

use super::{STAND_IN, UPCEANExtension2Support, UPCEANExtension5Support, UPCEANReader};

#[derive(Default)]
pub struct UPCEANExtensionSupport {
    twoSupport: UPCEANExtension2Support,
    fiveSupport: UPCEANExtension5Support,
}

impl UPCEANExtensionSupport {
    const EXTENSION_START_PATTERN: [u32; 3] = [1, 1, 2];

    pub fn decodeRow(
        &self,
        rowNumber: u32,
        row: &BitArray,
        rowOffset: usize,
    ) -> Result<RXingResult> {
        let extensionStartRange =
            STAND_IN.findGuardPattern(row, rowOffset, false, &Self::EXTENSION_START_PATTERN)?;
        // An add-on starts 7 to 12 modules after the main symbol. The start
        // pattern is 4 modules wide; allow a gap of up to 4 pattern widths so the
        // search does not pick up a neighbouring symbol further along the row.
        let [start, end] = extensionStartRange;
        if start.saturating_sub(rowOffset) > 4 * (end - start) {
            return Err(Exceptions::NOT_FOUND);
        }
        if let Ok(res_1) = self
            .fiveSupport
            .decodeRow(rowNumber, row, &extensionStartRange)
        {
            Ok(res_1)
        } else {
            // The two-digit helper takes the located range as [start, end, _].
            self.twoSupport
                .decodeRow(rowNumber, row, &[start as u32, end as u32, 0])
        }
    }
}
