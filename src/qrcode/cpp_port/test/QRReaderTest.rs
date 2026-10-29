// SPDX-License-Identifier: Apache-2.0

use crate::{
    BinaryBitmap, Luma8LuminanceSource, RXingResult, RXingResultMetadataType,
    RXingResultMetadataValue, Reader,
    common::{BitMatrix, HybridBinarizer},
    qrcode::cpp_port::QrReader,
};

/// Draws the symbol with 4 pixels per module and a 4-module quiet zone, and reads it
/// with QrReader and default hints, as MultiFormatReader does.
fn decode(code_str: &str) -> RXingResult {
    const SCALE: u32 = 4;
    const QUIET: u32 = 4;
    let matrix = BitMatrix::parse_strings(code_str, "X", " ").unwrap();
    let width = (matrix.getWidth() + 2 * QUIET) * SCALE;
    let height = (matrix.getHeight() + 2 * QUIET) * SCALE;
    let mut pixels = vec![255; (width * height) as usize];
    for y in 0..height {
        for x in 0..width {
            let (mx, my) = (x / SCALE, y / SCALE);
            if (QUIET..QUIET + matrix.getWidth()).contains(&mx)
                && (QUIET..QUIET + matrix.getHeight()).contains(&my)
                && matrix.get(mx - QUIET, my - QUIET)
            {
                pixels[(y * width + x) as usize] = 0;
            }
        }
    }
    let source = Luma8LuminanceSource::new(pixels, width, height).unwrap();
    QrReader
        .decode(&mut BinaryBitmap::new(HybridBinarizer::new(source)))
        .unwrap()
}

#[test]
fn StructuredAppendMetadata() {
    // zint -b QRCODE --structapp=2,3,77 -d ABC: symbol 2 of 3, parity data 77
    const CODE_STR: &str = r"XXXXXXX X XXX XXXXXXX
X     X XXX X X     X
X XXX X XX    X XXX X
X XXX X  X  X X XXX X
X XXX X  X XX X XXX X
X     X XXX   X     X
XXXXXXX X X X XXXXXXX
        X  X         
  XXX X XXXXXXXX  XXX
 XXXXX XX X  X    X X
     XXXX  X  XX  X  
X  X    XX XX X   XX 
X XX XXXX  X   X XXXX
        X XXX  X     
XXXXXXX  XXX  X XXXX 
X     X  X XX  XXX  X
X XXX X X  X X  X X  
X XXX X XXX   XX X X 
X XXX X XXX XX X XX  
X     X  X X X X  XXX
XXXXXXX  XX X  X X   
";
    let result = decode(CODE_STR);
    assert_eq!(result.getText(), "ABC");

    let metadata = result.getRXingResultMetadata();
    // Symbol sequence indicator: position - 1 = 1, then total - 1 = 2 (ISO/IEC 18004:2015 8.2)
    assert_eq!(
        metadata.get(&RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE),
        Some(&RXingResultMetadataValue::StructuredAppendSequence(
            (1 << 4) | 2
        ))
    );
    assert_eq!(
        metadata.get(&RXingResultMetadataType::STRUCTURED_APPEND_PARITY),
        Some(&RXingResultMetadataValue::StructuredAppendParity(77))
    );
}

#[test]
fn NoStructuredAppendMetadata() {
    // zint -b QRCODE -d ABC
    const QR_STR: &str = r"XXXXXXX X X X XXXXXXX
X     X X  X  X     X
X XXX X  XX X X XXX X
X XXX X X     X XXX X
X XXX X X XXX X XXX X
X     X XXXXX X     X
XXXXXXX X X X XXXXXXX
           X         
   X  X  XXX   XXX XX
XXX XX X   X XX  X   
 XX  XX     X X  X   
 XX    X XXXX      XX
  X X XXXX X    X XX 
        X    XXXXX X 
XXXXXXX  X X   X XX X
X     X     X  XXX   
X XXX X   X X  XX XXX
X XXX X X   XX   XXXX
X XXX X  X  XX  X   X
X     X  X    X X X X
XXXXXXX  X XXXX XXX  
";
    // zint -b MICROQR -d 12345 (Micro QR has no Structured Append, ISO/IEC 18004:2015 8.1)
    const MQR_STR: &str = r"XXXXXXX X X
X     X XX 
X XXX X X  
X XXX X    
X XXX X XXX
X     X  XX
XXXXXXX X  
         XX
XX  XXX  XX
 X X   XX  
XXXX     XX
";
    for (code_str, text) in [(QR_STR, "ABC"), (MQR_STR, "12345")] {
        let result = decode(code_str);
        assert_eq!(result.getText(), text);

        let metadata = result.getRXingResultMetadata();
        assert_eq!(
            metadata.get(&RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE),
            None
        );
        assert_eq!(
            metadata.get(&RXingResultMetadataType::STRUCTURED_APPEND_PARITY),
            None
        );
    }
}
