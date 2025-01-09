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

use crate::oned::rss::expanded::{binary_util, decoders::abstract_expanded_decoder::createDecoder};

/*
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */

#[allow(dead_code)]
pub const NUMERIC10: &str = "..X..XX";
#[allow(dead_code)]
pub const NUMERIC12: &str = "..X.X.X";
#[allow(dead_code)]
pub const NUMERIC1_FNC1: &str = "..XXX.X";
// static final String numericFNC11                  = "XXX.XXX";

#[allow(dead_code)]
pub const NUMERIC2ALPHA: &str = "....";

#[allow(dead_code)]
pub const ALPHA_A: &str = "X.....";
#[allow(dead_code)]
pub const ALPHA_FNC1: &str = ".XXXX";
#[allow(dead_code)]
pub const ALPHA2NUMERIC: &str = "...";
#[allow(dead_code)]
pub const ALPHA2ISOIEC646: &str = "..X..";

#[allow(dead_code)]
pub const I646_B: &str = "X.....X";
#[allow(dead_code)]
pub const I646_C: &str = "X....X.";
#[allow(dead_code)]
pub const I646_FNC1: &str = ".XXXX";
#[allow(dead_code)]
pub const ISOIEC6462ALPHA: &str = "..X..";

#[allow(dead_code)]
pub const COMPRESSED_GTIN900123456798908: &str = ".........X..XXX.X.X.X...XX.XXXXX.XXXX.X.";
#[allow(dead_code)]
pub const COMPRESSED_GTIN900000000000008: &str = "........................................";

#[allow(dead_code)]
pub const COMPRESSED15BIT_WEIGHT1750: &str = "....XX.XX.X.XX.";
#[allow(dead_code)]
pub const COMPRESSED15BIT_WEIGHT11750: &str = ".X.XX.XXXX..XX.";
#[allow(dead_code)]
pub const COMPRESSED15BIT_WEIGHT0: &str = "...............";

#[allow(dead_code)]
pub const COMPRESSED20BIT_WEIGHT1750: &str = ".........XX.XX.X.XX.";

#[allow(dead_code)]
pub const COMPRESSED_DATE_MARCH12TH2010: &str = "....XXXX.X..XX..";
#[allow(dead_code)]
pub const COMPRESSED_DATE_END: &str = "X..X.XX.........";

#[allow(dead_code)]
pub fn assertCorrectBinaryString(binaryString: &str, expectedNumber: &str) {
    let binary = binary_util::buildBitArrayFromStringWithoutSpaces(binaryString).expect("built");

    let mut decoder = createDecoder(&binary).expect("get decoder");
    let result = decoder.parseInformation().expect("information exists");
    assert_eq!(expectedNumber, result);
}
