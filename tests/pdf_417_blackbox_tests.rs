#![cfg(feature = "image")]

use rxing::MultiFormatReader;

mod common;

/**
 * This test consists of perfect, computer-generated images. We should have 100% passing.
 *
 * @author SITA Lab (kevin.osullivan@sita.aero)
 */
#[cfg(feature = "image_formats")]
#[test]
fn pdf417_black_box1_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/pdf417-1",
        MultiFormatReader::default(),
        rxing::BarcodeFormat::PDF_417,
    );
    //   super("src/test/resources/blackbox/pdf417-1", new MultiFormatReader(), BarcodeFormat.PDF_417);
    tester.add_test(10, 10, 0.0);
    tester.add_test(10, 10, 90.0);
    tester.add_test(10, 10, 180.0);
    tester.add_test(10, 10, 270.0);

    tester.test_black_box()
}

/**
 * This test contains 480x240 images captured from an Android device at preview resolution.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[cfg(feature = "image_formats")]
#[test]
fn pdf417_black_box2_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/pdf417-2",
        MultiFormatReader::default(),
        rxing::BarcodeFormat::PDF_417,
    );
    //   super("src/test/resources/blackbox/pdf417-2", new MultiFormatReader(), BarcodeFormat.PDF_417);
    tester.add_test_complex(25, 25, 0, 0, 0.0);
    tester.add_test_complex(25, 25, 0, 0, 180.0);
    tester.test_black_box()
}

/**
 * Tests {@link PDF417Reader} against more sample images.
 */
#[cfg(feature = "image_formats")]
#[test]
fn pdf417_black_box3_test_case() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/pdf417-3",
        MultiFormatReader::default(),
        rxing::BarcodeFormat::PDF_417,
    );
    //   super("src/test/resources/blackbox/pdf417-3", new MultiFormatReader(), BarcodeFormat.PDF_417);
    tester.add_test_complex(19, 19, 0, 0, 0.0);
    tester.add_test_complex(19, 19, 0, 0, 180.0);
    tester.test_black_box()
}
