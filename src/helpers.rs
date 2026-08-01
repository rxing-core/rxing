use std::{collections::HashSet, io::Write, path::PathBuf};

#[cfg(feature = "image")]
use image::DynamicImage;

use crate::{
    BarcodeFormat, Error,
    common::{BitMatrix, Result},
};

#[cfg(feature = "decoders")]
use crate::{
    BinaryBitmap, DecodeHints, Luma8LuminanceSource, RXingResult, Reader, common::HybridBinarizer,
};

#[cfg(feature = "decoders")]
use crate::{MultiFormatReader, MultiUseMultiFormatReader};

#[cfg(feature = "decoders")]
use crate::FilteredImageReader;

#[cfg(all(feature = "multi_barcode_readers", feature = "decoders"))]
use crate::multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader};

#[cfg(all(feature = "image", feature = "decoders"))]
use crate::BufferedImageLuminanceSource;

#[cfg(all(feature = "svg_read", feature = "decoders"))]
pub fn detect_in_svg(file_name: &str, barcode_type: Option<BarcodeFormat>) -> Result<RXingResult> {
    detect_in_svg_with_hints(file_name, barcode_type, &mut DecodeHints::default())
}

#[cfg(all(feature = "svg_read", feature = "decoders"))]
pub fn detect_in_svg_with_hints(
    file_name: &str,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    use std::{fs::File, io::Read};

    use crate::SVGLuminanceSource;

    let path = PathBuf::from(file_name);
    if !path.exists() {
        return Err(Error::illegal_argument_with("file does not exist"));
    }

    let mut file = File::open(path)?;

    let mut svg_data = Vec::new();
    file.read_to_end(&mut svg_data)?;

    let mut multi_format_reader = MultiFormatReader::default();

    if let Some(bc_type) = barcode_type {
        hints.PossibleFormats = Some(HashSet::from([bc_type]));
    }

    hints.TryHarder = hints.TryHarder.or(Some(true));

    multi_format_reader.decode_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(SVGLuminanceSource::new(&svg_data)?)),
        hints,
    )
}

#[cfg(all(
    feature = "svg_read",
    feature = "multi_barcode_readers",
    feature = "decoders"
))]
pub fn detect_multiple_in_svg(file_name: &str) -> Result<Vec<RXingResult>> {
    detect_multiple_in_svg_with_hints(file_name, &mut DecodeHints::default())
}

#[cfg(all(
    feature = "svg_read",
    feature = "multi_barcode_readers",
    feature = "decoders"
))]
pub fn detect_multiple_in_svg_with_hints(
    file_name: &str,
    hints: &mut DecodeHints,
) -> Result<Vec<RXingResult>> {
    use std::{fs::File, io::Read};

    use crate::SVGLuminanceSource;

    let path = PathBuf::from(file_name);
    if !path.exists() {
        return Err(Error::illegal_argument_with("file does not exist"));
    }

    let mut file = File::open(path)?;

    let mut svg_data = Vec::new();
    file.read_to_end(&mut svg_data)?;

    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints.TryHarder = hints.TryHarder.or(Some(true));

    scanner.decode_multiple_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(SVGLuminanceSource::new(&svg_data)?)),
        hints,
    )
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_file(file_name: &str, barcode_type: Option<BarcodeFormat>) -> Result<RXingResult> {
    detect_in_file_with_hints(file_name, barcode_type, &mut DecodeHints::default())
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_file_with_hints(
    file_name: &str,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    let img = image::open(file_name)?;
    
    detect_in_image_with_hints(img, barcode_type, hints)
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_buffer(buffer: &[u8], barcode_type: Option<BarcodeFormat>) -> Result<RXingResult> {
    detect_in_buffer_with_hints(buffer, barcode_type, &mut DecodeHints::default())
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_buffer_with_hints(
    buffer: &[u8],
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    let img = image::load_from_memory(buffer)?;
    detect_in_image_with_hints(img, barcode_type, hints)
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_image(
    img: DynamicImage,
    barcode_type: Option<BarcodeFormat>,
) -> Result<RXingResult> {
    detect_in_image_with_hints(img, barcode_type, &mut DecodeHints::default())
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_image_with_hints(
    img: DynamicImage,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    let mut multi_format_reader = MultiFormatReader::default();

    if let Some(bc_type) = barcode_type {
        hints.PossibleFormats = Some(HashSet::from([bc_type]));
    }

    hints.TryHarder = hints.TryHarder.or(Some(true));

    multi_format_reader.decode_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(img))),
        hints,
    )
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_file_filtered(
    file_name: &str,
    barcode_type: Option<BarcodeFormat>,
) -> Result<RXingResult> {
    detect_in_file_filtered_with_hints(file_name, barcode_type, &mut DecodeHints::default())
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_file_filtered_with_hints(
    file_name: &str,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    let img = image::open(file_name)?;
    detect_in_image_filtered_with_hints(img, barcode_type, hints)
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_image_filtered(
    img: DynamicImage,
    barcode_type: Option<BarcodeFormat>,
) -> Result<RXingResult> {
    detect_in_image_filtered_with_hints(img, barcode_type, &mut DecodeHints::default())
}

#[cfg(all(feature = "image", feature = "decoders"))]
pub fn detect_in_image_filtered_with_hints(
    img: DynamicImage,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    let mut multi_format_reader = FilteredImageReader::new(MultiFormatReader::default());

    if let Some(bc_type) = barcode_type {
        hints.PossibleFormats = Some(HashSet::from([bc_type]));
    }

    hints.TryHarder = hints.TryHarder.or(Some(true));

    multi_format_reader.decode_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(img))),
        hints,
    )
}

#[cfg(all(
    feature = "image",
    feature = "multi_barcode_readers",
    feature = "decoders"
))]
pub fn detect_multiple_in_file(file_name: &str) -> Result<Vec<RXingResult>> {
    detect_multiple_in_file_with_hints(file_name, &mut DecodeHints::default())
}

