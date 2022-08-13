mod common;
mod aztec;
mod datamatrix;
mod maxicode;
mod oned;
mod pdf417;
mod qrcode;

use std::{fmt, collections::HashMap};

use crate::common::{BitArray,BitMatrix};
use crate::aztec::{AztecReader,AztecWriter};
use crate::datamatrix::{DataMatrixReader,DataMatrixWriter};
use crate::maxicode::DataMatrixReader;
use crate::oned::MultiFormatOneDReader;
use crate::pdf417::PDF417Reader;
use crate::qrcode::QRCodeReader;

use crate::oned::{CodaBarWriter,Code128Writer,Code39Writer,Code93Writer,EAN13Writer,EAN8Writer,ITFWriter,UPCAWriter,UPCEWriter};
use crate::pdf417::PDF417Writer;
use crate::qrcode::QRCodeWriter;

use crate::common::detector::MathUtils;

// BarcodeFormat.java

/** Enumerates barcode formats known to this package. Please keep alphabetized. */
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

// Binarizer.java

/**
 * This class hierarchy provides a set of methods to convert luminance data to 1 bit data.
 * It allows the algorithm to vary polymorphically, for example allowing a very expensive
 * thresholding technique for servers and a fast one for mobile. It also permits the implementation
 * to vary, e.g. a JNI version for Android and a Java fallback version for other platforms.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
trait Binarizer {
    fn get_luminance_source(&self) -> dyn LuminanceSource;

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
    fn get_black_row(&self, y: i32, row: &BitArray) -> Result<BitArray, NotFoundException>;

    /**
     * Converts a 2D array of luminance data to 1 bit data. As above, assume this method is expensive
     * and do not call it repeatedly. This method is intended for decoding 2D barcodes and may or
     * may not apply sharpening. Therefore, a row from this matrix may not be identical to one
     * fetched using getBlackRow(), so don't mix and match between them.
     *
     * @return The 2D array of bits for the image (true means black).
     * @throws NotFoundException if image can't be binarized to make a matrix
     */
    fn get_black_matrix(&self) -> Result<BitMatrix, NotFoundException>;

    /**
     * Creates a new object with the same type as this Binarizer implementation, but with pristine
     * state. This is needed because Binarizer implementations may be stateful, e.g. keeping a cache
     * of 1 bit data. See Effective Java for why we can't use Java's clone() method.
     *
     * @param source The LuminanceSource this Binarizer will operate on.
     * @return A new concrete Binarizer implementation object.
     */
    fn create_binarizer(&self, source: &dyn LuminanceSource) -> dyn Binarizer;

    fn get_width(&self) -> i32;

    fn get_height(&self) -> i32;
}

// BinaryBitmap.java

/**
 * This class is the core bitmap class used by ZXing to represent 1 bit data. Reader objects
 * accept a BinaryBitmap and attempt to decode it.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct BinaryBitmap {
    binarizer: dyn Binarizer,
    matrix: Option<BitMatrix>
}

impl BinaryBitmap {

   pub fn new( binarizer: &impl Binarizer) -> BinaryBitmap {
       BinaryBitmap { binarizer: binarizer, matrix: () }
   }

   /**
  * @return The width of the bitmap.
  */
   pub fn  get_width(&self) -> i32  {
       return self.binarizer.get_width();
   }

   /**
  * @return The height of the bitmap.
  */
   pub fn  get_height(&self) -> i32  {
       return self.binarizer.get_height();
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
   pub fn  get_black_row(&self,  y: i32,  row: &BitArray) -> Result<BitArray, NotFoundException>   {
       return Ok(self.binarizer.get_black_row(y, row));
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
   pub fn  get_black_matrix(&self) -> Result<BitMatrix, NotFoundException>   {
       // 2. This work will only be done once even if the caller installs multiple 2D Readers.
       if self.matrix.is_none() {
           self.matrix = Some(self.binarizer.get_black_matrix())
       }
       return Ok(self.matrix);
   }

   /**
  * @return Whether this bitmap can be cropped.
  */
   pub fn  is_crop_supported(&self) -> bool  {
       return self.binarizer.get_luminance_source().is_crop_supported();
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
   pub fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> BinaryBitmap  {
        let new_source: LuminanceSource = self.binarizer.get_luminance_source().crop(left, top, width, height);
       return BinaryBitmap::new(&self.binarizer.create_binarizer(new_source));
   }

   /**
  * @return Whether this bitmap supports counter-clockwise rotation.
  */
   pub fn  is_rotate_supported(&self) -> bool  {
       return self.binarizer.get_luminance_source().is_rotate_supported();
   }

   /**
  * Returns a new object with rotated image data by 90 degrees counterclockwise.
  * Only callable if {@link #isRotateSupported()} is true.
  *
  * @return A rotated version of this object.
  */
   pub fn  rotate_counter_clockwise(&self) -> BinaryBitmap  {
        let new_source: LuminanceSource = self.binarizer.get_luminance_source().rotate_counter_clockwise();
       return BinaryBitmap::new(&self.binarizer.create_binarizer(new_source));
   }

   /**
  * Returns a new object with rotated image data by 45 degrees counterclockwise.
  * Only callable if {@link #isRotateSupported()} is true.
  *
  * @return A rotated version of this object.
  */
   pub fn  rotate_counter_clockwise45(&self) -> BinaryBitmap  {
        let new_source: LuminanceSource = self.binarizer.get_luminance_source().rotate_counter_clockwise45();
       return BinaryBitmap::new(&self.binarizer.create_binarizer(new_source));
   }
}

impl fmt::Display for BinaryBitmap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_black_matrix())
    }
}

