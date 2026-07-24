#![cfg(all(
    feature = "encoders",
    feature = "decoders",
    feature = "oned",
    feature = "qrcode",
    feature = "datamatrix"
))]

use std::collections::HashSet;

use rxing::common::HybridBinarizer;
use rxing::{
    BarcodeFormat, BinaryBitmap, DecodeHints, Luma8LuminanceSource, MultiFormatReader,
    MultiFormatWriter, Reader, Writer,
};

#[test]
fn close_applies_morphological_closing() {
    let source = Luma8LuminanceSource::new(vec![255u8; 32 * 32], 32, 32).expect("source");
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));
    // a hollow 3x3 ring of black pixels; closing (dilate then erode) fills it
    for (x, y) in [
        (10, 10),
        (11, 10),
        (12, 10),
        (10, 11),
        (12, 11),
        (10, 12),
        (11, 12),
        (12, 12),
    ] {
        bitmap.get_black_matrix_mut().set(x, y);
    }
    bitmap.close().expect("close succeeds");
    let expected = "                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                X X X                                           \n                X X X                                           \n                X X X                                           \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n                                                                \n";
    assert_eq!(bitmap.get_black_matrix().to_string(), expected);
}

/// Encode `content`, render to luma8 (black = 0), optionally invert every pixel.
fn encoded_luma(format: BarcodeFormat, content: &str, inverted: bool) -> (Vec<u8>, u32, u32) {
    let bits = MultiFormatWriter
        .encode(content, &format, 300, 150)
        .expect("encode succeeds");
    let (w, h) = (bits.width(), bits.height());
    let mut luma = Vec::with_capacity((w * h) as usize);
    for y in 0..h {
        for x in 0..w {
            let v: u8 = if bits.get(x, y) { 0 } else { 255 };
            luma.push(if inverted { 255 - v } else { v });
        }
    }
    (luma, w, h)
}

fn decodes(luma: Vec<u8>, w: u32, h: u32, also_inverted: bool) -> bool {
    let hints = DecodeHints {
        AlsoInverted: Some(also_inverted),
        TryHarder: Some(true),
        ..Default::default()
    };
    let source = Luma8LuminanceSource::new(luma, w, h).expect("source");
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));
    MultiFormatReader::default()
        .decode_with_hints(&mut bitmap, &hints)
        .is_ok()
}

#[test]
fn inverted_ean13_decodes_with_also_inverted_hint() {
    let (luma, w, h) = encoded_luma(BarcodeFormat::EAN_13, "5901234123457", true);
    assert!(decodes(luma, w, h, true));
}

#[test]
fn inverted_code128_decodes_with_also_inverted_hint() {
    let (luma, w, h) = encoded_luma(BarcodeFormat::CODE_128, "inversion test", true);
    assert!(decodes(luma, w, h, true));
}

#[test]
fn inverted_qr_decodes_with_also_inverted_hint() {
    let (luma, w, h) = encoded_luma(BarcodeFormat::QR_CODE, "inversion test", true);
    assert!(decodes(luma, w, h, true));
}

#[test]
fn inverted_datamatrix_decodes_with_also_inverted_hint() {
    let (luma, w, h) = encoded_luma(BarcodeFormat::DATA_MATRIX, "inversion test", true);
    assert!(decodes(luma, w, h, true));
}

#[test]
fn normal_images_decode_with_also_inverted_hint() {
    for (format, content) in [
        (BarcodeFormat::EAN_13, "5901234123457"),
        (BarcodeFormat::CODE_128, "inversion test"),
        (BarcodeFormat::QR_CODE, "inversion test"),
        (BarcodeFormat::DATA_MATRIX, "inversion test"),
    ] {
        let (luma, w, h) = encoded_luma(format, content, false);
        assert!(decodes(luma, w, h, true), "{format:?}");
    }
}

