use std::io::Read;

use image::DynamicImage;
use rxing::{Dimension, Reader, Writer};

#[test]
fn issue_27_part_2() {
    let mut data = Vec::new();
    std::fs::File::open("test_resources/blackbox/github_issue_cases/panic_data2_issue_27.bin")
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    rxing::helpers::detect_multiple_in_luma(data, 720, 618).unwrap_or_default();
}

#[cfg(feature = "image")]
#[test]
fn issue_28() {
    use rxing::DecodeHints;

    let mut hints: DecodeHints =
        DecodeHints::default().with(rxing::DecodeHintValue::TryHarder(true));
    rxing::helpers::detect_multiple_in_file_with_hints("test_resources/blackbox/github_issue_cases/226611447-be6041dc-5b21-42fe-827b-068ccc59082c.png", &mut hints).unwrap_or_default();
}

#[cfg(feature = "image")]
#[test]
fn dynamsoft_all_supported_formats_image_fault() {
    use rxing::DecodeHints;

    let mut hints: DecodeHints =
        DecodeHints::default().with(rxing::DecodeHintValue::TryHarder(true));
    let results = rxing::helpers::detect_multiple_in_file_with_hints(
        "test_resources/blackbox/multi-1/AllSupportedBarcodeTypes.png",
        &mut hints,
    )
    .expect("must not fault during read");

    assert!(
        results.len() >= 11,
        "regression detection, base count of 11 codes"
    );

    // ToDo: This test is incomplete. Some that should be detected aren't, and some that are detected shouldn't be.
}

#[cfg(feature = "image")]
#[test]
fn zxing_bench_issue_1() {
    use rxing::BarcodeFormat;
    use rxing::DecodeHints;

    let mut hints: DecodeHints =
        DecodeHints::default().with(rxing::DecodeHintValue::TryHarder(true));
    let results = rxing::helpers::detect_multiple_in_file_with_hints(
        "test_resources/blackbox/github_issue_cases/170050507-1f10f0ef-82ca-4e14-a2d2-4b288ec54809.png",
        &mut hints,
    )
    .expect("must not fault during read");

    assert_eq!(
        results.len(),
        9,
        "must detect 9 barcodes, found: {}",
        results.len()
    );

    assert_eq!(results[0].getText(), "CODE39");
    assert_eq!(results[0].getBarcodeFormat(), &BarcodeFormat::CODE_39);

    assert_eq!(results[1].getText(), "012345");
    assert_eq!(results[1].getBarcodeFormat(), &BarcodeFormat::CODABAR);

    assert_eq!(results[2].getText(), "CODE128");
    assert_eq!(results[2].getBarcodeFormat(), &BarcodeFormat::CODE_128);

    assert_eq!(results[3].getText(), "00123456");
    assert_eq!(results[3].getBarcodeFormat(), &BarcodeFormat::ITF);

    assert_eq!(results[4].getText(), "CODE93");
    assert_eq!(results[4].getBarcodeFormat(), &BarcodeFormat::CODE_93);

    assert_eq!(results[5].getText(), "012345678905");
    assert_eq!(results[5].getBarcodeFormat(), &BarcodeFormat::UPC_A);

    assert_eq!(results[6].getText(), "01234565");
    assert_eq!(results[6].getBarcodeFormat(), &BarcodeFormat::EAN_8);

    assert_eq!(results[7].getText(), "01234565");
    assert_eq!(results[7].getBarcodeFormat(), &BarcodeFormat::UPC_E);

    assert_eq!(results[8].getText(), "1234567890128");
    assert_eq!(results[8].getBarcodeFormat(), &BarcodeFormat::EAN_13);

    /*
       Found 9 results
       Result 0:
       (code 39) CODE39
       Result 1:
       (codabar) 012345
       Result 2:
       (code 128) CODE128
       Result 3:
       (itf) 00123456
       Result 4:
       (code 93) CODE93
       Result 5:
       (upc a) 012345678905
       Result 6:
       (ean 8) 01234565
       Result 7:
       (upc e) 01234565
       Result 8:
       (ean 13) 1234567890128

    */
}

