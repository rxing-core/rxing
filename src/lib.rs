use std::fmt;
use std::any::{Any,TypeId};
use std::time::SystemTime;

mod common;

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
 * Thrown when a barcode was not found in the image. It might have been
 * partially detected but could not be confirmed.
 *
 * @author Sean Owen
 */
pub struct NotFoundException;

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
 * Thrown when a barcode was successfully detected, but some aspect of
 * the content did not conform to the barcode's format rules. This could have
 * been due to a mis-detection.
 *
 * @author Sean Owen
 */
pub struct FormatException;

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
 * Thrown when a barcode was successfully detected and decoded, but
 * was not returned because its checksum feature failed.
 *
 * @author Sean Owen
 */
pub struct ChecksumException;

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
 * The general exception class throw when something goes wrong during decoding of a barcode.
 * This includes, but is not limited to, failing checksums / error correction algorithms, being
 * unable to locate finder timing patterns, and so on.
 *
 * @author Sean Owen
 */
pub struct ReaderException;

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
 * A base class which covers the range of exceptions which may occur when encoding a barcode using
 * the Writer framework.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct WriterException {
    message: String,
}

impl WriterException {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
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

/**
 * Enumerates barcode formats known to this package. Please keep alphabetized.
 *
 * @author Sean Owen
 */
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
  UPC_EAN_EXTENSION

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

use crate::common::BitMatrix;
use std::collections::HashMap;

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
    fn encode( contents : &str,  format : &BarcodeFormat,  width : i32,  height:i32) -> Result<BitMatrix,WriterException>;

  /**
   * @param contents The contents to encode in the barcode
   * @param format The barcode format to generate
   * @param width The preferred width in pixels
   * @param height The preferred height in pixels
   * @param hints Additional parameters to supply to the encoder
   * @return {@link BitMatrix} representing encoded barcode image
   * @throws WriterException if contents cannot be encoded legally in a format
   */
   fn encode_with_hints<T>( contents :&str,
                    format:&BarcodeFormat,
                    width:i32,
                    height:i32,
                    hints:HashMap<EncodeHintType,T>) -> Result<BitMatrix,WriterException>;

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

pub enum ReaderDecodeException {
    NotFoundException(NotFoundException),
    ChecksumException(ChecksumException),
    FormatException(FormatException)
}

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
  fn decode( image:BinaryBitmap) -> Result<RXingResult, ReaderDecodeException>;

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
   fn decode_with_hints<T>( image:BinaryBitmap,  hints:HashMap<DecodeHintType,T>) -> Result<RXingResult,ReaderDecodeException>;

  /**
   * Resets any internal state the implementation has after a decode, to prepare it
   * for reuse.
   */
  fn reset();

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
pub struct RXingResult<'a> {

   text:String,
   rawBytes: Vec<u8>,
   numBits :usize,
   resultPoints:Vec<RXingResultPoint>,
   format:BarcodeFormat,
   resultMetadata: Option<HashMap<RXingResultMetadataType,&'a dyn Any>>,
   timestamp:i64,

}
impl RXingResult<'_> {
  pub fn new( text: &str,
                 rawBytes: &Vec<u8>,
                 resultPoints:&Vec<RXingResultPoint>,
                 format:BarcodeFormat) -> Self{
    Self::new_timestamp(text, rawBytes, resultPoints, &format, SystemTime::now())
  }

  pub fn new_timestamp( text : &str,
                rawBytes : &Vec<u8>,
                 resultPoints:&Vec<RXingResultPoint>,
                 format:&BarcodeFormat,
                 timestamp : i64) -> Self{
    Self::new_complex(text, rawBytes, 8 * rawBytes.len(),
         resultPoints, format, timestamp)
  }

  pub fn  new_complex( text:&str,
    rawBytes : &Vec<u8>,
                 numBits :usize,
                 resultPoints:Vec<RXingResultPoint>,
                 format:BarcodeFormat,
                 timestamp:i64) -> Self{
                    Self {
                        text: text.to_owned(),
                        rawBytes: rawBytes,
                        numBits,
                        resultPoints,
                        format,
                        resultMetadata: None,
                        timestamp,
                    }
  }

  /**
   * @return raw text encoded by the barcode
   */
  pub fn getText(&self)->String{
    return self.text;
  }

  /**
   * @return raw bytes encoded by the barcode, if applicable, otherwise {@code null}
   */
  pub fn  getRawBytes(&self)-> Vec<u8>{
    return self.rawBytes;
  }

  /**
   * @return how many bits of {@link #getRawBytes()} are valid; typically 8 times its length
   * @since 3.3.0
   */
  pub fn getNumBits(&self) -> usize{
    return self.numBits;
  }

  /**
   * @return points related to the barcode in the image. These are typically points
   *         identifying finder patterns or the corners of the barcode. The exact meaning is
   *         specific to the type of barcode that was decoded.
   */
  pub fn  getRXingResultPoints(&self) -> Vec<RXingResultPoint>{
    return self.resultPoints;
  }

  /**
   * @return {@link BarcodeFormat} representing the format of the barcode that was decoded
   */
  pub fn  getBarcodeFormat(&self) ->BarcodeFormat{
    return self.format;
  }

  /**
   * @return {@link Map} mapping {@link RXingResultMetadataType} keys to values. May be
   *   {@code null}. This contains optional metadata about what was detected about the barcode,
   *   like orientation.
   */
  pub fn  getRXingResultMetadata(&self)  -> HashMap<RXingResultMetadataType,&dyn Any> {
    return self.resultMetadata;
  }

  pub fn putMetadata(&self, md_type :RXingResultMetadataType, value: &dyn Any) {
    if (self.resultMetadata.is_none()) {
      self.resultMetadata = Some(HashMap::new());
    }
    self.resultMetadata.unwrap().insert(md_type, value);
  }

  pub fn  putAllMetadata(&self, metadata: HashMap<RXingResultMetadataType,&dyn Any>) {
      if (self.resultMetadata.is_none()) {
        self.resultMetadata = Some(metadata);
      } else {
        for (key, value) in metadata.into_iter() {
            self.resultMetadata.unwrap().insert(key, value);
        }
      }
  }

  pub fn addRXingResultPoints(&self,  newPoints: Vec<RXingResultPoint>) {
    //RXingResultPoint[] oldPoints = resultPoints;
    if  !newPoints.is_empty() {
      // let allPoints:Vec<RXingResultPoint>= Vec::with_capacity(oldPoints.len() + newPoints.len());
      //System.arraycopy(oldPoints, 0, allPoints, 0, oldPoints.length);
      //System.arraycopy(newPoints, 0, allPoints, oldPoints.length, newPoints.length);
      //resultPoints = allPoints;
      self.resultPoints.append(&mut newPoints);
    }
  }

  pub fn getTimestamp(&self) -> i64 {
    return self.timestamp;
  }

}

