# rxing - cRustacean Crossing

[![Crate](https://img.shields.io/crates/v/rxing.svg)](https://crates.io/crates/rxing)
[![Documentation](https://docs.rs/rxing/badge.svg)](https://docs.rs/rxing)
[![License](https://img.shields.io/crates/l/rxing.svg)](https://github.com/rxing-core/rxing#copyright-notes)

This is a port of the [ZXing](https://github.com/zxing/zxing) Java barcode library to pure Rust, converted by hand. ZXing is licensed under the Apache License 2.0; copyright remains with the original ZXing authors.

Additional features were ported from [zxing-cpp](https://github.com/zxing-cpp/zxing-cpp), specifically enhancements to the QR Code, Datamatrix, and DX Film reader components. zxing-cpp is also licensed under the Apache License 2.0; copyright remains with the zxing-cpp contributors.

Porting of the testing library is incomplete. Currently all positive tests are implemented. Negative verification tests are not implemented.

Porting was done with the rust language in mind, though some parts may resemble java more directly than a proper clean-sheet rust implementation. The process of "rustifying" the code is ongoing.

## Installation

Add `rxing` to your `Cargo.toml`:

```toml
[dependencies]
rxing = "0.9.2"
```

Or run:

```bash
cargo add rxing
```

## CLI
If you're looking for a CLI interface into the library, please see [rxing-cli](https://crates.io/crates/rxing-cli).

## Online
An online demo is available at [scan.rxing.org](https://scan.rxing.org).

## WASM
If you're looking for a WASM version of this library, check out [rxing-wasm](https://github.com/rxing-core/rxing-wasm), or on [NPM](https://www.npmjs.com/package/rxing-wasm).

## Minimum Rust Version
Currently building with a minimum rust version of 1.85. Versions below that are not tested and may not compile or run as expected.

## Status
All barcode formats are tested and functioning in their current state against current tests.

| Symbology | Status | Encode | Decode |
| --- | --- | --- | --- |
| aztec | complete | yes | yes |
| datamatrix | complete | yes | yes |
| maxicode | complete | no | yes |
| pdf417 | complete | yes | yes |
| qrcode | complete | yes | yes |
| coda | complete | yes | yes |
| code 39 | complete | yes | yes |
| code 93 | complete | yes | yes |
| code 128 | complete | yes | yes |
| itf | complete | yes | yes |
| ean 8 | complete | yes | yes |
| ean 13 | complete | yes | yes |
| upc a | complete | yes | yes |
| upc e | complete | yes | yes |
| rss-14 | complete | no | yes |
| rss-expanded | complete | no | yes|
| telepen | complete | yes | yes |
| micro qr | complete | no | yes |
| rMQR | complete | no | yes |
| dxFilm | complete | no | yes |

Please note that currently UPC/EAN Extension 2/5 is supported.

## Feature Flags
The following feature flags are available:

### Core & Engine
* `image` (default): Enable features required for image manipulation and reading.
* `image_formats` (default): Enabled by default. Compile all `image` crate image format support options.
* `encoders` (default): Enable barcode encoders.
* `decoders` (default): Enable barcode decoders.
* `multi_barcode_readers` (default): Enable support for reading multiple barcodes in a single image.
* `client_support` (default): Enable the client library. Used for parsing barcode result types (e.g. URLs, contacts, wifi).
* `serde` (default): Adds support for `serde::Serialize` and `serde::Deserialize` for outward facing structs.
* `encoding_rs` (default): Enabled by default. Uses the modern `encoding_rs` crate for high-performance, WHATWG-compliant character encoding support.

### Symbology Granular Support
* `full_barcode_format_support` (default): Enables support for all barcode format modules listed below.
* `aztec`: Enable support for Aztec barcodes.
* `datamatrix`: Enable support for Data Matrix barcodes.
* `maxicode`: Enable support for MaxiCode barcodes.
* `oned`: Enable support for 1D barcodes.
* `pdf417`: Enable support for PDF417 barcodes.
* `qrcode`: Enable support for QR Code barcodes.

### Advanced & Optional
* `allow_forced_iso_ied_18004_compliance`: Allows the ability to force ISO/IED 18004 compliance. Leave disabled unless specifically needed.
* `legacy_encoding`: Provides original encoding behavior using the legacy `encoding` crate. Use this if you require exact compatibility with older versions or specific non-standard character mappings.
* `no_character_set_support`: Disable all CharacterSet support.
* `otsu_level`: Adds the Otsu level binarizer (`OtsuLevelBinarizer`). Not well tested and does *not* pass the current test suite.
* `reverse_pyramid_layers`: For `FilteredImageReader`, reverses the order of pyramid scans.
* `svg_read`: Enable support for reading SVG files.
* `svg_write`: Enable support for writing SVG files.
* `wasm_support`: Make certain changes to support building this module in WASM.
* `experimental_features`: Enable experimental features (risky).

The default feature set is:
```toml
default = [
    "image",
    "client_support",
    "image_formats",
    "serde",
    "encoding_rs",
    "encoders",
    "decoders",
    "full_barcode_format_support",
    "multi_barcode_readers"
]
```

## Incomplete
The library has only been thoroughly tested with the `BufferedImageLuminanceSource` source format. Using any other
source is currently experimental and may result in unexpected or undefined outputs. This means that the feature flag
used to enable the use of the `image` crate is currently on by default. The `Luma8LuminanceSource` is the second best
tested library, and is the underpinning for the wasm based wrapper for the library. Consider `Luma8LuminanceSource` as
a reasonable option if building the crate with the `image` feature turned off is desired.

## Example with helpers

```rust
fn main() {
    let file_name = "test_image.jpg";

    let results = rxing::helpers::detect_multiple_in_file(file_name).expect("decodes");

    for result in results {
        println!("{} -> {}", result.getBarcodeFormat(), result.getText())
    }
}
```

## Latest Release Notes
* *v0.9.2* -> Dependency updates and edition fixes.
* *v0.9.1* -> Maintenance release with performance and stability improvements.
* *v0.9.0* -> Refactor crate features. This is a **breaking change**.

    This version allows building the crate with much more granularity, only including features, symbologies, and 
    capabilities necessary for the task. Consumers who do not use `default-features = false` will likely not need
    to make any changes to their build or configuration.

    The new default features list for 0.9.0 is:
    ```toml
    default = [
        "image",
        "client_support",
        "image_formats",
        "serde",
        "encoding_rs",
        "encoders",
        "decoders",
        "full_barcode_format_support",
        "multi_barcode_readers"
    ]
    ```

    For instance, a program that needs only the ability to decode qr_codes could select the following features:
    `image, image_formats, encoding_rs, decoders, qrcode`. Similarly, a use case where encoding Aztec codes was
    all that was required would likely want to use: `image, image_formats, encoding_rs, encoders, aztec`. This
    change has been tested against the full test suite, which has required some modification of the test suite.
    
    Please file issues if any problems are detected.

* *v0.8.5* -> DX Film Edge Support added (decode only). Also adds several enhancements to performance and memory.

    The default character encoding backend has changed to the more modern `encoding_rs`. This should bring some slight
    performance improvements and better tracking of modern fixes and new encodings. All tests currently pass without 
    issue using the new backend, but if you encounter any issues please change back to the legacy backend using the
    `legacy_encoding` feature flag.

* *v0.6.1* -> Initial support for immutable symbol readers. Fixed an issue with the rss_expanded reader.

    Immutable readers: Many 2d readers now implement the `ImmutableReader` trait. This allows them to be called using the
    `immutable_decode` and `immutable_decode_with_hints` methods. The corresponding reader need not be declared `mut` in
    order to operate correctly. Please note that not all readers support this trait. Most notably: `MultiFormatReader`,
    `MultiUseMultiFormatReader`, `FilteredImageReader` and `MultiFormatOneDReader` do not implement `ImmutableReader`.
    This is because these readers all require some state be stored. There is ongoing work to reduce this list.
    This change also allows individual symbol readers to work in a `Lazy static` context without `unsafe`.

    Example:
    ```rust
    static LAZY_STATIC_QR_READER: Lazy<QRCodeReader> = Lazy::new(QRCodeReader::default);
    
    fn main() {
        let result = LAZY_STATIC_QR_READER.immutable_decode(
            &mut BinaryBitmap::new(
                HybridBinarizer::new(
                    Luma8LuminanceSource::new(luma_data, width, height),
        )));
    }
    ```

* *v0.7.0* -> Migration away from the previous HashMap based Encode/Decode hints method. The new method uses a configuration struct. You can construct these structs support `From` and `Into` the old HashMap implementation.
* *v0.6.0* -> rxing is now thread safe. This is a breaking change if you are using `PointCallback`/`RXingResultPointCallback` or the `Pdf417ExtraMetadata` field of `RXingResultMetadataValue`. In addition there should be some small performance improvements associated with moving away from using `Rc` and `Arc` in many situations throughout the library.
* *v0.5.8* -> Performance improvements. Memory Improvements. Added FilteredReader which performs a more complicated operation on images (resizes and closes binary bitmaps) at the expense of some performance.
* *v0.5.5* -> Add support for rMQR, allows building the library without image_formats, fixes an issue with multiple barcode detection.

    New default feature flag `image_formats` enables all of the `image` crates image formats for use.
    rMQR support is basic and is most effective on pure-barcodes.
    The previous version of the `GenericMultipleBarcodeReader` used the contents of the barcode as the determination of uniqueness.
    This was incorrect and the new version attempts to eliminate duplicates by detecting if they are within one another.

* *v0.5.0* -> Added support for [telepen](https://advanova.co.uk/wp-content/uploads/2022/05/Barcode-Symbology-information-and-History.pdf) thanks to the work of first time contributor [cpwood](https://github.com/cpwood).

    This release also adds the ability to exclude building the "client" result parsing features. Currently part of the default
    feature set, they can be disabled through the `client_support` feature.
    This release fixes several build issues associated with the `chrono` crate and some deprecated function messages. This change
    only impacts users building with the `client_support` feature.

* *v0.4.6* -> Fixed an issue with pdf417 whitespace, rotation, and compaction. Fix courtesy of first time contribution from GitHub user agkyunromb.
* *v0.4.4* -> Major update of QRCode support.

    The ZXing-Cpp QRCode library has been integrated. This brings large enhancements to the detection and decoding
    of QRCodes. This also brings the ability to detect and decode MicroQRCodes. This release also brings updates to the
    default Binarizer which should be slightly faster and more reliable. 
    
* *v0.4.0* -> Rewrite of the API to implement generics. This largely eliminates dynamic dispatch from the library.

    This release has many under-the-hood changes: better Point class, better Error handling, improved API 
    ergonomics with dynamics. For an understanding of how the new API works check out the `helper` functions.
    This release was made possible with PRs from Asha20 and SteveCookTU. A big thanks to them. This release does
    not have the improved QRCode support from the ZXing-CPP library, as that port is still in progress.

* *v0.3.1* -> Support for closures in NEEDS_RESULT_CALLBACK. Numerous code cleanups were performed between *v0.3.0* and *v.0.3.1* rxing has moved to https://github.com/rxing-core/rxing.
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
* Performance is slow for GenericMultipleBarcodeReader.

## Generative AI Policy
Some generative AI has been used since v0.9.1. The uses are primarily limited to code-review and PR review. The library does not feature "vibe code" and all changes suggested by AI are either fully implemented by a human or are driven by human-authored specifications and test-driven-development plans. All code is fully reviewed by a human. 

## ZXing Track
Currently tracking zxing 3.5.1

## Copyright Notes
rxing is licensed under the Apache License 2.0.

The ZXing library is licensed under the Apache License 2.0; copyright remains with the ZXing authors. Portions of this crate are ported from zxing-cpp, which is also licensed under the Apache License 2.0; copyright remains with the zxing-cpp contributors.
