/*
 * Copyright 2007 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//package com.google.zxing;

use std::collections::{HashMap, HashSet};

use crate::{BarcodeFormat, PointCallback};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/**
 * Encapsulates a type of hint that a caller may pass to a barcode reader to help it
 * more quickly or accurately decode it. It is up to implementations to decide what,
 * if anything, to do with the information that is supplied.
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 * @see Reader#decode(BinaryBitmap,java.util.Map)
 */
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum DecodeHintType {
    /**
     * Unspecified, application-specific hint. Maps to an unspecified {@link Object}.
     */
    OTHER,

    /**
     * Image is a pure monochrome image of a barcode. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    PURE_BARCODE,

    /**
     * Image is known to be of one of a few possible formats.
     * Maps to a {@link List} of {@link BarcodeFormat}s.
     */
    POSSIBLE_FORMATS,

    /**
     * Spend more time to try to find a barcode; optimize for accuracy, not speed.
     * Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    TRY_HARDER,

    /**
     * Specifies what character encoding to use when decoding, where applicable (type String)
     */
    CHARACTER_SET,

    /**
     * Allowed lengths of encoded data -- reject anything else. Maps to an {@code int[]}.
     */
    ALLOWED_LENGTHS,

    /**
     * Assume Code 39 codes employ a check digit. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    ASSUME_CODE_39_CHECK_DIGIT,

    /**
     * Assume the barcode is being processed as a GS1 barcode, and modify behavior as needed.
     * For example this affects FNC1 handling for Code 128 (aka GS1-128). Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    ASSUME_GS1,

    /**
     * If true, return the start and end digits in a Codabar barcode instead of stripping them. They
     * are alpha, whereas the rest are numeric. By default, they are stripped, but this causes them
     * to not be. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    RETURN_CODABAR_START_END,

    /**
     * The caller needs to be notified via callback when a possible {@link Point}
     * is found. Maps to a {@link PointCallback}.
     */
    NEED_RESULT_POINT_CALLBACK,

    /**
     * Allowed extension lengths for EAN or UPC barcodes. Other formats will ignore this.
     * Maps to an {@code int[]} of the allowed extension lengths, for example [2], [5], or [2, 5].
     * If it is optional to have an extension, do not set this hint. If this is set,
     * and a UPC or EAN barcode is found but an extension is not, then no result will be returned
     * at all.
     */
    ALLOWED_EAN_EXTENSIONS,

    /**
     * If true, also tries to decode as inverted image. All configured decoders are simply called a
     * second time with an inverted image. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    ALSO_INVERTED,

    /**
     * Specifies that the codes are expected to be in conformance with the specification
     * ISO/IEC 18004 regading the interpretation of character encoding. Values encoded in BYTE mode
     * or in KANJI mode are interpreted as ISO-8859-1 characters unless an ECI specified at a prior
     * location in the input specified a different encoding. By default the encoding of BYTE encoded
     * values is determinied by the {@link #CHARACTER_SET} hint or otherwise by a heuristic that
     * examines the bytes. By default KANJI encoded values are interpreted as the bytes of Shift-JIS
     * encoded characters (note that this is the case even if an ECI specifies a different
     * encoding).
     */
    #[cfg(feature = "allow_forced_iso_ied_18004_compliance")]
    QR_ASSUME_SPEC_CONFORM_INPUT,

    /*
     * Will translate the ASCII values parsed by the Telepen reader into the Telepen Numeric form.
     */
    TELEPEN_AS_NUMERIC,
    /*
     * Data type the hint is expecting.
     * Among the possible values the {@link Void} stands out as being used for
     * hints that do not expect a value to be supplied (flag hints). Such hints
     * will possibly have their value ignored, or replaced by a
     * {@link Boolean#TRUE}. Hint suppliers should probably use
     * {@link Boolean#TRUE} as directed by the actual hint documentation.
     */
    /*
    private final Class<?> valueType;

    DecodeHintType(Class<?> valueType) {
      this.valueType = valueType;
    }

    public Class<?> getValueType() {
      return valueType;
    }*/
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub enum DecodeHintValue {
    /**
     * Unspecified, application-specific hint. Maps to an unspecified {@link Object}.
     */
    Other(String),

    /**
     * Image is a pure monochrome image of a barcode. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    PureBarcode(bool),

    /**
     * Image is known to be of one of a few possible formats.
     * Maps to a {@link List} of {@link BarcodeFormat}s.
     */
    PossibleFormats(HashSet<BarcodeFormat>),

    /**
     * Spend more time to try to find a barcode; optimize for accuracy, not speed.
     * Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    TryHarder(bool),

    /**
     * Specifies what character encoding to use when decoding, where applicable (type String)
     */
    CharacterSet(String),

    /**
     * Allowed lengths of encoded data -- reject anything else. Maps to an {@code int[]}.
     */
    AllowedLengths(Vec<u32>),

    /**
     * Assume Code 39 codes employ a check digit. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    AssumeCode39CheckDigit(bool),

    /**
     * Assume the barcode is being processed as a GS1 barcode, and modify behavior as needed.
     * For example this affects FNC1 handling for Code 128 (aka GS1-128). Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    AssumeGs1(bool),

    /**
     * If true, return the start and end digits in a Codabar barcode instead of stripping them. They
     * are alpha, whereas the rest are numeric. By default, they are stripped, but this causes them
     * to not be. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    ReturnCodabarStartEnd(bool),

    /**
     * The caller needs to be notified via callback when a possible {@link Point}
     * is found. Maps to a {@link PointCallback}.
     */
    #[cfg_attr(feature = "serde", serde(skip_serializing, skip_deserializing))]
    NeedResultPointCallback(PointCallback),

    /**
     * Allowed extension lengths for EAN or UPC barcodes. Other formats will ignore this.
     * Maps to an {@code int[]} of the allowed extension lengths, for example [2], [5], or [2, 5].
     * If it is optional to have an extension, do not set this hint. If this is set,
     * and a UPC or EAN barcode is found but an extension is not, then no result will be returned
     * at all.
     */
    AllowedEanExtensions(Vec<u32>),

    /**
     * If true, also tries to decode as inverted image. All configured decoders are simply called a
     * second time with an inverted image. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    AlsoInverted(bool),

    /**
     * Specifies that the codes are expected to be in conformance with the specification
     * ISO/IEC 18004 regading the interpretation of character encoding. Values encoded in BYTE mode
     * or in KANJI mode are interpreted as ISO-8859-1 characters unless an ECI specified at a prior
     * location in the input specified a different encoding. By default the encoding of BYTE encoded
     * values is determinied by the {@link #CHARACTER_SET} hint or otherwise by a heuristic that
     * examines the bytes. By default KANJI encoded values are interpreted as the bytes of Shift-JIS
     * encoded characters (note that this is the case even if an ECI specifies a different
     * encoding).
     */
    #[cfg(feature = "allow_forced_iso_ied_18004_compliance")]
    QrAssumeSpecConformInput(bool),

    /**
     * Translate the ASCII values parsed by the Telepen reader into the Telepen Numeric form; use {@link Boolean#TRUE}.
     */
    TelepenAsNumeric(bool),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone)]
