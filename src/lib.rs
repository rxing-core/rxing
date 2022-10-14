#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod aztec;
pub mod client;
pub mod common;
mod exceptions;
pub mod qrcode;

pub use exceptions::Exceptions;

#[cfg(feature = "image")]
mod buffered_image_luminance_source;

#[cfg(feature = "image")]
pub use buffered_image_luminance_source::*;

use crate::common::{BitArray, BitMatrix};
use exceptions::*;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(test)]
mod PlanarYUVLuminanceSourceTestCase;

#[cfg(test)]
mod RGBLuminanceSourceTestCase;

pub type EncodingHintDictionary = HashMap<EncodeHintType, EncodeHintValue>;
pub type DecodingHintDictionary = HashMap<DecodeHintType, DecodeHintValue>;
pub type MetadataDictionary = HashMap<RXingResultMetadataType, RXingResultMetadataValue>;

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

/**
 * Enumerates barcode formats known to this package. Please keep alphabetized.
 *
 * @author Sean Owen
 */
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BarcodeFormat {
    /** Aztec 2D barcode format. */
    AZTEC,

    /** CODABAR 1D format. */
    CODABAR,

    /** Code 39 1D format. */
    CODE_39,

    /** Code 93 1D format. */
    CODE_93,

    /** Code 128 1D format. */
    CODE_128,

    /** Data Matrix 2D barcode format. */
    DATA_MATRIX,

    /** EAN-8 1D format. */
    EAN_8,

    /** EAN-13 1D format. */
    EAN_13,

    /** ITF (Interleaved Two of Five) 1D format. */
    ITF,

    /** MaxiCode 2D barcode format. */
    MAXICODE,

    /** PDF417 format. */
    PDF_417,

    /** QR Code 2D barcode format. */
    QR_CODE,

    /** RSS 14 */
    RSS_14,

    /** RSS EXPANDED */
    RSS_EXPANDED,

    /** UPC-A 1D format. */
    UPC_A,

    /** UPC-E 1D format. */
    UPC_E,

    /** UPC/EAN extension format. Not a stand-alone format. */
    UPC_EAN_EXTENSION,
}

/*
 * Copyright 2008 ZXing authors
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

/**
 * These are a set of hints that you may pass to Writers to specify their behavior.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EncodeHintType {
    /**
     * Specifies what degree of error correction to use, for example in QR Codes.
     * Type depends on the encoder. For example for QR codes it's type
     * {@link com.google.zxing.qrcode.decoder.ErrorCorrectionLevel ErrorCorrectionLevel}.
     * For Aztec it is of type {@link Integer}, representing the minimal percentage of error correction words.
     * For PDF417 it is of type {@link Integer}, valid values being 0 to 8.
     * In all cases, it can also be a {@link String} representation of the desired value as well.
     * Note: an Aztec symbol should have a minimum of 25% EC words.
     */
    ERROR_CORRECTION,

    /**
     * Specifies what character encoding to use where applicable (type {@link String})
     */
    CHARACTER_SET,

    /**
     * Specifies the matrix shape for Data Matrix (type {@link com.google.zxing.datamatrix.encoder.SymbolShapeHint})
     */
    DATA_MATRIX_SHAPE,

    /**
     * Specifies whether to use compact mode for Data Matrix (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * The compact encoding mode also supports the encoding of characters that are not in the ISO-8859-1
     * character set via ECIs.
     * Please note that in that case, the most compact character encoding is chosen for characters in
     * the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
     * support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
     * means of the {@link #CHARACTER_SET} encoding hint.
     * Compact encoding also provides GS1-FNC1 support when {@link #GS1_FORMAT} is selected. In this case
     * group-separator character (ASCII 29 decimal) can be used to encode the positions of FNC1 codewords
     * for the purpose of delimiting AIs.
     * This option and {@link #FORCE_C40} are mutually exclusive.
     */
    DATA_MATRIX_COMPACT,

    /**
     * Specifies a minimum barcode size (type {@link Dimension}). Only applicable to Data Matrix now.
     *
     * @deprecated use width/height params in
     * {@link com.google.zxing.datamatrix.DataMatrixWriter#encode(String, BarcodeFormat, int, int)}
     */
    #[deprecated]
    MIN_SIZE,

    /**
     * Specifies a maximum barcode size (type {@link Dimension}). Only applicable to Data Matrix now.
     *
     * @deprecated without replacement
     */
    #[deprecated]
    MAX_SIZE,

    /**
     * Specifies margin, in pixels, to use when generating the barcode. The meaning can vary
     * by format; for example it controls margin before and after the barcode horizontally for
     * most 1D formats. (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    MARGIN,

    /**
     * Specifies whether to use compact mode for PDF417 (type {@link Boolean}, or "true" or "false"
     * {@link String} value).
     */
    PDF417_COMPACT,

    /**
     * Specifies what compaction mode to use for PDF417 (type
     * {@link com.google.zxing.pdf417.encoder.Compaction Compaction} or {@link String} value of one of its
     * enum values).
     */
    PDF417_COMPACTION,

    /**
     * Specifies the minimum and maximum number of rows and columns for PDF417 (type
     * {@link com.google.zxing.pdf417.encoder.Dimensions Dimensions}).
     */
    PDF417_DIMENSIONS,

    /**
     * Specifies whether to automatically insert ECIs when encoding PDF417 (type {@link Boolean}, or "true" or "false"
     * {@link String} value).
     * Please note that in that case, the most compact character encoding is chosen for characters in
     * the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
     * support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
     * means of the {@link #CHARACTER_SET} encoding hint.
     */
    PDF417_AUTO_ECI,

    /**
     * Specifies the required number of layers for an Aztec code.
     * A negative number (-1, -2, -3, -4) specifies a compact Aztec code.
     * 0 indicates to use the minimum number of layers (the default).
     * A positive number (1, 2, .. 32) specifies a normal (non-compact) Aztec code.
     * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    AZTEC_LAYERS,

    /**
     * Specifies the exact version of QR code to be encoded.
     * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    QR_VERSION,

    /**
     * Specifies the QR code mask pattern to be used. Allowed values are
     * 0..QRCode.NUM_MASK_PATTERNS-1. By default the code will automatically select
     * the optimal mask pattern.
     * * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    QR_MASK_PATTERN,

    /**
     * Specifies whether to use compact mode for QR code (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * Please note that when compaction is performed, the most compact character encoding is chosen
     * for characters in the input that are not in the ISO-8859-1 character set. Based on experience,
     * some scanners do not support encodings like cp-1256 (Arabic). In such cases the encoding can
     * be forced to UTF-8 by means of the {@link #CHARACTER_SET} encoding hint.
     */
    QR_COMPACT,

    /**
     * Specifies whether the data should be encoded to the GS1 standard (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     */
    GS1_FORMAT,

    /**
     * Forces which encoding will be used. Currently only used for Code-128 code sets (Type {@link String}).
     * Valid values are "A", "B", "C".
     * This option and {@link #CODE128_COMPACT} are mutually exclusive.
     */
    FORCE_CODE_SET,

    /**
     * Forces C40 encoding for data-matrix (type {@link Boolean}, or "true" or "false") {@link String } value). This
     * option and {@link #DATA_MATRIX_COMPACT} are mutually exclusive.
     */
    FORCE_C40,

    /**
     * Specifies whether to use compact mode for Code-128 code (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * This can yield slightly smaller bar codes. This option and {@link #FORCE_CODE_SET} are mutually
     * exclusive.
     */
    CODE128_COMPACT,
}

