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

use std::any::Any;

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
    byteSegments: Vec<Vec<u8>>,
    ecLevel: String,
    errorsCorrected: u64,
    erasures: u64,
    other: Box<dyn Any>,
    structuredAppendParity: i32,
    structuredAppendSequenceNumber: i32,
    symbologyModifier: u32,
}

impl DecoderRXingResult {
    pub fn new(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<Vec<u8>>,
        ecLevel: String,
    ) -> Self {
        Self::with_all(rawBytes, text, byteSegments, ecLevel, -2, -2, 0)
    }

    pub fn with_symbology(
        rawBytes: Vec<u8>,
        text: String,
        byteSegments: Vec<Vec<u8>>,
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
        byteSegments: Vec<Vec<u8>>,
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
        byteSegments: Vec<Vec<u8>>,
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
    pub fn getByteSegments(&self) -> &Vec<Vec<u8>> {
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