pub struct DecodeHints {
    /**
     * Unspecified, application-specific hint. Maps to an unspecified {@link Object}.
     */
    pub Other: Option<String>,

    /**
     * Image is a pure monochrome image of a barcode. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    pub PureBarcode: Option<bool>,

    /**
     * Image is known to be of one of a few possible formats.
     * Maps to a {@link List} of {@link BarcodeFormat}s.
     */
    pub PossibleFormats: Option<HashSet<BarcodeFormat>>,

    /**
     * Spend more time to try to find a barcode; optimize for accuracy, not speed.
     * Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    pub TryHarder: Option<bool>,

    /**
     * Specifies what character encoding to use when decoding, where applicable (type String)
     */
    pub CharacterSet: Option<String>,

    /**
     * Allowed lengths of encoded data -- reject anything else. Maps to an {@code int[]}.
     */
    pub AllowedLengths: Option<Vec<u32>>,

    /**
     * Assume Code 39 codes employ a check digit. Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    pub AssumeCode39CheckDigit: Option<bool>,

    /**
     * Assume the barcode is being processed as a GS1 barcode, and modify behavior as needed.
     * For example this affects FNC1 handling for Code 128 (aka GS1-128). Doesn't matter what it maps to;
     * use {@link Boolean#TRUE}.
     */
    pub AssumeGs1: Option<bool>,

