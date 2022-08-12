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
// package com::google::zxing::qrcode::encoder;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */

 const NUM_MASK_PATTERNS: i32 = 8;
pub struct QRCode {

     let mut mode: Mode;

     let ec_level: ErrorCorrectionLevel;

     let version: Version;

     let mask_pattern: i32;

     let mut matrix: ByteMatrix;
}

impl QRCode {

    pub fn new() -> QRCode {
        mask_pattern = -1;
    }

    /**
   * @return the mode. Not relevant if {@link com.google.zxing.EncodeHintType#QR_COMPACT} is selected.
   */
    pub fn  get_mode(&self) -> Mode  {
        return self.mode;
    }

    pub fn  get_e_c_level(&self) -> ErrorCorrectionLevel  {
        return self.ec_level;
    }

    pub fn  get_version(&self) -> Version  {
        return self.version;
    }

    pub fn  get_mask_pattern(&self) -> i32  {
        return self.mask_pattern;
    }

    pub fn  get_matrix(&self) -> ByteMatrix  {
        return self.matrix;
    }

    pub fn  to_string(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(200);
        result.append("<<\n");
        result.append(" mode: ");
        result.append(self.mode);
        result.append("\n ecLevel: ");
        result.append(self.ec_level);
        result.append("\n version: ");
        result.append(self.version);
        result.append("\n maskPattern: ");
        result.append(self.mask_pattern);
        if self.matrix == null {
            result.append("\n matrix: null\n");
        } else {
            result.append("\n matrix:\n");
            result.append(self.matrix);
        }
        result.append(">>\n");
        return result.to_string();
    }

    pub fn  set_mode(&self,  value: &Mode)   {
        self.mode = value;
    }

    pub fn  set_e_c_level(&self,  value: &ErrorCorrectionLevel)   {
        self.ec_level = value;
    }

    pub fn  set_version(&self,  version: &Version)   {
        self.version = version;
    }

    pub fn  set_mask_pattern(&self,  value: i32)   {
        self.mask_pattern = value;
    }

    pub fn  set_matrix(&self,  value: &ByteMatrix)   {
        self.matrix = value;
    }

    // Check if "mask_pattern" is valid.
    pub fn  is_valid_mask_pattern( mask_pattern: i32) -> bool  {
        return mask_pattern >= 0 && mask_pattern < NUM_MASK_PATTERNS;
    }
}

