/*
 * Copyright 2013 ZXing authors
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
// package com::google::zxing::pdf417::decoder;

/**
 * @author Guenther Grau
 */

 const CODEWORD_SKEW_SIZE: i32 = 2;

 const MAX_ERRORS: i32 = 3;

 const MAX_EC_CODEWORDS: i32 = 512;

 let error_correction: ErrorCorrection = ErrorCorrection::new();
pub struct PDF417ScanningDecoder {
}

impl PDF417ScanningDecoder {

    fn new() -> PDF417ScanningDecoder {
    }

    // TODO don't pass in minCodewordWidth and maxCodewordWidth, pass in barcode columns for start and stop pattern
    // columns. That way width can be deducted from the pattern column.
    // This approach also allows to detect more details about the barcode, e.g. if a bar type (white or black) is wider
    // than it should be. This can happen if the scanner used a bad blackpoint.
    pub fn  decode( image: &BitMatrix,  image_top_left: &ResultPoint,  image_bottom_left: &ResultPoint,  image_top_right: &ResultPoint,  image_bottom_right: &ResultPoint,  min_codeword_width: i32,  max_codeword_width: i32) -> /*  throws NotFoundException, FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
         let bounding_box: BoundingBox = BoundingBox::new(image, image_top_left, image_bottom_left, image_top_right, image_bottom_right);
         let left_row_indicator_column: DetectionResultRowIndicatorColumn = null;
         let right_row_indicator_column: DetectionResultRowIndicatorColumn = null;
         let detection_result: DetectionResult;
         {
             let first_pass: bool = true;
            loop  {
                {
                    if image_top_left != null {
                        left_row_indicator_column = ::get_row_indicator_column(image, bounding_box, image_top_left, true, min_codeword_width, max_codeword_width);
                    }
                    if image_top_right != null {
                        right_row_indicator_column = ::get_row_indicator_column(image, bounding_box, image_top_right, false, min_codeword_width, max_codeword_width);
                    }
                    detection_result = ::merge(left_row_indicator_column, right_row_indicator_column);
                    if detection_result == null {
                        throw NotFoundException::get_not_found_instance();
                    }
                     let result_box: BoundingBox = detection_result.get_bounding_box();
                    if first_pass && result_box != null && (result_box.get_min_y() < bounding_box.get_min_y() || result_box.get_max_y() > bounding_box.get_max_y()) {
                        bounding_box = result_box;
                    } else {
                        break;
                    }
                }
                first_pass = false;
             }
         }

        detection_result.set_bounding_box(bounding_box);
         let max_barcode_column: i32 = detection_result.get_barcode_column_count() + 1;
        detection_result.set_detection_result_column(0, left_row_indicator_column);
        detection_result.set_detection_result_column(max_barcode_column, right_row_indicator_column);
         let left_to_right: bool = left_row_indicator_column != null;
         {
             let barcode_column_count: i32 = 1;
            while barcode_column_count <= max_barcode_column {
                {
                     let barcode_column: i32 =  if left_to_right { barcode_column_count } else { max_barcode_column - barcode_column_count };
                    if detection_result.get_detection_result_column(barcode_column) != null {
                        // This will be the case for the opposite row indicator column, which doesn't need to be decoded again.
                        continue;
                    }
                     let detection_result_column: DetectionResultColumn;
                    if barcode_column == 0 || barcode_column == max_barcode_column {
                        detection_result_column = DetectionResultRowIndicatorColumn::new(bounding_box, barcode_column == 0);
                    } else {
                        detection_result_column = DetectionResultColumn::new(bounding_box);
                    }
                    detection_result.set_detection_result_column(barcode_column, detection_result_column);
                     let start_column: i32 = -1;
                     let previous_start_column: i32 = start_column;
                    // TODO start at a row for which we know the start position, then detect upwards and downwards from there.
                     {
                         let image_row: i32 = bounding_box.get_min_y();
                        while image_row <= bounding_box.get_max_y() {
                            {
                                start_column = ::get_start_column(detection_result, barcode_column, image_row, left_to_right);
                                if start_column < 0 || start_column > bounding_box.get_max_x() {
                                    if previous_start_column == -1 {
                                        continue;
                                    }
                                    start_column = previous_start_column;
                                }
                                 let codeword: Codeword = ::detect_codeword(image, &bounding_box.get_min_x(), &bounding_box.get_max_x(), left_to_right, start_column, image_row, min_codeword_width, max_codeword_width);
                                if codeword != null {
                                    detection_result_column.set_codeword(image_row, codeword);
                                    previous_start_column = start_column;
                                    min_codeword_width = Math::min(min_codeword_width, &codeword.get_width());
                                    max_codeword_width = Math::max(max_codeword_width, &codeword.get_width());
                                }
                            }
                            image_row += 1;
                         }
                     }

                }
                barcode_column_count += 1;
             }
         }

        return Ok(::create_decoder_result(detection_result));
    }

    fn  merge( left_row_indicator_column: &DetectionResultRowIndicatorColumn,  right_row_indicator_column: &DetectionResultRowIndicatorColumn) -> /*  throws NotFoundException */Result<DetectionResult, Rc<Exception>>   {
        if left_row_indicator_column == null && right_row_indicator_column == null {
            return Ok(null);
        }
         let barcode_metadata: BarcodeMetadata = ::get_barcode_metadata(left_row_indicator_column, right_row_indicator_column);
        if barcode_metadata == null {
            return Ok(null);
        }
         let bounding_box: BoundingBox = BoundingBox::merge(&::adjust_bounding_box(left_row_indicator_column), &::adjust_bounding_box(right_row_indicator_column));
        return Ok(DetectionResult::new(barcode_metadata, bounding_box));
    }

