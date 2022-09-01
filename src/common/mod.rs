pub mod detector;
pub mod reedsolomon;

use core::num;
use std::any::Any;
use std::cmp;
use std::collections::HashMap;
use std::fmt;

use crate::DecodeHintType;
use crate::Exceptions;
use crate::RXingResultPoint;
use encoding::Encoding;

#[cfg(test)]
mod StringUtilsTestCase;

#[cfg(test)]
mod BitArrayTestCase;

#[cfg(test)]
mod BitMatrixTestCase;

#[cfg(test)]
mod BitSourceTestCase;

#[cfg(test)]
mod PerspectiveTransformTestCase;
/*
 * Copyright (C) 2010 ZXing authors
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

// import java.nio.charset.Charset;
// import java.nio.charset.StandardCharsets;
// import java.util.Map;

/**
 * Common string-related functions.
 *
 * @author Sean Owen
 * @author Alex Dupre
 */
pub struct StringUtils {
    //   private static final Charset PLATFORM_DEFAULT_ENCODING = Charset.defaultCharset();
    //   public static final Charset SHIFT_JIS_CHARSET = Charset.forName("SJIS");
    //   public static final Charset GB2312_CHARSET = Charset.forName("GB2312");
    //   private static final Charset EUC_JP = Charset.forName("EUC_JP");
    //   private static final boolean ASSUME_SHIFT_JIS =
    //       SHIFT_JIS_CHARSET.equals(PLATFORM_DEFAULT_ENCODING) ||
    //       EUC_JP.equals(PLATFORM_DEFAULT_ENCODING);

    //   // Retained for ABI compatibility with earlier versions
    //   public static final String SHIFT_JIS = "SJIS";
    //   public static final String GB2312 = "GB2312";
}

// const PLATFORM_DEFAULT_ENCODING: &dyn Encoding = encoding::all::UTF_8;
// const SHIFT_JIS_CHARSET: &dyn Encoding =
//     encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
// const GB2312_CHARSET: &dyn Encoding =
//     encoding::label::encoding_from_whatwg_label("GB2312").unwrap();
// const EUC_JP: &dyn Encoding = encoding::label::encoding_from_whatwg_label("EUC_JP").unwrap();
const ASSUME_SHIFT_JIS: bool = false;
static SHIFT_JIS: &'static str = "SJIS";
static GB2312: &'static str = "GB2312";

//    private static final boolean ASSUME_SHIFT_JIS =
//        SHIFT_JIS_CHARSET.equals(PLATFORM_DEFAULT_ENCODING) ||
//        EUC_JP.equals(PLATFORM_DEFAULT_ENCODING);

impl StringUtils {
    /**
     * @param bytes bytes encoding a string, whose encoding should be guessed
     * @param hints decode hints if applicable
     * @return name of guessed encoding; at the moment will only guess one of:
     *  "SJIS", "UTF8", "ISO8859_1", or the platform default encoding if none
     *  of these can possibly be correct
     */
    pub fn guessEncoding(bytes: &[u8], hints: HashMap<DecodeHintType, String>) -> String {
        let c = StringUtils::guessCharset(bytes, hints);
        if c.name()
            == encoding::label::encoding_from_whatwg_label("SJIS")
                .unwrap()
                .name()
        {
            return "SJIS".to_owned();
        } else if c.name() == encoding::all::UTF_8.name() {
            return "UTF8".to_owned();
        } else if c.name() == encoding::all::ISO_8859_1.name() {
            return "ISO8859_1".to_owned();
        }
        return c.name().to_owned();
    }

    /**
     * @param bytes bytes encoding a string, whose encoding should be guessed
     * @param hints decode hints if applicable
     * @return Charset of guessed encoding; at the moment will only guess one of:
     *  {@link #SHIFT_JIS_CHARSET}, {@link StandardCharsets#UTF_8},
     *  {@link StandardCharsets#ISO_8859_1}, {@link StandardCharsets#UTF_16},
     *  or the platform default encoding if
     *  none of these can possibly be correct
     */
    pub fn guessCharset(
        bytes: &[u8],
        hints: HashMap<DecodeHintType, String>,
    ) -> &'static dyn Encoding {
        match hints.get(&DecodeHintType::CHARACTER_SET) {
            Some(hint) => {
                return encoding::label::encoding_from_whatwg_label(hint).unwrap();
            }
            _ => {}
        };
        // if hints.contains_key(&DecodeHintType::CHARACTER_SET) {
        //   return Charset.forName(hints.get(DecodeHintType.CHARACTER_SET).toString());
        // }

        // First try UTF-16, assuming anything with its BOM is UTF-16
        if bytes.len() > 2
            && ((bytes[0] == 0xFE && bytes[1] == 0xFF) || (bytes[0] == 0xFF && bytes[1] == 0xFE))
        {
            if bytes[0] == 0xFE && bytes[1] == 0xFF {
                return encoding::all::UTF_16BE;
            } else {
                return encoding::all::UTF_16LE;
            }
        }

        // For now, merely tries to distinguish ISO-8859-1, UTF-8 and Shift_JIS,
        // which should be by far the most common encodings.
        let length = bytes.len();
        let mut canBeISO88591 = true;
        let mut canBeShiftJIS = true;
        let mut canBeUTF8 = true;
        let mut utf8BytesLeft = 0;
        let mut utf2BytesChars = 0;
        let mut utf3BytesChars = 0;
        let mut utf4BytesChars = 0;
        let mut sjisBytesLeft = 0;
        let mut sjisKatakanaChars = 0;
        let mut sjisCurKatakanaWordLength = 0;
        let mut sjisCurDoubleBytesWordLength = 0;
        let mut sjisMaxKatakanaWordLength = 0;
        let mut sjisMaxDoubleBytesWordLength = 0;
        let mut isoHighOther = 0;

        let utf8bom = bytes.len() > 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF;

        for i in 0..length {
            // for (int i = 0;
            //      i < length && (canBeISO88591 || canBeShiftJIS || canBeUTF8);
            //      i++) {
            if !(canBeISO88591 || canBeShiftJIS || canBeUTF8) {
                break;
            }

            let value = bytes[i] & 0xFF;

            // UTF-8 stuff
            if canBeUTF8 {
                if utf8BytesLeft > 0 {
                    if (value & 0x80) == 0 {
                        canBeUTF8 = false;
                    } else {
                        utf8BytesLeft -= 1;
                    }
                } else if (value & 0x80) != 0 {
                    if (value & 0x40) == 0 {
                        canBeUTF8 = false;
                    } else {
                        utf8BytesLeft += 1;
                        if (value & 0x20) == 0 {
                            utf2BytesChars += 1;
                        } else {
                            utf8BytesLeft += 1;
                            if (value & 0x10) == 0 {
                                utf3BytesChars += 1;
                            } else {
                                utf8BytesLeft += 1;
                                if (value & 0x08) == 0 {
                                    utf4BytesChars += 1;
                                } else {
                                    canBeUTF8 = false;
                                }
                            }
                        }
                    }
                }
            }

            // ISO-8859-1 stuff
            if canBeISO88591 {
                if value > 0x7F && value < 0xA0 {
                    canBeISO88591 = false;
                } else if value > 0x9F && (value < 0xC0 || value == 0xD7 || value == 0xF7) {
                    isoHighOther += 1;
                }
            }

            // Shift_JIS stuff
            if canBeShiftJIS {
                if sjisBytesLeft > 0 {
                    if value < 0x40 || value == 0x7F || value > 0xFC {
                        canBeShiftJIS = false;
                    } else {
                        sjisBytesLeft -= 1;
                    }
                } else if value == 0x80 || value == 0xA0 || value > 0xEF {
                    canBeShiftJIS = false;
                } else if value > 0xA0 && value < 0xE0 {
                    sjisKatakanaChars += 1;
                    sjisCurDoubleBytesWordLength = 0;
                    sjisCurKatakanaWordLength += 1;
                    if sjisCurKatakanaWordLength > sjisMaxKatakanaWordLength {
                        sjisMaxKatakanaWordLength = sjisCurKatakanaWordLength;
                    }
                } else if value > 0x7F {
                    sjisBytesLeft += 1;
                    //sjisDoubleBytesChars++;
                    sjisCurKatakanaWordLength = 0;
                    sjisCurDoubleBytesWordLength += 1;
                    if sjisCurDoubleBytesWordLength > sjisMaxDoubleBytesWordLength {
                        sjisMaxDoubleBytesWordLength = sjisCurDoubleBytesWordLength;
                    }
                } else {
                    //sjisLowChars++;
                    sjisCurKatakanaWordLength = 0;
                    sjisCurDoubleBytesWordLength = 0;
                }
            }
        }

