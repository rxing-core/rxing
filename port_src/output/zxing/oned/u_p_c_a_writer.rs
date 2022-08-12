/*
 * Copyright 2010 ZXing authors
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
// package com::google::zxing::oned;

/**
 * This object renders a UPC-A code as a {@link BitMatrix}.
 *
 * @author qwandor@google.com (Andrew Walbran)
 */
#[derive(Writer)]
pub struct UPCAWriter {

     let sub_writer: EAN13Writer = EAN13Writer::new();
}

impl UPCAWriter {

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32) -> BitMatrix  {
        return self.encode(&contents, format, width, height, null);
    }

    pub fn  encode(&self,  contents: &String,  format: &BarcodeFormat,  width: i32,  height: i32,  hints: &Map<EncodeHintType, ?>) -> BitMatrix  {
        if format != BarcodeFormat::UPC_A {
            throw IllegalArgumentException::new(format!("Can only encode UPC-A, but got {}", format));
        }
        // Transform a UPC-A code into the equivalent EAN-13 code and write it that way
        return self.sub_writer.encode(format!("0{}", contents), BarcodeFormat::EAN_13, width, height, &hints);
    }
}

