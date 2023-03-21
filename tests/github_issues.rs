use std::io::Read;

use rxing::DecodingHintDictionary;

#[test]
fn issue_27_part_2() {
    let mut data = Vec::new();
    std::fs::File::open("test_resources/blackbox/github_issue_cases/226611447-be6041dc-5b21-42fe-827b-068ccc59082c.png")
        .unwrap()
        .read_to_end(&mut data)
        .unwrap();

    rxing::helpers::detect_multiple_in_luma(data, 720, 618).unwrap_or_default();
}

#[test]
fn issue_28() {
    let mut hints: DecodingHintDictionary = DecodingHintDictionary::new();
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );
    rxing::helpers::detect_multiple_in_file_with_hints("test_resources/blackbox/github_issue_cases/226611447-be6041dc-5b21-42fe-827b-068ccc59082c.png", &mut hints).unwrap_or_default();
}
