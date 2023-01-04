# rxing - cRustacean Crossing

This is a port of the ZXing (https://github.com/zxing/zxing) java barcode library to pure rust. Conversion was done by hand. Original license resides with the authors of zxing.

Porting of the testing library is incomplete.

Porting was done with the rust language in mind, though some parts may resemble java more directly than a proper clean-sheet rust implementation.

## Status
All barcode formats are tested and functioning in their current state against current tests.

| Symbology | Status |
| --- | --- |
| aztec | complete |
| datamatrix | complete |
| maxicode | complete |
| pdf417 | complete |
| qrcode | complete |
| coda | complete |
| code 39 | complete |
| code 93 | complete |
| code 128 | complete |
| itf | complete |
| ean 8 | complete |
| ean 13 | complete |
| upc a | complete |
| upc e | complete |

## Incomplete
The library has only been thurougly tested with the `BufferedImageLuminanceSource` source format. Using any other
source is currently experimental and may result in unexpected or undefined outputs. This means that the feature flag
used to enable the use of the `image` crate is currently on by default. Turning it off may result in unexpected results.

## Example

```
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use rxing::multi::MultipleBarcodeReader;

use image;

use rxing;

fn main() {
    let file_name = "test_image.jpeg";

    let img = image::open(file_name).unwrap();
    // let img = img.resize(768, 1024, image::imageops::FilterType::Gaussian);

    let mut hints: rxing::DecodingHintDictionary = HashMap::new();
    hints.insert(
        rxing::DecodeHintType::TRY_HARDER,
        rxing::DecodeHintValue::TryHarder(true),
    );

    let multi_format_reader = rxing::MultiFormatReader::default();
    let mut scanner = rxing::multi::GenericMultipleBarcodeReader::new(multi_format_reader);
    let results = scanner
        .decode_multiple_with_hints(
            &mut rxing::BinaryBitmap::new(Rc::new(RefCell::new(
                rxing::common::HybridBinarizer::new(Box::new(
                    rxing::BufferedImageLuminanceSource::new(img),
                )),
            ))),
            &hints,
        )
        .expect("decodes");

    for result in results {
        println!("{} -> {}", result.getBarcodeFormat(), result.getText())
    }
}

```

## Latest Release Notes
v0.2.3 -> Implement most suggestions from clippy, as well as some simple changes, no surface changes.

v0.2.0 -> Dramatically improve performance when cropping a BufferedImageLuminanceSource.

v0.1.4 -> Dramatically improve performance for MultiFormatReader and for multiple barcode detection.

## Known Issues
Performance is low for GenericMultipleBarcodeReader.