pub enum EncodeHintValue {
    /**
     * Specifies what degree of error correction to use, for example in QR Codes.
     * Type depends on the encoder. For example for QR codes it's type
     * {@link com.google.zxing.qrcode.decoder.ErrorCorrectionLevel ErrorCorrectionLevel}.
     * For Aztec it is of type {@link Integer}, representing the minimal percentage of error correction words.
     * For PDF417 it is of type {@link Integer}, valid values being 0 to 8.
     * In all cases, it can also be a {@link String} representation of the desired value as well.
     * Note: an Aztec symbol should have a minimum of 25% EC words.
     */
    ErrorCorrection(String),

    /**
     * Specifies what character encoding to use where applicable (type {@link String})
     */
    CharacterSet(String),

    /**
     * Specifies the matrix shape for Data Matrix (type {@link com.google.zxing.datamatrix.encoder.SymbolShapeHint})
     */
    DataMatrixShape,

    /**
     * Specifies whether to use compact mode for Data Matrix (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * The compact encoding mode also supports the encoding of characters that are not in the ISO-8859-1
     * character set via ECIs.
     * Please note that in that case, the most compact character encoding is chosen for characters in
     * the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
     * support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
     * means of the {@link #CHARACTER_SET} encoding hint.
     * Compact encoding also provides GS1-FNC1 support when {@link #GS1_FORMAT} is selected. In this case
     * group-separator character (ASCII 29 decimal) can be used to encode the positions of FNC1 codewords
     * for the purpose of delimiting AIs.
     * This option and {@link #FORCE_C40} are mutually exclusive.
     */
    DataMatrixCompact(String),

    /**
     * Specifies a minimum barcode size (type {@link Dimension}). Only applicable to Data Matrix now.
     *
     * @deprecated use width/height params in
     * {@link com.google.zxing.datamatrix.DataMatrixWriter#encode(String, BarcodeFormat, int, int)}
     */
    #[deprecated]
    MinSize,

    /**
     * Specifies a maximum barcode size (type {@link Dimension}). Only applicable to Data Matrix now.
     *
     * @deprecated without replacement
     */
    #[deprecated]
    MaxSize(Dimension),

    /**
     * Specifies margin, in pixels, to use when generating the barcode. The meaning can vary
     * by format; for example it controls margin before and after the barcode horizontally for
     * most 1D formats. (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    Margin(String),

    /**
     * Specifies whether to use compact mode for PDF417 (type {@link Boolean}, or "true" or "false"
     * {@link String} value).
     */
    Pdf417Compact(String),

    /**
     * Specifies what compaction mode to use for PDF417 (type
     * {@link com.google.zxing.pdf417.encoder.Compaction Compaction} or {@link String} value of one of its
     * enum values).
     */
    Pdf417Compaction(String),

    /**
     * Specifies the minimum and maximum number of rows and columns for PDF417 (type
     * {@link com.google.zxing.pdf417.encoder.Dimensions Dimensions}).
     */
    Pdf417Dimensions,

    /**
     * Specifies whether to automatically insert ECIs when encoding PDF417 (type {@link Boolean}, or "true" or "false"
     * {@link String} value).
     * Please note that in that case, the most compact character encoding is chosen for characters in
     * the input that are not in the ISO-8859-1 character set. Based on experience, some scanners do not
     * support encodings like cp-1256 (Arabic). In such cases the encoding can be forced to UTF-8 by
     * means of the {@link #CHARACTER_SET} encoding hint.
     */
    Pdf417AutoEci(String),

    /**
     * Specifies the required number of layers for an Aztec code.
     * A negative number (-1, -2, -3, -4) specifies a compact Aztec code.
     * 0 indicates to use the minimum number of layers (the default).
     * A positive number (1, 2, .. 32) specifies a normal (non-compact) Aztec code.
     * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    AztecLayers(i32),

    /**
     * Specifies the exact version of QR code to be encoded.
     * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    QrVersion(String),

    /**
     * Specifies the QR code mask pattern to be used. Allowed values are
     * 0..QRCode.NUM_MASK_PATTERNS-1. By default the code will automatically select
     * the optimal mask pattern.
     * * (Type {@link Integer}, or {@link String} representation of the integer value).
     */
    QrMaskPattern(String),

    /**
     * Specifies whether to use compact mode for QR code (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * Please note that when compaction is performed, the most compact character encoding is chosen
     * for characters in the input that are not in the ISO-8859-1 character set. Based on experience,
     * some scanners do not support encodings like cp-1256 (Arabic). In such cases the encoding can
     * be forced to UTF-8 by means of the {@link #CHARACTER_SET} encoding hint.
     */
    QrCompact(String),

    /**
     * Specifies whether the data should be encoded to the GS1 standard (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     */
    Gs1Format(String),

    /**
     * Forces which encoding will be used. Currently only used for Code-128 code sets (Type {@link String}).
     * Valid values are "A", "B", "C".
     * This option and {@link #CODE128_COMPACT} are mutually exclusive.
     */
    ForceCodeSet(String),

    /**
     * Forces C40 encoding for data-matrix (type {@link Boolean}, or "true" or "false") {@link String } value). This
     * option and {@link #DATA_MATRIX_COMPACT} are mutually exclusive.
     */
    ForceC40(String),

    /**
     * Specifies whether to use compact mode for Code-128 code (type {@link Boolean}, or "true" or "false"
     * {@link String } value).
     * This can yield slightly smaller bar codes. This option and {@link #FORCE_CODE_SET} are mutually
     * exclusive.
     */
    Code128Compact(String),
}

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

/**
 * Callback which is invoked when a possible result point (significant
 * point in the barcode image such as a corner) is found.
 *
 * @see DecodeHintType#NEED_RESULT_POINT_CALLBACK
 */
pub type RXingResultPointCallback = fn(&dyn ResultPoint);
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

/*
 * Copyright 2008 ZXing authors
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

/**
 * The base class for all objects which encode/generate a barcode image.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub trait Writer {
    /**
     * Encode a barcode using the default settings.
     *
     * @param contents The contents to encode in the barcode
     * @param format The barcode format to generate
     * @param width The preferred width in pixels
     * @param height The preferred height in pixels
     * @return {@link BitMatrix} representing encoded barcode image
     * @throws WriterException if contents cannot be encoded legally in a format
     */
    fn encode(
        &self,
        contents: &str,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<BitMatrix, Exceptions>;

    /**
     * @param contents The contents to encode in the barcode
     * @param format The barcode format to generate
     * @param width The preferred width in pixels
     * @param height The preferred height in pixels
     * @param hints Additional parameters to supply to the encoder
     * @return {@link BitMatrix} representing encoded barcode image
     * @throws WriterException if contents cannot be encoded legally in a format
     */
    fn encode_with_hints(
        &self,
        contents: &str,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
        hints: &EncodingHintDictionary,
    ) -> Result<BitMatrix, Exceptions>;
}

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