    fn  adjust_bounding_box( row_indicator_column: &DetectionResultRowIndicatorColumn) -> /*  throws NotFoundException */Result<BoundingBox, Rc<Exception>>   {
        if row_indicator_column == null {
            return Ok(null);
        }
         let row_heights: Vec<i32> = row_indicator_column.get_row_heights();
        if row_heights == null {
            return Ok(null);
        }
         let max_row_height: i32 = ::get_max(&row_heights);
         let missing_start_rows: i32 = 0;
        for  let row_height: i32 in row_heights {
            missing_start_rows += max_row_height - row_height;
            if row_height > 0 {
                break;
            }
        }
         let codewords: Vec<Codeword> = row_indicator_column.get_codewords();
         {
             let mut row: i32 = 0;
            while missing_start_rows > 0 && codewords[row] == null {
                {
                    missing_start_rows -= 1;
                }
                row += 1;
             }
         }

         let missing_end_rows: i32 = 0;
         {
             let mut row: i32 = row_heights.len() - 1;
            while row >= 0 {
                {
                    missing_end_rows += max_row_height - row_heights[row];
                    if row_heights[row] > 0 {
                        break;
                    }
                }
                row -= 1;
             }
         }

         {
             let mut row: i32 = codewords.len() - 1;
            while missing_end_rows > 0 && codewords[row] == null {
                {
                    missing_end_rows -= 1;
                }
                row -= 1;
             }
         }

        return Ok(row_indicator_column.get_bounding_box().add_missing_rows(missing_start_rows, missing_end_rows, &row_indicator_column.is_left()));
    }

    fn  get_max( values: &Vec<i32>) -> i32  {
         let max_value: i32 = -1;
        for  let value: i32 in values {
            max_value = Math::max(max_value, value);
        }
        return max_value;
    }

    fn  get_barcode_metadata( left_row_indicator_column: &DetectionResultRowIndicatorColumn,  right_row_indicator_column: &DetectionResultRowIndicatorColumn) -> BarcodeMetadata  {
         let left_barcode_metadata: BarcodeMetadata;
        if left_row_indicator_column == null || (left_barcode_metadata = left_row_indicator_column.get_barcode_metadata()) == null {
            return  if right_row_indicator_column == null { null } else { right_row_indicator_column.get_barcode_metadata() };
        }
         let right_barcode_metadata: BarcodeMetadata;
        if right_row_indicator_column == null || (right_barcode_metadata = right_row_indicator_column.get_barcode_metadata()) == null {
            return left_barcode_metadata;
        }
        if left_barcode_metadata.get_column_count() != right_barcode_metadata.get_column_count() && left_barcode_metadata.get_error_correction_level() != right_barcode_metadata.get_error_correction_level() && left_barcode_metadata.get_row_count() != right_barcode_metadata.get_row_count() {
            return null;
        }
        return left_barcode_metadata;
    }

