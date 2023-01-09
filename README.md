# rxing - cRustacean Crossing

This is a port of the ZXing (https://github.com/zxing/zxing) java barcode library to pure rust. Conversion was done by hand. Original license resides with the authors of zxing.

Porting of the testing library is incomplete.

Porting was done with the rust language in mind, though some parts may resemble java more directly than a proper clean-sheet rust implementation.

## CLI
If you're looking for a CLI interface into the library, please see [rxing-cli](https://crates.io/crates/rxing-cli).

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

## Example with helpers

```
use rxing;

fn main() {
    let file_name = "test_image.jpg";

    let results = rxing::helpers::detect_multiple_in_file(file_name).expect("decodes");

    for result in results {
        println!("{} -> {}", result.getBarcodeFormat(), result.getText())
    }
}
```

## Latest Release Notes
v0.2.10 -> Fix major issue with qrcode generation.

v0.2.9 -> Major fix, codabar was not being encoded by multiformat writer.

v0.2.6 -> Fix missing result point callback for rss14

v0.2.4 -> Add helper functions for common cases (read a file, use raw luma8 data).

v0.2.3 -> Implement most suggestions from clippy, as well as some simple changes, no surface changes.

v0.2.0 -> Dramatically improve performance when cropping a BufferedImageLuminanceSource.

v0.1.4 -> Dramatically improve performance for MultiFormatReader and for multiple barcode detection.

## Known Issues
Performance is low for GenericMultipleBarcodeReader.

## ZXing Track
Currently tracking zxing 3.5.1
