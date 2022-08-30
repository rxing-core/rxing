pub mod detector;
pub mod reedsolomon;

use core::num;
use std::cmp;
use std::collections::HashMap;
use std::fmt;

use crate::Exceptions;
use crate::DecodeHintType;
use crate::RXingResultPoint;
use encoding::Encoding;

#[cfg(test)]
mod StringUtilsTestCase;

#[cfg(test)]
mod BitArrayTestCase;
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
            }else {
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
            let mask:u64 = (2 << lastBit) - (1 << firstBit);
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
    pub fn isRange(
        &self,
        start: usize,
        end: usize,
        value: bool,
    ) -> Result<bool, Exceptions> {
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
            let mask:u64 = (2 << lastBit) - (1 << firstBit);

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
    pub fn appendBits(
        &mut self,
        value: u32,
        numBits: usize,
    ) -> Result<(), Exceptions> {
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
            return Err(Exceptions::IllegalArgumentException("Sizes don't match".to_owned()));
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
        let mut newBits = vec![0;self.bits.len()];
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

        let mut bits = vec![false;stringRepresentation.len()];
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
                        return Err(Exceptions::IllegalArgumentException("row lengths do not match".to_owned()));
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
                return Err(Exceptions::IllegalArgumentException("row lengths do not match".to_owned()));
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
        return self.bits[y as usize * self.rowSize..self.rowSize]
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
        let mut newBits = vec![0;(newRowSize * newHeight).try_into().unwrap()];

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
        let mut bitsOffset = self.bits.len() - 1;
        while bitsOffset >= 0 && self.bits[bitsOffset] == 0 {
            bitsOffset -= 1;
        }
        if bitsOffset < 0 {
            return None;
        }

        let y = bitsOffset / self.rowSize;
        let mut x = (bitsOffset % self.rowSize) * 32;

        let theBits = self.bits[bitsOffset];
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
    byteOffset: usize,
    bitOffset: usize,
}

impl BitSource {
    /**
     * @param bytes bytes from which this will read bits. Bits will be read from the first byte first.
     * Bits are read within a byte from most-significant to least-significant bit.
     */
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            byteOffset: 0,
            bitOffset: 0,
        }
    }

    /**
     * @return index of next bit in current byte which would be read by the next call to {@link #readBits(int)}.
     */
    pub fn getBitOffset(&self) -> usize {
        return self.bitOffset;
    }

    /**
     * @return index of next byte in input byte array which would be read by the next call to {@link #readBits(int)}.
     */
    pub fn getByteOffset(&self) -> usize {
        return self.byteOffset;
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

        let mut numBits = numBits;

        // First, read remainder from current byte
        if self.bitOffset > 0 {
            let bitsLeft = 8 - self.bitOffset;
            let toRead = cmp::min(numBits, bitsLeft);
            let bitsToNotRead = bitsLeft - toRead;
            let mask = (0xFF >> (8 - toRead)) << bitsToNotRead;
            result = (self.bytes[self.byteOffset] & mask) >> bitsToNotRead;
            numBits -= toRead;
            self.bitOffset += toRead;
            if self.bitOffset == 8 {
                self.bitOffset = 0;
                self.byteOffset += 1;
            }
        }

        // Next read whole bytes
        if numBits > 0 {
            while numBits >= 8 {
                result = (result << 8) | (self.bytes[self.byteOffset] & 0xFF);
                self.byteOffset += 1;
                numBits -= 8;
            }

            // Finally read a partial byte
            if numBits > 0 {
                let bitsToNotRead = 8 - numBits;
                let mask = (0xFF >> bitsToNotRead) << bitsToNotRead;
                result =
                    (result << numBits) | ((self.bytes[self.byteOffset] & mask) >> bitsToNotRead);
                self.bitOffset += numBits;
            }
        }

        return Ok(result.into());
    }

    /**
     * @return number of bits that can be read successfully
     */
    pub fn available(&self) -> usize {
        return 8 * (self.bytes.len() - self.byteOffset) - self.bitOffset;
    }
}
