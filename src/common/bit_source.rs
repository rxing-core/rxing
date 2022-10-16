
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

//package com.google.zxing.common;

use std::cmp;

use crate::Exceptions;

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
    bytes: Vec<u8>,
    byte_offset: usize,
    bit_offset: usize,
}

impl BitSource {
    /**
     * @param bytes bytes from which this will read bits. Bits will be read from the first byte first.
     * Bits are read within a byte from most-significant to least-significant bit.
     */
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            byte_offset: 0,
            bit_offset: 0,
        }
    }

    /**
     * @return index of next bit in current byte which would be read by the next call to {@link #readBits(int)}.
     */
    pub fn getBitOffset(&self) -> usize {
        return self.bit_offset;
    }

    /**
     * @return index of next byte in input byte array which would be read by the next call to {@link #readBits(int)}.
     */
    pub fn getByteOffset(&self) -> usize {
        return self.byte_offset;
    }

    /**
     * @param numBits number of bits to read
     * @return int representing the bits read. The bits will appear as the least-significant
     *         bits of the int
     * @throws IllegalArgumentException if numBits isn't in [1,32] or more than is available
     */
    pub fn readBits(&mut self, numBits: usize) -> Result<u32, Exceptions> {
        if numBits < 1 || numBits > 32 || numBits > self.available() {
            return Err(Exceptions::IllegalArgumentException(numBits.to_string()));
        }

        let mut result: u32 = 0;

        let mut num_bits = numBits;

        // First, read remainder from current byte
        if self.bit_offset > 0 {
            let bitsLeft = 8 - self.bit_offset;
            let toRead = cmp::min(num_bits, bitsLeft);
            let bitsToNotRead = bitsLeft - toRead;
            let mask = (0xFF >> (8 - toRead)) << bitsToNotRead;

            result = (self.bytes[self.byte_offset] & mask) as u32 >> bitsToNotRead;
            num_bits -= toRead;
            self.bit_offset += toRead;
            if self.bit_offset == 8 {
                self.bit_offset = 0;
                self.byte_offset += 1;
            }
        }

        // Next read whole bytes
        if num_bits > 0 {
            while num_bits >= 8 {
                result = (result << 8) | (self.bytes[self.byte_offset] & 0xFF) as u32;
                // result = ((result as u16) << 8) as u8 | (self.bytes[self.byte_offset]);
                self.byte_offset += 1;
                num_bits -= 8;
            }

            // Finally read a partial byte
            if num_bits > 0 {
                let bits_to_not_read = 8 - num_bits;
                let mask = (0xFF >> bits_to_not_read) << bits_to_not_read;
                result = (result << num_bits)
                    | ((self.bytes[self.byte_offset] & mask) as u32 >> bits_to_not_read);
                self.bit_offset += num_bits;
            }
        }

        return Ok(result);
    }

    /**
     * @return number of bits that can be read successfully
     */
    pub fn available(&self) -> usize {
        return 8 * (self.bytes.len() - self.byte_offset) - self.bit_offset;
    }
}