    fn  get_row_indicator_column( image: &BitMatrix,  bounding_box: &BoundingBox,  start_point: &ResultPoint,  left_to_right: bool,  min_codeword_width: i32,  max_codeword_width: i32) -> DetectionResultRowIndicatorColumn  {
         let row_indicator_column: DetectionResultRowIndicatorColumn = DetectionResultRowIndicatorColumn::new(bounding_box, left_to_right);
         {
             let mut i: i32 = 0;
            while i < 2 {
                {
                     let increment: i32 =  if i == 0 { 1 } else { -1 };
                     let start_column: i32 = start_point.get_x() as i32;
                     {
                         let image_row: i32 = start_point.get_y() as i32;
                        while image_row <= bounding_box.get_max_y() && image_row >= bounding_box.get_min_y() {
                            {
                                 let codeword: Codeword = ::detect_codeword(image, 0, &image.get_width(), left_to_right, start_column, image_row, min_codeword_width, max_codeword_width);
                                if codeword != null {
                                    row_indicator_column.set_codeword(image_row, codeword);
                                    if left_to_right {
                                        start_column = codeword.get_start_x();
                                    } else {
                                        start_column = codeword.get_end_x();
                                    }
                                }
                            }
                            image_row += increment;
                         }
                     }

                }
                i += 1;
             }
         }

        return row_indicator_column;
    }

    fn  adjust_codeword_count( detection_result: &DetectionResult,  barcode_matrix: &Vec<Vec<BarcodeValue>>)  -> /*  throws NotFoundException */Result<Void, Rc<Exception>>   {
         let barcode_matrix01: BarcodeValue = barcode_matrix[0][1];
         let number_of_codewords: Vec<i32> = barcode_matrix01.get_value();
         let calculated_number_of_codewords: i32 = detection_result.get_barcode_column_count() * detection_result.get_barcode_row_count() - ::get_number_of_e_c_code_words(&detection_result.get_barcode_e_c_level());
        if number_of_codewords.len() == 0 {
            if calculated_number_of_codewords < 1 || calculated_number_of_codewords > PDF417Common.MAX_CODEWORDS_IN_BARCODE {
                throw NotFoundException::get_not_found_instance();
            }
            barcode_matrix01.set_value(calculated_number_of_codewords);
        } else if number_of_codewords[0] != calculated_number_of_codewords {
            if calculated_number_of_codewords >= 1 && calculated_number_of_codewords <= PDF417Common.MAX_CODEWORDS_IN_BARCODE {
                // The calculated one is more reliable as it is derived from the row indicator columns
                barcode_matrix01.set_value(calculated_number_of_codewords);
            }
        }
    }

