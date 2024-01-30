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
#![cfg(feature = "image")]

use rxing::{BarcodeFormat, MultiFormatReader};

mod common;

/**
 * @author Sean Owen
 */
#[cfg(feature = "image_formats")]
#[test]
fn upcablack_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upca-1",
        MultiFormatReader::default(),
        // UPCAReader::default(),
        BarcodeFormat::UPC_A,
    );

    //  super("src/test/resources/blackbox/upca-1", new MultiFormatReader(), BarcodeFormat.UPC_A);
    tester.add_test_complex(14, 18, 0, 1, 0.0);
    tester.add_test_complex(16, 18, 0, 1, 180.0);

    tester.test_black_box()
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn upcablack_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upca-2",
        MultiFormatReader::default(),
        // UPCAReader::default(),
        BarcodeFormat::UPC_A,
    );

    //   super("src/test/resources/blackbox/upca-2", new MultiFormatReader(), BarcodeFormat.UPC_A);
    tester.add_test_complex(28, 36, 0, 2, 0.0);
    tester.add_test_complex(29, 36, 0, 2, 180.0);

    tester.test_black_box()
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn upcablack_box3_reflective_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upca-3",
        MultiFormatReader::default(),
        // UPCAReader::default(),
        BarcodeFormat::UPC_A,
    );

    //   super("src/test/resources/blackbox/upca-3", new MultiFormatReader(), BarcodeFormat.UPC_A);
    tester.add_test_complex(7, 9, 0, 2, 0.0);
    tester.add_test_complex(8, 9, 0, 2, 180.0);

    tester.test_black_box()
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn upcablack_box4_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upca-4",
        MultiFormatReader::default(),
        // UPCAReader::default(),
        BarcodeFormat::UPC_A,
    );

    //   super("src/test/resources/blackbox/upca-4", new MultiFormatReader(), BarcodeFormat.UPC_A);
    tester.add_test_complex(9, 11, 0, 1, 0.0);
    tester.add_test_complex(9, 11, 0, 1, 180.0);

    tester.test_black_box()
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn upcablack_box5_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upca-5",
        MultiFormatReader::default(),
        // UPCAReader::default(),
        BarcodeFormat::UPC_A,
    );

    //   super("src/test/resources/blackbox/upca-5", new MultiFormatReader(), BarcodeFormat.UPC_A);
    tester.add_test_complex(20, 23, 0, 0, 0.0);
    tester.add_test_complex(22, 23, 0, 0, 180.0);

    tester.test_black_box()
}

/**
 * A set of blurry images taken with a fixed-focus device.
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn upcablack_box6_blurry_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/upca-6",
        MultiFormatReader::default(),
        // UPCAReader::default(),
        BarcodeFormat::UPC_A,
    );

    //   super("src/test/resources/blackbox/upca-6", new MultiFormatReader(), BarcodeFormat.UPC_A);
    tester.add_test(0, 0, 0.0);
    tester.add_test(0, 0, 180.0);

    tester.test_black_box()
}
