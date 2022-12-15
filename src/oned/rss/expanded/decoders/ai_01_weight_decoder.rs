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

use super::AI01decoder;

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
pub trait AI01weightDecoder: AI01decoder {
    fn encodeCompressedWeight(&self, buf: &mut String, currentPos: usize, weightSize: u32) {
        let originalWeightNumeric = self
            .getGeneralDecoder()
            .extractNumericValueFromBitArray(currentPos, weightSize);
        self.addWeightCode(buf, originalWeightNumeric);

        let weightNumeric = self.checkWeight(originalWeightNumeric);

        let mut currentDivisor = 100000;
        for _i in 0..5 {
            // for (int i = 0; i < 5; ++i) {
            if weightNumeric / currentDivisor == 0 {
                buf.push('0');
            }
            currentDivisor /= 10;
        }
        buf.push_str(&weightNumeric.to_string());
    }

    fn addWeightCode(&self, buf: &mut String, weight: u32);

    fn checkWeight(&self, weight: u32) -> u32;
}