/**
 * Implementations of this interface can decode an image of a barcode in some format into
 * the String it encodes. For example, {@link com.google.zxing.qrcode.QRCodeReader} can
 * decode a QR code. The decoder may optionally receive hints from the caller which may help
 * it decode more quickly or accurately.
 *
 * See {@link MultiFormatReader}, which attempts to determine what barcode
 * format is present within the image as well, and then decodes it accordingly.
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub trait Reader {
    /**
     * Locates and decodes a barcode in some format within an image.
     *
     * @param image image of barcode to decode
     * @return String which the barcode encodes
     * @throws NotFoundException if no potential barcode is found
     * @throws ChecksumException if a potential barcode is found but does not pass its checksum
     * @throws FormatException if a potential barcode is found but format is invalid
     */
    fn decode(&self, image: &BinaryBitmap) -> Result<RXingResult, Exceptions>;

    /**
     * Locates and decodes a barcode in some format within an image. This method also accepts
     * hints, each possibly associated to some data, which may help the implementation decode.
     *
     * @param image image of barcode to decode
     * @param hints passed as a {@link Map} from {@link DecodeHintType}
     * to arbitrary data. The
     * meaning of the data depends upon the hint type. The implementation may or may not do
     * anything with these hints.
     * @return String which the barcode encodes
     * @throws NotFoundException if no potential barcode is found
     * @throws ChecksumException if a potential barcode is found but does not pass its checksum
     * @throws FormatException if a potential barcode is found but format is invalid
     */
    fn decode_with_hints(
        &self,
        image: &BinaryBitmap,
        hints: &DecodingHintDictionary,
    ) -> Result<RXingResult, Exceptions>;

    /**
     * Resets any internal state the implementation has after a decode, to prepare it
     * for reuse.
     */
    fn reset(&self);
}

/*
 * Copyright 2008 ZXing authors
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

/**
 * Represents some type of metadata about the result of the decoding that the decoder
 * wishes to communicate back to the caller.
 *
 * @author Sean Owen
 */
#[derive(Eq, PartialEq, Hash, Debug)]
pub enum RXingResultMetadataType {
    /**
     * Unspecified, application-specific metadata. Maps to an unspecified {@link Object}.
     */
    OTHER,

    /**
     * Denotes the likely approximate orientation of the barcode in the image. This value
     * is given as degrees rotated clockwise from the normal, upright orientation.
     * For example a 1D barcode which was found by reading top-to-bottom would be
     * said to have orientation "90". This key maps to an {@link Integer} whose
     * value is in the range [0,360).
     */
    ORIENTATION,

    /**
     * <p>2D barcode formats typically encode text, but allow for a sort of 'byte mode'
     * which is sometimes used to encode binary data. While {@link RXingResult} makes available
     * the complete raw bytes in the barcode for these formats, it does not offer the bytes
     * from the byte segments alone.</p>
     *
     * <p>This maps to a {@link java.util.List} of byte arrays corresponding to the
     * raw bytes in the byte segments in the barcode, in order.</p>
     */
    BYTE_SEGMENTS,

    /**
     * Error correction level used, if applicable. The value type depends on the
     * format, but is typically a String.
     */
    ERROR_CORRECTION_LEVEL,

    /**
     * For some periodicals, indicates the issue number as an {@link Integer}.
     */
    ISSUE_NUMBER,

    /**
     * For some products, indicates the suggested retail price in the barcode as a
     * formatted {@link String}.
     */
    SUGGESTED_PRICE,

    /**
     * For some products, the possible country of manufacture as a {@link String} denoting the
     * ISO country code. Some map to multiple possible countries, like "US/CA".
     */
    POSSIBLE_COUNTRY,

    /**
     * For some products, the extension text
     */
    UPC_EAN_EXTENSION,

    /**
     * PDF417-specific metadata
     */
    PDF417_EXTRA_METADATA,

    /**
     * If the code format supports structured append and the current scanned code is part of one then the
     * sequence number is given with it.
     */
    STRUCTURED_APPEND_SEQUENCE,

    /**
     * If the code format supports structured append and the current scanned code is part of one then the
     * parity is given with it.
     */
    STRUCTURED_APPEND_PARITY,

    /**
     * Barcode Symbology Identifier.
     * Note: According to the GS1 specification the identifier may have to replace a leading FNC1/GS character
     * when prepending to the barcode content.
     */
    SYMBOLOGY_IDENTIFIER,
}

