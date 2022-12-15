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

use super::{AI01decoder, AI01weightDecoder, AbstractExpandedDecoder, GeneralAppIdDecoder};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
pub struct AI013x0x1xDecoder<'a> {
    information: &'a BitArray,
    decoder: GeneralAppIdDecoder<'a>,
    dateCode: String,
    firstAIdigits: String,
}

impl AI01weightDecoder for AI013x0x1xDecoder<'_> {
    fn addWeightCode(&self, buf: &mut String, weight: u32) {
        buf.push('(');
        buf.push_str(&self.firstAIdigits);
        buf.push_str(&(weight / 100000).to_string());
        buf.push(')');
    }

    fn checkWeight(&self, weight: u32) -> u32 {
        weight % 100000
    }
}
impl AI01decoder for AI013x0x1xDecoder<'_> {}
impl AbstractExpandedDecoder for AI013x0x1xDecoder<'_> {
    fn parseInformation(&mut self) -> Result<String, crate::Exceptions> {
        if self.information.getSize()
            != Self::HEADER_SIZE + Self::GTIN_SIZE as usize + Self::WEIGHT_SIZE + Self::DATE_SIZE
        {
            return Err(crate::Exceptions::NotFoundException("".to_owned()));
        }

        let mut buf = String::new();

        self.encodeCompressedGtin(&mut buf, Self::HEADER_SIZE);
        self.encodeCompressedWeight(
            &mut buf,
            Self::HEADER_SIZE + Self::GTIN_SIZE as usize,
            Self::WEIGHT_SIZE as u32,
        );
        self.encodeCompressedDate(
            &mut buf,
            Self::HEADER_SIZE + Self::GTIN_SIZE as usize + Self::WEIGHT_SIZE,
        );

        Ok(buf)
    }

    fn getGeneralDecoder(&self) -> &super::GeneralAppIdDecoder {
        &self.decoder
    }
}

impl<'a> AI013x0x1xDecoder<'_> {
    const HEADER_SIZE: usize = 7 + 1;
    const WEIGHT_SIZE: usize = 20;
    const DATE_SIZE: usize = 16;

    pub fn new(
        information: &'a BitArray,
        firstAIdigits: String,
        dateCode: String,
    ) -> AI013x0x1xDecoder<'a> {
        AI013x0x1xDecoder {
            information,
            decoder: GeneralAppIdDecoder::new(information),
            dateCode,
            firstAIdigits,
        }
    }

    fn encodeCompressedDate(&self, buf: &mut String, currentPos: usize) {
        let mut numericDate = self
            .getGeneralDecoder()
            .extractNumericValueFromBitArray(currentPos, Self::DATE_SIZE as u32);
        if numericDate == 38400 {
            return;
        }

        buf.push('(');
        buf.push_str(&self.dateCode);
        buf.push(')');

        let day = numericDate % 32;
        numericDate /= 32;
        let month = numericDate % 12 + 1;
        numericDate /= 12;
        let year = numericDate;

        if year / 10 == 0 {
            buf.push('0');
        }
        buf.push_str(&year.to_string());
        if month / 10 == 0 {
            buf.push('0');
        }
        buf.push_str(&month.to_string());
        if day / 10 == 0 {
            buf.push('0');
        }
        buf.push_str(&day.to_string());
    }
}