impl fmt::Display for RXingResult<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.text)
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
use crate::common::detector::MathUtils;

/**
 * <p>Encapsulates a point of interest in an image containing a barcode. Typically, this
 * would be the location of a finder pattern or the corner of the barcode, for example.</p>
 *
 * @author Sean Owen
 */
#[derive(Eq,Hash,PartialEq)]
pub struct RXingResultPoint {

  x: f32,
  y: f32,
}
impl RXingResultPoint {
  pub fn new(x:f32, y:f32) -> Self{
    Self{
        x,
        y,
    }
  }

  pub fn getX(&self) -> f32{
    return self.x;
  }

  pub fn getY(&self) -> f32 {
    return self.y;
  }

  /**
   * Orders an array of three RXingResultPoints in an order [A,B,C] such that AB is less than AC
   * and BC is less than AC, and the angle between BC and BA is less than 180 degrees.
   *
   * @param patterns array of three {@code RXingResultPoint} to order
   */
  pub fn orderBestPatterns( patterns: &Vec<RXingResultPoint>) {

    // Find distances between pattern centers
    let zeroOneDistance = MathUtils::distance_float(patterns[0], patterns[1]);
    let oneTwoDistance = MathUtils::distance_float(patterns[1], patterns[2]);
    let zeroTwoDistance = MathUtils::distance_float(patterns[0], patterns[2]);

    let  pointA:RXingResultPoint;
    let pointB:RXingResultPoint;
    let pointC:RXingResultPoint;
    // Assume one closest to other two is B; A and C will just be guesses at first
    if (oneTwoDistance >= zeroOneDistance && oneTwoDistance >= zeroTwoDistance) {
      pointB = patterns[0];
      pointA = patterns[1];
      pointC = patterns[2];
    } else if (zeroTwoDistance >= oneTwoDistance && zeroTwoDistance >= zeroOneDistance) {
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
    if (RXingResultPoint::crossProductZ(&pointA, &pointB, &pointC) < 0.0f32) {
      let temp = pointA;
      pointA = pointC;
      pointC = temp;
    }

    patterns[0] = pointA;
    patterns[1] = pointB;
    patterns[2] = pointC;
  }

  /**
   * @param pattern1 first pattern
   * @param pattern2 second pattern
   * @return distance between two points
   */
  pub fn distance( pattern1:&RXingResultPoint,  pattern2:&RXingResultPoint) -> f32 {
    return MathUtils::distance_float(pattern1.x, pattern1.y, pattern2.x, pattern2.y);
  }

  /**
   * Returns the z component of the cross product between vectors BC and BA.
   */
  pub fn crossProductZ( pointA :&RXingResultPoint,
                                      pointB : &RXingResultPoint,
                                      pointC: &RXingResultPoint) -> f32 {
    let bX = pointB.x;
    let bY = pointB.y;
    return ((pointC.x - bX) * (pointA.y - bY)) - ((pointC.y - bY) * (pointA.x - bX));
  }

}

impl fmt::Display for RXingResultPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})", self.x, self.y)
    }
}