// ChecksumException.java
pub struct ChecksumException;

// DecodeHintType.java
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
   * The caller needs to be notified via callback when a possible {@link ResultPoint}
   * is found. Maps to a {@link ResultPointCallback}.
   */
    NEED_RESULT_POINT_CALLBAC,
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
    ALSO_INVERTED

    // End of enumeration values.
}

// Dimension.java

/**
 * Simply encapsulates a width and height.
 */
#[derive(Hash,Eq)]
pub struct Dimension {

    width: i32,
    height: i32
}

impl Dimension {

   pub fn new( width: i32,  height: i32) -> Result<Dimension,IllegalArgumentException> {
       if width < 0 || height < 0 {
           return Err(IllegalArgumentException::new());
       }
       Ok(Dimension{width,height})
   }

   pub fn  get_width(&self) -> i32  {
       return self.width;
   }

   pub fn  get_height(&self) -> i32  {
       return self.height;
   }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

// EncodeHintType.java

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
    CODE128_COMPACT
}

// FormatException.java
pub struct FormatException;

// LuminanceS

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
  fn  get_row(&self,  y: i32,  row: &Vec<i8>) -> Vec<i8> ;

  /**
 * Fetches luminance data for the underlying bitmap. Values should be fetched using:
 * {@code int luminance = array[y * width + x] & 0xff}
 *
 * @return A row-major 2D array of luminance values. Do not use result.length as it may be
 *         larger than width * height bytes on some platforms. Do not modify the contents
 *         of the result.
 */
   fn  get_matrix(&self) -> Vec<i8> ;

  /**
 * @return The width of the bitmap.
 */
   fn  get_width(&self) -> i32  ;

  /**
 * @return The height of the bitmap.
 */
   fn  get_height(&self) -> i32  ;

  /**
 * @return Whether this subclass supports cropping.
 */
   fn  is_crop_supported(&self) -> bool  {
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
   fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> Result<LuminanceSource,UnsupportedOperationException>  {
      Err(UnsupportedOperationException::new("This luminance source does not support cropping."))
  }

  /**
 * @return Whether this subclass supports counter-clockwise rotation.
 */
   fn  is_rotate_supported(&self) -> bool  {
      return false;
  }

  /**
 * @return a wrapper of this {@code LuminanceSource} which inverts the luminances it returns -- black becomes
 *  white and vice versa, and each value becomes (255-value).
 */
   fn  invert(&self) -> LuminanceSource  {
      return InvertedLuminanceSource::new(self);
  }

  /**
 * Returns a new object with rotated image data by 90 degrees counterclockwise.
 * Only callable if {@link #isRotateSupported()} is true.
 *
 * @return A rotated version of this object.
 */
   fn  rotate_counter_clockwise(&self) -> Result<LuminanceSource,UnsupportedOperationException>  {
      Err( UnsupportedOperationException::new("This luminance source does not support rotation by 90 degrees."))
  }

  /**
 * Returns a new object with rotated image data by 45 degrees counterclockwise.
 * Only callable if {@link #isRotateSupported()} is true.
 *
 * @return A rotated version of this object.
 */
   fn  rotate_counter_clockwise45(&self) -> Result<LuminanceSource,UnsupportedOperationException>  {
      Err( UnsupportedOperationException::new("This luminance source does not support rotation by 45 degrees."))
  }

}

// InvertedLuminanceSource.java
/**
 * A wrapper implementation of {@link LuminanceSource} which inverts the luminances it returns -- black becomes
 * white and vice versa, and each value becomes (255-value).
 *
 * @author Sean Owen
 */
pub struct InvertedLuminanceSource {
width : i32,
height: i32,
      delegate: dyn LuminanceSource
}

impl InvertedLuminanceSource {

