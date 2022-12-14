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

use super::DecodedObject;

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
pub struct DecodedChar {
    value: char,
    newPosition: usize,
}
impl DecodedObject for DecodedChar {
    fn getNewPosition(&self) -> usize {
        self.newPosition
    }
}
impl DecodedChar {
    pub const FNC1: char = '$'; // It's not in Alphanumeric neither in ISO/IEC 646 charset

    pub fn new(newPosition: usize, value: char) -> Self {
        Self { value, newPosition }
    }

    pub fn getValue(&self) -> char {
        self.value
    }

    pub fn isFNC1(&self) -> bool {
        self.value == Self::FNC1
    }
}
