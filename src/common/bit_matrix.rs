/*
 * Copyright 2007 ZXing authors
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

// package com.google.zxing.common;

// import java.util.Arrays;

use std::fmt;

use crate::Exceptions;

use super::BitArray;

/**
 * <p>Represents a 2D matrix of bits. In function arguments below, and throughout the common
 * module, x is the column position, and y is the row position. The ordering is always x, y.
 * The origin is at the top-left.</p>
 *
 * <p>Internally the bits are represented in a 1-D array of 32-bit ints. However, each row begins
 * with a new int. This is done intentionally so that we can copy out a row into a BitArray very
 * efficiently.</p>
 *
 * <p>The ordering of bits is row-major. Within each int, the least significant bits are used first,
 * meaning they represent lower x values. This is compatible with BitArray's implementation.</p>
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BitMatrix {
    width: u32,
    height: u32,
    row_size: usize,
    bits: Vec<u32>,
}

impl BitMatrix {
    /**
     * Creates an empty square {@code BitMatrix}.
     *
     * @param dimension height and width
     */
    pub fn with_single_dimension(dimension: u32) -> Self {
        Self::new(dimension, dimension).unwrap()
    }

    /**
     * Creates an empty {@code BitMatrix}.
     *
     * @param width bit matrix width
     * @param height bit matrix height
     */
    pub fn new(width: u32, height: u32) -> Result<Self, Exceptions> {
        if width < 1 || height < 1 {
            return Err(Exceptions::IllegalArgumentException(
                "Both dimensions must be greater than 0".to_owned(),
            ));
        }
        Ok(Self {
            width,
            height,
            row_size: ((width + 31) / 32) as usize,
            bits: vec![0; (((width + 31) / 32) * height) as usize],
        })
        // this.width = width;
        // this.height = height;
        // this.rowSize = (width + 31) / 32;
        // bits = new int[rowSize * height];
    }

    #[allow(dead_code)]
    fn with_all_data(&self, width: u32, height: u32, rowSize: usize, bits: Vec<u32>) -> Self {
        Self {
            width,
            height,
            row_size: rowSize,
            bits,
        }
    }

    /**
     * Interprets a 2D array of booleans as a {@code BitMatrix}, where "true" means an "on" bit.
     *
     * @param image bits of the image, as a row-major 2D array. Elements are arrays representing rows
     * @return {@code BitMatrix} representation of image
     */
    pub fn parse_bools(image: &Vec<Vec<bool>>) -> Self {
        let height: u32 = image.len().try_into().unwrap();
        let width: u32 = image[0].len().try_into().unwrap();
        let mut bits = BitMatrix::new(width, height).unwrap();
        for i in 0..height as usize {
            //for (int i = 0; i < height; i++) {
            let imageI = &image[i];
            for j in 0..width as usize {
                //for (int j = 0; j < width; j++) {
                if imageI[j] {
                    bits.set(j as u32, i as u32);
                }
            }
        }
        return bits;
    }

    pub fn parse_strings(
        string_representation: &str,
        set_string: &str,
        unset_string: &str,
    ) -> Result<Self, Exceptions> {
        // cannot pass nulls in rust
        // if (stringRepresentation == null) {
        //   throw new IllegalArgumentException();
        // }

        let mut bits = vec![false; string_representation.len()];
        let mut bitsPos = 0;
        let mut rowStartPos = 0;
        let mut rowLength = 0; //-1;
        let mut first_run = true;
        let mut nRows = 0;
        let mut pos = 0;
        while pos < string_representation.len() {
            if string_representation.chars().nth(pos).unwrap() == '\n'
                || string_representation.chars().nth(pos).unwrap() == '\r'
            {
                if bitsPos > rowStartPos {
                    //if rowLength == -1 {
                    if first_run {
                        first_run = false;
                        rowLength = bitsPos - rowStartPos;
                    } else if bitsPos - rowStartPos != rowLength {
                        return Err(Exceptions::IllegalArgumentException(
                            "row lengths do not match".to_owned(),
                        ));
                    }
                    rowStartPos = bitsPos;
                    nRows += 1;
                }
                pos += 1;
            } else if string_representation[pos..].starts_with(set_string) {
                pos += set_string.len();
                bits[bitsPos] = true;
                bitsPos += 1;
            } else if string_representation[pos..].starts_with(unset_string) {
                pos += unset_string.len();
                bits[bitsPos] = false;
                bitsPos += 1;
            } else {
                return Err(Exceptions::IllegalArgumentException(format!(
                    "illegal character encountered: {}",
                    string_representation[pos..].to_owned()
                )));
            }
        }

        // no EOL at end?
        if bitsPos > rowStartPos {
            //if rowLength == -1 {
            if first_run {
                // first_run = false;
                rowLength = bitsPos - rowStartPos;
            } else if bitsPos - rowStartPos != rowLength {
                return Err(Exceptions::IllegalArgumentException(
                    "row lengths do not match".to_owned(),
                ));
            }
            nRows += 1;
        }

        let mut matrix = BitMatrix::new(rowLength.try_into().unwrap(), nRows)?;
        for i in 0..bitsPos {
            //for (int i = 0; i < bitsPos; i++) {
            if bits[i] {
                matrix.set(
                    (i % rowLength).try_into().unwrap(),
                    (i / rowLength).try_into().unwrap(),
                );
            }
        }
        return Ok(matrix);
    }

    /**
     * <p>Gets the requested bit, where true means black.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     * @return value of given bit in matrix
     */
    pub fn get(&self, x: u32, y: u32) -> bool {
        let offset = y as usize * self.row_size + (x as usize / 32);
        return ((self.bits[offset] >> (x & 0x1f)) & 1) != 0;
    }

    pub fn try_get(&self, x: u32, y: u32) -> Result<bool, Exceptions> {
        let offset = y as usize * self.row_size + (x as usize / 32);
        if offset > self.bits.len() {
            return Err(Exceptions::IndexOutOfBoundsException("".to_owned()));
        }
        return Ok(((self.bits[offset] >> (x & 0x1f)) & 1) != 0);
    }

    pub fn check_in_bounds(&self, x: u32, y: u32) -> bool {
        (y as usize * self.row_size + (x as usize / 32)) > self.bits.len()
    }

    /**
     * <p>Sets the given bit to true.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     */
    pub fn set(&mut self, x: u32, y: u32) {
        let offset = y as usize * self.row_size + (x as usize / 32);
        self.bits[offset] |= 1 << (x & 0x1f);
    }

    pub fn unset(&mut self, x: u32, y: u32) {
        let offset = y as usize * self.row_size + (x as usize / 32);
        self.bits[offset] &= !(1 << (x & 0x1f));
    }

    /**
     * <p>Flips the given bit.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     */
    pub fn flip_coords(&mut self, x: u32, y: u32) {
        let offset = y as usize * self.row_size + (x as usize / 32);
        self.bits[offset] ^= 1 << (x & 0x1f);
    }

    /**
     * <p>Flips every bit in the matrix.</p>
     */
    pub fn flip_self(&mut self) {
        let max = self.bits.len();
        for i in 0..max {
            //for (int i = 0; i < max; i++) {
            self.bits[i] = !self.bits[i];
        }
    }

    /**
     * Exclusive-or (XOR): Flip the bit in this {@code BitMatrix} if the corresponding
     * mask bit is set.
     *
     * @param mask XOR mask
     */
    pub fn xor(&mut self, mask: &BitMatrix) -> Result<(), Exceptions> {
        if self.width != mask.width || self.height != mask.height || self.row_size != mask.row_size
        {
            return Err(Exceptions::IllegalArgumentException(
                "input matrix dimensions do not match".to_owned(),
            ));
        }
        // let mut rowArray = BitArray::with_size(self.width as usize);
        for y in 0..self.height {
            //for (int y = 0; y < height; y++) {
            let offset = y as usize * self.row_size;
            let rowArray = mask.getRow(y);
            let row = rowArray.getBitArray();
            for x in 0..self.row_size {
                //for (int x = 0; x < rowSize; x++) {
                self.bits[offset + x] ^= row[x];
            }
        }
        Ok(())
    }

    /**
     * Clears all bits (sets to false).
     */
    pub fn clear(&mut self) {
        // let max = self.bits.len();
        // for i in 0..max {
        //     //for (int i = 0; i < max; i++) {
        //     self.bits[i] = 0;
        // }
        self.bits.fill(0);
    }

    /**
     * <p>Sets a square region of the bit matrix to true.</p>
     *
     * @param left The horizontal position to begin at (inclusive)
     * @param top The vertical position to begin at (inclusive)
     * @param width The width of the region
     * @param height The height of the region
     */
    pub fn setRegion(
        &mut self,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) -> Result<(), Exceptions> {
        // if top < 0 || left < 0 {
        //     return Err(Exceptions::IllegalArgumentException(
        //         "Left and top must be nonnegative".to_owned(),
        //     ));
        // }
        if height < 1 || width < 1 {
            return Err(Exceptions::IllegalArgumentException(
                "Height and width must be at least 1".to_owned(),
            ));
        }
        let right = left + width;
        let bottom = top + height;
        if bottom > self.height || right > self.width {
            return Err(Exceptions::IllegalArgumentException(
                "The region must fit inside the matrix".to_owned(),
            ));
        }
        for y in top..bottom {
            //for (int y = top; y < bottom; y++) {
            let offset = y as usize * self.row_size;
            for x in left..right {
                //for (int x = left; x < right; x++) {
                self.bits[offset + (x as usize / 32)] |= 1 << (x & 0x1f);
            }
        }
        Ok(())
    }

    /**
     * A fast method to retrieve one row of data from the matrix as a BitArray.
     *
     * @param y The row to retrieve
     * @param row An optional caller-allocated BitArray, will be allocated if null or too small
     * @return The resulting BitArray - this reference should always be used even when passing
     *         your own row
     */
    pub fn getRow(&self, y: u32) -> BitArray {
        // let mut rw: BitArray = if row.getSize() < self.width as usize {
        //     BitArray::with_size(self.width as usize)
        // } else {
        //     let mut z = row; //row.clone();
        //     z.clear();
        //     z
        //     // row.clear();
        //     // row.clone()
        // };
        let mut rw = BitArray::with_size(self.width as usize);

        let offset = y as usize * self.row_size;
        for x in 0..self.row_size {
            //for (int x = 0; x < rowSize; x++) {
            rw.setBulk(x * 32, self.bits[offset + x]);
        }
        return rw;
    }

    /**
     * @param y row to set
     * @param row {@link BitArray} to copy from
     */
    pub fn setRow(&mut self, y: u32, row: &BitArray) {
        return self.bits[y as usize * self.row_size..y as usize * self.row_size + self.row_size]
            .clone_from_slice(&row.getBitArray()[0..self.row_size]);
        //System.arraycopy(row.getBitArray(), 0, self.bits, y * self.rowSize, self.rowSize);
    }

    /**
     * Modifies this {@code BitMatrix} to represent the same but rotated the given degrees (0, 90, 180, 270)
     *
     * @param degrees number of degrees to rotate through counter-clockwise (0, 90, 180, 270)
     */
    pub fn rotate(&mut self, degrees: u32) -> Result<(), Exceptions> {
        match degrees % 360 {
            0 => Ok(()),
            90 => {
                self.rotate90();
                Ok(())
            }
            180 => {
                self.rotate180();
                Ok(())
            }
            270 => {
                self.rotate90();
                self.rotate180();
                Ok(())
            }
            _ => Err(Exceptions::IllegalArgumentException(
                "degrees must be a multiple of 0, 90, 180, or 270".to_owned(),
            )),
        }
    }

    /**
     * Modifies this {@code BitMatrix} to represent the same but rotated 180 degrees
     */
    pub fn rotate180(&mut self) {
        // let mut topRow = BitArray::with_size(self.width as usize);
        // let mut bottomRow = BitArray::with_size(self.width as usize);
        let maxHeight = (self.height + 1) / 2;
        for i in 0..maxHeight {
            //for (int i = 0; i < maxHeight; i++) {
            let mut topRow = self.getRow(i);
            let bottomRowIndex = self.height - 1 - i;
            let mut bottomRow = self.getRow(bottomRowIndex);
            topRow.reverse();
            bottomRow.reverse();
            self.setRow(i, &bottomRow);
            self.setRow(bottomRowIndex, &topRow);
        }
    }

    /**
     * Modifies this {@code BitMatrix} to represent the same but rotated 90 degrees counterclockwise
     */
    pub fn rotate90(&mut self) {
        let newWidth = self.height;
        let newHeight = self.width;
        let newRowSize = (newWidth + 31) / 32;
        let mut newBits = vec![0; (newRowSize * newHeight) as usize];

        for y in 0..self.height {
            //for (int y = 0; y < height; y++) {
            for x in 0..self.width {
                //for (int x = 0; x < width; x++) {
                let offset = y as usize * self.row_size + (x as usize / 32);
                if ((self.bits[offset] >> (x & 0x1f)) & 1) != 0 {
                    let newOffset: usize = ((newHeight - 1 - x) * newRowSize + (y / 32)) as usize;
                    newBits[newOffset] |= 1 << (y & 0x1f);
                }
            }
        }
        self.width = newWidth;
        self.height = newHeight;
        self.row_size = newRowSize as usize;
        self.bits = newBits;
    }

    /**
     * This is useful in detecting the enclosing rectangle of a 'pure' barcode.
     *
     * @return {@code left,top,width,height} enclosing rectangle of all 1 bits, or null if it is all white
     */
    pub fn getEnclosingRectangle(&self) -> Option<Vec<u32>> {
        let mut left = self.width;
        let mut top = self.height;
        // let right = -1;
        // let bottom = -1;
        let mut right: u32 = 0;
        let mut bottom = 0;

        for y in 0..self.height {
            //for (int y = 0; y < height; y++) {
            for x32 in 0..self.row_size {
                //for (int x32 = 0; x32 < rowSize; x32++) {
                let theBits = self.bits[y as usize * self.row_size + x32];
                if theBits != 0 {
                    if y < top {
                        top = y;
                    }
                    if y > bottom {
                        bottom = y;
                    }
                    if x32 * 32 < left.try_into().unwrap() {
                        let mut bit = 0;
                        while (theBits << (31 - bit)) == 0 {
                            bit += 1;
                        }
                        if (x32 * 32 + bit) < left.try_into().unwrap() {
                            left = (x32 * 32 + bit).try_into().unwrap();
                        }
                    }
                    if x32 * 32 + 31 > right.try_into().unwrap() {
                        let mut bit = 31;
                        while (theBits >> bit) == 0 {
                            bit -= 1;
                        }
                        if (x32 * 32 + bit) > right.try_into().unwrap() {
                            right = (x32 * 32 + bit).try_into().unwrap();
                        }
                    }
                }
            }
        }

        if right < left || bottom < top {
            return None;
        }

        return Some(vec![left, top, right - left + 1, bottom - top + 1]);
    }

    /**
     * This is useful in detecting a corner of a 'pure' barcode.
     *
     * @return {@code x,y} coordinate of top-left-most 1 bit, or null if it is all white
     */
    pub fn getTopLeftOnBit(&self) -> Option<Vec<u32>> {
        let mut bitsOffset = 0;
        while bitsOffset < self.bits.len() && self.bits[bitsOffset] == 0 {
            bitsOffset += 1;
        }
        if bitsOffset == self.bits.len() {
            return None;
        }
        let y = bitsOffset / self.row_size;
        let mut x = (bitsOffset % self.row_size) * 32;

        let theBits = self.bits[bitsOffset];
        let mut bit = 0;
        while (theBits << (31 - bit)) == 0 {
            bit += 1;
        }
        x += bit;
        return Some(vec![x as u32, y as u32]);
    }

    pub fn getBottomRightOnBit(&self) -> Option<Vec<u32>> {
        let mut bitsOffset = self.bits.len() as i64 - 1;
        while bitsOffset >= 0 && self.bits[bitsOffset as usize] == 0 {
            bitsOffset -= 1;
        }
        if bitsOffset < 0 {
            return None;
        }

        let y = bitsOffset as usize / self.row_size;
        let mut x = (bitsOffset as usize % self.row_size) * 32;

        let theBits = self.bits[bitsOffset as usize];
        let mut bit = 31;
        while (theBits >> bit) == 0 {
            bit -= 1;
        }
        x += bit;

        return Some(vec![x as u32, y as u32]);
    }

    /**
     * @return The width of the matrix
     */
    pub fn getWidth(&self) -> u32 {
        return self.width;
    }

    /**
     * @return The height of the matrix
     */
    pub fn getHeight(&self) -> u32 {
        return self.height;
    }

    /**
     * @return The row size of the matrix
     */
    pub fn getRowSize(&self) -> usize {
        return self.row_size;
    }

    // @Override
    // public boolean equals(Object o) {
    //   if (!(o instanceof BitMatrix)) {
    //     return false;
    //   }
    //   BitMatrix other = (BitMatrix) o;
    //   return width == other.width && height == other.height && rowSize == other.rowSize &&
    //   Arrays.equals(bits, other.bits);
    // }

    // @Override
    // public int hashCode() {
    //   int hash = width;
    //   hash = 31 * hash + width;
    //   hash = 31 * hash + height;
    //   hash = 31 * hash + rowSize;
    //   hash = 31 * hash + Arrays.hashCode(bits);
    //   return hash;
    // }

    /**
     * @param setString representation of a set bit
     * @param unsetString representation of an unset bit
     * @return string representation of entire matrix utilizing given strings
     */
    pub fn toString(&self, setString: &str, unsetString: &str) -> String {
        return self.buildToString(setString, unsetString, "\n");
    }

    /**
     * @param setString representation of a set bit
     * @param unsetString representation of an unset bit
     * @param lineSeparator newline character in string representation
     * @return string representation of entire matrix utilizing given strings and line separator
     * @deprecated call {@link #toString(String,String)} only, which uses \n line separator always
     */
    // @Deprecated
    // public String toString(String setString, String unsetString, String lineSeparator) {
    //   return buildToString(setString, unsetString, lineSeparator);
    // }

    fn buildToString(&self, setString: &str, unsetString: &str, lineSeparator: &str) -> String {
        let mut result =
            String::with_capacity((self.height * (self.width + 1)).try_into().unwrap());
        for y in 0..self.height {
            //for (int y = 0; y < height; y++) {
            for x in 0..self.width {
                //for (int x = 0; x < width; x++) {
                result.push_str(if self.get(x, y) {
                    setString
                } else {
                    unsetString
                });
            }
            result.push_str(lineSeparator);
        }
        return result;
    }

    // @Override
    // public BitMatrix clone() {
    //   return new BitMatrix(width, height, rowSize, bits.clone());
    // }
    // pub fn crop(&self, top:usize, left:usize, height: usize, width: usize) -> BitMatrix {
    //     let area = self.bits.iter().skip(self.row_size * top).take(self.row_size * height)
    //     .copied().collect::<Vec<u32>>();
    //     let new_bits = area.chunks(self.row_size)
    //     .skip(left).take(width).flatten().copied().collect::<Vec<u32>>();
    //     Self { width: width, height: height, row_size: width, bits: () }
    // }
    pub fn crop(&self, top: usize, left: usize, height: usize, width: usize) -> BitMatrix {
        let mut new_bm = BitMatrix::new(width as u32, height as u32).expect("create empty");
        for y in top..top + height {
            // let row = self.getRow(y as u32);
            for x in left..left + width {
                if self.get(x as u32, y as u32) {
                    new_bm.set(x as u32, y as u32)
                }
            }
        }
        new_bm
    }
}

impl fmt::Display for BitMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.toString("X ", "  "))
    }
}
