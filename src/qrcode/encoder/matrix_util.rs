/*
 * Copyright 2008 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    common::{BitArray, Result},
    qrcode::decoder::{ErrorCorrectionLevel, Version},
    Exceptions,
};

use super::{mask_util, ByteMatrix, QRCode};

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */

const POSITION_DETECTION_PATTERN: [[u8; 7]; 7] = [
    [1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1],
];

const POSITION_ADJUSTMENT_PATTERN: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
];

// From Appendix E. Table 1, JIS0510X:2004 (p 71). The table was double-checked by komatsu.
const POSITION_ADJUSTMENT_PATTERN_COORDINATE_TABLE: [[i16; 7]; 40] = [
    [-1, -1, -1, -1, -1, -1, -1],   // Version 1
    [6, 18, -1, -1, -1, -1, -1],    // Version 2
    [6, 22, -1, -1, -1, -1, -1],    // Version 3
    [6, 26, -1, -1, -1, -1, -1],    // Version 4
    [6, 30, -1, -1, -1, -1, -1],    // Version 5
    [6, 34, -1, -1, -1, -1, -1],    // Version 6
    [6, 22, 38, -1, -1, -1, -1],    // Version 7
    [6, 24, 42, -1, -1, -1, -1],    // Version 8
    [6, 26, 46, -1, -1, -1, -1],    // Version 9
    [6, 28, 50, -1, -1, -1, -1],    // Version 10
    [6, 30, 54, -1, -1, -1, -1],    // Version 11
    [6, 32, 58, -1, -1, -1, -1],    // Version 12
    [6, 34, 62, -1, -1, -1, -1],    // Version 13
    [6, 26, 46, 66, -1, -1, -1],    // Version 14
    [6, 26, 48, 70, -1, -1, -1],    // Version 15
    [6, 26, 50, 74, -1, -1, -1],    // Version 16
    [6, 30, 54, 78, -1, -1, -1],    // Version 17
    [6, 30, 56, 82, -1, -1, -1],    // Version 18
    [6, 30, 58, 86, -1, -1, -1],    // Version 19
    [6, 34, 62, 90, -1, -1, -1],    // Version 20
    [6, 28, 50, 72, 94, -1, -1],    // Version 21
    [6, 26, 50, 74, 98, -1, -1],    // Version 22
    [6, 30, 54, 78, 102, -1, -1],   // Version 23
    [6, 28, 54, 80, 106, -1, -1],   // Version 24
    [6, 32, 58, 84, 110, -1, -1],   // Version 25
    [6, 30, 58, 86, 114, -1, -1],   // Version 26
    [6, 34, 62, 90, 118, -1, -1],   // Version 27
    [6, 26, 50, 74, 98, 122, -1],   // Version 28
    [6, 30, 54, 78, 102, 126, -1],  // Version 29
    [6, 26, 52, 78, 104, 130, -1],  // Version 30
    [6, 30, 56, 82, 108, 134, -1],  // Version 31
    [6, 34, 60, 86, 112, 138, -1],  // Version 32
    [6, 30, 58, 86, 114, 142, -1],  // Version 33
    [6, 34, 62, 90, 118, 146, -1],  // Version 34
    [6, 30, 54, 78, 102, 126, 150], // Version 35
    [6, 24, 50, 76, 102, 128, 154], // Version 36
    [6, 28, 54, 80, 106, 132, 158], // Version 37
    [6, 32, 58, 84, 110, 136, 162], // Version 38
    [6, 26, 54, 82, 110, 138, 166], // Version 39
    [6, 30, 58, 86, 114, 142, 170], // Version 40
];

// Type info cells at the left top corner.
const TYPE_INFO_COORDINATES: [[u32; 2]; 15] = [
    [8, 0],
    [8, 1],
    [8, 2],
    [8, 3],
    [8, 4],
    [8, 5],
    [8, 7],
    [8, 8],
    [7, 8],
    [5, 8],
    [4, 8],
    [3, 8],
    [2, 8],
    [1, 8],
    [0, 8],
];

