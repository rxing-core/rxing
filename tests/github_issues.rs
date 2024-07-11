use std::io::Read;

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
    use rxing::DecodingHintDictionary;

    let mut hints: DecodingHintDictionary = DecodingHintDictionary::new();
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );
    rxing::helpers::detect_multiple_in_file_with_hints("test_resources/blackbox/github_issue_cases/226611447-be6041dc-5b21-42fe-827b-068ccc59082c.png", &mut hints).unwrap_or_default();
}

#[cfg(feature = "image")]
#[test]
fn dynamsoft_all_supported_formats_image_fault() {
    use rxing::DecodingHintDictionary;

    let mut hints: DecodingHintDictionary = DecodingHintDictionary::new();
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );
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
    use rxing::{BarcodeFormat, DecodingHintDictionary};

    let mut hints: DecodingHintDictionary = DecodingHintDictionary::new();
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );
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
    use rxing::{BarcodeFormat, DecodingHintDictionary};

    let mut hints: DecodingHintDictionary = DecodingHintDictionary::new();
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );
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
        BarcodeFormat, BinaryBitmap, BufferedImageLuminanceSource, DecodeHintType, DecodeHintValue,
        DecodingHintDictionary, Exceptions, MultiUseMultiFormatReader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/170050507-1f10f0ef-82ca-4e14-a2d2-4b288ec54809.png";

    let mut hints = DecodingHintDictionary::default();

    let img = DynamicImage::from(
        image::open(FILE_NAME)
            .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
            .unwrap()
            .to_luma8(),
    );
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints
        .entry(DecodeHintType::TRY_HARDER)
        .or_insert(DecodeHintValue::TryHarder(true));

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
        BarcodeFormat, BinaryBitmap, DecodeHintType, DecodeHintValue, DecodingHintDictionary,
        Exceptions, Luma8LuminanceSource, MultiUseMultiFormatReader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/170050507-1f10f0ef-82ca-4e14-a2d2-4b288ec54809.png";

    let mut hints = DecodingHintDictionary::default();

    let img = image::open(FILE_NAME)
        .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
        .unwrap();
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints
        .entry(DecodeHintType::TRY_HARDER)
        .or_insert(DecodeHintValue::TryHarder(true));

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
        BarcodeFormat, BinaryBitmap, DecodeHintType, DecodeHintValue, DecodingHintDictionary,
        Exceptions, Luma8LuminanceSource, MultiUseMultiFormatReader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/345143005-4538852a-242a-4f77-87cc-fefb66856ecf.png";
    let mut hints = DecodingHintDictionary::default();

    let img = image::open(FILE_NAME)
        .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
        .unwrap();
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints
        .entry(DecodeHintType::TRY_HARDER)
        .or_insert(DecodeHintValue::TryHarder(true));

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
        common::HybridBinarizer, BarcodeFormat, BinaryBitmap, DecodeHintType, DecodeHintValue,
        DecodingHintDictionary, Exceptions, Luma8LuminanceSource, MultiUseMultiFormatReader,
        Reader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/346304318-16acfb7a-4a41-4b15-af78-7ccf061e72bd.png";
    let mut hints = DecodingHintDictionary::default();

    let img = image::open(FILE_NAME)
        .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
        .unwrap();
    let mut scanner = MultiUseMultiFormatReader::default();

    hints
        .entry(DecodeHintType::TRY_HARDER)
        .or_insert(DecodeHintValue::TryHarder(true));

    hints
        .entry(DecodeHintType::PURE_BARCODE)
        .or_insert(DecodeHintValue::PureBarcode(true));

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
        common::{AdaptiveThresholdBinarizer, HybridBinarizer},
        BarcodeFormat, BinaryBitmap, DecodeHintType, DecodeHintValue, DecodingHintDictionary,
        Exceptions, Luma8LuminanceSource, MultiUseMultiFormatReader, Reader,
    };

    const FILE_NAME : &str = "test_resources/blackbox/github_issue_cases/346304318-16acfb7a-4a41-4b15-af78-7ccf061e72bd.png";
    let mut hints = DecodingHintDictionary::default();

    let img = image::open(FILE_NAME)
        .map_err(|e| Exceptions::runtime_with(format!("couldn't read {FILE_NAME}: {e}")))
        .unwrap();
    let mut scanner = MultiUseMultiFormatReader::default();

    hints
        .entry(DecodeHintType::TRY_HARDER)
        .or_insert(DecodeHintValue::TryHarder(true));

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
