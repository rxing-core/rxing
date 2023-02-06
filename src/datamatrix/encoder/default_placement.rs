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

use crate::Exceptions;

const EMPTY_BIT_VAL: u8 = 13;

/**
 * Symbol Character Placement Program. Adapted from Annex M.1 in ISO/IEC 16022:2000(E).
 */
pub struct DefaultPlacement {
    codewords: String,
    numrows: usize,
    numcols: usize,
    bits: Vec<u8>,
}
impl DefaultPlacement {
    /**
     * Main constructor
     *
     * @param codewords the codewords to place
     * @param numcols   the number of columns
     * @param numrows   the number of rows
     */
    pub fn new(codewords: String, numcols: usize, numrows: usize) -> Self {
        // this.codewords = codewords;
        // this.numcols = numcols;
        // this.numrows = numrows;
        // this.bits = new byte[numcols * numrows];
        // Arrays.fill(this.bits, (byte) -1); //Initialize with "not set" value
        Self {
            codewords,
            numrows,
            numcols,
            bits: vec![EMPTY_BIT_VAL; numcols * numrows],
        }
    }

    pub fn getNumrows(&self) -> usize {
        self.numrows
    }

    pub fn getNumcols(&self) -> usize {
        self.numcols
    }

    pub fn getBits(&self) -> &[u8] {
        &self.bits
    }

    pub fn getBit(&self, col: usize, row: usize) -> bool {
        self.bits[row * self.numcols + col] == 1
    }

    pub fn setBit(&mut self, col: usize, row: usize, bit: bool) {
        self.bits[row * self.numcols + col] = u8::from(bit); //if bit { 1 } else { 0 };
    }

    pub fn noBit(&self, col: usize, row: usize) -> bool {
        self.bits[row * self.numcols + col] == EMPTY_BIT_VAL
    }

    pub fn place(&mut self) -> Result<(), Exceptions> {
        let mut pos = 0;
        let mut row = 4_isize;
        let mut col = 0_isize;

        loop {
            // repeatedly first check for one of the special corner cases, then...
            if (row == self.numrows as isize) && (col == 0) {
                self.corner1(pos)?;
                pos += 1;
            }
            if (row == self.numrows as isize - 2) && (col == 0) && ((self.numcols % 4) != 0) {
                self.corner2(pos)?;
                pos += 1;
            }
            if (row == self.numrows as isize - 2) && (col == 0) && (self.numcols % 8 == 4) {
                self.corner3(pos)?;
                pos += 1;
            }
            if (row == self.numrows as isize + 4) && (col == 2) && ((self.numcols % 8) == 0) {
                self.corner4(pos)?;
                pos += 1;
            }
            // sweep upward diagonally, inserting successive characters...
            loop {
                if (row < self.numrows as isize)
                    && (col >= 0)
                    && self.noBit(col as usize, row as usize)
                {
                    self.utah(row, col, pos)?;
                    pos += 1;
                }
                row -= 2;
                col += 2;
                if !(row >= 0 && (col < self.numcols as isize)) {
                    break;
                }
            } //while (row >= 0 && (col < numcols));
            row += 1;
            col += 3;

            // and then sweep downward diagonally, inserting successive characters, ...
            loop {
                if (row >= 0)
                    && (col < self.numcols as isize)
                    && self.noBit(col as usize, row as usize)
                {
                    self.utah(row, col, pos)?;
                    pos += 1;
                }
                row += 2;
                col -= 2;

                if !((row < self.numrows as isize) && (col >= 0)) {
                    break;
                }
            } //while ((row < numrows) && (col >= 0));
            row += 3;
            col += 1;

            // ...until the entire array is scanned
            if !((row < self.numrows as isize) || (col < self.numcols as isize)) {
                break;
            }
        } // while ((row < numrows) || (col < numcols));

        // Lastly, if the lower right-hand corner is untouched, fill in fixed pattern
        if self.noBit(self.numcols - 1, self.numrows - 1) {
            self.setBit(self.numcols - 1, self.numrows - 1, true);
            self.setBit(self.numcols - 2, self.numrows - 2, true);
        }
        Ok(())
    }

    fn module(&mut self, row: isize, col: isize, pos: usize, bit: u32) -> Result<(), Exceptions> {
        let mut row = row;
        let mut col = col;

        if row < 0 {
            row += self.numrows as isize;
            col += 4 - ((self.numrows + 4) % 8) as isize;
        }
        if col < 0 {
            col += self.numcols as isize;
            row += 4 - ((self.numcols + 4) % 8) as isize;
        }
        // Note the conversion:
        let mut v = self
            .codewords
            .chars()
            .nth(pos)
            .ok_or(Exceptions::IndexOutOfBoundsException(None))? as u32;
        v &= 1 << (8 - bit);
        self.setBit(col as usize, row as usize, v != 0);

        Ok(())
    }

