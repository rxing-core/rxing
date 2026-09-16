#![cfg(all(
    feature = "image",
    feature = "datamatrix",
    feature = "encoders",
    feature = "decoders"
))]

use image::{DynamicImage, GrayImage, Luma};
use rxing::{
    BarcodeFormat, BinaryBitmap, BufferedImageLuminanceSource, EncodeHints, RXingResult,
    RXingResultMetadataType, RXingResultMetadataValue, Reader, Writer,
    common::{BitMatrix, HybridBinarizer},
    datamatrix::{DataMatrixReader, DataMatrixWriter},
};

/// GS1 separator, encoded as a FNC1 codeword when the GS1 format hint is given
const GS: char = '\u{1D}';

/// A GS1 message with two application identifiers separated by a GS character
const CONTENTS: &str = "0103453120000011\u{1D}2112345";

/// A requested size of 0x0 gives one pixel per module without a quiet zone, which no detector can
/// find. Render the modules larger and surround them with a quiet zone instead.
fn to_detectable_image(matrix: &BitMatrix) -> DynamicImage {
    const SCALE: u32 = 4;
    const QUIET_ZONE_MODULES: u32 = 4;

    let offset = QUIET_ZONE_MODULES * SCALE;
    let mut image = GrayImage::from_pixel(
        (matrix.getWidth() * SCALE) + (offset * 2),
        (matrix.getHeight() * SCALE) + (offset * 2),
        Luma([255]),
    );

    for y in 0..matrix.getHeight() {
        for x in 0..matrix.getWidth() {
            if matrix.get(x, y) {
                for dy in 0..SCALE {
                    for dx in 0..SCALE {
                        image.put_pixel(
                            offset + (x * SCALE) + dx,
                            offset + (y * SCALE) + dy,
                            Luma([0]),
                        );
                    }
                }
            }
        }
    }

    DynamicImage::ImageLuma8(image)
}

fn encode_and_decode(contents: &str, hints: &EncodeHints) -> RXingResult {
    let writer = DataMatrixWriter;
    let bitmatrix = writer
        .encode_with_hints(contents, &BarcodeFormat::DATA_MATRIX, 0, 0, hints)
        .expect("failed to encode Data Matrix");

    let source = BufferedImageLuminanceSource::new(to_detectable_image(&bitmatrix));
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));

    DataMatrixReader
        .decode(&mut bitmap)
        .expect("failed to decode Data Matrix")
}

fn content_type(result: &RXingResult) -> Option<&str> {
    match result
        .getRXingResultMetadata()
        .get(&RXingResultMetadataType::CONTENT_TYPE)
    {
        Some(RXingResultMetadataValue::ContentType(content_type)) => Some(content_type),
        _ => None,
    }
}

fn symbology_identifier(result: &RXingResult) -> Option<&str> {
    match result
        .getRXingResultMetadata()
        .get(&RXingResultMetadataType::SYMBOLOGY_IDENTIFIER)
    {
        Some(RXingResultMetadataValue::SymbologyIdentifier(identifier)) => Some(identifier),
        _ => None,
    }
}

/// The GS1 format hint is only read by the minimal encoder, which is selected with the compaction
/// hint, so the two have to be given together.
fn gs1_hints() -> EncodeHints {
    EncodeHints {
        Gs1Format: Some(true),
        DataMatrixCompact: Some(true),
        ..Default::default()
    }
}

/// The GS1 format hint prepends a FNC1 codeword and turns every GS separator into a FNC1
/// codeword, which the decoder reports as GS1 content.
#[test]
fn gs1_format_hint_round_trip() {
    let result = encode_and_decode(CONTENTS, &gs1_hints());

    assert_eq!(result.getText(), CONTENTS);
    assert_eq!(content_type(&result), Some("GS1"));
    assert_eq!(symbology_identifier(&result), Some("]d2"));
}

/// Without the compaction hint the GS1 format hint is ignored: the content still round trips but
/// the symbol is not GS1.
#[test]
fn gs1_format_hint_without_compaction_hint_is_ignored() {
    let hints = EncodeHints {
        Gs1Format: Some(true),
        ..Default::default()
    };

    let result = encode_and_decode(CONTENTS, &hints);

    assert_eq!(result.getText(), CONTENTS);
    assert_eq!(content_type(&result), None);
    assert_eq!(symbology_identifier(&result), Some("]d1"));
}

/// Without the GS1 format hint the separator is a plain ASCII character and the symbol is not GS1.
#[test]
fn without_gs1_format_hint_round_trip() {
    for hints in [
        EncodeHints::default(),
        EncodeHints {
            DataMatrixCompact: Some(true),
            ..Default::default()
        },
    ] {
        let result = encode_and_decode(CONTENTS, &hints);

        assert_eq!(result.getText(), CONTENTS);
        assert_eq!(content_type(&result), None);
        assert_eq!(symbology_identifier(&result), Some("]d1"));
    }
}

/// Content without any separator, the leading FNC1 codeword alone marks the symbol as GS1.
#[test]
fn gs1_format_hint_without_separator_round_trip() {
    let contents = "01034531200000111719112510ABCD1234";

    let result = encode_and_decode(contents, &gs1_hints());

    assert_eq!(result.getText(), contents);
    assert_eq!(content_type(&result), Some("GS1"));
}

/// Every separator has to come back as a GS character, only the leading FNC1 is consumed.
#[test]
fn gs1_format_hint_keeps_separators() {
    let contents = format!("11{GS}22{GS}33");

    let result = encode_and_decode(&contents, &gs1_hints());

    assert_eq!(result.getText(), contents);
    assert_eq!(result.getText().matches(GS).count(), 2);
    assert_eq!(content_type(&result), Some("GS1"));
}

/// Regression test: this message is 18 codewords without the leading FNC1 codeword, exactly the
/// capacity of an 18x18 symbol. The FNC1 pushed it into a 20x20 symbol while the encoder had
/// already decided that the trailing C40 run needed no unlatch, so the padding of the larger
/// symbol was decoded as C40 text and appended "GR2u" to the message.
#[test]
fn gs1_format_hint_exact_symbol_fill_round_trip() {
    let contents = "01012345678901281720010110ABC123";

    let result = encode_and_decode(contents, &gs1_hints());

    assert_eq!(result.getText(), contents);
    assert_eq!(content_type(&result), Some("GS1"));
    assert_eq!(symbology_identifier(&result), Some("]d2"));
}