#[cfg(all(
    feature = "image",
    feature = "multi_barcode_readers",
    feature = "decoders"
))]
pub fn detect_multiple_in_file_with_hints(
    file_name: &str,
    hints: &mut DecodeHints,
) -> Result<Vec<RXingResult>> {
    let img = image::open(file_name)?;
    detect_multiple_in_image_with_hints(img, hints)
}

#[cfg(all(
    feature = "image",
    feature = "multi_barcode_readers",
    feature = "decoders"
))]
pub fn detect_multiple_in_buffer(buffer: &[u8]) -> Result<Vec<RXingResult>> {
    detect_multiple_in_buffer_with_hints(buffer, &mut DecodeHints::default())
}

#[cfg(all(
    feature = "image",
    feature = "multi_barcode_readers",
    feature = "decoders"
))]
pub fn detect_multiple_in_buffer_with_hints(
    buffer: &[u8],
    hints: &mut DecodeHints,
) -> Result<Vec<RXingResult>> {
    let img = image::load_from_memory(buffer)?;
    detect_multiple_in_image_with_hints(img, hints)
}

#[cfg(all(
    feature = "image",
    feature = "multi_barcode_readers",
    feature = "decoders"
))]
pub fn detect_multiple_in_image(img: DynamicImage) -> Result<Vec<RXingResult>> {
    detect_multiple_in_image_with_hints(img, &mut DecodeHints::default())
}

#[cfg(all(
    feature = "image",
    feature = "multi_barcode_readers",
    feature = "decoders"
))]
pub fn detect_multiple_in_image_with_hints(
    img: DynamicImage,
    hints: &mut DecodeHints,
) -> Result<Vec<RXingResult>> {
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints.TryHarder = hints.TryHarder.or(Some(true));

    scanner.decode_multiple_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(img))),
        hints,
    )
}

#[cfg(feature = "decoders")]
pub fn detect_in_luma(
    luma: Vec<u8>,
    width: u32,
    height: u32,
    barcode_type: Option<BarcodeFormat>,
) -> Result<RXingResult> {
    detect_in_luma_with_hints(
        luma,
        width,
        height,
        barcode_type,
        &mut DecodeHints::default(),
    )
}

#[cfg(feature = "decoders")]
pub fn detect_in_luma_with_hints(
    luma: Vec<u8>,
    width: u32,
    height: u32,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    if width == 0 || height == 0 {
        return Err(Error::illegal_argument_with(
            "Both dimensions must be greater than 0",
        ));
    }
    let mut multi_format_reader = MultiFormatReader::default();

    if let Some(bc_type) = barcode_type {
        hints.PossibleFormats = Some(HashSet::from([bc_type]));
    }

    hints.TryHarder = hints.TryHarder.or(Some(true));

    multi_format_reader.decode_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(Luma8LuminanceSource::new(
            luma, width, height,
        )?)),
        hints,
    )
}

/// Decode a barcode from a borrowed luma8 buffer without copying it.
/// Zero-copy counterpart of [`detect_in_luma`].
#[cfg(feature = "decoders")]
pub fn detect_in_luma_slice(
    luma: &[u8],
    width: u32,
    height: u32,
    barcode_type: Option<BarcodeFormat>,
) -> Result<RXingResult> {
    detect_in_luma_slice_with_hints(
        luma,
        width,
        height,
        barcode_type,
        &mut DecodeHints::default(),
    )
}

