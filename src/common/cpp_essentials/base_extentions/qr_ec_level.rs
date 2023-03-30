use crate::common::Result;
use crate::qrcode::decoder::ErrorCorrectionLevel;

impl ErrorCorrectionLevel {
    pub fn ECLevelFromBits(bits: u8, isMicro: bool) -> Self {
        if (isMicro) {
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
        return LEVEL_FOR_BITS[bits as usize & 0x3];
    }
}
