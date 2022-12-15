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

use crate::oned::rss::expanded::{binary_util, decoders::AbstractExpandedDecoder};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 */

const numeric10: &str = "..X..XX";
const numeric12: &str = "..X.X.X";
const numeric1FNC1: &str = "..XXX.X";
// static final String numericFNC11                  = "XXX.XXX";

const numeric2alpha: &str = "....";

const alphaA: &str = "X.....";
const alphaFNC1: &str = ".XXXX";
const alpha2numeric: &str = "...";
const alpha2isoiec646: &str = "..X..";

const i646B: &str = "X.....X";
const i646C: &str = "X....X.";
const i646FNC1: &str = ".XXXX";
const isoiec6462alpha: &str = "..X..";

const compressedGtin900123456798908: &str = ".........X..XXX.X.X.X...XX.XXXXX.XXXX.X.";
const compressedGtin900000000000008: &str = "........................................";

const compressed15bitWeight1750: &str = "....XX.XX.X.XX.";
const compressed15bitWeight11750: &str = ".X.XX.XXXX..XX.";
const compressed15bitWeight0: &str = "...............";

const compressed20bitWeight1750: &str = ".........XX.XX.X.XX.";

const compressedDateMarch12th2010: &str = "....XXXX.X..XX..";
const compressedDateEnd: &str = "X..X.XX.........";

pub fn assertCorrectBinaryString(binaryString: &str, expectedNumber: &str) {
    let binary = binary_util::buildBitArrayFromStringWithoutSpaces(binaryString).expect("built");
    panic!("finish implementation for test");
    // let decoder = AbstractExpandedDecoder.createDecoder(binary);
    // let result = decoder.parseInformation();
    // assert_eq!(expectedNumber, result);
}
