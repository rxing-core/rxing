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
#![cfg(feature = "image")]

use rxing::{BarcodeFormat, MultiFormatReader};

mod common;

/**
 * A test of {@link RSSExpandedReader} against a fixed test set of images.
 */
#[test]
fn rssexpanded_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/rssexpanded-1",
        MultiFormatReader::default(),
        BarcodeFormat::RSS_EXPANDED,
    );

    //  super("src/test/resources/blackbox/rssexpanded-1", new MultiFormatReader(), BarcodeFormat.RSS_EXPANDED);
    tester.add_test(32, 32, 0.0);
    tester.add_test(32, 32, 180.0);

    tester.test_black_box()
}

/**
 * A test of {@link RSSExpandedReader} against a fixed test set of images.
 */
#[test]
fn rssexpanded_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/rssexpanded-2",
        MultiFormatReader::default(),
        BarcodeFormat::RSS_EXPANDED,
    );

    // super("src/test/resources/blackbox/rssexpanded-2", new MultiFormatReader(), BarcodeFormat.RSS_EXPANDED);
    tester.add_test(21, 23, 0.0);
    tester.add_test(21, 23, 180.0);

    tester.test_black_box()
}

/**
 * A test of {@link RSSExpandedReader} against a fixed test set of images.
 */
#[test]
fn rssexpanded_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/rssexpanded-3",
        MultiFormatReader::default(),
        BarcodeFormat::RSS_EXPANDED,
    );

    //   super("src/test/resources/blackbox/rssexpanded-3", new MultiFormatReader(), BarcodeFormat.RSS_EXPANDED);
    tester.add_test(117, 117, 0.0);
    tester.add_test(117, 117, 180.0);

    tester.test_black_box()
}

/**
 * A test of {@link RSSExpandedReader} against a fixed test set of images including
 * stacked RSS barcodes.
 */
#[test]
fn rssexpanded_stacked_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/rssexpandedstacked-1",
        MultiFormatReader::default(),
        BarcodeFormat::RSS_EXPANDED,
    );

    //   super("src/test/resources/blackbox/rssexpandedstacked-1", new MultiFormatReader(), BarcodeFormat.RSS_EXPANDED);
    tester.add_test(59, 64, 0.0);
    tester.add_test(59, 64, 180.0);

    tester.test_black_box()
}

/**
 * A test of {@link RSSExpandedReader} against a fixed test set of images including
 * stacked RSS barcodes.
 */
#[test]
fn rssexpanded_stacked_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/rssexpandedstacked-2",
        MultiFormatReader::default(),
        BarcodeFormat::RSS_EXPANDED,
    );

    //   super("src/test/resources/blackbox/rssexpandedstacked-2", new MultiFormatReader(), BarcodeFormat.RSS_EXPANDED);
    tester.add_test(2, 7, 0.0);
    tester.add_test(2, 7, 180.0);

    tester.test_black_box()
}

/* one too few images are passing on above, probably a problem edge case */
