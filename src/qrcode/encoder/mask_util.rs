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

use crate::common::Result;
use crate::Exceptions;

use super::ByteMatrix;

/**
 * @author Satoru Takabayashi
 * @author Daniel Switkin
 * @author Sean Owen
 */

// Penalty weights from section 6.8.2.1
const N1: u32 = 3;
const N2: u32 = 3;
const N3: u32 = 40;
const N4: u32 = 10;

/**
 * Apply mask penalty rule 1 and return the penalty. Find repetitive cells with the same color and
 * give penalty to them. Example: 00000 or 11111.
 */
pub fn applyMaskPenaltyRule1(matrix: &ByteMatrix) -> u32 {
    applyMaskPenaltyRule1Internal(matrix, true) + applyMaskPenaltyRule1Internal(matrix, false)
}

/**
 * Apply mask penalty rule 2 and return the penalty. Find 2x2 blocks with the same color and give
 * penalty to them. This is actually equivalent to the spec's rule, which is to find MxN blocks and give a
 * penalty proportional to (M-1)x(N-1), because this is the number of 2x2 blocks inside such a block.
 */
pub fn applyMaskPenaltyRule2(matrix: &ByteMatrix) -> u32 {
    let mut penalty = 0;
    let array = matrix.getArray();
    let width = matrix.getWidth();
    let height = matrix.getHeight();
    for y in 0..(height - 1) as usize {
        let arrayY = &array[y];
        for x in 0..(width - 1) as usize {
            let value = arrayY[x];
            if value == arrayY[x + 1] && value == array[y + 1][x] && value == array[y + 1][x + 1] {
                penalty += 1;
            }
        }
    }
    N2 * penalty
}

/**
 * Apply mask penalty rule 3 and return the penalty. Find consecutive runs of 1:1:3:1:1:4
 * starting with black, or 4:1:1:3:1:1 starting with white, and give penalty to them.  If we
 * find patterns like 000010111010000, we give penalty once.
 */
pub fn applyMaskPenaltyRule3(matrix: &ByteMatrix) -> u32 {
    let mut numPenalties = 0;
    let array = matrix.getArray();
    let width = matrix.getWidth();
    let height = matrix.getHeight();
    for y in 0..height as usize {
        for x in 0..width as usize {
            let arrayY = &array[y]; // We can at least optimize this access
            if x + 6 < width as usize
                && arrayY[x] == 1
                && arrayY[x + 1] == 0
                && arrayY[x + 2] == 1
                && arrayY[x + 3] == 1
                && arrayY[x + 4] == 1
                && arrayY[x + 5] == 0
                && arrayY[x + 6] == 1
                && (isWhiteHorizontal(arrayY, x as i32 - 4, x as u32)
                    || isWhiteHorizontal(arrayY, x as i32 + 7, x as u32 + 11))
            {
                numPenalties += 1;
            }
            if y + 6 < height as usize
                && array[y][x] == 1
                && array[y + 1][x] == 0
                && array[y + 2][x] == 1
                && array[y + 3][x] == 1
                && array[y + 4][x] == 1
                && array[y + 5][x] == 0
                && array[y + 6][x] == 1
                && (isWhiteVertical(array, x as u32, y as i32 - 4, y as u32)
                    || isWhiteVertical(array, x as u32, y as i32 + 7, y as u32 + 11))
            {
                numPenalties += 1;
            }
        }
    }
    numPenalties * N3
}

pub fn isWhiteHorizontal(rowArray: &[u8], from: i32, to: u32) -> bool {
    if from < 0 || rowArray.len() < to as usize {
        return false;
    }
    for i in from..to as i32 {
        if rowArray[i as usize] == 1 {
            return false;
        }
    }
    true
}

pub fn isWhiteVertical(array: &Vec<Vec<u8>>, col: u32, from: i32, to: u32) -> bool {
    if from < 0 || array.len() < to as usize {
        return false;
    }
    for i in from..to as i32 {
        if array[i as usize][col as usize] == 1 {
            return false;
        }
    }
    true
}

