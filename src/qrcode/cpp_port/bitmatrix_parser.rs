// /*
// * Copyright 2016 Nu-book Inc.
// * Copyright 2016 ZXing authors
// */
// // SPDX-License-Identifier: Apache-2.0

use crate::{
    common::{BitMatrix, Result},
    qrcode::decoder::{FormatInformation, Version, VersionRef},
    Exceptions,
};

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
    if (isMicro) {
        dimension >= 11 && dimension <= 17 && (dimension % 2) == 1
    } else {
        dimension >= 21 && dimension <= 177 && (dimension % 4) == 1
    }
}

pub fn ReadVersion(bitMatrix: &BitMatrix) -> Result<VersionRef> {
    todo!()
    // int dimension = bitMatrix.height();

    // const Version* version = Version::FromDimension(dimension);

    // if (!version || version->versionNumber() < 7)
    // 	return version;

    // for (bool mirror : {false, true}) {
    // 	// Read top-right/bottom-left version info: 3 wide by 6 tall (depending on mirrored)
    // 	int versionBits = 0;
    // 	for (int y = 5; y >= 0; --y)
    // 		for (int x = dimension - 9; x >= dimension - 11; --x)
    // 			AppendBit(versionBits, getBit(bitMatrix, x, y, mirror));

    // 	version = Version::DecodeVersionInformation(versionBits);
    // 	if (version && version->dimension() == dimension)
    // 		return version;
    // }

    // return nullptr;
}

pub fn ReadFormatInformation(bitMatrix: &BitMatrix, isMicro: bool) -> Result<FormatInformation> {
    todo!()
    // if (!hasValidDimension(bitMatrix, isMicro))
    // 	return {};

    // if (isMicro) {
    // 	// Read top-left format info bits
    // 	int formatInfoBits = 0;
    // 	for (int x = 1; x < 9; x++)
    // 		AppendBit(formatInfoBits, getBit(bitMatrix, x, 8));
    // 	for (int y = 7; y >= 1; y--)
    // 		AppendBit(formatInfoBits, getBit(bitMatrix, 8, y));

    // 	return FormatInformation::DecodeMQR(formatInfoBits);
    // }

    // // Read top-left format info bits
    // int formatInfoBits1 = 0;
    // for (int x = 0; x < 6; x++)
    // 	AppendBit(formatInfoBits1, getBit(bitMatrix, x, 8));
    // // .. and skip a bit in the timing pattern ...
    // AppendBit(formatInfoBits1, getBit(bitMatrix, 7, 8));
    // AppendBit(formatInfoBits1, getBit(bitMatrix, 8, 8));
    // AppendBit(formatInfoBits1, getBit(bitMatrix, 8, 7));
    // // .. and skip a bit in the timing pattern ...
    // for (int y = 5; y >= 0; y--)
    // 	AppendBit(formatInfoBits1, getBit(bitMatrix, 8, y));

    // // Read the top-right/bottom-left pattern including the 'Dark Module' from the bottom-left
    // // part that has to be considered separately when looking for mirrored symbols.
    // // See also FormatInformation::DecodeQR
    // int dimension = bitMatrix.height();
    // int formatInfoBits2 = 0;
    // for (int y = dimension - 1; y >= dimension - 8; y--)
    // 	AppendBit(formatInfoBits2, getBit(bitMatrix, 8, y));
    // for (int x = dimension - 8; x < dimension; x++)
    // 	AppendBit(formatInfoBits2, getBit(bitMatrix, x, 8));

    // return FormatInformation::DecodeQR(formatInfoBits1, formatInfoBits2);
}

