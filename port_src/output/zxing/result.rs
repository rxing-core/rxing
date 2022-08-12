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
// package com::google::zxing;

/**
 * <p>Encapsulates the result of decoding a barcode within an image.</p>
 *
 * @author Sean Owen
 */
pub struct Result {

     let text: String;

     let raw_bytes: Vec<i8>;

     let num_bits: i32;

     let result_points: Vec<ResultPoint>;

     let format: BarcodeFormat;

     let result_metadata: Map<ResultMetadataType, Object>;

     let timestamp: i64;
}

impl Result {

    pub fn new( text: &String,  raw_bytes: &Vec<i8>,  result_points: &Vec<ResultPoint>,  format: &BarcodeFormat) -> Result {
        this(&text, &raw_bytes, result_points, format, &System::current_time_millis());
    }

    pub fn new( text: &String,  raw_bytes: &Vec<i8>,  result_points: &Vec<ResultPoint>,  format: &BarcodeFormat,  timestamp: i64) -> Result {
        this(&text, &raw_bytes,  if raw_bytes == null { 0 } else { 8 * raw_bytes.len() }, result_points, format, timestamp);
    }

    pub fn new( text: &String,  raw_bytes: &Vec<i8>,  num_bits: i32,  result_points: &Vec<ResultPoint>,  format: &BarcodeFormat,  timestamp: i64) -> Result {
        let .text = text;
        let .rawBytes = raw_bytes;
        let .numBits = num_bits;
        let .resultPoints = result_points;
        let .format = format;
        let .resultMetadata = null;
        let .timestamp = timestamp;
    }

    /**
   * @return raw text encoded by the barcode
   */
    pub fn  get_text(&self) -> String  {
        return self.text;
    }

    /**
   * @return raw bytes encoded by the barcode, if applicable, otherwise {@code null}
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
   * @return points related to the barcode in the image. These are typically points
   *         identifying finder patterns or the corners of the barcode. The exact meaning is
   *         specific to the type of barcode that was decoded.
   */
    pub fn  get_result_points(&self) -> Vec<ResultPoint>  {
        return self.result_points;
    }

    /**
   * @return {@link BarcodeFormat} representing the format of the barcode that was decoded
   */
    pub fn  get_barcode_format(&self) -> BarcodeFormat  {
        return self.format;
    }

    /**
   * @return {@link Map} mapping {@link ResultMetadataType} keys to values. May be
   *   {@code null}. This contains optional metadata about what was detected about the barcode,
   *   like orientation.
   */
    pub fn  get_result_metadata(&self) -> Map<ResultMetadataType, Object>  {
        return self.result_metadata;
    }

    pub fn  put_metadata(&self,  type: &ResultMetadataType,  value: &Object)   {
        if self.result_metadata == null {
            self.result_metadata = EnumMap<>::new(ResultMetadataType.class);
        }
        self.result_metadata.put(type, &value);
    }

    pub fn  put_all_metadata(&self,  metadata: &Map<ResultMetadataType, Object>)   {
        if metadata != null {
            if self.result_metadata == null {
                self.result_metadata = metadata;
            } else {
                self.result_metadata.put_all(&metadata);
            }
        }
    }

    pub fn  add_result_points(&self,  new_points: &Vec<ResultPoint>)   {
         let old_points: Vec<ResultPoint> = self.result_points;
        if old_points == null {
            self.result_points = new_points;
        } else if new_points != null && new_points.len() > 0 {
             let all_points: [Option<ResultPoint>; old_points.len() + new_points.len()] = [None; old_points.len() + new_points.len()];
            System::arraycopy(old_points, 0, all_points, 0, old_points.len());
            System::arraycopy(new_points, 0, all_points, old_points.len(), new_points.len());
            self.result_points = all_points;
        }
    }

    pub fn  get_timestamp(&self) -> i64  {
        return self.timestamp;
    }

    pub fn  to_string(&self) -> String  {
        return self.text;
    }
}

