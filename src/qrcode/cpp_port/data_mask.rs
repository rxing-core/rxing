/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::common::BitMatrix;
use crate::common::Result;
use crate::Exceptions;

/**
* <p>Encapsulates data masks for the data bits in a QR  and micro QR code, per ISO 18004:2006 6.8.</p>
*
* <p>Note that the diagram in section 6.8.1 is misleading since it indicates that i is column position
* and j is row position. In fact, as the text says, i is row position and j is column position.</p>
*/

pub fn GetDataMaskBit(maskIndex: u32, x: u32, y: u32, isMicro: Option<bool>) -> Result<bool> {
    let isMicro = isMicro.unwrap_or(false);
    let mut maskIndex = maskIndex;
    if isMicro {
        if !(0..4).contains(&maskIndex) {
            return Err(Exceptions::illegal_argument_with(
                "QRCode maskIndex out of range",
            ));
        }
        maskIndex = [1, 4, 6, 7][maskIndex as usize]; // map from MQR to QR indices
    }

    match maskIndex {
        0 => return Ok((y + x) % 2 == 0),
        1 => return Ok(y % 2 == 0),
        2 => return Ok(x % 3 == 0),
        3 => return Ok((y + x) % 3 == 0),
        4 => return Ok(((y / 2) + (x / 3)) % 2 == 0),
        5 => return Ok((y * x) % 6 == 0),
        6 => return Ok(((y * x) % 6) < 3),
        7 => return Ok((y + x + ((y * x) % 3)) % 2 == 0),
        _ => {}
    }

    Err(Exceptions::illegal_argument_with(
        "QRCode maskIndex out of range",
    ))
}

pub fn GetMaskedBit(
    bits: &BitMatrix,
    x: u32,
    y: u32,
    maskIndex: u32,
    isMicro: Option<bool>,
) -> Result<bool> {
    Ok(GetDataMaskBit(maskIndex, x, y, isMicro)? != bits.get(x, y))
}