        if canBeUTF8 && utf8BytesLeft > 0 {
            canBeUTF8 = false;
        }
        if canBeShiftJIS && sjisBytesLeft > 0 {
            canBeShiftJIS = false;
        }

        // Easy -- if there is BOM or at least 1 valid not-single byte character (and no evidence it can't be UTF-8), done
        if canBeUTF8 && (utf8bom || utf2BytesChars + utf3BytesChars + utf4BytesChars > 0) {
            return encoding::all::UTF_8;
        }
        // Easy -- if assuming Shift_JIS or >= 3 valid consecutive not-ascii characters (and no evidence it can't be), done
        if canBeShiftJIS
            && (ASSUME_SHIFT_JIS
                || sjisMaxKatakanaWordLength >= 3
                || sjisMaxDoubleBytesWordLength >= 3)
        {
            return encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
        }
        // Distinguishing Shift_JIS and ISO-8859-1 can be a little tough for short words. The crude heuristic is:
        // - If we saw
        //   - only two consecutive katakana chars in the whole text, or
        //   - at least 10% of bytes that could be "upper" not-alphanumeric Latin1,
        // - then we conclude Shift_JIS, else ISO-8859-1
        if canBeISO88591 && canBeShiftJIS {
            return if (sjisMaxKatakanaWordLength == 2 && sjisKatakanaChars == 2)
                || isoHighOther * 10 >= length
            {
                encoding::label::encoding_from_whatwg_label("SJIS").unwrap()
            } else {
                encoding::all::ISO_8859_1
            };
        }

