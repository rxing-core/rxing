# rxing - cRustacean Crossing

This is a port of the ZXing (https://github.com/zxing/zxing) java barcode library to pure rust. Conversion was done by hand. Original license resides with the authors of zxing.

Porting of the testing library is incomplete.

Porting was done with the rust language in mind, though some parts may resemble java more directly than a proper clean-sheet rust implementation.

## Statis
All barcode formats are tested and functioning in their current state against current tests.

## Incomplete
The library has only been thurougly tested with the `BufferedImageLuminanceSource` source format. Using any other
source is currently experimental and may result in unexpected or undefined outputs. This means that the feature flag
used to enable the use of the `image` crate is currently on by default. Turning it off may result in unexpected results.

## Example

```
use std::{collections::HashMap, rc::Rc};

use rxing::multi::MultipleBarcodeReader;

use image;

use rxing;

fn main() {
    let file_name = "test_image.jpeg";

    let img = image::open(file_name).unwrap();

    let multi_format_reader = rxing::MultiFormatReader::default();
    let mut scanner = rxing::multi::GenericMultipleBarcodeReader::new(multi_format_reader);
    let results = scanner.decode_multiple_with_hints(
        &rxing::BinaryBitmap::new(Rc::new(rxing::common::HybridBinarizer::new(Box::new(
            rxing::BufferedImageLuminanceSource::new(img),
        )))),
        &HashMap::new(),
    ).expect("decodes");

    for result in results {
        println!("{} -> {}",result.getBarcodeFormat(), result.getText())
    }
}
```