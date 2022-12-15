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
 */
pub struct AI013x0xDecoder<'a> {
    information: &'a BitArray,
    decoder: GeneralAppIdDecoder<'a>,
}
impl AI01weightDecoder for AI013x0xDecoder<'_> {
    fn addWeightCode(&self, _buf: &mut String, _weight: u32) {
        unimplemented!("no java implementation exists")
    }

    fn checkWeight(&self, _weight: u32) -> u32 {
        unimplemented!("no java implementation exists")
    }
}
impl AbstractExpandedDecoder for AI013x0xDecoder<'_> {
    fn parseInformation(&mut self) -> Result<String, crate::Exceptions> {
        if self.information.getSize()
            != Self::HEADER_SIZE + Self::GTIN_SIZE as usize + Self::WEIGHT_SIZE
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

        Ok(buf)
    }

    fn getGeneralDecoder(&self) -> &GeneralAppIdDecoder {
        &self.decoder
    }
}
impl AI01decoder for AI013x0xDecoder<'_> {}

impl<'a> AI013x0xDecoder<'_> {
    const HEADER_SIZE: usize = 4 + 1;
    const WEIGHT_SIZE: usize = 15;

    pub fn new(information: &'a BitArray) -> AI013x0xDecoder<'a> {
        AI013x0xDecoder {
            information,
            decoder: GeneralAppIdDecoder::new(information),
        }
    }
}
