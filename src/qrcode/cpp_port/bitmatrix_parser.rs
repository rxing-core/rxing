// /*
// * Copyright 2016 Nu-book Inc.
// * Copyright 2016 ZXing authors
// */
// // SPDX-License-Identifier: Apache-2.0

use crate::{
    common::{BitMatrix, Result},
    qrcode::decoder::{ErrorCorrectionLevel, FormatInformation, Version, VersionRef},
    Exceptions,
};

use super::{data_mask::GetDataMaskBit, detector::AppendBit};

pub fn getBit(bitMatrix: &BitMatrix, x: u32, y: u32, mirrored: Option<bool>) -> bool {
    let mirrored = mirrored.unwrap_or(false);
    if mirrored {
        bitMatrix.get(y, x)
    } else {
        bitMatrix.get(x, y)
    }
}

pub fn hasValidDimension(bitMatrix: &BitMatrix, isMicro: bool) -> bool {
    let dimension = bitMatrix.height();
    if isMicro {
        (11..=17).contains(&dimension) && (dimension % 2) == 1
    } else {
        (21..=177).contains(&dimension) && (dimension % 4) == 1
    }
}

pub fn ReadVersion(bitMatrix: &BitMatrix) -> Result<VersionRef> {
    let dimension = bitMatrix.height();

    let mut version = Version::FromDimension(dimension)?;

    if version.getVersionNumber() < 7 {
        return Ok(version);
    }

    for mirror in [false, true] {
        // for (bool mirror : {false, true}) {
        // Read top-right/bottom-left version info: 3 wide by 6 tall (depending on mirrored)
        let mut versionBits = 0;
        for y in (0..=5).rev() {
            // for (int y = 5; y >= 0; --y) {
            for x in ((dimension - 11)..=(dimension - 9)).rev() {
                // for (int x = dimension - 9; x >= dimension - 11; --x) {
                AppendBit(&mut versionBits, getBit(bitMatrix, x, y, Some(mirror)));
            }
        }

        version = Version::DecodeVersionInformation(versionBits, 0)?; // THIS MIGHT BE WRONG todo!()
        if version.getDimensionForVersion() == dimension {
            return Ok(version);
        }
    }

    Err(Exceptions::FORMAT)
}

pub fn ReadFormatInformation(bitMatrix: &BitMatrix, isMicro: bool) -> Result<FormatInformation> {
    if !hasValidDimension(bitMatrix, isMicro) {
        return Err(Exceptions::FORMAT);
    }

    if isMicro {
        // Read top-left format info bits
        let mut formatInfoBits = 0;
        for x in 1..9 {
            // for (int x = 1; x < 9; x++)
            AppendBit(&mut formatInfoBits, getBit(bitMatrix, x, 8, None));
        }
        for y in (1..=7).rev() {
            // for (int y = 7; y >= 1; y--)
            AppendBit(&mut formatInfoBits, getBit(bitMatrix, 8, y, None));
        }

        return Ok(FormatInformation::DecodeMQR(formatInfoBits as u32));
    }

    // Read top-left format info bits
    let mut formatInfoBits1 = 0;
    for x in 0..6 {
        // for (int x = 0; x < 6; x++)
        AppendBit(&mut formatInfoBits1, getBit(bitMatrix, x, 8, None));
    }
    // .. and skip a bit in the timing pattern ...
    AppendBit(&mut formatInfoBits1, getBit(bitMatrix, 7, 8, None));
    AppendBit(&mut formatInfoBits1, getBit(bitMatrix, 8, 8, None));
    AppendBit(&mut formatInfoBits1, getBit(bitMatrix, 8, 7, None));
    // .. and skip a bit in the timing pattern ...
    for y in (0..=5).rev() {
        // for (int y = 5; y >= 0; y--)
        AppendBit(&mut formatInfoBits1, getBit(bitMatrix, 8, y, None));
    }

    // Read the top-right/bottom-left pattern including the 'Dark Module' from the bottom-left
    // part that has to be considered separately when looking for mirrored symbols.
    // See also FormatInformation::DecodeQR
    let dimension = bitMatrix.height();
    let mut formatInfoBits2 = 0;
    for y in ((dimension - 8)..=(dimension - 1)).rev() {
        // for (int y = dimension - 1; y >= dimension - 8; y--)
        AppendBit(&mut formatInfoBits2, getBit(bitMatrix, 8, y, None));
    }
    for x in (dimension - 8)..dimension {
        // for (int x = dimension - 8; x < dimension; x++)
        AppendBit(&mut formatInfoBits2, getBit(bitMatrix, x, 8, None));
    }

    Ok(FormatInformation::DecodeQR(
        formatInfoBits1 as u32,
        formatInfoBits2 as u32,
    ))
}

