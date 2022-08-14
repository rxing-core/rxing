import com.google.zxing.pdf417.PDF417Common;

import com.google.zxing.NotFoundException;
import com.google.zxing.ResultPoint;
import com.google.zxing.common.BitMatrix;

import com.google.zxing.FormatException;
import com.google.zxing.common.ECIStringBuilder;
import com.google.zxing.common.DecoderResult;
import com.google.zxing.pdf417.PDF417ResultMetadata;

import com.google.zxing.pdf417.PDF417Common;

import com.google.zxing.ResultPoint;
import com.google.zxing.pdf417.PDF417Common;

import com.google.zxing.common.detector.MathUtils;
import com.google.zxing.pdf417.PDF417Common;

import com.google.zxing.ChecksumException;
import com.google.zxing.FormatException;
import com.google.zxing.NotFoundException;
import com.google.zxing.ResultPoint;
import com.google.zxing.common.BitMatrix;
import com.google.zxing.common.DecoderResult;
import com.google.zxing.common.detector.MathUtils;
import com.google.zxing.pdf417.PDF417Common;
import com.google.zxing.pdf417.decoder.ec.ErrorCorrection;

// NEW FILE: barcode_metadata.rs
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
struct BarcodeMetadata {

     let column_count: i32;

     let error_correction_level: i32;

     let row_count_upper_part: i32;

     let row_count_lower_part: i32;

     let row_count: i32;
}

impl BarcodeMetadata {

    fn new( column_count: i32,  row_count_upper_part: i32,  row_count_lower_part: i32,  error_correction_level: i32) -> BarcodeMetadata {
        let .columnCount = column_count;
        let .errorCorrectionLevel = error_correction_level;
        let .rowCountUpperPart = row_count_upper_part;
        let .rowCountLowerPart = row_count_lower_part;
        let .rowCount = row_count_upper_part + row_count_lower_part;
    }

    fn  get_column_count(&self) -> i32  {
        return self.column_count;
    }

    fn  get_error_correction_level(&self) -> i32  {
        return self.error_correction_level;
    }

    fn  get_row_count(&self) -> i32  {
        return self.row_count;
    }

    fn  get_row_count_upper_part(&self) -> i32  {
        return self.row_count_upper_part;
    }

    fn  get_row_count_lower_part(&self) -> i32  {
        return self.row_count_lower_part;
    }
}

// NEW FILE: barcode_value.rs
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
struct BarcodeValue {

     let values: Map<Integer, Integer> = HashMap<>::new();
}

impl BarcodeValue {

    /**
   * Add an occurrence of a value
   */
    fn  set_value(&self,  value: i32)   {
         let mut confidence: Integer = self.values.get(value);
        if confidence == null {
            confidence = 0;
        }
        confidence += 1;
        self.values.put(value, &confidence);
    }

    /**
   * Determines the maximum occurrence of a set value and returns all values which were set with this occurrence.
   * @return an array of int, containing the values with the highest occurrence, or null, if no value was set
   */
    fn  get_value(&self) -> Vec<i32>  {
         let max_confidence: i32 = -1;
         let result: Collection<Integer> = ArrayList<>::new();
        for  let entry: Entry<Integer, Integer> in self.values.entry_set() {
            if entry.get_value() > max_confidence {
                max_confidence = entry.get_value();
                result.clear();
                result.add(&entry.get_key());
            } else if entry.get_value() == max_confidence {
                result.add(&entry.get_key());
            }
        }
        return PDF417Common::to_int_array(&result);
    }

    fn  get_confidence(&self,  value: i32) -> Integer  {
        return self.values.get(value);
    }
}

// NEW FILE: bounding_box.rs
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
struct BoundingBox {

     let mut image: BitMatrix;

     let top_left: ResultPoint;

     let bottom_left: ResultPoint;

     let top_right: ResultPoint;

     let bottom_right: ResultPoint;

     let min_x: i32;

     let max_x: i32;

     let min_y: i32;

     let max_y: i32;
}

impl BoundingBox {

    fn new( image: &BitMatrix,  top_left: &ResultPoint,  bottom_left: &ResultPoint,  top_right: &ResultPoint,  bottom_right: &ResultPoint) -> BoundingBox throws NotFoundException {
         let left_unspecified: bool = top_left == null || bottom_left == null;
         let right_unspecified: bool = top_right == null || bottom_right == null;
        if left_unspecified && right_unspecified {
            throw NotFoundException::get_not_found_instance();
        }
        if left_unspecified {
            top_left = ResultPoint::new(0, &top_right.get_y());
            bottom_left = ResultPoint::new(0, &bottom_right.get_y());
        } else if right_unspecified {
            top_right = ResultPoint::new(image.get_width() - 1, &top_left.get_y());
            bottom_right = ResultPoint::new(image.get_width() - 1, &bottom_left.get_y());
        }
        let .image = image;
        let .topLeft = top_left;
        let .bottomLeft = bottom_left;
        let .topRight = top_right;
        let .bottomRight = bottom_right;
        let .minX = Math::min(&top_left.get_x(), &bottom_left.get_x()) as i32;
        let .maxX = Math::max(&top_right.get_x(), &bottom_right.get_x()) as i32;
        let .minY = Math::min(&top_left.get_y(), &top_right.get_y()) as i32;
        let .maxY = Math::max(&bottom_left.get_y(), &bottom_right.get_y()) as i32;
    }

    fn new( bounding_box: &BoundingBox) -> BoundingBox {
        let .image = bounding_box.image;
        let .topLeft = bounding_box.topLeft;
        let .bottomLeft = bounding_box.bottomLeft;
        let .topRight = bounding_box.topRight;
        let .bottomRight = bounding_box.bottomRight;
        let .minX = bounding_box.minX;
        let .maxX = bounding_box.maxX;
        let .minY = bounding_box.minY;
        let .maxY = bounding_box.maxY;
    }

    fn  merge( left_box: &BoundingBox,  right_box: &BoundingBox) -> /*  throws NotFoundException */Result<BoundingBox, Rc<Exception>>   {
        if left_box == null {
            return Ok(right_box);
        }
        if right_box == null {
            return Ok(left_box);
        }
        return Ok(BoundingBox::new(left_box.image, left_box.topLeft, left_box.bottomLeft, right_box.topRight, right_box.bottomRight));
    }

    fn  add_missing_rows(&self,  missing_start_rows: i32,  missing_end_rows: i32,  is_left: bool) -> /*  throws NotFoundException */Result<BoundingBox, Rc<Exception>>   {
         let new_top_left: ResultPoint = self.top_left;
         let new_bottom_left: ResultPoint = self.bottom_left;
         let new_top_right: ResultPoint = self.top_right;
         let new_bottom_right: ResultPoint = self.bottom_right;
        if missing_start_rows > 0 {
             let top: ResultPoint =  if is_left { self.top_left } else { self.top_right };
             let new_min_y: i32 = top.get_y() as i32 - missing_start_rows;
            if new_min_y < 0 {
                new_min_y = 0;
            }
             let new_top: ResultPoint = ResultPoint::new(&top.get_x(), new_min_y);
            if is_left {
                new_top_left = new_top;
            } else {
                new_top_right = new_top;
            }
        }
        if missing_end_rows > 0 {
             let bottom: ResultPoint =  if is_left { self.bottom_left } else { self.bottom_right };
             let new_max_y: i32 = bottom.get_y() as i32 + missing_end_rows;
            if new_max_y >= self.image.get_height() {
                new_max_y = self.image.get_height() - 1;
            }
             let new_bottom: ResultPoint = ResultPoint::new(&bottom.get_x(), new_max_y);
            if is_left {
                new_bottom_left = new_bottom;
            } else {
                new_bottom_right = new_bottom;
            }
        }
        return Ok(BoundingBox::new(self.image, new_top_left, new_bottom_left, new_top_right, new_bottom_right));
    }

    fn  get_min_x(&self) -> i32  {
        return self.min_x;
    }

    fn  get_max_x(&self) -> i32  {
        return self.max_x;
    }

    fn  get_min_y(&self) -> i32  {
        return self.min_y;
    }

    fn  get_max_y(&self) -> i32  {
        return self.max_y;
    }

    fn  get_top_left(&self) -> ResultPoint  {
        return self.top_left;
    }

    fn  get_top_right(&self) -> ResultPoint  {
        return self.top_right;
    }

