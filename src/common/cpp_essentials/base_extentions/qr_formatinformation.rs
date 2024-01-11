use crate::qrcode::{
    cpp_port::Type,
    decoder::{
        ErrorCorrectionLevel, FormatInformation, FORMAT_INFO_MASK_MODEL2, FORMAT_INFO_MASK_QR,
    },
};

pub const FORMAT_INFO_MASK_QR_MODEL1: u32 = 0x2825;
pub const FORMAT_INFO_MASK_MICRO: u32 = 0x4445;

// pub const FORMAT_INFO_DECODE_LOOKUP_MICRO: [u32 ;32] = [
//     0x4445,
//     0x4172,
//     0x4E2B,
//     0x4B1C,
//     0x55AE,
//     0x5099,
//     0x5FC0,
//     0x5AF7,
//     0x6793,
//     0x62A4,
//     0x6DFD,
//     0x68CA,
//     0x7678,
//     0x734F,
//     0x7C16,
//     0x7921,
//     0x06DE,
//     0x03E9,
//     0x0CB0,
//     0x0987,
//     0x1735,
//     0x1202,
//     0x1D5B,
//     0x186C,
//     0x2508,
//     0x203F,
//     0x2F66,
//     0x2A51,
//     0x34E3,
//     0x31D4,
//     0x3E8D,
//     0x3BBA,
// ];

impl FormatInformation {
    /**
     * @param formatInfoBits1 format info indicator, with mask still applied
     * @param formatInfoBits2 second copy of same info; both are checked at the same time to establish best match
     */
    pub fn DecodeQR(formatInfoBits1: u32, formatInfoBits2: u32) -> Self {
        // maks out the 'Dark Module' for mirrored and non-mirrored case (see Figure 25 in ISO/IEC 18004:2015)
        let mirroredFormatInfoBits2 = Self::MirrorBits(
            ((formatInfoBits2 >> 1) & 0b111111110000000) | (formatInfoBits2 & 0b1111111),
        );
        let formatInfoBits2 =
            ((formatInfoBits2 >> 1) & 0b111111100000000) | (formatInfoBits2 & 0b11111111);
        // Some (Model2) QR codes apparently do not apply the XOR mask. Try with (standard) and without (quirk) masking.
        let mut fi = Self::FindBestFormatInfo(
            &[FORMAT_INFO_MASK_QR, 0, FORMAT_INFO_MASK_QR_MODEL1],
            &[
                formatInfoBits1,
                formatInfoBits2,
                Self::MirrorBits(formatInfoBits1),
                mirroredFormatInfoBits2,
            ],
        );

        // Use bits 3/4 for error correction, and 0-2 for mask.
        fi.error_correction_level =
            ErrorCorrectionLevel::ECLevelFromBits((fi.data >> 3) as u8 & 0x03, false);
        fi.data_mask = fi.data as u8 & 0x07;
        fi.isMirrored = fi.bitsIndex > 1;

        fi
    }

    pub fn DecodeMQR(formatInfoBits: u32) -> Self {
        // We don't use the additional masking (with 0x4445) to work around potentially non complying MicroQRCode encoders
        let mut fi = Self::FindBestFormatInfo(
            &[FORMAT_INFO_MASK_MICRO, 0],
            &[formatInfoBits, Self::MirrorBits(formatInfoBits)],
        );

        const BITS_TO_VERSION: [u8; 8] = [1, 2, 2, 3, 3, 4, 4, 4];

        // Bits 2/3/4 contain both error correction level and version, 0/1 contain mask.
        fi.error_correction_level =
            ErrorCorrectionLevel::ECLevelFromBits((fi.data >> 2) as u8 & 0x07, true);
        fi.data_mask = fi.data as u8 & 0x03;
        fi.microVersion = BITS_TO_VERSION[((fi.data >> 2) as u8 & 0x07) as usize] as u32;
        fi.isMirrored = fi.bitsIndex == 1;

        fi
    }

