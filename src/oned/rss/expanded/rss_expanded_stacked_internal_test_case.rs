/*
 * Copyright (C) 2012 ZXing authors
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

use crate::{oned::rss::expanded::ExpandedPair, Exceptions, Reader};

use super::{test_case_util, RSSExpandedReader};

/**
 * Tests {@link RSSExpandedReader} handling of stacked RSS barcodes.
 */

#[test]
fn testDecodingRowByRow() {
    let mut rssExpandedReader = RSSExpandedReader::new();

    let binaryMap = test_case_util::getBinaryBitmap("1000.png");

    let firstRowNumber = binaryMap.getHeight() / 3;
    let firstRow = binaryMap.getBlackRow(firstRowNumber).expect("get row");

    // let tester = ;

    assert!(|| -> Result<Vec<ExpandedPair>, Exceptions> {
        rssExpandedReader.decodeRow2pairs(firstRowNumber as u32, &firstRow)
        // fail(NotFoundException.class.getName() + " expected");
    }()
    .is_err());

    assert_eq!(1, rssExpandedReader.getRows().len());
    let firstExpandedRow = &mut rssExpandedReader.rows[0]; //&mut rssExpandedReader.getRowsMut()[0];//.expect("not None");

    assert_eq!(firstRowNumber as u32, firstExpandedRow.getRowNumber());

    assert_eq!(2, firstExpandedRow.getPairs().len());

    firstExpandedRow.getPairsMut()[1]
        .getFinderPatternMut()
        .as_mut()
        .unwrap()
        .getStartEndMut()[1] = 0;
    rssExpandedReader
        .pairs
        .last_mut()
        .as_mut()
        .unwrap()
        .getFinderPatternMut()
        .as_mut()
        .unwrap()
        .getStartEndMut()[1] = 0;

    let secondRowNumber = 2 * binaryMap.getHeight() / 3;
    let mut secondRow = binaryMap.getBlackRow(secondRowNumber).expect("get row").into_owned();
    secondRow.reverse();

    let totalPairs = rssExpandedReader
        .decodeRow2pairs(secondRowNumber as u32, &secondRow)
        .expect("decode pairs");

    let result = RSSExpandedReader::constructRXingResult(&totalPairs).expect("construct");
    assert_eq!("(01)98898765432106(3202)012345(15)991231", result.getText());
}

#[test]
fn testCompleteDecode() {
    let mut rssExpandedReader = RSSExpandedReader::new();

    let mut binaryMap = test_case_util::getBinaryBitmap("1000.png");

    let result = rssExpandedReader.decode(&mut binaryMap).expect("decode");
    assert_eq!("(01)98898765432106(3202)012345(15)991231", result.getText());
}
