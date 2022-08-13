use crate::{BinaryBitmap,DecodeHintType,NotFoundException,RXingResult,Reader,ReaderException,ResultPoint,ChecksumException,FormatException};

// ByQuadrantReader.java
/**
 * This class attempts to decode a barcode from an image, not by scanning the whole image,
 * but by scanning subsets of the image. This is important when there may be multiple barcodes in
 * an image, and detecting a barcode may find parts of multiple barcode and fail to decode
 * (e.g. QR Codes). Instead this scans the four quadrants of the image -- and also the center
 * 'quadrant' to cover the case where a barcode is found in the center.
 *
 * @see GenericMultipleBarcodeReader
 */
pub struct ByQuadrantReader {

     let delegate: Reader;
}

impl Reader for ByQuadrantReader {
    pub fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(self.decode(image, null));
    }

    pub fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException, ChecksumException, FormatException */Result<Result, Rc<Exception>>   {
         let width: i32 = image.get_width();
         let height: i32 = image.get_height();
         let half_width: i32 = width / 2;
         let half_height: i32 = height / 2;
        let tryResult1 = 0;
        'try1: loop {
        {
            // No need to call makeAbsolute as results will be relative to original top left here
            return Ok(self.delegate.decode(&image.crop(0, 0, half_width, half_height), &hints));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &NotFoundException) {
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
             let result: Result = self.delegate.decode(&image.crop(half_width, 0, half_width, half_height), &hints);
            ::make_absolute(&result.get_result_points(), half_width, 0);
            return Ok(result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &NotFoundException) {
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
             let result: Result = self.delegate.decode(&image.crop(0, half_height, half_width, half_height), &hints);
            ::make_absolute(&result.get_result_points(), 0, half_height);
            return Ok(result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &NotFoundException) {
            }  0 => break
        }

        let tryResult1 = 0;
        'try1: loop {
        {
             let result: Result = self.delegate.decode(&image.crop(half_width, half_height, half_width, half_height), &hints);
            ::make_absolute(&result.get_result_points(), half_width, half_height);
            return Ok(result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( re: &NotFoundException) {
            }  0 => break
        }

         let quarter_width: i32 = half_width / 2;
         let quarter_height: i32 = half_height / 2;
         let center: BinaryBitmap = image.crop(quarter_width, quarter_height, half_width, half_height);
         let result: Result = self.delegate.decode(center, &hints);
        ::make_absolute(&result.get_result_points(), quarter_width, quarter_height);
        return Ok(result);
    }

    pub fn  reset(&self)   {
        self.delegate.reset();
    }
}

impl ByQuadrantReader {

    pub fn new( delegate: &Reader) -> ByQuadrantReader {
        let .delegate = delegate;
    }

    

    fn  make_absolute( points: &Vec<ResultPoint>,  left_offset: i32,  top_offset: i32)   {
        if points != null {
             {
                 let mut i: i32 = 0;
                while i < points.len() {
                    {
                         let relative: ResultPoint = points[i];
                        if relative != null {
                            points[i] = ResultPoint::new(relative.get_x() + left_offset, relative.get_y() + top_offset);
                        }
                    }
                    i += 1;
                 }
             }

        }
    }
}


// MultipleBarcodeReader.java
/**
 * Implementation of this interface attempt to read several barcodes from one image.
 *
 * @see com.google.zxing.Reader
 * @author Sean Owen
 */
pub trait MultipleBarcodeReader {

    fn  decode_multiple(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>  ;

    fn  decode_multiple(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>  ;
}

// GenericMultipleBarcodeReader.java
/**
 * <p>Attempts to locate multiple barcodes in an image by repeatedly decoding portion of the image.
 * After one barcode is found, the areas left, above, right and below the barcode's
 * {@link ResultPoint}s are scanned, recursively.</p>
 *
 * <p>A caller may want to also employ {@link ByQuadrantReader} when attempting to find multiple
 * 2D barcodes, like QR Codes, in an image, where the presence of multiple barcodes might prevent
 * detecting any one of them.</p>
 *
 * <p>That is, instead of passing a {@link Reader} a caller might pass
 * {@code new ByQuadrantReader(reader)}.</p>
 *
 * @author Sean Owen
 */

const MIN_DIMENSION_TO_RECUR: i32 = 100;

const MAX_DEPTH: i32 = 4;

const EMPTY_RESULT_ARRAY: [Option<Result>; 0] = [None; 0];

pub struct GenericMultipleBarcodeReader {

    let delegate: Reader;
}

impl MultipleBarcodeReader for GenericMultipleBarcodeReader {
    pub fn  decode_multiple(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>   {
        return Ok(self.decode_multiple(image, null));
    }
 
    pub fn  decode_multiple(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>   {
         let results: List<Result> = ArrayList<>::new();
        self.do_decode_multiple(image, &hints, &results, 0, 0, 0);
        if results.is_empty() {
            throw NotFoundException::get_not_found_instance();
        }
        return Ok(results.to_array(EMPTY_RESULT_ARRAY));
    }
}

impl GenericMultipleBarcodeReader {

   pub fn new( delegate: &Reader) -> GenericMultipleBarcodeReader {
       let .delegate = delegate;
   }


   fn  do_decode_multiple(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>,  results: &List<Result>,  x_offset: i32,  y_offset: i32,  current_depth: i32)   {
       if current_depth > MAX_DEPTH {
           return;
       }
        let mut result: Result;
       let tryResult1 = 0;
       'try1: loop {
       {
           result = self.delegate.decode(image, &hints);
       }
       break 'try1
       }
       match tryResult1 {
            catch ( ignored: &ReaderException) {
               return;
           }  0 => break
       }

        let already_found: bool = false;
       for  let existing_result: Result in results {
           if existing_result.get_text().equals(&result.get_text()) {
               already_found = true;
               break;
           }
       }
       if !already_found {
           results.add(&::translate_result_points(result, x_offset, y_offset));
       }
        let result_points: Vec<ResultPoint> = result.get_result_points();
       if result_points == null || result_points.len() == 0 {
           return;
       }
        let width: i32 = image.get_width();
        let height: i32 = image.get_height();
        let min_x: f32 = width;
        let min_y: f32 = height;
        let max_x: f32 = 0.0f;
        let max_y: f32 = 0.0f;
       for  let point: ResultPoint in result_points {
           if point == null {
               continue;
           }
            let x: f32 = point.get_x();
            let y: f32 = point.get_y();
           if x < min_x {
               min_x = x;
           }
           if y < min_y {
               min_y = y;
           }
           if x > max_x {
               max_x = x;
           }
           if y > max_y {
               max_y = y;
           }
       }
       // Decode left of barcode
       if min_x > MIN_DIMENSION_TO_RECUR {
           self.do_decode_multiple(&image.crop(0, 0, min_x as i32, height), &hints, &results, x_offset, y_offset, current_depth + 1);
       }
       // Decode above barcode
       if min_y > MIN_DIMENSION_TO_RECUR {
           self.do_decode_multiple(&image.crop(0, 0, width, min_y as i32), &hints, &results, x_offset, y_offset, current_depth + 1);
       }
       // Decode right of barcode
       if max_x < width - MIN_DIMENSION_TO_RECUR {
           self.do_decode_multiple(&image.crop(max_x as i32, 0, width - max_x as i32, height), &hints, &results, x_offset + max_x as i32, y_offset, current_depth + 1);
       }
       // Decode below barcode
       if max_y < height - MIN_DIMENSION_TO_RECUR {
           self.do_decode_multiple(&image.crop(0, max_y as i32, width, height - max_y as i32), &hints, &results, x_offset, y_offset + max_y as i32, current_depth + 1);
       }
   }

   fn  translate_result_points( result: &Result,  x_offset: i32,  y_offset: i32) -> Result  {
        let old_result_points: Vec<ResultPoint> = result.get_result_points();
       if old_result_points == null {
           return result;
       }
        let new_result_points: [Option<ResultPoint>; old_result_points.len()] = [None; old_result_points.len()];
        {
            let mut i: i32 = 0;
           while i < old_result_points.len() {
               {
                    let old_point: ResultPoint = old_result_points[i];
                   if old_point != null {
                       new_result_points[i] = ResultPoint::new(old_point.get_x() + x_offset, old_point.get_y() + y_offset);
                   }
               }
               i += 1;
            }
        }

        let new_result: Result = Result::new(&result.get_text(), &result.get_raw_bytes(), &result.get_num_bits(), new_result_points, &result.get_barcode_format(), &result.get_timestamp());
       new_result.put_all_metadata(&result.get_result_metadata());
       return new_result;
   }
}
