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
#![cfg(all(feature = "image", feature = "datamatrix", feature = "decoders"))]

use rxing::MultiFormatReader;

mod common;

/**
 * @author bbrown@google.com (Brian Brown)
 */
#[cfg(feature = "image_formats")]
#[test]
fn data_matrix_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/datamatrix-1",
        MultiFormatReader::default(),
        rxing::BarcodeFormat::DATA_MATRIX,
    );
    // super("src/test/resources/blackbox/datamatrix-1", new MultiFormatReader(), BarcodeFormat.DATA_MATRIX);
    tester.add_test(29, 29, 0.0);
    tester.add_test(27, 27, 90.0);
    tester.add_test(27, 27, 180.0);
    tester.add_test(27, 27, 270.0);

    tester.test_black_box();
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn data_matrix_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/datamatrix-2",
        MultiFormatReader::default(),
        rxing::BarcodeFormat::DATA_MATRIX,
    );

    // super("src/test/resources/blackbox/datamatrix-2", new MultiFormatReader(), BarcodeFormat.DATA_MATRIX);
    tester.add_test_complex(18, 18, 0, 0, 0.0);
    tester.add_test_complex(18, 18, 0, 0, 90.0);
    tester.add_test_complex(18, 18, 0, 0, 180.0);
    tester.add_test_complex(18, 18, 0, 0, 270.0);
    tester.test_black_box();
}

/**
 * @author gitlost
 */
#[cfg(feature = "image_formats")]
#[test]
fn data_matrix_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/datamatrix-3",
        MultiFormatReader::default(),
        rxing::BarcodeFormat::DATA_MATRIX,
    );

    // super("src/test/resources/blackbox/datamatrix-3", new MultiFormatReader(), BarcodeFormat.DATA_MATRIX);
    tester.add_test(18, 18, 0.0);
    tester.add_test(18, 18, 90.0);
    tester.add_test(18, 18, 180.0);
    tester.add_test(18, 18, 270.0);
    tester.test_black_box();
}