    fn  create_decoder_result( detection_result: &DetectionResult) -> /*  throws FormatException, ChecksumException, NotFoundException */Result<DecoderResult, Rc<Exception>>   {
         let barcode_matrix: Vec<Vec<BarcodeValue>> = ::create_barcode_matrix(detection_result);
        ::adjust_codeword_count(detection_result, barcode_matrix);
         let erasures: Collection<Integer> = ArrayList<>::new();
         let mut codewords: [i32; detection_result.get_barcode_row_count() * detection_result.get_barcode_column_count()] = [0; detection_result.get_barcode_row_count() * detection_result.get_barcode_column_count()];
         let ambiguous_index_values_list: List<Vec<i32>> = ArrayList<>::new();
         let ambiguous_indexes_list: Collection<Integer> = ArrayList<>::new();
         {
             let mut row: i32 = 0;
            while row < detection_result.get_barcode_row_count() {
                {
                     {
                         let mut column: i32 = 0;
                        while column < detection_result.get_barcode_column_count() {
                            {
                                 let values: Vec<i32> = barcode_matrix[row][column + 1].get_value();
                                 let codeword_index: i32 = row * detection_result.get_barcode_column_count() + column;
                                if values.len() == 0 {
                                    erasures.add(codeword_index);
                                } else if values.len() == 1 {
                                    codewords[codeword_index] = values[0];
                                } else {
                                    ambiguous_indexes_list.add(codeword_index);
                                    ambiguous_index_values_list.add(&values);
                                }
                            }
                            column += 1;
                         }
                     }

                }
                row += 1;
             }
         }

         let ambiguous_index_values: [i32; ambiguous_index_values_list.size()] = [0; ambiguous_index_values_list.size()];
         {
             let mut i: i32 = 0;
            while i < ambiguous_index_values.len() {
                {
                    ambiguous_index_values[i] = ambiguous_index_values_list.get(i);
                }
                i += 1;
             }
         }

        return Ok(::create_decoder_result_from_ambiguous_values(&detection_result.get_barcode_e_c_level(), &codewords, &PDF417Common::to_int_array(&erasures), &PDF417Common::to_int_array(&ambiguous_indexes_list), &ambiguous_index_values));
    }

    /**
   * This method deals with the fact, that the decoding process doesn't always yield a single most likely value. The
   * current error correction implementation doesn't deal with erasures very well, so it's better to provide a value
   * for these ambiguous codewords instead of treating it as an erasure. The problem is that we don't know which of
   * the ambiguous values to choose. We try decode using the first value, and if that fails, we use another of the
   * ambiguous values and try to decode again. This usually only happens on very hard to read and decode barcodes,
   * so decoding the normal barcodes is not affected by this.
   *
   * @param erasureArray contains the indexes of erasures
   * @param ambiguousIndexes array with the indexes that have more than one most likely value
   * @param ambiguousIndexValues two dimensional array that contains the ambiguous values. The first dimension must
   * be the same length as the ambiguousIndexes array
   */
    fn  create_decoder_result_from_ambiguous_values( ec_level: i32,  codewords: &Vec<i32>,  erasure_array: &Vec<i32>,  ambiguous_indexes: &Vec<i32>,  ambiguous_index_values: &Vec<Vec<i32>>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
         let ambiguous_index_count: [i32; ambiguous_indexes.len()] = [0; ambiguous_indexes.len()];
         let mut tries: i32 = 100;
        while tries -= 1 !!!check!!! post decrement > 0 {
             {
                 let mut i: i32 = 0;
                while i < ambiguous_index_count.len() {
                    {
                        codewords[ambiguous_indexes[i]] = ambiguous_index_values[i][ambiguous_index_count[i]];
                    }
                    i += 1;
                 }
             }

            let tryResult1 = 0;
            'try1: loop {
            {
                return Ok(::decode_codewords(&codewords, ec_level, &erasure_array));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( ignored: &ChecksumException) {
                }  0 => break
            }

            if ambiguous_index_count.len() == 0 {
                throw ChecksumException::get_checksum_instance();
            }
             {
                 let mut i: i32 = 0;
                while i < ambiguous_index_count.len() {
                    {
                        if ambiguous_index_count[i] < ambiguous_index_values[i].len() - 1 {
                            ambiguous_index_count[i] += 1;
                            break;
                        } else {
                            ambiguous_index_count[i] = 0;
                            if i == ambiguous_index_count.len() - 1 {
                                throw ChecksumException::get_checksum_instance();
                            }
                        }
                    }
                    i += 1;
                 }
             }

        }
        throw ChecksumException::get_checksum_instance();
    }

