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

use std::fmt::Display;

use crate::oned::rss::{DataCharacter, FinderPattern};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct ExpandedPair {
    leftChar: Option<DataCharacter>,
    rightChar: Option<DataCharacter>,
    finderPattern: Option<FinderPattern>,
}

impl ExpandedPair {
    pub const fn new(
        leftChar: Option<DataCharacter>,
        rightChar: Option<DataCharacter>,
        finderPattern: Option<FinderPattern>,
    ) -> Self {
        Self {
            leftChar,
            rightChar,
            finderPattern,
        }
    }

    pub fn getLeftChar(&self) -> &Option<DataCharacter> {
        &self.leftChar
    }

    pub fn getRightChar(&self) -> &Option<DataCharacter> {
        &self.rightChar
    }

    pub fn getFinderPattern(&self) -> &Option<FinderPattern> {
        &self.finderPattern
    }

    #[cfg(all(test, feature = "image"))]
    pub(crate) fn getFinderPatternMut(&mut self) -> &mut Option<FinderPattern> {
        &mut self.finderPattern
    }

    pub fn mustBeLast(&self) -> bool {
        self.rightChar.is_none()
    }
}

impl Display for ExpandedPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ {:?} , {:?} : {} ]",
            self.leftChar,
            self.rightChar,
            if let Some(fp) = &self.finderPattern {
                fp.getValue().to_string()
            } else {
                "null".to_owned()
            }
        )
    }
}