    pub fn new( delegate: &impl LuminanceSource) -> InvertedLuminanceSource {
        InvertedLuminanceSource{ width: delegate.get_width(), height: delegate.get_height(), delegate }
    }
}
impl LuminanceSource for InvertedLuminanceSource{

     fn  get_row(&self,  y: i32,  row: &Vec<i8>) -> Vec<i8>  {
        row = &self.delegate.get_row(y, &row);
         let width: i32 = self.get_width();
         {
             let mut i: i32 = 0;
            while i < width {
                {
                    row[i] = (255 - (row[i] & 0xFF)) as i8;
                }
                i += 1;
             }
         }

        return row;
    }

     fn  get_matrix(&self) -> Vec<i8>  {
         let matrix: Vec<i8> = self.delegate.get_matrix();
         let length: i32 = self.get_width() * self.get_height();
         let inverted_matrix: [i8; length] = [0; length];
         {
             let mut i: i32 = 0;
            while i < length {
                {
                    inverted_matrix[i] = (255 - (matrix[i] & 0xFF)) as i8;
                }
                i += 1;
             }
         }

        return inverted_matrix;
    }

     fn  is_crop_supported(&self) -> bool  {
        return self.delegate.is_crop_supported();
    }

     fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> LuminanceSource  {
        return InvertedLuminanceSource::new(&self.delegate.crop(left, top, width, height));
    }

     fn  is_rotate_supported(&self) -> bool  {
        return self.delegate.is_rotate_supported();
    }

    /**
   * @return original delegate {@link LuminanceSource} since invert undoes itself
   */
     fn  invert(&self) -> dyn LuminanceSource  {
        return self.delegate;
    }

     fn  rotate_counter_clockwise(&self) -> dyn LuminanceSource  {
        return InvertedLuminanceSource::new(&self.delegate.rotate_counter_clockwise());
    }

     fn  rotate_counter_clockwise45(&self) -> dyn LuminanceSource  {
        return InvertedLuminanceSource::new(&self.delegate.rotate_counter_clockwise45());
    }

    fn  get_width(&self) -> i32   {
        self.width
    }

    fn  get_height(&self) -> i32   {
        self.height
    }
}

impl fmt::Display for InvertedLuminanceSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row: [i8; self.width] = [0; self.width];
    //let result: StringBuilder = StringBuilder::new(self.height * (self.width + 1));
    {
        let mut y: i32 = 0;
       while y < self.height {
           {
               row = self.get_row(y, &row);
                {
                    let mut x: i32 = 0;
                   while x < self.width {
                       {
                            let luminance: i32 = row[x] & 0xFF;
                            let mut c: char;
                           if luminance < 0x40 {
                               c = '#';
                           } else if luminance < 0x80 {
                               c = '+';
                           } else if luminance < 0xC0 {
                               c = '.';
                           } else {
                               c = ' ';
                           }
                           write!(f, "{}", c);
                       }
                       x += 1;
                    }
                }

                write!(f, "\n");
           }
           y += 1;
        }
    }

   Ok(())
    }
}

// Reader.java

pub enum ReaderException{
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
    fn  decode<T>(&self,  image: &BinaryBitmap, hints:Option<&HashMap<DecodeHintType, T>>) -> Result<RXingResult, ReaderException>  ;

    /**
   * Resets any internal state the implementation has after a decode, to prepare it
   * for reuse.
   */
    fn  reset(&self)  ;
}

// Writer.java
/**
 * The base class for all objects which encode/generate a barcode image.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub trait Writer {

     /**
   * @param contents The contents to encode in the barcode
   * @param format The barcode format to generate
   * @param width The preferred width in pixels
   * @param height The preferred height in pixels
   * @param hints Additional parameters to supply to the encoder
   * @return {@link BitMatrix} representing encoded barcode image
   * @throws WriterException if contents cannot be encoded legally in a format
   */
    fn  encode<T>(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32, hints: &HashMap<EncodeHintType, T>) -> Result<BitMatrix, WriterException>  ;
 
}

// MultiFormatReader.java

/**
 * MultiFormatReader is a convenience class and the main entry point into the library for most uses.
 * By default it attempts to decode all barcode formats that the library supports. Optionally, you
 * can provide a hints object to request different behavior, for example only decoding QR codes.
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 */

const EMPTY_READER_ARRAY: [Option<dyn Reader>; 0] = [None; 0];
pub struct MultiFormatReader<T> {

      hints: HashMap<DecodeHintType, T>,

      readers: Vec<dyn Reader>
}

impl Reader for MultiFormatReader <T>{
    fn  decode<T>(&self,  image: &BinaryBitmap, hints:Option<&HashMap<DecodeHintType, T>>) -> Result<RXingResult, ReaderException>  {
        self.set_hints(&hints)
        Ok(self.decode_internal(image))
    }