    /**
     * If true, return the start and end digits in a Codabar barcode instead of stripping them. They
     * are alpha, whereas the rest are numeric. By default, they are stripped, but this causes them
     * to not be. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    pub ReturnCodabarStartEnd: Option<bool>,

    /**
     * The caller needs to be notified via callback when a possible {@link Point}
     * is found. Maps to a {@link PointCallback}.
     */
    #[cfg_attr(feature = "serde", serde(skip_serializing, skip_deserializing))]
    pub NeedResultPointCallback: Option<PointCallback>,

    /**
     * Allowed extension lengths for EAN or UPC barcodes. Other formats will ignore this.
     * Maps to an {@code int[]} of the allowed extension lengths, for example [2], [5], or [2, 5].
     * If it is optional to have an extension, do not set this hint. If this is set,
     * and a UPC or EAN barcode is found but an extension is not, then no result will be returned
     * at all.
     */
    pub AllowedEanExtensions: Option<Vec<u32>>,

    /**
     * If true, also tries to decode as inverted image. All configured decoders are simply called a
     * second time with an inverted image. Doesn't matter what it maps to; use {@link Boolean#TRUE}.
     */
    pub AlsoInverted: Option<bool>,

    /**
     * Specifies that the codes are expected to be in conformance with the specification
     * ISO/IEC 18004 regading the interpretation of character encoding. Values encoded in BYTE mode
     * or in KANJI mode are interpreted as ISO-8859-1 characters unless an ECI specified at a prior
     * location in the input specified a different encoding. By default the encoding of BYTE encoded
     * values is determinied by the {@link #CHARACTER_SET} hint or otherwise by a heuristic that
     * examines the bytes. By default KANJI encoded values are interpreted as the bytes of Shift-JIS
     * encoded characters (note that this is the case even if an ECI specifies a different
     * encoding).
     */
    #[cfg(feature = "allow_forced_iso_ied_18004_compliance")]
    pub QrAssumeSpecConformInput: Option<bool>,

    /**
     * Translate the ASCII values parsed by the Telepen reader into the Telepen Numeric form; use {@link Boolean#TRUE}.
     */
    pub TelepenAsNumeric: Option<bool>,
}

impl From<super::DecodingHintDictionary> for DecodeHints {
    fn from(value: super::DecodingHintDictionary) -> Self {
        let mut new_self: Self = Self::default();
        for (_, v) in value.into_iter() {
            match v {
                DecodeHintValue::Other(v) => new_self.Other = Some(v),
                DecodeHintValue::PureBarcode(v) => new_self.PureBarcode = Some(v),
                DecodeHintValue::PossibleFormats(v) => new_self.PossibleFormats = Some(v),
                DecodeHintValue::TryHarder(v) => new_self.TryHarder = Some(v),
                DecodeHintValue::CharacterSet(v) => new_self.CharacterSet = Some(v),
                DecodeHintValue::AllowedLengths(v) => new_self.AllowedLengths = Some(v),
                DecodeHintValue::AssumeCode39CheckDigit(v) => {
                    new_self.AssumeCode39CheckDigit = Some(v)
                }
                DecodeHintValue::AssumeGs1(v) => new_self.AssumeGs1 = Some(v),
                DecodeHintValue::ReturnCodabarStartEnd(v) => {
                    new_self.ReturnCodabarStartEnd = Some(v)
                }
                DecodeHintValue::NeedResultPointCallback(v) => {
                    new_self.NeedResultPointCallback = Some(v)
                }
                DecodeHintValue::AllowedEanExtensions(v) => new_self.AllowedEanExtensions = Some(v),
                DecodeHintValue::AlsoInverted(v) => new_self.AlsoInverted = Some(v),
                DecodeHintValue::TelepenAsNumeric(v) => new_self.TelepenAsNumeric = Some(v),
                #[cfg(feature = "allow_forced_iso_ied_18004_compliance")]
                DecodeHintValue::QrAssumeSpecConformInput(v) => {
                    new_self.QrAssumeSpecConformInput = Some(v)
                }
            }
        }
        new_self
    }
}

