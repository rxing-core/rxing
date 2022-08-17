pub mod decoder; 
pub mod detector;
pub mod encoder;

use crate::{ResultPoint,BarcodeFormat,EncodeHintType,Writer,Reader,ReaderException,WriterException};
use crate::common::{BitMatrix,DetectorResult,DecoderResult};
use crate::{BarcodeFormat,BinaryBitmap,DecodeHintType,FormatException,NotFoundException,Reader,Result,ResultMetadataType,ResultPoint,ResultPointCallback};
use crate::aztec::decoder::Decoder;
use crate::aztec::detector::Detector;

use crate::aztec::encoder::{AztecCode,Encoder};


// AztecDetectorResult.java
/**
 * <p>Extends {@link DetectorResult} with more information specific to the Aztec format,
 * like the number of layers and whether it's compact.</p>
 *
 * @author Sean Owen
 */
pub struct AztecDetectorResult {
    //super: DetectorResult;

      compact: bool,

      nb_datablocks: i32,

      nb_layers: i32,

      bits: BitMatrix,

      points: Vec<ResultPoint>,
}

impl DetectorResult for AztecDetectorResult {
     fn get_bits(&self) -> BitMatrix {
        return self.bits;
    }

     fn get_points(&self) -> Vec<ResultPoint> {
        return self.points;
    }
}

impl AztecDetectorResult {

    pub fn new( bits: &BitMatrix,  points: &Vec<ResultPoint>,  compact: bool,  nb_datablocks: i32,  nb_layers: i32) -> Self {
        Self { compact: compact, nb_datablocks: nd_datablocks, nb_layers: nb_layers, bits: bits, points: points }
    }

    pub fn  get_nb_layers(&self) -> i32  {
        return self.nb_layers;
    }

    pub fn  get_nb_datablocks(&self) -> i32  {
        return self.nb_datablocks;
    }

    pub fn  is_compact(&self) -> bool  {
        return self.compact;
    }
}

// AztecReader.java

/**
 * This implementation can detect and decode Aztec codes in an image.
 *
 * @author David Olivier
 */
pub struct AztecReader {
}

impl Reader for AztecReader {

    /**
   * Locates and decodes a Data Matrix code in an image.
   *
   * @return a String representing the content encoded by the Data Matrix code
   * @throws NotFoundException if a Data Matrix code cannot be found
   * @throws FormatException if a Data Matrix code cannot be decoded
   */
     /*fn  decode(&self,  image: &BinaryBitmap) -> /*  throws NotFoundException, FormatException */Result<Result, Rc<Exception>>   {
        return Ok(self.decode(image, null));
    }*/

     fn  decode(&self,  image: &BinaryBitmap,  hints: &Map<DecodeHintType, _>) -> Result<Result, ReaderException>   {
         let not_found_exception: NotFoundException = null;
         let format_exception: FormatException = null;
         let detector: Detector = Detector::new(&image.get_black_matrix());
         let mut points: Vec<ResultPoint> = null;
         let decoder_result: DecoderResult = null;

         let detector_result: AztecDetectorResult = detector.detect(Some(false))?;
            points = detector_result.get_points()?;
            decoder_result = Decoder::new().decode(detector_result);
         /*
         let tryResult1 = 0;
        'try1: loop {
        {
             let detector_result: AztecDetectorResult = detector.detect(Some(false));
            points = detector_result.get_points();
            decoder_result = Decoder::new().decode(detector_result);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( e: &NotFoundException) {
                not_found_exception = e;
            } catch ( e: &FormatException) {
                format_exception = e;
            }  0 => break
        }

        if decoder_result == null {
            let tryResult1 = 0;
            'try1: loop {
            {
                 let detector_result: AztecDetectorResult = detector.detect(true);
                points = detector_result.get_points();
                decoder_result = Decoder::new().decode(detector_result);
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( e: &NotFoundExceptionFormatException | ) {
                    if not_found_exception != null {
                        throw not_found_exception;
                    }
                    if format_exception != null {
                        throw format_exception;
                    }
                    throw e;
                }  0 => break
            }

        }
        */
        if hints != null {
             let rpcb: ResultPointCallback = hints.get(DecodeHintType::NEED_RESULT_POINT_CALLBACK) as ResultPointCallback;
            if rpcb != null {
                for   point in points {
                    rpcb.found_possible_result_point(&point);
                }
            }
        }
         let result: Result = Result::new(&decoder_result.get_text(), &decoder_result.get_raw_bytes(), &decoder_result.get_num_bits(), points, BarcodeFormat::AZTEC, &System::current_time_millis());
         let byte_segments: List<Vec<i8>> = decoder_result.get_byte_segments();
        if byte_segments != null {
            result.put_metadata(ResultMetadataType::BYTE_SEGMENTS, &byte_segments);
        }
         let ec_level: String = decoder_result.get_e_c_level();
        if ec_level != null {
            result.put_metadata(ResultMetadataType::ERROR_CORRECTION_LEVEL, &ec_level);
        }
        result.put_metadata(ResultMetadataType::SYMBOLOGY_IDENTIFIER, format!("]z{}", decoder_result.get_symbology_modifier()));
        return Ok(result);
    }