#[cfg(feature = "image")]
#[test]
fn issue_48() {
    use rxing::BarcodeFormat;
    use rxing::DecodeHints;

    let mut hints: DecodeHints =
        DecodeHints::default().with(rxing::DecodeHintValue::TryHarder(true));
    let results = rxing::helpers::detect_multiple_in_file_with_hints(
        "test_resources/blackbox/github_issue_cases/300908088-2b3ffe34-1067-48c9-8663-f841b5d0acf6.png",
        &mut hints,
    )
    .expect("must not fault during read");

    /*
       Found 3 results
       Result 0:
       (datamatrix) This is a Data Matrix by TEC-IT
       Result 1:
       (datamatrix) This is a Data Matrix by TEC-IT
       Result 2:
       (datamatrix) Hello world
    */

    assert_eq!(
        results.len(),
        3,
        "must detect 3 barcodes, found: {}",
        results.len()
    );

    assert_eq!(results[0].getText(), "This is a Data Matrix by TEC-IT");
    assert_eq!(results[0].getBarcodeFormat(), &BarcodeFormat::DATA_MATRIX);

    assert_eq!(results[1].getText(), "This is a Data Matrix by TEC-IT");
    assert_eq!(results[1].getBarcodeFormat(), &BarcodeFormat::DATA_MATRIX);

    assert_eq!(results[2].getText(), "Hello world");
    assert_eq!(results[2].getBarcodeFormat(), &BarcodeFormat::DATA_MATRIX);
}

#[cfg(feature = "image")]
#[test]
fn zxing_bench_grey_image_issue_luma8_image() {
    use image::DynamicImage;
    use rxing::{
        common::HybridBinarizer,
        multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader},
        BarcodeFormat, BinaryBitmap, BufferedImageLuminanceSource, DecodeHints, Exceptions,
        MultiUseMultiFormatReader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/170050507-1f10f0ef-82ca-4e14-a2d2-4b288ec54809.png";

    let mut hints = DecodeHints::default();

    let img = DynamicImage::from(
        image::open(FILE_NAME)
            .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
            .unwrap()
            .to_luma8(),
    );
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints.TryHarder = Some(true);

    let results = scanner
        .decode_multiple_with_hints(
            &mut BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(img))),
            &hints,
        )
        .expect("must not fault during read");

    assert_eq!(
        results.len(),
        9,
        "must detect 9 barcodes, found: {}",
        results.len()
    );

    assert_eq!(results[0].getText(), "CODE39");
    assert_eq!(results[0].getBarcodeFormat(), &BarcodeFormat::CODE_39);

    assert_eq!(results[1].getText(), "012345");
    assert_eq!(results[1].getBarcodeFormat(), &BarcodeFormat::CODABAR);

    assert_eq!(results[2].getText(), "CODE128");
    assert_eq!(results[2].getBarcodeFormat(), &BarcodeFormat::CODE_128);

    assert_eq!(results[3].getText(), "00123456");
    assert_eq!(results[3].getBarcodeFormat(), &BarcodeFormat::ITF);

    assert_eq!(results[4].getText(), "CODE93");
    assert_eq!(results[4].getBarcodeFormat(), &BarcodeFormat::CODE_93);

    assert_eq!(results[5].getText(), "012345678905");
    assert_eq!(results[5].getBarcodeFormat(), &BarcodeFormat::UPC_A);

    assert_eq!(results[6].getText(), "01234565");
    assert_eq!(results[6].getBarcodeFormat(), &BarcodeFormat::EAN_8);

    assert_eq!(results[7].getText(), "01234565");
    assert_eq!(results[7].getBarcodeFormat(), &BarcodeFormat::UPC_E);

    assert_eq!(results[8].getText(), "1234567890128");
    assert_eq!(results[8].getBarcodeFormat(), &BarcodeFormat::EAN_13);

    /*
       Found 9 results
       Result 0:
       (code 39) CODE39
       Result 1:
       (codabar) 012345
       Result 2:
       (code 128) CODE128
       Result 3:
       (itf) 00123456
       Result 4:
       (code 93) CODE93
       Result 5:
       (upc a) 012345678905
       Result 6:
       (ean 8) 01234565
       Result 7:
       (upc e) 01234565
       Result 8:
       (ean 13) 1234567890128

    */
}

