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

//package com.google.zxing;

use crate::{common::BitMatrix, BarcodeFormat, EncodingHintDictionary, Exceptions};

/**
 * The base class for all objects which encode/generate a barcode image.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub trait Writer {
    /**
     * Encode a barcode using the default settings.
     *
     * @param contents The contents to encode in the barcode
     * @param format The barcode format to generate
     * @param width The preferred width in pixels
     * @param height The preferred height in pixels
     * @return {@link BitMatrix} representing encoded barcode image
     * @throws WriterException if contents cannot be encoded legally in a format
     */
    fn encode(
        &self,
        contents: &str,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<BitMatrix, Exceptions>;

    /**
     * @param contents The contents to encode in the barcode
     * @param format The barcode format to generate
     * @param width The preferred width in pixels
     * @param height The preferred height in pixels
     * @param hints Additional parameters to supply to the encoder
     * @return {@link BitMatrix} representing encoded barcode image
     * @throws WriterException if contents cannot be encoded legally in a format
     */
    fn encode_with_hints(
        &self,
        contents: &str,
        format: &BarcodeFormat,
        width: i32,
        height: i32,
        hints: &EncodingHintDictionary,
    ) -> Result<BitMatrix, Exceptions>;
}
