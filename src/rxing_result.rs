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

use std::{collections::HashMap, fmt};

use crate::{BarcodeFormat, RXingResultMetadataType, RXingResultMetadataValue, Point};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/**
 * <p>Encapsulates the result of decoding a barcode within an image.</p>
 *
 * @author Sean Owen
 */
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub struct RXingResult {
    text: String,
    rawBytes: Vec<u8>,
    numBits: usize,
    resultPoints: Vec<Point>,
    format: BarcodeFormat,
    resultMetadata: HashMap<RXingResultMetadataType, RXingResultMetadataValue>,
    timestamp: u128,
}
impl RXingResult {
    pub fn new(
        text: &str,
        rawBytes: Vec<u8>,
        resultPoints: Vec<Point>,
        format: BarcodeFormat,
    ) -> Self {
        Self::new_timestamp(
            text,
            rawBytes,
            resultPoints,
            format,
            chrono::Utc::now().timestamp_millis() as u128,
        )
    }

    pub fn new_timestamp(
        text: &str,
        rawBytes: Vec<u8>,
        resultPoints: Vec<Point>,
        format: BarcodeFormat,
        timestamp: u128,
    ) -> Self {
        let l = rawBytes.len();
        Self::new_complex(text, rawBytes, 8 * l, resultPoints, format, timestamp)
    }

    pub fn new_complex(
        text: &str,
        rawBytes: Vec<u8>,
        numBits: usize,
        resultPoints: Vec<Point>,
        format: BarcodeFormat,
        timestamp: u128,
    ) -> Self {
        Self {
            text: text.to_owned(),
            rawBytes,
            numBits,
            resultPoints,
            format,
            resultMetadata: HashMap::new(),
            timestamp,
        }
    }

    pub fn new_from_existing_result(prev: Self, points: Vec<Point>) -> Self {
        Self {
            text: prev.text,
            rawBytes: prev.rawBytes,
            numBits: prev.numBits,
            resultPoints: points,
            format: prev.format,
            resultMetadata: prev.resultMetadata,
            timestamp: prev.timestamp,
        }
    }

    /**
     * @return raw text encoded by the barcode
     */
    pub fn getText(&self) -> &str {
        &self.text
    }

    /**
     * @return raw bytes encoded by the barcode, if applicable, otherwise {@code null}
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
     * @return points related to the barcode in the image. These are typically points
     *         identifying finder patterns or the corners of the barcode. The exact meaning is
     *         specific to the type of barcode that was decoded.
     */
    pub fn getPoints(&self) -> &Vec<Point> {
        &self.resultPoints
    }

    pub fn getPointsMut(&mut self) -> &mut Vec<Point> {
        &mut self.resultPoints
    }

    /** Currently necessary because the external OneDReader proc macro uses it. */
    pub fn getRXingResultPoints(&self) -> &Vec<Point> {
        &&self.resultPoints
    }

    /** Currently necessary because the external OneDReader proc macro uses it. */
    pub fn getRXingResultPointsMut(&mut self) -> &mut Vec<Point> {
        &mut self.resultPoints
    }

    /**
     * @return {@link BarcodeFormat} representing the format of the barcode that was decoded
     */
    pub fn getBarcodeFormat(&self) -> &BarcodeFormat {
        &self.format
    }

    /**
     * @return {@link Map} mapping {@link RXingResultMetadataType} keys to values. May be
     *   {@code null}. This contains optional metadata about what was detected about the barcode,
     *   like orientation.
     */
    pub fn getRXingResultMetadata(
        &self,
    ) -> &HashMap<RXingResultMetadataType, RXingResultMetadataValue> {
        &self.resultMetadata
    }

    pub fn putMetadata(
        &mut self,
        md_type: RXingResultMetadataType,
        value: RXingResultMetadataValue,
    ) {
        self.resultMetadata.insert(md_type, value);
    }

    pub fn putAllMetadata(
        &mut self,
        metadata: HashMap<RXingResultMetadataType, RXingResultMetadataValue>,
    ) {
        if self.resultMetadata.is_empty() {
            self.resultMetadata = metadata;
        } else {
            for (key, value) in metadata.into_iter() {
                self.resultMetadata.insert(key, value);
            }
        }
    }

    pub fn addPoints(&mut self, newPoints: &mut Vec<Point>) {
        if !newPoints.is_empty() {
            self.resultPoints.append(newPoints);
        }
    }

    pub fn getTimestamp(&self) -> u128 {
        self.timestamp
    }
}

impl fmt::Display for RXingResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
