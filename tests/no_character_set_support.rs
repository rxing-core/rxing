#![cfg(all(
    feature = "no_character_set_support",
    feature = "encoders",
    feature = "decoders",
    feature = "qrcode"
))]

//! A build without character set tables still reads and writes the text most
//! QR codes carry. `no_character_set_support` used to panic on the first
//! byte-mode segment, which is every QR code holding a URL.

use rxing::{
    BarcodeFormat, EncodeHintValue, EncodeHints, MultiFormatWriter, Writer, common::CharacterSet,
    helpers::detect_in_luma_slice,
};

fn roundtrip(content: &str, hints: &EncodeHints) -> rxing::common::Result<String> {
    let bits =
        MultiFormatWriter.encode_with_hints(content, &BarcodeFormat::QR_CODE, 256, 256, hints)?;
    let (w, h) = (bits.width(), bits.height());
    let mut luma = Vec::with_capacity((w * h) as usize);
    for y in 0..h {
        for x in 0..w {
            luma.push(if bits.get(x, y) { 0u8 } else { 255u8 });
        }
    }
    let result = detect_in_luma_slice(&luma, w, h, Some(BarcodeFormat::QR_CODE))?;
    Ok(result.getText().to_owned())
}

fn charset(name: &str) -> EncodeHints {
    EncodeHints::default().with(EncodeHintValue::CharacterSet(name.to_owned()))
}

#[test]
fn ascii_roundtrips() {
    let content = "https://example.com/enroll?host=h:1#code=0123456789abcdef";
    assert_eq!(
        roundtrip(content, &EncodeHints::default()).unwrap(),
        content
    );
}

#[test]
fn iso_8859_1_roundtrips() {
    let content = "Grüße aus Luleå";
    assert_eq!(
        roundtrip(content, &EncodeHints::default()).unwrap(),
        content
    );
}

#[test]
fn utf_8_roundtrips() {
    let content = "QR コード – ℮";
    assert_eq!(roundtrip(content, &charset("UTF-8")).unwrap(), content);
}

#[test]
fn a_set_that_needs_tables_is_an_error_not_a_panic() {
    assert!(roundtrip("テスト", &charset("Shift_JIS")).is_err());
}

#[test]
fn table_free_sets_roundtrip() {
    let text = "Grüße, 𝄞";
    for set in [
        CharacterSet::UTF8,
        CharacterSet::UTF16BE,
        CharacterSet::UTF16LE,
        CharacterSet::UTF32BE,
        CharacterSet::UTF32LE,
    ] {
        let bytes = set.encode(text).unwrap();
        assert_eq!(set.decode(&bytes).unwrap(), text, "{set:?}");
    }
    let latin = "Grüße";
    for set in [CharacterSet::ISO8859_1, CharacterSet::Binary] {
        let bytes = set.encode(latin).unwrap();
        assert_eq!(set.decode(&bytes).unwrap(), latin, "{set:?}");
    }
    assert_eq!(CharacterSet::ASCII.decode(b"plain").unwrap(), "plain");
}

#[test]
fn sets_that_need_tables_are_errors() {
    for set in [
        CharacterSet::Shift_JIS,
        CharacterSet::Big5,
        CharacterSet::GB18030,
        CharacterSet::EUC_KR,
        CharacterSet::Cp437,
        CharacterSet::Cp1252,
    ] {
        assert!(set.decode(b"abc").is_err(), "{set:?}");
        assert!(set.encode("abc").is_err(), "{set:?}");
    }
}

#[test]
fn malformed_input_is_an_error_or_replaced() {
    assert!(CharacterSet::UTF8.decode(b"\xFF").is_err());
    assert_eq!(
        CharacterSet::UTF8.decode_replace(b"a\xFF").unwrap(),
        "a\u{FFFD}"
    );
    assert!(CharacterSet::UTF16BE.decode(b"\x00a\x00").is_err());
    assert_eq!(
        CharacterSet::UTF16BE.decode_replace(b"\x00a\x00").unwrap(),
        "a\u{FFFD}"
    );
    assert!(CharacterSet::ASCII.decode(b"\x80").is_err());
    assert!(CharacterSet::ISO8859_1.encode("€").is_err());
    assert_eq!(CharacterSet::ISO8859_1.encode_replace("a€").unwrap(), b"a?");
}

#[test]
fn a_utf_8_byte_order_mark_is_not_text() {
    assert_eq!(
        CharacterSet::UTF8.decode(b"\xEF\xBB\xBFtext").unwrap(),
        "text"
    );
}
