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

use crate::oned::rss::expanded::{binary_util, decoders::{AbstractExpandedDecoder, abstract_expanded_decoder::createDecoder}};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */

pub const numeric10: &str = "..X..XX";
pub const numeric12: &str = "..X.X.X";
pub const numeric1FNC1: &str = "..XXX.X";
 // static final String numericFNC11                  = "XXX.XXX";

pub const numeric2alpha: &str = "....";

pub const alphaA: &str = "X.....";
pub const alphaFNC1: &str = ".XXXX";
pub const alpha2numeric: &str = "...";
pub const alpha2isoiec646: &str = "..X..";

pub const i646B: &str = "X.....X";
pub const i646C: &str = "X....X.";
pub const i646FNC1: &str = ".XXXX";
pub const isoiec6462alpha: &str = "..X..";

pub const compressedGtin900123456798908: &str = ".........X..XXX.X.X.X...XX.XXXXX.XXXX.X.";
pub const compressedGtin900000000000008: &str = "........................................";

pub const compressed15bitWeight1750: &str = "....XX.XX.X.XX.";
pub const compressed15bitWeight11750: &str = ".X.XX.XXXX..XX.";
pub const compressed15bitWeight0: &str = "...............";

pub const compressed20bitWeight1750: &str = ".........XX.XX.X.XX.";

pub const compressedDateMarch12th2010: &str = "....XXXX.X..XX..";
pub const compressedDateEnd: &str = "X..X.XX.........";

pub fn assertCorrectBinaryString(binaryString: &str, expectedNumber: &str) {
    let binary = binary_util::buildBitArrayFromStringWithoutSpaces(binaryString).expect("built");
    
    let mut decoder = createDecoder(&binary).expect("get decoder");
    let result = decoder.parseInformation().expect("information exists");
    assert_eq!(expectedNumber, result);
}
