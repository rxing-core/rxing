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

use crate::{common::BitArray, oned::rss::DataCharacterTrait};

use super::ExpandedPair;

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */

pub fn buildBitArray(pairs: &[ExpandedPair]) -> Option<BitArray> {
    let mut charNumber = (pairs.len() * 2) - 1;
    if let Some(pair) = pairs.last() {
        if pair.getRightChar().is_none() {
            charNumber -= 1;
        }
    }
    // if pairs.last().unwrap().getRightChar().is_none() {
    //     charNumber -= 1;
    // }

    let size = 12 * charNumber;

    let mut binary = BitArray::with_size(size);
    let mut accPos = 0;

    let firstPair = pairs.first()?;
    let rp = firstPair.getRightChar().as_ref()?;
    let firstValue = rp.getValue();
    let mut i = 11;
    while i >= 0 {
        // for (int i = 11; i >= 0; --i) {
        if (firstValue & (1 << i)) != 0 {
            binary.set(accPos);
        }
        accPos += 1;

        i -= 1;
    }

    for i in 1..pairs.len() {
        // for (int i = 1; i < pairs.size(); ++i) {
        let currentPair = pairs.get(i)?;
        let lv = currentPair.getLeftChar().as_ref()?;
        let leftValue = lv.getValue();
        let mut j = 11;
        while j >= 0 {
            // for (int j = 11; j >= 0; --j) {
            if (leftValue & (1 << j)) != 0 {
                binary.set(accPos);
            }
            accPos += 1;

            j -= 1;
        }

        if let Some(rc) = currentPair.getRightChar() {
            let rightValue = rc.getValue(); //currentPair.getRightChar().getValue();
            let mut j = 11;
            while j >= 0 {
                // for (int j = 11; j >= 0; --j) {
                if (rightValue & (1 << j)) != 0 {
                    binary.set(accPos);
                }
                accPos += 1;

                j -= 1;
            }
        }
    }
    Some(binary)
}

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
#[cfg(test)]
mod BitArrayBuilderTest {
    use crate::{
        common::BitArray,
        oned::rss::{expanded::ExpandedPair, DataCharacter},
    };

    #[test]
    fn testBuildBitArray1() {
        let pairValues = vec![vec![19], vec![673, 16]];

        let expected = " .......X ..XX..X. X.X....X .......X ....";

        checkBinary(&pairValues, expected);
    }

    fn checkBinary(pairValues: &[Vec<u32>], expected: &str) {
        let binary = buildBitArray(pairValues);
        assert_eq!(expected, binary.to_string());
    }

    fn buildBitArray(pairValues: &[Vec<u32>]) -> BitArray {
        let mut pairs = Vec::new(); //new ArrayList<>();
        for (i, pair) in pairValues.iter().enumerate() {
            // for (int i = 0; i < pairValues.length; ++i) {

            let leftChar = if i == 0 {
                None
            } else {
                Some(DataCharacter::new(pair[0], 0))
            };

            let rightChar = if i == 0 {
                Some(DataCharacter::new(pair[0], 0))
            } else if pair.len() == 2 {
                Some(DataCharacter::new(pair[1], 0))
            } else {
                None
            };

            let expandedPair = ExpandedPair::new(leftChar, rightChar, None);
            pairs.push(expandedPair);
        }

        super::buildBitArray(&pairs).unwrap()
    }
}