// From Appendix D in JISX0510:2004 (p. 67)
const VERSION_INFO_POLY: u32 = 0x1f25; // 1 1111 0010 0101

// From Appendix C in JISX0510:2004 (p.65).
const TYPE_INFO_POLY: u32 = 0x537;
const TYPE_INFO_MASK_PATTERN: u32 = 0x5412;

// Set all cells to -1.  -1 means that the cell is empty (not set yet).
//
// JAVAPORT: We shouldn't need to do this at all. The code should be rewritten to begin encoding
// with the ByteMatrix initialized all to zero.
pub fn clearMatrix(matrix: &mut ByteMatrix) {
    matrix.clear(-1i8 as u8);
}

// Build 2D matrix of QR Code from "dataBits" with "ecLevel", "version" and "getMaskPattern". On
// success, store the result in "matrix" and return true.
pub fn buildMatrix(
    dataBits: &BitArray,
    ecLevel: &ErrorCorrectionLevel,
    version: &Version,
    maskPattern: i32,
    matrix: &mut ByteMatrix,
) -> Result<()> {
    clearMatrix(matrix);
    embedBasicPatterns(version, matrix)?;
    // Type information appear with any version.
    embedTypeInfo(ecLevel, maskPattern, matrix)?;
    // Version info appear if version >= 7.
    maybeEmbedVersionInfo(version, matrix)?;
    // Data should be embedded at end.
    embedDataBits(dataBits, maskPattern, matrix)?;
    Ok(())
}

// Embed basic patterns. On success, modify the matrix and return true.
// The basic patterns are:
// - Position detection patterns
// - Timing patterns
// - Dark dot at the left bottom corner
// - Position adjustment patterns, if need be
pub fn embedBasicPatterns(version: &Version, matrix: &mut ByteMatrix) -> Result<()> {
    // Let's get started with embedding big squares at corners.
    embedPositionDetectionPatternsAndSeparators(matrix)?;
    // Then, embed the dark dot at the left bottom corner.
    embedDarkDotAtLeftBottomCorner(matrix)?;

    // Position adjustment patterns appear if version >= 2.
    maybeEmbedPositionAdjustmentPatterns(version, matrix);
    // Timing patterns should be embedded after position adj. patterns.
    embedTimingPatterns(matrix);
    Ok(())
}

// Embed type information. On success, modify the matrix.
pub fn embedTypeInfo(
    ecLevel: &ErrorCorrectionLevel,
    maskPattern: i32,
    matrix: &mut ByteMatrix,
) -> Result<()> {
    let mut typeInfoBits = BitArray::new();
    makeTypeInfoBits(ecLevel, maskPattern as u32, &mut typeInfoBits)?;

    for (i, coordinates) in TYPE_INFO_COORDINATES
        .iter()
        .enumerate()
        .take(typeInfoBits.get_size())
    {
        // Place bits in LSB to MSB order.  LSB (least significant bit) is the last value in
        // "typeInfoBits".
        let bit = typeInfoBits.get(typeInfoBits.get_size() - 1 - i);

        // Type info bits at the left top corner. See 8.9 of JISX0510:2004 (p.46).
        // let coordinates = TYPE_INFO_COORDINATES[i];
        let x1 = coordinates[0];
        let y1 = coordinates[1];
        matrix.set_bool(x1, y1, bit);

        let x2;
        let y2;
        if i < 8 {
            // Right top corner.
            x2 = matrix.getWidth() - i as u32 - 1;
            y2 = 8;
        } else {
            // Left bottom corner.
            x2 = 8;
            y2 = matrix.getHeight() - 7 + (i as u32 - 8);
        }
        matrix.set_bool(x2, y2, bit);
    }
    Ok(())
}

