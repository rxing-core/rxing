// SPDX-License-Identifier: Apache-2.0

use crate::{
    BarcodeFormat, BinaryBitmap, DecodeHintValue, DecodeHints, EncodeHintValue, EncodeHints,
    Luma8LuminanceSource, MultiFormatReader, RXingResult, RXingResultMetadataType,
    RXingResultMetadataValue, Reader, Writer,
    common::{BitArray, BitMatrix, HybridBinarizer, cpp_essentials::ByteMatrix},
    qrcode::{
        QRCodeWriter,
        common::{ErrorCorrectionLevel, Version},
        encoder::{matrix_util, qrcode_encoder},
    },
};

fn read(matrix: &BitMatrix, hints: &DecodeHints) -> RXingResult {
    // 4 modules of quiet zone, 8 pixels per module
    let width = (matrix.getWidth() + 8) * 8;
    let height = (matrix.getHeight() + 8) * 8;
    let luma = (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .map(|(x, y)| {
            let (x, y) = (x / 8, y / 8);
            let dark = (4..matrix.getWidth() + 4).contains(&x)
                && (4..matrix.getHeight() + 4).contains(&y)
                && matrix.get(x - 4, y - 4);
            if dark { 0 } else { 255 }
        })
        .collect();
    let source = Luma8LuminanceSource::new(luma, width, height).expect("luma");
    let result = MultiFormatReader::default()
        .decode_with_hints(&mut BinaryBitmap::new(HybridBinarizer::new(source)), hints)
        .expect("decode");
    assert_eq!(BarcodeFormat::QR_CODE, *result.getBarcodeFormat());
    result
}

fn write_and_read(contents: &str, encode_hints: &EncodeHints, hints: &DecodeHints) -> RXingResult {
    let matrix = QRCodeWriter
        .encode_with_hints(contents, &BarcodeFormat::QR_CODE, 0, 0, encode_hints)
        .expect("encode");
    read(&matrix, hints)
}

/// A version 1-L symbol of byte-mode segments, each with an optional ECI before it.
fn segments_symbol(segments: &[(Option<usize>, &[u8])]) -> BitMatrix {
    let mut bits = BitArray::new();
    for (eci, data) in segments {
        if let Some(eci) = eci {
            bits.appendBits(0x07, 4).expect("append"); // ECI mode, then a one-byte designator
            bits.appendBits(*eci, 8).expect("append");
        }
        bits.appendBits(0x04, 4).expect("append"); // Byte mode
        bits.appendBits(data.len(), 8).expect("append");
        for &byte in *data {
            bits.appendBits(byte as usize, 8).expect("append");
        }
    }
    qrcode_encoder::terminateBits(19, &mut bits).expect("terminate");
    let bits = qrcode_encoder::interleaveWithECBytes(&bits, 26, 19, 1).expect("interleave");
    let mut matrix = ByteMatrix::new(21, 21);
    let version = Version::getVersionForNumber(1).expect("version");
    matrix_util::buildMatrix(&bits, &ErrorCorrectionLevel::L, version, 0, &mut matrix)
        .expect("build");
    matrix.try_into().expect("matrix")
}

fn symbology_identifier(result: &RXingResult) -> &str {
    match result
        .getRXingResultMetadata()
        .get(&RXingResultMetadataType::SYMBOLOGY_IDENTIFIER)
    {
        Some(RXingResultMetadataValue::SymbologyIdentifier(id)) => id,
        _ => panic!("no symbology identifier"),
    }
}

fn character_set_hint(name: &str) -> DecodeHints {
    DecodeHints::default().with(DecodeHintValue::CharacterSet(name.to_owned()))
}

#[test]
fn CharacterSetHintForByteModeWithoutECI() {
    // With default hints, QRCodeWriter writes this code point as the byte-mode byte C3 without
    // an ECI: "ﾃ" in Shift_JIS. A single half-width katakana is guessed as ISO-8859-1.
    let result = write_and_read(
        "\u{C3}",
        &EncodeHints::default(),
        &character_set_hint("Shift_JIS"),
    );
    assert_eq!("]Q1", symbology_identifier(&result));
    assert_eq!("\u{FF83}", result.getText());
}

#[test]
fn CharacterSetHintBeforeECI() {
    // Byte C3, then ECI 26 and byte 41: the ECI applies from its position on, so the byte
    // before it takes the hint.
    let symbol = segments_symbol(&[(None, &[0xC3]), (Some(26), b"A")]);
    let result = read(&symbol, &character_set_hint("Shift_JIS"));
    assert_eq!("]Q2", symbology_identifier(&result));
    assert_eq!("\u{FF83}A", result.getText());
}

#[test]
fn CharacterSetHintBeforeUnknownECI() {
    // As above, with ECI 100, which rxing does not know.
    let symbol = segments_symbol(&[(None, &[0xC3]), (Some(100), b"A")]);
    let result = read(&symbol, &character_set_hint("Shift_JIS"));
    assert_eq!("]Q2", symbology_identifier(&result));
    assert_eq!("\u{FF83}A", result.getText());
}

#[test]
fn ECIOverridesCharacterSetHint() {
    // ECI 26 (UTF-8), then the UTF-8 bytes 47 72 C3 B6 C3 9F 65.
    let encode_hints =
        EncodeHints::default().with(EncodeHintValue::CharacterSet("UTF-8".to_owned()));
    let result = write_and_read("Größe", &encode_hints, &character_set_hint("Shift_JIS"));
    assert_eq!("]Q2", symbology_identifier(&result));
    assert_eq!("Größe", result.getText());
}

#[test]
fn UnknownECIIgnoresCharacterSetHint() {
    // ECI 100, which rxing does not know, then the UTF-8 bytes of "Größe": the hint must not
    // apply under it.
    let symbol = segments_symbol(&[(Some(100), "Größe".as_bytes())]);
    let result = read(&symbol, &character_set_hint("Shift_JIS"));
    assert_eq!("]Q2", symbology_identifier(&result));
    assert_eq!("Größe", result.getText());
}

#[test]
fn UnknownCharacterSetHintIsIgnored() {
    // The bytes 47 72 C3 B6 C3 9F 65 without an ECI: "Größe" in UTF-8. A name rxing
    // does not know must not turn the guess into Latin-1.
    let result = write_and_read(
        "Gr\u{C3}\u{B6}\u{C3}\u{9F}e",
        &EncodeHints::default(),
        &character_set_hint("SJIS"),
    );
    assert_eq!("]Q1", symbology_identifier(&result));
    assert_eq!("Größe", result.getText());
}
