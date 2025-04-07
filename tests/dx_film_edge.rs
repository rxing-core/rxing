// dxfilmedge-1

#![cfg(all(feature = "image", feature = "experimental_features"))]

use rxing::{BarcodeFormat, FilteredImageReader, MultiFormatReader};

mod common;

#[cfg(feature = "image_formats")]
#[test]
fn dx_film_edge() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/dxfilmedge-1",
        FilteredImageReader::new(MultiFormatReader::default()),
        BarcodeFormat::DXFilmEdge,
    );

    tester.add_test(1, 2, 0.0);
    tester.add_test(1, 2, 90.0);
    tester.add_test(1, 2, 180.0);
    tester.add_test(1, 2, 320.0);

    tester.test_black_box()
}
