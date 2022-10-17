
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

use crate::{BarcodeFormat, RXingResultPointCallback};

/**
 * Encapsulates a type of hint that a caller may pass to a barcode reader to help it
 * more quickly or accurately decode it. It is up to implementations to decide what,
 * if anything, to do with the information that is supplied.
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 * @see Reader#decode(BinaryBitmap,java.util.Map)
 */
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
     * The caller needs to be notified via callback when a possible {@link RXingResultPoint}
     * is found. Maps to a {@link RXingResultPointCallback}.
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
    // End of enumeration values.

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
    PossibleFormats(BarcodeFormat),

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
     * The caller needs to be notified via callback when a possible {@link RXingResultPoint}
     * is found. Maps to a {@link RXingResultPointCallback}.
     */
    NeedResultPointCallback(RXingResultPointCallback),

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
    // End of enumeration values.
}