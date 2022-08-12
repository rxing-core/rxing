use std::fmt;

use common::{BitArray,BitMatrix};

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
    fn get_luminance_source(&self) -> LuminanceSource;

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
    fn create_binarizer(&self, source: &LuminanceSource) -> Binarizer;

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
    binarizer: Binarizer,
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