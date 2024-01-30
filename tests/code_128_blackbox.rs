#![cfg(feature = "image")]

use rxing::{BarcodeFormat, MultiFormatReader};

mod common;

/**
 * @author Sean Owen
 */
#[cfg(feature = "image_formats")]
#[test]
fn code128_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/code128-1",
        MultiFormatReader::default(),
        BarcodeFormat::CODE_128,
    );

    //   super("src/test/resources/blackbox/code128-1", new MultiFormatReader(), BarcodeFormat.CODE_128);
    tester.add_test(6, 6, 0.0);
    tester.add_test(6, 6, 180.0);

    tester.test_black_box()
}

/**
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn code128_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/code128-2",
        MultiFormatReader::default(),
        BarcodeFormat::CODE_128,
    );

    //   super("src/test/resources/blackbox/code128-2", new MultiFormatReader(), BarcodeFormat.CODE_128);
    tester.add_test(36, 39, 0.0);
    tester.add_test(36, 39, 180.0);

    tester.test_black_box()
}

/**
 * @author Sean Owen
 */
#[cfg(feature = "image_formats")]
#[test]
fn code128_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/code128-3",
        MultiFormatReader::default(),
        BarcodeFormat::CODE_128,
    );

    //   super("src/test/resources/blackbox/code128-3", new MultiFormatReader(), BarcodeFormat.CODE_128);
    tester.add_test(2, 2, 0.0);
    tester.add_test(2, 2, 180.0);

    tester.test_black_box()
}
