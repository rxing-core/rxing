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

use crate::common::{BitArray, Result};

use super::{AI013x0xDecoder, AI01decoder, AI01weightDecoder, AbstractExpandedDecoder};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
pub struct AI01320xDecoder<'a>(AI013x0xDecoder<'a>);

impl AI01weightDecoder for AI01320xDecoder<'_> {
    fn addWeightCode(&self, buf: &mut String, weight: u32) {
        self.0.addWeightCode(buf, weight)
    }

    fn checkWeight(&self, weight: u32) -> u32 {
        self.0.checkWeight(weight)
    }
}
impl AbstractExpandedDecoder for AI01320xDecoder<'_> {
    fn parseInformation(&mut self) -> Result<String> {
        self.0.parseInformation()
    }

    fn getGeneralDecoder(&'_ self) -> &'_ super::GeneralAppIdDecoder<'_> {
        self.0.getGeneralDecoder()
    }
}
impl AI01decoder for AI01320xDecoder<'_> {}
impl<'a> AI01320xDecoder<'_> {
    pub fn new(information: &'a BitArray) -> AI01320xDecoder<'a> {
        AI01320xDecoder(AI013x0xDecoder::new(
            information,
            addWeightCode,
            checkWeight,
        ))
    }
}

fn addWeightCode(buf: &mut String, weight: u32) {
    if weight < 10000 {
        buf.push_str("(3202)");
    } else {
        buf.push_str("(3203)");
    }
}

fn checkWeight(weight: u32) -> u32 {
    if weight < 10000 {
        weight
    } else {
        weight - 10000
    }
}
