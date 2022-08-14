use crate::{BarcodeFormat,BinaryBitmap,DecodeHintType,NotFoundException,ReaderException,RXingResult,ResultMetadataType,ResultPoint};
use crate::common::{DecoderResult,DetectorResult};
use crate::multi::{MultipleBarcodeReader};
use crate::multi::qrcode::detector::MultiDetector;
use create::qrcode::{QRCodeReader};
use crate::multi::qrcode::decoder::QRCodeDecoderMetaData;


// QRCodeMultiReader.java
/**
 * This implementation can detect and decode multiple QR Codes in an image.
 *
 * @author Sean Owen
 * @author Hannes Erven
 */

const EMPTY_RESULT_ARRAY: [Option<Result>; 0] = [None; 0];

const NO_POINTS: [Option<ResultPoint>; 0] = [None; 0];
pub struct QRCodeMultiReader {
   super: QRCodeReader;
}

impl MultipleBarcodeReader for QRCodeMultiReader {
    pub fn  decode_multiple(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>   {
        return Ok(self.decode_multiple(image, null));
    }
 
    pub fn  decode_multiple(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, ?>) -> /*  throws NotFoundException */Result<Vec<Result>, Rc<Exception>>   {
         let mut results: List<Result> = ArrayList<>::new();
         let detector_results: Vec<DetectorResult> = MultiDetector::new(&image.get_black_matrix()).detect_multi(&hints);
        for  let detector_result: DetectorResult in detector_results {
            let tryResult1 = 0;
            'try1: loop {
            {
                 let decoder_result: DecoderResult = get_decoder().decode(&detector_result.get_bits(), &hints);
                 let points: Vec<ResultPoint> = detector_result.get_points();
                // If the code was mirrored: swap the bottom-left and the top-right points.
                if decoder_result.get_other() instanceof QRCodeDecoderMetaData {
                    (decoder_result.get_other() as QRCodeDecoderMetaData).apply_mirrored_correction(points);
                }
                 let result: Result = Result::new(&decoder_result.get_text(), &decoder_result.get_raw_bytes(), points, BarcodeFormat::QR_CODE);
                 let byte_segments: List<Vec<i8>> = decoder_result.get_byte_segments();
                if byte_segments != null {
                    result.put_metadata(ResultMetadataType::BYTE_SEGMENTS, &byte_segments);
                }
                 let ec_level: String = decoder_result.get_e_c_level();
                if ec_level != null {
                    result.put_metadata(ResultMetadataType::ERROR_CORRECTION_LEVEL, &ec_level);
                }
                if decoder_result.has_structured_append() {
                    result.put_metadata(ResultMetadataType::STRUCTURED_APPEND_SEQUENCE, &decoder_result.get_structured_append_sequence_number());
                    result.put_metadata(ResultMetadataType::STRUCTURED_APPEND_PARITY, &decoder_result.get_structured_append_parity());
                }
                results.add(result);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( re: &ReaderException) {
                }  0 => break
            }
 
        }
        if results.is_empty() {
            return Ok(EMPTY_RESULT_ARRAY);
        } else {
            results = ::process_structured_append(&results);
            return Ok(results.to_array(EMPTY_RESULT_ARRAY));
        }
    }
}

impl QRCodeMultiReader {

  

   fn  process_structured_append( results: &List<Result>) -> List<Result>  {
        let new_results: List<Result> = ArrayList<>::new();
        let sa_results: List<Result> = ArrayList<>::new();
       for  let result: Result in results {
           if result.get_result_metadata().contains_key(ResultMetadataType::STRUCTURED_APPEND_SEQUENCE) {
               sa_results.add(result);
           } else {
               new_results.add(result);
           }
       }
       if sa_results.is_empty() {
           return results;
       }
       // sort and concatenate the SA list items
       Collections::sort(&sa_results, SAComparator::new());
        let new_text: StringBuilder = StringBuilder::new();
        let new_raw_bytes: ByteArrayOutputStream = ByteArrayOutputStream::new();
        let new_byte_segment: ByteArrayOutputStream = ByteArrayOutputStream::new();
       for  let sa_result: Result in sa_results {
           new_text.append(&sa_result.get_text());
            let sa_bytes: Vec<i8> = sa_result.get_raw_bytes();
           new_raw_bytes.write(&sa_bytes, 0, sa_bytes.len());
            let byte_segments: Iterable<Vec<i8>> = sa_result.get_result_metadata().get(ResultMetadataType::BYTE_SEGMENTS) as Iterable<Vec<i8>>;
           if byte_segments != null {
               for  let segment: Vec<i8> in byte_segments {
                   new_byte_segment.write(&segment, 0, segment.len());
               }
           }
       }
        let new_result: Result = Result::new(&new_text.to_string(), &new_raw_bytes.to_byte_array(), NO_POINTS, BarcodeFormat::QR_CODE);
       if new_byte_segment.size() > 0 {
           new_result.put_metadata(ResultMetadataType::BYTE_SEGMENTS, &Collections::singleton_list(&new_byte_segment.to_byte_array()));
       }
       new_results.add(new_result);
       return new_results;
   }

   #[derive(Comparator<Result>, Serializable)]
   struct SAComparator {
   }
   
   impl SAComparator {

       pub fn  compare(&self,  a: &Result,  b: &Result) -> i32  {
            let a_number: i32 = a.get_result_metadata().get(ResultMetadataType::STRUCTURED_APPEND_SEQUENCE) as i32;
            let b_number: i32 = b.get_result_metadata().get(ResultMetadataType::STRUCTURED_APPEND_SEQUENCE) as i32;
           return Integer::compare(a_number, b_number);
       }
   }

}

