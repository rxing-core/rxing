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