/// Decode a barcode from a borrowed luma8 buffer without copying it.
/// Zero-copy counterpart of [`detect_in_luma_with_hints`].
#[cfg(feature = "decoders")]
pub fn detect_in_luma_slice_with_hints(
    luma: &[u8],
    width: u32,
    height: u32,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    if width == 0 || height == 0 {
        return Err(Error::illegal_argument_with(
            "Both dimensions must be greater than 0",
        ));
    }
    let mut multi_format_reader = MultiFormatReader::default();

    if let Some(bc_type) = barcode_type {
        hints.PossibleFormats = Some(HashSet::from([bc_type]));
    }

    hints.TryHarder = hints.TryHarder.or(Some(true));

    multi_format_reader.decode_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(crate::Luma8Source::new_with_slice(
            luma, width, height,
        )?)),
        hints,
    )
}

#[cfg(feature = "decoders")]
pub fn detect_in_luma_filtered(
    luma: Vec<u8>,
    width: u32,
    height: u32,
    barcode_type: Option<BarcodeFormat>,
) -> Result<RXingResult> {
    crate::helpers::detect_in_luma_filtered_with_hints(
        luma,
        width,
        height,
        barcode_type,
        &mut DecodeHints::default(),
    )
}

#[cfg(feature = "decoders")]
pub fn detect_in_luma_filtered_with_hints(
    luma: Vec<u8>,
    width: u32,
    height: u32,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodeHints,
) -> Result<RXingResult> {
    if width == 0 || height == 0 {
        return Err(Error::illegal_argument_with(
            "Both dimensions must be greater than 0",
        ));
    }
    let mut multi_format_reader = FilteredImageReader::new(MultiFormatReader::default());

    if let Some(bc_type) = barcode_type {
        hints.PossibleFormats = Some(HashSet::from([bc_type]));
    }

    hints.TryHarder = hints.TryHarder.or(Some(true));

    multi_format_reader.decode_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(Luma8LuminanceSource::new(
            luma, width, height,
        )?)),
        hints,
    )
}

#[cfg(all(feature = "multi_barcode_readers", feature = "decoders"))]
pub fn detect_multiple_in_luma(luma: Vec<u8>, width: u32, height: u32) -> Result<Vec<RXingResult>> {
    detect_multiple_in_luma_with_hints(luma, width, height, &mut DecodeHints::default())
}

#[cfg(all(feature = "multi_barcode_readers", feature = "decoders"))]
pub fn detect_multiple_in_luma_with_hints(
    luma: Vec<u8>,
    width: u32,
    height: u32,
    hints: &mut DecodeHints,
) -> Result<Vec<RXingResult>> {
    if width == 0 || height == 0 {
        return Err(Error::illegal_argument_with(
            "Both dimensions must be greater than 0",
        ));
    }
    let multi_format_reader = MultiUseMultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    hints.TryHarder = hints.TryHarder.or(Some(true));

    scanner.decode_multiple_with_hints(
        &mut BinaryBitmap::new(HybridBinarizer::new(Luma8LuminanceSource::new(
            luma, width, height,
        )?)),
        hints,
    )
}

#[cfg(feature = "image")]
pub fn save_image(file_name: &str, bit_matrix: &BitMatrix) -> Result<()> {
    let image: image::DynamicImage = bit_matrix.into();
    match image.save(file_name) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

#[cfg(feature = "svg_write")]
pub fn save_svg(file_name: &str, bit_matrix: &BitMatrix) -> Result<()> {
    let svg: svg::Document = bit_matrix.into();

    match svg::save(file_name, &svg) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

pub fn save_file(file_name: &str, bit_matrix: &BitMatrix) -> Result<()> {
    let path = PathBuf::from(file_name);

    #[allow(unused_variables)]
    let ext: String = if let Some(e) = path.extension() {
        e.to_string_lossy().to_string()
    } else {
        String::default()
    };

    #[cfg(feature = "svg_write")]
    if ext == "svg" {
        return save_svg(file_name, bit_matrix);
    }

    #[cfg(feature = "image")]
    if !ext.is_empty() && ext != "txt" {
        return save_image(file_name, bit_matrix);
    }

    let result_tester = || -> std::io::Result<_> {
        let file = std::fs::File::create(path)?;
        let mut output = std::io::BufWriter::new(file);
        output.write_all(bit_matrix.to_string().as_bytes())?;
        output.flush()?;
        Ok(())
    };

    match result_tester() {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