// Embed version information if need be. On success, modify the matrix and return true.
// See 8.10 of JISX0510:2004 (p.47) for how to embed version information.
pub fn maybeEmbedVersionInfo(version: &Version, matrix: &mut ByteMatrix) -> Result<()> {
    if version.getVersionNumber() < 7 {
        // Version info is necessary if version >= 7.
        return Ok(()); // Don't need version info.
    }
    let mut versionInfoBits = BitArray::new();
    makeVersionInfoBits(version, &mut versionInfoBits)?;

    let mut bitIndex = 6 * 3 - 1; // It will decrease from 17 to 0.
    for i in 0..6 {
        for j in 0..3 {
            // Place bits in LSB (least significant bit) to MSB order.
            let bit = versionInfoBits.get(bitIndex);
            bitIndex = bitIndex.saturating_sub(1);
            // Left bottom corner.
            matrix.set_bool(i, matrix.getHeight() - 11 + j, bit);
            // Right bottom corner.
            matrix.set_bool(matrix.getHeight() - 11 + j, i, bit);
        }
    }
    Ok(())
}

// Embed "dataBits" using "getMaskPattern". On success, modify the matrix and return true.
// For debugging purposes, it skips masking process if "getMaskPattern" is -1.
// See 8.7 of JISX0510:2004 (p.38) for how to embed data bits.
pub fn embedDataBits(dataBits: &BitArray, maskPattern: i32, matrix: &mut ByteMatrix) -> Result<()> {
    let mut bitIndex = 0;
    let mut direction: i32 = -1;
    // Start from the right bottom cell.
    let mut x = matrix.getWidth() as i32 - 1;
    let mut y = matrix.getHeight() as i32 - 1;
    while x > 0 {
        // Skip the vertical timing pattern.
        if x == 6 {
            x -= 1;
        }
        while y >= 0 && y < matrix.getHeight() as i32 {
            for i in 0..2 {
                let xx = x - i;
                // Skip the cell if it's not empty.
                if !isEmpty(matrix.get(xx as u32, y as u32)) {
                    continue;
                }
                let mut bit;
                if bitIndex < dataBits.get_size() {
                    bit = dataBits.get(bitIndex);
                    bitIndex += 1;
                } else {
                    // Padding bit. If there is no bit left, we'll fill the left cells with 0, as described
                    // in 8.4.9 of JISX0510:2004 (p. 24).
                    bit = false;
                }

                // Skip masking if mask_pattern is -1.
                if maskPattern != -1
                    && mask_util::getDataMaskBit(maskPattern as u32, xx as u32, y as u32)?
                {
                    bit = !bit;
                }
                matrix.set_bool(xx as u32, y as u32, bit);
            }
            y += direction;
        }
        direction = -direction; // Reverse the direction.
        y += direction;
        x -= 2; // Move to the left.
    }
    // All bits should be consumed.
    if bitIndex != dataBits.get_size() {
        return Err(Exceptions::writer_with(format!(
            "Not all bits consumed: {}/{}",
            bitIndex,
            dataBits.get_size()
        )));
    }
    Ok(())
}

// Return the position of the most significant bit set (to one) in the "value". The most
// significant bit is position 32. If there is no bit set, return 0. Examples:
// - findMSBSet(0) => 0
// - findMSBSet(1) => 1
// - findMSBSet(255) => 8
pub fn findMSBSet(value: u32) -> u32 {
    32 - value.leading_zeros()
}