    fn  reset(&self)   {
        if self.readers != null {
            for  let reader: Reader in self.readers {
                reader.reset();
            }
        }
    }
}

impl MultiFormatReader<T> {

    /**
   * Decode an image using the state set up by calling setHints() previously. Continuous scan
   * clients will get a <b>large</b> speed increase by using this instead of decode().
   *
   * @param image The pixel data to decode
   * @return The contents of the image
   * @throws NotFoundException Any errors which occurred
   */
    pub fn  decode_with_state(&self,  image: &BinaryBitmap) -> Result<RXingResult, NotFoundException>   {
        // Make sure to set up the default state so we don't crash
        if self.readers == null {
            self.set_hints(null);
        }
        return Ok(self.decode_internal(image));
    }

    /**
   * This method adds state to the MultiFormatReader. By setting the hints once, subsequent calls
   * to decodeWithState(image) can reuse the same set of readers without reallocating memory. This
   * is important for performance in continuous scan clients.
   *
   * @param hints The set of hints to use for subsequent calls to decode(image)
   */
    pub fn  set_hints<T>(&self,  hints: &HashMap<DecodeHintType, T>)   {
        self.hints = hints;
         let try_harder: bool = hints != null && hints.contains_key(DecodeHintType::TRY_HARDER);
         let formats: Collection<BarcodeFormat> =  if hints == null { null } else { hints.get(DecodeHintType::POSSIBLE_FORMATS) as Collection<BarcodeFormat> };
         let mut readers: Collection<Reader> = ArrayList<>::new();
        if formats != null {
             let add_one_d_reader: bool = formats.contains(BarcodeFormat::UPC_A) || formats.contains(BarcodeFormat::UPC_E) || formats.contains(BarcodeFormat::EAN_13) || formats.contains(BarcodeFormat::EAN_8) || formats.contains(BarcodeFormat::CODABAR) || formats.contains(BarcodeFormat::CODE_39) || formats.contains(BarcodeFormat::CODE_93) || formats.contains(BarcodeFormat::CODE_128) || formats.contains(BarcodeFormat::ITF) || formats.contains(BarcodeFormat::RSS_14) || formats.contains(BarcodeFormat::RSS_EXPANDED);
            // Put 1D readers upfront in "normal" mode
            if add_one_d_reader && !try_harder {
                readers.add(MultiFormatOneDReader::new(&hints));
            }
            if formats.contains(BarcodeFormat::QR_CODE) {
                readers.add(QRCodeReader::new());
            }
            if formats.contains(BarcodeFormat::DATA_MATRIX) {
                readers.add(DataMatrixReader::new());
            }
            if formats.contains(BarcodeFormat::AZTEC) {
                readers.add(AztecReader::new());
            }
            if formats.contains(BarcodeFormat::PDF_417) {
                readers.add(PDF417Reader::new());
            }
            if formats.contains(BarcodeFormat::MAXICODE) {
                readers.add(MaxiCodeReader::new());
            }
            // At end in "try harder" mode
            if add_one_d_reader && try_harder {
                readers.add(MultiFormatOneDReader::new(&hints));
            }
        }
        if readers.is_empty() {
            if !try_harder {
                readers.add(MultiFormatOneDReader::new(&hints));
            }
            readers.add(QRCodeReader::new());
            readers.add(DataMatrixReader::new());
            readers.add(AztecReader::new());
            readers.add(PDF417Reader::new());
            readers.add(MaxiCodeReader::new());
            if try_harder {
                readers.add(MultiFormatOneDReader::new(&hints));
            }
        }
        self.readers = readers.to_array(EMPTY_READER_ARRAY);
    }

    fn  decode_internal(&self,  image: &BinaryBitmap) -> Result<RXingResult, NotFoundException>   {
        if self.readers != null {
            for  let reader: Reader in self.readers {
                if Thread::current_thread()::is_interrupted() {
                    return Err( NotFoundException::get_not_found_instance() );
                }
                let tryResult1 = 0;
                    return reader.decode(image, &self.hints);
                

            }
            if self.hints != null && self.hints.contains_key(DecodeHintType::ALSO_INVERTED) {
                // Calling all readers again with inverted image
                image.get_black_matrix().flip();
                for  let reader: Reader in self.readers {
                    if Thread::current_thread()::is_interrupted() {
                        return Err( NotFoundException::get_not_found_instance());
                    }
                    let tryResult1 = 0;
                    
                        return reader.decode(image, &self.hints);
                    

                }
            }
        }
        Err(NotFoundException::get_not_found_instance())
    }
}

