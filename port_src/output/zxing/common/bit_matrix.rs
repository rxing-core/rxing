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
// package com::google::zxing::common;

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
#[derive(Cloneable)]
pub struct BitMatrix {

     let mut width: i32;

     let mut height: i32;

     let row_size: i32;

     let mut bits: Vec<i32>;
}

impl BitMatrix {

    /**
   * Creates an empty square {@code BitMatrix}.
   *
   * @param dimension height and width
   */
    pub fn new( dimension: i32) -> BitMatrix {
        this(dimension, dimension);
    }

    /**
   * Creates an empty {@code BitMatrix}.
   *
   * @param width bit matrix width
   * @param height bit matrix height
   */
    pub fn new( width: i32,  height: i32) -> BitMatrix {
        if width < 1 || height < 1 {
            throw IllegalArgumentException::new("Both dimensions must be greater than 0");
        }
        let .width = width;
        let .height = height;
        let .rowSize = (width + 31) / 32;
        bits = : [i32; row_size * height] = [0; row_size * height];
    }

    fn new( width: i32,  height: i32,  row_size: i32,  bits: &Vec<i32>) -> BitMatrix {
        let .width = width;
        let .height = height;
        let .rowSize = row_size;
        let .bits = bits;
    }

    /**
   * Interprets a 2D array of booleans as a {@code BitMatrix}, where "true" means an "on" bit.
   *
   * @param image bits of the image, as a row-major 2D array. Elements are arrays representing rows
   * @return {@code BitMatrix} representation of image
   */
    pub fn  parse( image: &Vec<Vec<bool>>) -> BitMatrix  {
         let height: i32 = image.len();
         let width: i32 = image[0].len();
         let bits: BitMatrix = BitMatrix::new(width, height);
         {
             let mut i: i32 = 0;
            while i < height {
                {
                     let image_i: Vec<bool> = image[i];
                     {
                         let mut j: i32 = 0;
                        while j < width {
                            {
                                if image_i[j] {
                                    bits.set(j, i);
                                }
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        return bits;
    }

    pub fn  parse( string_representation: &String,  set_string: &String,  unset_string: &String) -> BitMatrix  {
        if string_representation == null {
            throw IllegalArgumentException::new();
        }
         let mut bits: [bool; string_representation.length()] = [false; string_representation.length()];
         let bits_pos: i32 = 0;
         let row_start_pos: i32 = 0;
         let row_length: i32 = -1;
         let n_rows: i32 = 0;
         let mut pos: i32 = 0;
        while pos < string_representation.length() {
            if string_representation.char_at(pos) == '\n' || string_representation.char_at(pos) == '\r' {
                if bits_pos > row_start_pos {
                    if row_length == -1 {
                        row_length = bits_pos - row_start_pos;
                    } else if bits_pos - row_start_pos != row_length {
                        throw IllegalArgumentException::new("row lengths do not match");
                    }
                    row_start_pos = bits_pos;
                    n_rows += 1;
                }
                pos += 1;
            } else if string_representation.starts_with(&set_string, pos) {
                pos += set_string.length();
                bits[bits_pos] = true;
                bits_pos += 1;
            } else if string_representation.starts_with(&unset_string, pos) {
                pos += unset_string.length();
                bits[bits_pos] = false;
                bits_pos += 1;
            } else {
                throw IllegalArgumentException::new(format!("illegal character encountered: {}", string_representation.substring(pos)));
            }
        }
        // no EOL at end?
        if bits_pos > row_start_pos {
            if row_length == -1 {
                row_length = bits_pos - row_start_pos;
            } else if bits_pos - row_start_pos != row_length {
                throw IllegalArgumentException::new("row lengths do not match");
            }
            n_rows += 1;
        }
         let matrix: BitMatrix = BitMatrix::new(row_length, n_rows);
         {
             let mut i: i32 = 0;
            while i < bits_pos {
                {
                    if bits[i] {
                        matrix.set(i % row_length, i / row_length);
                    }
                }
                i += 1;
             }
         }

        return matrix;
    }

    /**
   * <p>Gets the requested bit, where true means black.</p>
   *
   * @param x The horizontal component (i.e. which column)
   * @param y The vertical component (i.e. which row)
   * @return value of given bit in matrix
   */
    pub fn  get(&self,  x: i32,  y: i32) -> bool  {
         let offset: i32 = y * self.row_size + (x / 32);
        return ((self.bits[offset] >> /* >>> */ (x & 0x1f)) & 1) != 0;
    }

    /**
   * <p>Sets the given bit to true.</p>
   *
   * @param x The horizontal component (i.e. which column)
   * @param y The vertical component (i.e. which row)
   */
    pub fn  set(&self,  x: i32,  y: i32)   {
         let mut offset: i32 = y * self.row_size + (x / 32);
        self.bits[offset] |= 1 << (x & 0x1f);
    }

    pub fn  unset(&self,  x: i32,  y: i32)   {
         let mut offset: i32 = y * self.row_size + (x / 32);
        self.bits[offset] &= ~(1 << (x & 0x1f));
    }

    /**
   * <p>Flips the given bit.</p>
   *
   * @param x The horizontal component (i.e. which column)
   * @param y The vertical component (i.e. which row)
   */
    pub fn  flip(&self,  x: i32,  y: i32)   {
         let mut offset: i32 = y * self.row_size + (x / 32);
        self.bits[offset] ^= 1 << (x & 0x1f);
    }

    /**
   * <p>Flips every bit in the matrix.</p>
   */
    pub fn  flip(&self)   {
         let max: i32 = self.bits.len();
         {
             let mut i: i32 = 0;
            while i < max {
                {
                    self.bits[i] = ~self.bits[i];
                }
                i += 1;
             }
         }

    }

    /**
   * Exclusive-or (XOR): Flip the bit in this {@code BitMatrix} if the corresponding
   * mask bit is set.
   *
   * @param mask XOR mask
   */
    pub fn  xor(&self,  mask: &BitMatrix)   {
        if self.width != mask.width || self.height != mask.height || self.row_size != mask.rowSize {
            throw IllegalArgumentException::new("input matrix dimensions do not match");
        }
         let row_array: BitArray = BitArray::new(self.width);
         {
             let mut y: i32 = 0;
            while y < self.height {
                {
                     let mut offset: i32 = y * self.row_size;
                     let row: Vec<i32> = mask.get_row(y, row_array).get_bit_array();
                     {
                         let mut x: i32 = 0;
                        while x < self.row_size {
                            {
                                self.bits[offset + x] ^= row[x];
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

    }

    /**
   * Clears all bits (sets to false).
   */
    pub fn  clear(&self)   {
         let max: i32 = self.bits.len();
         {
             let mut i: i32 = 0;
            while i < max {
                {
                    self.bits[i] = 0;
                }
                i += 1;
             }
         }

    }

    /**
   * <p>Sets a square region of the bit matrix to true.</p>
   *
   * @param left The horizontal position to begin at (inclusive)
   * @param top The vertical position to begin at (inclusive)
   * @param width The width of the region
   * @param height The height of the region
   */
    pub fn  set_region(&self,  left: i32,  top: i32,  width: i32,  height: i32)   {
        if top < 0 || left < 0 {
            throw IllegalArgumentException::new("Left and top must be nonnegative");
        }
        if height < 1 || width < 1 {
            throw IllegalArgumentException::new("Height and width must be at least 1");
        }
         let right: i32 = left + width;
         let bottom: i32 = top + height;
        if bottom > self.height || right > self.width {
            throw IllegalArgumentException::new("The region must fit inside the matrix");
        }
         {
             let mut y: i32 = top;
            while y < bottom {
                {
                     let mut offset: i32 = y * self.row_size;
                     {
                         let mut x: i32 = left;
                        while x < right {
                            {
                                self.bits[offset + (x / 32)] |= 1 << (x & 0x1f);
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

    }

    /**
   * A fast method to retrieve one row of data from the matrix as a BitArray.
   *
   * @param y The row to retrieve
   * @param row An optional caller-allocated BitArray, will be allocated if null or too small
   * @return The resulting BitArray - this reference should always be used even when passing
   *         your own row
   */
    pub fn  get_row(&self,  y: i32,  row: &BitArray) -> BitArray  {
        if row == null || row.get_size() < self.width {
            row = BitArray::new(self.width);
        } else {
            row.clear();
        }
         let offset: i32 = y * self.row_size;
         {
             let mut x: i32 = 0;
            while x < self.row_size {
                {
                    row.set_bulk(x * 32, self.bits[offset + x]);
                }
                x += 1;
             }
         }

        return row;
    }

    /**
   * @param y row to set
   * @param row {@link BitArray} to copy from
   */
    pub fn  set_row(&self,  y: i32,  row: &BitArray)   {
        System::arraycopy(&row.get_bit_array(), 0, &self.bits, y * self.row_size, self.row_size);
    }

    /**
   * Modifies this {@code BitMatrix} to represent the same but rotated the given degrees (0, 90, 180, 270)
   *
   * @param degrees number of degrees to rotate through counter-clockwise (0, 90, 180, 270)
   */
    pub fn  rotate(&self,  degrees: i32)   {
        match degrees % 360 {
              0 => 
                 {
                    return;
                }
              90 => 
                 {
                    self.rotate90();
                    return;
                }
              180 => 
                 {
                    self.rotate180();
                    return;
                }
              270 => 
                 {
                    self.rotate90();
                    self.rotate180();
                    return;
                }
        }
        throw IllegalArgumentException::new("degrees must be a multiple of 0, 90, 180, or 270");
    }

    /**
   * Modifies this {@code BitMatrix} to represent the same but rotated 180 degrees
   */
    pub fn  rotate180(&self)   {
         let top_row: BitArray = BitArray::new(self.width);
         let bottom_row: BitArray = BitArray::new(self.width);
         let max_height: i32 = (self.height + 1) / 2;
         {
             let mut i: i32 = 0;
            while i < max_height {
                {
                    top_row = self.get_row(i, top_row);
                     let bottom_row_index: i32 = self.height - 1 - i;
                    bottom_row = self.get_row(bottom_row_index, bottom_row);
                    top_row.reverse();
                    bottom_row.reverse();
                    self.set_row(i, bottom_row);
                    self.set_row(bottom_row_index, top_row);
                }
                i += 1;
             }
         }

    }

    /**
   * Modifies this {@code BitMatrix} to represent the same but rotated 90 degrees counterclockwise
   */
    pub fn  rotate90(&self)   {
         let new_width: i32 = self.height;
         let new_height: i32 = self.width;
         let new_row_size: i32 = (new_width + 31) / 32;
         let new_bits: [i32; new_row_size * new_height] = [0; new_row_size * new_height];
         {
             let mut y: i32 = 0;
            while y < self.height {
                {
                     {
                         let mut x: i32 = 0;
                        while x < self.width {
                            {
                                 let offset: i32 = y * self.row_size + (x / 32);
                                if ((self.bits[offset] >> /* >>> */ (x & 0x1f)) & 1) != 0 {
                                     let new_offset: i32 = (new_height - 1 - x) * new_row_size + (y / 32);
                                    new_bits[new_offset] |= 1 << (y & 0x1f);
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        self.width = new_width;
        self.height = new_height;
        self.row_size = new_row_size;
        self.bits = new_bits;
    }

    /**
   * This is useful in detecting the enclosing rectangle of a 'pure' barcode.
   *
   * @return {@code left,top,width,height} enclosing rectangle of all 1 bits, or null if it is all white
   */
    pub fn  get_enclosing_rectangle(&self) -> Vec<i32>  {
         let mut left: i32 = self.width;
         let mut top: i32 = self.height;
         let mut right: i32 = -1;
         let mut bottom: i32 = -1;
         {
             let mut y: i32 = 0;
            while y < self.height {
                {
                     {
                         let mut x32: i32 = 0;
                        while x32 < self.row_size {
                            {
                                 let the_bits: i32 = self.bits[y * self.row_size + x32];
                                if the_bits != 0 {
                                    if y < top {
                                        top = y;
                                    }
                                    if y > bottom {
                                        bottom = y;
                                    }
                                    if x32 * 32 < left {
                                         let mut bit: i32 = 0;
                                        while (the_bits << (31 - bit)) == 0 {
                                            bit += 1;
                                        }
                                        if (x32 * 32 + bit) < left {
                                            left = x32 * 32 + bit;
                                        }
                                    }
                                    if x32 * 32 + 31 > right {
                                         let mut bit: i32 = 31;
                                        while (the_bits >> /* >>> */ bit) == 0 {
                                            bit -= 1;
                                        }
                                        if (x32 * 32 + bit) > right {
                                            right = x32 * 32 + bit;
                                        }
                                    }
                                }
                            }
                            x32 += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        if right < left || bottom < top {
            return null;
        }
        return  : vec![i32; 4] = vec![left, top, right - left + 1, bottom - top + 1, ]
        ;
    }

    /**
   * This is useful in detecting a corner of a 'pure' barcode.
   *
   * @return {@code x,y} coordinate of top-left-most 1 bit, or null if it is all white
   */
    pub fn  get_top_left_on_bit(&self) -> Vec<i32>  {
         let bits_offset: i32 = 0;
        while bits_offset < self.bits.len() && self.bits[bits_offset] == 0 {
            bits_offset += 1;
        }
        if bits_offset == self.bits.len() {
            return null;
        }
         let y: i32 = bits_offset / self.row_size;
         let mut x: i32 = (bits_offset % self.row_size) * 32;
         let the_bits: i32 = self.bits[bits_offset];
         let mut bit: i32 = 0;
        while (the_bits << (31 - bit)) == 0 {
            bit += 1;
        }
        x += bit;
        return  : vec![i32; 2] = vec![x, y, ]
        ;
    }

    pub fn  get_bottom_right_on_bit(&self) -> Vec<i32>  {
         let bits_offset: i32 = self.bits.len() - 1;
        while bits_offset >= 0 && self.bits[bits_offset] == 0 {
            bits_offset -= 1;
        }
        if bits_offset < 0 {
            return null;
        }
         let y: i32 = bits_offset / self.row_size;
         let mut x: i32 = (bits_offset % self.row_size) * 32;
         let the_bits: i32 = self.bits[bits_offset];
         let mut bit: i32 = 31;
        while (the_bits >> /* >>> */ bit) == 0 {
            bit -= 1;
        }
        x += bit;
        return  : vec![i32; 2] = vec![x, y, ]
        ;
    }

    /**
   * @return The width of the matrix
   */
    pub fn  get_width(&self) -> i32  {
        return self.width;
    }

    /**
   * @return The height of the matrix
   */
    pub fn  get_height(&self) -> i32  {
        return self.height;
    }

    /**
   * @return The row size of the matrix
   */
    pub fn  get_row_size(&self) -> i32  {
        return self.row_size;
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof BitMatrix) {
            return false;
        }
         let other: BitMatrix = o as BitMatrix;
        return self.width == other.width && self.height == other.height && self.row_size == other.rowSize && Arrays::equals(&self.bits, other.bits);
    }

    pub fn  hash_code(&self) -> i32  {
         let mut hash: i32 = self.width;
        hash = 31 * hash + self.width;
        hash = 31 * hash + self.height;
        hash = 31 * hash + self.row_size;
        hash = 31 * hash + Arrays::hash_code(&self.bits);
        return hash;
    }

    /**
   * @return string representation using "X" for set and " " for unset bits
   */
    pub fn  to_string(&self) -> String  {
        return self.to_string("X ", "  ");
    }

    /**
   * @param setString representation of a set bit
   * @param unsetString representation of an unset bit
   * @return string representation of entire matrix utilizing given strings
   */
    pub fn  to_string(&self,  set_string: &String,  unset_string: &String) -> String  {
        return self.build_to_string(&set_string, &unset_string, "\n");
    }

    /**
   * @param setString representation of a set bit
   * @param unsetString representation of an unset bit
   * @param lineSeparator newline character in string representation
   * @return string representation of entire matrix utilizing given strings and line separator
   * @deprecated call {@link #toString(String,String)} only, which uses \n line separator always
   */
    pub fn  to_string(&self,  set_string: &String,  unset_string: &String,  line_separator: &String) -> String  {
        return self.build_to_string(&set_string, &unset_string, &line_separator);
    }

    fn  build_to_string(&self,  set_string: &String,  unset_string: &String,  line_separator: &String) -> String  {
         let result: StringBuilder = StringBuilder::new(self.height * (self.width + 1));
         {
             let mut y: i32 = 0;
            while y < self.height {
                {
                     {
                         let mut x: i32 = 0;
                        while x < self.width {
                            {
                                result.append( if self.get(x, y) { set_string } else { unset_string });
                            }
                            x += 1;
                         }
                     }

                    result.append(&line_separator);
                }
                y += 1;
             }
         }

        return result.to_string();
    }

    pub fn  clone(&self) -> BitMatrix  {
        return BitMatrix::new(self.width, self.height, self.row_size, &self.bits.clone());
    }
}