// Calculate BCH (Bose-Chaudhuri-Hocquenghem) code for "value" using polynomial "poly". The BCH
// code is used for encoding type information and version information.
// Example: Calculation of version information of 7.
// f(x) is created from 7.
//   - 7 = 000111 in 6 bits
//   - f(x) = x^2 + x^1 + x^0
// g(x) is given by the standard (p. 67)
//   - g(x) = x^12 + x^11 + x^10 + x^9 + x^8 + x^5 + x^2 + 1
// Multiply f(x) by x^(18 - 6)
//   - f'(x) = f(x) * x^(18 - 6)
//   - f'(x) = x^14 + x^13 + x^12
// Calculate the remainder of f'(x) / g(x)
//         x^2
//         __________________________________________________
//   g(x) )x^14 + x^13 + x^12
//         x^14 + x^13 + x^12 + x^11 + x^10 + x^7 + x^4 + x^2
//         --------------------------------------------------
//                              x^11 + x^10 + x^7 + x^4 + x^2
//
// The remainder is x^11 + x^10 + x^7 + x^4 + x^2
// Encode it in binary: 110010010100
// The return value is 0xc94 (1100 1001 0100)
//
// Since all coefficients in the polynomials are 1 or 0, we can do the calculation by bit
// operations. We don't care if coefficients are positive or negative.
pub fn calculateBCHCode(value: u32, poly: u32) -> Result<u32> {
    if poly == 0 {
        return Err(Exceptions::illegal_argument_with("0 polynomial"));
    }
    let mut value = value;
    // If poly is "1 1111 0010 0101" (version info poly), msbSetInPoly is 13. We'll subtract 1
    // from 13 to make it 12.
    let msbSetInPoly = findMSBSet(poly);
    value <<= msbSetInPoly - 1;
    // Do the division business using exclusive-or operations.
    while findMSBSet(value) >= msbSetInPoly {
        value ^= poly << (findMSBSet(value) - msbSetInPoly);
    }
    // Now the "value" is the remainder (i.e. the BCH code)
    Ok(value)
}

// Make bit vector of type information. On success, store the result in "bits" and return true.
// Encode error correction level and mask pattern. See 8.9 of
// JISX0510:2004 (p.45) for details.
pub fn makeTypeInfoBits(
    ecLevel: &ErrorCorrectionLevel,
    maskPattern: u32,
    bits: &mut BitArray,
) -> Result<()> {
    if !QRCode::isValidMaskPattern(maskPattern as i32) {
        return Err(Exceptions::writer_with("Invalid mask pattern"));
    }
    let typeInfo = (ecLevel.get_value() << 3) as u32 | maskPattern;
    bits.appendBits(typeInfo, 5)?;

    let bchCode = calculateBCHCode(typeInfo, TYPE_INFO_POLY)?;
    bits.appendBits(bchCode, 10)?;

    let mut maskBits = BitArray::new();
    maskBits.appendBits(TYPE_INFO_MASK_PATTERN, 15)?;
    bits.xor(&maskBits)?;

    if bits.get_size() != 15 {
        // Just in case.
        return Err(Exceptions::writer_with(format!(
            "should not happen but we got: {}",
            bits.get_size()
        )));
    }
    Ok(())
}

// Make bit vector of version information. On success, store the result in "bits" and return true.
// See 8.10 of JISX0510:2004 (p.45) for details.
pub fn makeVersionInfoBits(version: &Version, bits: &mut BitArray) -> Result<()> {
    bits.appendBits(version.getVersionNumber(), 6)?;
    let bchCode = calculateBCHCode(version.getVersionNumber(), VERSION_INFO_POLY)?;
    bits.appendBits(bchCode, 12)?;

    if bits.get_size() != 18 {
        // Just in case.
        return Err(Exceptions::writer_with(format!(
            "should not happen but we got: {}",
            bits.get_size()
        )));
    }
    Ok(())
}

// Check if "value" is empty.
pub fn isEmpty(value: u8) -> bool {
    value == -1i8 as u8
}

pub fn embedTimingPatterns(matrix: &mut ByteMatrix) {
    // -8 is for skipping position detection patterns (size 7), and two horizontal/vertical
    // separation patterns (size 1). Thus, 8 = 7 + 1.
    for i in 8..matrix.getWidth() - 8 {
        // for (int i = 8; i < matrix.getWidth() - 8; ++i) {
        let bit = (i as u8 + 1) % 2;
        // Horizontal line.
        if isEmpty(matrix.get(i, 6)) {
            matrix.set(i, 6, bit);
        }
        // Vertical line.
        if isEmpty(matrix.get(6, i)) {
            matrix.set(6, i, bit);
        }
    }
}

// Embed the lonely dark dot at left bottom corner. JISX0510:2004 (p.46)
pub fn embedDarkDotAtLeftBottomCorner(matrix: &mut ByteMatrix) -> Result<()> {
    if matrix.get(8, matrix.getHeight() - 8) == 0 {
        return Err(Exceptions::WRITER);
    }
    matrix.set(8, matrix.getHeight() - 8, 1);
    Ok(())
}

