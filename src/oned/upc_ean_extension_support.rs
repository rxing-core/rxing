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

use crate::{common::BitArray, Exceptions, RXingResult};

use super::{UPCEANExtension2Support, UPCEANExtension5Support, UPCEANReader, STAND_IN};

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
    ) -> Result<RXingResult, Exceptions> {
        let extensionStartRange =
            STAND_IN.findGuardPattern(row, rowOffset, false, &Self::EXTENSION_START_PATTERN)?;
        if let Ok(res_1) = self
            .fiveSupport
            .decodeRow(rowNumber, row, &extensionStartRange)
        {
            Ok(res_1)
        } else {
            self.twoSupport
                .decodeRow(rowNumber, row, &Self::EXTENSION_START_PATTERN)
        }
        // let res_2 = twoSupport.decodeRow(rowNumber, row, extensionStartRange);
        // try {
        //   return fiveSupport.decodeRow(rowNumber, row, extensionStartRange);
        // } catch (ReaderException ignored) {
        //   return twoSupport.decodeRow(rowNumber, row, extensionStartRange);
        // }
    }
}
