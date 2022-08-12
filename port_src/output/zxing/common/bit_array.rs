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
 * <p>A simple, fast array of bits, represented compactly by an array of ints internally.</p>
 *
 * @author Sean Owen
 */

 const EMPTY_BITS;

 const LOAD_FACTOR: f32 = 0.75f;
#[derive(Cloneable)]
pub struct BitArray {

     let mut bits: Vec<i32>;

     let mut size: i32;
}

impl BitArray {

    pub fn new() -> BitArray {
        let .size = 0;
        let .bits = EMPTY_BITS;
    }

    pub fn new( size: i32) -> BitArray {
        let .size = size;
        let .bits = ::make_array(size);
    }

    // For testing only
    fn new( bits: &Vec<i32>,  size: i32) -> BitArray {
        let .bits = bits;
        let .size = size;
    }

    pub fn  get_size(&self) -> i32  {
        return self.size;
    }

    pub fn  get_size_in_bytes(&self) -> i32  {
        return (self.size + 7) / 8;
    }

    fn  ensure_capacity(&self,  new_size: i32)   {
        if new_size > self.bits.len() * 32 {
             let new_bits: Vec<i32> = ::make_array(Math::ceil(new_size / LOAD_FACTOR) as i32);
            System::arraycopy(&self.bits, 0, &new_bits, 0, self.bits.len());
            self.bits = new_bits;
        }
    }

    /**
   * @param i bit to get
   * @return true iff bit i is set
   */
    pub fn  get(&self,  i: i32) -> bool  {
        return (self.bits[i / 32] & (1 << (i & 0x1F))) != 0;
    }

    /**
   * Sets bit i.
   *
   * @param i bit to set
   */
    pub fn  set(&self,  i: i32)   {
        self.bits[i / 32] |= 1 << (i & 0x1F);
    }

    /**
   * Flips bit i.
   *
   * @param i bit to set
   */
    pub fn  flip(&self,  i: i32)   {
        self.bits[i / 32] ^= 1 << (i & 0x1F);
    }

    /**
   * @param from first bit to check
   * @return index of first bit that is set, starting from the given index, or size if none are set
   *  at or beyond this given index
   * @see #getNextUnset(int)
   */
    pub fn  get_next_set(&self,  from: i32) -> i32  {
        if from >= self.size {
            return self.size;
        }
         let bits_offset: i32 = from / 32;
         let current_bits: i32 = self.bits[bits_offset];
        // mask off lesser bits first
        current_bits &= -(1 << (from & 0x1F));
        while current_bits == 0 {
            if bits_offset += 1 == self.bits.len() {
                return self.size;
            }
            current_bits = self.bits[bits_offset];
        }
         let result: i32 = (bits_offset * 32) + Integer::number_of_trailing_zeros(current_bits);
        return Math::min(result, self.size);
    }

    /**
   * @param from index to start looking for unset bit
   * @return index of next unset bit, or {@code size} if none are unset until the end
   * @see #getNextSet(int)
   */
    pub fn  get_next_unset(&self,  from: i32) -> i32  {
        if from >= self.size {
            return self.size;
        }
         let bits_offset: i32 = from / 32;
         let current_bits: i32 = ~self.bits[bits_offset];
        // mask off lesser bits first
        current_bits &= -(1 << (from & 0x1F));
        while current_bits == 0 {
            if bits_offset += 1 == self.bits.len() {
                return self.size;
            }
            current_bits = ~self.bits[bits_offset];
        }
         let result: i32 = (bits_offset * 32) + Integer::number_of_trailing_zeros(current_bits);
        return Math::min(result, self.size);
    }

    /**
   * Sets a block of 32 bits, starting at bit i.
   *
   * @param i first bit to set
   * @param newBits the new value of the next 32 bits. Note again that the least-significant bit
   * corresponds to bit i, the next-least-significant to i+1, and so on.
   */
    pub fn  set_bulk(&self,  i: i32,  new_bits: i32)   {
        self.bits[i / 32] = new_bits;
    }

