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

use super::ErrorCorrectionLevel;

const FORMAT_INFO_MASK_QR: u32 = 0x5412;

/**
 * See ISO 18004:2006, Annex C, Table C.1
 */
const FORMAT_INFO_DECODE_LOOKUP: [[u32; 2]; 32] = [
    [0x5412, 0x00],
    [0x5125, 0x01],
    [0x5E7C, 0x02],
    [0x5B4B, 0x03],
    [0x45F9, 0x04],
    [0x40CE, 0x05],
    [0x4F97, 0x06],
    [0x4AA0, 0x07],
    [0x77C4, 0x08],
    [0x72F3, 0x09],
    [0x7DAA, 0x0A],
    [0x789D, 0x0B],
    [0x662F, 0x0C],
    [0x6318, 0x0D],
    [0x6C41, 0x0E],
    [0x6976, 0x0F],
    [0x1689, 0x10],
    [0x13BE, 0x11],
    [0x1CE7, 0x12],
    [0x19D0, 0x13],
    [0x0762, 0x14],
    [0x0255, 0x15],
    [0x0D0C, 0x16],
    [0x083B, 0x17],
    [0x355F, 0x18],
    [0x3068, 0x19],
    [0x3F31, 0x1A],
    [0x3A06, 0x1B],
    [0x24B4, 0x1C],
    [0x2183, 0x1D],
    [0x2EDA, 0x1E],
    [0x2BED, 0x1F],
];

/**
 * <p>Encapsulates a QR Code's format information, including the data mask used and
 * error correction level.</p>
 *
 * @author Sean Owen
 * @see DataMask
 * @see ErrorCorrectionLevel
 */
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct FormatInformation {
    error_correction_level: ErrorCorrectionLevel,
    data_mask: u8,
}

impl FormatInformation {
    fn new(format_info: u8) -> Self {
        // Bits 3,4
        let errorCorrectionLevel =
            ErrorCorrectionLevel::forBits((format_info >> 3) & 0x03).expect("pass in valid bits");
        // Bottom 3 bits
        let dataMask = format_info & 0x07;
        Self {
            error_correction_level: errorCorrectionLevel,
            data_mask: dataMask,
        }
    }

    pub fn numBitsDiffering(a: u32, b: u32) -> u32 {
        (a ^ b).count_ones()
        // return Integer.bitCount(a ^ b);
    }

    /**
     * @param maskedFormatInfo1 format info indicator, with mask still applied
     * @param maskedFormatInfo2 second copy of same info; both are checked at the same time
     *  to establish best match
     * @return information about the format it specifies, or {@code null}
     *  if doesn't seem to match any known pattern
     */
    pub fn decodeFormatInformation(
        masked_format_info1: u32,
        masked_format_info2: u32,
    ) -> Option<FormatInformation> {
        let formatInfo = Self::doDecodeFormatInformation(masked_format_info1, masked_format_info2);
        if formatInfo.is_some() {
            return formatInfo
        }
        // Should return null, but, some QR codes apparently
        // do not mask this info. Try again by actually masking the pattern
        // first
         Self::doDecodeFormatInformation(
            masked_format_info1 ^ FORMAT_INFO_MASK_QR,
            masked_format_info2 ^ FORMAT_INFO_MASK_QR,
        )
    }

    fn doDecodeFormatInformation(
        masked_format_info1: u32,
        masked_format_info2: u32,
    ) -> Option<FormatInformation> {
        // Find the int in FORMAT_INFO_DECODE_LOOKUP with fewest bits differing
        let mut best_difference = u32::MAX;
        let mut best_format_info = 0;
        for decodeInfo in FORMAT_INFO_DECODE_LOOKUP {
            // for (int[] decodeInfo : FORMAT_INFO_DECODE_LOOKUP) {
            let targetInfo = decodeInfo[0];
            if targetInfo == masked_format_info1 || targetInfo == masked_format_info2 {
                // Found an exact match
                return Some(FormatInformation::new(decodeInfo[1] as u8));
            }
            let mut bits_difference = Self::numBitsDiffering(masked_format_info1, targetInfo);
            if bits_difference < best_difference {
                best_format_info = decodeInfo[1] as u8;
                best_difference = bits_difference;
            }
            if masked_format_info1 != masked_format_info2 {
                // also try the other option
                bits_difference = Self::numBitsDiffering(masked_format_info2, targetInfo);
                if (bits_difference < best_difference) {
                    best_format_info = decodeInfo[1] as u8;
                    best_difference = bits_difference;
                }
            }
        }
        // Hamming distance of the 32 masked codes is 7, by construction, so <= 3 bits
        // differing means we found a match
        if best_difference <= 3 {
            return Some(FormatInformation::new(best_format_info));
        }
        None
    }

    pub fn getErrorCorrectionLevel(&self) -> ErrorCorrectionLevel {
        self.error_correction_level
    }

    pub fn getDataMask(&self) -> u8 {
        self.data_mask
    }

    // @Override
    // public int hashCode() {
    //   return (errorCorrectionLevel.ordinal() << 3) | dataMask;
    // }

    // @Override
    // public boolean equals(Object o) {
    //   if (!(o instanceof FormatInformation)) {
    //     return false;
    //   }
    //   FormatInformation other = (FormatInformation) o;
    //   return this.errorCorrectionLevel == other.errorCorrectionLevel &&
    //       this.dataMask == other.dataMask;
    // }
}
