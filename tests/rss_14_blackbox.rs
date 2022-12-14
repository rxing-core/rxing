/*
 * Copyright 2008 ZXing authors
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

use rxing::{maxicode::MaxiCodeReader, BarcodeFormat, DecodeHintType, MultiFormatReader};

mod common;

/**
 * @author Sean Owen
 */
#[test]
fn rss14_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/rss14-1",
        MultiFormatReader::default(),
        BarcodeFormat::RSS_14,
    );

    //  super("src/test/resources/blackbox/rss14-1", new MultiFormatReader(), BarcodeFormat.RSS_14);
    tester.add_test(6, 6, 0.0);
    tester.add_test(6, 6, 180.0);

    tester.test_black_box()
}

/**
 * @author Sean Owen
 */
#[test]
fn rss14_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/rss14-2",
        MultiFormatReader::default(),
        BarcodeFormat::RSS_14,
    );

    //   super("src/test/resources/blackbox/rss14-2", new MultiFormatReader(), BarcodeFormat.RSS_14);
    tester.add_test_complex(4, 8, 1, 1, 0.0);
    tester.add_test_complex(3, 8, 0, 1, 180.0);

    tester.test_black_box()
}