        // Otherwise, try in order ISO-8859-1, Shift JIS, UTF-8 and fall back to default platform encoding
        if canBeISO88591 {
            return encoding::all::ISO_8859_1;
        }
        if canBeShiftJIS {
            return encoding::label::encoding_from_whatwg_label("SJIS").unwrap();
        }
        if canBeUTF8 {
            return encoding::all::UTF_8;
        }
        // Otherwise, we take a wild guess with platform encoding
        return encoding::all::UTF_8;
    }
}

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

    fn ensureCapacity(&mut self, newSize: usize) {
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
        return (self.bits[i / 32] & (1 << (i & 0x1F))) != 0;
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
        let mut currentBits = !self.bits[bitsOffset] as i32;
        // mask off lesser bits first
        currentBits &= -(1 << (from & 0x1F));
        while currentBits == 0 {
            bitsOffset += 1;
            if bitsOffset == self.bits.len() {
                return self.size;
            }
            currentBits = !self.bits[bitsOffset] as i32;
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
        if end < start || start < 0 || end > self.size {
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
        let max = self.bits.len();
        for i in 0..max {
            //for (int i = 0; i < max; i++) {
            self.bits[i] = 0;
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
    pub fn isRange(&self, start: usize, end: usize, value: bool) -> Result<bool, Exceptions> {
        let mut end = end;
        if end < start || start < 0 || end > self.size {
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
        self.ensureCapacity(self.size + 1);
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
    pub fn appendBits(&mut self, value: u32, numBits: usize) -> Result<(), Exceptions> {
        if numBits < 0 || numBits > 32 {
            return Err(Exceptions::IllegalArgumentException(
                "Num bits must be between 0 and 32".to_owned(),
            ));
        }
        let mut nextSize = self.size;
        self.ensureCapacity(nextSize + numBits);
        for numBitsLeft in (0..(numBits - 1)).rev() {
            //for (int numBitsLeft = numBits - 1; numBitsLeft >= 0; numBitsLeft--) {
            if (value & (1 << numBitsLeft)) != 0 {
                self.bits[nextSize / 32] |= 1 << (nextSize & 0x1F);
            }
            nextSize += 1;
        }
        self.size = nextSize;
        Ok(())
    }

    pub fn appendBitArray(&mut self, other: BitArray) {
        let otherSize = other.size;
        self.ensureCapacity(self.size + otherSize);
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

// import com.google.zxing.RXingResultPoint;

/**
 * <p>Encapsulates the result of detecting a barcode in an image. This includes the raw
 * matrix of black/white pixels corresponding to the barcode, and possibly points of interest
 * in the image, like the location of finder patterns or corners of the barcode in the image.</p>
 *
 * @author Sean Owen
 */
pub struct DetectorRXingResult {
    bits: BitMatrix,
    points: Vec<RXingResultPoint>,
}

impl DetectorRXingResult {
    pub fn new(bits: BitMatrix, points: Vec<RXingResultPoint>) -> Self {
        Self {
            bits: bits,
            points: points,
        }
    }

    pub fn getBits(&self) -> &BitMatrix {
        return &self.bits;
    }

    pub fn getPoints(&self) -> &Vec<RXingResultPoint> {
        return &self.points;
    }
}

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
    rowSize: usize,
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
            rowSize: ((width + 31) / 32) as usize,
            bits: vec![0; (((width + 31) / 32) * height) as usize],
        })
        // this.width = width;
        // this.height = height;
        // this.rowSize = (width + 31) / 32;
        // bits = new int[rowSize * height];
    }

    fn with_all_data(&self, width: u32, height: u32, rowSize: usize, bits: Vec<u32>) -> Self {
        Self {
            width,
            height,
            rowSize,
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
        stringRepresentation: &str,
        setString: &str,
        unsetString: &str,
    ) -> Result<Self, Exceptions> {
        // cannot pass nulls in rust
        // if (stringRepresentation == null) {
        //   throw new IllegalArgumentException();
        // }

        let mut bits = vec![false; stringRepresentation.len()];
        let mut bitsPos = 0;
        let mut rowStartPos = 0;
        let mut rowLength = 0; //-1;
        let mut first_run = true;
        let mut nRows = 0;
        let mut pos = 0;
        while pos < stringRepresentation.len() {
            if stringRepresentation.chars().nth(pos).unwrap() == '\n'
                || stringRepresentation.chars().nth(pos).unwrap() == '\r'
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
            } else if stringRepresentation[pos..].starts_with(setString) {
                pos += setString.len();
                bits[bitsPos] = true;
                bitsPos += 1;
            } else if stringRepresentation[pos..].starts_with(unsetString) {
                pos += unsetString.len();
                bits[bitsPos] = false;
                bitsPos += 1;
            } else {
                return Err(Exceptions::IllegalArgumentException(format!(
                    "illegal character encountered: {}",
                    stringRepresentation[pos..].to_owned()
                )));
            }
        }

        // no EOL at end?
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
        let offset = y as usize * self.rowSize + (x as usize / 32);
        return ((self.bits[offset] >> (x & 0x1f)) & 1) != 0;
    }

    /**
     * <p>Sets the given bit to true.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     */
    pub fn set(&mut self, x: u32, y: u32) {
        let offset = y as usize * self.rowSize + (x as usize / 32);
        self.bits[offset] |= 1 << (x & 0x1f);
    }

    pub fn unset(&mut self, x: u32, y: u32) {
        let offset = y as usize * self.rowSize + (x as usize / 32);
        self.bits[offset] &= !(1 << (x & 0x1f));
    }

    /**
     * <p>Flips the given bit.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     */
    pub fn flip_coords(&mut self, x: u32, y: u32) {
        let offset = y as usize * self.rowSize + (x as usize / 32);
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
        if self.width != mask.width || self.height != mask.height || self.rowSize != mask.rowSize {
            return Err(Exceptions::IllegalArgumentException(
                "input matrix dimensions do not match".to_owned(),
            ));
        }
        let rowArray = BitArray::with_size(self.width as usize);
        for y in 0..self.height {
            //for (int y = 0; y < height; y++) {
            let offset = y as usize * self.rowSize;
            let tmp = mask.getRow(y, &rowArray);
            let row = tmp.getBitArray();
            for x in 0..self.rowSize {
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
        let max = self.bits.len();
        for i in 0..max {
            //for (int i = 0; i < max; i++) {
            self.bits[i] = 0;
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
    pub fn setRegion(
        &mut self,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
    ) -> Result<(), Exceptions> {
        if top < 0 || left < 0 {
            return Err(Exceptions::IllegalArgumentException(
                "Left and top must be nonnegative".to_owned(),
            ));
        }
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
            let offset = y as usize * self.rowSize;
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
    pub fn getRow(&self, y: u32, row: &BitArray) -> BitArray {
        let mut rw: BitArray = if row.getSize() < self.width as usize {
            BitArray::with_size(self.width as usize)
        } else {
            let mut z = row.clone();
            z.clear();
            z
            // row.clear();
            // row.clone()
        };

        let offset = y as usize * self.rowSize;
        for x in 0..self.rowSize {
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
        return self.bits[y as usize * self.rowSize..y as usize * self.rowSize + self.rowSize]
            .clone_from_slice(&row.getBitArray()[0..self.rowSize]);
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
        let mut topRow = BitArray::with_size(self.width as usize);
        let mut bottomRow = BitArray::with_size(self.width as usize);
        let mut maxHeight = (self.height + 1) / 2;
        for i in 0..maxHeight {
            //for (int i = 0; i < maxHeight; i++) {
            topRow = self.getRow(i, &topRow);
            let bottomRowIndex = self.height - 1 - i;
            bottomRow = self.getRow(bottomRowIndex, &bottomRow);
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
        let mut newWidth = self.height;
        let mut newHeight = self.width;
        let mut newRowSize = (newWidth + 31) / 32;
        let mut newBits = vec![0; (newRowSize * newHeight).try_into().unwrap()];

        for y in 0..self.height {
            //for (int y = 0; y < height; y++) {
            for x in 0..self.width {
                //for (int x = 0; x < width; x++) {
                let offset = y as usize * self.rowSize + (x as usize / 32);
                if ((self.bits[offset] >> (x & 0x1f)) & 1) != 0 {
                    let newOffset: usize = ((newHeight - 1 - x) * newRowSize + (y / 32))
                        .try_into()
                        .unwrap();
                    newBits[newOffset] |= 1 << (y & 0x1f);
                }
            }
        }
        self.width = newWidth;
        self.height = newHeight;
        self.rowSize = newRowSize.try_into().unwrap();
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
            for x32 in 0..self.rowSize {
                //for (int x32 = 0; x32 < rowSize; x32++) {
                let theBits = self.bits[y as usize * self.rowSize + x32];
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
        let y = bitsOffset / self.rowSize;
        let mut x = (bitsOffset % self.rowSize) * 32;

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

        let y = bitsOffset as usize / self.rowSize;
        let mut x = (bitsOffset as usize % self.rowSize) * 32;

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
        return self.rowSize;
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
}

impl fmt::Display for BitMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.toString("X ", "  "))
    }
}

/*
 * Copyright 2021 ZXing authors
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

/**
 * Interface to navigate a sequence of ECIs and bytes.
 *
 * @author Alex Geller
 */
pub trait ECIInput {
    /**
     * Returns the length of this input.  The length is the number
     * of {@code byte}s in or ECIs in the sequence.
     *
     * @return  the number of {@code char}s in this sequence
     */
    fn length() -> usize;

    /**
     * Returns the {@code byte} value at the specified index.  An index ranges from zero
     * to {@code length() - 1}.  The first {@code byte} value of the sequence is at
     * index zero, the next at index one, and so on, as for array
     * indexing.
     *
     * @param   index the index of the {@code byte} value to be returned
     *
     * @return  the specified {@code byte} value as character or the FNC1 character
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     * @throws  IllegalArgumentException
     *          if the value at the {@code index} argument is an ECI (@see #isECI)
     */
    fn charAt(index: usize) -> char;

    /**
     * Returns a {@code CharSequence} that is a subsequence of this sequence.
     * The subsequence starts with the {@code char} value at the specified index and
     * ends with the {@code char} value at index {@code end - 1}.  The length
     * (in {@code char}s) of the
     * returned sequence is {@code end - start}, so if {@code start == end}
     * then an empty sequence is returned.
     *
     * @param   start   the start index, inclusive
     * @param   end     the end index, exclusive
     *
     * @return  the specified subsequence
     *
     * @throws  IndexOutOfBoundsException
     *          if {@code start} or {@code end} are negative,
     *          if {@code end} is greater than {@code length()},
     *          or if {@code start} is greater than {@code end}
     * @throws  IllegalArgumentException
     *          if a value in the range {@code start}-{@code end} is an ECI (@see #isECI)
     */
    fn subSequence(start: usize, end: usize) -> Vec<char>;

    /**
     * Determines if a value is an ECI
     *
     * @param   index the index of the value
     *
     * @return  true if the value at position {@code index} is an ECI
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     */
    fn isECI(index: u32) -> bool;

    /**
     * Returns the {@code int} ECI value at the specified index.  An index ranges from zero
     * to {@code length() - 1}.  The first {@code byte} value of the sequence is at
     * index zero, the next at index one, and so on, as for array
     * indexing.
     *
     * @param   index the index of the {@code int} value to be returned
     *
     * @return  the specified {@code int} ECI value.
     *          The ECI specified the encoding of all bytes with a higher index until the
     *          next ECI or until the end of the input if no other ECI follows.
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     * @throws  IllegalArgumentException
     *          if the value at the {@code index} argument is not an ECI (@see #isECI)
     */
    fn getECIValue(index: usize) -> u32;
    fn haveNCharacters(index: usize, n: usize) -> bool;
}

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

        let mut result = 0;

        let mut num_bits = numBits;

        // First, read remainder from current byte
        if self.bit_offset > 0 {
            let bitsLeft = 8 - self.bit_offset;
            let toRead = cmp::min(num_bits, bitsLeft);
            let bitsToNotRead = bitsLeft - toRead;
            let mask = (0xFF >> (8 - toRead)) << bitsToNotRead;
            result = (self.bytes[self.byte_offset] & mask) >> bitsToNotRead;
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
                result = ((result as u16) << 8) as u8 | (self.bytes[self.byte_offset] & 0xFF);
                self.byte_offset += 1;
                num_bits -= 8;
            }

            // Finally read a partial byte
            if num_bits > 0 {
                let bits_to_not_read = 8 - num_bits;
                let mask = (0xFF >> bits_to_not_read) << bits_to_not_read;
                result = (result << num_bits)
                    | ((self.bytes[self.byte_offset] & mask) >> bits_to_not_read);
                self.bit_offset += num_bits;
            }
        }

        return Ok(result.into());
    }

    /**
     * @return number of bits that can be read successfully
     */
    pub fn available(&self) -> usize {
        return 8 * (self.bytes.len() - self.byte_offset) - self.bit_offset;
    }
}

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

/**
 * <p>This class implements a perspective transform in two dimensions. Given four source and four
 * destination points, it will compute the transformation implied between them. The code is based
 * directly upon section 3.4.2 of George Wolberg's "Digital Image Warping"; see pages 54-56.</p>
 *
 * @author Sean Owen
 */
pub struct PerspectiveTransform {
    a11: f32,
    a12: f32,
    a13: f32,
    a21: f32,
    a22: f32,
    a23: f32,
    a31: f32,
    a32: f32,
    a33: f32,
}

impl PerspectiveTransform {
    fn new(
        a11: f32,
        a21: f32,
        a31: f32,
        a12: f32,
        a22: f32,
        a32: f32,
        a13: f32,
        a23: f32,
        a33: f32,
    ) -> Self {
        Self {
            a11,
            a12,
            a13,
            a21,
            a22,
            a23,
            a31,
            a32,
            a33,
        }
    }

    pub fn quadrilateralToQuadrilateral(
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x0p: f32,
        y0p: f32,
        x1p: f32,
        y1p: f32,
        x2p: f32,
        y2p: f32,
        x3p: f32,
        y3p: f32,
    ) -> Self {
        let qToS = PerspectiveTransform::quadrilateralToSquare(x0, y0, x1, y1, x2, y2, x3, y3);
        let sToQ =
            PerspectiveTransform::squareToQuadrilateral(x0p, y0p, x1p, y1p, x2p, y2p, x3p, y3p);
        return sToQ.times(&qToS);
    }

    pub fn transform_points_single(&self, points: &mut [f32]) {
        let a11 = self.a11;
        let a12 = self.a12;
        let a13 = self.a13;
        let a21 = self.a21;
        let a22 = self.a22;
        let a23 = self.a23;
        let a31 = self.a31;
        let a32 = self.a32;
        let a33 = self.a33;
        let maxI = points.len() - 1; // points.length must be even
        let mut i = 0;
        while i < maxI {
            // for (int i = 0; i < maxI; i += 2) {
            let x = points[i];
            let y = points[i + 1];
            let denominator = a13 * x + a23 * y + a33;
            points[i] = (a11 * x + a21 * y + a31) / denominator;
            points[i + 1] = (a12 * x + a22 * y + a32) / denominator;
            i += 2;
        }
    }

    pub fn transform_points_double(&self, x_values: &mut [f32], y_valuess: &mut [f32]) {
        let n = x_values.len();
        for i in 0..n {
            // for (int i = 0; i < n; i++) {
            let x = x_values[i];
            let y = y_valuess[i];
            let denominator = self.a13 * x + self.a23 * y + self.a33;
            x_values[i] = (self.a11 * x + self.a21 * y + self.a31) / denominator;
            y_valuess[i] = (self.a12 * x + self.a22 * y + self.a32) / denominator;
        }
    }

    pub fn squareToQuadrilateral(
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> Self {
        let dx3 = x0 - x1 + x2 - x3;
        let dy3 = y0 - y1 + y2 - y3;
        if dx3 == 0.0f32 && dy3 == 0.0f32 {
            // Affine
            return PerspectiveTransform::new(
                x1 - x0,
                x2 - x1,
                x0,
                y1 - y0,
                y2 - y1,
                y0,
                0.0f32,
                0.0f32,
                1.0f32,
            );
        } else {
            let dx1 = x1 - x2;
            let dx2 = x3 - x2;
            let dy1 = y1 - y2;
            let dy2 = y3 - y2;
            let denominator = dx1 * dy2 - dx2 * dy1;
            let a13 = (dx3 * dy2 - dx2 * dy3) / denominator;
            let a23 = (dx1 * dy3 - dx3 * dy1) / denominator;
            return PerspectiveTransform::new(
                x1 - x0 + a13 * x1,
                x3 - x0 + a23 * x3,
                x0,
                y1 - y0 + a13 * y1,
                y3 - y0 + a23 * y3,
                y0,
                a13,
                a23,
                1.0f32,
            );
        }
    }

    pub fn quadrilateralToSquare(
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> Self {
        // Here, the adjoint serves as the inverse
        return PerspectiveTransform::squareToQuadrilateral(x0, y0, x1, y1, x2, y2, x3, y3)
            .buildAdjoint();
    }

    fn buildAdjoint(&self) -> Self {
        // Adjoint is the transpose of the cofactor matrix:
        return PerspectiveTransform::new(
            self.a22 * self.a33 - self.a23 * self.a32,
            self.a23 * self.a31 - self.a21 * self.a33,
            self.a21 * self.a32 - self.a22 * self.a31,
            self.a13 * self.a32 - self.a12 * self.a33,
            self.a11 * self.a33 - self.a13 * self.a31,
            self.a12 * self.a31 - self.a11 * self.a32,
            self.a12 * self.a23 - self.a13 * self.a22,
            self.a13 * self.a21 - self.a11 * self.a23,
            self.a11 * self.a22 - self.a12 * self.a21,
        );
    }

    fn times(&self, other: &Self) -> Self {
        return PerspectiveTransform::new(
            self.a11 * other.a11 + self.a21 * other.a12 + self.a31 * other.a13,
            self.a11 * other.a21 + self.a21 * other.a22 + self.a31 * other.a23,
            self.a11 * other.a31 + self.a21 * other.a32 + self.a31 * other.a33,
            self.a12 * other.a11 + self.a22 * other.a12 + self.a32 * other.a13,
            self.a12 * other.a21 + self.a22 * other.a22 + self.a32 * other.a23,
            self.a12 * other.a31 + self.a22 * other.a32 + self.a32 * other.a33,
            self.a13 * other.a11 + self.a23 * other.a12 + self.a33 * other.a13,
            self.a13 * other.a21 + self.a23 * other.a22 + self.a33 * other.a23,
            self.a13 * other.a31 + self.a23 * other.a32 + self.a33 * other.a33,
        );
    }
}

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

// import java.util.List;

/**
 * <p>Encapsulates the result of decoding a matrix of bits. This typically
 * applies to 2D barcode formats. For now it contains the raw bytes obtained,
 * as well as a String interpretation of those bytes, if applicable.</p>
 *
 * @author Sean Owen
 */
pub struct DecoderRXingResult {
    rawBytes: Vec<u8>,
    numBits: usize,
    text: String,
    byteSegments: Vec<u8>,
    ecLevel: String,
    errorsCorrected: u64,
    erasures: u64,
    other: Box<dyn Any>,
    structuredAppendParity: i32,
    structuredAppendSequenceNumber: i32,
    symbologyModifier: u32,
}

impl DecoderRXingResult {
    pub fn new(rawBytes: Vec<u8>, text: String, byteSegments: Vec<u8>, ecLevel: String) -> Self {
        Self::with_all(rawBytes, text, byteSegments, ecLevel, -2, -2, 0)
    }

    pub fn with_symbology(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<u8>,
        ecLevel: String,
        symbologyModifier: u32,
    ) -> Self {
        Self::with_all(
            rawBytes,
            text,
            byteSegments,
            ecLevel,
            -1,
            -1,
            symbologyModifier,
        )
    }

    pub fn with_sa(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<u8>,
        ecLevel: String,
        saSequence: i32,
        saParity: i32,
    ) -> Self {
        Self::with_all(
            rawBytes,
            text,
            byteSegments,
            ecLevel,
            saSequence,
            saParity,
            0,
        )
    }

    pub fn with_all(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<u8>,
        ecLevel: String,
        saSequence: i32,
        saParity: i32,
        symbologyModifier: u32,
    ) -> Self {
        let nb = rawBytes.len();
        Self {
            rawBytes,
            numBits: nb,
            text,
            byteSegments,
            ecLevel,
            errorsCorrected: 0,
            erasures: 0,
            other: Box::new(false),
            structuredAppendParity: saParity,
            structuredAppendSequenceNumber: saSequence,
            symbologyModifier,
        }
    }

    /**
     * @return raw bytes representing the result, or {@code null} if not applicable
     */
    pub fn getRawBytes(&self) -> &Vec<u8> {
        &self.rawBytes
    }

    /**
     * @return how many bits of {@link #getRawBytes()} are valid; typically 8 times its length
     * @since 3.3.0
     */
    pub fn getNumBits(&self) -> usize {
        self.numBits
    }

    /**
     * @param numBits overrides the number of bits that are valid in {@link #getRawBytes()}
     * @since 3.3.0
     */
    pub fn setNumBits(&mut self, numBits: usize) {
        self.numBits = numBits;
    }

    /**
     * @return text representation of the result
     */
    pub fn getText(&self) -> &str {
        &self.text
    }

    /**
     * @return list of byte segments in the result, or {@code null} if not applicable
     */
    pub fn getByteSegments(&self) -> &Vec<u8> {
        &self.byteSegments
    }

    /**
     * @return name of error correction level used, or {@code null} if not applicable
     */
    pub fn getECLevel(&self) -> &str {
        &self.ecLevel
    }

    /**
     * @return number of errors corrected, or {@code null} if not applicable
     */
    pub fn getErrorsCorrected(&self) -> u64 {
        self.errorsCorrected
    }

    pub fn setErrorsCorrected(&mut self, errorsCorrected: u64) {
        self.errorsCorrected = errorsCorrected;
    }

    /**
     * @return number of erasures corrected, or {@code null} if not applicable
     */
    pub fn getErasures(&self) -> u64 {
        self.erasures
    }

    pub fn setErasures(&mut self, erasures: u64) {
        self.erasures = erasures
    }

    /**
     * @return arbitrary additional metadata
     */
    pub fn getOther(&self) -> &Box<dyn Any> {
        &self.other
    }

    pub fn setOther(&mut self, other: Box<dyn Any>) {
        self.other = other
    }

    pub fn hasStructuredAppend(&self) -> bool {
        self.structuredAppendParity >= 0 && self.structuredAppendSequenceNumber >= 0
    }

    pub fn getStructuredAppendParity(&self) -> i32 {
        self.structuredAppendParity
    }

    pub fn getStructuredAppendSequenceNumber(&self) -> i32 {
        self.structuredAppendSequenceNumber
    }

    pub fn getSymbologyModifier(&self) -> u32 {
        self.symbologyModifier
    }
}

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

// package com.google.zxing.common;

// import java.io.ByteArrayOutputStream;

/**
 * Class that lets one easily build an array of bytes by appending bits at a time.
 *
 * @author Sean Owen
 */
pub struct BitSourceBuilder {
    output: Vec<u8>,
    nextByte: u32,
    bitsLeftInNextByte: u32,
}

impl BitSourceBuilder {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            nextByte: 0,
            bitsLeftInNextByte: 8,
        }
    }

    pub fn write(&mut self, value: u32, numBits: u32) {
        if numBits <= self.bitsLeftInNextByte {
            self.nextByte <<= numBits;
            self.nextByte |= value;
            self.bitsLeftInNextByte -= numBits;
            if self.bitsLeftInNextByte == 0 {
                self.output.push(self.nextByte as u8);
                self.nextByte = 0;
                self.bitsLeftInNextByte = 8;
            }
        } else {
            let bitsToWriteNow = self.bitsLeftInNextByte;
            let numRestOfBits = numBits - bitsToWriteNow;
            let mask = 0xFF >> (8 - bitsToWriteNow);
            let valueToWriteNow = (value >> numRestOfBits) & mask;
            self.write(valueToWriteNow, bitsToWriteNow);
            self.write(value, numRestOfBits);
        }
    }

    pub fn toByteArray(&mut self) -> &Vec<u8> {
        if self.bitsLeftInNextByte < 8 {
            self.write(0, self.bitsLeftInNextByte);
        }
        &self.output
    }
}

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

// import com.google.zxing.NotFoundException;

/**
 * Implementations of this class can, given locations of finder patterns for a QR code in an
 * image, sample the right points in the image to reconstruct the QR code, accounting for
 * perspective distortion. It is abstracted since it is relatively expensive and should be allowed
 * to take advantage of platform-specific optimized implementations, like Sun's Java Advanced
 * Imaging library, but which may not be available in other environments such as J2ME, and vice
 * versa.
 *
 * The implementation used can be controlled by calling {@link #setGridSampler(GridSampler)}
 * with an instance of a class which implements this interface.
 *
 * @author Sean Owen
 */

pub trait GridSampler {
    //   /**
    //    * Sets the implementation of GridSampler used by the library. One global
    //    * instance is stored, which may sound problematic. But, the implementation provided
    //    * ought to be appropriate for the entire platform, and all uses of this library
    //    * in the whole lifetime of the JVM. For instance, an Android activity can swap in
    //    * an implementation that takes advantage of native platform libraries.
    //    *
    //    * @param newGridSampler The platform-specific object to install.
    //    */
    //   public static void setGridSampler(GridSampler newGridSampler) {
    //     gridSampler = newGridSampler;
    //   }

    //   /**
    //    * @return the current implementation of GridSampler
    //    */
    //   public static GridSampler getInstance() {
    //     return gridSampler;
    //   }

    /**
     * Samples an image for a rectangular matrix of bits of the given dimension. The sampling
     * transformation is determined by the coordinates of 4 points, in the original and transformed
     * image space.
     *
     * @param image image to sample
     * @param dimensionX width of {@link BitMatrix} to sample from image
     * @param dimensionY height of {@link BitMatrix} to sample from image
     * @param p1ToX point 1 preimage X
     * @param p1ToY point 1 preimage Y
     * @param p2ToX point 2 preimage X
     * @param p2ToY point 2 preimage Y
     * @param p3ToX point 3 preimage X
     * @param p3ToY point 3 preimage Y
     * @param p4ToX point 4 preimage X
     * @param p4ToY point 4 preimage Y
     * @param p1FromX point 1 image X
     * @param p1FromY point 1 image Y
     * @param p2FromX point 2 image X
     * @param p2FromY point 2 image Y
     * @param p3FromX point 3 image X
     * @param p3FromY point 3 image Y
     * @param p4FromX point 4 image X
     * @param p4FromY point 4 image Y
     * @return {@link BitMatrix} representing a grid of points sampled from the image within a region
     *   defined by the "from" parameters
     * @throws NotFoundException if image can't be sampled, for example, if the transformation defined
     *   by the given points is invalid or results in sampling outside the image boundaries
     */
    fn sample_grid_detailed(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        p1ToX: f32,
        p1ToY: f32,
        p2ToX: f32,
        p2ToY: f32,
        p3ToX: f32,
        p3ToY: f32,
        p4ToX: f32,
        p4ToY: f32,
        p1FromX: f32,
        p1FromY: f32,
        p2FromX: f32,
        p2FromY: f32,
        p3FromX: f32,
        p3FromY: f32,
        p4FromX: f32,
        p4FromY: f32,
    ) -> Result<BitMatrix, Exceptions>;

    fn sample_grid(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        transform: &PerspectiveTransform,
    ) -> Result<BitMatrix, Exceptions>;

    /**
     * <p>Checks a set of points that have been transformed to sample points on an image against
     * the image's dimensions to see if the point are even within the image.</p>
     *
     * <p>This method will actually "nudge" the endpoints back onto the image if they are found to be
     * barely (less than 1 pixel) off the image. This accounts for imperfect detection of finder
     * patterns in an image where the QR Code runs all the way to the image border.</p>
     *
     * <p>For efficiency, the method will check points from either end of the line until one is found
     * to be within the image. Because the set of points are assumed to be linear, this is valid.</p>
     *
     * @param image image into which the points should map
     * @param points actual points in x1,y1,...,xn,yn form
     * @throws NotFoundException if an endpoint is lies outside the image boundaries
     */
    fn checkAndNudgePoints(&self, image: &BitMatrix, points: &mut [f32]) -> Result<(), Exceptions> {
        let width = image.getWidth();
        let height = image.getHeight();
        // Check and nudge points from start until we see some that are OK:
        let mut nudged = true;
        let max_offset = points.len() - 1; // points.length must be even
        let mut offset = 0;
        while offset < max_offset && nudged {
            // for (int offset = 0; offset < maxOffset && nudged; offset += 2) {
            let x = points[offset] as i32;
            let y = points[offset + 1] as i32;
            if x < -1 || x > width.try_into().unwrap() || y < -1 || y > height.try_into().unwrap() {
                return Err(Exceptions::NotFoundException(
                    "getNotFoundInstance".to_owned(),
                ));
            }
            nudged = false;
            if x == -1 {
                points[offset] = 0.0f32;
                nudged = true;
            } else if x == width.try_into().unwrap() {
                points[offset] = width as f32 - 1f32;
                nudged = true;
            }
            if y == -1 {
                points[offset + 1] = 0.0f32;
                nudged = true;
            } else if (y == height.try_into().unwrap()) {
                points[offset + 1] = height as f32 - 1f32;
                nudged = true;
            }
            offset += 2;
        }
        // Check and nudge points from end:
        nudged = true;
        let mut offset = points.len() - 2;
        while offset >= 0 && nudged {
            // for (int offset = points.length - 2; offset >= 0 && nudged; offset -= 2) {
            let x = points[offset] as i32;
            let y = points[offset + 1] as i32;
            if x < -1 || x > width.try_into().unwrap() || y < -1 || y > height.try_into().unwrap() {
                return Err(Exceptions::NotFoundException(
                    "getNotFoundInstance".to_owned(),
                ));
            }
            nudged = false;
            if x == -1 {
                points[offset] = 0.0f32;
                nudged = true;
            } else if (x == width.try_into().unwrap()) {
                points[offset] = width as f32 - 1f32;
                nudged = true;
            }
            if y == -1 {
                points[offset + 1] = 0.0f32;
                nudged = true;
            } else if (y == height.try_into().unwrap()) {
                points[offset + 1] = height as f32 - 1f32;
                nudged = true;
            }
            offset += 2;
        }
        Ok(())
    }
}

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

// import com.google.zxing.NotFoundException;

/**
 * @author Sean Owen
 */
pub struct DefaultGridSampler {}

impl GridSampler for DefaultGridSampler {
    fn sample_grid_detailed(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        p1ToX: f32,
        p1ToY: f32,
        p2ToX: f32,
        p2ToY: f32,
        p3ToX: f32,
        p3ToY: f32,
        p4ToX: f32,
        p4ToY: f32,
        p1FromX: f32,
        p1FromY: f32,
        p2FromX: f32,
        p2FromY: f32,
        p3FromX: f32,
        p3FromY: f32,
        p4FromX: f32,
        p4FromY: f32,
    ) -> Result<BitMatrix, Exceptions> {
        let transform = PerspectiveTransform::quadrilateralToQuadrilateral(
            p1ToX, p1ToY, p2ToX, p2ToY, p3ToX, p3ToY, p4ToX, p4ToY, p1FromX, p1FromY, p2FromX,
            p2FromY, p3FromX, p3FromY, p4FromX, p4FromY,
        );

        self.sample_grid(image, dimensionX, dimensionY, &transform)
    }

    fn sample_grid(
        &self,
        image: &BitMatrix,
        dimensionX: u32,
        dimensionY: u32,
        transform: &PerspectiveTransform,
    ) -> Result<BitMatrix, Exceptions> {
        if dimensionX <= 0 || dimensionY <= 0 {
            return Err(Exceptions::NotFoundException(
                "getNotFoundInstance".to_owned(),
            ));
        }
        let mut bits = BitMatrix::new(dimensionX, dimensionY)?;
        let mut points = vec![0_f32; 2 * dimensionX as usize];
        for y in 0..dimensionY {
            //   for (int y = 0; y < dimensionY; y++) {
            let max = points.len();
            let iValue = y as f32 + 0.5f32;
            let mut x = 0;
            while x < max {
                // for (int x = 0; x < max; x += 2) {
                points[x] = (x as f32 / 2.0) + 0.5f32;
                points[x + 1] = iValue;
                x += 2;
            }
            transform.transform_points_single(&mut points);
            // Quick check to see if points transformed to something inside the image;
            // sufficient to check the endpoints
            self.checkAndNudgePoints(image, &mut points);
            // try {
            let mut x = 0;
            while x < max {
                //   for (int x = 0; x < max; x += 2) {
                if image.get(points[x] as u32, points[x + 1] as u32) {
                    // Black(-ish) pixel
                    bits.set(x as u32 / 2, y);
                    x += 2;
                }
            }
            // } catch (ArrayIndexOutOfBoundsException aioobe) {
            //   // This feels wrong, but, sometimes if the finder patterns are misidentified, the resulting
            //   // transform gets "twisted" such that it maps a straight line of points to a set of points
            //   // whose endpoints are in bounds, but others are not. There is probably some mathematical
            //   // way to detect this about the transformation that I don't know yet.
            //   // This results in an ugly runtime exception despite our clever checks above -- can't have
            //   // that. We could check each point's coordinates but that feels duplicative. We settle for
            //   // catching and wrapping ArrayIndexOutOfBoundsException.
            //   throw NotFoundException.getNotFoundInstance();
            // }
        }
        return Ok(bits);
    }
}

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

// package com.google.zxing.common;

// import com.google.zxing.FormatException;

// import java.nio.charset.Charset;

// import java.util.HashMap;
// import java.util.Map;

/**
 * Encapsulates a Character Set ECI, according to "Extended Channel Interpretations" 5.3.1.1
 * of ISO 18004.
 *
 * @author Sean Owen
 */
pub enum CharacterSetECI {
    // Enum name is a Java encoding valid for java.lang and java.io
    Cp437,              //(new int[]{0,2}),
    ISO8859_1,          //(new int[]{1,3}, "ISO-8859-1"),
    ISO8859_2,          //(4, "ISO-8859-2"),
    ISO8859_3,          //(5, "ISO-8859-3"),
    ISO8859_4,          //(6, "ISO-8859-4"),
    ISO8859_5,          //(7, "ISO-8859-5"),
    ISO8859_6,          //(8, "ISO-8859-6"),
    ISO8859_7,          //(9, "ISO-8859-7"),
    ISO8859_8,          //(10, "ISO-8859-8"),
    ISO8859_9,          //(11, "ISO-8859-9"),
    ISO8859_10,         //(12, "ISO-8859-10"),
    ISO8859_11,         //(13, "ISO-8859-11"),
    ISO8859_13,         //(15, "ISO-8859-13"),
    ISO8859_14,         //(16, "ISO-8859-14"),
    ISO8859_15,         //(17, "ISO-8859-15"),
    ISO8859_16,         //(18, "ISO-8859-16"),
    SJIS,               //(20, "Shift_JIS"),
    Cp1250,             //(21, "windows-1250"),
    Cp1251,             //(22, "windows-1251"),
    Cp1252,             //(23, "windows-1252"),
    Cp1256,             //(24, "windows-1256"),
    UnicodeBigUnmarked, //(25, "UTF-16BE", "UnicodeBig"),
    UTF8,               //(26, "UTF-8"),
    ASCII,              //(new int[] {27, 170}, "US-ASCII"),
    Big5,               //(28),
    GB18030,            //(29, "GB2312", "EUC_CN", "GBK"),
    EUC_KR,             //(30, "EUC-KR");
}
impl CharacterSetECI {
    //   private static final Map<Integer,CharacterSetECI> VALUE_TO_ECI = new HashMap<>();
    //   private static final Map<String,CharacterSetECI> NAME_TO_ECI = new HashMap<>();
    //   static {
    //     for (CharacterSetECI eci : values()) {
    //       for (int value : eci.values) {
    //         VALUE_TO_ECI.put(value, eci);
    //       }
    //       NAME_TO_ECI.put(eci.name(), eci);
    //       for (String name : eci.otherEncodingNames) {
    //         NAME_TO_ECI.put(name, eci);
    //       }
    //     }
    //   }

    //   private final int[] values;
    //   private final String[] otherEncodingNames;

    //   CharacterSetECI(int value) {
    //     this(new int[] {value});
    //   }

    //   CharacterSetECI(int value, String... otherEncodingNames) {
    //     this.values = new int[] {value};
    //     this.otherEncodingNames = otherEncodingNames;
    //   }

    //   CharacterSetECI(int[] values, String... otherEncodingNames) {
    //     this.values = values;
    //     this.otherEncodingNames = otherEncodingNames;
    //   }

    pub fn getValue(cs_eci: &CharacterSetECI) -> u32 {
        match cs_eci {
            CharacterSetECI::Cp437 => 0,
            CharacterSetECI::ISO8859_1 => 1,
            CharacterSetECI::ISO8859_2 => 4,
            CharacterSetECI::ISO8859_3 => 5,
            CharacterSetECI::ISO8859_4 => 6,
            CharacterSetECI::ISO8859_5 => 7,
            CharacterSetECI::ISO8859_6 => 8,
            CharacterSetECI::ISO8859_7 => 9,
            CharacterSetECI::ISO8859_8 => 10,
            CharacterSetECI::ISO8859_9 => 11,
            CharacterSetECI::ISO8859_10 => 12,
            CharacterSetECI::ISO8859_11 => 13,
            CharacterSetECI::ISO8859_13 => 15,
            CharacterSetECI::ISO8859_14 => 16,
            CharacterSetECI::ISO8859_15 => 17,
            CharacterSetECI::ISO8859_16 => 18,
            CharacterSetECI::SJIS => 20,
            CharacterSetECI::Cp1250 => 21,
            CharacterSetECI::Cp1251 => 22,
            CharacterSetECI::Cp1252 => 23,
            CharacterSetECI::Cp1256 => 24,
            CharacterSetECI::UnicodeBigUnmarked => 25,
            CharacterSetECI::UTF8 => 26,
            CharacterSetECI::ASCII => 27,
            CharacterSetECI::Big5 => 28,
            CharacterSetECI::GB18030 => 29,
            CharacterSetECI::EUC_KR => 30,
        }
    }

    pub fn getCharset(cs_eci: &CharacterSetECI) -> &'static dyn Encoding {
        let name = match cs_eci {
            CharacterSetECI::Cp437 => "CP437",
            CharacterSetECI::ISO8859_1 => "ISO-8859-1",
            CharacterSetECI::ISO8859_2 => "ISO-8859-2",
            CharacterSetECI::ISO8859_3 => "ISO-8859-3",
            CharacterSetECI::ISO8859_4 => "ISO-8859-4",
            CharacterSetECI::ISO8859_5 => "ISO-8859-5",
            CharacterSetECI::ISO8859_6 => "ISO-8859-6",
            CharacterSetECI::ISO8859_7 => "ISO-8859-7",
            CharacterSetECI::ISO8859_8 => "ISO-8859-8",
            CharacterSetECI::ISO8859_9 => "ISO-8859-9",
            CharacterSetECI::ISO8859_10 => "ISO-8859-10",
            CharacterSetECI::ISO8859_11 => "ISO-8859-11",
            CharacterSetECI::ISO8859_13 => "ISO-8859-13",
            CharacterSetECI::ISO8859_14 => "ISO-8859-14",
            CharacterSetECI::ISO8859_15 => "ISO-8859-15",
            CharacterSetECI::ISO8859_16 => "ISO-8859-16",
            CharacterSetECI::SJIS => "Shift_JIS",
            CharacterSetECI::Cp1250 => "windows-1250",
            CharacterSetECI::Cp1251 => "windows-1251",
            CharacterSetECI::Cp1252 => "windows-1252",
            CharacterSetECI::Cp1256 => "windows-1256",
            CharacterSetECI::UnicodeBigUnmarked => "UTF-16BE",
            CharacterSetECI::UTF8 => "UTF-8",
            CharacterSetECI::ASCII => "US-ASCII",
            CharacterSetECI::Big5 => "Big5",
            CharacterSetECI::GB18030 => "GB2312",
            CharacterSetECI::EUC_KR => "EUC-KR",
        };
        encoding::label::encoding_from_whatwg_label(name).unwrap()
    }

    /**
     * @param charset Java character set object
     * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
     *   but unsupported
     */
    pub fn getCharacterSetECI(charset: &'static dyn Encoding) -> Option<CharacterSetECI> {
        match charset.whatwg_name().unwrap() {
            "CP437" => Some(CharacterSetECI::Cp437),
            "ISO-8859-1" => Some(CharacterSetECI::ISO8859_1),
            "ISO-8859-2" => Some(CharacterSetECI::ISO8859_2),
            "ISO-8859-3" => Some(CharacterSetECI::ISO8859_3),
            "ISO-8859-4" => Some(CharacterSetECI::ISO8859_4),
            "ISO-8859-5" => Some(CharacterSetECI::ISO8859_5),
            "ISO-8859-6" => Some(CharacterSetECI::ISO8859_6),
            "ISO-8859-7" => Some(CharacterSetECI::ISO8859_7),
            "ISO-8859-8" => Some(CharacterSetECI::ISO8859_8),
            "ISO-8859-9" => Some(CharacterSetECI::ISO8859_9),
            "ISO-8859-10" => Some(CharacterSetECI::ISO8859_10),
            "ISO-8859-11" => Some(CharacterSetECI::ISO8859_11),
            "ISO-8859-13" => Some(CharacterSetECI::ISO8859_13),
            "ISO-8859-14" => Some(CharacterSetECI::ISO8859_14),
            "ISO-8859-15" => Some(CharacterSetECI::ISO8859_15),
            "ISO-8859-16" => Some(CharacterSetECI::ISO8859_16),
            "Shift_JIS" => Some(CharacterSetECI::SJIS),
            "windows-1250" => Some(CharacterSetECI::Cp1250),
            "windows-1251" => Some(CharacterSetECI::Cp1251),
            "windows-1252" => Some(CharacterSetECI::Cp1252),
            "windows-1256" => Some(CharacterSetECI::Cp1256),
            "UTF-16BE" => Some(CharacterSetECI::UnicodeBigUnmarked),
            "UTF-8" => Some(CharacterSetECI::UTF8),
            "US-ASCII" => Some(CharacterSetECI::ASCII),
            "Big5" => Some(CharacterSetECI::Big5),
            "GB2312" => Some(CharacterSetECI::GB18030),
            "EUC-KR" => Some(CharacterSetECI::EUC_KR),
            _ => None,
        }
    }

    /**
     * @param value character set ECI value
     * @return {@code CharacterSetECI} representing ECI of given value, or null if it is legal but
     *   unsupported
     * @throws FormatException if ECI value is invalid
     */
    pub fn getCharacterSetECIByValue(value: u32) -> Result<CharacterSetECI, Exceptions> {
        match value {
            0 | 2 => Ok(CharacterSetECI::Cp437),
            1 | 3 => Ok(CharacterSetECI::ISO8859_1),
            4 => Ok(CharacterSetECI::ISO8859_2),
            5 => Ok(CharacterSetECI::ISO8859_3),
            6 => Ok(CharacterSetECI::ISO8859_4),
            7 => Ok(CharacterSetECI::ISO8859_5),
            8 => Ok(CharacterSetECI::ISO8859_6),
            9 => Ok(CharacterSetECI::ISO8859_7),
            10 => Ok(CharacterSetECI::ISO8859_8),
            11 => Ok(CharacterSetECI::ISO8859_9),
            12 => Ok(CharacterSetECI::ISO8859_10),
            13 => Ok(CharacterSetECI::ISO8859_11),
            15 => Ok(CharacterSetECI::ISO8859_13),
            16 => Ok(CharacterSetECI::ISO8859_14),
            17 => Ok(CharacterSetECI::ISO8859_15),
            18 => Ok(CharacterSetECI::ISO8859_16),
            20 => Ok(CharacterSetECI::SJIS),
            21 => Ok(CharacterSetECI::Cp1250),
            22 => Ok(CharacterSetECI::Cp1251),
            23 => Ok(CharacterSetECI::Cp1252),
            24 => Ok(CharacterSetECI::Cp1256),
            25 => Ok(CharacterSetECI::UnicodeBigUnmarked),
            26 => Ok(CharacterSetECI::UTF8),
            27 | 170 => Ok(CharacterSetECI::ASCII),
            28 => Ok(CharacterSetECI::Big5),
            29 => Ok(CharacterSetECI::GB18030),
            30 => Ok(CharacterSetECI::EUC_KR),
            _ => Err(Exceptions::NotFoundException("Bad ECI Value".to_owned())),
        }
    }

    /**
     * @param name character set ECI encoding name
     * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
     *   but unsupported
     */
    pub fn getCharacterSetECIByName(name: &str) -> Option<CharacterSetECI> {
        match name {
            "CP437" => Some(CharacterSetECI::Cp437),
            "ISO-8859-1" => Some(CharacterSetECI::ISO8859_1),
            "ISO-8859-2" => Some(CharacterSetECI::ISO8859_2),
            "ISO-8859-3" => Some(CharacterSetECI::ISO8859_3),
            "ISO-8859-4" => Some(CharacterSetECI::ISO8859_4),
            "ISO-8859-5" => Some(CharacterSetECI::ISO8859_5),
            "ISO-8859-6" => Some(CharacterSetECI::ISO8859_6),
            "ISO-8859-7" => Some(CharacterSetECI::ISO8859_7),
            "ISO-8859-8" => Some(CharacterSetECI::ISO8859_8),
            "ISO-8859-9" => Some(CharacterSetECI::ISO8859_9),
            "ISO-8859-10" => Some(CharacterSetECI::ISO8859_10),
            "ISO-8859-11" => Some(CharacterSetECI::ISO8859_11),
            "ISO-8859-13" => Some(CharacterSetECI::ISO8859_13),
            "ISO-8859-14" => Some(CharacterSetECI::ISO8859_14),
            "ISO-8859-15" => Some(CharacterSetECI::ISO8859_15),
            "ISO-8859-16" => Some(CharacterSetECI::ISO8859_16),
            "Shift_JIS" => Some(CharacterSetECI::SJIS),
            "windows-1250" => Some(CharacterSetECI::Cp1250),
            "windows-1251" => Some(CharacterSetECI::Cp1251),
            "windows-1252" => Some(CharacterSetECI::Cp1252),
            "windows-1256" => Some(CharacterSetECI::Cp1256),
            "UTF-16BE" => Some(CharacterSetECI::UnicodeBigUnmarked),
            "UTF-8" => Some(CharacterSetECI::UTF8),
            "US-ASCII" => Some(CharacterSetECI::ASCII),
            "Big5" => Some(CharacterSetECI::Big5),
            "GB2312" => Some(CharacterSetECI::GB18030),
            "EUC-KR" => Some(CharacterSetECI::EUC_KR),
            _ => None,
        }
    }
}

/*
 * Copyright 2022 ZXing authors
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

// import com.google.zxing.FormatException;

// import java.nio.charset.Charset;
// import java.nio.charset.StandardCharsets;

/**
 * Class that converts a sequence of ECIs and bytes into a string
 *
 * @author Alex Geller
 */
pub struct ECIStringBuilder {
    current_bytes: Vec<u8>,
    result: String,
    current_charset: &'static dyn Encoding, //= StandardCharsets.ISO_8859_1;
}

impl ECIStringBuilder {
    pub fn new() -> Self {
        Self {
            current_bytes: Vec::new(),
            result: String::new(),
            current_charset: encoding::all::UTF_8,
        }
    }
    pub fn with_capacity(initial_capacity: usize) -> Self {
        Self {
            current_bytes: Vec::with_capacity(initial_capacity),
            result: String::new(),
            current_charset: encoding::all::ISO_8859_1,
        }
    }

    /**
     * Appends {@code value} as a byte value
     *
     * @param value character whose lowest byte is to be appended
     */
    pub fn append_char(&mut self, value: char) {
        self.current_bytes.push(value as u8);
    }

    /**
     * Appends {@code value} as a byte value
     *
     * @param value byte to append
     */
    pub fn append_byte(&mut self, value: u8) {
        self.current_bytes.push(value);
    }

    /**
     * Appends the characters in {@code value} as bytes values
     *
     * @param value string to append
     */
    pub fn append_string(&mut self, value: &str) {
        value.as_bytes().iter().map(|b| self.current_bytes.push(*b));
        // self.current_bytes.push(value.as_bytes());
    }

    /**
     * Append the string repesentation of {@code value} (short for {@code append(String.valueOf(value))})
     *
     * @param value int to append as a string
     */
    pub fn append(&mut self, value: i32) {
        self.append_string(&format!("{}", value));
    }

    /**
     * Appends ECI value to output.
     *
     * @param value ECI value to append, as an int
     * @throws FormatException on invalid ECI value
     */
    pub fn appendECI(&mut self, value: u32) -> Result<(), Exceptions> {
        self.encodeCurrentBytesIfAny();
        let character_set_eci = CharacterSetECI::getCharacterSetECIByValue(value)?;
        // if (character_set_eci == null) {
        //   throw FormatException.getFormatInstance();
        // }
        self.current_charset = CharacterSetECI::getCharset(&character_set_eci);
        Ok(())
    }

    pub fn encodeCurrentBytesIfAny(&mut self) {
        if self.current_charset.name() == encoding::all::UTF_8.name() {
            if !self.current_bytes.is_empty() {
                // if result == null {
                //   result = currentBytes;
                //   currentBytes = new StringBuilder();
                // } else {
                self.result
                    .push_str(&String::from_utf8(self.current_bytes.clone()).unwrap());
                self.current_bytes.clear();
                // }
            }
        } else if !self.current_bytes.is_empty() {
            let bytes = self.current_bytes.clone();
            self.current_bytes.clear();
            //   if (result == null) {
            //     result = new StringBuilder(new String(bytes, currentCharset));
            //   } else {
            let encoded_value = self
                .current_charset
                .decode(&bytes, encoding::DecoderTrap::Replace)
                .unwrap();
            self.result.push_str(&encoded_value);
            //   }
        }
    }

    /**
     * Appends the characters from {@code value} (unlike all other append methods of this class who append bytes)
     *
     * @param value characters to append
     */
    pub fn appendCharacters(&mut self, value: &str) {
        self.encodeCurrentBytesIfAny();
        self.result.push_str(value);
    }

    /**
     * Short for {@code toString().length()} (if possible, use {@link #isEmpty()} instead)
     *
     * @return length of string representation in characters
     */
    pub fn len(&mut self) -> usize {
        self.encodeCurrentBytesIfAny(); //return toString().length();
        self.result.len()
    }

    /**
     * @return true iff nothing has been appended
     */
    pub fn is_empty(&self) -> bool {
        return self.current_bytes.is_empty() && self.result.is_empty();
    }
}

impl fmt::Display for ECIStringBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //self.encodeCurrentBytesIfAny();
        write!(f, "{}", self.result)
    }
}

/*
 * Copyright 2021 ZXing authors
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

// import java.nio.charset.Charset;
// import java.nio.charset.CharsetEncoder;
// import java.nio.charset.StandardCharsets;
// import java.nio.charset.UnsupportedCharsetException;
// import java.util.ArrayList;
// import java.util.List;

/**
 * Set of CharsetEncoders for a given input string
 *
 * Invariants:
 * - The list contains only encoders from CharacterSetECI (list is shorter then the list of encoders available on
 *   the platform for which ECI values are defined).
 * - The list contains encoders at least one encoder for every character in the input.
 * - The first encoder in the list is always the ISO-8859-1 encoder even of no character in the input can be encoded
 *       by it.
 * - If the input contains a character that is not in ISO-8859-1 then the last two entries in the list will be the
 *   UTF-8 encoder and the UTF-16BE encoder.
 *
 * @author Alex Geller
 */
pub struct ECIEncoderSet {
    encoders: Vec<&'static dyn encoding::Encoding>,
    priorityEncoderIndex: usize,
}

impl ECIEncoderSet {
    /**
     * Constructs an encoder set
     *
     * @param stringToEncode the string that needs to be encoded
     * @param priorityCharset The preferred {@link Charset} or null.
     * @param fnc1 fnc1 denotes the character in the input that represents the FNC1 character or -1 for a non-GS1 bar
     * code. When specified, it is considered an error to pass it as argument to the methods canEncode() or encode().
     */
    pub fn new(
        stringToEncode: &str,
        priorityCharset: &'static dyn encoding::Encoding,
        fnc1: char,
    ) -> Self {
        // List of encoders that potentially encode characters not in ISO-8859-1 in one byte.
        let mut ENCODERS = Vec::new();

        let names = [
            "IBM437",
            "ISO-8859-2",
            "ISO-8859-3",
            "ISO-8859-4",
            "ISO-8859-5",
            "ISO-8859-6",
            "ISO-8859-7",
            "ISO-8859-8",
            "ISO-8859-9",
            "ISO-8859-10",
            "ISO-8859-11",
            "ISO-8859-13",
            "ISO-8859-14",
            "ISO-8859-15",
            "ISO-8859-16",
            "windows-1250",
            "windows-1251",
            "windows-1252",
            "windows-1256",
            "Shift_JIS",
        ];
        for name in names {
            if let Some(enc) = CharacterSetECI::getCharacterSetECIByName(name) {
                // try {
                ENCODERS.push(CharacterSetECI::getCharset(&enc));
                // } catch (UnsupportedCharsetException e) {
                // continue
                // }
            }
        }

        let mut encoders: Vec<&'static dyn Encoding>;
        let mut priorityEncoderIndexValue = 0;

        let mut neededEncoders: Vec<&'static dyn encoding::Encoding> = Vec::new();

        //we always need the ISO-8859-1 encoder. It is the default encoding
        neededEncoders.push(encoding::all::ISO_8859_1);
        neededEncoders.push(encoding::all::UTF_8);
        let mut needUnicodeEncoder = priorityCharset.name().starts_with("UTF");

        //Walk over the input string and see if all characters can be encoded with the list of encoders
        for i in 0..stringToEncode.len() {
            // for (int i = 0; i < stringToEncode.length(); i++) {
            let mut canEncode = false;
            for encoder in &neededEncoders {
                //   for (CharsetEncoder encoder : neededEncoders) {
                let c = stringToEncode.chars().nth(i).unwrap();
                if c == fnc1
                    || encoder
                        .encode(&c.to_string(), encoding::EncoderTrap::Strict)
                        .is_ok()
                {
                    canEncode = true;
                    break;
                }
            }
            if !canEncode {
                //for the character at position i we don't yet have an encoder in the list
                for encoder in &ENCODERS {
                    // for (CharsetEncoder encoder : ENCODERS) {
                    if encoder
                        .encode(
                            &stringToEncode.chars().nth(i).unwrap().to_string(),
                            encoding::EncoderTrap::Strict,
                        )
                        .is_ok()
                    {
                        //Good, we found an encoder that can encode the character. We add him to the list and continue scanning
                        //the input
                        neededEncoders.push(*encoder);
                        canEncode = true;
                        break;
                    }
                }
            }

            if !canEncode {
                //The character is not encodeable by any of the single byte encoders so we remember that we will need a
                //Unicode encoder.
                needUnicodeEncoder = true;
            }
        }

        if neededEncoders.len() == 1 && !needUnicodeEncoder {
            //the entire input can be encoded by the ISO-8859-1 encoder
            encoders = vec![encoding::all::ISO_8859_1];
        } else {
            // we need more than one single byte encoder or we need a Unicode encoder.
            // In this case we append a UTF-8 and UTF-16 encoder to the list
            //   encoders = [] new CharsetEncoder[neededEncoders.size() + 2];
            encoders = Vec::new();
            let index = 0;

            encoders.push(encoding::all::UTF_8);
            encoders.push(encoding::all::UTF_16BE);

            for encoder in neededEncoders {
                //   for (CharsetEncoder encoder : neededEncoders) {
                //encoders[index++] = encoder;
                encoders.push(encoder);
            }
        }

        //Compute priorityEncoderIndex by looking up priorityCharset in encoders
        // if priorityCharset != null {
        for i in 0..encoders.len() {
            //   for (int i = 0; i < encoders.length; i++) {
            if priorityCharset.name() == encoders[i].name() {
                priorityEncoderIndexValue = i;
                break;
            }
        }
        // }
        //invariants
        assert_eq!(encoders[0].name(), encoding::all::ISO_8859_1.name());
        Self {
            encoders: encoders,
            priorityEncoderIndex: priorityEncoderIndexValue,
        }
    }

    pub fn len(&self) -> usize {
        return self.encoders.len();
    }

    pub fn getCharsetName(&self, index: usize) -> &'static str {
        assert!(index < self.len());
        return self.encoders[index].name();
    }

    pub fn getCharset(&self, index: usize) -> &'static dyn Encoding {
        assert!(index < self.len());
        return self.encoders[index];
    }

    pub fn getECIValue(&self, encoderIndex: usize) -> u32 {
        CharacterSetECI::getValue(
            &CharacterSetECI::getCharacterSetECI(self.encoders[encoderIndex]).unwrap(),
        )
    }

    /*
     *  returns -1 if no priority charset was defined
     */
    pub fn getPriorityEncoderIndex(&self) -> usize {
        return self.priorityEncoderIndex;
    }

    pub fn canEncode(&self, c: char, encoderIndex: usize) -> bool {
        assert!(encoderIndex < self.len());
        let encoder = self.encoders[encoderIndex];
        let enc_data = encoder.encode(&c.to_string(), encoding::EncoderTrap::Strict);

        enc_data.is_ok()
    }

    pub fn encode_char(&self, c: char, encoderIndex: usize) -> Vec<u8> {
        assert!(encoderIndex < self.len());
        let encoder = self.encoders[encoderIndex];
        let enc_data = encoder.encode(&c.to_string(), encoding::EncoderTrap::Strict);
        assert!(enc_data.is_ok());
        return enc_data.unwrap();
    }

    pub fn encode_string(&self, s: &str, encoderIndex: usize) -> Vec<u8> {
        assert!(encoderIndex < self.len());
        let encoder = self.encoders[encoderIndex];
        encoder.encode(s, encoding::EncoderTrap::Replace).unwrap()
    }
}
