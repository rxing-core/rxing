use crate::common::Result;

use crate::qrcode::decoder::{
    ErrorCorrectionLevel, FormatInformation, FORMAT_INFO_DECODE_LOOKUP, FORMAT_INFO_MASK_QR,
};

pub const FORMAT_INFO_DECODE_LOOKUP_MICRO: [[u32; 2]; 32] = [
    [0x4445, 0x00],
    [0x4172, 0x01],
    [0x4E2B, 0x02],
    [0x4B1C, 0x03],
    [0x55AE, 0x04],
    [0x5099, 0x05],
    [0x5FC0, 0x06],
    [0x5AF7, 0x07],
    [0x6793, 0x08],
    [0x62A4, 0x09],
    [0x6DFD, 0x0A],
    [0x68CA, 0x0B],
    [0x7678, 0x0C],
    [0x734F, 0x0D],
    [0x7C16, 0x0E],
    [0x7921, 0x0F],
    [0x06DE, 0x10],
    [0x03E9, 0x11],
    [0x0CB0, 0x12],
    [0x0987, 0x13],
    [0x1735, 0x14],
    [0x1202, 0x15],
    [0x1D5B, 0x16],
    [0x186C, 0x17],
    [0x2508, 0x18],
    [0x203F, 0x19],
    [0x2F66, 0x1A],
    [0x2A51, 0x1B],
    [0x34E3, 0x1C],
    [0x31D4, 0x1D],
    [0x3E8D, 0x1E],
    [0x3BBA, 0x1F],
];

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
        let mut fi = Self::FindBestFormatInfo(
            FORMAT_INFO_MASK_QR,
            FORMAT_INFO_DECODE_LOOKUP,
            &[
                formatInfoBits1,
                formatInfoBits2,
                Self::MirrorBits(formatInfoBits1),
                mirroredFormatInfoBits2,
            ],
        );

        // Use bits 3/4 for error correction, and 0-2 for mask.
        fi.error_correction_level =
            ErrorCorrectionLevel::ECLevelFromBits((fi.index >> 3) & 0x03, true);
        fi.data_mask = fi.index & 0x07;
        fi.isMirrored = fi.bitsIndex > 1;

        fi
    }

    pub fn DecodeMQR(formatInfoBits: u32) -> Self {
        // We don't use the additional masking (with 0x4445) to work around potentially non complying MicroQRCode encoders
        let mut fi = Self::FindBestFormatInfo(
            0,
            FORMAT_INFO_DECODE_LOOKUP_MICRO,
            &[formatInfoBits, Self::MirrorBits(formatInfoBits)],
        );

        const BITS_TO_VERSION: [u8; 8] = [1, 2, 2, 3, 3, 4, 4, 4];

        // Bits 2/3/4 contain both error correction level and version, 0/1 contain mask.
        fi.error_correction_level =
            ErrorCorrectionLevel::ECLevelFromBits((fi.index >> 2) & 0x07, true);
        fi.data_mask = (fi.index & 0x03);
        fi.microVersion = BITS_TO_VERSION[((fi.index >> 2) & 0x07) as usize] as u32;
        fi.isMirrored = fi.bitsIndex == 1;

        fi
    }

    pub fn MirrorBits(bits: u32) -> u32 {
        todo!()
        // return BitHacks::Reverse(bits) >> 17;
    }

    pub fn FindBestFormatInfo(mask: u32, lookup: [[u32; 2]; 32], bits: &[u32]) -> Self {
        todo!()
        // FormatInformation fi;

        // // Some QR codes apparently do not apply the XOR mask. Try without and with additional masking.
        // for (auto mask : {0, mask})
        // 	for (int bitsIndex = 0; bitsIndex < Size(bits); ++bitsIndex)
        // 		for (const auto& [pattern, index] : lookup) {
        // 			// Find the int in lookup with fewest bits differing
        // 			if (int hammingDist = BitHacks::CountBitsSet((bits[bitsIndex] ^ mask) ^ pattern); hammingDist < fi.hammingDistance) {
        // 				fi.index = index;
        // 				fi.hammingDistance = hammingDist;
        // 				fi.bitsIndex = bitsIndex;
        // 			}
        // 		}

        // return fi;
    }

    pub fn isValid(&self) -> bool {
        todo!()
    }
}