    fn  create_barcode_matrix( detection_result: &DetectionResult) -> Vec<Vec<BarcodeValue>>  {
         let barcode_matrix: [[Option<BarcodeValue>; detection_result.get_barcode_column_count() + 2]; detection_result.get_barcode_row_count()] = [[None; detection_result.get_barcode_column_count() + 2]; detection_result.get_barcode_row_count()];
         {
             let mut row: i32 = 0;
            while row < barcode_matrix.len() {
                {
                     {
                         let mut column: i32 = 0;
                        while column < barcode_matrix[row].len() {
                            {
                                barcode_matrix[row][column] = BarcodeValue::new();
                            }
                            column += 1;
                         }
                     }

                }
                row += 1;
             }
         }

         let mut column: i32 = 0;
        for  let detection_result_column: DetectionResultColumn in detection_result.get_detection_result_columns() {
            if detection_result_column != null {
                for  let codeword: Codeword in detection_result_column.get_codewords() {
                    if codeword != null {
                         let row_number: i32 = codeword.get_row_number();
                        if row_number >= 0 {
                            if row_number >= barcode_matrix.len() {
                                // We have more rows than the barcode metadata allows for, ignore them.
                                continue;
                            }
                            barcode_matrix[row_number][column].set_value(&codeword.get_value());
                        }
                    }
                }
            }
            column += 1;
        }
        return barcode_matrix;
    }

    fn  is_valid_barcode_column( detection_result: &DetectionResult,  barcode_column: i32) -> bool  {
        return barcode_column >= 0 && barcode_column <= detection_result.get_barcode_column_count() + 1;
    }

    fn  get_start_column( detection_result: &DetectionResult,  barcode_column: i32,  image_row: i32,  left_to_right: bool) -> i32  {
         let offset: i32 =  if left_to_right { 1 } else { -1 };
         let mut codeword: Codeword = null;
        if ::is_valid_barcode_column(detection_result, barcode_column - offset) {
            codeword = detection_result.get_detection_result_column(barcode_column - offset).get_codeword(image_row);
        }
        if codeword != null {
            return  if left_to_right { codeword.get_end_x() } else { codeword.get_start_x() };
        }
        codeword = detection_result.get_detection_result_column(barcode_column).get_codeword_nearby(image_row);
        if codeword != null {
            return  if left_to_right { codeword.get_start_x() } else { codeword.get_end_x() };
        }
        if ::is_valid_barcode_column(detection_result, barcode_column - offset) {
            codeword = detection_result.get_detection_result_column(barcode_column - offset).get_codeword_nearby(image_row);
        }
        if codeword != null {
            return  if left_to_right { codeword.get_end_x() } else { codeword.get_start_x() };
        }
         let skipped_columns: i32 = 0;
        while ::is_valid_barcode_column(detection_result, barcode_column - offset) {
            barcode_column -= offset;
            for  let previous_row_codeword: Codeword in detection_result.get_detection_result_column(barcode_column).get_codewords() {
                if previous_row_codeword != null {
                    return ( if left_to_right { previous_row_codeword.get_end_x() } else { previous_row_codeword.get_start_x() }) + offset * skipped_columns * (previous_row_codeword.get_end_x() - previous_row_codeword.get_start_x());
                }
            }
            skipped_columns += 1;
        }
        return  if left_to_right { detection_result.get_bounding_box().get_min_x() } else { detection_result.get_bounding_box().get_max_x() };
    }

    fn  detect_codeword( image: &BitMatrix,  min_column: i32,  max_column: i32,  left_to_right: bool,  start_column: i32,  image_row: i32,  min_codeword_width: i32,  max_codeword_width: i32) -> Codeword  {
        start_column = ::adjust_codeword_start_column(image, min_column, max_column, left_to_right, start_column, image_row);
        // we usually know fairly exact now how long a codeword is. We should provide minimum and maximum expected length
        // and try to adjust the read pixels, e.g. remove single pixel errors or try to cut off exceeding pixels.
        // min and maxCodewordWidth should not be used as they are calculated for the whole barcode an can be inaccurate
        // for the current position
         let module_bit_count: Vec<i32> = ::get_module_bit_count(image, min_column, max_column, left_to_right, start_column, image_row);
        if module_bit_count == null {
            return null;
        }
         let end_column: i32;
         let codeword_bit_count: i32 = MathUtils::sum(&module_bit_count);
        if left_to_right {
            end_column = start_column + codeword_bit_count;
        } else {
             {
                 let mut i: i32 = 0;
                while i < module_bit_count.len() / 2 {
                    {
                         let tmp_count: i32 = module_bit_count[i];
                        module_bit_count[i] = module_bit_count[module_bit_count.len() - 1 - i];
                        module_bit_count[module_bit_count.len() - 1 - i] = tmp_count;
                    }
                    i += 1;
                 }
             }

            end_column = start_column;
            start_column = end_column - codeword_bit_count;
        }
        // sufficient for now
        if !::check_codeword_skew(codeword_bit_count, min_codeword_width, max_codeword_width) {
            // create the bit count from it and normalize it to 8. This would help with single pixel errors.
            return null;
        }
         let decoded_value: i32 = PDF417CodewordDecoder::get_decoded_value(&module_bit_count);
         let codeword: i32 = PDF417Common::get_codeword(decoded_value);
        if codeword == -1 {
            return null;
        }
        return Codeword::new(start_column, end_column, &::get_codeword_bucket_number(decoded_value), codeword);
    }

