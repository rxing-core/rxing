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

