/*
 * Copyright 2007 ZXing authors
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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>Encapsulates a QR Code's format information, including the data mask used and
 * error correction level.</p>
 *
 * @author Sean Owen
 * @see DataMask
 * @see ErrorCorrectionLevel
 */

 const FORMAT_INFO_MASK_QR: i32 = 0x5412;

/**
   * See ISO 18004:2006, Annex C, Table C.1
   */
 const FORMAT_INFO_DECODE_LOOKUP: vec![vec![Vec<Vec<i32>>; 2]; 32] = vec![vec![0x5412, 0x00, ]
, vec![0x5125, 0x01, ]
, vec![0x5E7C, 0x02, ]
, vec![0x5B4B, 0x03, ]
, vec![0x45F9, 0x04, ]
, vec![0x40CE, 0x05, ]
, vec![0x4F97, 0x06, ]
, vec![0x4AA0, 0x07, ]
, vec![0x77C4, 0x08, ]
, vec![0x72F3, 0x09, ]
, vec![0x7DAA, 0x0A, ]
, vec![0x789D, 0x0B, ]
, vec![0x662F, 0x0C, ]
, vec![0x6318, 0x0D, ]
, vec![0x6C41, 0x0E, ]
, vec![0x6976, 0x0F, ]
, vec![0x1689, 0x10, ]
, vec![0x13BE, 0x11, ]
, vec![0x1CE7, 0x12, ]
, vec![0x19D0, 0x13, ]
, vec![0x0762, 0x14, ]
, vec![0x0255, 0x15, ]
, vec![0x0D0C, 0x16, ]
, vec![0x083B, 0x17, ]
, vec![0x355F, 0x18, ]
, vec![0x3068, 0x19, ]
, vec![0x3F31, 0x1A, ]
, vec![0x3A06, 0x1B, ]
, vec![0x24B4, 0x1C, ]
, vec![0x2183, 0x1D, ]
, vec![0x2EDA, 0x1E, ]
, vec![0x2BED, 0x1F, ]
, ]
;
struct FormatInformation {

     let error_correction_level: ErrorCorrectionLevel;

     let data_mask: i8;
}

impl FormatInformation {

    fn new( format_info: i32) -> FormatInformation {
        // Bits 3,4
        error_correction_level = ErrorCorrectionLevel::for_bits((format_info >> 3) & 0x03);
        // Bottom 3 bits
        data_mask = (format_info & 0x07) as i8;
    }

    fn  num_bits_differing( a: i32,  b: i32) -> i32  {
        return Integer::bit_count(a ^ b);
    }

    /**
   * @param maskedFormatInfo1 format info indicator, with mask still applied
   * @param maskedFormatInfo2 second copy of same info; both are checked at the same time
   *  to establish best match
   * @return information about the format it specifies, or {@code null}
   *  if doesn't seem to match any known pattern
   */
    fn  decode_format_information( masked_format_info1: i32,  masked_format_info2: i32) -> FormatInformation  {
         let format_info: FormatInformation = ::do_decode_format_information(masked_format_info1, masked_format_info2);
        if format_info != null {
            return format_info;
        }
        // first
        return ::do_decode_format_information(masked_format_info1 ^ FORMAT_INFO_MASK_QR, masked_format_info2 ^ FORMAT_INFO_MASK_QR);
    }

    fn  do_decode_format_information( masked_format_info1: i32,  masked_format_info2: i32) -> FormatInformation  {
        // Find the int in FORMAT_INFO_DECODE_LOOKUP with fewest bits differing
         let best_difference: i32 = Integer::MAX_VALUE;
         let best_format_info: i32 = 0;
        for  let decode_info: Vec<i32> in FORMAT_INFO_DECODE_LOOKUP {
             let target_info: i32 = decode_info[0];
            if target_info == masked_format_info1 || target_info == masked_format_info2 {
                // Found an exact match
                return FormatInformation::new(decode_info[1]);
            }
             let bits_difference: i32 = ::num_bits_differing(masked_format_info1, target_info);
            if bits_difference < best_difference {
                best_format_info = decode_info[1];
                best_difference = bits_difference;
            }
            if masked_format_info1 != masked_format_info2 {
                // also try the other option
                bits_difference = ::num_bits_differing(masked_format_info2, target_info);
                if bits_difference < best_difference {
                    best_format_info = decode_info[1];
                    best_difference = bits_difference;
                }
            }
        }
        // differing means we found a match
        if best_difference <= 3 {
            return FormatInformation::new(best_format_info);
        }
        return null;
    }

    fn  get_error_correction_level(&self) -> ErrorCorrectionLevel  {
        return self.error_correction_level;
    }

    fn  get_data_mask(&self) -> i8  {
        return self.data_mask;
    }

    pub fn  hash_code(&self) -> i32  {
        return (self.error_correction_level.ordinal() << 3) | self.data_mask;
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof FormatInformation) {
            return false;
        }
         let other: FormatInformation = o as FormatInformation;
        return self.errorCorrectionLevel == other.errorCorrectionLevel && self.dataMask == other.dataMask;
    }
}