    fn  get_module_bit_count( image: &BitMatrix,  min_column: i32,  max_column: i32,  left_to_right: bool,  start_column: i32,  image_row: i32) -> Vec<i32>  {
         let image_column: i32 = start_column;
         let module_bit_count: [i32; 8] = [0; 8];
         let module_number: i32 = 0;
         let increment: i32 =  if left_to_right { 1 } else { -1 };
         let previous_pixel_value: bool = left_to_right;
        while ( if left_to_right { image_column < max_column } else { image_column >= min_column }) && module_number < module_bit_count.len() {
            if image.get(image_column, image_row) == previous_pixel_value {
                module_bit_count[module_number] += 1;
                image_column += increment;
            } else {
                module_number += 1;
                previous_pixel_value = !previous_pixel_value;
            }
        }
        if module_number == module_bit_count.len() || ((image_column == ( if left_to_right { max_column } else { min_column })) && module_number == module_bit_count.len() - 1) {
            return module_bit_count;
        }
        return null;
    }

    fn  get_number_of_e_c_code_words( barcode_e_c_level: i32) -> i32  {
        return 2 << barcode_e_c_level;
    }

    fn  adjust_codeword_start_column( image: &BitMatrix,  min_column: i32,  max_column: i32,  left_to_right: bool,  codeword_start_column: i32,  image_row: i32) -> i32  {
         let corrected_start_column: i32 = codeword_start_column;
         let mut increment: i32 =  if left_to_right { -1 } else { 1 };
        // there should be no black pixels before the start column. If there are, then we need to start earlier.
         {
             let mut i: i32 = 0;
            while i < 2 {
                {
                    while ( if left_to_right { corrected_start_column >= min_column } else { corrected_start_column < max_column }) && left_to_right == image.get(corrected_start_column, image_row) {
                        if Math::abs(codeword_start_column - corrected_start_column) > CODEWORD_SKEW_SIZE {
                            return codeword_start_column;
                        }
                        corrected_start_column += increment;
                    }
                    increment = -increment;
                    left_to_right = !left_to_right;
                }
                i += 1;
             }
         }

        return corrected_start_column;
    }

    fn  check_codeword_skew( codeword_size: i32,  min_codeword_width: i32,  max_codeword_width: i32) -> bool  {
        return min_codeword_width - CODEWORD_SKEW_SIZE <= codeword_size && codeword_size <= max_codeword_width + CODEWORD_SKEW_SIZE;
    }

    fn  decode_codewords( codewords: &Vec<i32>,  ec_level: i32,  erasures: &Vec<i32>) -> /*  throws FormatException, ChecksumException */Result<DecoderResult, Rc<Exception>>   {
        if codewords.len() == 0 {
            throw FormatException::get_format_instance();
        }
         let num_e_c_codewords: i32 = 1 << (ec_level + 1);
         let corrected_errors_count: i32 = ::correct_errors(&codewords, &erasures, num_e_c_codewords);
        ::verify_codeword_count(&codewords, num_e_c_codewords);
        // Decode the codewords
         let decoder_result: DecoderResult = DecodedBitStreamParser::decode(&codewords, &String::value_of(ec_level));
        decoder_result.set_errors_corrected(corrected_errors_count);
        decoder_result.set_erasures(erasures.len());
        return Ok(decoder_result);
    }

