use crate::qrcode::{
    cpp_port::Type,
    decoder::{
        ErrorCorrectionLevel, FormatInformation, FORMAT_INFO_MASK_MODEL2, FORMAT_INFO_MASK_QR,
    },
};

pub const FORMAT_INFO_MASK_QR_MODEL1: u32 = 0x2825;
pub const FORMAT_INFO_MASK_MICRO: u32 = 0x4445;
pub const FORMAT_INFO_MASK_RMQR: u32 = 0x1FAB2; // Finder pattern side
pub const FORMAT_INFO_MASK_RMQR_SUB: u32 = 0x20A7B; // Finder sub pattern side

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

    /**
     * @param formatInfoBits1 format info indicator, with mask still applied
     * @param formatInfoBits2 second copy of same info; both are checked at the same time to establish best match
     */
    pub fn DecodeRMQR(formatInfoBits1: u32, formatInfoBits2: u32) -> Self {
        //FormatInformation fi;
        
        let mut fi = if formatInfoBits2 != 0 {
            Self::FindBestFormatInfoRMQR(
                &[formatInfoBits1],
                &[formatInfoBits2],
            )
        } else {
            // TODO probably remove if `sampleRMQR()` done properly
            Self::FindBestFormatInfoRMQR(&[formatInfoBits1], &[])
        };

        // Bit 6 is error correction (M/H), and bits 0-5 version.
        fi.error_correction_level =
            ErrorCorrectionLevel::ECLevelFromBits(((fi.data >> 5) as u8 & 1) << 1, false); // Shift to match QRCode M/H
        fi.data_mask = 4; // ((y / 2) + (x / 3)) % 2 == 0
        fi.microVersion = (fi.data & 0x1F) + 1;
        fi.isMirrored = false; // TODO: implement mirrored format bit reading

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

    pub fn FindBestFormatInfoRMQR(bits: &[u32], subbits: &[u32]) -> Self {
        // See ISO/IEC 23941:2022, Annex C, Table C.1 - Valid format information sequences
        const MASKED_PATTERNS: [u32; 64] = [
            // Finder pattern side
            0x1FAB2, 0x1E597, 0x1DBDD, 0x1C4F8, 0x1B86C, 0x1A749, 0x19903, 0x18626, 0x17F0E,
            0x1602B, 0x15E61, 0x14144, 0x13DD0, 0x122F5, 0x11CBF, 0x1039A, 0x0F1CA, 0x0EEEF,
            0x0D0A5, 0x0CF80, 0x0B314, 0x0AC31, 0x0927B, 0x08D5E, 0x07476, 0x06B53, 0x05519,
            0x04A3C, 0x036A8, 0x0298D, 0x017C7, 0x008E2, 0x3F367, 0x3EC42, 0x3D208, 0x3CD2D,
            0x3B1B9, 0x3AE9C, 0x390D6, 0x38FF3, 0x376DB, 0x369FE, 0x357B4, 0x34891, 0x33405,
            0x32B20, 0x3156A, 0x30A4F, 0x2F81F, 0x2E73A, 0x2D970, 0x2C655, 0x2BAC1, 0x2A5E4,
            0x29BAE, 0x2848B, 0x27DA3, 0x26286, 0x25CCC, 0x243E9, 0x23F7D, 0x22058, 0x21E12,
            0x20137,
        ];
        const MASKED_PATTERNS_SUB: [u32; 64] = [
            // Finder sub pattern side
            0x20A7B, 0x2155E, 0x22B14, 0x23431, 0x248A5, 0x25780, 0x269CA, 0x276EF, 0x28FC7,
            0x290E2, 0x2AEA8, 0x2B18D, 0x2CD19, 0x2D23C, 0x2EC76, 0x2F353, 0x30103, 0x31E26,
            0x3206C, 0x33F49, 0x343DD, 0x35CF8, 0x362B2, 0x37D97, 0x384BF, 0x39B9A, 0x3A5D0,
            0x3BAF5, 0x3C661, 0x3D944, 0x3E70E, 0x3F82B, 0x003AE, 0x01C8B, 0x022C1, 0x03DE4,
            0x04170, 0x05E55, 0x0601F, 0x07F3A, 0x08612, 0x09937, 0x0A77D, 0x0B858, 0x0C4CC,
            0x0DBE9, 0x0E5A3, 0x0FA86, 0x108D6, 0x117F3, 0x129B9, 0x1369C, 0x14A08, 0x1552D,
            0x16B67, 0x17442, 0x18D6A, 0x1924F, 0x1AC05, 0x1B320, 0x1CFB4, 0x1D091, 0x1EEDB,
            0x1F1FE,
        ];

        let mut fi = FormatInformation::default();

        let mut best = |bits: &[u32], &patterns: &[u32; 64], mask: u32| {
            for bitsIndex in 0..bits.len() {
                // for (int bitsIndex = 0; bitsIndex < Size(bits); ++bitsIndex) {
                for l_pattern in patterns {
                    // for (uint32_t pattern : patterns) {
                    // 'unmask' the pattern first to get the original 6-data bits + 12-ec bits back
                    let pattern = l_pattern ^ mask;
                    // Find the pattern with fewest bits differing
                    let hammingDist = ((bits[bitsIndex] ^ mask) ^ pattern).count_ones();
                    if hammingDist < fi.hammingDistance {
                        fi.mask = mask; // store the used mask to discriminate between types/models
                        fi.data = pattern >> 12; // drop the 12 BCH error correction bits
                        fi.hammingDistance = hammingDist;
                        fi.bitsIndex = bitsIndex as u8;
                    }
                }
            }
        };

        best(bits, &MASKED_PATTERNS, FORMAT_INFO_MASK_RMQR);
        if !subbits.is_empty()
        // TODO probably remove if `sampleRMQR()` done properly
        {
            best(subbits, &MASKED_PATTERNS_SUB, FORMAT_INFO_MASK_RMQR_SUB);
        }

        fi
    }

    pub const fn qr_type(&self) -> Type {
        match self.mask {
            FORMAT_INFO_MASK_QR_MODEL1 => Type::Model1,
            FORMAT_INFO_MASK_MICRO => Type::Micro,
            FORMAT_INFO_MASK_RMQR | FORMAT_INFO_MASK_RMQR_SUB => Type::RectMicro,
            _ => Type::Model2,
        }
    }

    // Hamming distance of the 32 masked codes is 7 (64 and 8 for rMQR), by construction, so <= 3 bits differing means we found a match
    pub const fn isValid(&self) -> bool {
        self.hammingDistance <= 3
    }

    pub fn cpp_eq(&self, other: &Self) -> bool {
        self.data_mask == other.data_mask
            && self.error_correction_level == other.error_correction_level
            && self.qr_type() == other.qr_type()
    }
}
