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

use crate::{
    common::{BitArray, Result},
    Exceptions,
};

use super::{
    AI013103decoder, AI01320xDecoder, AI01392xDecoder, AI01393xDecoder, AI013x0x1xDecoder,
    AI01AndOtherAIs, AnyAIDecoder, GeneralAppIdDecoder,
};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */

pub trait AbstractExpandedDecoder {
    // private final BitArray information;
    // private final GeneralAppIdDecoder generalDecoder;

    // AbstractExpandedDecoder(BitArray information) {
    //   this.information = information;
    //   this.generalDecoder = new GeneralAppIdDecoder(information);
    // }

    // protected final BitArray getInformation() {
    //   return information;
    // }

    // protected final GeneralAppIdDecoder getGeneralDecoder() {
    //   return generalDecoder;
    // }

    fn parseInformation(&mut self) -> Result<String>;
    fn getGeneralDecoder(&self) -> &GeneralAppIdDecoder;
    // fn new(information:&BitArray) -> Self where Self:Sized;
}

pub fn createDecoder<'a>(
    information: &'a BitArray,
) -> Result<Box<dyn AbstractExpandedDecoder + 'a>> {
    if information.get(1) {
        return Ok(Box::new(AI01AndOtherAIs::new(information)));
    }
    if !information.get(2) {
        return Ok(Box::new(AnyAIDecoder::new(information)));
    }

    //   let gen_decode = GeneralAppIdDecoder::new(information);

    let fourBitEncodationMethod =
        GeneralAppIdDecoder::extractNumericValueFromBitArrayWithInformation(information, 1, 4);

    match fourBitEncodationMethod {
        4 => return Ok(Box::new(AI013103decoder::new(information))),
        5 => return Ok(Box::new(AI01320xDecoder::new(information))),
        _ => {}
    }

    let fiveBitEncodationMethod =
        GeneralAppIdDecoder::extractNumericValueFromBitArrayWithInformation(information, 1, 5);
    match fiveBitEncodationMethod {
        12 => return Ok(Box::new(AI01392xDecoder::new(information))),
        13 => return Ok(Box::new(AI01393xDecoder::new(information))),
        _ => {}
    }

    let sevenBitEncodationMethod =
        GeneralAppIdDecoder::extractNumericValueFromBitArrayWithInformation(information, 1, 7);
    match sevenBitEncodationMethod {
        56 => return Ok(Box::new(AI013x0x1xDecoder::new(information, "310", "11"))),
        57 => return Ok(Box::new(AI013x0x1xDecoder::new(information, "320", "11"))),
        58 => return Ok(Box::new(AI013x0x1xDecoder::new(information, "310", "13"))),
        59 => return Ok(Box::new(AI013x0x1xDecoder::new(information, "320", "13"))),
        60 => return Ok(Box::new(AI013x0x1xDecoder::new(information, "310", "15"))),
        61 => return Ok(Box::new(AI013x0x1xDecoder::new(information, "320", "15"))),
        62 => return Ok(Box::new(AI013x0x1xDecoder::new(information, "310", "17"))),
        63 => return Ok(Box::new(AI013x0x1xDecoder::new(information, "320", "17"))),
        _ => {}
    }

    Err(Exceptions::illegal_state_with(format!(
        "unknown decoder: {information}"
    )))
}
