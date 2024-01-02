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
 *
 * This software consists of contributions made by many individuals,
 * listed below:
 *
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 *
 * These authors would like to acknowledge the Spanish Ministry of Industry,
 * Tourism and Trade, for the support in the project TSI020301-2008-2
 * "PIRAmIDE: Personalizable Interactions with Resources on AmI-enabled
 * Mobile Dynamic Environments", leaded by Treelogic
 * ( http://www.treelogic.com/ ):
 *
 *   http://www.piramidepse.com/
 *
 */

use std::collections::HashMap;

#[cfg(feature = "client_support")]
use crate::{
    client::result::{ExpandedProductParsedRXingResult, ParsedClientResult},
    common::GlobalHistogramBinarizer,
    oned::{rss::expanded::RSSExpandedReader, OneDReader},
    BarcodeFormat, BinaryBitmap, BufferedImageLuminanceSource,
};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
#[cfg(feature = "client_support")]
#[test]
fn testDecodeRow2result2() {
    // (01)90012345678908(3103)001750
    let expected = ExpandedProductParsedRXingResult::new(
        "(01)90012345678908(3103)001750".to_owned(),
        "90012345678908".to_owned(),
        String::default(),
        String::default(),
        String::default(),
        String::default(),
        String::default(),
        String::default(),
        "001750".to_owned(),
        ExpandedProductParsedRXingResult::KILOGRAM.to_owned(),
        "3".to_owned(),
        String::default(),
        String::default(),
        String::default(),
        HashMap::new(),
    );

    assertCorrectImage2result("2.png", expected);
}

#[cfg(feature = "client_support")]
fn assertCorrectImage2result(fileName: &str, expected: ExpandedProductParsedRXingResult) {
    let path = format!("test_resources/blackbox/rssexpanded-1/{fileName}");

    let image = image::open(path).expect("image must exist");
    let binaryMap = BinaryBitmap::new(GlobalHistogramBinarizer::new(
        BufferedImageLuminanceSource::new(image),
    ));
    let rowNumber = binaryMap.get_height() / 2;
    let row = binaryMap.get_black_row(rowNumber).expect("get row");

    let mut rssExpandedReader = RSSExpandedReader::new();
    let theRXingResult = rssExpandedReader
        .decode_row(rowNumber as u32, &row, &HashMap::new())
        .expect("must decode");

    assert_eq!(
        &BarcodeFormat::RSS_EXPANDED,
        theRXingResult.getBarcodeFormat()
    );

    let ParsedClientResult::ExpandedProductResult(result) =
        crate::client::result::parseRXingResult(&theRXingResult)
    else {
        panic!("incorrect result type found");
    };

    assert_eq!(expected, result);
}