impl From<String> for RXingResultMetadataType {
    fn from(in_str: String) -> Self {
        match in_str.as_str() {
            "OTHER" => RXingResultMetadataType::OTHER,
            "ORIENTATION" => RXingResultMetadataType::ORIENTATION,
            "BYTE_SEGMENTS" => RXingResultMetadataType::BYTE_SEGMENTS,
            "ERROR_CORRECTION_LEVEL" => RXingResultMetadataType::ERROR_CORRECTION_LEVEL,
            "ISSUE_NUMBER" => RXingResultMetadataType::ISSUE_NUMBER,
            "SUGGESTED_PRICE" => RXingResultMetadataType::SUGGESTED_PRICE,
            "POSSIBLE_COUNTRY" => RXingResultMetadataType::POSSIBLE_COUNTRY,
            "UPC_EAN_EXTENSION" => RXingResultMetadataType::UPC_EAN_EXTENSION,
            "PDF417_EXTRA_METADATA" => RXingResultMetadataType::PDF417_EXTRA_METADATA,
            "STRUCTURED_APPEND_SEQUENCE" => RXingResultMetadataType::STRUCTURED_APPEND_SEQUENCE,
            "STRUCTURED_APPEND_PARITY" => RXingResultMetadataType::STRUCTURED_APPEND_PARITY,
            "SYMBOLOGY_IDENTIFIER" => RXingResultMetadataType::SYMBOLOGY_IDENTIFIER,
            _ => RXingResultMetadataType::OTHER,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RXingResultMetadataValue {
    /**
     * Unspecified, application-specific metadata. Maps to an unspecified {@link Object}.
     */
    OTHER(String),

    /**
     * Denotes the likely approximate orientation of the barcode in the image. This value
     * is given as degrees rotated clockwise from the normal, upright orientation.
     * For example a 1D barcode which was found by reading top-to-bottom would be
     * said to have orientation "90". This key maps to an {@link Integer} whose
     * value is in the range [0,360).
     */
    Orientation(i32),

    /**
     * <p>2D barcode formats typically encode text, but allow for a sort of 'byte mode'
     * which is sometimes used to encode binary data. While {@link RXingResult} makes available
     * the complete raw bytes in the barcode for these formats, it does not offer the bytes
     * from the byte segments alone.</p>
     *
     * <p>This maps to a {@link java.util.List} of byte arrays corresponding to the
     * raw bytes in the byte segments in the barcode, in order.</p>
     */
    ByteSegments(Vec<Vec<u8>>),

    /**
     * Error correction level used, if applicable. The value type depends on the
     * format, but is typically a String.
     */
    ErrorCorrectionLevel(String),

    /**
     * For some periodicals, indicates the issue number as an {@link Integer}.
     */
    IssueNumber(i32),

    /**
     * For some products, indicates the suggested retail price in the barcode as a
     * formatted {@link String}.
     */
    SuggestedPrice(String),

    /**
     * For some products, the possible country of manufacture as a {@link String} denoting the
     * ISO country code. Some map to multiple possible countries, like "US/CA".
     */
    PossibleCountry(String),

    /**
     * For some products, the extension text
     */
    UpcEanExtension(String),

    /**
     * PDF417-specific metadata
     */
    Pdf417ExtraMetadata(String),

    /**
     * If the code format supports structured append and the current scanned code is part of one then the
     * sequence number is given with it.
     */
    StructuredAppendSequence(i32),

    /**
     * If the code format supports structured append and the current scanned code is part of one then the
     * parity is given with it.
     */
    StructuredAppendParity(i32),

    /**
     * Barcode Symbology Identifier.
     * Note: According to the GS1 specification the identifier may have to replace a leading FNC1/GS character
     * when prepending to the barcode content.
     */
    SymbologyIdentifier(String),
}

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

//import java.util.EnumMap;
//import java.util.Map;

/**
 * <p>Encapsulates the result of decoding a barcode within an image.</p>
 *
 * @author Sean Owen
 */
pub struct RXingResult {
    text: String,
    rawBytes: Vec<u8>,
    numBits: usize,
    resultPoints: Vec<RXingResultPoint>,
    format: BarcodeFormat,
    resultMetadata: HashMap<RXingResultMetadataType, RXingResultMetadataValue>,
    timestamp: u128,
}
impl RXingResult {
    pub fn new(
        text: &str,
        rawBytes: Vec<u8>,
        resultPoints: Vec<RXingResultPoint>,
        format: BarcodeFormat,
    ) -> Self {
        Self::new_timestamp(
            text,
            rawBytes,
            resultPoints,
            format,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis(),
        )
    }

    pub fn new_timestamp(
        text: &str,
        rawBytes: Vec<u8>,
        resultPoints: Vec<RXingResultPoint>,
        format: BarcodeFormat,
        timestamp: u128,
    ) -> Self {
        let l = rawBytes.len();
        Self::new_complex(text, rawBytes, 8 * l, resultPoints, format, timestamp)
    }

    pub fn new_complex(
        text: &str,
        rawBytes: Vec<u8>,
        numBits: usize,
        resultPoints: Vec<RXingResultPoint>,
        format: BarcodeFormat,
        timestamp: u128,
    ) -> Self {
        Self {
            text: text.to_owned(),
            rawBytes: rawBytes,
            numBits,
            resultPoints,
            format,
            resultMetadata: HashMap::new(),
            timestamp,
        }
    }

    /**
     * @return raw text encoded by the barcode
     */
    pub fn getText(&self) -> &String {
        return &self.text;
    }

    /**
     * @return raw bytes encoded by the barcode, if applicable, otherwise {@code null}
     */
    pub fn getRawBytes(&self) -> &Vec<u8> {
        return &self.rawBytes;
    }

    /**
     * @return how many bits of {@link #getRawBytes()} are valid; typically 8 times its length
     * @since 3.3.0
     */
    pub fn getNumBits(&self) -> usize {
        return self.numBits;
    }

    /**
     * @return points related to the barcode in the image. These are typically points
     *         identifying finder patterns or the corners of the barcode. The exact meaning is
     *         specific to the type of barcode that was decoded.
     */
    pub fn getRXingResultPoints(&self) -> &Vec<RXingResultPoint> {
        return &self.resultPoints;
    }

    /**
     * @return {@link BarcodeFormat} representing the format of the barcode that was decoded
     */
    pub fn getBarcodeFormat(&self) -> &BarcodeFormat {
        return &self.format;
    }

    /**
     * @return {@link Map} mapping {@link RXingResultMetadataType} keys to values. May be
     *   {@code null}. This contains optional metadata about what was detected about the barcode,
     *   like orientation.
     */
    pub fn getRXingResultMetadata(
        &self,
    ) -> &HashMap<RXingResultMetadataType, RXingResultMetadataValue> {
        return &self.resultMetadata;
    }

    pub fn putMetadata(
        &mut self,
        md_type: RXingResultMetadataType,
        value: RXingResultMetadataValue,
    ) {
        self.resultMetadata.insert(md_type, value);
    }

    pub fn putAllMetadata(
        &mut self,
        metadata: HashMap<RXingResultMetadataType, RXingResultMetadataValue>,
    ) {
        if self.resultMetadata.is_empty() {
            self.resultMetadata = metadata;
        } else {
            for (key, value) in metadata.into_iter() {
                self.resultMetadata.insert(key, value);
            }
        }
    }

    pub fn addRXingResultPoints(&mut self, newPoints: &mut Vec<RXingResultPoint>) {
        //RXingResultPoint[] oldPoints = resultPoints;
        if !newPoints.is_empty() {
            // let allPoints:Vec<RXingResultPoint>= Vec::with_capacity(oldPoints.len() + newPoints.len());
            //System.arraycopy(oldPoints, 0, allPoints, 0, oldPoints.length);
            //System.arraycopy(newPoints, 0, allPoints, oldPoints.length, newPoints.length);
            //resultPoints = allPoints;
            self.resultPoints.append(newPoints);
        }
    }

    pub fn getTimestamp(&self) -> u128 {
        return self.timestamp;
    }
}

impl fmt::Display for RXingResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

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

pub trait ResultPoint {
    fn getX(&self) -> f32;
    fn getY(&self) -> f32;
    fn into_rxing_result_point(self) -> RXingResultPoint;
}

pub mod result_point_utils {
    use crate::{common::detector::MathUtils, ResultPoint};

    /**
     * Orders an array of three RXingResultPoints in an order [A,B,C] such that AB is less than AC
     * and BC is less than AC, and the angle between BC and BA is less than 180 degrees.
     *
     * @param patterns array of three {@code RXingResultPoint} to order
     */
    pub fn orderBestPatterns<T: ResultPoint + Copy + Clone>(patterns: &mut [T; 3]) {
        // Find distances between pattern centers
        let zeroOneDistance = MathUtils::distance_float(
            patterns[0].getX(),
            patterns[0].getY(),
            patterns[1].getX(),
            patterns[1].getY(),
        );
        let oneTwoDistance = MathUtils::distance_float(
            patterns[1].getX(),
            patterns[1].getY(),
            patterns[2].getX(),
            patterns[2].getY(),
        );
        let zeroTwoDistance = MathUtils::distance_float(
            patterns[0].getX(),
            patterns[0].getY(),
            patterns[2].getX(),
            patterns[2].getY(),
        );

        let mut pointA; //: &RXingResultPoint;
        let mut pointB; //: &RXingResultPoint;
        let mut pointC; //: &RXingResultPoint;
                        // Assume one closest to other two is B; A and C will just be guesses at first
        if oneTwoDistance >= zeroOneDistance && oneTwoDistance >= zeroTwoDistance {
            pointB = patterns[0];
            pointA = patterns[1];
            pointC = patterns[2];
        } else if zeroTwoDistance >= oneTwoDistance && zeroTwoDistance >= zeroOneDistance {
            pointB = patterns[1];
            pointA = patterns[0];
            pointC = patterns[2];
        } else {
            pointB = patterns[2];
            pointA = patterns[0];
            pointC = patterns[1];
        }

        // Use cross product to figure out whether A and C are correct or flipped.
        // This asks whether BC x BA has a positive z component, which is the arrangement
        // we want for A, B, C. If it's negative, then we've got it flipped around and
        // should swap A and C.
        if crossProductZ(pointA, pointB, pointC) < 0.0f32 {
            let temp = pointA;
            pointA = pointC;
            pointC = temp;
        }

        let pa = pointA;
        let pb = pointB;
        let pc = pointC;

        patterns[0] = pa;
        patterns[1] = pb;
        patterns[2] = pc;
    }

    /**
     * @param pattern1 first pattern
     * @param pattern2 second pattern
     * @return distance between two points
     */
    pub fn distance<T: ResultPoint>(pattern1: &T, pattern2: &T) -> f32 {
        return MathUtils::distance_float(
            pattern1.getX(),
            pattern1.getY(),
            pattern2.getX(),
            pattern2.getY(),
        );
    }

    /**
     * Returns the z component of the cross product between vectors BC and BA.
     */
    pub fn crossProductZ<T: ResultPoint>(pointA: T, pointB: T, pointC: T) -> f32 {
        let bX = pointB.getX();
        let bY = pointB.getY();
        return ((pointC.getX() - bX) * (pointA.getY() - bY))
            - ((pointC.getY() - bY) * (pointA.getX() - bX));
    }
}

/**
 * <p>Encapsulates a point of interest in an image containing a barcode. Typically, this
 * would be the location of a finder pattern or the corner of the barcode, for example.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, Clone, Copy)]
pub struct RXingResultPoint {
    x: f32,
    y: f32,
}
impl Hash for RXingResultPoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.to_string().hash(state);
        self.y.to_string().hash(state);
    }
}
impl PartialEq for RXingResultPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x.to_string() == other.x.to_string() && self.y.to_string() == other.y.to_string()
    }
}
impl Eq for RXingResultPoint {}
impl RXingResultPoint {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl ResultPoint for RXingResultPoint {
    fn getX(&self) -> f32 {
        return self.x;
    }

    fn getY(&self) -> f32 {
        return self.y;
    }

    fn into_rxing_result_point(self) -> RXingResultPoint {
        self
    }
}

impl fmt::Display for RXingResultPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

/*
 * Copyright 2012 ZXing authors
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

/**
 * Simply encapsulates a width and height.
 */
#[derive(Eq, PartialEq, Hash)]
pub struct Dimension {
    width: usize,
    height: usize,
}

impl Dimension {
    pub fn new(width: usize, height: usize) -> Result<Self, Exceptions> {
        Ok(Self { width, height })
    }

    pub fn getWidth(&self) -> usize {
        return self.width;
    }

    pub fn getHeight(&self) -> usize {
        return self.height;
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

/*
 * Copyright 2009 ZXing authors
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

/**
 * This class hierarchy provides a set of methods to convert luminance data to 1 bit data.
 * It allows the algorithm to vary polymorphically, for example allowing a very expensive
 * thresholding technique for servers and a fast one for mobile. It also permits the implementation
 * to vary, e.g. a JNI version for Android and a Java fallback version for other platforms.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub trait Binarizer {
    //private final LuminanceSource source;
    //fn new(source:dyn LuminanceSource) -> Self;

    fn getLuminanceSource(&self) -> &Box<dyn LuminanceSource>;

    /**
     * Converts one row of luminance data to 1 bit data. May actually do the conversion, or return
     * cached data. Callers should assume this method is expensive and call it as seldom as possible.
     * This method is intended for decoding 1D barcodes and may choose to apply sharpening.
     * For callers which only examine one row of pixels at a time, the same BitArray should be reused
     * and passed in with each call for performance. However it is legal to keep more than one row
     * at a time if needed.
     *
     * @param y The row to fetch, which must be in [0, bitmap height)
     * @param row An optional preallocated array. If null or too small, it will be ignored.
     *            If used, the Binarizer will call BitArray.clear(). Always use the returned object.
     * @return The array of bits for this row (true means black).
     * @throws NotFoundException if row can't be binarized
     */
    fn getBlackRow(&self, y: usize, row: &mut BitArray) -> Result<BitArray, Exceptions>;

    /**
     * Converts a 2D array of luminance data to 1 bit data. As above, assume this method is expensive
     * and do not call it repeatedly. This method is intended for decoding 2D barcodes and may or
     * may not apply sharpening. Therefore, a row from this matrix may not be identical to one
     * fetched using getBlackRow(), so don't mix and match between them.
     *
     * @return The 2D array of bits for the image (true means black).
     * @throws NotFoundException if image can't be binarized to make a matrix
     */
    fn getBlackMatrix(&self) -> Result<BitMatrix, Exceptions>;

    /**
     * Creates a new object with the same type as this Binarizer implementation, but with pristine
     * state. This is needed because Binarizer implementations may be stateful, e.g. keeping a cache
     * of 1 bit data. See Effective Java for why we can't use Java's clone() method.
     *
     * @param source The LuminanceSource this Binarizer will operate on.
     * @return A new concrete Binarizer implementation object.
     */
    fn createBinarizer(&self, source: Box<dyn LuminanceSource>) -> Box<dyn Binarizer>;

    fn getWidth(&self) -> usize;

    fn getHeight(&self) -> usize;
}

/*
 * Copyright 2009 ZXing authors
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

/**
 * This class is the core bitmap class used by ZXing to represent 1 bit data. Reader objects
 * accept a BinaryBitmap and attempt to decode it.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct BinaryBitmap {
    binarizer: Box<dyn Binarizer>,
    matrix: BitMatrix,
}

impl BinaryBitmap {
    pub fn new(binarizer: Box<dyn Binarizer>) -> Self {
        Self {
            matrix: binarizer.getBlackMatrix().unwrap(),
            binarizer: binarizer,
        }
    }

    /**
     * @return The width of the bitmap.
     */
    pub fn getWidth(&self) -> usize {
        return self.binarizer.getWidth();
    }

    /**
     * @return The height of the bitmap.
     */
    pub fn getHeight(&self) -> usize {
        return self.binarizer.getHeight();
    }

    /**
     * Converts one row of luminance data to 1 bit data. May actually do the conversion, or return
     * cached data. Callers should assume this method is expensive and call it as seldom as possible.
     * This method is intended for decoding 1D barcodes and may choose to apply sharpening.
     *
     * @param y The row to fetch, which must be in [0, bitmap height)
     * @param row An optional preallocated array. If null or too small, it will be ignored.
     *            If used, the Binarizer will call BitArray.clear(). Always use the returned object.
     * @return The array of bits for this row (true means black).
     * @throws NotFoundException if row can't be binarized
     */
    pub fn getBlackRow(&self, y: usize, row: &mut BitArray) -> Result<BitArray, Exceptions> {
        return self.binarizer.getBlackRow(y, row);
    }

    /**
     * Converts a 2D array of luminance data to 1 bit. As above, assume this method is expensive
     * and do not call it repeatedly. This method is intended for decoding 2D barcodes and may or
     * may not apply sharpening. Therefore, a row from this matrix may not be identical to one
     * fetched using getBlackRow(), so don't mix and match between them.
     *
     * @return The 2D array of bits for the image (true means black).
     * @throws NotFoundException if image can't be binarized to make a matrix
     */
    pub fn getBlackMatrix(&self) -> Result<&BitMatrix, Exceptions> {
        // The matrix is created on demand the first time it is requested, then cached. There are two
        // reasons for this:
        // 1. This work will never be done if the caller only installs 1D Reader objects, or if a
        //    1D Reader finds a barcode before the 2D Readers run.
        // 2. This work will only be done once even if the caller installs multiple 2D Readers.
        return Ok(&self.matrix);
    }

    /**
     * @return Whether this bitmap can be cropped.
     */
    pub fn isCropSupported(&self) -> bool {
        let b = &self.binarizer;
        let r = &b.getLuminanceSource();
        let isCropOk = r.isCropSupported();
        return isCropOk;
    }

    /**
     * Returns a new object with cropped image data. Implementations may keep a reference to the
     * original data rather than a copy. Only callable if isCropSupported() is true.
     *
     * @param left The left coordinate, which must be in [0,getWidth())
     * @param top The top coordinate, which must be in [0,getHeight())
     * @param width The width of the rectangle to crop.
     * @param height The height of the rectangle to crop.
     * @return A cropped version of this object.
     */
    pub fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> BinaryBitmap {
        let newSource = self
            .binarizer
            .getLuminanceSource()
            .crop(left, top, width, height);
        return BinaryBitmap::new(
            self.binarizer
                .createBinarizer(newSource.expect("new lum source expected")),
        );
    }

    /**
     * @return Whether this bitmap supports counter-clockwise rotation.
     */
    pub fn isRotateSupported(&self) -> bool {
        return self.binarizer.getLuminanceSource().isRotateSupported();
    }

    /**
     * Returns a new object with rotated image data by 90 degrees counterclockwise.
     * Only callable if {@link #isRotateSupported()} is true.
     *
     * @return A rotated version of this object.
     */
    pub fn rotateCounterClockwise(&self) -> BinaryBitmap {
        let newSource = self.binarizer.getLuminanceSource().rotateCounterClockwise();
        return BinaryBitmap::new(
            self.binarizer
                .createBinarizer(newSource.expect("new lum source expected")),
        );
    }

    /**
     * Returns a new object with rotated image data by 45 degrees counterclockwise.
     * Only callable if {@link #isRotateSupported()} is true.
     *
     * @return A rotated version of this object.
     */
    pub fn rotateCounterClockwise45(&self) -> BinaryBitmap {
        let newSource = self
            .binarizer
            .getLuminanceSource()
            .rotateCounterClockwise45();
        return BinaryBitmap::new(
            self.binarizer
                .createBinarizer(newSource.expect("new lum source expected")),
        );
    }
}

impl fmt::Display for BinaryBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.getBlackMatrix().unwrap())
    }
}