    /**
     * Places the 8 bits of a utah-shaped symbol character in ECC200.
     *
     * @param row the row
     * @param col the column
     * @param pos character position
     */
    fn utah(&mut self, row: isize, col: isize, pos: usize) -> Result<(), Exceptions> {
        self.module(row - 2, col - 2, pos, 1)?;
        self.module(row - 2, col - 1, pos, 2)?;
        self.module(row - 1, col - 2, pos, 3)?;
        self.module(row - 1, col - 1, pos, 4)?;
        self.module(row - 1, col, pos, 5)?;
        self.module(row, col - 2, pos, 6)?;
        self.module(row, col - 1, pos, 7)?;
        self.module(row, col, pos, 8)?;
        Ok(())
    }

    fn corner1(&mut self, pos: usize) -> Result<(), Exceptions> {
        self.module(self.numrows as isize - 1, 0, pos, 1)?;
        self.module(self.numrows as isize - 1, 1, pos, 2)?;
        self.module(self.numrows as isize - 1, 2, pos, 3)?;
        self.module(0, self.numcols as isize - 2, pos, 4)?;
        self.module(0, self.numcols as isize - 1, pos, 5)?;
        self.module(1, self.numcols as isize - 1, pos, 6)?;
        self.module(2, self.numcols as isize - 1, pos, 7)?;
        self.module(3, self.numcols as isize - 1, pos, 8)?;
        Ok(())
    }

    fn corner2(&mut self, pos: usize) -> Result<(), Exceptions> {
        self.module(self.numrows as isize - 3, 0, pos, 1)?;
        self.module(self.numrows as isize - 2, 0, pos, 2)?;
        self.module(self.numrows as isize - 1, 0, pos, 3)?;
        self.module(0, self.numcols as isize - 4, pos, 4)?;
        self.module(0, self.numcols as isize - 3, pos, 5)?;
        self.module(0, self.numcols as isize - 2, pos, 6)?;
        self.module(0, self.numcols as isize - 1, pos, 7)?;
        self.module(1, self.numcols as isize - 1, pos, 8)?;
        Ok(())
    }

    fn corner3(&mut self, pos: usize) -> Result<(), Exceptions> {
        self.module(self.numrows as isize - 3, 0, pos, 1)?;
        self.module(self.numrows as isize - 2, 0, pos, 2)?;
        self.module(self.numrows as isize - 1, 0, pos, 3)?;
        self.module(0, self.numcols as isize - 2, pos, 4)?;
        self.module(0, self.numcols as isize - 1, pos, 5)?;
        self.module(1, self.numcols as isize - 1, pos, 6)?;
        self.module(2, self.numcols as isize - 1, pos, 7)?;
        self.module(3, self.numcols as isize - 1, pos, 8)?;
        Ok(())
    }

    fn corner4(&mut self, pos: usize) -> Result<(), Exceptions> {
        self.module(self.numrows as isize - 1, 0, pos, 1)?;
        self.module(self.numrows as isize - 1, self.numcols as isize - 1, pos, 2)?;
        self.module(0, self.numcols as isize - 3, pos, 3)?;
        self.module(0, self.numcols as isize - 2, pos, 4)?;
        self.module(0, self.numcols as isize - 1, pos, 5)?;
        self.module(1, self.numcols as isize - 3, pos, 6)?;
        self.module(1, self.numcols as isize - 2, pos, 7)?;
        self.module(1, self.numcols as isize - 1, pos, 8)?;
        Ok(())
    }

    #[allow(dead_code)]
    fn toBitFieldStringArray(&self) -> Vec<String> {
        let bits = self.getBits();
        let numrows = self.getNumrows();
        let numcols = self.getNumcols();
        let mut array = Vec::with_capacity(numrows); //;[numrows];
        let mut startpos = 0;
        for _row in 0..numrows {
            // for (int row = 0; row < numrows; row++) {
            let mut sb = String::with_capacity(bits.len());
            for i in 0..numcols {
                // for (int i = 0; i < numcols; i++) {
                sb.push(if bits[startpos + i] == 1 { '1' } else { '0' });
            }
            //array[row] = sb.toString();
            array.push(sb);
            startpos += numcols;
        }

        array
    }
}

#[cfg(test)]
mod test_placement {
    //private static final Pattern SPACE = Pattern.compile(" ");

    use super::DefaultPlacement;

    #[test]
    fn testPlacement() {
        let codewords = unvisualize("66 74 78 66 74 78 129 56 35 102 192 96 226 100 156 1 107 221"); //"AIMAIM" encoded
        let mut placement = DefaultPlacement::new(codewords, 12, 12);
        placement.place().expect("msg");
        let expected = [
            "011100001111",
            "001010101000",
            "010001010100",
            "001010100010",
            "000111000100",
            "011000010100",
            "000100001101",
            "011000010000",
            "001100001101",
            "100010010111",
            "011101011010",
            "001011001010",
        ];
        let actual = placement.toBitFieldStringArray();
        for i in 0..actual.len() {
            // for (int i = 0; i < actual.length; i++) {
            assert_eq!(expected[i], actual[i], "Row {i}");
        }
    }

    fn unvisualize(visualized: &str) -> String {
        let mut sb = String::new();
        for token in visualized.split(' ') {
            // for (String token : SPACE.split(visualized)) {
            let tkn: u32 = token.parse().unwrap();
            sb.push(char::from_u32(tkn).unwrap());
            // sb.push((char) Integer.parseInt(token));
        }

        sb
    }
}