pub fn embedHorizontalSeparationPattern(
    xStart: u32,
    yStart: u32,
    matrix: &mut ByteMatrix,
) -> Result<()> {
    for x in 0..8 {
        if !isEmpty(matrix.get(xStart + x, yStart)) {
            return Err(Exceptions::WRITER);
        }
        matrix.set(xStart + x, yStart, 0);
    }
    Ok(())
}

pub fn embedVerticalSeparationPattern(
    xStart: u32,
    yStart: u32,
    matrix: &mut ByteMatrix,
) -> Result<()> {
    for y in 0..7 {
        if !isEmpty(matrix.get(xStart, yStart + y)) {
            return Err(Exceptions::WRITER);
        }
        matrix.set(xStart, yStart + y, 0);
    }
    Ok(())
}

pub fn embedPositionAdjustmentPattern(xStart: u32, yStart: u32, matrix: &mut ByteMatrix) {
    for (y, patternY) in POSITION_ADJUSTMENT_PATTERN.iter().enumerate() {
        for x in 0..5 {
            matrix.set(xStart + x, yStart + y as u32, patternY[x as usize]);
        }
    }
}

pub fn embedPositionDetectionPattern(xStart: u32, yStart: u32, matrix: &mut ByteMatrix) {
    for (y, patternY) in POSITION_DETECTION_PATTERN.iter().enumerate() {
        for x in 0..7 {
            matrix.set(xStart + x, yStart + y as u32, patternY[x as usize]);
        }
    }
}

// Embed position detection patterns and surrounding vertical/horizontal separators.
pub fn embedPositionDetectionPatternsAndSeparators(matrix: &mut ByteMatrix) -> Result<()> {
    // Embed three big squares at corners.
    let pdpWidth = POSITION_DETECTION_PATTERN[0].len() as u32;
    // Left top corner.
    embedPositionDetectionPattern(0, 0, matrix);
    // Right top corner.
    embedPositionDetectionPattern(matrix.getWidth() - pdpWidth, 0, matrix);
    // Left bottom corner.
    embedPositionDetectionPattern(0, matrix.getWidth() - pdpWidth, matrix);

    // Embed horizontal separation patterns around the squares.
    let hspWidth = 8;
    // Left top corner.
    embedHorizontalSeparationPattern(0, hspWidth - 1, matrix)?;
    // Right top corner.
    embedHorizontalSeparationPattern(matrix.getWidth() - hspWidth, hspWidth - 1, matrix)?;
    // Left bottom corner.
    embedHorizontalSeparationPattern(0, matrix.getWidth() - hspWidth, matrix)?;

    // Embed vertical separation patterns around the squares.
    let vspSize = 7;
    // Left top corner.
    embedVerticalSeparationPattern(vspSize, 0, matrix)?;
    // Right top corner.
    embedVerticalSeparationPattern(matrix.getHeight() - vspSize - 1, 0, matrix)?;
    // Left bottom corner.
    embedVerticalSeparationPattern(vspSize, matrix.getHeight() - vspSize, matrix)?;

    Ok(())
}

// Embed position adjustment patterns if need be.
pub fn maybeEmbedPositionAdjustmentPatterns(version: &Version, matrix: &mut ByteMatrix) {
    if version.getVersionNumber() < 2 {
        // The patterns appear if version >= 2
        return;
    }
    let index = version.getVersionNumber() - 1;
    let coordinates = POSITION_ADJUSTMENT_PATTERN_COORDINATE_TABLE[index as usize];
    for y in coordinates {
        if y >= 0 {
            for x in coordinates {
                if x >= 0 && isEmpty(matrix.get(x as u32, y as u32)) {
                    // If the cell is unset, we embed the position adjustment pattern here.
                    // -2 is necessary since the x/y coordinates point to the center of the pattern, not the
                    // left top corner.
                    embedPositionAdjustmentPattern((x - 2) as u32, (y - 2) as u32, matrix);
                }
            }
        }
    }
}