    fn  get_bottom_left(&self) -> ResultPoint  {
        return self.bottom_left;
    }

    fn  get_bottom_right(&self) -> ResultPoint  {
        return self.bottom_right;
    }
}

// NEW FILE: codeword.rs
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

 const BARCODE_ROW_UNKNOWN: i32 = -1;
struct Codeword {

     let start_x: i32;

     let end_x: i32;

     let bucket: i32;

     let value: i32;

     let row_number: i32 = BARCODE_ROW_UNKNOWN;
}

impl Codeword {

    fn new( start_x: i32,  end_x: i32,  bucket: i32,  value: i32) -> Codeword {
        let .startX = start_x;
        let .endX = end_x;
        let .bucket = bucket;
        let .value = value;
    }

    fn  has_valid_row_number(&self) -> bool  {
        return self.is_valid_row_number(self.row_number);
    }

    fn  is_valid_row_number(&self,  row_number: i32) -> bool  {
        return row_number != BARCODE_ROW_UNKNOWN && self.bucket == (row_number % 3) * 3;
    }

    fn  set_row_number_as_row_indicator_column(&self)   {
        self.row_number = (self.value / 30) * 3 + self.bucket / 3;
    }

    fn  get_width(&self) -> i32  {
        return self.end_x - self.start_x;
    }

    fn  get_start_x(&self) -> i32  {
        return self.start_x;
    }

    fn  get_end_x(&self) -> i32  {
        return self.end_x;
    }

    fn  get_bucket(&self) -> i32  {
        return self.bucket;
    }

    fn  get_value(&self) -> i32  {
        return self.value;
    }

    fn  get_row_number(&self) -> i32  {
        return self.row_number;
    }

    fn  set_row_number(&self,  row_number: i32)   {
        self.rowNumber = row_number;
    }

    pub fn  to_string(&self) -> String  {
        return format!("{}|{}", self.row_number, self.value);
    }
}

// NEW FILE: decoded_bit_stream_parser.rs
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
// package com::google::zxing::pdf417::decoder;

/**
 * <p>This class contains the methods for decoding the PDF417 codewords.</p>
 *
 * @author SITA Lab (kevin.osullivan@sita.aero)
 * @author Guenther Grau
 */

 const TEXT_COMPACTION_MODE_LATCH: i32 = 900;

 const BYTE_COMPACTION_MODE_LATCH: i32 = 901;

 const NUMERIC_COMPACTION_MODE_LATCH: i32 = 902;

 const BYTE_COMPACTION_MODE_LATCH_6: i32 = 924;

 const ECI_USER_DEFINED: i32 = 925;

 const ECI_GENERAL_PURPOSE: i32 = 926;

 const ECI_CHARSET: i32 = 927;

 const BEGIN_MACRO_PDF417_CONTROL_BLOCK: i32 = 928;

 const BEGIN_MACRO_PDF417_OPTIONAL_FIELD: i32 = 923;

 const MACRO_PDF417_TERMINATOR: i32 = 922;

 const MODE_SHIFT_TO_BYTE_COMPACTION_MODE: i32 = 913;

 const MAX_NUMERIC_CODEWORDS: i32 = 15;

 const MACRO_PDF417_OPTIONAL_FIELD_FILE_NAME: i32 = 0;

 const MACRO_PDF417_OPTIONAL_FIELD_SEGMENT_COUNT: i32 = 1;

 const MACRO_PDF417_OPTIONAL_FIELD_TIME_STAMP: i32 = 2;

 const MACRO_PDF417_OPTIONAL_FIELD_SENDER: i32 = 3;

 const MACRO_PDF417_OPTIONAL_FIELD_ADDRESSEE: i32 = 4;

 const MACRO_PDF417_OPTIONAL_FIELD_FILE_SIZE: i32 = 5;

 const MACRO_PDF417_OPTIONAL_FIELD_CHECKSUM: i32 = 6;

 const PL: i32 = 25;

 const LL: i32 = 27;

 const AS: i32 = 27;

 const ML: i32 = 28;

 const AL: i32 = 28;

 const PS: i32 = 29;

 const PAL: i32 = 29;

 const PUNCT_CHARS: Vec<char> = ";<>@[\\]_`~!\r\t,:\n-.$/\"|*()?{}'".to_char_array();

 const MIXED_CHARS: Vec<char> = "0123456789&\r\t,:#-.$/+%*=^".to_char_array();

/**
   * Table containing values for the exponent of 900.
   * This is used in the numeric compaction decode algorithm.
   */
 const EXP900: Vec<BigInteger>;

 const NUMBER_OF_SEQUENCE_CODEWORDS: i32 = 2;
struct DecodedBitStreamParser {
}

impl DecodedBitStreamParser {

    enum Mode {

        ALPHA(), LOWER(), MIXED(), PUNCT(), ALPHA_SHIFT(), PUNCT_SHIFT()
    }

    static {
        EXP900 = : [Option<BigInteger>; 16] = [None; 16];
        EXP900[0] = BigInteger::ONE;
         let nine_hundred: BigInteger = BigInteger::value_of(900);
        EXP900[1] = nine_hundred;
         {
             let mut i: i32 = 2;
            while i < EXP900.len() {
                {
                    EXP900[i] = EXP900[i - 1]::multiply(&nine_hundred);
                }
                i += 1;
             }
         }

    }

    fn new() -> DecodedBitStreamParser {
    }

