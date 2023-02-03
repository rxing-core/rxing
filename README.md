# rxing - cRustacean Crossing

This is a port of the ZXing (https://github.com/zxing/zxing) java barcode library to pure rust. Conversion was done by hand. Original license resides with the authors of zxing.

Porting of the testing library is incomplete. Currently all positive tests are implemented. Negative verfication tests are not implemented.

Porting was done with the rust language in mind, though some parts may resemble java more directly than a proper clean-sheet rust implementation. The process of "rustifying" the code is ongoing.

## CLI
If you're looking for a CLI interface into the library, please see [rxing-cli](https://crates.io/crates/rxing-cli).

## WASM
If you're looking for a WASM version of this library, check out [rxing-wasm](https://github.com/hschimke/rxing-wasm), or on [NPM](https://www.npmjs.com/package/rxing-wasm).


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

## feature flags
The following feature flags are available:
* `image`: Enable features required for image manipulation and reading.
* `allow_forced_iso_ied_18004_compliance`: Allows the ability to force ISO/IED 18004 compliance. Leave disabled unless specificially needed.
* `svg_write`: Enable support for writing SVG files
* `svg_read`: Enable support for reading SVG files
* `wasm_support`: Make certain changes to support building this module in WASM
* `experimental_features`: Enable experimental features, risky.
* `serde`: Adds support for serde Serialize and Deserialize for outward facing structs

The default feature set includes only the `image` feature mentioned above.

## Incomplete
The library has only been thurougly tested with the `BufferedImageLuminanceSource` source format. Using any other
source is currently experimental and may result in unexpected or undefined outputs. This means that the feature flag
used to enable the use of the `image` crate is currently on by default. Turning it off may result in unexpected results.

## Example with helpers

```rust
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
* *v0.2.21* -> Adds partial support for detecting and decoding rotated MaxiCode symbols. Adds support for basic serialization of many public facing datatypes using serde (gated behind `serde` feature).

    Rotation detection is no longer gated behind the `experimental_features` flag. Rotation of maxicodes is simplistic. Current tests detect about 50% of codes when rotated 90 degrees. Detection of skewed MaxiCodes is now behind `experimental_features`.

* *v0.2.20* -> Adds rudimentary support for MaxiCode detection. The detector works best on unrotated images on a flat plane. Very basic support for rotation correction is gated behind the `experimental_features` flag, but it is not ready for most use cases. The MaxiCode detector is gated behind the `TryHarder` decoder hint, by default rxing uses the old `PureBarcode` implementation.
* *v0.2.19* -> The datamatrix detector for the c++ version of zxing [zxing-cpp](https://github.com/zxing-cpp/zxing-cpp) has been ported. This features a dramatically different method of detecting datamatrix symbols. If you want to fallback to the original version, include the decode hint TRY_HARDER.
* *v0.2.15* -> Support for reading and writing svg files through the feature flags `svg_read` and `svg_write`.

    These flags are off by default.
    
* *v0.2.14* -> Support for more image output formats, many rustification changes to the codebase.

    If you were using very deep, specific functions in the encoder/decoder sections this may require a function rename. For instance `qrcode::encoder::encoder` is now `qrcode::encoder::qrcode_encoder`.

* *v0.2.10* -> Fix major issue with qrcode generation.
* *v0.2.9* -> Major fix, codabar was not being encoded by multiformat writer.
* *v0.2.6* -> Fix missing result point callback for rss14
* *v0.2.4* -> Add helper functions for common cases (read a file, use raw luma8 data).
* *v0.2.3* -> Implement most suggestions from clippy, as well as some simple changes, no surface changes.
* *v0.2.0* -> Dramatically improve performance when cropping a BufferedImageLuminanceSource.
* *v0.1.4* -> Dramatically improve performance for MultiFormatReader and for multiple barcode detection.

## Known Issues
* Performance is low for GenericMultipleBarcodeReader.
* Datamatrix codes are sometimes not correctly decoded, especially when they are _actually_ pure barcodes. This appears to be an issue with zxing 3.5.1 as well.

## ZXing Track
Currently tracking zxing 3.5.1

## Copyright notes
The original license remains with the zxing developers. The license for the ported components of the c++ port remain with the developers of that port, where applicable.