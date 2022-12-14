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
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
pub struct AI01AndOtherAIs<'a>(&'a BitArray, GeneralAppIdDecoder<'a>);
impl<'a> AI01decoder for AI01AndOtherAIs<'_> {}
impl AbstractExpandedDecoder for AI01AndOtherAIs<'_> {
    fn parseInformation(&mut self) -> Result<String, crate::Exceptions> {
        let mut buff = String::new(); //new StringBuilder();

        buff.push_str("(01)");
        let initialGtinPosition = buff.chars().count();
        let firstGtinDigit = self
            .getGeneralDecoder()
            .extractNumericValueFromBitArray(Self::HEADER_SIZE, 4);
        buff.push_str(&firstGtinDigit.to_string());

        self.encodeCompressedGtinWithoutAI(&mut buff, Self::HEADER_SIZE + 4, initialGtinPosition);

        self.1.decodeAllCodes(buff, Self::HEADER_SIZE + 44)
    }

    fn getGeneralDecoder(&self) -> &super::GeneralAppIdDecoder {
        &self.1
    }
}
impl<'a> AI01AndOtherAIs<'_> {
    fn new(information: &'a BitArray) -> AI01AndOtherAIs<'a> {
        AI01AndOtherAIs(information, GeneralAppIdDecoder::new(information))
    }

    const HEADER_SIZE: usize = 1 + 1 + 2; //first bit encodes the linkage flag,
                                          //the second one is the encodation method, and the other two are for the variable length
                                          // AI01AndOtherAIs(BitArray information) {
                                          //   super(information);
                                          // }
}
