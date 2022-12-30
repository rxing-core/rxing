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

use super::{AbstractExpandedDecoder, GeneralAppIdDecoder};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
pub struct AnyAIDecoder<'a> {
    // information: &'a BitArray,
    general_decoder: GeneralAppIdDecoder<'a>,
}
impl AbstractExpandedDecoder for AnyAIDecoder<'_> {
    fn parseInformation(&mut self) -> Result<String, crate::Exceptions> {
        let buf = String::new();
        self.general_decoder.decodeAllCodes(buf, Self::HEADER_SIZE)
    }

    fn getGeneralDecoder(&self) -> &super::GeneralAppIdDecoder {
        &self.general_decoder
    }
}

impl<'a> AnyAIDecoder<'_> {
    const HEADER_SIZE: usize = 2 + 1 + 2;

    pub fn new(information: &'a BitArray) -> AnyAIDecoder<'a> {
        AnyAIDecoder {
            // information,
            general_decoder: GeneralAppIdDecoder::new(information),
        }
    }
}

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
#[cfg(test)]
mod AnyAIDecoderTest {
    use crate::oned::rss::expanded::decoders::abstract_decoder_test_utils::*;

    const HEADER: &str = ".....";

    #[test]
    fn testAnyAIDecoder1() {
        let data = format!(
            "{}{}{}{}{}{}{}",
            HEADER, NUMERIC10, NUMERIC12, NUMERIC2ALPHA, ALPHA_A, ALPHA2NUMERIC, NUMERIC12
        );
        let expected = "(10)12A12";

        assertCorrectBinaryString(&data, expected);
    }

    #[test]
    fn testAnyAIDecoder2() {
        let data = format!(
            "{}{}{}{}{}{}{}",
            HEADER, NUMERIC10, NUMERIC12, NUMERIC2ALPHA, ALPHA_A, ALPHA2ISOIEC646, I646_B
        );
        let expected = "(10)12AB";

        assertCorrectBinaryString(&data, expected);
    }

    #[test]
    fn testAnyAIDecoder3() {
        let data = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            HEADER,
            NUMERIC10,
            NUMERIC2ALPHA,
            ALPHA2ISOIEC646,
            I646_B,
            I646_C,
            ISOIEC6462ALPHA,
            ALPHA_A,
            ALPHA2NUMERIC,
            NUMERIC10
        );
        let expected = "(10)BCA10";

        assertCorrectBinaryString(&data, expected);
    }

    #[test]
    fn testAnyAIDecodernumericFNC1secondDigit() {
        let data = format!("{}{}{}", HEADER, NUMERIC10, NUMERIC1_FNC1);
        let expected = "(10)1";

        assertCorrectBinaryString(&data, expected);
    }

    #[test]
    fn testAnyAIDecoderalphaFNC1() {
        let data = format!(
            "{}{}{}{}{}",
            HEADER, NUMERIC10, NUMERIC2ALPHA, ALPHA_A, ALPHA_FNC1
        );
        let expected = "(10)A";

        assertCorrectBinaryString(&data, expected);
    }

    #[test]
    fn testAnyAIDecoder646FNC1() {
        let data = format!(
            "{}{}{}{}{}{}{}",
            HEADER, NUMERIC10, NUMERIC2ALPHA, ALPHA_A, ISOIEC6462ALPHA, I646_B, I646_FNC1
        );
        let expected = "(10)AB";

        assertCorrectBinaryString(&data, expected);
    }
}
