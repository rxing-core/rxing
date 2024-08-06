/*
 * Copyright 2009 ZXing authors
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

use crate::{common::Result, Binarizer, BinaryBitmap, DecodeHints, DecodingHintDictionary, RXingResult};

/**
 * Implementation of this interface attempt to read several barcodes from one image.
 *
 * @see com.google.zxing.Reader
 * @author Sean Owen
 */
pub trait MultipleBarcodeReader {
    fn decode_multiple<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
    ) -> Result<Vec<RXingResult>>;

    fn decode_multiple_with_hints<B: Binarizer>(
        &mut self,
        image: &mut BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> Result<Vec<RXingResult>>;
}