     fn  reset(&self)   {
    // do nothing
    }
}

// AztecWriter.java

/**
 * Renders an Aztec code as a {@link BitMatrix}.
 */
pub struct AztecWriter {
}

impl Writer for AztecWriter {

     fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: Option<&HashMap<EncodeHintType, _>>) -> BitMatrix  {
        // Do not add any ECI code by default
         let mut charset: Charset = null;
         let ecc_percent: i32 = Encoder::DEFAULT_EC_PERCENT;
         let mut layers: i32 = Encoder::DEFAULT_AZTEC_LAYERS;
        if hints != null {
            if hints.contains_key(EncodeHintType::CHARACTER_SET) {
                charset = Charset::for_name(&hints.get(EncodeHintType::CHARACTER_SET).to_string());
            }
            if hints.contains_key(EncodeHintType::ERROR_CORRECTION) {
                ecc_percent = Integer::parse_int(&hints.get(EncodeHintType::ERROR_CORRECTION).to_string());
            }
            if hints.contains_key(EncodeHintType::AZTEC_LAYERS) {
                layers = Integer::parse_int(&hints.get(EncodeHintType::AZTEC_LAYERS).to_string());
            }
        }
        return ::encode(&contents, format, width, height, &charset, ecc_percent, layers);
    }
/*
    fn  encode( contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  charset: &Charset,  ecc_percent: i32,  layers: i32) -> BitMatrix  {
        if format != BarcodeFormat::AZTEC {
            return Err( IllegalArgumentException::new(format!("Can only encode AZTEC, but got {}", format)));
        }
         let aztec: AztecCode = Encoder::encode(&contents, ecc_percent, layers, &charset);
        return ::render_result(aztec, width, height);
    }*/

    
}

impl AztecWriter {
    fn  render_result( code: &AztecCode,  width: i32,  height: i32) -> BitMatrix  {
        let input: BitMatrix = code.get_matrix();
       if input == null {
           return Err( IllegalStateException::new());
       }
        let input_width: i32 = input.get_width();
        let input_height: i32 = input.get_height();
        let output_width: i32 = Math::max(width, input_width);
        let output_height: i32 = Math::max(height, input_height);
        let multiple: i32 = Math::min(output_width / input_width, output_height / input_height);
        let left_padding: i32 = (output_width - (input_width * multiple)) / 2;
        let top_padding: i32 = (output_height - (input_height * multiple)) / 2;
        let output: BitMatrix = BitMatrix::new(output_width, output_height);
        {
            let input_y: i32 = 0;
             let output_y: i32 = top_padding;
           while input_y < input_height {
               {
                   // Write the contents of this row of the barcode
                    {
                        let input_x: i32 = 0;
                         let output_x: i32 = left_padding;
                       while input_x < input_width {
                           {
                               if input.get(input_x, input_y) {
                                   output.set_region(output_x, output_y, multiple, multiple);
                               }
                           }
                           input_x += 1;
                           output_x += multiple;
                        }
                    }

               }
               input_y += 1;
               output_y += multiple;
            }
        }

       return output;
   }
}