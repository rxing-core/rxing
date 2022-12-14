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

/*
 * These authors would like to acknowledge the Spanish Ministry of Industry,
 * Tourism and Trade, for the support in the project TSI020301-2008-2
 * "PIRAmIDE: Personalizable Interactions with Resources on AmI-enabled
 * Mobile Dynamic Environments", led by Treelogic
 * ( http://www.treelogic.com/ ):
 *
 *   http://www.piramidepse.com/
 */

use super::AbstractExpandedDecoder;

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
pub trait AI01decoder: AbstractExpandedDecoder {
    const GTIN_SIZE: u32 = 40;

    fn encodeCompressedGtin(&self, buf: &mut String, currentPos: usize) {
        buf.push_str("(01)");
        let initialPosition = buf.chars().count();
        buf.push('9');

        self.encodeCompressedGtinWithoutAI(buf, currentPos, initialPosition);
    }

    fn encodeCompressedGtinWithoutAI(
        &self,
        buf: &mut String,
        currentPos: usize,
        initialBufferPosition: usize,
    ) {
        for i in 0..4 {
            // for (int i = 0; i < 4; ++i) {
            let currentBlock = self
                .getGeneralDecoder()
                .extractNumericValueFromBitArray(currentPos + 10 * i, 10);
            if currentBlock / 100 == 0 {
                buf.push('0');
            }
            if currentBlock / 10 == 0 {
                buf.push('0');
            }
            buf.push_str(&currentBlock.to_string());
        }

        appendCheckDigit(buf, initialBufferPosition);
    }
}

pub(super) fn appendCheckDigit(buf: &mut String, currentPos: usize) {
    let mut checkDigit = 0;
    for i in 0..13 {
        // for (int i = 0; i < 13; i++) {
        let digit = buf.chars().nth(i + currentPos).unwrap() as u32 - '0' as u32;
        checkDigit += if (i & 0x01) == 0 { 3 * digit } else { digit };
    }

    checkDigit = 10 - (checkDigit % 10);
    if checkDigit == 10 {
        checkDigit = 0;
    }

    buf.push_str(&checkDigit.to_string());
}