// MultiFormatWriter.java
/**
 * This is a factory class which finds the appropriate Writer subclass for the BarcodeFormat
 * requested and encodes the barcode with the supplied contents.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct MultiFormatWriter {
}

impl Writer for MultiFormatWriter {

     fn  encode<T>(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &HashMap<EncodeHintType, T>) -> Result<BitMatrix,WriterException>   {
         let mut writer: Writer;
        match format {
              EAN_8 => 
                 {
                    writer = EAN8Writer::new();
                }
              UPC_E => 
                 {
                    writer = UPCEWriter::new();
                }
              EAN_13 => 
                 {
                    writer = EAN13Writer::new();
                }
              UPC_A => 
                 {
                    writer = UPCAWriter::new();
                }
              QR_CODE => 
                 {
                    writer = QRCodeWriter::new();
                }
              CODE_39 => 
                 {
                    writer = Code39Writer::new();
                }
              CODE_93 => 
                 {
                    writer = Code93Writer::new();
                }
              CODE_128 => 
                 {
                    writer = Code128Writer::new();
                }
              ITF => 
                 {
                    writer = ITFWriter::new();
                }
              PDF_417 => 
                 {
                    writer = PDF417Writer::new();
                }
              CODABAR => 
                 {
                    writer = CodaBarWriter::new();
                }
              DATA_MATRIX => 
                 {
                    writer = DataMatrixWriter::new();
                }
              AZTEC => 
                 {
                    writer = AztecWriter::new();
                }
            _ => 
                 {
                    return Err(IllegalArgumentException::new(format!("No encoder available for format {}", format)));
                }
        }
        return Ok(writer.encode(&contents, format, width, height, &hints));
    }
}

// NotFoundException.java
pub struct NotFoundException;

// PlanarYUVLuminanceSource.java

const THUMBNAIL_SCALE_FACTOR: i32 = 2;

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

pub struct PlanarYUVLuminanceSource {
      yuv_data: Vec<i8>,

      data_width: i32,

      data_height: i32,

      left: i32,

      top: i32,
      width: i32,
      height: i32
}

impl LuminanceSource for PlanarYUVLuminanceSource {
     fn  get_row(&self,  y: i32,  row: &Vec<i8>) -> Vec<i8>  {
        if y < 0 || y >= get_height() {
            throw IllegalArgumentException::new(format!("Requested row is outside the image: {}", y));
        }
         let width: i32 = get_width();
        if row == null || row.len() < width {
            row = : [i8; width] = [0; width];
        }
         let offset: i32 = (y + self.top) * self.data_width + self.left;
        System::arraycopy(&self.yuv_data, offset, &row, 0, width);
        return row;
    }

     fn  get_matrix(&self) -> Vec<i8>  {
         let width: i32 = self.get_width();
         let height: i32 = self.get_height();
        // original data. The docs specifically warn that result.length must be ignored.
        if width == self.data_width && height == self.data_height {
            return self.yuv_data;
        }
         let area: i32 = width * height;
         let matrix: [i8; area] = [0; area];
         let input_offset: i32 = self.top * self.data_width + self.left;
        // If the width matches the full width of the underlying data, perform a single copy.
        if width == self.data_width {
            System::arraycopy(&self.yuv_data, input_offset, &matrix, 0, area);
            return matrix;
        }
        // Otherwise copy one cropped row at a time.
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let output_offset: i32 = y * width;
                    System::arraycopy(&self.yuv_data, input_offset, &matrix, output_offset, width);
                    input_offset += self.data_width;
                }
                y += 1;
             }
         }

        return matrix;
    }

     fn  is_crop_supported(&self) -> bool  {
        return true;
    }

     fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> dyn LuminanceSource  {
        return PlanarYUVLuminanceSource::new(&self.yuv_data, self.data_width, self.data_height, self.left + left, self.top + top, width, height, false);
    }
}

impl PlanarYUVLuminanceSource {

    pub fn new( yuv_data: &Vec<i8>,  data_width: i32,  data_height: i32,  left: i32,  top: i32,  width: i32,  height: i32,  reverse_horizontal: bool) -> Result<PlanarYUVLuminanceSource,IllegalArgumentException> {
        super(width, height);
        if left + width > data_width || top + height > data_height {
            throw IllegalArgumentException::new("Crop rectangle does not fit within image data.");
        }
        let .yuvData = yuv_data;
        let .dataWidth = data_width;
        let .dataHeight = data_height;
        let .left = left;
        let .top = top;
        if reverse_horizontal {
            self.reverse_horizontal(width, height);
        }
    }

    pub fn  render_thumbnail(&self) -> Vec<i32>  {
         let width: i32 = self.get_width() / THUMBNAIL_SCALE_FACTOR;
         let height: i32 = self.get_height() / THUMBNAIL_SCALE_FACTOR;
         let mut pixels: [i32; width * height] = [0; width * height];
         let yuv: Vec<i8> = self.yuv_data;
         let input_offset: i32 = self.top * self.data_width + self.left;
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let output_offset: i32 = y * width;
                     {
                         let mut x: i32 = 0;
                        while x < width {
                            {
                                 let grey: i32 = yuv[input_offset + x * THUMBNAIL_SCALE_FACTOR] & 0xff;
                                pixels[output_offset + x] = 0xFF000000 | (grey * 0x00010101);
                            }
                            x += 1;
                         }
                     }

                    input_offset += self.data_width * THUMBNAIL_SCALE_FACTOR;
                }
                y += 1;
             }
         }

        return pixels;
    }

    /**
   * @return width of image from {@link #renderThumbnail()}
   */
    pub fn  get_thumbnail_width(&self) -> i32  {
        return self.get_width() / THUMBNAIL_SCALE_FACTOR;
    }

    /**
   * @return height of image from {@link #renderThumbnail()}
   */
    pub fn  get_thumbnail_height(&self) -> i32  {
        return self.get_height() / THUMBNAIL_SCALE_FACTOR;
    }

    fn  reverse_horizontal(&self,  width: i32,  height: i32)   {
         let yuv_data: Vec<i8> = self.yuvData;
         {
             let mut y: i32 = 0, let row_start: i32 = self.top * self.data_width + self.left;
            while y < height {
                {
                     let middle: i32 = row_start + width / 2;
                     {
                         let mut x1: i32 = row_start, let mut x2: i32 = row_start + width - 1;
                        while x1 < middle {
                            {
                                 let temp: i8 = yuv_data[x1];
                                yuv_data[x1] = yuv_data[x2];
                                yuv_data[x2] = temp;
                            }
                            x1 += 1;
                            x2 -= 1;
                         }
                     }

                }
                y += 1;
                row_start += self.data_width;
             }
         }

    }
}

