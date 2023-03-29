use crate::qrcode::decoder::FormatInformation;

impl FormatInformation {
    /**
    * @param formatInfoBits1 format info indicator, with mask still applied
    * @param formatInfoBits2 second copy of same info; both are checked at the same time to establish best match
    */
    pub fn DecodeQR(formatInfoBits1: u32, formatInfoBits2: u32) -> Self {
        todo!()
        // // maks out the 'Dark Module' for mirrored and non-mirrored case (see Figure 25 in ISO/IEC 18004:2015)
        // uint32_t mirroredFormatInfoBits2 = MirrorBits(((formatInfoBits2 >> 1) & 0b111111110000000) | (formatInfoBits2 & 0b1111111));
        // formatInfoBits2 = ((formatInfoBits2 >> 1) & 0b111111100000000) | (formatInfoBits2 & 0b11111111);
        // auto fi = FindBestFormatInfo(FORMAT_INFO_MASK_QR, FORMAT_INFO_DECODE_LOOKUP,
        // 							 {formatInfoBits1, formatInfoBits2, MirrorBits(formatInfoBits1), mirroredFormatInfoBits2});

        // // Use bits 3/4 for error correction, and 0-2 for mask.
        // fi.ecLevel = ECLevelFromBits((fi.index >> 3) & 0x03);
        // fi.dataMask = static_cast<uint8_t>(fi.index & 0x07);
        // fi.isMirrored = fi.bitsIndex > 1;

        // return fi;
    }

    pub fn DecodeMQR(formatInfoBits: u32) -> Self {
        todo!()
        // // We don't use the additional masking (with 0x4445) to work around potentially non complying MicroQRCode encoders
        // auto fi = FindBestFormatInfo(0, FORMAT_INFO_DECODE_LOOKUP_MICRO, {formatInfoBits, MirrorBits(formatInfoBits)});

        // constexpr uint8_t BITS_TO_VERSION[] = {1, 2, 2, 3, 3, 4, 4, 4};

        // // Bits 2/3/4 contain both error correction level and version, 0/1 contain mask.
        // fi.ecLevel = ECLevelFromBits((fi.index >> 2) & 0x07, true);
        // fi.dataMask = static_cast<uint8_t>(fi.index & 0x03);
        // fi.microVersion = BITS_TO_VERSION[(fi.index >> 2) & 0x07];
        // fi.isMirrored = fi.bitsIndex == 1;

        // return fi;
    }

    pub fn isValid(&self) -> bool {
        todo!()
    }
}
