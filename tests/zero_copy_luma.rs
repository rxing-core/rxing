#![cfg(all(feature = "encoders", feature = "decoders", feature = "qrcode"))]

use rxing::{BarcodeFormat, MultiFormatWriter, Writer, helpers::detect_in_luma_slice};

/// End-to-end: encode a QR code, render it to a luma buffer, and decode it
/// through the zero-copy slice helper without handing over ownership.
#[test]
fn detect_in_luma_slice_decodes_borrowed_buffer() {
    let content = "zero copy roundtrip";
    let bits = MultiFormatWriter
        .encode(content, &BarcodeFormat::QR_CODE, 256, 256)
        .expect("encode succeeds");
    let (w, h) = (bits.width(), bits.height());
    let mut luma = Vec::with_capacity((w * h) as usize);
    for y in 0..h {
        for x in 0..w {
            luma.push(if bits.get(x, y) { 0u8 } else { 255u8 });
        }
    }

    let result =
        detect_in_luma_slice(&luma, w, h, Some(BarcodeFormat::QR_CODE)).expect("decode succeeds");
    assert_eq!(result.getText(), content);

    // the caller still owns the buffer afterwards
    assert_eq!(luma.len(), (w * h) as usize);
}
