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
 * <p>Encapsulates the result of decoding a matrix of bits. This typically
 * applies to 2D barcode formats. For now it contains the raw bytes obtained,
 * as well as a String interpretation of those bytes, if applicable.</p>
 *
 * @author Sean Owen
 */
pub struct DecoderResult {

     let raw_bytes: Vec<i8>;

     let num_bits: i32;

     let text: String;

     let byte_segments: List<Vec<i8>>;

     let ec_level: String;

     let errors_corrected: Integer;

     let erasures: Integer;

     let other: Object;

     let structured_append_parity: i32;

     let structured_append_sequence_number: i32;

     let symbology_modifier: i32;
}

impl DecoderResult {

    pub fn new( raw_bytes: &Vec<i8>,  text: &String,  byte_segments: &List<Vec<i8>>,  ec_level: &String) -> DecoderResult {
        this(&raw_bytes, &text, &byte_segments, &ec_level, -1, -1, 0);
    }

    pub fn new( raw_bytes: &Vec<i8>,  text: &String,  byte_segments: &List<Vec<i8>>,  ec_level: &String,  symbology_modifier: i32) -> DecoderResult {
        this(&raw_bytes, &text, &byte_segments, &ec_level, -1, -1, symbology_modifier);
    }

    pub fn new( raw_bytes: &Vec<i8>,  text: &String,  byte_segments: &List<Vec<i8>>,  ec_level: &String,  sa_sequence: i32,  sa_parity: i32) -> DecoderResult {
        this(&raw_bytes, &text, &byte_segments, &ec_level, sa_sequence, sa_parity, 0);
    }

    pub fn new( raw_bytes: &Vec<i8>,  text: &String,  byte_segments: &List<Vec<i8>>,  ec_level: &String,  sa_sequence: i32,  sa_parity: i32,  symbology_modifier: i32) -> DecoderResult {
        let .rawBytes = raw_bytes;
        let .numBits =  if raw_bytes == null { 0 } else { 8 * raw_bytes.len() };
        let .text = text;
        let .byteSegments = byte_segments;
        let .ecLevel = ec_level;
        let .structuredAppendParity = sa_parity;
        let .structuredAppendSequenceNumber = sa_sequence;
        let .symbologyModifier = symbology_modifier;
    }

    /**
   * @return raw bytes representing the result, or {@code null} if not applicable
   */
    pub fn  get_raw_bytes(&self) -> Vec<i8>  {
        return self.raw_bytes;
    }

    /**
   * @return how many bits of {@link #getRawBytes()} are valid; typically 8 times its length
   * @since 3.3.0
   */
    pub fn  get_num_bits(&self) -> i32  {
        return self.num_bits;
    }

    /**
   * @param numBits overrides the number of bits that are valid in {@link #getRawBytes()}
   * @since 3.3.0
   */
    pub fn  set_num_bits(&self,  num_bits: i32)   {
        self.numBits = num_bits;
    }

    /**
   * @return text representation of the result
   */
    pub fn  get_text(&self) -> String  {
        return self.text;
    }

    /**
   * @return list of byte segments in the result, or {@code null} if not applicable
   */
    pub fn  get_byte_segments(&self) -> List<Vec<i8>>  {
        return self.byte_segments;
    }

    /**
   * @return name of error correction level used, or {@code null} if not applicable
   */
    pub fn  get_e_c_level(&self) -> String  {
        return self.ec_level;
    }

    /**
   * @return number of errors corrected, or {@code null} if not applicable
   */
    pub fn  get_errors_corrected(&self) -> Integer  {
        return self.errors_corrected;
    }

    pub fn  set_errors_corrected(&self,  errors_corrected: &Integer)   {
        self.errorsCorrected = errors_corrected;
    }

    /**
   * @return number of erasures corrected, or {@code null} if not applicable
   */
    pub fn  get_erasures(&self) -> Integer  {
        return self.erasures;
    }

    pub fn  set_erasures(&self,  erasures: &Integer)   {
        self.erasures = erasures;
    }

    /**
   * @return arbitrary additional metadata
   */
    pub fn  get_other(&self) -> Object  {
        return self.other;
    }

    pub fn  set_other(&self,  other: &Object)   {
        self.other = other;
    }

    pub fn  has_structured_append(&self) -> bool  {
        return self.structured_append_parity >= 0 && self.structured_append_sequence_number >= 0;
    }

    pub fn  get_structured_append_parity(&self) -> i32  {
        return self.structured_append_parity;
    }

    pub fn  get_structured_append_sequence_number(&self) -> i32  {
        return self.structured_append_sequence_number;
    }

    pub fn  get_symbology_modifier(&self) -> i32  {
        return self.symbology_modifier;
    }
}

