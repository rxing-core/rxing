use crate::qrcode::decoder::FormatInformation;

impl FormatInformation {
    pub fn DecodeMQR(formatInfoBits: u32) -> Self {
        unimplemented!()
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
        unimplemented!()
    }
}
