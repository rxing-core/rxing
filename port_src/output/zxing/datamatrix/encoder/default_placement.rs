/*
 * Copyright 2006 Jeremias Maerki.
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
// package com::google::zxing::datamatrix::encoder;

/**
 * Symbol Character Placement Program. Adapted from Annex M.1 in ISO/IEC 16022:2000(E).
 */
pub struct DefaultPlacement {

     let codewords: CharSequence;

     let numrows: i32;

     let mut numcols: i32;

     let mut bits: Vec<i8>;
}

impl DefaultPlacement {

    /**
   * Main constructor
   *
   * @param codewords the codewords to place
   * @param numcols   the number of columns
   * @param numrows   the number of rows
   */
    pub fn new( codewords: &CharSequence,  numcols: i32,  numrows: i32) -> DefaultPlacement {
        let .codewords = codewords;
        let .numcols = numcols;
        let .numrows = numrows;
        let .bits = : [i8; numcols * numrows] = [0; numcols * numrows];
        //Initialize with "not set" value
        Arrays::fill(let .bits, -1 as i8);
    }

    fn  get_numrows(&self) -> i32  {
        return self.numrows;
    }

    fn  get_numcols(&self) -> i32  {
        return self.numcols;
    }

    fn  get_bits(&self) -> Vec<i8>  {
        return self.bits;
    }

    pub fn  get_bit(&self,  col: i32,  row: i32) -> bool  {
        return self.bits[row * self.numcols + col] == 1;
    }

    fn  set_bit(&self,  col: i32,  row: i32,  bit: bool)   {
        self.bits[row * self.numcols + col] = ( if bit { 1 } else { 0 }) as i8;
    }

    fn  no_bit(&self,  col: i32,  row: i32) -> bool  {
        return self.bits[row * self.numcols + col] < 0;
    }

    pub fn  place(&self)   {
         let mut pos: i32 = 0;
         let mut row: i32 = 4;
         let mut col: i32 = 0;
        loop { {
            // repeatedly first check for one of the special corner cases, then...
            if (row == self.numrows) && (col == 0) {
                self.corner1(pos += 1 !!!check!!! post increment);
            }
            if (row == self.numrows - 2) && (col == 0) && ((self.numcols % 4) != 0) {
                self.corner2(pos += 1 !!!check!!! post increment);
            }
            if (row == self.numrows - 2) && (col == 0) && (self.numcols % 8 == 4) {
                self.corner3(pos += 1 !!!check!!! post increment);
            }
            if (row == self.numrows + 4) && (col == 2) && ((self.numcols % 8) == 0) {
                self.corner4(pos += 1 !!!check!!! post increment);
            }
            // sweep upward diagonally, inserting successive characters...
            loop { {
                if (row < self.numrows) && (col >= 0) && self.no_bit(col, row) {
                    self.utah(row, col, pos += 1 !!!check!!! post increment);
                }
                row -= 2;
                col += 2;
            }if !(row >= 0 && (col < self.numcols)) break;}
            row += 1;
            col += 3;
            // and then sweep downward diagonally, inserting successive characters, ...
            loop { {
                if (row >= 0) && (col < self.numcols) && self.no_bit(col, row) {
                    self.utah(row, col, pos += 1 !!!check!!! post increment);
                }
                row += 2;
                col -= 2;
            }if !((row < self.numrows) && (col >= 0)) break;}
            row += 3;
            col += 1;
        // ...until the entire array is scanned
        }if !((row < self.numrows) || (col < self.numcols)) break;}
        // Lastly, if the lower right-hand corner is untouched, fill in fixed pattern
        if self.no_bit(self.numcols - 1, self.numrows - 1) {
            self.set_bit(self.numcols - 1, self.numrows - 1, true);
            self.set_bit(self.numcols - 2, self.numrows - 2, true);
        }
    }

    fn  module(&self,  row: i32,  col: i32,  pos: i32,  bit: i32)   {
        if row < 0 {
            row += self.numrows;
            col += 4 - ((self.numrows + 4) % 8);
        }
        if col < 0 {
            col += self.numcols;
            row += 4 - ((self.numcols + 4) % 8);
        }
        // Note the conversion:
         let mut v: i32 = self.codewords.char_at(pos);
        v &= 1 << (8 - bit);
        self.set_bit(col, row, v != 0);
    }

    /**
   * Places the 8 bits of a utah-shaped symbol character in ECC200.
   *
   * @param row the row
   * @param col the column
   * @param pos character position
   */
    fn  utah(&self,  row: i32,  col: i32,  pos: i32)   {
        self.module(row - 2, col - 2, pos, 1);
        self.module(row - 2, col - 1, pos, 2);
        self.module(row - 1, col - 2, pos, 3);
        self.module(row - 1, col - 1, pos, 4);
        self.module(row - 1, col, pos, 5);
        self.module(row, col - 2, pos, 6);
        self.module(row, col - 1, pos, 7);
        self.module(row, col, pos, 8);
    }

    fn  corner1(&self,  pos: i32)   {
        self.module(self.numrows - 1, 0, pos, 1);
        self.module(self.numrows - 1, 1, pos, 2);
        self.module(self.numrows - 1, 2, pos, 3);
        self.module(0, self.numcols - 2, pos, 4);
        self.module(0, self.numcols - 1, pos, 5);
        self.module(1, self.numcols - 1, pos, 6);
        self.module(2, self.numcols - 1, pos, 7);
        self.module(3, self.numcols - 1, pos, 8);
    }

    fn  corner2(&self,  pos: i32)   {
        self.module(self.numrows - 3, 0, pos, 1);
        self.module(self.numrows - 2, 0, pos, 2);
        self.module(self.numrows - 1, 0, pos, 3);
        self.module(0, self.numcols - 4, pos, 4);
        self.module(0, self.numcols - 3, pos, 5);
        self.module(0, self.numcols - 2, pos, 6);
        self.module(0, self.numcols - 1, pos, 7);
        self.module(1, self.numcols - 1, pos, 8);
    }

    fn  corner3(&self,  pos: i32)   {
        self.module(self.numrows - 3, 0, pos, 1);
        self.module(self.numrows - 2, 0, pos, 2);
        self.module(self.numrows - 1, 0, pos, 3);
        self.module(0, self.numcols - 2, pos, 4);
        self.module(0, self.numcols - 1, pos, 5);
        self.module(1, self.numcols - 1, pos, 6);
        self.module(2, self.numcols - 1, pos, 7);
        self.module(3, self.numcols - 1, pos, 8);
    }

    fn  corner4(&self,  pos: i32)   {
        self.module(self.numrows - 1, 0, pos, 1);
        self.module(self.numrows - 1, self.numcols - 1, pos, 2);
        self.module(0, self.numcols - 3, pos, 3);
        self.module(0, self.numcols - 2, pos, 4);
        self.module(0, self.numcols - 1, pos, 5);
        self.module(1, self.numcols - 3, pos, 6);
        self.module(1, self.numcols - 2, pos, 7);
        self.module(1, self.numcols - 1, pos, 8);
    }
}

