/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::common::BitMatrix;

/**
* <p>Encapsulates data masks for the data bits in a QR  and micro QR code, per ISO 18004:2006 6.8.</p>
*
* <p>Note that the diagram in section 6.8.1 is misleading since it indicates that i is column position
* and j is row position. In fact, as the text says, i is row position and j is column position.</p>
*/

pub fn GetDataMaskBit(maskIndex: u32, x: u32, y: u32, isMicro: Option<bool>) -> bool {
    let isMicro = isMicro.unwrap_or(false);
    todo!()
    // if (isMicro) {
    // 	if (maskIndex < 0 || maskIndex >= 4)
    // 		throw std::invalid_argument("QRCode maskIndex out of range");
    // 	maskIndex = std::array{1, 4, 6, 7}[maskIndex]; // map from MQR to QR indices
    // }

    // switch (maskIndex) {
    // case 0: return (y + x) % 2 == 0;
    // case 1: return y % 2 == 0;
    // case 2: return x % 3 == 0;
    // case 3: return (y + x) % 3 == 0;
    // case 4: return ((y / 2) + (x / 3)) % 2 == 0;
    // case 5: return (y * x) % 6 == 0;
    // case 6: return ((y * x) % 6) < 3;
    // case 7: return (y + x + ((y * x) % 3)) % 2 == 0;
    // }

    // throw std::invalid_argument("QRCode maskIndex out of range");
}

pub fn GetMaskedBit(
    bits: &BitMatrix,
    x: u32,
    y: u32,
    maskIndex: u32,
    isMicro: Option<bool>,
) -> bool {
    return GetDataMaskBit(maskIndex, x, y, isMicro) != bits.get(x, y);
}