    fn  decode( codewords: &Vec<i32>,  ec_level: &String) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
         let result: ECIStringBuilder = ECIStringBuilder::new(codewords.len() * 2);
         let code_index: i32 = ::text_compaction(&codewords, 1, result);
         let result_metadata: PDF417ResultMetadata = PDF417ResultMetadata::new();
        while code_index < codewords[0] {
             let code: i32 = codewords[code_index += 1 !!!check!!! post increment];
            match code {
                  TEXT_COMPACTION_MODE_LATCH => 
                     {
                        code_index = ::text_compaction(&codewords, code_index, result);
                        break;
                    }
                  BYTE_COMPACTION_MODE_LATCH => 
                     {
                    }
                  BYTE_COMPACTION_MODE_LATCH_6 => 
                     {
                        code_index = ::byte_compaction(code, &codewords, code_index, result);
                        break;
                    }
                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                     {
                        result.append(codewords[code_index += 1 !!!check!!! post increment] as char);
                        break;
                    }
                  NUMERIC_COMPACTION_MODE_LATCH => 
                     {
                        code_index = ::numeric_compaction(&codewords, code_index, result);
                        break;
                    }
                  ECI_CHARSET => 
                     {
                        result.append_e_c_i(codewords[code_index += 1 !!!check!!! post increment]);
                        break;
                    }
                  ECI_GENERAL_PURPOSE => 
                     {
                        // Can't do anything with generic ECI; skip its 2 characters
                        code_index += 2;
                        break;
                    }
                  ECI_USER_DEFINED => 
                     {
                        // Can't do anything with user ECI; skip its 1 character
                        code_index += 1;
                        break;
                    }
                  BEGIN_MACRO_PDF417_CONTROL_BLOCK => 
                     {
                        code_index = ::decode_macro_block(&codewords, code_index, result_metadata);
                        break;
                    }
                  BEGIN_MACRO_PDF417_OPTIONAL_FIELD => 
                     {
                    }
                  MACRO_PDF417_TERMINATOR => 
                     {
                        // Should not see these outside a macro block
                        throw FormatException::get_format_instance();
                    }
                _ => 
                     {
                        // Default to text compaction. During testing numerous barcodes
                        // appeared to be missing the starting mode. In these cases defaulting
                        // to text compaction seems to work.
                        code_index -= 1;
                        code_index = ::text_compaction(&codewords, code_index, result);
                        break;
                    }
            }
        }
        if result.is_empty() && result_metadata.get_file_id() == null {
            throw FormatException::get_format_instance();
        }
         let decoder_result: DecoderResult = DecoderResult::new(null, &result.to_string(), null, &ec_level);
        decoder_result.set_other(result_metadata);
        return Ok(decoder_result);
    }

    fn  decode_macro_block( codewords: &Vec<i32>,  code_index: i32,  result_metadata: &PDF417ResultMetadata) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
        if code_index + NUMBER_OF_SEQUENCE_CODEWORDS > codewords[0] {
            // we must have at least two bytes left for the segment index
            throw FormatException::get_format_instance();
        }
         let segment_index_array: [i32; NUMBER_OF_SEQUENCE_CODEWORDS] = [0; NUMBER_OF_SEQUENCE_CODEWORDS];
         {
             let mut i: i32 = 0;
            while i < NUMBER_OF_SEQUENCE_CODEWORDS {
                {
                    segment_index_array[i] = codewords[code_index];
                }
                i += 1;
                code_index += 1;
             }
         }

         let segment_index_string: String = ::decode_base900to_base10(&segment_index_array, NUMBER_OF_SEQUENCE_CODEWORDS);
        if segment_index_string.is_empty() {
            result_metadata.set_segment_index(0);
        } else {
            let tryResult1 = 0;
            'try1: loop {
            {
                result_metadata.set_segment_index(&Integer::parse_int(&segment_index_string));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( nfe: &NumberFormatException) {
                    throw FormatException::get_format_instance();
                }  0 => break
            }

        }
        // Decoding the fileId codewords as 0-899 numbers, each 0-filled to width 3. This follows the spec
        // (See ISO/IEC 15438:2015 Annex H.6) and preserves all info, but some generators (e.g. TEC-IT) write
        // the fileId using text compaction, so in those cases the fileId will appear mangled.
         let file_id: StringBuilder = StringBuilder::new();
        while code_index < codewords[0] && code_index < codewords.len() && codewords[code_index] != MACRO_PDF417_TERMINATOR && codewords[code_index] != BEGIN_MACRO_PDF417_OPTIONAL_FIELD {
            file_id.append(&String::format("%03d", codewords[code_index]));
            code_index += 1;
        }
        if file_id.length() == 0 {
            // at least one fileId codeword is required (Annex H.2)
            throw FormatException::get_format_instance();
        }
        result_metadata.set_file_id(&file_id.to_string());
         let optional_fields_start: i32 = -1;
        if codewords[code_index] == BEGIN_MACRO_PDF417_OPTIONAL_FIELD {
            optional_fields_start = code_index + 1;
        }
        while code_index < codewords[0] {
            match codewords[code_index] {
                  BEGIN_MACRO_PDF417_OPTIONAL_FIELD => 
                     {
                        code_index += 1;
                        match codewords[code_index] {
                              MACRO_PDF417_OPTIONAL_FIELD_FILE_NAME => 
                                 {
                                     let file_name: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::text_compaction(&codewords, code_index + 1, file_name);
                                    result_metadata.set_file_name(&file_name.to_string());
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_SENDER => 
                                 {
                                     let sender: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::text_compaction(&codewords, code_index + 1, sender);
                                    result_metadata.set_sender(&sender.to_string());
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_ADDRESSEE => 
                                 {
                                     let addressee: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::text_compaction(&codewords, code_index + 1, addressee);
                                    result_metadata.set_addressee(&addressee.to_string());
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_SEGMENT_COUNT => 
                                 {
                                     let segment_count: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::numeric_compaction(&codewords, code_index + 1, segment_count);
                                    result_metadata.set_segment_count(&Integer::parse_int(&segment_count.to_string()));
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_TIME_STAMP => 
                                 {
                                     let timestamp: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::numeric_compaction(&codewords, code_index + 1, timestamp);
                                    result_metadata.set_timestamp(&Long::parse_long(&timestamp.to_string()));
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_CHECKSUM => 
                                 {
                                     let checksum: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::numeric_compaction(&codewords, code_index + 1, checksum);
                                    result_metadata.set_checksum(&Integer::parse_int(&checksum.to_string()));
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_FILE_SIZE => 
                                 {
                                     let file_size: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::numeric_compaction(&codewords, code_index + 1, file_size);
                                    result_metadata.set_file_size(&Long::parse_long(&file_size.to_string()));
                                    break;
                                }
                            _ => 
                                 {
                                    throw FormatException::get_format_instance();
                                }
                        }
                        break;
                    }
                  MACRO_PDF417_TERMINATOR => 
                     {
                        code_index += 1;
                        result_metadata.set_last_segment(true);
                        break;
                    }
                _ => 
                     {
                        throw FormatException::get_format_instance();
                    }
            }
        }
        // copy optional fields to additional options
        if optional_fields_start != -1 {
             let optional_fields_length: i32 = code_index - optional_fields_start;
            if result_metadata.is_last_segment() {
                // do not include terminator
                optional_fields_length -= 1;
            }
            result_metadata.set_optional_data(&Arrays::copy_of_range(&codewords, optional_fields_start, optional_fields_start + optional_fields_length));
        }
        return Ok(code_index);
    }

    /**
   * Text Compaction mode (see 5.4.1.5) permits all printable ASCII characters to be
   * encoded, i.e. values 32 - 126 inclusive in accordance with ISO/IEC 646 (IRV), as
   * well as selected control characters.
   *
   * @param codewords The array of codewords (data + error)
   * @param codeIndex The current index into the codeword array.
   * @param result    The decoded data is appended to the result.
   * @return The next index into the codeword array.
   */
    fn  text_compaction( codewords: &Vec<i32>,  code_index: i32,  result: &ECIStringBuilder) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
        // 2 character per codeword
         let text_compaction_data: [i32; (codewords[0] - code_index) * 2] = [0; (codewords[0] - code_index) * 2];
        // Used to hold the byte compaction value if there is a mode shift
         let byte_compaction_data: [i32; (codewords[0] - code_index) * 2] = [0; (codewords[0] - code_index) * 2];
         let mut index: i32 = 0;
         let mut end: bool = false;
         let sub_mode: Mode = Mode::ALPHA;
        while (code_index < codewords[0]) && !end {
             let mut code: i32 = codewords[code_index += 1 !!!check!!! post increment];
            if code < TEXT_COMPACTION_MODE_LATCH {
                text_compaction_data[index] = code / 30;
                text_compaction_data[index + 1] = code % 30;
                index += 2;
            } else {
                match code {
                      TEXT_COMPACTION_MODE_LATCH => 
                         {
                            // reinitialize text compaction mode to alpha sub mode
                            text_compaction_data[index += 1 !!!check!!! post increment] = TEXT_COMPACTION_MODE_LATCH;
                            break;
                        }
                      BYTE_COMPACTION_MODE_LATCH => 
                         {
                        }
                      BYTE_COMPACTION_MODE_LATCH_6 => 
                         {
                        }
                      NUMERIC_COMPACTION_MODE_LATCH => 
                         {
                        }
                      BEGIN_MACRO_PDF417_CONTROL_BLOCK => 
                         {
                        }
                      BEGIN_MACRO_PDF417_OPTIONAL_FIELD => 
                         {
                        }
                      MACRO_PDF417_TERMINATOR => 
                         {
                            code_index -= 1;
                            end = true;
                            break;
                        }
                      MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                         {
                            // The Mode Shift codeword 913 shall cause a temporary
                            // switch from Text Compaction mode to Byte Compaction mode.
                            // This switch shall be in effect for only the next codeword,
                            // after which the mode shall revert to the prevailing sub-mode
                            // of the Text Compaction mode. Codeword 913 is only available
                            // in Text Compaction mode; its use is described in 5.4.2.4.
                            text_compaction_data[index] = MODE_SHIFT_TO_BYTE_COMPACTION_MODE;
                            code = codewords[code_index += 1 !!!check!!! post increment];
                            byte_compaction_data[index] = code;
                            index += 1;
                            break;
                        }
                      ECI_CHARSET => 
                         {
                            sub_mode = ::decode_text_compaction(&text_compaction_data, &byte_compaction_data, index, result, sub_mode);
                            result.append_e_c_i(codewords[code_index += 1 !!!check!!! post increment]);
                            text_compaction_data = : [i32; (codewords[0] - code_index) * 2] = [0; (codewords[0] - code_index) * 2];
                            byte_compaction_data = : [i32; (codewords[0] - code_index) * 2] = [0; (codewords[0] - code_index) * 2];
                            index = 0;
                            break;
                        }
                }
            }
        }
        ::decode_text_compaction(&text_compaction_data, &byte_compaction_data, index, result, sub_mode);
        return Ok(code_index);
    }

    /**
   * The Text Compaction mode includes all the printable ASCII characters
   * (i.e. values from 32 to 126) and three ASCII control characters: HT or tab
   * (ASCII value 9), LF or line feed (ASCII value 10), and CR or carriage
   * return (ASCII value 13). The Text Compaction mode also includes various latch
   * and shift characters which are used exclusively within the mode. The Text
   * Compaction mode encodes up to 2 characters per codeword. The compaction rules
   * for converting data into PDF417 codewords are defined in 5.4.2.2. The sub-mode
   * switches are defined in 5.4.2.3.
   *
   * @param textCompactionData The text compaction data.
   * @param byteCompactionData The byte compaction data if there
   *                           was a mode shift.
   * @param length             The size of the text compaction and byte compaction data.
   * @param result             The decoded data is appended to the result.
   * @param startMode          The mode in which decoding starts
   * @return The mode in which decoding ended
   */
    fn  decode_text_compaction( text_compaction_data: &Vec<i32>,  byte_compaction_data: &Vec<i32>,  length: i32,  result: &ECIStringBuilder,  start_mode: &Mode) -> Mode  {
        // Beginning from an initial state
        // The default compaction mode for PDF417 in effect at the start of each symbol shall always be Text
        // Compaction mode Alpha sub-mode (uppercase alphabetic). A latch codeword from another mode to the Text
        // Compaction mode shall always switch to the Text Compaction Alpha sub-mode.
         let sub_mode: Mode = start_mode;
         let prior_to_shift_mode: Mode = start_mode;
         let latched_mode: Mode = start_mode;
         let mut i: i32 = 0;
        while i < length {
             let sub_mode_ch: i32 = text_compaction_data[i];
             let mut ch: char = 0;
            match sub_mode {
                  ALPHA => 
                     {
                        // Alpha (uppercase alphabetic)
                        if sub_mode_ch < 26 {
                            // Upper case Alpha Character
                            ch = ('A' + sub_mode_ch) as char;
                        } else {
                            match sub_mode_ch {
                                  26 => 
                                     {
                                        ch = ' ';
                                        break;
                                    }
                                  LL => 
                                     {
                                        sub_mode = Mode::LOWER;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  ML => 
                                     {
                                        sub_mode = Mode::MIXED;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  PS => 
                                     {
                                        // Shift to punctuation
                                        prior_to_shift_mode = sub_mode;
                                        sub_mode = Mode::PUNCT_SHIFT;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  LOWER => 
                     {
                        // Lower (lowercase alphabetic)
                        if sub_mode_ch < 26 {
                            ch = ('a' + sub_mode_ch) as char;
                        } else {
                            match sub_mode_ch {
                                  26 => 
                                     {
                                        ch = ' ';
                                        break;
                                    }
                                  AS => 
                                     {
                                        // Shift to alpha
                                        prior_to_shift_mode = sub_mode;
                                        sub_mode = Mode::ALPHA_SHIFT;
                                        break;
                                    }
                                  ML => 
                                     {
                                        sub_mode = Mode::MIXED;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  PS => 
                                     {
                                        // Shift to punctuation
                                        prior_to_shift_mode = sub_mode;
                                        sub_mode = Mode::PUNCT_SHIFT;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  MIXED => 
                     {
                        // Mixed (numeric and some punctuation)
                        if sub_mode_ch < PL {
                            ch = MIXED_CHARS[sub_mode_ch];
                        } else {
                            match sub_mode_ch {
                                  PL => 
                                     {
                                        sub_mode = Mode::PUNCT;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  26 => 
                                     {
                                        ch = ' ';
                                        break;
                                    }
                                  LL => 
                                     {
                                        sub_mode = Mode::LOWER;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  AL => 
                                     {
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  PS => 
                                     {
                                        // Shift to punctuation
                                        prior_to_shift_mode = sub_mode;
                                        sub_mode = Mode::PUNCT_SHIFT;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  PUNCT => 
                     {
                        // Punctuation
                        if sub_mode_ch < PAL {
                            ch = PUNCT_CHARS[sub_mode_ch];
                        } else {
                            match sub_mode_ch {
                                  PAL => 
                                     {
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  ALPHA_SHIFT => 
                     {
                        // Restore sub-mode
                        sub_mode = prior_to_shift_mode;
                        if sub_mode_ch < 26 {
                            ch = ('A' + sub_mode_ch) as char;
                        } else {
                            match sub_mode_ch {
                                  26 => 
                                     {
                                        ch = ' ';
                                        break;
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  PUNCT_SHIFT => 
                     {
                        // Restore sub-mode
                        sub_mode = prior_to_shift_mode;
                        if sub_mode_ch < PAL {
                            ch = PUNCT_CHARS[sub_mode_ch];
                        } else {
                            match sub_mode_ch {
                                  PAL => 
                                     {
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        // PS before Shift-to-Byte is used as a padding character,
                                        // see 5.4.2.4 of the specification
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                            }
                        }
                        break;
                    }
            }
            if ch != 0 {
                // Append decoded character to result
                result.append(ch);
            }
            i += 1;
        }
        return latched_mode;
    }

    /**
   * Byte Compaction mode (see 5.4.3) permits all 256 possible 8-bit byte values to be encoded.
   * This includes all ASCII characters value 0 to 127 inclusive and provides for international
   * character set support.
   *
   * @param mode      The byte compaction mode i.e. 901 or 924
   * @param codewords The array of codewords (data + error)
   * @param codeIndex The current index into the codeword array.
   * @param result    The decoded data is appended to the result.
   * @return The next index into the codeword array.
   */
    fn  byte_compaction( mode: i32,  codewords: &Vec<i32>,  code_index: i32,  result: &ECIStringBuilder) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
         let mut end: bool = false;
        while code_index < codewords[0] && !end {
            //handle leading ECIs
            while code_index < codewords[0] && codewords[code_index] == ECI_CHARSET {
                result.append_e_c_i(codewords[code_index += 1]);
                code_index += 1;
            }
            if code_index >= codewords[0] || codewords[code_index] >= TEXT_COMPACTION_MODE_LATCH {
                end = true;
            } else {
                //decode one block of 5 codewords to 6 bytes
                 let mut value: i64 = 0;
                 let mut count: i32 = 0;
                loop { {
                    value = 900 * value + codewords[code_index += 1 !!!check!!! post increment];
                    count += 1;
                }if !(count < 5 && code_index < codewords[0] && codewords[code_index] < TEXT_COMPACTION_MODE_LATCH) break;}
                if count == 5 && (mode == BYTE_COMPACTION_MODE_LATCH_6 || code_index < codewords[0] && codewords[code_index] < TEXT_COMPACTION_MODE_LATCH) {
                     {
                         let mut i: i32 = 0;
                        while i < 6 {
                            {
                                result.append((value >> (8 * (5 - i))) as i8);
                            }
                            i += 1;
                         }
                     }

                } else {
                    code_index -= count;
                    while (code_index < codewords[0]) && !end {
                         let code: i32 = codewords[code_index += 1 !!!check!!! post increment];
                        if code < TEXT_COMPACTION_MODE_LATCH {
                            result.append(code as i8);
                        } else if code == ECI_CHARSET {
                            result.append_e_c_i(codewords[code_index += 1 !!!check!!! post increment]);
                        } else {
                            code_index -= 1;
                            end = true;
                        }
                    }
                }
            }
        }
        return Ok(code_index);
    }

    /**
   * Numeric Compaction mode (see 5.4.4) permits efficient encoding of numeric data strings.
   *
   * @param codewords The array of codewords (data + error)
   * @param codeIndex The current index into the codeword array.
   * @param result    The decoded data is appended to the result.
   * @return The next index into the codeword array.
   */
    fn  numeric_compaction( codewords: &Vec<i32>,  code_index: i32,  result: &ECIStringBuilder) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
         let mut count: i32 = 0;
         let mut end: bool = false;
         let numeric_codewords: [i32; MAX_NUMERIC_CODEWORDS] = [0; MAX_NUMERIC_CODEWORDS];
        while code_index < codewords[0] && !end {
             let code: i32 = codewords[code_index += 1 !!!check!!! post increment];
            if code_index == codewords[0] {
                end = true;
            }
            if code < TEXT_COMPACTION_MODE_LATCH {
                numeric_codewords[count] = code;
                count += 1;
            } else {
                match code {
                      TEXT_COMPACTION_MODE_LATCH => 
                         {
                        }
                      BYTE_COMPACTION_MODE_LATCH => 
                         {
                        }
                      BYTE_COMPACTION_MODE_LATCH_6 => 
                         {
                        }
                      BEGIN_MACRO_PDF417_CONTROL_BLOCK => 
                         {
                        }
                      BEGIN_MACRO_PDF417_OPTIONAL_FIELD => 
                         {
                        }
                      MACRO_PDF417_TERMINATOR => 
                         {
                        }
                      ECI_CHARSET => 
                         {
                            code_index -= 1;
                            end = true;
                            break;
                        }
                }
            }
            if (count % MAX_NUMERIC_CODEWORDS == 0 || code == NUMERIC_COMPACTION_MODE_LATCH || end) && count > 0 {
                // Re-invoking Numeric Compaction mode (by using codeword 902
                // while in Numeric Compaction mode) serves  to terminate the
                // current Numeric Compaction mode grouping as described in 5.4.4.2,
                // and then to start a new one grouping.
                result.append(&::decode_base900to_base10(&numeric_codewords, count));
                count = 0;
            }
        }
        return Ok(code_index);
    }

    /**
   * Convert a list of Numeric Compacted codewords from Base 900 to Base 10.
   *
   * @param codewords The array of codewords
   * @param count     The number of codewords
   * @return The decoded string representing the Numeric data.
   */
    /*
     EXAMPLE
     Encode the fifteen digit numeric string 000213298174000
     Prefix the numeric string with a 1 and set the initial value of
     t = 1 000 213 298 174 000
     Calculate codeword 0
     d0 = 1 000 213 298 174 000 mod 900 = 200

     t = 1 000 213 298 174 000 div 900 = 1 111 348 109 082
     Calculate codeword 1
     d1 = 1 111 348 109 082 mod 900 = 282

     t = 1 111 348 109 082 div 900 = 1 234 831 232
     Calculate codeword 2
     d2 = 1 234 831 232 mod 900 = 632

     t = 1 234 831 232 div 900 = 1 372 034
     Calculate codeword 3
     d3 = 1 372 034 mod 900 = 434

     t = 1 372 034 div 900 = 1 524
     Calculate codeword 4
     d4 = 1 524 mod 900 = 624

     t = 1 524 div 900 = 1
     Calculate codeword 5
     d5 = 1 mod 900 = 1
     t = 1 div 900 = 0
     Codeword sequence is: 1, 624, 434, 632, 282, 200

     Decode the above codewords involves
       1 x 900 power of 5 + 624 x 900 power of 4 + 434 x 900 power of 3 +
     632 x 900 power of 2 + 282 x 900 power of 1 + 200 x 900 power of 0 = 1000213298174000

     Remove leading 1 =>  Result is 000213298174000
   */
    fn  decode_base900to_base10( codewords: &Vec<i32>,  count: i32) -> /*  throws FormatException */Result<String, Rc<Exception>>   {
         let mut result: BigInteger = BigInteger::ZERO;
         {
             let mut i: i32 = 0;
            while i < count {
                {
                    result = result.add(&EXP900[count - i - 1]::multiply(&BigInteger::value_of(codewords[i])));
                }
                i += 1;
             }
         }

         let result_string: String = result.to_string();
        if result_string.char_at(0) != '1' {
            throw FormatException::get_format_instance();
        }
        return Ok(result_string.substring(1));
    }
}

// NEW FILE: detection_result.rs
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

 const ADJUST_ROW_NUMBER_SKIP: i32 = 2;
struct DetectionResult {

     let barcode_metadata: BarcodeMetadata;

     let detection_result_columns: Vec<DetectionResultColumn>;

     let bounding_box: BoundingBox;

     let barcode_column_count: i32;
}

impl DetectionResult {

    fn new( barcode_metadata: &BarcodeMetadata,  bounding_box: &BoundingBox) -> DetectionResult {
        let .barcodeMetadata = barcode_metadata;
        let .barcodeColumnCount = barcode_metadata.get_column_count();
        let .boundingBox = bounding_box;
        detection_result_columns = : [Option<DetectionResultColumn>; barcode_column_count + 2] = [None; barcode_column_count + 2];
    }

    fn  get_detection_result_columns(&self) -> Vec<DetectionResultColumn>  {
        self.adjust_indicator_column_row_numbers(self.detection_result_columns[0]);
        self.adjust_indicator_column_row_numbers(self.detection_result_columns[self.barcode_column_count + 1]);
         let unadjusted_codeword_count: i32 = PDF417Common.MAX_CODEWORDS_IN_BARCODE;
         let previous_unadjusted_count: i32;
        loop { {
            previous_unadjusted_count = unadjusted_codeword_count;
            unadjusted_codeword_count = self.adjust_row_numbers();
        }if !(unadjusted_codeword_count > 0 && unadjusted_codeword_count < previous_unadjusted_count) break;}
        return self.detection_result_columns;
    }

    fn  adjust_indicator_column_row_numbers(&self,  detection_result_column: &DetectionResultColumn)   {
        if detection_result_column != null {
            (detection_result_column as DetectionResultRowIndicatorColumn).adjust_complete_indicator_column_row_numbers(self.barcode_metadata);
        }
    }

    // TODO ensure that no detected codewords with unknown row number are left
    // we should be able to estimate the row height and use it as a hint for the row number
    // we should also fill the rows top to bottom and bottom to top
    /**
   * @return number of codewords which don't have a valid row number. Note that the count is not accurate as codewords
   * will be counted several times. It just serves as an indicator to see when we can stop adjusting row numbers
   */
    fn  adjust_row_numbers(&self) -> i32  {
         let unadjusted_count: i32 = self.adjust_row_numbers_by_row();
        if unadjusted_count == 0 {
            return 0;
        }
         {
             let barcode_column: i32 = 1;
            while barcode_column < self.barcode_column_count + 1 {
                {
                     let codewords: Vec<Codeword> = self.detection_result_columns[barcode_column].get_codewords();
                     {
                         let codewords_row: i32 = 0;
                        while codewords_row < codewords.len() {
                            {
                                if codewords[codewords_row] == null {
                                    continue;
                                }
                                if !codewords[codewords_row].has_valid_row_number() {
                                    self.adjust_row_numbers(barcode_column, codewords_row, codewords);
                                }
                            }
                            codewords_row += 1;
                         }
                     }

                }
                barcode_column += 1;
             }
         }

        return unadjusted_count;
    }

    fn  adjust_row_numbers_by_row(&self) -> i32  {
        self.adjust_row_numbers_from_both_r_i();
        // TODO we should only do full row adjustments if row numbers of left and right row indicator column match.
        // Maybe it's even better to calculated the height (in codeword rows) and divide it by the number of barcode
        // rows. This, together with the LRI and RRI row numbers should allow us to get a good estimate where a row
        // number starts and ends.
         let unadjusted_count: i32 = self.adjust_row_numbers_from_l_r_i();
        return unadjusted_count + self.adjust_row_numbers_from_r_r_i();
    }

    fn  adjust_row_numbers_from_both_r_i(&self)   {
        if self.detection_result_columns[0] == null || self.detection_result_columns[self.barcode_column_count + 1] == null {
            return;
        }
         const LRIcodewords: Vec<Codeword> = self.detection_result_columns[0].get_codewords();
         const RRIcodewords: Vec<Codeword> = self.detection_result_columns[self.barcode_column_count + 1].get_codewords();
         {
             let codewords_row: i32 = 0;
            while codewords_row < LRIcodewords.len() {
                {
                    if LRIcodewords[codewords_row] != null && RRIcodewords[codewords_row] != null && LRIcodewords[codewords_row]::get_row_number() == RRIcodewords[codewords_row]::get_row_number() {
                         {
                             let barcode_column: i32 = 1;
                            while barcode_column <= self.barcode_column_count {
                                {
                                     let codeword: Codeword = self.detection_result_columns[barcode_column].get_codewords()[codewords_row];
                                    if codeword == null {
                                        continue;
                                    }
                                    codeword.set_row_number(&LRIcodewords[codewords_row]::get_row_number());
                                    if !codeword.has_valid_row_number() {
                                        self.detection_result_columns[barcode_column].get_codewords()[codewords_row] = null;
                                    }
                                }
                                barcode_column += 1;
                             }
                         }

                    }
                }
                codewords_row += 1;
             }
         }

    }

    fn  adjust_row_numbers_from_r_r_i(&self) -> i32  {
        if self.detection_result_columns[self.barcode_column_count + 1] == null {
            return 0;
        }
         let unadjusted_count: i32 = 0;
         let codewords: Vec<Codeword> = self.detection_result_columns[self.barcode_column_count + 1].get_codewords();
         {
             let codewords_row: i32 = 0;
            while codewords_row < codewords.len() {
                {
                    if codewords[codewords_row] == null {
                        continue;
                    }
                     let row_indicator_row_number: i32 = codewords[codewords_row].get_row_number();
                     let invalid_row_counts: i32 = 0;
                     {
                         let barcode_column: i32 = self.barcode_column_count + 1;
                        while barcode_column > 0 && invalid_row_counts < ADJUST_ROW_NUMBER_SKIP {
                            {
                                 let codeword: Codeword = self.detection_result_columns[barcode_column].get_codewords()[codewords_row];
                                if codeword != null {
                                    invalid_row_counts = ::adjust_row_number_if_valid(row_indicator_row_number, invalid_row_counts, codeword);
                                    if !codeword.has_valid_row_number() {
                                        unadjusted_count += 1;
                                    }
                                }
                            }
                            barcode_column -= 1;
                         }
                     }

                }
                codewords_row += 1;
             }
         }

        return unadjusted_count;
    }

    fn  adjust_row_numbers_from_l_r_i(&self) -> i32  {
        if self.detection_result_columns[0] == null {
            return 0;
        }
         let unadjusted_count: i32 = 0;
         let codewords: Vec<Codeword> = self.detection_result_columns[0].get_codewords();
         {
             let codewords_row: i32 = 0;
            while codewords_row < codewords.len() {
                {
                    if codewords[codewords_row] == null {
                        continue;
                    }
                     let row_indicator_row_number: i32 = codewords[codewords_row].get_row_number();
                     let invalid_row_counts: i32 = 0;
                     {
                         let barcode_column: i32 = 1;
                        while barcode_column < self.barcode_column_count + 1 && invalid_row_counts < ADJUST_ROW_NUMBER_SKIP {
                            {
                                 let codeword: Codeword = self.detection_result_columns[barcode_column].get_codewords()[codewords_row];
                                if codeword != null {
                                    invalid_row_counts = ::adjust_row_number_if_valid(row_indicator_row_number, invalid_row_counts, codeword);
                                    if !codeword.has_valid_row_number() {
                                        unadjusted_count += 1;
                                    }
                                }
                            }
                            barcode_column += 1;
                         }
                     }

                }
                codewords_row += 1;
             }
         }

        return unadjusted_count;
    }

    fn  adjust_row_number_if_valid( row_indicator_row_number: i32,  invalid_row_counts: i32,  codeword: &Codeword) -> i32  {
        if codeword == null {
            return invalid_row_counts;
        }
        if !codeword.has_valid_row_number() {
            if codeword.is_valid_row_number(row_indicator_row_number) {
                codeword.set_row_number(row_indicator_row_number);
                invalid_row_counts = 0;
            } else {
                invalid_row_counts += 1;
            }
        }
        return invalid_row_counts;
    }

    fn  adjust_row_numbers(&self,  barcode_column: i32,  codewords_row: i32,  codewords: &Vec<Codeword>)   {
         let codeword: Codeword = codewords[codewords_row];
         let previous_column_codewords: Vec<Codeword> = self.detection_result_columns[barcode_column - 1].get_codewords();
         let next_column_codewords: Vec<Codeword> = previous_column_codewords;
        if self.detection_result_columns[barcode_column + 1] != null {
            next_column_codewords = self.detection_result_columns[barcode_column + 1].get_codewords();
        }
         let other_codewords: [Option<Codeword>; 14] = [None; 14];
        other_codewords[2] = previous_column_codewords[codewords_row];
        other_codewords[3] = next_column_codewords[codewords_row];
        if codewords_row > 0 {
            other_codewords[0] = codewords[codewords_row - 1];
            other_codewords[4] = previous_column_codewords[codewords_row - 1];
            other_codewords[5] = next_column_codewords[codewords_row - 1];
        }
        if codewords_row > 1 {
            other_codewords[8] = codewords[codewords_row - 2];
            other_codewords[10] = previous_column_codewords[codewords_row - 2];
            other_codewords[11] = next_column_codewords[codewords_row - 2];
        }
        if codewords_row < codewords.len() - 1 {
            other_codewords[1] = codewords[codewords_row + 1];
            other_codewords[6] = previous_column_codewords[codewords_row + 1];
            other_codewords[7] = next_column_codewords[codewords_row + 1];
        }
        if codewords_row < codewords.len() - 2 {
            other_codewords[9] = codewords[codewords_row + 2];
            other_codewords[12] = previous_column_codewords[codewords_row + 2];
            other_codewords[13] = next_column_codewords[codewords_row + 2];
        }
        for  let other_codeword: Codeword in other_codewords {
            if ::adjust_row_number(codeword, other_codeword) {
                return;
            }
        }
    }

    /**
   * @return true, if row number was adjusted, false otherwise
   */
    fn  adjust_row_number( codeword: &Codeword,  other_codeword: &Codeword) -> bool  {
        if other_codeword == null {
            return false;
        }
        if other_codeword.has_valid_row_number() && other_codeword.get_bucket() == codeword.get_bucket() {
            codeword.set_row_number(&other_codeword.get_row_number());
            return true;
        }
        return false;
    }

    fn  get_barcode_column_count(&self) -> i32  {
        return self.barcode_column_count;
    }

    fn  get_barcode_row_count(&self) -> i32  {
        return self.barcode_metadata.get_row_count();
    }

    fn  get_barcode_e_c_level(&self) -> i32  {
        return self.barcode_metadata.get_error_correction_level();
    }

    fn  set_bounding_box(&self,  bounding_box: &BoundingBox)   {
        self.boundingBox = bounding_box;
    }

    fn  get_bounding_box(&self) -> BoundingBox  {
        return self.bounding_box;
    }

    fn  set_detection_result_column(&self,  barcode_column: i32,  detection_result_column: &DetectionResultColumn)   {
        self.detection_result_columns[barcode_column] = detection_result_column;
    }

    fn  get_detection_result_column(&self,  barcode_column: i32) -> DetectionResultColumn  {
        return self.detection_result_columns[barcode_column];
    }

    pub fn  to_string(&self) -> String  {
         let row_indicator_column: DetectionResultColumn = self.detection_result_columns[0];
        if row_indicator_column == null {
            row_indicator_column = self.detection_result_columns[self.barcode_column_count + 1];
        }
        let tryResult1 = 0;
        'try1: loop {
        ( let formatter: Formatter = Formatter::new()) {
             {
                 let codewords_row: i32 = 0;
                while codewords_row < row_indicator_column.get_codewords().len() {
                    {
                        formatter.format("CW %3d:", codewords_row);
                         {
                             let barcode_column: i32 = 0;
                            while barcode_column < self.barcode_column_count + 2 {
                                {
                                    if self.detection_result_columns[barcode_column] == null {
                                        formatter.format("    |   ");
                                        continue;
                                    }
                                     let codeword: Codeword = self.detection_result_columns[barcode_column].get_codewords()[codewords_row];
                                    if codeword == null {
                                        formatter.format("    |   ");
                                        continue;
                                    }
                                    formatter.format(" %3d|%3d", &codeword.get_row_number(), &codeword.get_value());
                                }
                                barcode_column += 1;
                             }
                         }

                        formatter.format("%n");
                    }
                    codewords_row += 1;
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

// NEW FILE: detection_result_column.rs
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

 const MAX_NEARBY_DISTANCE: i32 = 5;
struct DetectionResultColumn {

     let bounding_box: BoundingBox;

     let mut codewords: Vec<Codeword>;
}

impl DetectionResultColumn {

    fn new( bounding_box: &BoundingBox) -> DetectionResultColumn {
        let .boundingBox = BoundingBox::new(bounding_box);
        codewords = : [Option<Codeword>; bounding_box.get_max_y() - bounding_box.get_min_y() + 1] = [None; bounding_box.get_max_y() - bounding_box.get_min_y() + 1];
    }

    fn  get_codeword_nearby(&self,  image_row: i32) -> Codeword  {
         let mut codeword: Codeword = self.get_codeword(image_row);
        if codeword != null {
            return codeword;
        }
         {
             let mut i: i32 = 1;
            while i < MAX_NEARBY_DISTANCE {
                {
                     let near_image_row: i32 = self.image_row_to_codeword_index(image_row) - i;
                    if near_image_row >= 0 {
                        codeword = self.codewords[near_image_row];
                        if codeword != null {
                            return codeword;
                        }
                    }
                    near_image_row = self.image_row_to_codeword_index(image_row) + i;
                    if near_image_row < self.codewords.len() {
                        codeword = self.codewords[near_image_row];
                        if codeword != null {
                            return codeword;
                        }
                    }
                }
                i += 1;
             }
         }

        return null;
    }

    fn  image_row_to_codeword_index(&self,  image_row: i32) -> i32  {
        return image_row - self.bounding_box.get_min_y();
    }

    fn  set_codeword(&self,  image_row: i32,  codeword: &Codeword)   {
        self.codewords[self.image_row_to_codeword_index(image_row)] = codeword;
    }

    fn  get_codeword(&self,  image_row: i32) -> Codeword  {
        return self.codewords[self.image_row_to_codeword_index(image_row)];
    }

    fn  get_bounding_box(&self) -> BoundingBox  {
        return self.bounding_box;
    }

    fn  get_codewords(&self) -> Vec<Codeword>  {
        return self.codewords;
    }

    pub fn  to_string(&self) -> String  {
        let tryResult1 = 0;
        'try1: loop {
        ( let formatter: Formatter = Formatter::new()) {
             let mut row: i32 = 0;
            for  let codeword: Codeword in self.codewords {
                if codeword == null {
                    formatter.format("%3d:    |   %n", row += 1 !!!check!!! post increment);
                    continue;
                }
                formatter.format("%3d: %3d|%3d%n", row += 1 !!!check!!! post increment, &codeword.get_row_number(), &codeword.get_value());
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

// NEW FILE: detection_result_row_indicator_column.rs
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
struct DetectionResultRowIndicatorColumn {
    super: DetectionResultColumn;

     let is_left: bool;
}

impl DetectionResultRowIndicatorColumn {

    fn new( bounding_box: &BoundingBox,  is_left: bool) -> DetectionResultRowIndicatorColumn {
        super(bounding_box);
        let .isLeft = is_left;
    }

    fn  set_row_numbers(&self)   {
        for  let codeword: Codeword in get_codewords() {
            if codeword != null {
                codeword.set_row_number_as_row_indicator_column();
            }
        }
    }

    // TODO implement properly
    // TODO maybe we should add missing codewords to store the correct row number to make
    // finding row numbers for other columns easier
    // use row height count to make detection of invalid row numbers more reliable
    fn  adjust_complete_indicator_column_row_numbers(&self,  barcode_metadata: &BarcodeMetadata)   {
         let mut codewords: Vec<Codeword> = get_codewords();
        self.set_row_numbers();
        self.remove_incorrect_codewords(codewords, barcode_metadata);
         let bounding_box: BoundingBox = get_bounding_box();
         let top: ResultPoint =  if self.is_left { bounding_box.get_top_left() } else { bounding_box.get_top_right() };
         let bottom: ResultPoint =  if self.is_left { bounding_box.get_bottom_left() } else { bounding_box.get_bottom_right() };
         let first_row: i32 = image_row_to_codeword_index(top.get_y() as i32);
         let last_row: i32 = image_row_to_codeword_index(bottom.get_y() as i32);
        // We need to be careful using the average row height. Barcode could be skewed so that we have smaller and
        // taller rows
        //float averageRowHeight = (lastRow - firstRow) / (float) barcodeMetadata.getRowCount();
         let barcode_row: i32 = -1;
         let max_row_height: i32 = 1;
         let current_row_height: i32 = 0;
         {
             let codewords_row: i32 = first_row;
            while codewords_row < last_row {
                {
                    if codewords[codewords_row] == null {
                        continue;
                    }
                     let codeword: Codeword = codewords[codewords_row];
                     let row_difference: i32 = codeword.get_row_number() - barcode_row;
                    if row_difference == 0 {
                        current_row_height += 1;
                    } else if row_difference == 1 {
                        max_row_height = Math::max(max_row_height, current_row_height);
                        current_row_height = 1;
                        barcode_row = codeword.get_row_number();
                    } else if row_difference < 0 || codeword.get_row_number() >= barcode_metadata.get_row_count() || row_difference > codewords_row {
                        codewords[codewords_row] = null;
                    } else {
                         let checked_rows: i32;
                        if max_row_height > 2 {
                            checked_rows = (max_row_height - 2) * row_difference;
                        } else {
                            checked_rows = row_difference;
                        }
                         let close_previous_codeword_found: bool = checked_rows >= codewords_row;
                         {
                             let mut i: i32 = 1;
                            while i <= checked_rows && !close_previous_codeword_found {
                                {
                                    // there must be (height * rowDifference) number of codewords missing. For now we assume height = 1.
                                    // This should hopefully get rid of most problems already.
                                    close_previous_codeword_found = codewords[codewords_row - i] != null;
                                }
                                i += 1;
                             }
                         }

                        if close_previous_codeword_found {
                            codewords[codewords_row] = null;
                        } else {
                            barcode_row = codeword.get_row_number();
                            current_row_height = 1;
                        }
                    }
                }
                codewords_row += 1;
             }
         }

    //return (int) (averageRowHeight + 0.5);
    }

    fn  get_row_heights(&self) -> Vec<i32>  {
         let barcode_metadata: BarcodeMetadata = self.get_barcode_metadata();
        if barcode_metadata == null {
            return null;
        }
        self.adjust_incomplete_indicator_column_row_numbers(barcode_metadata);
         let mut result: [i32; barcode_metadata.get_row_count()] = [0; barcode_metadata.get_row_count()];
        for  let codeword: Codeword in get_codewords() {
            if codeword != null {
                 let row_number: i32 = codeword.get_row_number();
                if row_number >= result.len() {
                    // We have more rows than the barcode metadata allows for, ignore them.
                    continue;
                }
                result[row_number] += 1;
            }
        // else throw exception?
        }
        return result;
    }

    // TODO maybe we should add missing codewords to store the correct row number to make
    // finding row numbers for other columns easier
    // use row height count to make detection of invalid row numbers more reliable
    fn  adjust_incomplete_indicator_column_row_numbers(&self,  barcode_metadata: &BarcodeMetadata)   {
         let bounding_box: BoundingBox = get_bounding_box();
         let top: ResultPoint =  if self.is_left { bounding_box.get_top_left() } else { bounding_box.get_top_right() };
         let bottom: ResultPoint =  if self.is_left { bounding_box.get_bottom_left() } else { bounding_box.get_bottom_right() };
         let first_row: i32 = image_row_to_codeword_index(top.get_y() as i32);
         let last_row: i32 = image_row_to_codeword_index(bottom.get_y() as i32);
        //float averageRowHeight = (lastRow - firstRow) / (float) barcodeMetadata.getRowCount();
         let mut codewords: Vec<Codeword> = get_codewords();
         let barcode_row: i32 = -1;
         let max_row_height: i32 = 1;
         let current_row_height: i32 = 0;
         {
             let codewords_row: i32 = first_row;
            while codewords_row < last_row {
                {
                    if codewords[codewords_row] == null {
                        continue;
                    }
                     let codeword: Codeword = codewords[codewords_row];
                    codeword.set_row_number_as_row_indicator_column();
                     let row_difference: i32 = codeword.get_row_number() - barcode_row;
                    if row_difference == 0 {
                        current_row_height += 1;
                    } else if row_difference == 1 {
                        max_row_height = Math::max(max_row_height, current_row_height);
                        current_row_height = 1;
                        barcode_row = codeword.get_row_number();
                    } else if codeword.get_row_number() >= barcode_metadata.get_row_count() {
                        codewords[codewords_row] = null;
                    } else {
                        barcode_row = codeword.get_row_number();
                        current_row_height = 1;
                    }
                }
                codewords_row += 1;
             }
         }

    //return (int) (averageRowHeight + 0.5);
    }

    fn  get_barcode_metadata(&self) -> BarcodeMetadata  {
         let codewords: Vec<Codeword> = get_codewords();
         let barcode_column_count: BarcodeValue = BarcodeValue::new();
         let barcode_row_count_upper_part: BarcodeValue = BarcodeValue::new();
         let barcode_row_count_lower_part: BarcodeValue = BarcodeValue::new();
         let barcode_e_c_level: BarcodeValue = BarcodeValue::new();
        for  let codeword: Codeword in codewords {
            if codeword == null {
                continue;
            }
            codeword.set_row_number_as_row_indicator_column();
             let row_indicator_value: i32 = codeword.get_value() % 30;
             let codeword_row_number: i32 = codeword.get_row_number();
            if !self.is_left {
                codeword_row_number += 2;
            }
            match codeword_row_number % 3 {
                  0 => 
                     {
                        barcode_row_count_upper_part.set_value(row_indicator_value * 3 + 1);
                        break;
                    }
                  1 => 
                     {
                        barcode_e_c_level.set_value(row_indicator_value / 3);
                        barcode_row_count_lower_part.set_value(row_indicator_value % 3);
                        break;
                    }
                  2 => 
                     {
                        barcode_column_count.set_value(row_indicator_value + 1);
                        break;
                    }
            }
        }
        // Maybe we should check if we have ambiguous values?
        if (barcode_column_count.get_value().len() == 0) || (barcode_row_count_upper_part.get_value().len() == 0) || (barcode_row_count_lower_part.get_value().len() == 0) || (barcode_e_c_level.get_value().len() == 0) || barcode_column_count.get_value()[0] < 1 || barcode_row_count_upper_part.get_value()[0] + barcode_row_count_lower_part.get_value()[0] < PDF417Common.MIN_ROWS_IN_BARCODE || barcode_row_count_upper_part.get_value()[0] + barcode_row_count_lower_part.get_value()[0] > PDF417Common.MAX_ROWS_IN_BARCODE {
            return null;
        }
         let barcode_metadata: BarcodeMetadata = BarcodeMetadata::new(barcode_column_count.get_value()[0], barcode_row_count_upper_part.get_value()[0], barcode_row_count_lower_part.get_value()[0], barcode_e_c_level.get_value()[0]);
        self.remove_incorrect_codewords(codewords, barcode_metadata);
        return barcode_metadata;
    }

    fn  remove_incorrect_codewords(&self,  codewords: &Vec<Codeword>,  barcode_metadata: &BarcodeMetadata)   {
        // TODO Maybe we should keep the incorrect codewords for the start and end positions?
         {
             let codeword_row: i32 = 0;
            while codeword_row < codewords.len() {
                {
                     let codeword: Codeword = codewords[codeword_row];
                    if codewords[codeword_row] == null {
                        continue;
                    }
                     let row_indicator_value: i32 = codeword.get_value() % 30;
                     let codeword_row_number: i32 = codeword.get_row_number();
                    if codeword_row_number > barcode_metadata.get_row_count() {
                        codewords[codeword_row] = null;
                        continue;
                    }
                    if !self.is_left {
                        codeword_row_number += 2;
                    }
                    match codeword_row_number % 3 {
                          0 => 
                             {
                                if row_indicator_value * 3 + 1 != barcode_metadata.get_row_count_upper_part() {
                                    codewords[codeword_row] = null;
                                }
                                break;
                            }
                          1 => 
                             {
                                if row_indicator_value / 3 != barcode_metadata.get_error_correction_level() || row_indicator_value % 3 != barcode_metadata.get_row_count_lower_part() {
                                    codewords[codeword_row] = null;
                                }
                                break;
                            }
                          2 => 
                             {
                                if row_indicator_value + 1 != barcode_metadata.get_column_count() {
                                    codewords[codeword_row] = null;
                                }
                                break;
                            }
                    }
                }
                codeword_row += 1;
             }
         }

    }

    fn  is_left(&self) -> bool  {
        return self.is_left;
    }

    pub fn  to_string(&self) -> String  {
        return format!("IsLeft: {}\n{}", self.is_left, super.to_string());
    }
}

// NEW FILE: p_d_f417_codeword_decoder.rs
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
 * @author creatale GmbH (christoph.schulz@creatale.de)
 */

 const RATIOS_TABLE: [[f32; PDF417Common.BARS_IN_MODULE]; PDF417Common.SYMBOL_TABLE.len()] = [[0.0; PDF417Common.BARS_IN_MODULE]; PDF417Common.SYMBOL_TABLE.len()];
struct PDF417CodewordDecoder {
}

impl PDF417CodewordDecoder {

    static {
        // Pre-computes the symbol ratio table.
         {
             let mut i: i32 = 0;
            while i < PDF417Common.SYMBOL_TABLE.len() {
                {
                     let current_symbol: i32 = PDF417Common.SYMBOL_TABLE[i];
                     let current_bit: i32 = current_symbol & 0x1;
                     {
                         let mut j: i32 = 0;
                        while j < PDF417Common.BARS_IN_MODULE {
                            {
                                 let mut size: f32 = 0.0f;
                                while (current_symbol & 0x1) == current_bit {
                                    size += 1.0f;
                                    current_symbol >>= 1;
                                }
                                current_bit = current_symbol & 0x1;
                                RATIOS_TABLE[i][PDF417Common.BARS_IN_MODULE - j - 1] = size / PDF417Common.MODULES_IN_CODEWORD;
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

    }

    fn new() -> PDF417CodewordDecoder {
    }

    fn  get_decoded_value( module_bit_count: &Vec<i32>) -> i32  {
         let decoded_value: i32 = ::get_decoded_codeword_value(&::sample_bit_counts(&module_bit_count));
        if decoded_value != -1 {
            return decoded_value;
        }
        return ::get_closest_decoded_value(&module_bit_count);
    }

    fn  sample_bit_counts( module_bit_count: &Vec<i32>) -> Vec<i32>  {
         let bit_count_sum: f32 = MathUtils::sum(&module_bit_count);
         let mut result: [i32; PDF417Common.BARS_IN_MODULE] = [0; PDF417Common.BARS_IN_MODULE];
         let bit_count_index: i32 = 0;
         let sum_previous_bits: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < PDF417Common.MODULES_IN_CODEWORD {
                {
                     let sample_index: f32 = bit_count_sum / (2.0 * PDF417Common.MODULES_IN_CODEWORD) + (i * bit_count_sum) / PDF417Common.MODULES_IN_CODEWORD;
                    if sum_previous_bits + module_bit_count[bit_count_index] <= sample_index {
                        sum_previous_bits += module_bit_count[bit_count_index];
                        bit_count_index += 1;
                    }
                    result[bit_count_index] += 1;
                }
                i += 1;
             }
         }

        return result;
    }

    fn  get_decoded_codeword_value( module_bit_count: &Vec<i32>) -> i32  {
         let decoded_value: i32 = ::get_bit_value(&module_bit_count);
        return  if PDF417Common::get_codeword(decoded_value) == -1 { -1 } else { decoded_value };
    }

    fn  get_bit_value( module_bit_count: &Vec<i32>) -> i32  {
         let mut result: i64 = 0;
         {
             let mut i: i32 = 0;
            while i < module_bit_count.len() {
                {
                     {
                         let mut bit: i32 = 0;
                        while bit < module_bit_count[i] {
                            {
                                result = (result << 1) | ( if i % 2 == 0 { 1 } else { 0 });
                            }
                            bit += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        return result as i32;
    }

    fn  get_closest_decoded_value( module_bit_count: &Vec<i32>) -> i32  {
         let bit_count_sum: i32 = MathUtils::sum(&module_bit_count);
         let bit_count_ratios: [f32; PDF417Common.BARS_IN_MODULE] = [0.0; PDF417Common.BARS_IN_MODULE];
        if bit_count_sum > 1 {
             {
                 let mut i: i32 = 0;
                while i < bit_count_ratios.len() {
                    {
                        bit_count_ratios[i] = module_bit_count[i] / bit_count_sum as f32;
                    }
                    i += 1;
                 }
             }

        }
         let best_match_error: f32 = Float::MAX_VALUE;
         let best_match: i32 = -1;
         {
             let mut j: i32 = 0;
            while j < RATIOS_TABLE.len() {
                {
                     let mut error: f32 = 0.0f;
                     let ratio_table_row: Vec<f32> = RATIOS_TABLE[j];
                     {
                         let mut k: i32 = 0;
                        while k < PDF417Common.BARS_IN_MODULE {
                            {
                                 let diff: f32 = ratio_table_row[k] - bit_count_ratios[k];
                                error += diff * diff;
                                if error >= best_match_error {
                                    break;
                                }
                            }
                            k += 1;
                         }
                     }

                    if error < best_match_error {
                        best_match_error = error;
                        best_match = PDF417Common.SYMBOL_TABLE[j];
                    }
                }
                j += 1;
             }
         }

        return best_match;
    }
}

// NEW FILE: p_d_f417_scanning_decoder.rs
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