pub fn ReadQRCodewords(
    bitMatrix: &BitMatrix,
    version: VersionRef,
    formatInfo: &FormatInformation,
) -> Result<Vec<u8>> {
    todo!()
    // BitMatrix functionPattern = version.buildFunctionPattern();

    // ByteArray result;
    // result.reserve(version.totalCodewords());
    // uint8_t currentByte = 0;
    // bool readingUp = true;
    // int bitsRead = 0;
    // int dimension = bitMatrix.height();
    // // Read columns in pairs, from right to left
    // for (int x = dimension - 1; x > 0; x -= 2) {
    // 	// Skip whole column with vertical timing pattern.
    // 	if (x == 6)
    // 		x--;
    // 	// Read alternatingly from bottom to top then top to bottom
    // 	for (int row = 0; row < dimension; row++) {
    // 		int y = readingUp ? dimension - 1 - row : row;
    // 		for (int col = 0; col < 2; col++) {
    // 			int xx = x - col;
    // 			// Ignore bits covered by the function pattern
    // 			if (!functionPattern.get(xx, y)) {
    // 				// Read a bit
    // 				AppendBit(currentByte,
    // 						  GetDataMaskBit(formatInfo.dataMask, xx, y) != getBit(bitMatrix, xx, y, formatInfo.isMirrored));
    // 				// If we've made a whole byte, save it off
    // 				if (++bitsRead % 8 == 0)
    // 					result.push_back(std::exchange(currentByte, 0));
    // 			}
    // 		}
    // 	}
    // 	readingUp = !readingUp; // switch directions
    // }
    // if (Size(result) != version.totalCodewords())
    // 	return {};

    // return result;
}

pub fn ReadMQRCodewords(
    bitMatrix: &BitMatrix,
    version: VersionRef,
    formatInfo: &FormatInformation,
) -> Result<Vec<u8>> {
    todo!()
    // BitMatrix functionPattern = version.buildFunctionPattern();

    // // D3 in a Version M1 symbol, D11 in a Version M3-L symbol and D9
    // // in a Version M3-M symbol is a 2x2 square 4-module block.
    // // See ISO 18004:2006 6.7.3.
    // bool hasD4mBlock = version.versionNumber() % 2 == 1;
    // int d4mBlockIndex =
    // 	version.versionNumber() == 1 ? 3 : (formatInfo.ecLevel == QRCode::ErrorCorrectionLevel::Low ? 11 : 9);

    // ByteArray result;
    // result.reserve(version.totalCodewords());
    // uint8_t currentByte = 0;
    // bool readingUp = true;
    // int bitsRead = 0;
    // int dimension = bitMatrix.height();
    // // Read columns in pairs, from right to left
    // for (int x = dimension - 1; x > 0; x -= 2) {
    // 	// Read alternatingly from bottom to top then top to bottom
    // 	for (int row = 0; row < dimension; row++) {
    // 		int y = readingUp ? dimension - 1 - row : row;
    // 		for (int col = 0; col < 2; col++) {
    // 			int xx = x - col;
    // 			// Ignore bits covered by the function pattern
    // 			if (!functionPattern.get(xx, y)) {
    // 				// Read a bit
    // 				AppendBit(currentByte,
    // 						  GetDataMaskBit(formatInfo.dataMask, xx, y, true) != getBit(bitMatrix, xx, y, formatInfo.isMirrored));
    // 				++bitsRead;
    // 				// If we've made a whole byte, save it off; save early if 2x2 data block.
    // 				if (bitsRead == 8 || (bitsRead == 4 && hasD4mBlock && Size(result) == d4mBlockIndex - 1)) {
    // 					result.push_back(std::exchange(currentByte, 0));
    // 					bitsRead = 0;
    // 				}
    // 			}
    // 		}
    // 	}
    // 	readingUp = !readingUp; // switch directions
    // }
    // if (Size(result) != version.totalCodewords())
    // 	return {};

    // return result;
}

pub fn ReadCodewords(
    bitMatrix: &BitMatrix,
    version: VersionRef,
    formatInfo: &FormatInformation,
) -> Result<Vec<u8>> {
    if (!hasValidDimension(bitMatrix, version.isMicroQRCode())) {
        return Err(Exceptions::FORMAT);
    }

    if version.isMicroQRCode() {
        ReadMQRCodewords(bitMatrix, version, formatInfo)
    } else {
        ReadQRCodewords(bitMatrix, version, formatInfo)
    }
}
