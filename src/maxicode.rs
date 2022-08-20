pub mod decoder;

use crate::{BarcodeFormat,BinaryBitmap,ChecksumException,DecodeHintType,FormatException,NotFoundException,Reader,XRingResultResultMetadataType,ResultPoint};
use crate::common::{BitMatrix,DecoderResult};
use crate::maxicode::decoder::Decoder;

// MaxiCodeReader.java

/**
 * This implementation can detect and decode a MaxiCode in an image.
 */

const NO_POINTS: [Option<ResultPoint>; 0] = [None; 0];

const MATRIX_WIDTH: i32 = 30;

const MATRIX_HEIGHT: i32 = 33;
pub struct MaxiCodeReader {

     decoder: Decoder
}

impl Reader for MaxiCodeReader{
/**
  * Locates and decodes a MaxiCode in an image.
  *
  * @return a String representing the content encoded by the MaxiCode
  * @throws NotFoundException if a MaxiCode cannot be found
  * @throws FormatException if a MaxiCode cannot be decoded
  * @throws ChecksumException if error correction fails
  */

 fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, _>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
    // Note that MaxiCode reader effectively always assumes PURE_BARCODE mode
    // and can't detect it in an image
     let bits: BitMatrix = ::extract_pure_bits(&image.get_black_matrix());
     let decoder_result: DecoderResult = self.decoder.decode(&bits, &hints);
     let result: Result = Result::new(&decoder_result.get_text(), &decoder_result.get_raw_bytes(), NO_POINTS, BarcodeFormat::MAXICODE);
     let ec_level: String = decoder_result.get_e_c_level();
    if ec_level != null {
        result.put_metadata(ResultMetadataType::ERROR_CORRECTION_LEVEL, &ec_level);
    }
    return Ok(result);
}

 fn  reset(&self)   {
// do nothing
}
}

impl MaxiCodeReader {

   pub fn new() -> Self {
    Self { decoder: Decoder::new() }
   }

   /**
  * This method detects a code in a "pure" image -- that is, pure monochrome image
  * which contains only an unrotated, unskewed, image of a code, with some white border
  * around it. This is a specialized method that works exceptionally fast in this special
  * case.
  */
   fn  extract_pure_bits( image: &BitMatrix) -> Result<BitMatrix,NotFoundException>   {
        let enclosing_rectangle: Vec<i32> = image.get_enclosing_rectangle();
       if enclosing_rectangle == null {
           return Err( NotFoundException::get_not_found_instance());
       }
        let left: i32 = enclosing_rectangle[0];
        let top: i32 = enclosing_rectangle[1];
        let width: i32 = enclosing_rectangle[2];
        let height: i32 = enclosing_rectangle[3];
       // Now just read off the bits
        let bits: BitMatrix = BitMatrix::new(MATRIX_WIDTH, MATRIX_HEIGHT);
        {
            let mut y: i32 = 0;
           while y < MATRIX_HEIGHT {
               {
                    let iy: i32 = Math::min(top + (y * height + height / 2) / MATRIX_HEIGHT, height - 1);
                    {
                        let mut x: i32 = 0;
                       while x < MATRIX_WIDTH {
                           {
                               // srowen: I don't quite understand why the formula below is necessary, but it
                               // can walk off the image if left + width = the right boundary. So cap it.
                                let ix: i32 = left + Math::min((x * width + width / 2 + (y & 0x01) * width / 2) / MATRIX_WIDTH, width - 1);
                               if image.get(ix, iy) {
                                   bits.set(x, y);
                               }
                           }
                           x += 1;
                        }
                    }

               }
               y += 1;
            }
        }

       return Ok(bits);
   }
}