/**
 * Apply mask penalty rule 4 and return the penalty. Calculate the ratio of dark cells and give
 * penalty if the ratio is far from 50%. It gives 10 penalty for 5% distance.
 */
pub fn applyMaskPenaltyRule4(matrix: &ByteMatrix) -> u32 {
    let mut numDarkCells = 0;
    let array = matrix.getArray();
    let width = matrix.getWidth();
    let height = matrix.getHeight();
    for val_y in array.iter().take(height as usize) {
        for val_x in val_y.iter().take(width as usize) {
            if val_x == &1 {
                numDarkCells += 1;
            }
        }
    }
    let numTotalCells = matrix.getHeight() * matrix.getWidth();
    let fivePercentVariances =
        (numDarkCells as i64 * 2 - numTotalCells as i64).unsigned_abs() as u32 * 10 / numTotalCells;
    fivePercentVariances * N4
}

/**
 * Return the mask bit for "getMaskPattern" at "x" and "y". See 8.8 of JISX0510:2004 for mask
 * pattern conditions.
 */
pub fn getDataMaskBit(maskPattern: u32, x: u32, y: u32) -> Result<bool> {
    let intermediate = match maskPattern {
        0 => (y + x) & 0x1,
        1 => y & 0x1,
        2 => x % 3,
        3 => (y + x) % 3,
        4 => ((y / 2) + (x / 3)) & 0x1,
        5 => {
            let temp = y * x;
            (temp & 0x1) + (temp % 3)
        }
        6 => {
            let temp = y * x;
            ((temp & 0x1) + (temp % 3)) & 0x1
        }
        7 => {
            let temp = y * x;
            ((temp % 3) + ((y + x) & 0x1)) & 0x1
        }
        _ => {
            return Err(Exceptions::IllegalArgumentException(Some(format!(
                "Invalid mask pattern: {maskPattern}"
            ))))
        }
    };
    // switch (maskPattern) {
    //   case 0:
    //     intermediate = (y + x) & 0x1;
    //     break;
    //   case 1:
    //     intermediate = y & 0x1;
    //     break;
    //   case 2:
    //     intermediate = x % 3;
    //     break;
    //   case 3:
    //     intermediate = (y + x) % 3;
    //     break;
    //   case 4:
    //     intermediate = ((y / 2) + (x / 3)) & 0x1;
    //     break;
    //   case 5:
    //     temp = y * x;
    //     intermediate = (temp & 0x1) + (temp % 3);
    //     break;
    //   case 6:
    //     temp = y * x;
    //     intermediate = ((temp & 0x1) + (temp % 3)) & 0x1;
    //     break;
    //   case 7:
    //     temp = y * x;
    //     intermediate = ((temp % 3) + ((y + x) & 0x1)) & 0x1;
    //     break;
    //   default:
    //     throw new IllegalArgumentException("Invalid mask pattern: " + maskPattern);
    // }
    Ok(intermediate == 0)
}

/**
 * Helper function for applyMaskPenaltyRule1. We need this for doing this calculation in both
 * vertical and horizontal orders respectively.
 */
fn applyMaskPenaltyRule1Internal(matrix: &ByteMatrix, isHorizontal: bool) -> u32 {
    let mut penalty = 0;
    let iLimit = if isHorizontal {
        matrix.getHeight()
    } else {
        matrix.getWidth()
    };
    let jLimit = if isHorizontal {
        matrix.getWidth()
    } else {
        matrix.getHeight()
    };
    let array = matrix.getArray();
    for i in 0..iLimit as usize {
        let mut numSameBitCells = 0;
        let mut prevBit = 0;
        for j in 0..jLimit as usize {
            let bit = if isHorizontal {
                array[i][j]
            } else {
                array[j][i]
            };
            if bit == prevBit {
                numSameBitCells += 1;
            } else {
                if numSameBitCells >= 5 {
                    penalty += N1 + (numSameBitCells - 5);
                }
                numSameBitCells = 1; // Include the cell itself.
                prevBit = bit;
            }
        }
        if numSameBitCells >= 5 {
            penalty += N1 + (numSameBitCells - 5);
        }
    }
    penalty
}