#[cfg(feature = "image")]
#[test]
fn zxing_bench_grey_image_issue_raw_luma8() {
    use rxing::{
        common::HybridBinarizer,
        multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader},
        BarcodeFormat, BinaryBitmap, DecodeHints, Exceptions, Luma8LuminanceSource,
        MultiUseMultiFormatReader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/170050507-1f10f0ef-82ca-4e14-a2d2-4b288ec54809.png";

    let mut hints = DecodeHints::default();

    let img = image::open(FILE_NAME)
        .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
        .unwrap();
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints.TryHarder = Some(true);

    let results = scanner
        .decode_multiple_with_hints(
            &mut BinaryBitmap::new(HybridBinarizer::new(Luma8LuminanceSource::new(
                img.to_luma8().into_raw(),
                img.width(),
                img.height(),
            ))),
            &hints,
        )
        .expect("must not fault during read");

    assert_eq!(
        results.len(),
        9,
        "must detect 9 barcodes, found: {}",
        results.len()
    );

    assert_eq!(results[0].getText(), "CODE39");
    assert_eq!(results[0].getBarcodeFormat(), &BarcodeFormat::CODE_39);

    assert_eq!(results[1].getText(), "012345");
    assert_eq!(results[1].getBarcodeFormat(), &BarcodeFormat::CODABAR);

    assert_eq!(results[2].getText(), "CODE128");
    assert_eq!(results[2].getBarcodeFormat(), &BarcodeFormat::CODE_128);

    assert_eq!(results[3].getText(), "00123456");
    assert_eq!(results[3].getBarcodeFormat(), &BarcodeFormat::ITF);

    assert_eq!(results[4].getText(), "CODE93");
    assert_eq!(results[4].getBarcodeFormat(), &BarcodeFormat::CODE_93);

    assert_eq!(results[5].getText(), "012345678905");
    assert_eq!(results[5].getBarcodeFormat(), &BarcodeFormat::UPC_A);

    assert_eq!(results[6].getText(), "01234565");
    assert_eq!(results[6].getBarcodeFormat(), &BarcodeFormat::EAN_8);

    assert_eq!(results[7].getText(), "01234565");
    assert_eq!(results[7].getBarcodeFormat(), &BarcodeFormat::UPC_E);

    assert_eq!(results[8].getText(), "1234567890128");
    assert_eq!(results[8].getBarcodeFormat(), &BarcodeFormat::EAN_13);

    /*
       Found 9 results
       Result 0:
       (code 39) CODE39
       Result 1:
       (codabar) 012345
       Result 2:
       (code 128) CODE128
       Result 3:
       (itf) 00123456
       Result 4:
       (code 93) CODE93
       Result 5:
       (upc a) 012345678905
       Result 6:
       (ean 8) 01234565
       Result 7:
       (upc e) 01234565
       Result 8:
       (ean 13) 1234567890128

    */
}

#[cfg(feature = "image")]
#[test]
fn test_issue_49() {
    use rxing::{
        common::HybridBinarizer,
        multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader},
        BarcodeFormat, BinaryBitmap, DecodeHints, Exceptions, Luma8LuminanceSource,
        MultiUseMultiFormatReader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/345143005-4538852a-242a-4f77-87cc-fefb66856ecf.png";
    let mut hints = DecodeHints::default();

    let img = image::open(FILE_NAME)
        .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
        .unwrap();
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints.TryHarder = Some(true);

    let results = scanner
        .decode_multiple_with_hints(
            &mut BinaryBitmap::new(HybridBinarizer::new(Luma8LuminanceSource::new(
                img.to_luma8().into_raw(),
                img.width(),
                img.height(),
            ))),
            &hints,
        )
        .expect("must not fault during read");

    assert_eq!(results.len(), 3);

    let itf_result = results
        .iter()
        .find(|r| r.getBarcodeFormat() == &BarcodeFormat::ITF)
        .expect("must find an ITF barcode");

    const EXPECTED_ITF_TEXT: &str = "85680000000343202687700000014672192100000000";

    assert_eq!(EXPECTED_ITF_TEXT, itf_result.getText());
}