#[test]
fn inverted_image_without_hint_still_fails() {
    // inversion handling is opt-in; without the hint the decode must not succeed
    let (luma, w, h) = encoded_luma(BarcodeFormat::EAN_13, "5901234123457", true);
    assert!(!decodes(luma, w, h, false));
}

/// Transpose a WxH luma buffer, turning a horizontal barcode into a vertical
/// one (bars running top-to-bottom). Returns the (h, w) buffer.
fn transpose(luma: &[u8], w: u32, h: u32) -> (Vec<u8>, u32, u32) {
    let (w, h) = (w as usize, h as usize);
    let mut out = vec![0u8; w * h];
    for y in 0..h {
        for x in 0..w {
            // new image is h wide, w tall; new(x=y, y=x)
            out[x * h + y] = luma[y * w + x];
        }
    }
    (out, h as u32, w as u32)
}

/// Decode with `PossibleFormats` restricted to a single 1D format. This forces
/// the classic `MultiFormatOneDReader` (whose TryHarder path rotates the
/// bitmap) rather than the cpp column-scanning reader.
fn decodes_1d(luma: Vec<u8>, w: u32, h: u32, format: BarcodeFormat, also_inverted: bool) -> bool {
    let hints = DecodeHints {
        PossibleFormats: Some(HashSet::from([format])),
        AlsoInverted: Some(also_inverted),
        TryHarder: Some(true),
        ..Default::default()
    };
    let source = Luma8LuminanceSource::new(luma, w, h).expect("source");
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));
    MultiFormatReader::default()
        .decode_with_hints(&mut bitmap, &hints)
        .is_ok()
}

#[test]
fn normal_vertical_1d_decodes() {
    // sanity: the vertical (non-inverted) barcode decodes via the rotate path,
    // proving the test construction reaches the reader under test
    let (luma, w, h) = encoded_luma(BarcodeFormat::EAN_13, "5901234123457", false);
    let (luma, w, h) = transpose(&luma, w, h);
    assert!(decodes_1d(luma, w, h, BarcodeFormat::EAN_13, true));
}

#[test]
fn inverted_vertical_1d_decodes_with_also_inverted_hint() {
    // an inverted, vertically-oriented 1D barcode restricted to a 1D format
    // exercises the classic reader's rotate path, which must carry the
    // inverted flag into the derived (rotated) bitmap
    let (luma, w, h) = encoded_luma(BarcodeFormat::EAN_13, "5901234123457", true);
    let (luma, w, h) = transpose(&luma, w, h);
    assert!(decodes_1d(luma, w, h, BarcodeFormat::EAN_13, true));
}

#[test]
fn inverted_get_black_row_out_of_bounds_errors() {
    // While inverted, an out-of-range row must return Err (matching the
    // binarizer path's contract), not panic from indexing the flipped matrix.
    let source = Luma8LuminanceSource::new(vec![255u8; 16 * 16], 16, 16).expect("source");
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));
    bitmap.invert();
    assert!(bitmap.get_black_row(9999).is_err());
    assert!(
        bitmap
            .get_black_line(9999, rxing::common::LineOrientation::Row)
            .is_err()
    );
}

#[test]
fn bitmap_state_restored_after_failed_retry() {
    // deterministic noise that fails both the normal and the inverted attempt
    let luma: Vec<u8> = (0..64 * 64)
        .map(|i| ((i as u64).wrapping_mul(2654435761) >> 8) as u8)
        .collect();
    let source = Luma8LuminanceSource::new(luma, 64, 64).expect("source");
    let mut bitmap = BinaryBitmap::new(HybridBinarizer::new(source));
    let before = bitmap.get_black_matrix().clone();

    let hints = DecodeHints {
        AlsoInverted: Some(true),
        ..Default::default()
    };
    let _ = MultiFormatReader::default().decode_with_hints(&mut bitmap, &hints);

    assert_eq!(
        *bitmap.get_black_matrix(),
        before,
        "a failed AlsoInverted retry must leave the caller's bitmap unflipped"
    );
}
