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

