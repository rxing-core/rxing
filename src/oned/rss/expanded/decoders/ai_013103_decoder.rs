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

use super::{AI013x0xDecoder, AI01decoder, AI01weightDecoder, AbstractExpandedDecoder};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
pub struct AI013103decoder<'a>(AI013x0xDecoder<'a>);

impl AI01weightDecoder for AI013103decoder<'_> {
    fn addWeightCode(&self, buf: &mut String, weight: u32) {
        self.0.addWeightCode(buf, weight)
    }

    fn checkWeight(&self, weight: u32) -> u32 {
        self.0.checkWeight(weight)
    }
}
impl AbstractExpandedDecoder for AI013103decoder<'_> {
    fn parseInformation(&mut self) -> Result<String, crate::Exceptions> {
        self.0.parseInformation()
    }

    fn getGeneralDecoder(&self) -> &super::GeneralAppIdDecoder {
        self.0.getGeneralDecoder()
    }
}
impl AI01decoder for AI013103decoder<'_> {}

impl<'a> AI013103decoder<'_> {
    pub fn new(information: &'a BitArray) -> AI013103decoder<'a> {
        AI013103decoder(AI013x0xDecoder::new(
            information,
            addWeightCode,
            checkWeight,
        ))
    }
}

fn addWeightCode(buf: &mut String, _weight: u32) {
    buf.push_str("(3103)");
}

fn checkWeight(weight: u32) -> u32 {
    weight
}

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
#[cfg(test)]
mod AI013103DecoderTest {
    use crate::oned::rss::expanded::decoders::abstract_decoder_test_utils::*;

    const HEADER: &str = "..X..";

    #[test]
    fn test0131031() {
        let data = format!("{HEADER}{COMPRESSED_GTIN900123456798908}{COMPRESSED15BIT_WEIGHT1750}");
        let expected = "(01)90012345678908(3103)001750";
        assertCorrectBinaryString(&data, expected);
    }

    #[test]
    fn test0131032() {
        let data = format!("{HEADER}{COMPRESSED_GTIN900000000000008}{COMPRESSED15BIT_WEIGHT0}");
        let expected = "(01)90000000000003(3103)000000";
        assertCorrectBinaryString(&data, expected);
    }

    #[test]
    #[should_panic]
    fn test013103invalid() {
        let data =
            format!("{HEADER}{COMPRESSED_GTIN900123456798908}{COMPRESSED15BIT_WEIGHT1750}..");
        assertCorrectBinaryString(&data, "");
    }
}