pub fn ReadQRCodewords(
    bitMatrix: &BitMatrix,
    version: VersionRef,
    formatInfo: &FormatInformation,
) -> Result<Vec<u8>> {
    let functionPattern: BitMatrix = version.buildFunctionPattern()?;

    let mut result = Vec::with_capacity(version.getTotalCodewords() as usize);
    let mut currentByte = 0;
    let mut readingUp = true;
    let mut bitsRead = 0;
    let dimension = bitMatrix.height();
    // Read columns in pairs, from right to left
    let mut x = (dimension as i32) - 1;
    while x > 0 {
        // for (int x = dimension - 1; x > 0; x -= 2) {
        // Skip whole column with vertical timing pattern.
        if x == 6 {
            x -= 1;
        }
        // Read alternatingly from bottom to top then top to bottom
        for row in 0..dimension {
            // for (int row = 0; row < dimension; row++) {
            let y = if readingUp { dimension - 1 - row } else { row };
            for col in 0..2 {
                // for (int col = 0; col < 2; col++) {
                let xx = (x - col) as u32;
                // Ignore bits covered by the function pattern
                if !functionPattern.get(xx, y) {
                    // Read a bit
                    AppendBit(
                        &mut currentByte,
                        GetDataMaskBit(formatInfo.data_mask as u32, xx, y, None)?
                            != getBit(bitMatrix, xx, y, Some(formatInfo.isMirrored)),
                    );
                    // If we've made a whole byte, save it off
                    bitsRead += 1;
                    if bitsRead % 8 == 0 {
                        result.push(std::mem::take(&mut currentByte));
                    }
                }
            }
        }
        readingUp = !readingUp; // switch directions

        x -= 2;
    }
    if (result.len()) != version.getTotalCodewords() as usize {
        return Err(Exceptions::FORMAT);
    }

    Ok(result.iter().copied().map(|x| x as u8).collect())
}

pub fn ReadMQRCodewords(
    bitMatrix: &BitMatrix,
    version: VersionRef,
    formatInfo: &FormatInformation,
) -> Result<Vec<u8>> {
    let functionPattern = version.buildFunctionPattern()?;

    // D3 in a Version M1 symbol, D11 in a Version M3-L symbol and D9
    // in a Version M3-M symbol is a 2x2 square 4-module block.
    // See ISO 18004:2006 6.7.3.
    let hasD4mBlock = version.getVersionNumber() % 2 == 1;
    let d4mBlockIndex = if version.getVersionNumber() == 1 {
        3
    } else if formatInfo.error_correction_level == ErrorCorrectionLevel::L {
        11
    } else {
        9
    };

    let mut result = Vec::with_capacity(version.getTotalCodewords() as usize);
    let mut currentByte = 0;
    let mut readingUp = true;
    let mut bitsRead = 0;
    let dimension = bitMatrix.height();
    // Read columns in pairs, from right to left
    let mut x = dimension - 1;
    while x > 0 {
        // for (int x = dimension - 1; x > 0; x -= 2) {
        // Read alternatingly from bottom to top then top to bottom
        for row in 0..dimension {
            // for (int row = 0; row < dimension; row++) {
            let y = if readingUp { dimension - 1 - row } else { row };
            for col in 0..2 {
                // for (int col = 0; col < 2; col++) {
                let xx = x - col;
                // Ignore bits covered by the function pattern
                if !functionPattern.get(xx, y) {
                    // Read a bit
                    AppendBit(
                        &mut currentByte,
                        GetDataMaskBit(formatInfo.data_mask as u32, xx, y, Some(true))?
                            != getBit(bitMatrix, xx, y, Some(formatInfo.isMirrored)),
                    );
                    bitsRead += 1;
                    // If we've made a whole byte, save it off; save early if 2x2 data block.
                    if bitsRead == 8
                        || (bitsRead == 4 && hasD4mBlock && (result.len()) == d4mBlockIndex - 1)
                    {
                        result.push(std::mem::take(&mut currentByte));
                        bitsRead = 0;
                    }
                }
            }
        }
        readingUp = !readingUp; // switch directions

        x -= 2;
    }
    if (result.len()) != version.getTotalCodewords() as usize {
        return Err(Exceptions::FORMAT);
    }

    Ok(result.iter().copied().map(|x| x as u8).collect())
}

pub fn ReadCodewords(
    bitMatrix: &BitMatrix,
    version: VersionRef,
    formatInfo: &FormatInformation,
) -> Result<Vec<u8>> {
    if !hasValidDimension(bitMatrix, version.isMicroQRCode()) {
        return Err(Exceptions::FORMAT);
    }

    if version.isMicroQRCode() {
        ReadMQRCodewords(bitMatrix, version, formatInfo)
    } else {
        ReadQRCodewords(bitMatrix, version, formatInfo)
    }
}