impl From<DecodeHints> for super::DecodingHintDictionary {
    fn from(value: DecodeHints) -> Self {
        let mut new_self = HashMap::default();

        if let Some(v) = value.Other {
            new_self.insert(DecodeHintType::OTHER, DecodeHintValue::Other(v));
        }

        if let Some(v) = value.PureBarcode {
            new_self.insert(
                DecodeHintType::PURE_BARCODE,
                DecodeHintValue::PureBarcode(v),
            );
        }

        if let Some(v) = value.PossibleFormats {
            new_self.insert(
                DecodeHintType::POSSIBLE_FORMATS,
                DecodeHintValue::PossibleFormats(v),
            );
        }

        if let Some(v) = value.TryHarder {
            new_self.insert(DecodeHintType::TRY_HARDER, DecodeHintValue::TryHarder(v));
        }

        if let Some(v) = value.CharacterSet {
            new_self.insert(
                DecodeHintType::CHARACTER_SET,
                DecodeHintValue::CharacterSet(v),
            );
        }

        if let Some(v) = value.AllowedLengths {
            new_self.insert(
                DecodeHintType::ALLOWED_LENGTHS,
                DecodeHintValue::AllowedLengths(v),
            );
        }

        if let Some(v) = value.AssumeCode39CheckDigit {
            new_self.insert(
                DecodeHintType::ASSUME_CODE_39_CHECK_DIGIT,
                DecodeHintValue::AssumeCode39CheckDigit(v),
            );
        }

        if let Some(v) = value.AssumeGs1 {
            new_self.insert(DecodeHintType::ASSUME_GS1, DecodeHintValue::AssumeGs1(v));
        }

        if let Some(v) = value.ReturnCodabarStartEnd {
            new_self.insert(
                DecodeHintType::RETURN_CODABAR_START_END,
                DecodeHintValue::ReturnCodabarStartEnd(v),
            );
        }

        if let Some(v) = value.NeedResultPointCallback {
            new_self.insert(
                DecodeHintType::NEED_RESULT_POINT_CALLBACK,
                DecodeHintValue::NeedResultPointCallback(v),
            );
        }

        if let Some(v) = value.AllowedEanExtensions {
            new_self.insert(
                DecodeHintType::ALLOWED_EAN_EXTENSIONS,
                DecodeHintValue::AllowedEanExtensions(v),
            );
        }

        if let Some(v) = value.AlsoInverted {
            new_self.insert(
                DecodeHintType::ALSO_INVERTED,
                DecodeHintValue::AlsoInverted(v),
            );
        }

        if let Some(v) = value.TelepenAsNumeric {
            new_self.insert(
                DecodeHintType::TELEPEN_AS_NUMERIC,
                DecodeHintValue::TelepenAsNumeric(v),
            );
        }

        #[cfg(feature = "allow_forced_iso_ied_18004_compliance")]
        if let Some(v) = value.QrAssumeSpecConformInput {
            new_self.insert(
                DecodeHintType::QR_ASSUME_SPEC_CONFORM_INPUT,
                DecodeHintValue::QrAssumeSpecConformInput(v),
            );
        }

        new_self
    }
}
