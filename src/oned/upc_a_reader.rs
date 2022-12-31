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

use std::marker::PhantomData;

use crate::{BarcodeFormat, Exceptions, RXingResult, Reader, LuminanceSource, Binarizer};

use super::{EAN13Reader, OneDReader, UPCEANReader};

/**
 * <p>Implements decoding of the UPC-A format.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */
pub struct UPCAReader<L:LuminanceSource,B:Binarizer<L>>(EAN13Reader<L,B>,PhantomData<L>,PhantomData<B>);

impl<L:LuminanceSource,B:Binarizer<L>> Reader<L,B> for UPCAReader<L,B> {
    fn decode(&mut self, image: &crate::BinaryBitmap<L,B>) -> Result<crate::RXingResult, Exceptions> {
        Self::maybeReturnRXingResult(self.0.decode(image)?)
    }

    fn decode_with_hints(
        &mut self,
        image: &crate::BinaryBitmap<L,B>,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, Exceptions> {
        Self::maybeReturnRXingResult(self.0.decode_with_hints(image, hints)?)
    }
}

impl<L:LuminanceSource,B:Binarizer<L>> OneDReader<L,B> for UPCAReader<L,B> {
    fn decodeRow(
        &mut self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, Exceptions> {
        Self::maybeReturnRXingResult(self.0.decodeRow(rowNumber, row, hints)?)
    }
}

impl<L:LuminanceSource,B:Binarizer<L>> UPCEANReader<L,B> for UPCAReader<L,B> {
    fn getBarcodeFormat(&self) -> crate::BarcodeFormat {
        BarcodeFormat::UPC_A
    }

    fn decodeMiddle(
        &self,
        row: &crate::common::BitArray,
        startRange: &[usize; 2],
        resultString: &mut String,
    ) -> Result<usize, Exceptions> {
        self.0.decodeMiddle(row, startRange, resultString)
    }

    fn decodeRowWithGuardRange(
        &self,
        rowNumber: u32,
        row: &crate::common::BitArray,
        startGuardRange: &[usize; 2],
        hints: &crate::DecodingHintDictionary,
    ) -> Result<crate::RXingResult, Exceptions>
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
}

impl<L:LuminanceSource,B:Binarizer<L>> Default for UPCAReader<L,B> {
    fn default() -> Self {
        Self(Default::default(),PhantomData,PhantomData)
    }
}

impl<L:LuminanceSource,B:Binarizer<L>> UPCAReader<L,B> {
    // private final UPCEANReader ean13Reader = new EAN13Reader();

    fn maybeReturnRXingResult(result: RXingResult) -> Result<RXingResult, Exceptions> {
        let text = result.getText();
        if text.chars().nth(0).unwrap() == '0' {
            let mut upcaRXingResult = RXingResult::new(
                &text[1..],
                Vec::new(),
                result.getRXingResultPoints().to_vec(),
                BarcodeFormat::UPC_A,
            );
            // if result.getRXingResultMetadata() != null {
            upcaRXingResult.putAllMetadata(result.getRXingResultMetadata().clone());
            // }
            Ok(upcaRXingResult)
        } else {
            Err(Exceptions::NotFoundException("".to_owned()))
        }
    }
}
