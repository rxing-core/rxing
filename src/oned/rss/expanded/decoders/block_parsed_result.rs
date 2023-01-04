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

use super::DecodedInformation;

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
pub struct BlockParsedRXingResult {
    decodedInformation: Option<DecodedInformation>,
    finished: bool,
}

impl Default for BlockParsedRXingResult {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockParsedRXingResult {
    pub fn new() -> Self {
        Self::with_information(None, false)
    }

    pub fn with_information(information: Option<DecodedInformation>, finished: bool) -> Self {
        Self {
            decodedInformation: information,
            finished,
        }
    }

    pub fn getDecodedInformation(&self) -> &Option<DecodedInformation> {
        &self.decodedInformation
    }

    pub fn isFinished(&self) -> bool {
        self.finished
    }
}
