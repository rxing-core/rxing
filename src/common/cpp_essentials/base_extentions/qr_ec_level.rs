use crate::qrcode::decoder::ErrorCorrectionLevel;

impl ErrorCorrectionLevel {
    pub fn ECLevelFromBitsSigned(bits: i8, isMicro: bool) -> Self {
        if isMicro {
            let LEVEL_FOR_BITS: [ErrorCorrectionLevel; 8] = [
                ErrorCorrectionLevel::L,
                ErrorCorrectionLevel::L,
                ErrorCorrectionLevel::M,
                ErrorCorrectionLevel::L,
                ErrorCorrectionLevel::M,
                ErrorCorrectionLevel::L,
                ErrorCorrectionLevel::M,
                ErrorCorrectionLevel::Q,
            ];
            return LEVEL_FOR_BITS[bits as usize & 0x07];
        }
        let LEVEL_FOR_BITS: [ErrorCorrectionLevel; 4] = [
            ErrorCorrectionLevel::M,
            ErrorCorrectionLevel::L,
            ErrorCorrectionLevel::H,
            ErrorCorrectionLevel::Q,
        ];
        LEVEL_FOR_BITS[bits as usize & 0x3]
    }

    pub fn ECLevelFromBits(bits: u8, isMicro: bool) -> Self {
        Self::ECLevelFromBitsSigned(bits as i8, isMicro)
    }
}