/*
 * Copyright 2009 ZXing authors
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

/**
 * The purpose of this class hierarchy is to abstract different bitmap implementations across
 * platforms into a standard interface for requesting greyscale luminance values. The interface
 * only provides immutable methods; therefore crop and rotation create copies. This is to ensure
 * that one Reader does not modify the original luminance source and leave it in an unknown state
 * for other Readers in the chain.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub trait LuminanceSource {
    //private final int width;
    //private final int height;

    //fn new( width:usize,  height:usize) -> Self;

    /**
     * Fetches one row of luminance data from the underlying platform's bitmap. Values range from
     * 0 (black) to 255 (white). Because Java does not have an unsigned byte type, callers will have
     * to bitwise and with 0xff for each value. It is preferable for implementations of this method
     * to only fetch this row rather than the whole image, since no 2D Readers may be installed and
     * getMatrix() may never be called.
     *
     * @param y The row to fetch, which must be in [0,getHeight())
     * @param row An optional preallocated array. If null or too small, it will be ignored.
     *            Always use the returned object, and ignore the .length of the array.
     * @return An array containing the luminance data.
     */
    fn getRow(&self, y: usize, row: &Vec<u8>) -> Vec<u8>;

    /**
     * Fetches luminance data for the underlying bitmap. Values should be fetched using:
     * {@code int luminance = array[y * width + x] & 0xff}
     *
     * @return A row-major 2D array of luminance values. Do not use result.length as it may be
     *         larger than width * height bytes on some platforms. Do not modify the contents
     *         of the result.
     */
    fn getMatrix(&self) -> Vec<u8>;

    /**
     * @return The width of the bitmap.
     */
    fn getWidth(&self) -> usize;

    /**
     * @return The height of the bitmap.
     */
    fn getHeight(&self) -> usize;

    /**
     * @return Whether this subclass supports cropping.
     */
    fn isCropSupported(&self) -> bool {
        return false;
    }

    /**
     * Returns a new object with cropped image data. Implementations may keep a reference to the
     * original data rather than a copy. Only callable if isCropSupported() is true.
     *
     * @param left The left coordinate, which must be in [0,getWidth())
     * @param top The top coordinate, which must be in [0,getHeight())
     * @param width The width of the rectangle to crop.
     * @param height The height of the rectangle to crop.
     * @return A cropped version of this object.
     */
    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        return Err(Exceptions::UnsupportedOperationException(
            "This luminance source does not support cropping.".to_owned(),
        ));
    }

    /**
     * @return Whether this subclass supports counter-clockwise rotation.
     */
    fn isRotateSupported(&self) -> bool {
        return false;
    }

    /**
     * @return a wrapper of this {@code LuminanceSource} which inverts the luminances it returns -- black becomes
     *  white and vice versa, and each value becomes (255-value).
     */
    fn invert(&mut self); /* {
                            return InvertedLuminanceSource::new_with_delegate(self);
                          }*/

    /**
     * Returns a new object with rotated image data by 90 degrees counterclockwise.
     * Only callable if {@link #isRotateSupported()} is true.
     *
     * @return A rotated version of this object.
     */
    fn rotateCounterClockwise(&self) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        return Err(Exceptions::UnsupportedOperationException(
            "This luminance source does not support rotation by 90 degrees.".to_owned(),
        ));
    }

    /**
     * Returns a new object with rotated image data by 45 degrees counterclockwise.
     * Only callable if {@link #isRotateSupported()} is true.
     *
     * @return A rotated version of this object.
     */
    fn rotateCounterClockwise45(&self) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        return Err(Exceptions::UnsupportedOperationException(
            "This luminance source does not support rotation by 45 degrees.".to_owned(),
        ));
    }

    fn invert_block_of_bytes(&self, vec_to_invert: Vec<u8>) -> Vec<u8> {
        let mut iv = vec_to_invert.clone();
        for itm in iv.iter_mut() {
            let z = *itm;
            *itm = 255 - (z & 0xFF);
        }
        iv
    }

    /*
    @Override
    public final String toString() {
      byte[] row = new byte[width];
      StringBuilder result = new StringBuilder(height * (width + 1));
      for (int y = 0; y < height; y++) {
        row = getRow(y, row);
        for (int x = 0; x < width; x++) {
          int luminance = row[x] & 0xFF;
          char c;
          if (luminance < 0x40) {
            c = '#';
          } else if (luminance < 0x80) {
            c = '+';
          } else if (luminance < 0xC0) {
            c = '.';
          } else {
            c = ' ';
          }
          result.append(c);
        }
        result.append('\n');
      }
      return result.toString();
    }*/
}