    /**
   * <p>Given data and error-correction codewords received, possibly corrupted by errors, attempts to
   * correct the errors in-place.</p>
   *
   * @param codewords   data and error correction codewords
   * @param erasures positions of any known erasures
   * @param numECCodewords number of error correction codewords that are available in codewords
   * @throws ChecksumException if error correction fails
   */
    fn  correct_errors( codewords: &Vec<i32>,  erasures: &Vec<i32>,  num_e_c_codewords: i32) -> /*  throws ChecksumException */Result<i32, Rc<Exception>>   {
        if erasures != null && erasures.len() > num_e_c_codewords / 2 + MAX_ERRORS || num_e_c_codewords < 0 || num_e_c_codewords > MAX_EC_CODEWORDS {
            // Too many errors or EC Codewords is corrupted
            throw ChecksumException::get_checksum_instance();
        }
        return Ok(error_correction.decode(&codewords, num_e_c_codewords, &erasures));
    }

    /**
   * Verify that all is OK with the codeword array.
   */
    fn  verify_codeword_count( codewords: &Vec<i32>,  num_e_c_codewords: i32)  -> /*  throws FormatException */Result<Void, Rc<Exception>>   {
        if codewords.len() < 4 {
            // Count CW, At least one Data CW, Error Correction CW, Error Correction CW
            throw FormatException::get_format_instance();
        }
        // The first codeword, the Symbol Length Descriptor, shall always encode the total number of data
        // codewords in the symbol, including the Symbol Length Descriptor itself, data codewords and pad
        // codewords, but excluding the number of error correction codewords.
         let number_of_codewords: i32 = codewords[0];
        if number_of_codewords > codewords.len() {
            throw FormatException::get_format_instance();
        }
        if number_of_codewords == 0 {
            // Reset to the length of the array - 8 (Allow for at least level 3 Error Correction (8 Error Codewords)
            if num_e_c_codewords < codewords.len() {
                codewords[0] = codewords.len() - num_e_c_codewords;
            } else {
                throw FormatException::get_format_instance();
            }
        }
    }

    fn  get_bit_count_for_codeword( codeword: i32) -> Vec<i32>  {
         let mut result: [i32; 8] = [0; 8];
         let previous_value: i32 = 0;
         let mut i: i32 = result.len() - 1;
        while true {
            if (codeword & 0x1) != previous_value {
                previous_value = codeword & 0x1;
                i -= 1;
                if i < 0 {
                    break;
                }
            }
            result[i] += 1;
            codeword >>= 1;
        }
        return result;
    }

    fn  get_codeword_bucket_number( codeword: i32) -> i32  {
        return ::get_codeword_bucket_number(&::get_bit_count_for_codeword(codeword));
    }

    fn  get_codeword_bucket_number( module_bit_count: &Vec<i32>) -> i32  {
        return (module_bit_count[0] - module_bit_count[2] + module_bit_count[4] - module_bit_count[6] + 9) % 9;
    }

    pub fn  to_string( barcode_matrix: &Vec<Vec<BarcodeValue>>) -> String  {
        let tryResult1 = 0;
        'try1: loop {
        ( let formatter: Formatter = Formatter::new()) {
             {
                 let mut row: i32 = 0;
                while row < barcode_matrix.len() {
                    {
                        formatter.format("Row %2d: ", row);
                         {
                             let mut column: i32 = 0;
                            while column < barcode_matrix[row].len() {
                                {
                                     let barcode_value: BarcodeValue = barcode_matrix[row][column];
                                    if barcode_value.get_value().len() == 0 {
                                        formatter.format("        ", null as Vec<Object>);
                                    } else {
                                        formatter.format("%4d(%2d)", barcode_value.get_value()[0], &barcode_value.get_confidence(barcode_value.get_value()[0]));
                                    }
                                }
                                column += 1;
                             }
                         }

                        formatter.format("%n");
                    }
                    row += 1;
                 }
             }

            return formatter.to_string();
        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }

    }
}