#[cfg(feature = "image")]
#[test]
fn test_issue_50() {
    use rxing::{
        common::HybridBinarizer, BarcodeFormat, BinaryBitmap, DecodeHints, Exceptions,
        Luma8LuminanceSource, MultiUseMultiFormatReader, Reader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/346304318-16acfb7a-4a41-4b15-af78-7ccf061e72bd.png";
    let mut hints = DecodeHints::default();

    let img = image::open(FILE_NAME)
        .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
        .unwrap();
    let mut scanner = MultiUseMultiFormatReader::default();

    hints.TryHarder = Some(true);
    hints.PureBarcode = Some(true);

    let result = scanner
        .decode_with_hints(
            &mut BinaryBitmap::new(HybridBinarizer::new(Luma8LuminanceSource::new(
                img.to_luma8().into_raw(),
                img.width(),
                img.height(),
            ))),
            &hints,
        )
        .expect("must not fault during read");

    const EXPECTED_FORMAT: &BarcodeFormat = &BarcodeFormat::ITF;
    const EXPECTED_ITF_TEXT: &str = "85680000001403303242024070501202400002535294";

    assert_eq!(EXPECTED_ITF_TEXT, result.getText());

    assert_eq!(EXPECTED_FORMAT, result.getBarcodeFormat());
}

#[cfg(feature = "image")]
#[test]
fn test_issue_50_2() {
    use rxing::{
        common::AdaptiveThresholdBinarizer, BarcodeFormat, BinaryBitmap, DecodeHints, Exceptions,
        Luma8LuminanceSource, MultiUseMultiFormatReader, Reader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/346304318-16acfb7a-4a41-4b15-af78-7ccf061e72bd.png";
    let mut hints = DecodeHints::default();

    let img = image::open(FILE_NAME)
        .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
        .unwrap();
    let mut scanner = MultiUseMultiFormatReader::default();

    hints.TryHarder = Some(true);

    let result = scanner
        .decode_with_hints(
            &mut BinaryBitmap::new(AdaptiveThresholdBinarizer::new(
                Luma8LuminanceSource::new(img.to_luma8().into_raw(), img.width(), img.height()),
                1,
            )),
            &hints,
        )
        .expect("must not fault during read");

    const EXPECTED_FORMAT: &BarcodeFormat = &BarcodeFormat::ITF;
    const EXPECTED_ITF_TEXT: &str = "85680000001403303242024070501202400002535294";

    assert_eq!(EXPECTED_ITF_TEXT, result.getText());

    assert_eq!(EXPECTED_FORMAT, result.getBarcodeFormat());
}

#[cfg(feature = "image")]
#[test]
fn issue_51_multiple_detection() {
    use image::DynamicImage;
    use rxing::{
        common::HybridBinarizer,
        multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader},
        BinaryBitmap, BufferedImageLuminanceSource, DecodeHints, Exceptions,
        MultiUseMultiFormatReader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/349949736-8e3b9d66-d114-41ca-a8e0-f1332d111827.jpeg";

    let mut hints = DecodeHints::default();

    let img = DynamicImage::from(
        image::open(FILE_NAME)
            .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
            .unwrap()
            .to_luma8(),
    );
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints.TryHarder = Some(true);

    let results = scanner
        .decode_multiple_with_hints(
            &mut BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(img))),
            &hints,
        )
        .expect("must not fault during read image 1");

    assert_eq!(
        results.len(),
        1,
        "must detect 1 barcodes, found: {}",
        results.len()
    );

    const FILE_NAME2 : &str = "test_resources/blackbox/github_issue_cases/349949791-1e8b67a7-0994-46fb-bd86-a5f3cd79f0e5.jpeg";

    let mut hints = DecodeHints::default();

    let img = DynamicImage::from(
        image::open(FILE_NAME2)
            .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME2}: {e}")))
            .unwrap()
            .to_luma8(),
    );
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints.TryHarder = Some(true);

    let results = scanner
        .decode_multiple_with_hints(
            &mut BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(img))),
            &hints,
        )
        .expect("must not fault during read of image 2");

    assert_eq!(
        results.len(),
        1,
        "must detect 1 barcodes, found: {}",
        results.len()
    );
}

#[cfg(feature = "image")]
#[test]
fn issue_58() {
    use rxing::{DecodeHints, Exceptions};

    let mut hints: DecodeHints =
        DecodeHints::default().with(rxing::DecodeHintValue::TryHarder(true));
    assert!(rxing::helpers::detect_multiple_in_file_with_hints(
        "test_resources/blackbox/github_issue_cases/empty_issue_58.png",
        &mut hints
    )
    .is_err_and(|e| { e == Exceptions::NOT_FOUND }));

    hints.PureBarcode = Some(true);

    assert!(rxing::helpers::detect_multiple_in_file_with_hints(
        "test_resources/blackbox/github_issue_cases/empty_issue_58.png",
        &mut hints
    )
    .is_err_and(|e| { e == Exceptions::NOT_FOUND }));
}