// /*
//  * Copyright 2013 ZXing authors
//  *
//  * Licensed under the Apache License, Version 2.0 (the "License");
//  * you may not use this file except in compliance with the License.
//  * You may obtain a copy of the License at
//  *
//  *      http://www.apache.org/licenses/LICENSE-2.0
//  *
//  * Unless required by applicable law or agreed to in writing, software
//  * distributed under the License is distributed on an "AS IS" BASIS,
//  * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  * See the License for the specific language governing permissions and
//  * limitations under the License.
//  */
// //package com.google.zxing;

// /**
//  * A wrapper implementation of {@link LuminanceSource} which inverts the luminances it returns -- black becomes
//  * white and vice versa, and each value becomes (255-value).
//  *
//  * @author Sean Owen
//  */
// pub struct InvertedLuminanceSource {
//     width: usize,
//     height: usize,
//     delegate: Box<dyn LuminanceSource>,
// }

// impl InvertedLuminanceSource {
//     pub fn new_with_delegate(delegate: Box<dyn LuminanceSource>) -> Self {
//         Self {
//             width: delegate.getWidth(),
//             height: delegate.getHeight(),
//             delegate,
//         }
//     }
// }

// impl LuminanceSource for InvertedLuminanceSource {
//     fn getRow(&self, y: usize, row: &Vec<u8>) -> Vec<u8> {
//         let mut new_row = self.delegate.getRow(y, row);
//         let width = self.getWidth();
//         for i in 0..width {
//             //for (int i = 0; i < width; i++) {
//             new_row[i] = 255 - (new_row[i] & 0xFF);
//         }
//         return new_row;
//     }

