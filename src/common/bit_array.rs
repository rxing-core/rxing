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

use num::traits::ops::overflowing::OverflowingSub;

use crate::common::Result;
use crate::Exceptions;

const LOAD_FACTOR: f32 = 0.75;

type BaseType = super::BitFieldBaseType;
const BASE_BITS: usize = super::BIT_FIELD_BASE_BITS;
const SHIFT_BITS: usize = super::BIT_FIELD_SHIFT_BITS;

/**
 * <p>A simple, fast array of bits, represented compactly by an array of ints internally.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct BitArray {
    bits: Vec<BaseType>,
    size: usize,
    read_offset: usize,
}

impl BitArray {
    pub fn new() -> Self {
        Self {
            bits: Vec::new(),
            size: 0,
            read_offset: 0,
        }
    }

    pub fn with_size(size: usize) -> Self {
        Self {
            bits: makeArray(size),
            size,
            read_offset: 0,
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            bits: makeArray(size),
            size: 0,
            read_offset: 0,
        }
    }

    /// For testing only
    #[cfg(test)]
    pub fn with_initial_values(bits: Vec<BaseType>, size: usize) -> Self {
        Self {
            bits,
            size,
            read_offset: 0,
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn getSizeInBytes(&self) -> usize {
        (self.size + 7) / 8
    }

    #[inline]
    fn ensure_capacity(&mut self, newSize: usize) {
        let target_size = (newSize as f32 / LOAD_FACTOR).ceil() as usize;
        let array_desired_length = target_size.div_ceil(BASE_BITS);

        if self.bits.len() < array_desired_length {
            let additional_capacity = array_desired_length - self.bits.len();
            self.bits.extend(vec![0; additional_capacity]);
        }
    }

    /**
     * @param i bit to get
     * @return true iff bit i is set
     */
    pub fn get(&self, i: usize) -> bool {
        (self.bits[i / BASE_BITS] & (1 << (i & SHIFT_BITS))) != 0
    }

    pub fn try_get(&self, i: usize) -> Option<bool> {
        if (i / BASE_BITS) >= self.bits.len() {
            None
        } else {
            Some(self.get(i))
        }
    }

    /**
     * Sets bit i.
     *
     * @param i bit to set
     */
    pub fn set(&mut self, i: usize) {
        self.bits[i / BASE_BITS] |= 1 << (i & SHIFT_BITS);
    }

    /**
     * Sets bit i.
     *
     * @param i bit to set
     */
    pub fn unset(&mut self, i: usize) {
        self.bits[i / BASE_BITS] |= 0 << (i & SHIFT_BITS);
    }

    /**
     * Flips bit i.
     *
     * @param i bit to set
     */
    pub fn flip(&mut self, i: usize) {
        self.bits[i / BASE_BITS] ^= 1 << (i & SHIFT_BITS);
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
        let mut bitsOffset = from / BASE_BITS;
        let mut currentBits = self.bits[bitsOffset] as i128;
        // mask off lesser bits first
        currentBits &= -(1 << (from & SHIFT_BITS));
        while currentBits == 0 {
            bitsOffset += 1;
            if bitsOffset == self.bits.len() {
                return self.size;
            }
            currentBits = self.bits[bitsOffset] as i128;
        }
        let result = (bitsOffset * BASE_BITS) + currentBits.trailing_zeros() as usize;
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
        let mut bitsOffset = from / BASE_BITS;
        let mut currentBits = !self.bits[bitsOffset] as i128;
        // mask off lesser bits first
        currentBits &= -(1 << (from & SHIFT_BITS));
        while currentBits == 0 {
            bitsOffset += 1;
            if bitsOffset == self.bits.len() {
                return self.size;
            }
            currentBits = !self.bits[bitsOffset] as i128;
        }
        let result = (bitsOffset * BASE_BITS) + currentBits.trailing_zeros() as usize;
        cmp::min(result, self.size)
    }

    /**
     * Sets a block of 32 bits, starting at bit i.
     *
     * @param i first bit to set
     * @param newBits the new value of the next 32 bits. Note again that the least-significant bit
     * corresponds to bit i, the next-least-significant to i+1, and so on.
     */
    pub fn setBulk(&mut self, i: usize, newBits: BaseType) {
        let bits = if i % BASE_BITS != 0 {
            newBits << i
        } else {
            newBits
        };
        self.bits[i / BASE_BITS] = bits;
    }

    /**
     * Sets a range of bits.
     *
     * @param start start of range, inclusive.
     * @param end end of range, exclusive
     */
    pub fn setRange(&mut self, start: usize, end: usize) -> Result<()> {
        let mut end = end;
        if end < start || end > self.size {
            return Err(Exceptions::ILLEGAL_ARGUMENT);
        }
        if end == start {
            return Ok(());
        }
        end -= 1; // will be easier to treat this as the last actually set bit -- inclusive
        let firstInt = start / BASE_BITS;
        let lastInt = end / BASE_BITS;
        for i in firstInt..=lastInt {
            //for (int i = firstInt; i <= lastInt; i++) {
            let firstBit = if i > firstInt { 0 } else { start & SHIFT_BITS };
            let lastBit = if i < lastInt {
                SHIFT_BITS
            } else {
                end & SHIFT_BITS
            };
            // Ones from firstBit to lastBit, inclusive
            let mask: u64 = (2 << lastBit) - (1 << firstBit);
            self.bits[i] |= mask as BaseType;
        }
        Ok(())
    }

    /**
     * Clears all bits (sets to false).
     */
    pub fn clear(&mut self) {
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
    pub fn isRange(&self, start: usize, end: usize, value: bool) -> Result<bool> {
        let mut end = end;
        if end < start || end > self.size {
            return Err(Exceptions::ILLEGAL_ARGUMENT);
        }
        if end == start {
            return Ok(true); // empty range matches
        }
        end -= 1; // will be easier to treat this as the last actually set bit -- inclusive
        let firstInt = start / BASE_BITS;
        let lastInt = end / BASE_BITS;
        for i in firstInt..=lastInt {
            //for (int i = firstInt; i <= lastInt; i++) {
            let firstBit = if i > firstInt { 0 } else { start & SHIFT_BITS };
            let lastBit = if i < lastInt {
                SHIFT_BITS
            } else {
                end & SHIFT_BITS
            };
            // Ones from firstBit to lastBit, inclusive
            let (mask, _): (BaseType, _) = (2 << lastBit).overflowing_sub(&(1 << firstBit));
            // let mask: u128 = (2 << lastBit) - (1 << firstBit);

            // Return false if we're looking for 1s and the masked bits[i] isn't all 1s (that is,
            // equals the mask, or we're looking for 0s and the masked portion is not all 0s
            if (self.bits[i] & mask as BaseType) != (if value { mask as BaseType } else { 0 }) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn appendBit(&mut self, bit: bool) {
        self.ensure_capacity(self.size + 1);
        if bit {
            self.bits[self.size / BASE_BITS] |= 1 << (self.size & SHIFT_BITS);
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
    pub fn appendBits(&mut self, value: BaseType, num_bits: usize) -> Result<()> {
        if num_bits > BASE_BITS {
            return Err(Exceptions::illegal_argument_with(format!(
                "num bits must be between 0 and {}",
                BaseType::BITS
            )));
        }

        if num_bits == 0 {
            return Ok(());
        }

        let mut next_size = self.size;
        self.ensure_capacity(next_size + num_bits);
        for numBitsLeft in (0..num_bits).rev() {
            //for (int numBitsLeft = numBits - 1; numBitsLeft >= 0; numBitsLeft--) {
            if (value & (1 << numBitsLeft)) != 0 {
                self.bits[next_size / BASE_BITS] |= 1 << (next_size & SHIFT_BITS);
            }
            next_size += 1;
        }
        self.size = next_size;
        Ok(())
    }

    pub fn appendBitArray(&mut self, other: BitArray) {
        self.appendBitArrayRef(&other)
    }

    pub fn appendBitArrayRef(&mut self, other: &BitArray) {
        let otherSize = other.size;
        self.ensure_capacity(self.size + otherSize);
        for i in 0..otherSize {
            //for (int i = 0; i < otherSize; i++) {
            self.appendBit(other.get(i));
        }
    }

    pub fn xor(&mut self, other: &BitArray) -> Result<()> {
        if self.size != other.size {
            return Err(Exceptions::illegal_argument_with("Sizes don't match"));
        }
        for (lhs, rhs) in self.bits.iter_mut().zip(other.bits.iter()) {
            //for (int i = 0; i < bits.length; i++) {
            // The last int could be incomplete (i.e. not have 32 bits in
            // it) but there is no problem since 0 XOR 0 == 0.
            *lhs ^= rhs;
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
            let mut the_byte = 0;
            for j in 0..8 {
                //for (int j = 0; j < 8; j++) {
                if self.get(bitOffset) {
                    the_byte |= 1 << (7 - j);
                }
                bitOffset += 1;
            }
            array[offset + i] = the_byte;
        }
    }

    /**
     * @return underlying array of ints. The first element holds the first 32 bits, and the least
     *         significant bit is bit 0.
     */
    pub fn getBitArray(&self) -> &[BaseType] {
        &self.bits
    }

    /**
     * Reverses all bits in the array.
     */
    pub fn reverse(&mut self) {
        // let mut newBits = vec![0; self.bits.len()];
        // reverse all int's first
        let len = (self.size - 1) / BASE_BITS;
        let oldBitsLen = len + 1;
        // let array_size = self.size.div_ceil(BASE_TYPE::BITS as usize);

        // for (new_bits, old_bits) in newBits.iter_mut().take(oldBitsLen).zip(self.bits.iter().take(oldBitsLen).rev()) {
        //     *new_bits = old_bits.reverse_bits();
        // }
        self.bits[..oldBitsLen].reverse();
        self.bits[..oldBitsLen]
            .iter_mut()
            .for_each(|bit_store| *bit_store = bit_store.reverse_bits());
        self.bits[oldBitsLen..].fill(0);

        // now correct the int's if the bit size isn't a multiple of 32
        if self.size != oldBitsLen * BASE_BITS {
            let leftOffset = oldBitsLen * BASE_BITS - self.size;
            let mut currentInt = self.bits[0] >> leftOffset;
            for i in 1..oldBitsLen {
                //for (int i = 1; i < oldBitsLen; i++) {
                let nextInt = self.bits[i];
                currentInt |= nextInt << (BASE_BITS - leftOffset);
                self.bits[i - 1] = currentInt;
                currentInt = nextInt >> leftOffset;
            }
            self.bits[oldBitsLen - 1] = currentInt;
        }
    }
}

impl fmt::Display for BitArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut _str = String::with_capacity(self.size + (self.size / 8) + 1);
        for i in 0..self.size {
            //for (int i = 0; i < size; i++) {
            if (i & 0x07) == 0 {
                _str.push(' ');
            }
            _str.push_str(if self.get(i) { "X" } else { "." });
        }
        write!(f, "{_str}")
    }
}

impl Default for BitArray {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BitArray> for Vec<u8> {
    fn from(value: BitArray) -> Self {
        (&value).into()
    }
}

impl From<&BitArray> for Vec<u8> {
    fn from(value: &BitArray) -> Self {
        let mut array = vec![0; value.getSizeInBytes()];
        value.toBytes(0, &mut array, 0, value.getSizeInBytes());
        array
    }
}

impl From<BitArray> for Vec<bool> {
    fn from(value: BitArray) -> Self {
        Self::from(&value)
    }
}

impl From<&BitArray> for Vec<bool> {
    fn from(value: &BitArray) -> Self {
        let mut array = vec![false; value.size];

        for (pixel, element) in array.iter_mut().enumerate().take(value.size) {
            *element = value.get(pixel);
        }

        array
    }
}

impl From<Vec<u8>> for BitArray {
    fn from(val: Vec<u8>) -> Self {
        let mut new_array = BitArray::with_capacity(val.len());
        for (pos, byte) in val.into_iter().enumerate() {
            if byte == 0 {
                new_array.set(pos)
            }
        }
        new_array
    }
}

fn makeArray(size: usize) -> Vec<BaseType> {
    vec![0; size.div_ceil(BASE_BITS)]
}

impl std::io::Read for BitArray {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = self.size;
        let desired = buf.len();
        let current_offset = self.read_offset;

        let available = size - current_offset;

        let to_read = if desired <= available {
            desired
        } else {
            available
        };

        self.toBytes(current_offset, buf, 0, to_read);

        self.read_offset = current_offset + to_read;

        Ok(to_read)
    }
}

impl std::io::Write for BitArray {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for byte in buf {
            self.appendBits(*byte as BaseType, 8)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
