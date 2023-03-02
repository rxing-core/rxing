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

use std::rc::Rc;

use crate::{
    common::GlobalHistogramBinarizer,
    oned::rss::{DataCharacterTrait, FinderPattern},
    BinaryBitmap, BufferedImageLuminanceSource,
};

use super::RSSExpandedReader;

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */

#[test]
fn testFindFinderPatterns() {
    let image = readImage("2.png");
    let binaryMap = BinaryBitmap::new(GlobalHistogramBinarizer::new(
        BufferedImageLuminanceSource::new(image),
    ));
    let rowNumber = binaryMap.get_height() as u32 / 2;
    let row = binaryMap.get_black_row(rowNumber as usize).expect("ok");
    let mut previousPairs = Vec::new(); //new ArrayList<>();

    let mut rssExpandedReader = RSSExpandedReader::new();
    let pair1 = rssExpandedReader
        .retrieveNextPair(&row, &previousPairs, rowNumber)
        .expect("finder");
    previousPairs.push(pair1.clone());
    let mut finderPattern = pair1.getFinderPattern().as_ref().unwrap();
    // assertNotNull(finderPattern);
    assert_eq!(0, finderPattern.getValue());

    let pair2 = rssExpandedReader
        .retrieveNextPair(&row, &previousPairs, rowNumber)
        .expect("finder");
    previousPairs.push(pair2.clone());
    finderPattern = pair2.getFinderPattern().as_ref().unwrap();
    // assertNotNull(finderPattern);
    assert_eq!(1, finderPattern.getValue());

    let pair3 = rssExpandedReader
        .retrieveNextPair(&row, &previousPairs, rowNumber)
        .expect("finder");
    previousPairs.push(pair3.clone());
    finderPattern = pair3.getFinderPattern().as_ref().unwrap();
    // assertNotNull(finderPattern);
    assert_eq!(1, finderPattern.getValue());

    // try {
    assert!(rssExpandedReader
        .retrieveNextPair(&row, &previousPairs, rowNumber)
        .is_err());
    //   the previous was the last pair
    // fail(NotFoundException.class.getName() + " expected");
    // } catch (NotFoundException nfe) {
    // ok
    // }
}

#[test]
fn testRetrieveNextPairPatterns() {
    let image = readImage("3.png");
    let binaryMap = BinaryBitmap::new(GlobalHistogramBinarizer::new(
        BufferedImageLuminanceSource::new(image),
    ));
    let rowNumber = binaryMap.get_height() as u32 / 2;
    let row = binaryMap.get_black_row(rowNumber as usize).expect("create");
    let mut previousPairs = Vec::new(); //new ArrayList<>();

    let mut rssExpandedReader = RSSExpandedReader::new();
    let pair1 = rssExpandedReader
        .retrieveNextPair(&row, &previousPairs, rowNumber)
        .expect("finder");
    previousPairs.push(pair1.clone());
    let mut finderPattern = pair1.getFinderPattern().as_ref().unwrap();
    // assertNotNull(finderPattern);
    assert_eq!(0, finderPattern.getValue());

    let pair2 = rssExpandedReader
        .retrieveNextPair(&row, &previousPairs, rowNumber)
        .expect("finder");
    previousPairs.push(pair2.clone());
    finderPattern = pair2.getFinderPattern().as_ref().unwrap();
    // assertNotNull(finderPattern);
    assert_eq!(0, finderPattern.getValue());
}

#[test]
fn testDecodeCheckCharacter() {
    let image = readImage("3.png");
    let binaryMap = BinaryBitmap::new(GlobalHistogramBinarizer::new(
        BufferedImageLuminanceSource::new(image.clone()),
    ));
    let row = binaryMap
        .get_black_row(binaryMap.get_height() / 2)
        .expect("create");

    let startEnd = [145, 243]; //image pixels where the A1 pattern starts (at 124) and ends (at 214)
    let value = 0; // A
    let finderPatternA1 = FinderPattern::new(
        value,
        startEnd,
        startEnd[0],
        startEnd[1],
        image.height() / 2,
    );
    //{1, 8, 4, 1, 1};
    let mut rssExpandedReader = RSSExpandedReader::new();
    let dataCharacter = rssExpandedReader
        .decodeDataCharacter(&row, &finderPatternA1, true, true)
        .expect("decode");

    assert_eq!(98, dataCharacter.getValue());
}

#[test]
fn testDecodeDataCharacter() {
    let image = readImage("3.png");
    let binaryMap = BinaryBitmap::new(GlobalHistogramBinarizer::new(
        BufferedImageLuminanceSource::new(image.clone()),
    ));
    let row = binaryMap
        .get_black_row(binaryMap.get_height() / 2)
        .expect("create");

    let startEnd = [145, 243]; //image pixels where the A1 pattern starts (at 124) and ends (at 214)
    let value = 0; // A
    let finderPatternA1 = FinderPattern::new(
        value,
        startEnd,
        startEnd[0],
        startEnd[1],
        image.height() / 2,
    );
    //{1, 8, 4, 1, 1};
    let mut rssExpandedReader = RSSExpandedReader::new();
    let dataCharacter = rssExpandedReader
        .decodeDataCharacter(&row, &finderPatternA1, true, false)
        .expect("decode");

    assert_eq!(19, dataCharacter.getValue());
    assert_eq!(1007, dataCharacter.getChecksumPortion());
}

fn readImage(fileName: &str) -> image::DynamicImage {
    image::open(format!("test_resources/blackbox/rssexpanded-1/{fileName}")).unwrap()
    // Path path = AbstractBlackBoxTestCase.buildTestBase("src/test/resources/blackbox/rssexpanded-1/").resolve(fileName);
    // return ImageIO.read(path.toFile());
}