    /**
   * Sets a range of bits.
   *
   * @param start start of range, inclusive.
   * @param end end of range, exclusive
   */
    pub fn  set_range(&self,  start: i32,  end: i32)   {
        if end < start || start < 0 || end > self.size {
            throw IllegalArgumentException::new();
        }
        if end == start {
            return;
        }
        // will be easier to treat this as the last actually set bit -- inclusive
        end -= 1;
         let first_int: i32 = start / 32;
         let last_int: i32 = end / 32;
         {
             let mut i: i32 = first_int;
            while i <= last_int {
                {
                     let first_bit: i32 =  if i > first_int { 0 } else { start & 0x1F };
                     let last_bit: i32 =  if i < last_int { 31 } else { end & 0x1F };
                    // Ones from firstBit to lastBit, inclusive
                     let mask: i32 = (2 << last_bit) - (1 << first_bit);
                    self.bits[i] |= mask;
                }
                i += 1;
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
   * Efficient method to check if a range of bits is set, or not set.
   *
   * @param start start of range, inclusive.
   * @param end end of range, exclusive
   * @param value if true, checks that bits in range are set, otherwise checks that they are not set
   * @return true iff all bits are set or not set in range, according to value argument
   * @throws IllegalArgumentException if end is less than start or the range is not contained in the array
   */
    pub fn  is_range(&self,  start: i32,  end: i32,  value: bool) -> bool  {
        if end < start || start < 0 || end > self.size {
            throw IllegalArgumentException::new();
        }
        if end == start {
            // empty range matches
            return true;
        }
        // will be easier to treat this as the last actually set bit -- inclusive
        end -= 1;
         let first_int: i32 = start / 32;
         let last_int: i32 = end / 32;
         {
             let mut i: i32 = first_int;
            while i <= last_int {
                {
                     let first_bit: i32 =  if i > first_int { 0 } else { start & 0x1F };
                     let last_bit: i32 =  if i < last_int { 31 } else { end & 0x1F };
                    // Ones from firstBit to lastBit, inclusive
                     let mask: i32 = (2 << last_bit) - (1 << first_bit);
                    // equals the mask, or we're looking for 0s and the masked portion is not all 0s
                    if (self.bits[i] & mask) != ( if value { mask } else { 0 }) {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    pub fn  append_bit(&self,  bit: bool)   {
        self.ensure_capacity(self.size + 1);
        if bit {
            self.bits[self.size / 32] |= 1 << (self.size & 0x1F);
        }
        self.size += 1;
    }

    /**
   * Appends the least-significant bits, from value, in order from most-significant to
   * least-significant. For example, appending 6 bits from 0x000001E will append the bits
   * 0, 1, 1, 1, 1, 0 in that order.
   *
   * @param value {@code int} containing bits to append
   * @param numBits bits from value to append
   */
    pub fn  append_bits(&self,  value: i32,  num_bits: i32)   {
        if num_bits < 0 || num_bits > 32 {
            throw IllegalArgumentException::new("Num bits must be between 0 and 32");
        }
         let next_size: i32 = self.size;
        self.ensure_capacity(next_size + num_bits);
         {
             let num_bits_left: i32 = num_bits - 1;
            while num_bits_left >= 0 {
                {
                    if (value & (1 << num_bits_left)) != 0 {
                        self.bits[next_size / 32] |= 1 << (next_size & 0x1F);
                    }
                    next_size += 1;
                }
                num_bits_left -= 1;
             }
         }

        self.size = next_size;
    }

    pub fn  append_bit_array(&self,  other: &BitArray)   {
         let other_size: i32 = other.size;
        self.ensure_capacity(self.size + other_size);
         {
             let mut i: i32 = 0;
            while i < other_size {
                {
                    self.append_bit(&other.get(i));
                }
                i += 1;
             }
         }

    }

    pub fn  xor(&self,  other: &BitArray)   {
        if self.size != other.size {
            throw IllegalArgumentException::new("Sizes don't match");
        }
         {
             let mut i: i32 = 0;
            while i < self.bits.len() {
                {
                    // The last int could be incomplete (i.e. not have 32 bits in
                    // it) but there is no problem since 0 XOR 0 == 0.
                    self.bits[i] ^= other.bits[i];
                }
                i += 1;
             }
         }

    }

    /**
   *
   * @param bitOffset first bit to start writing
   * @param array array to write into. Bytes are written most-significant byte first. This is the opposite
   *  of the internal representation, which is exposed by {@link #getBitArray()}
   * @param offset position in array to start writing
   * @param numBytes how many bytes to write
   */
    pub fn  to_bytes(&self,  bit_offset: i32,  array: &Vec<i8>,  offset: i32,  num_bytes: i32)   {
         {
             let mut i: i32 = 0;
            while i < num_bytes {
                {
                     let the_byte: i32 = 0;
                     {
                         let mut j: i32 = 0;
                        while j < 8 {
                            {
                                if self.get(bit_offset) {
                                    the_byte |= 1 << (7 - j);
                                }
                                bit_offset += 1;
                            }
                            j += 1;
                         }
                     }

                    array[offset + i] = the_byte as i8;
                }
                i += 1;
             }
         }

    }

    /**
   * @return underlying array of ints. The first element holds the first 32 bits, and the least
   *         significant bit is bit 0.
   */
    pub fn  get_bit_array(&self) -> Vec<i32>  {
        return self.bits;
    }

    /**
   * Reverses all bits in the array.
   */
    pub fn  reverse(&self)   {
         let new_bits: [i32; self.bits.len()] = [0; self.bits.len()];
        // reverse all int's first
         let mut len: i32 = (self.size - 1) / 32;
         let old_bits_len: i32 = len + 1;
         {
             let mut i: i32 = 0;
            while i < old_bits_len {
                {
                    new_bits[len - i] = Integer::reverse(self.bits[i]);
                }
                i += 1;
             }
         }

        // now correct the int's if the bit size isn't a multiple of 32
        if self.size != old_bits_len * 32 {
             let left_offset: i32 = old_bits_len * 32 - self.size;
             let current_int: i32 = new_bits[0] >> /* >>> */ left_offset;
             {
                 let mut i: i32 = 1;
                while i < old_bits_len {
                    {
                         let next_int: i32 = new_bits[i];
                        current_int |= next_int << (32 - left_offset);
                        new_bits[i - 1] = current_int;
                        current_int = next_int >> /* >>> */ left_offset;
                    }
                    i += 1;
                 }
             }

            new_bits[old_bits_len - 1] = current_int;
        }
        self.bits = new_bits;
    }

    fn  make_array( size: i32) -> Vec<i32>  {
        return : [i32; (size + 31) / 32] = [0; (size + 31) / 32];
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof BitArray) {
            return false;
        }
         let other: BitArray = o as BitArray;
        return self.size == other.size && Arrays::equals(&self.bits, other.bits);
    }

    pub fn  hash_code(&self) -> i32  {
        return 31 * self.size + Arrays::hash_code(&self.bits);
    }

    pub fn  to_string(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(self.size + (self.size / 8) + 1);
         {
             let mut i: i32 = 0;
            while i < self.size {
                {
                    if (i & 0x07) == 0 {
                        result.append(' ');
                    }
                    result.append( if self.get(i) { 'X' } else { '.' });
                }
                i += 1;
             }
         }

        return result.to_string();
    }

    pub fn  clone(&self) -> BitArray  {
        return BitArray::new(&self.bits.clone(), self.size);
    }
}