    #[inline(always)]
    pub fn MirrorBits(bits: u32) -> u32 {
        (bits.reverse_bits()) >> 17
    }

    pub fn FindBestFormatInfo(masks: &[u32], bits: &[u32]) -> Self {
        let mut fi = FormatInformation::default();

        // See ISO 18004:2015, Annex C, Table C.1
        const MODEL2_MASKED_PATTERNS: [u32; 32] = [
            0x5412, 0x5125, 0x5E7C, 0x5B4B, 0x45F9, 0x40CE, 0x4F97, 0x4AA0, 0x77C4, 0x72F3, 0x7DAA,
            0x789D, 0x662F, 0x6318, 0x6C41, 0x6976, 0x1689, 0x13BE, 0x1CE7, 0x19D0, 0x0762, 0x0255,
            0x0D0C, 0x083B, 0x355F, 0x3068, 0x3F31, 0x3A06, 0x24B4, 0x2183, 0x2EDA, 0x2BED,
        ];

        for mask in masks {
            // for (auto mask : masks)
            for bitsIndex in 0..bits.len() {
                // for (int bitsIndex = 0; bitsIndex < Size(bits); ++bitsIndex)
                for ref_pattern in MODEL2_MASKED_PATTERNS {
                    // for (uint32_t pattern : MODEL2_MASKED_PATTERNS) {
                    // 'unmask' the pattern first to get the original 5-data bits + 10-ec bits back
                    let pattern = ref_pattern ^ FORMAT_INFO_MASK_MODEL2;
                    // Find the pattern with fewest bits differing
                    let hammingDist = ((bits[bitsIndex] ^ mask) ^ pattern).count_ones();
                    // if (int hammingDist = BitHacks::CountBitsSet((bits[bitsIndex] ^ mask) ^ pattern);
                    if hammingDist < fi.hammingDistance {
                        fi.mask = *mask; // store the used mask to discriminate between types/models
                        fi.data = pattern >> 10; // drop the 10 BCH error correction bits
                        fi.hammingDistance = hammingDist;
                        fi.bitsIndex = bitsIndex as u8;
                    }
                }
            }
        }

        // // Some QR codes apparently do not apply the XOR mask. Try without and with additional masking.
        // for mask in masks {
        //     // for (auto mask : {0, mask})
        //     for (bitsIndex, bit_set) in bits.iter().enumerate() {
        //         // for (int bitsIndex = 0; bitsIndex < Size(bits); ++bitsIndex)
        //         for [pattern, _index] in FORMAT_INFO_DECODE_LOOKUP {
        //             // for (const auto& [pattern, index] : lookup) {
        //             // Find the int in lookup with fewest bits differing
        //             let hammingDist = ((bit_set ^ mask) ^ pattern).count_ones();
        //             if hammingDist < fi.hammingDistance {
        //                 // if (int hammingDist = BitHacks::CountBitsSet((bits[bitsIndex] ^ mask) ^ pattern); hammingDist < fi.hammingDistance) {
        //                     fi.mask = *mask; // store the used mask to discriminate between types/models
        //                     fi.data = pattern >> 10; // drop the 10 BCH error correction bits
        //                 fi.hammingDistance = hammingDist;
        //                 fi.bitsIndex = bitsIndex as u8;
        //             }
        //         }
        //     }
        // }

        fi
    }

    pub fn qr_type(&self) -> Type {
        match self.mask {
            FORMAT_INFO_MASK_QR_MODEL1 => Type::Model1,
            FORMAT_INFO_MASK_MICRO => Type::Micro,
            _ => Type::Model2,
        }
    }

    // Hamming distance of the 32 masked codes is 7, by construction, so <= 3 bits differing means we found a match
    pub fn isValid(&self) -> bool {
        self.hammingDistance <= 3
    }

    pub fn cpp_eq(&self, other: &Self) -> bool {
        self.data_mask == other.data_mask
            && self.error_correction_level == other.error_correction_level
            && self.qr_type() == other.qr_type()
    }
}
