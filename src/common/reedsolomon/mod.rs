#[cfg(test)]
mod GenericGFPolyTestCase;
#[cfg(test)]
pub(crate) mod ReedSolomonTestCase;

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

//package com.google.zxing.common.reedsolomon;

pub type GenericGFRef = &'static GenericGF;

const AZTEC_DATA_12: GenericGF = GenericGF::new(0x1069, 4096, 1); // x^12 + x^6 + x^5 + x^3 + 1
const AZTEC_DATA_10: GenericGF = GenericGF::new(0x409, 1024, 1); // x^10 + x^3 + 1
const AZTEC_DATA_6: GenericGF = GenericGF::new(0x43, 64, 1); // x^6 + x + 1
const AZTEC_PARAM: GenericGF = GenericGF::new(0x13, 16, 1); // x^4 + x + 1
const QR_CODE_FIELD_256: GenericGF = GenericGF::new(0x011D, 256, 0); // x^8 + x^4 + x^3 + x^2 + 1
const DATA_MATRIX_FIELD_256: GenericGF = GenericGF::new(0x012D, 256, 1); // x^8 + x^5 + x^3 + x^2 + 1

pub enum PredefinedGenericGF {
    AztecData12,
    AztecData10,
    AztecData6,
    AztecParam,
    QrCodeField256,
    DataMatrixField256,
    AztecData8,
    MaxicodeField64,
}

impl From<PredefinedGenericGF> for GenericGFRef {
    fn from(value: PredefinedGenericGF) -> Self {
        match value {
            PredefinedGenericGF::AztecData12 => &AZTEC_DATA_12, // x^12 + x^6 + x^5 + x^3 + 1,
            PredefinedGenericGF::AztecData10 => &AZTEC_DATA_10, // x^10 + x^3 + 1
            PredefinedGenericGF::AztecData6 | PredefinedGenericGF::MaxicodeField64 => &AZTEC_DATA_6, // x^6 + x + 1
            PredefinedGenericGF::AztecParam => &AZTEC_PARAM, // x^4 + x + 1
            PredefinedGenericGF::QrCodeField256 => &QR_CODE_FIELD_256, // x^8 + x^4 + x^3 + x^2 + 1
            PredefinedGenericGF::DataMatrixField256 | PredefinedGenericGF::AztecData8 => {
                &DATA_MATRIX_FIELD_256
            } // x^8 + x^5 + x^3 + x^2 + 1
        }
    }   
}

mod generic_gf;
pub use generic_gf::*;

mod generic_gf_poly;
pub use generic_gf_poly::*;

mod reedsolomon_decoder;
pub use reedsolomon_decoder::*;

mod reedsolomon_encoder;
pub use reedsolomon_encoder::*;