#[cfg(feature = "image")]
#[test]
fn issue_59() {
    use rand::prelude::*;
    use rxing::{BufferedImageLuminanceSource, DecodeHints, EncodeHints};

    const TEST_SIZE: usize = 1400;
    const TEST_2_SIZE: usize = 100;

    let mut rnd_data = [0; TEST_SIZE];
    rand::rng().fill_bytes(&mut rnd_data);
    let data = rnd_data.into_iter().map(|c| c as char).collect::<String>();

    let writer = rxing::datamatrix::DataMatrixWriter;
    let data_matrix = writer
        .encode(&data, &rxing::BarcodeFormat::DATA_MATRIX, 0, 0)
        .expect("must encode with size of 500");

    let mut rnd_data_2 = [0; TEST_2_SIZE];
    rand::rng().fill_bytes(&mut rnd_data_2);
    let data2 = rnd_data_2
        .into_iter()
        .map(|c| c as char)
        .collect::<String>();

    #[allow(deprecated)]
    let hints =
        EncodeHints::default().with(rxing::EncodeHintValue::MinSize(Dimension::new(48, 48)));
    let data_matrix_2 = writer
        .encode_with_hints(&data2, &rxing::BarcodeFormat::DATA_MATRIX, 0, 0, &hints)
        .expect("must encode with minimum size of 48x48");

    let decode_hints = DecodeHints::default().with(rxing::DecodeHintValue::TryHarder(true));

    let img: DynamicImage = data_matrix.into();
    let ls = BufferedImageLuminanceSource::new(img);
    let mut bb = rxing::BinaryBitmap::new(rxing::common::HybridBinarizer::new(ls));
    let detection = rxing::MultiFormatReader::default()
        .decode_with_hints(&mut bb, &decode_hints)
        .expect("must decode first image");
    assert_eq!(detection.getText(), data);

    let img: DynamicImage = data_matrix_2.into();
    let ls = BufferedImageLuminanceSource::new(img);
    let mut bb = rxing::BinaryBitmap::new(rxing::common::HybridBinarizer::new(ls));
    let detection = rxing::MultiFormatReader::default()
        .decode_with_hints(&mut bb, &decode_hints)
        .expect("must decode first image");
    assert_eq!(detection.getText(), data2);
}

#[cfg(feature = "image")]
#[test]
fn issue_69_timed() {
    use std::{sync::mpsc, thread, time::Duration};

    use rxing::DecodeHints;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Capture panic so the test can surface it
        let mut hints: DecodeHints =
            DecodeHints::default().with(rxing::DecodeHintValue::TryHarder(true));

        use std::panic::{catch_unwind, AssertUnwindSafe};
        let r = catch_unwind(AssertUnwindSafe(|| {
            assert!(rxing::helpers::detect_multiple_in_file_with_hints(
        "test_resources/blackbox/github_issue_cases/481273207-b43215de-7369-4a04-a695-984f27fd1225.jpg",
        &mut hints
    ).is_ok())
        }));
        let _ = tx.send(r);
    });

    match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(Ok(())) => {} // finished in time
        Ok(Err(e)) => panic!("search panicked with: {e:?}"),
        Err(_) => panic!("search timed out"), // did not finish in time
    }
}

#[cfg(feature = "image")]
#[test]
fn issue_69() {
    use rxing::DecodeHints;

    let mut hints: DecodeHints =
        DecodeHints::default().with(rxing::DecodeHintValue::TryHarder(true));
    let results = rxing::helpers::detect_multiple_in_file_with_hints(
        "test_resources/blackbox/github_issue_cases/481273207-b43215de-7369-4a04-a695-984f27fd1225.jpg",
        &mut hints
    ).expect("should return results from decoding");

    assert!(
        results.len() >= 5,
        "Search should return at least 5 results"
    );

    /*

    Found 5 results
    Result 0:
    (datamatrix) 011060329549205420151734113010D24121203
    Result 1:
    (datamatrix) 01106032951299431726113010M8190P
    Result 2:
    (code 128) 01106032954920542015
    Result 3:
    (datamatrix) 0110603295041283202217350131104559233
    Result 4:
    (code 128) 1734113010D24121203
     */
}