// Result.java
/**
 * <p>Encapsulates the result of decoding a barcode within an image.</p>
 *
 * @author Sean Owen
 */
pub struct RXingResult {

     text: String,

     raw_bytes: Vec<i8>,

     num_bits: i32,

     result_points: Vec<ResultPoint>,

     format: BarcodeFormat,

     result_metadata: HashMap<ResultMetadataType, Object>,

     timestamp: i64,
}

impl RXingResult {

   pub fn new( text: &String,  raw_bytes: &Vec<i8>,  result_points: &Vec<ResultPoint>,  format: &BarcodeFormat) -> Result {
       this(&text, &raw_bytes, result_points, format, &System::current_time_millis());
   }

   pub fn new( text: &String,  raw_bytes: &Vec<i8>,  result_points: &Vec<ResultPoint>,  format: &BarcodeFormat,  timestamp: i64) -> Result {
       this(&text, &raw_bytes,  if raw_bytes == null { 0 } else { 8 * raw_bytes.len() }, result_points, format, timestamp);
   }

   pub fn new( text: &String,  raw_bytes: &Vec<i8>,  num_bits: i32,  result_points: &Vec<ResultPoint>,  format: &BarcodeFormat,  timestamp: i64) -> Result {
       let .text = text;
       let .rawBytes = raw_bytes;
       let .numBits = num_bits;
       let .resultPoints = result_points;
       let .format = format;
       let .resultMetadata = null;
       let .timestamp = timestamp;
   }

   /**
  * @return raw text encoded by the barcode
  */
   pub fn  get_text(&self) -> String  {
       return self.text;
   }

   /**
  * @return raw bytes encoded by the barcode, if applicable, otherwise {@code null}
  */
   pub fn  get_raw_bytes(&self) -> Vec<i8>  {
       return self.raw_bytes;
   }

   /**
  * @return how many bits of {@link #getRawBytes()} are valid; typically 8 times its length
  * @since 3.3.0
  */
   pub fn  get_num_bits(&self) -> i32  {
       return self.num_bits;
   }

   /**
  * @return points related to the barcode in the image. These are typically points
  *         identifying finder patterns or the corners of the barcode. The exact meaning is
  *         specific to the type of barcode that was decoded.
  */
   pub fn  get_result_points(&self) -> Vec<ResultPoint>  {
       return self.result_points;
   }

   /**
  * @return {@link BarcodeFormat} representing the format of the barcode that was decoded
  */
   pub fn  get_barcode_format(&self) -> BarcodeFormat  {
       return self.format;
   }

   /**
  * @return {@link Map} mapping {@link ResultMetadataType} keys to values. May be
  *   {@code null}. This contains optional metadata about what was detected about the barcode,
  *   like orientation.
  */
   pub fn  get_result_metadata(&self) -> HashMap<ResultMetadataType, Object>  {
       return self.result_metadata;
   }

