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
 * <p>This provides an easy abstraction to read bits at a time from a sequence of bytes, where the
 * number of bits read is not often a multiple of 8.</p>
 *
 * <p>This class is thread-safe but not reentrant -- unless the caller modifies the bytes array
 * it passed in, in which case all bets are off.</p>
 *
 * @author Sean Owen
 */
pub struct BitSource {

     let bytes: Vec<i8>;

     let byte_offset: i32;

     let bit_offset: i32;
}

impl BitSource {

    /**
   * @param bytes bytes from which this will read bits. Bits will be read from the first byte first.
   * Bits are read within a byte from most-significant to least-significant bit.
   */
    pub fn new( bytes: &Vec<i8>) -> BitSource {
        let .bytes = bytes;
    }

    /**
   * @return index of next bit in current byte which would be read by the next call to {@link #readBits(int)}.
   */
    pub fn  get_bit_offset(&self) -> i32  {
        return self.bit_offset;
    }

    /**
   * @return index of next byte in input byte array which would be read by the next call to {@link #readBits(int)}.
   */
    pub fn  get_byte_offset(&self) -> i32  {
        return self.byte_offset;
    }

    /**
   * @param numBits number of bits to read
   * @return int representing the bits read. The bits will appear as the least-significant
   *         bits of the int
   * @throws IllegalArgumentException if numBits isn't in [1,32] or more than is available
   */
    pub fn  read_bits(&self,  num_bits: i32) -> i32  {
        if num_bits < 1 || num_bits > 32 || num_bits > self.available() {
            throw IllegalArgumentException::new(&String::value_of(num_bits));
        }
         let mut result: i32 = 0;
        // First, read remainder from current byte
        if self.bit_offset > 0 {
             let bits_left: i32 = 8 - self.bit_offset;
             let to_read: i32 = Math::min(num_bits, bits_left);
             let bits_to_not_read: i32 = bits_left - to_read;
             let mask: i32 = (0xFF >> (8 - to_read)) << bits_to_not_read;
            result = (self.bytes[self.byte_offset] & mask) >> bits_to_not_read;
            num_bits -= to_read;
            self.bit_offset += to_read;
            if self.bit_offset == 8 {
                self.bit_offset = 0;
                self.byte_offset += 1;
            }
        }
        // Next read whole bytes
        if num_bits > 0 {
            while num_bits >= 8 {
                result = (result << 8) | (self.bytes[self.byte_offset] & 0xFF);
                self.byte_offset += 1;
                num_bits -= 8;
            }
            // Finally read a partial byte
            if num_bits > 0 {
                 let bits_to_not_read: i32 = 8 - num_bits;
                 let mask: i32 = (0xFF >> bits_to_not_read) << bits_to_not_read;
                result = (result << num_bits) | ((self.bytes[self.byte_offset] & mask) >> bits_to_not_read);
                self.bit_offset += num_bits;
            }
        }
        return result;
    }

    /**
   * @return number of bits that can be read successfully
   */
    pub fn  available(&self) -> i32  {
        return 8 * (self.bytes.len() - self.byte_offset) - self.bit_offset;
    }
}

