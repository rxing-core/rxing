/*
 * Copyright 2011 ZXing authors
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

use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{
    common::{
        reedsolomon::{get_predefined_genericgf, PredefinedGenericGF, ReedSolomonDecoder},
        BitMatrix, DecoderRXingResult, Result,
    },
    DecodingHintDictionary, Exceptions,
};

use super::{decoded_bit_stream_parser, BitMatrixParser};

/**
 * <p>The main class which implements MaxiCode decoding -- as opposed to locating and extracting
 * the MaxiCode from an image.</p>
 *
 * @author Manuel Kasten
 */

const ALL: u32 = 0;
const EVEN: u32 = 1;
const ODD: u32 = 2;

static RS_DECODER: Lazy<ReedSolomonDecoder> = Lazy::new(|| {
    ReedSolomonDecoder::new(get_predefined_genericgf(
        PredefinedGenericGF::MaxicodeField64,
    ))
});

pub fn decode(bits: &BitMatrix) -> Result<DecoderRXingResult> {
    decode_with_hints(bits, &HashMap::new())
}

pub fn decode_with_hints(
    bits: &BitMatrix,
    _hints: &DecodingHintDictionary,
) -> Result<DecoderRXingResult> {
    let parser = BitMatrixParser::new(bits);
    let mut codewords = parser.readCodewords();

    correctErrors(&mut codewords, 0, 10, 10, ALL)?;
    let mode = codewords[0] & 0x0F;
    let mut datawords;
    match mode {
        2 | 3 | 4 => {
            correctErrors(&mut codewords, 20, 84, 40, EVEN)?;
            correctErrors(&mut codewords, 20, 84, 40, ODD)?;
            datawords = vec![0u8; 94];
        }
        5 => {
            correctErrors(&mut codewords, 20, 68, 56, EVEN)?;
            correctErrors(&mut codewords, 20, 68, 56, ODD)?;
            datawords = vec![0u8; 78];
        }
        _ => return Err(Exceptions::NOT_FOUND),
    }

    datawords[0..10].clone_from_slice(&codewords[0..10]);
    // System.arraycopy(codewords, 0, datawords, 0, 10);
    let datawords_len = datawords.len();
    datawords[10..datawords_len].clone_from_slice(&codewords[20..datawords_len + 10]);
    // System.arraycopy(codewords, 20, datawords, 10, datawords.length - 10);

    decoded_bit_stream_parser::decode(&datawords, mode)
}

fn correctErrors(
    codewordBytes: &mut [u8],
    start: u32,
    dataCodewords: u32,
    ecCodewords: u32,
    mode: u32,
) -> Result<()> {
    let codewords = dataCodewords + ecCodewords;

    // in EVEN or ODD mode only half the codewords
    let divisor = if mode == ALL { 1 } else { 2 };

    // First read into an array of ints
    let mut codewordsInts = vec![0; (codewords / divisor) as usize];
    for i in 0..codewords {
        if (mode == ALL) || (i % 2 == (mode - 1)) {
            codewordsInts[(i / divisor) as usize] = codewordBytes[(i + start) as usize] as i32;
        }
    }

    RS_DECODER.decode(&mut codewordsInts, (ecCodewords / divisor) as i32)?;

    // Copy back into array of bytes -- only need to worry about the bytes that were data
    // We don't care about errors in the error-correction codewords
    for i in 0..dataCodewords {
        // for (int i = 0; i < dataCodewords; i++) {
        if (mode == ALL) || (i % 2 == (mode - 1)) {
            codewordBytes[(i + start) as usize] = codewordsInts[(i / divisor) as usize] as u8;
        }
    }
    Ok(())
}
