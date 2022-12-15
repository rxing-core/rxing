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

use crate::common::BitArray;

use super::{AI01decoder, AbstractExpandedDecoder, GeneralAppIdDecoder};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
pub struct AI01393xDecoder<'a> {
    information: &'a BitArray,
    general_decoder: GeneralAppIdDecoder<'a>,
}
impl AI01decoder for AI01393xDecoder<'_> {}
impl AbstractExpandedDecoder for AI01393xDecoder<'_> {
    fn parseInformation(&mut self) -> Result<String, crate::Exceptions> {
        if self.information.getSize() < Self::HEADER_SIZE + Self::GTIN_SIZE as usize {
            return Err(crate::Exceptions::NotFoundException("".to_owned()));
        }

        let mut buf = String::new();

        self.encodeCompressedGtin(&mut buf, Self::HEADER_SIZE);

        let lastAIdigit = self.getGeneralDecoder().extractNumericValueFromBitArray(
            Self::HEADER_SIZE + Self::GTIN_SIZE as usize,
            Self::LAST_DIGIT_SIZE as u32,
        );

        buf.push_str("(393");
        buf.push_str(&lastAIdigit.to_string());
        buf.push(')');

        let firstThreeDigits = self.getGeneralDecoder().extractNumericValueFromBitArray(
            Self::HEADER_SIZE + Self::GTIN_SIZE as usize + Self::LAST_DIGIT_SIZE,
            Self::FIRST_THREE_DIGITS_SIZE as u32,
        );
        if firstThreeDigits / 100 == 0 {
            buf.push('0');
        }
        if firstThreeDigits / 10 == 0 {
            buf.push('0');
        }
        buf.push_str(&firstThreeDigits.to_string());

        let generalInformation = self.general_decoder.decodeGeneralPurposeField(
            Self::HEADER_SIZE
                + Self::GTIN_SIZE as usize
                + Self::LAST_DIGIT_SIZE
                + Self::FIRST_THREE_DIGITS_SIZE,
            "",
        )?;
        buf.push_str(&generalInformation.getNewString());

        Ok(buf)
    }

    fn getGeneralDecoder(&self) -> &GeneralAppIdDecoder {
        &self.general_decoder
    }
}
impl<'a> AI01393xDecoder<'_> {
    const HEADER_SIZE: usize = 5 + 1 + 2;
    const LAST_DIGIT_SIZE: usize = 2;
    const FIRST_THREE_DIGITS_SIZE: usize = 10;

    pub fn new(information: &'a BitArray) -> AI01393xDecoder<'a> {
        AI01393xDecoder {
            information,
            general_decoder: GeneralAppIdDecoder::new(information),
        }
    }
}
