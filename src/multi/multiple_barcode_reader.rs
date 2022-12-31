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

use crate::{BinaryBitmap, DecodingHintDictionary, Exceptions, RXingResult, LuminanceSource, Binarizer};

/**
 * Implementation of this interface attempt to read several barcodes from one image.
 *
 * @see com.google.zxing.Reader
 * @author Sean Owen
 */
pub trait MultipleBarcodeReader<L:LuminanceSource,B:Binarizer<L>> {
    fn decode_multiple(&mut self, image: &BinaryBitmap<L,B>) -> Result<Vec<RXingResult>, Exceptions>;

    fn decode_multiple_with_hints(
        &mut self,
        image: &BinaryBitmap<L,B>,
        hints: &DecodingHintDictionary,
    ) -> Result<Vec<RXingResult>, Exceptions>;
}
