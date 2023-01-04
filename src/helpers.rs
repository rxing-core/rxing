use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    common::HybridBinarizer,
    multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader},
    BarcodeFormat, BinaryBitmap, BufferedImageLuminanceSource, DecodeHintType, DecodeHintValue,
    DecodingHintDictionary, Exceptions, MultiFormatReader, RXingResult, Reader,
};

pub fn detect_in_file(
    file_name: &str,
    barcode_type: Option<BarcodeFormat>,
) -> Result<RXingResult, Exceptions> {
    detect_in_file_with_hints(file_name, barcode_type, &mut HashMap::new())
}

pub fn detect_in_file_with_hints(
    file_name: &str,
    barcode_type: Option<BarcodeFormat>,
    hints: &mut DecodingHintDictionary,
) -> Result<RXingResult, Exceptions> {
    let img = image::open(file_name).unwrap();
    let mut multi_format_reader = MultiFormatReader::default();

    if let Some(bc_type) = barcode_type {
        hints.insert(
            DecodeHintType::POSSIBLE_FORMATS,
            DecodeHintValue::PossibleFormats(HashSet::from([bc_type])),
        );
    }

    if !hints.contains_key(&DecodeHintType::TRY_HARDER) {
        hints.insert(DecodeHintType::TRY_HARDER, DecodeHintValue::TryHarder(true));
    }

    multi_format_reader.decode_with_hints(
        &mut BinaryBitmap::new(Rc::new(RefCell::new(HybridBinarizer::new(Box::new(
            BufferedImageLuminanceSource::new(img),
        ))))),
        &hints,
    )
}

pub fn detect_multiple_in_file(file_name: &str) -> Result<Vec<RXingResult>, Exceptions> {
    detect_multiple_in_file_with_hints(file_name, &mut HashMap::new())
}

pub fn detect_multiple_in_file_with_hints(
    file_name: &str,
    hints: &mut DecodingHintDictionary,
) -> Result<Vec<RXingResult>, Exceptions> {
    let img = image::open(file_name).unwrap();
    let multi_format_reader = MultiFormatReader::default();
    let mut scanner = GenericMultipleBarcodeReader::new(multi_format_reader);

    if !hints.contains_key(&DecodeHintType::TRY_HARDER) {
        hints.insert(DecodeHintType::TRY_HARDER, DecodeHintValue::TryHarder(true));
    }

    scanner.decode_multiple_with_hints(
        &mut BinaryBitmap::new(Rc::new(RefCell::new(HybridBinarizer::new(Box::new(
            BufferedImageLuminanceSource::new(img),
        ))))),
        &hints,
    )
}