   pub fn  put_metadata(&self,  type: &ResultMetadataType,  value: &Object)   {
       if self.result_metadata == null {
           self.result_metadata = EnumMap<>::new(ResultMetadataType.class);
       }
       self.result_metadata.put(type, &value);
   }

   pub fn  put_all_metadata(&self,  metadata: &HashMap<ResultMetadataType, Object>)   {
       if metadata != null {
           if self.result_metadata == null {
               self.result_metadata = metadata;
           } else {
               self.result_metadata.put_all(&metadata);
           }
       }
   }

   pub fn  add_result_points(&self,  new_points: &Vec<ResultPoint>)   {
        let old_points: Vec<ResultPoint> = self.result_points;
       if old_points == null {
           self.result_points = new_points;
       } else if new_points != null && new_points.len() > 0 {
            let all_points: [Option<ResultPoint>; old_points.len() + new_points.len()] = [None; old_points.len() + new_points.len()];
           System::arraycopy(old_points, 0, all_points, 0, old_points.len());
           System::arraycopy(new_points, 0, all_points, old_points.len(), new_points.len());
           self.result_points = all_points;
       }
   }

   pub fn  get_timestamp(&self) -> i64  {
       return self.timestamp;
   }

   pub fn  to_string(&self) -> String  {
       return self.text;
   }
}

// ResultMetadataType.java
/**
 * Represents some type of metadata about the result of the decoding that the decoder
 * wishes to communicate back to the caller.
 *
 * @author Sean Owen
 */
pub enum ResultMetadataType {

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
   * which is sometimes used to encode binary data. While {@link Result} makes available
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
    SYMBOLOGY_IDENTIFIER
}

// ResultPoint.java
/**
 * <p>Encapsulates a point of interest in an image containing a barcode. Typically, this
 * would be the location of a finder pattern or the corner of the barcode, for example.</p>
 *
 * @author Sean Owen
 */
pub struct ResultPoint {

     x: f32,

     y: f32
}

impl ResultPoint {

   pub fn new( x: f32,  y: f32) -> ResultPoint {
       let .x = x;
       let .y = y;
   }

   pub fn  get_x(&self) -> f32  {
       return self.x;
   }

   pub fn  get_y(&self) -> f32  {
       return self.y;
   }

   pub fn  equals(&self,  other: &Object) -> bool  {
       if other instanceof ResultPoint {
            let other_point: ResultPoint = other as ResultPoint;
           return self.x == other_point.x && self.y == other_point.y;
       }
       return false;
   }

   pub fn  hash_code(&self) -> i32  {
       return 31 * Float::float_to_int_bits(self.x) + Float::float_to_int_bits(self.y);
   }

   pub fn  to_string(&self) -> String  {
       return format!("({},{})", self.x, self.y);
   }

   /**
  * Orders an array of three ResultPoints in an order [A,B,C] such that AB is less than AC
  * and BC is less than AC, and the angle between BC and BA is less than 180 degrees.
  *
  * @param patterns array of three {@code ResultPoint} to order
  */
   pub fn  order_best_patterns( patterns: &Vec<ResultPoint>)   {
       // Find distances between pattern centers
        let zero_one_distance: f32 = common::detector::MathUtils::distance(patterns[0], patterns[1]);
        let one_two_distance: f32 = common::detector::MathUtils::distance(patterns[1], patterns[2]);
        let zero_two_distance: f32 = common::detector::MathUtils::distance(patterns[0], patterns[2]);
        let point_a: ResultPoint;
        let point_b: ResultPoint;
        let point_c: ResultPoint;
       // Assume one closest to other two is B; A and C will just be guesses at first
       if one_two_distance >= zero_one_distance && one_two_distance >= zero_two_distance {
           point_b = patterns[0];
           point_a = patterns[1];
           point_c = patterns[2];
       } else if zero_two_distance >= one_two_distance && zero_two_distance >= zero_one_distance {
           point_b = patterns[1];
           point_a = patterns[0];
           point_c = patterns[2];
       } else {
           point_b = patterns[2];
           point_a = patterns[0];
           point_c = patterns[1];
       }
       // should swap A and C.
       if common::detector::MathUtils::cross_product_z(point_a, point_b, point_c) < 0.0f {
            let temp: ResultPoint = point_a;
           point_a = point_c;
           point_c = temp;
       }
       patterns[0] = point_a;
       patterns[1] = point_b;
       patterns[2] = point_c;
   }

   /**
  * @param pattern1 first pattern
  * @param pattern2 second pattern
  * @return distance between two points
  */
   pub fn  distance( pattern1: &ResultPoint,  pattern2: &ResultPoint) -> f32  {
       return common::detector::MathUtils::distance(pattern1.x, pattern1.y, pattern2.x, pattern2.y);
   }