//     fn getMatrix(&self) -> Vec<u8> {
//         let matrix = self.delegate.getMatrix();
//         let length = self.getWidth() * self.getHeight();
//         let mut invertedMatrix = Vec::with_capacity(length);
//         for i in 0..length {
//             //for (int i = 0; i < length; i++) {
//             invertedMatrix[i] = 255 - (matrix[i] & 0xFF);
//         }
//         return invertedMatrix;
//     }

//     fn getWidth(&self) -> usize {
//         self.width
//     }

//     fn getHeight(&self) -> usize {
//         self.height
//     }

//     fn isCropSupported(&self) -> bool {
//         return self.delegate.isCropSupported();
//     }

//     fn crop(
//         &self,
//         left: usize,
//         top: usize,
//         width: usize,
//         height: usize,
//     ) -> Result<Box<dyn LuminanceSource>, UnsupportedOperationException> {
//         let crop = self.delegate.crop(left, top, width, height)?;
//         return Ok(Box::new(InvertedLuminanceSource::new_with_delegate(crop)));
//     }

//     fn isRotateSupported(&self) -> bool {
//         return self.delegate.isRotateSupported();
//     }

//     /**
//      * @return original delegate {@link LuminanceSource} since invert undoes itself
//      */
//     fn invert(&self) -> Box<dyn LuminanceSource> {
//         return self.delegate;
//     }

//     fn rotateCounterClockwise(
//         &self,
//     ) -> Result<Box<dyn LuminanceSource>, UnsupportedOperationException> {
//         let rot = self.delegate.rotateCounterClockwise()?;
//         return Ok(Box::new(InvertedLuminanceSource::new_with_delegate(rot)));
//     }

//     fn rotateCounterClockwise45(
//         &self,
//     ) -> Result<Box<dyn LuminanceSource>, UnsupportedOperationException> {
//         let rot_45 = self.delegate.rotateCounterClockwise45()?;
//         return Ok(Box::new(InvertedLuminanceSource::new_with_delegate(rot_45)));
//     }
// }

/*
 * Copyright 2009 ZXing authors
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

const THUMBNAIL_SCALE_FACTOR: usize = 2;

/**
 * This object extends LuminanceSource around an array of YUV data returned from the camera driver,
 * with the option to crop to a rectangle within the full data. This can be used to exclude
 * superfluous pixels around the perimeter and speed up decoding.
 *
 * It works for any pixel format where the Y channel is planar and appears first, including
 * YCbCr_420_SP and YCbCr_422_SP.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Debug, Clone)]
pub struct PlanarYUVLuminanceSource {
    yuv_data: Vec<u8>,
    data_width: usize,
    data_height: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    invert: bool,
}

impl PlanarYUVLuminanceSource {
    pub fn new_with_all(
        yuv_data: Vec<u8>,
        data_width: usize,
        data_height: usize,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
        reverse_horizontal: bool,
        inverted: bool,
    ) -> Result<Self, Exceptions> {
        if left + width > data_width || top + height > data_height {
            return Err(Exceptions::IllegalArgumentException(
                "Crop rectangle does not fit within image data.".to_owned(),
            ));
        }

        let mut new_s: Self = Self {
            yuv_data,
            data_width,
            data_height,
            left,
            top,
            width,
            height,
            invert: inverted,
        };

        if reverse_horizontal {
            new_s.reverseHorizontal(width, height);
        }

        Ok(new_s)
    }

    pub fn renderThumbnail(&self) -> Vec<u8> {
        let width = self.getWidth() / THUMBNAIL_SCALE_FACTOR;
        let height = self.getHeight() / THUMBNAIL_SCALE_FACTOR;
        let mut pixels = vec![0; width * height];
        let yuv = &self.yuv_data;
        let mut input_offset = self.top * self.data_width + self.left;

        for y in 0..height {
            //for (int y = 0; y < height; y++) {
            let output_offset = y * width;
            for x in 0..width {
                //for (int x = 0; x < width; x++) {
                let grey = yuv[input_offset + x * THUMBNAIL_SCALE_FACTOR] & 0xff;
                pixels[output_offset + x] = (0xFF000000 | (grey as u32 * 0x00010101)) as u8;
            }
            input_offset += self.data_width * THUMBNAIL_SCALE_FACTOR;
        }
        return pixels;
    }

    /**
     * @return width of image from {@link #renderThumbnail()}
     */
    pub fn getThumbnailWidth(&self) -> usize {
        return self.getWidth() / THUMBNAIL_SCALE_FACTOR;
    }

    /**
     * @return height of image from {@link #renderThumbnail()}
     */
    pub fn getThumbnailHeight(&self) -> usize {
        return self.getHeight() / THUMBNAIL_SCALE_FACTOR;
    }

    fn reverseHorizontal(&mut self, width: usize, height: usize) {
        //let mut yuvData = self.yuvData;
        let mut rowStart = self.top * self.data_width + self.left;
        for _y in 0..height {
            let middle = rowStart + width / 2;
            let mut x2 = rowStart + width - 1;
            for x1 in rowStart..middle {
                //for (int x1 = rowStart, x2 = rowStart + width - 1; x1 < middle; x1++, x2--) {
                let temp = self.yuv_data[x1];
                self.yuv_data[x1] = self.yuv_data[x2];
                self.yuv_data[x2] = temp;
                x2 -= 1;
            }
            rowStart += self.data_width;
        }
        //self.yuvData = yuvData;
        /*for (int y = 0, rowStart = top * dataWidth + left; y < height; y++, rowStart += dataWidth) {
          let middle = rowStart + width / 2;
          for (int x1 = rowStart, x2 = rowStart + width - 1; x1 < middle; x1++, x2--) {
            let temp = yuvData[x1];
            yuvData[x1] = yuvData[x2];
            yuvData[x2] = temp;
          }
        }*/
    }
}

impl LuminanceSource for PlanarYUVLuminanceSource {
    fn getRow(&self, y: usize, row: &Vec<u8>) -> Vec<u8> {
        if y >= self.getHeight() {
            //throw new IllegalArgumentException("Requested row is outside the image: " + y);
            panic!("Requested row is outside the image: {}", y);
        }
        let width = self.getWidth();

        let offset = (y + self.top) * self.data_width + self.left;

        let mut row = if row.len() >= width {
            row.to_vec()
        } else {
            vec![0; width]
        };

        row[..width].clone_from_slice(&self.yuv_data[offset..width + offset]);
        //System.arraycopy(yuvData, offset, row, 0, width);
        if self.invert {
            row = self.invert_block_of_bytes(row);
        }
        return row;
    }

