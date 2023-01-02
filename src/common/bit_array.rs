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

use std::{cmp, fmt};

use crate::Exceptions;

static EMPTY_BITS: [u32; 0] = [0; 0];
static LOAD_FACTOR: f32 = 0.75f32;

/**
 * <p>A simple, fast array of bits, represented compactly by an array of ints internally.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BitArray {
    bits: Vec<u32>,
    size: usize,
}

impl BitArray {
    pub fn new() -> Self {
        Self {
            bits: EMPTY_BITS.to_vec(),
            size: 0,
        }
    }

    pub fn with_size(size: usize) -> Self {
        Self {
            bits: BitArray::makeArray(size),
            size: size,
        }
    }

    // For testing only
    pub fn with_initial_values(bits: Vec<u32>, size: usize) -> Self {
        Self {
            bits: bits,
            size: size,
        }
    }

    pub fn getSize(&self) -> usize {
        self.size
    }

    pub fn getSizeInBytes(&self) -> usize {
        return (self.size + 7) / 8;
    }

    fn ensure_capacity(&mut self, newSize: usize) {
        if newSize > self.bits.len() * 32 {
            let mut newBits = BitArray::makeArray((newSize as f32 / LOAD_FACTOR).ceil() as usize);
            //System.arraycopy(bits, 0, newBits, 0, bits.length);
            newBits[0..self.bits.len()].clone_from_slice(&self.bits[0..self.bits.len()]);
            self.bits = newBits;
        }
    }

    /**
     * @param i bit to get
     * @return true iff bit i is set
     */
    pub fn get(&self, i: usize) -> bool {
        (self.bits[i / 32] & (1 << (i & 0x1F))) != 0
    }

    /**
     * Sets bit i.
     *
     * @param i bit to set
     */
    pub fn set(&mut self, i: usize) {
        self.bits[i / 32] |= 1 << (i & 0x1F);
    }

    /**
     * Flips bit i.
     *
     * @param i bit to set
     */
    pub fn flip(&mut self, i: usize) {
        self.bits[i / 32] ^= 1 << (i & 0x1F);
    }

    /**
     * @param from first bit to check
     * @return index of first bit that is set, starting from the given index, or size if none are set
     *  at or beyond this given index
     * @see #getNextUnset(int)
     */
    pub fn getNextSet(&self, from: usize) -> usize {
        if from >= self.size {
            return self.size;
        }
        let mut bitsOffset = from / 32;
        let mut currentBits = self.bits[bitsOffset] as i64;
        // mask off lesser bits first
        currentBits &= -(1 << (from & 0x1F));
        while currentBits == 0 {
            bitsOffset += 1;
            if bitsOffset == self.bits.len() {
                return self.size;
            }
            currentBits = self.bits[bitsOffset] as i64;
        }
        let result = (bitsOffset * 32) + currentBits.trailing_zeros() as usize;
        cmp::min(result, self.size)
    }

    /**
     * @param from index to start looking for unset bit
     * @return index of next unset bit, or {@code size} if none are unset until the end
     * @see #getNextSet(int)
     */
    pub fn getNextUnset(&self, from: usize) -> usize {
        if from >= self.size {
            return self.size;
        }
        let mut bitsOffset = from / 32;
        let mut currentBits = !self.bits[bitsOffset] as i64;
        // mask off lesser bits first
        currentBits &= -(1 << (from & 0x1F));
        while currentBits == 0 {
            bitsOffset += 1;
            if bitsOffset == self.bits.len() {
                return self.size;
            }
            currentBits = !self.bits[bitsOffset] as i64;
        }
        let result = (bitsOffset * 32) + currentBits.trailing_zeros() as usize;
        return cmp::min(result, self.size);
    }

    /**
     * Sets a block of 32 bits, starting at bit i.
     *
     * @param i first bit to set
     * @param newBits the new value of the next 32 bits. Note again that the least-significant bit
     * corresponds to bit i, the next-least-significant to i+1, and so on.
     */
    pub fn setBulk(&mut self, i: usize, newBits: u32) {
        self.bits[i / 32] = newBits;
    }

    /**
     * Sets a range of bits.
     *
     * @param start start of range, inclusive.
     * @param end end of range, exclusive
     */
    pub fn setRange(&mut self, start: usize, end: usize) -> Result<(), Exceptions> {
        let mut end = end;
        if end < start || end > self.size {
            return Err(Exceptions::IllegalArgumentException(
                "end < start || start < 0 || end > self.size".to_owned(),
            ));
        }
        if end == start {
            return Ok(());
        }
        end -= 1; // will be easier to treat this as the last actually set bit -- inclusive
        let firstInt = start / 32;
        let lastInt = end / 32;
        for i in firstInt..=lastInt {
            //for (int i = firstInt; i <= lastInt; i++) {
            let firstBit = if i > firstInt { 0 } else { start & 0x1F };
            let lastBit = if i < lastInt { 31 } else { end & 0x1F };
            // Ones from firstBit to lastBit, inclusive
            let mask: u64 = (2 << lastBit) - (1 << firstBit);
            self.bits[i] |= mask as u32;
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
     * Efficient method to check if a range of bits is set, or not set.
     *
     * @param start start of range, inclusive.
     * @param end end of range, exclusive
     * @param value if true, checks that bits in range are set, otherwise checks that they are not set
     * @return true iff all bits are set or not set in range, according to value argument
     * @throws IllegalArgumentException if end is less than start or the range is not contained in the array
     */
    pub fn isRange(&self, start: usize, end: usize, value: bool) -> Result<bool, Exceptions> {
        let mut end = end;
        if end < start || end > self.size {
            return Err(Exceptions::IllegalArgumentException(
                "end < start || start < 0 || end > self.size".to_owned(),
            ));
        }
        if end == start {
            return Ok(true); // empty range matches
        }
        end -= 1; // will be easier to treat this as the last actually set bit -- inclusive
        let firstInt = start / 32;
        let lastInt = end / 32;
        for i in firstInt..=lastInt {
            //for (int i = firstInt; i <= lastInt; i++) {
            let firstBit = if i > firstInt { 0 } else { start & 0x1F };
            let lastBit = if i < lastInt { 31 } else { end & 0x1F };
            // Ones from firstBit to lastBit, inclusive
            let mask: u64 = (2 << lastBit) - (1 << firstBit);

            // Return false if we're looking for 1s and the masked bits[i] isn't all 1s (that is,
            // equals the mask, or we're looking for 0s and the masked portion is not all 0s
            if (self.bits[i] & mask as u32) != (if value { mask as u32 } else { 0 }) {
                return Ok(false);
            }
        }
        return Ok(true);
    }

    pub fn appendBit(&mut self, bit: bool) {
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
    pub fn appendBits(&mut self, value: u32, num_bits: usize) -> Result<(), Exceptions> {
        if num_bits > 32 {
            return Err(Exceptions::IllegalArgumentException(
                "Num bits must be between 0 and 32".to_owned(),
            ));
        }

        if num_bits == 0 {
            return Ok(());
        }

        let mut next_size = self.size;
        self.ensure_capacity(next_size + num_bits);
        for numBitsLeft in (0..num_bits).rev() {
            //for (int numBitsLeft = numBits - 1; numBitsLeft >= 0; numBitsLeft--) {
            if (value & (1 << numBitsLeft)) != 0 {
                self.bits[next_size / 32] |= 1 << (next_size & 0x1F);
            }
            next_size += 1;
        }
        self.size = next_size;
        Ok(())
    }

    pub fn appendBitArray(&mut self, other: BitArray) {
        let otherSize = other.size;
        self.ensure_capacity(self.size + otherSize);
        for i in 0..otherSize {
            //for (int i = 0; i < otherSize; i++) {
            self.appendBit(other.get(i));
        }
    }

    pub fn xor(&mut self, other: &BitArray) -> Result<(), Exceptions> {
        if self.size != other.size {
            return Err(Exceptions::IllegalArgumentException(
                "Sizes don't match".to_owned(),
            ));
        }
        for i in 0..self.bits.len() {
            //for (int i = 0; i < bits.length; i++) {
            // The last int could be incomplete (i.e. not have 32 bits in
            // it) but there is no problem since 0 XOR 0 == 0.
            self.bits[i] ^= other.bits[i];
        }
        Ok(())
    }

    /**
     *
     * @param bitOffset first bit to start writing
     * @param array array to write into. Bytes are written most-significant byte first. This is the opposite
     *  of the internal representation, which is exposed by {@link #getBitArray()}
     * @param offset position in array to start writing
     * @param numBytes how many bytes to write
     */
    pub fn toBytes(&self, bitOffset: usize, array: &mut [u8], offset: usize, numBytes: usize) {
        let mut bitOffset = bitOffset;
        for i in 0..numBytes {
            //for (int i = 0; i < numBytes; i++) {
            let mut theByte = 0;
            for j in 0..8 {
                //for (int j = 0; j < 8; j++) {
                if self.get(bitOffset) {
                    theByte |= 1 << (7 - j);
                }
                bitOffset += 1;
            }
            array[offset + i] = theByte;
        }
    }

    /**
     * @return underlying array of ints. The first element holds the first 32 bits, and the least
     *         significant bit is bit 0.
     */
    pub fn getBitArray(&self) -> &Vec<u32> {
        return &self.bits;
    }

    /**
     * Reverses all bits in the array.
     */
    pub fn reverse(&mut self) {
        let mut newBits = vec![0; self.bits.len()];
        // reverse all int's first
        let len = (self.size - 1) / 32;
        let oldBitsLen = len + 1;
        for i in 0..oldBitsLen {
            //for (int i = 0; i < oldBitsLen; i++) {
            newBits[len - i] = self.bits[i].reverse_bits();
        }
        // now correct the int's if the bit size isn't a multiple of 32
        if self.size != oldBitsLen * 32 {
            let leftOffset = oldBitsLen * 32 - self.size;
            let mut currentInt = newBits[0] >> leftOffset;
            for i in 1..oldBitsLen {
                //for (int i = 1; i < oldBitsLen; i++) {
                let nextInt = newBits[i];
                currentInt |= nextInt << (32 - leftOffset);
                newBits[i - 1] = currentInt;
                currentInt = nextInt >> leftOffset;
            }
            newBits[oldBitsLen - 1] = currentInt;
        }
        self.bits = newBits;
    }

    fn makeArray(size: usize) -> Vec<u32> {
        return vec![0; (size + 31) / 32];
    }

    //   @Override
    //   public boolean equals(Object o) {
    //     if (!(o instanceof BitArray)) {
    //       return false;
    //     }
    //     BitArray other = (BitArray) o;
    //     return size == other.size && Arrays.equals(bits, other.bits);
    //   }

    //   @Override
    //   public int hashCode() {
    //     return 31 * size + Arrays.hashCode(bits);
    //   }

    //   @Override
    //   public BitArray clone() {
    //     return new BitArray(bits.clone(), size);
    //   }
}

impl fmt::Display for BitArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut _str = String::with_capacity(self.size + (self.size / 8) + 1);
        for i in 0..self.size {
            //for (int i = 0; i < size; i++) {
            if (i & 0x07) == 0 {
                _str.push_str(" ");
            }
            _str.push_str(if self.get(i) { "X" } else { "." });
        }
        write!(f, "{}", _str)
    }
}