   /**
  * Returns the z component of the cross product between vectors BC and BA.
  */
   fn  cross_product_z( point_a: &ResultPoint,  point_b: &ResultPoint,  point_c: &ResultPoint) -> f32  {
        let b_x: f32 = point_b.x;
        let b_y: f32 = point_b.y;
       return ((point_c.x - b_x) * (point_a.y - b_y)) - ((point_c.y - b_y) * (point_a.x - b_x));
   }
}

// ResultPointCallback.java
/**
 * Callback which is invoked when a possible result point (significant
 * point in the barcode image such as a corner) is found.
 *
 * @see DecodeHintType#NEED_RESULT_POINT_CALLBACK
 */
pub trait ResultPointCallback {

    fn  found_possible_result_point(&self,  point: &ResultPoint)  ;
}

// RGBLuminanceSource.java
/**
 * This class is used to help decode images from files which arrive as RGB data from
 * an ARGB pixel array. It does not support rotation.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Betaminos
 */
pub struct RGBLuminanceSource {
       luminances: Vec<i8>,

      data_width: i32,

      data_height: i32,

       left: i32,

       top: i32
}

impl LuminanceSource for RGBLuminanceSource {
     fn  get_row(&self,  y: i32,  row: &Vec<i8>) -> Vec<i8>  {
        if y < 0 || y >= get_height() {
            throw IllegalArgumentException::new(format!("Requested row is outside the image: {}", y));
        }
         let width: i32 = get_width();
        if row == null || row.len() < width {
            row = : [i8; width] = [0; width];
        }
         let offset: i32 = (y + self.top) * self.data_width + self.left;
        System::arraycopy(&self.luminances, offset, &row, 0, width);
        return row;
    }

     fn  get_matrix(&self) -> Vec<i8>  {
         let width: i32 = get_width();
         let height: i32 = get_height();
        // original data. The docs specifically warn that result.length must be ignored.
        if width == self.data_width && height == self.data_height {
            return self.luminances;
        }
         let area: i32 = width * height;
         let matrix: [i8; area] = [0; area];
         let input_offset: i32 = self.top * self.data_width + self.left;
        // If the width matches the full width of the underlying data, perform a single copy.
        if width == self.data_width {
            System::arraycopy(&self.luminances, input_offset, &matrix, 0, area);
            return matrix;
        }
        // Otherwise copy one cropped row at a time.
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let output_offset: i32 = y * width;
                    System::arraycopy(&self.luminances, input_offset, &matrix, output_offset, width);
                    input_offset += self.data_width;
                }
                y += 1;
             }
         }

        return matrix;
    }

     fn  is_crop_supported(&self) -> bool  {
        return true;
    }

     fn  crop(&self,  left: i32,  top: i32,  width: i32,  height: i32) -> LuminanceSource  {
        return RGBLuminanceSource::new(&self.luminances, self.data_width, self.data_height, self.left + left, self.top + top, width, height);
    }
}

impl RGBLuminanceSource {

    pub fn new( width: i32,  height: i32,  pixels: &Vec<i32>) -> RGBLuminanceSource {
        super(width, height);
        data_width = width;
        data_height = height;
        left = 0;
        top = 0;
        // In order to measure pure decoding speed, we convert the entire image to a greyscale array
        // up front, which is the same as the Y channel of the YUVLuminanceSource in the real app.
        //
        // Total number of pixels suffices, can ignore shape
         let size: i32 = width * height;
        luminances = : [i8; size] = [0; size];
         {
             let mut offset: i32 = 0;
            while offset < size {
                {
                     let pixel: i32 = pixels[offset];
                    // red
                     let r: i32 = (pixel >> 16) & 0xff;
                    // 2 * green
                     let g2: i32 = (pixel >> 7) & 0x1fe;
                    // blue
                     let b: i32 = pixel & 0xff;
                    // Calculate green-favouring average cheaply
                    luminances[offset] = ((r + g2 + b) / 4) as i8;
                }
                offset += 1;
             }
         }

    }

    fn new( pixels: &Vec<i8>,  data_width: i32,  data_height: i32,  left: i32,  top: i32,  width: i32,  height: i32) -> RGBLuminanceSource {
        super(width, height);
        if left + width > data_width || top + height > data_height {
            throw IllegalArgumentException::new("Crop rectangle does not fit within image data.");
        }
        let .luminances = pixels;
        let .dataWidth = data_width;
        let .dataHeight = data_height;
        let .left = left;
        let .top = top;
    }
}

// WriterException.java
/**
 * A base class which covers the range of exceptions which may occur when encoding a barcode using
 * the Writer framework.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct WriterException;