    fn getMatrix(&self) -> Vec<u8> {
        let width = self.getWidth();
        let height = self.getHeight();

        // If the caller asks for the entire underlying image, save the copy and give them the
        // original data. The docs specifically warn that result.length must be ignored.
        if width == self.data_width && height == self.data_height {
            let mut v = self.yuv_data.clone();
            if self.invert {
                v = self.invert_block_of_bytes(v);
            }
            return v;
        }

        let area = width * height;
        let mut matrix = vec![0; area];
        let mut inputOffset = self.top * self.data_width + self.left;

        // If the width matches the full width of the underlying data, perform a single copy.
        if width == self.data_width {
            matrix[0..area].clone_from_slice(&self.yuv_data[inputOffset..area]);
            //System.arraycopy(yuvData, inputOffset, matrix, 0, area);
            if self.invert {
                matrix = self.invert_block_of_bytes(matrix);
            }
            return matrix;
        }

        // Otherwise copy one cropped row at a time.
        for y in 0..height {
            //for (int y = 0; y < height; y++) {
            let outputOffset = y * width;
            matrix[outputOffset..outputOffset + width]
                .clone_from_slice(&self.yuv_data[inputOffset..inputOffset + width]);
            //System.arraycopy(yuvData, inputOffset, matrix, outputOffset, width);
            inputOffset += self.data_width;
        }

        if self.invert {
            matrix = self.invert_block_of_bytes(matrix);
        }

        return matrix;
    }

    fn getWidth(&self) -> usize {
        self.width
    }

    fn getHeight(&self) -> usize {
        self.height
    }

    fn isCropSupported(&self) -> bool {
        return true;
    }

    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        match PlanarYUVLuminanceSource::new_with_all(
            self.yuv_data.clone(),
            self.data_width,
            self.data_height,
            self.left + left,
            self.top + top,
            width,
            height,
            false,
            self.invert,
        ) {
            Ok(new) => Ok(Box::new(new)),
            Err(_err) => Err(Exceptions::UnsupportedOperationException("".to_owned())),
        }
    }

    fn isRotateSupported(&self) -> bool {
        return false;
    }

    fn invert(&mut self) {
        self.invert = !self.invert;
    }
}

/*
 * Copyright 2009 ZXing authors
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

/**
 * This class is used to help decode images from files which arrive as RGB data from
 * an ARGB pixel array. It does not support rotation.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Betaminos
 */
#[derive(Debug, Clone)]
pub struct RGBLuminanceSource {
    luminances: Vec<u8>,
    dataWidth: usize,
    dataHeight: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
    invert: bool,
}

impl LuminanceSource for RGBLuminanceSource {
    fn getRow(&self, y: usize, row: &Vec<u8>) -> Vec<u8> {
        if y >= self.getHeight() {
            panic!("Requested row is outside the image: {}", y);
        }
        let width = self.getWidth();

        let offset = (y + self.top) * self.dataWidth + self.left;

        let mut row = if row.len() >= width {
            row.to_vec()
        } else {
            vec![0; width]
        };

        row[..width].clone_from_slice(&self.luminances[offset..offset + width]);
        //System.arraycopy(self.luminances, offset, row, 0, width);
        if self.invert {
            row = self.invert_block_of_bytes(row);
        }
        return row;
    }

    fn getMatrix(&self) -> Vec<u8> {
        let width = self.getWidth();
        let height = self.getHeight();

        // If the caller asks for the entire underlying image, save the copy and give them the
        // original data. The docs specifically warn that result.length must be ignored.
        if width == self.dataWidth && height == self.dataHeight {
            let mut z = self.luminances.clone();
            if self.invert {
                z = self.invert_block_of_bytes(z);
            }
            return z;
        }

        let area = width * height;
        let mut matrix = vec![0; area];
        let mut inputOffset = self.top * self.dataWidth + self.left;

        // If the width matches the full width of the underlying data, perform a single copy.
        if width == self.dataWidth {
            matrix[..area].clone_from_slice(&self.luminances[inputOffset..area + inputOffset]);
            //System.arraycopy(self.luminances, inputOffset, matrix, 0, area);
            if self.invert {
                matrix = self.invert_block_of_bytes(matrix);
            }
            return matrix;
        }

        // Otherwise copy one cropped row at a time.
        for y in 0..height {
            //for (int y = 0; y < height; y++) {
            let outputOffset = y * width;
            matrix[outputOffset..width + outputOffset]
                .clone_from_slice(&self.luminances[inputOffset..width + inputOffset]);
            //System.arraycopy(luminances, inputOffset, matrix, outputOffset, width);
            inputOffset += self.dataWidth;
        }

        if self.invert {
            matrix = self.invert_block_of_bytes(matrix);
        }
        return matrix;
    }

    fn getWidth(&self) -> usize {
        self.width
    }

    fn getHeight(&self) -> usize {
        self.height
    }

    fn isCropSupported(&self) -> bool {
        return true;
    }

    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>, Exceptions> {
        match RGBLuminanceSource::new_complex(
            &self.luminances,
            self.dataWidth,
            self.dataHeight,
            self.left + left,
            self.top + top,
            width,
            height,
        ) {
            Ok(crop) => Ok(Box::new(crop)),
            Err(_error) => Err(Exceptions::UnsupportedOperationException("".to_owned())),
        }
    }

    fn invert(&mut self) {
        self.invert = !self.invert;
    }
}

impl RGBLuminanceSource {
    pub fn new_with_width_height_pixels(width: usize, height: usize, pixels: &Vec<u32>) -> Self {
        //super(width, height);

        let dataWidth = width;
        let dataHeight = height;
        let left = 0;
        let top = 0;

        // In order to measure pure decoding speed, we convert the entire image to a greyscale array
        // up front, which is the same as the Y channel of the YUVLuminanceSource in the real app.
        //
        // Total number of pixels suffices, can ignore shape
        let size = width * height;
        let mut luminances: Vec<u8> = vec![0; size];
        for offset in 0..size {
            //for (int offset = 0; offset < size; offset++) {
            let pixel = pixels[offset];
            let r = (pixel >> 16) & 0xff; // red
            let g2 = (pixel >> 7) & 0x1fe; // 2 * green
            let b = pixel & 0xff; // blue
                                  // Calculate green-favouring average cheaply
            luminances[offset] = ((r + g2 + b) / 4).try_into().unwrap();
        }
        Self {
            luminances,
            dataWidth,
            dataHeight,
            left: left,
            top: top,
            width,
            height,
            invert: false,
        }
    }

    fn new_complex(
        pixels: &Vec<u8>,
        data_width: usize,
        data_height: usize,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Self, Exceptions> {
        if left + width > data_width || top + height > data_height {
            return Err(Exceptions::IllegalArgumentException(
                "Crop rectangle does not fit within image data.".to_owned(),
            ));
        }
        Ok(Self {
            luminances: pixels.clone(),
            dataWidth: data_width,
            dataHeight: data_height,
            left,
            top,
            width,
            height,
            invert: false,
        })
    }
}
