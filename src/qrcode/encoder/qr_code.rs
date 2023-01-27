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

use std::fmt;

use crate::qrcode::decoder::{ErrorCorrectionLevel, Mode, Version, VersionRef};

use super::ByteMatrix;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */
#[derive(Debug, Clone)]
pub struct QRCode {
    // public static final int NUM_MASK_PATTERNS = 8;
    mode: Option<Mode>,
    ecLevel: Option<ErrorCorrectionLevel>,
    version: Option<VersionRef>,
    maskPattern: i32,
    matrix: Option<ByteMatrix>,
}

impl QRCode {
    pub const NUM_MASK_PATTERNS: i32 = 8;

    pub fn new() -> Self {
        Self {
            mode: None,
            ecLevel: None,
            version: None,
            maskPattern: -1,
            matrix: None,
        }
    }

    /**
     * @return the mode. Not relevant if {@link com.google.zxing.EncodeHintType#QR_COMPACT} is selected.
     */
    pub fn getMode(&self) -> &Option<Mode> {
        &self.mode
    }

    pub fn getECLevel(&self) -> &Option<ErrorCorrectionLevel> {
        &self.ecLevel
    }

    pub fn getVersion(&self) -> &Option<&'static Version> {
        &self.version
    }

    pub fn getMaskPattern(&self) -> i32 {
        self.maskPattern
    }

    pub fn getMatrix(&self) -> &Option<ByteMatrix> {
        &self.matrix
    }

    pub fn setMode(&mut self, value: Mode) {
        self.mode = Some(value);
    }

    pub fn setECLevel(&mut self, value: ErrorCorrectionLevel) {
        self.ecLevel = Some(value);
    }

    pub fn setVersion(&mut self, version: &'static Version) {
        self.version = Some(version);
    }

    pub fn setMaskPattern(&mut self, value: i32) {
        self.maskPattern = value;
    }

    pub fn setMatrix(&mut self, value: ByteMatrix) {
        self.matrix = Some(value);
    }

    // Check if "mask_pattern" is valid.
    pub fn isValidMaskPattern(maskPattern: i32) -> bool {
        (0..Self::NUM_MASK_PATTERNS).contains(&maskPattern)
    }
}

impl fmt::Display for QRCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::with_capacity(200);
        result.push_str("<<\n");
        result.push_str(" mode: ");
        if self.mode.is_some() {
            result.push_str(&format!("{:?}", self.mode.as_ref().unwrap()));
        } else {
            result.push_str("null");
        }
        // result.push_str(&format!("{:?}", self.mode));
        result.push_str("\n ecLevel: ");
        if self.ecLevel.is_some() {
            result.push_str(&format!("{:?}", self.ecLevel.as_ref().unwrap()));
        } else {
            result.push_str("null");
        }
        // result.push_str(&format!("{:?}", self.ecLevel));
        result.push_str("\n version: ");
        if self.version.is_some() {
            result.push_str(&format!("{}", self.version.as_ref().unwrap()));
        } else {
            result.push_str("null");
        }
        result.push_str("\n maskPattern: ");
        result.push_str(&format!("{}", self.maskPattern));
        if self.matrix.is_none() {
            result.push_str("\n matrix: null\n");
        } else {
            result.push_str("\n matrix:\n");
            if self.matrix.is_some() {
                result.push_str(&format!("{}", self.matrix.as_ref().unwrap()));
            } else {
                result.push_str("null");
            }
        }
        result.push_str(">>\n");

        write!(f, "{result}")
    }
}

impl Default for QRCode {
    fn default() -> Self {
        Self::new()
    }
}
