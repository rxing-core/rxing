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

use crate::{common::Result, BarcodeFormat, Binarizer, Exceptions, RXingResult, Reader};

use super::{EAN13Reader, OneDReader, UPCEANReader};

/**
 * <p>Implements decoding of the UPC-A format.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
#[derive(Default)]
pub struct UPCAReader(EAN13Reader);

impl Reader for UPCAReader {
    fn decode<B: Binarizer>(&mut self, image: &mut crate::BinaryBitmap<B>) -> Result<RXingResult> {
        Self::maybeReturnRXingResult(self.0.decode(image)?)
    }

    fn decode_with_hints<B: Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<RXingResult> {
        Self::maybeReturnRXingResult(self.0.decode_with_hints(image, hints)?)
    }
}

impl OneDReader for UPCAReader {
    fn decode_row(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<RXingResult> {
        Self::maybeReturnRXingResult(self.0.decode_row(rowNumber, row, hints)?)
    }
}

impl UPCEANReader for UPCAReader {
    fn decodeRowWithGuardRange(
        &self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        startGuardRange: &[usize; 2],
        hints: &crate::DecodingHintDictionary,
    ) -> Result<RXingResult>
    where
        Self: Sized,
    {
        Self::maybeReturnRXingResult(self.0.decodeRowWithGuardRange(
            rowNumber,
            row,
            startGuardRange,
            hints,
        )?)
    }

    fn getBarcodeFormat(&self) -> BarcodeFormat {
        BarcodeFormat::UPC_A
    }

    fn decodeMiddle(
        &self,
        row: &crate::common::BitArray,
        startRange: &[usize; 2],
        resultString: &mut String,
    ) -> Result<usize> {
        self.0.decodeMiddle(row, startRange, resultString)
    }
}

impl UPCAReader {
    // private final UPCEANReader ean13Reader = new EAN13Reader();

    fn maybeReturnRXingResult(result: RXingResult) -> Result<RXingResult> {
        let text = result.getText();
        if let Some(stripped_text) = text.strip_prefix('0') {
            let mut upcaRXingResult = RXingResult::new(
                stripped_text,
                Vec::new(),
                result.getPoints().to_vec(),
                BarcodeFormat::UPC_A,
            );
            upcaRXingResult.putAllMetadata(result.getRXingResultMetadata().clone());

            Ok(upcaRXingResult)
        } else {
            Err(Exceptions::NOT_FOUND)
        }
    }
}
