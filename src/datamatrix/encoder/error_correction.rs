/*
 * Copyright 2006 Jeremias Maerki.
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

use crate::common::Result;
use crate::Exceptions;

use super::SymbolInfo;

use once_cell::sync::Lazy;

/**
 * Error Correction Code for ECC200.
 */

/**
 * Lookup table which factors to use for which number of error correction codewords.
 * See FACTORS.
 */
const FACTOR_SETS: [u32; 16] = [5, 7, 10, 11, 12, 14, 18, 20, 24, 28, 36, 42, 48, 56, 62, 68];

/**
 * Precomputed polynomial factors for ECC 200.
 */
static FACTORS: Lazy<[Vec<u32>; 16]> = Lazy::new(|| {
    [
        vec![228, 48, 15, 111, 62],
        vec![23, 68, 144, 134, 240, 92, 254],
        vec![28, 24, 185, 166, 223, 248, 116, 255, 110, 61],
        vec![175, 138, 205, 12, 194, 168, 39, 245, 60, 97, 120],
        vec![41, 153, 158, 91, 61, 42, 142, 213, 97, 178, 100, 242],
        vec![
            156, 97, 192, 252, 95, 9, 157, 119, 138, 45, 18, 186, 83, 185,
        ],
        vec![
            83, 195, 100, 39, 188, 75, 66, 61, 241, 213, 109, 129, 94, 254, 225, 48, 90, 188,
        ],
        vec![
            15, 195, 244, 9, 233, 71, 168, 2, 188, 160, 153, 145, 253, 79, 108, 82, 27, 174, 186,
            172,
        ],
        vec![
            52, 190, 88, 205, 109, 39, 176, 21, 155, 197, 251, 223, 155, 21, 5, 172, 254, 124, 12,
            181, 184, 96, 50, 193,
        ],
        vec![
            211, 231, 43, 97, 71, 96, 103, 174, 37, 151, 170, 53, 75, 34, 249, 121, 17, 138, 110,
            213, 141, 136, 120, 151, 233, 168, 93, 255,
        ],
        vec![
            245, 127, 242, 218, 130, 250, 162, 181, 102, 120, 84, 179, 220, 251, 80, 182, 229, 18,
            2, 4, 68, 33, 101, 137, 95, 119, 115, 44, 175, 184, 59, 25, 225, 98, 81, 112,
        ],
        vec![
            77, 193, 137, 31, 19, 38, 22, 153, 247, 105, 122, 2, 245, 133, 242, 8, 175, 95, 100, 9,
            167, 105, 214, 111, 57, 121, 21, 1, 253, 57, 54, 101, 248, 202, 69, 50, 150, 177, 226,
            5, 9, 5,
        ],
        vec![
            245, 132, 172, 223, 96, 32, 117, 22, 238, 133, 238, 231, 205, 188, 237, 87, 191, 106,
            16, 147, 118, 23, 37, 90, 170, 205, 131, 88, 120, 100, 66, 138, 186, 240, 82, 44, 176,
            87, 187, 147, 160, 175, 69, 213, 92, 253, 225, 19,
        ],
        vec![
            175, 9, 223, 238, 12, 17, 220, 208, 100, 29, 175, 170, 230, 192, 215, 235, 150, 159,
            36, 223, 38, 200, 132, 54, 228, 146, 218, 234, 117, 203, 29, 232, 144, 238, 22, 150,
            201, 117, 62, 207, 164, 13, 137, 245, 127, 67, 247, 28, 155, 43, 203, 107, 233, 53,
            143, 46,
        ],
        vec![
            242, 93, 169, 50, 144, 210, 39, 118, 202, 188, 201, 189, 143, 108, 196, 37, 185, 112,
            134, 230, 245, 63, 197, 190, 250, 106, 185, 221, 175, 64, 114, 71, 161, 44, 147, 6, 27,
            218, 51, 63, 87, 10, 40, 130, 188, 17, 163, 31, 176, 170, 4, 107, 232, 7, 94, 166, 224,
            124, 86, 47, 11, 204,
        ],
        vec![
            220, 228, 173, 89, 251, 149, 159, 56, 89, 33, 147, 244, 154, 36, 73, 127, 213, 136,
            248, 180, 234, 197, 158, 177, 68, 122, 93, 213, 15, 160, 227, 236, 66, 139, 153, 185,
            202, 167, 179, 25, 220, 232, 96, 210, 231, 136, 223, 239, 181, 241, 59, 52, 172, 25,
            49, 232, 211, 189, 64, 54, 108, 153, 132, 63, 96, 103, 82, 186,
        ],
    ]
});

const MODULO_VALUE: usize = 0x12D;

const LOG: [u32; 256] = {
    let mut log_array = [0u32; 256];
    let mut p = 1;
    let mut i = 0;
    while i < 255 {
        log_array[p] = i;
        p *= 2;
        if p >= 256 {
            p ^= MODULO_VALUE;
        }

        i += 1;
    }

    log_array
};
const ALOG: [u32; 255] = {
    let mut alog_array = [0u32; 255];
    let mut p = 1_usize;
    let mut i = 0;
    while i < 255 {
        // for (int i = 0; i < 255; i++) {
        alog_array[i] = p as u32;
        p *= 2;
        if p >= 256 {
            p ^= MODULO_VALUE;
        }

        i += 1;
    }

    alog_array
};

// static {
//   //Create log and antilog table
//   LOG = new int[256];
//   ALOG = new int[255];

//   int p = 1;
//   for (int i = 0; i < 255; i++) {
//     ALOG[i] = p;
//     LOG[p] = i;
//     p *= 2;
//     if (p >= 256) {
//       p ^= MODULO_VALUE;
//     }
//   }
// }

/**
 * Creates the ECC200 error correction for an encoded message.
 *
 * @param codewords  the codewords
 * @param symbolInfo information about the symbol to be encoded
 * @return the codewords with interleaved error correction.
 */
pub fn encodeECC200(codewords: &str, symbolInfo: &SymbolInfo) -> Result<String> {
    if codewords.chars().count() != symbolInfo.getDataCapacity() as usize {
        return Err(Exceptions::illegalArgumentWith(
            "The number of codewords does not match the selected symbol",
        ));
    }
    let mut sb = String::with_capacity(
        (symbolInfo.getDataCapacity() + symbolInfo.getErrorCodewords()) as usize,
    );
    sb.push_str(codewords);
    let blockCount = symbolInfo.getInterleavedBlockCount() as usize;
    if blockCount == 1 {
        let ecc = createECCBlock(codewords, symbolInfo.getErrorCodewords() as usize)?;
        sb.push_str(&ecc);
    } else {
        //sb.setLength(sb.capacity());
        let mut dataSizes = vec![0u32; blockCount];
        let mut errorSizes = vec![0u32; blockCount];
        for i in 0..blockCount {
            // for (int i = 0; i < blockCount; i++) {
            dataSizes[i] = symbolInfo.getDataLengthForInterleavedBlock(i as u32 + 1) as u32;
            errorSizes[i] = symbolInfo.getErrorLengthForInterleavedBlock(i as u32 + 1);
        }
        for block in 0..blockCount {
            // for (int block = 0; block < blockCount; block++) {
            let mut temp = String::with_capacity(dataSizes[block] as usize);
            let mut d = block;
            while d < symbolInfo.getDataCapacity() as usize {
                // for (int d = block; d < symbolInfo.getDataCapacity(); d += blockCount) {
                temp.push(
                    codewords
                        .chars()
                        .nth(d)
                        .ok_or(Exceptions::indexOutOfBounds)?,
                );

                d += blockCount;
            }
            let ecc = createECCBlock(&temp, errorSizes[block] as usize)?;
            let mut pos = 0;
            let mut e = block;
            while e < errorSizes[block] as usize * blockCount {
                // for (int e = block; e < errorSizes[block] * blockCount; e += blockCount) {
                let (char_index, replace_char) = sb
                    .char_indices()
                    .nth(symbolInfo.getDataCapacity() as usize + e)
                    .ok_or(Exceptions::indexOutOfBounds)?;
                sb.replace_range(
                    char_index..(replace_char.len_utf8()),
                    &ecc.chars()
                        .nth(pos)
                        .ok_or(Exceptions::indexOutOfBounds)?
                        .to_string(),
                );
                // sb.setCharAt(symbolInfo.getDataCapacity() + e, ecc.charAt(pos));
                pos += 1;

                e += blockCount;
            }
        }
    }

    Ok(sb)
}

fn createECCBlock(codewords: &str, numECWords: usize) -> Result<String> {
    let mut table = -1_isize;
    for (i, set) in FACTOR_SETS.iter().enumerate() {
        // for i in 0..FACTOR_SETS.len() {
        // for (int i = 0; i < FACTOR_SETS.length; i++) {
        if set == &(numECWords as u32) {
            table = i as isize;
            break;
        }
    }
    if table < 0 {
        return Err(Exceptions::illegalArgumentWith(format!(
            "Illegal number of error correction codewords specified: {numECWords}"
        )));
    }
    let poly = &FACTORS[table as usize];
    let mut ecc = vec![0 as char; numECWords];
    // for i in 0..numECWords {
    // // for (int i = 0; i < numECWords; i++) {
    //   ecc[i] = 0;
    // }
    for i in 0..codewords.chars().count() {
        // for (int i = 0; i < codewords.length(); i++) {
        let m = ecc[numECWords - 1] as usize
            ^ codewords
                .chars()
                .nth(i)
                .ok_or(Exceptions::indexOutOfBounds)? as usize;
        for k in (1..numECWords).rev() {
            // for (int k = numECWords - 1; k > 0; k--) {
            if m != 0 && poly[k] != 0 {
                ecc[k] = char::from_u32(
                    ecc[k - 1] as u32 ^ ALOG[(LOG[m] + LOG[poly[k] as usize]) as usize % 255],
                )
                .ok_or(Exceptions::indexOutOfBounds)?;
            } else {
                ecc[k] = ecc[k - 1];
            }
        }
        if m != 0 && poly[0] != 0 {
            ecc[0] = char::from_u32(ALOG[(LOG[m] + LOG[poly[0] as usize]) as usize % 255])
                .ok_or(Exceptions::indexOutOfBounds)?;
        } else {
            ecc[0] = 0 as char;
        }
    }
    // let eccReversed = new char[numECWords];
    // for (int i = 0; i < numECWords; i++) {
    //   eccReversed[i] = ecc[numECWords - i - 1];
    // }
    // return String.valueOf(eccReversed);
    Ok(ecc.into_iter().rev().collect())
}

#[cfg(test)]
mod test_case {
    use crate::datamatrix::encoder::{high_level_encode_test_case::visualize, SymbolInfoLookup};

    use super::encodeECC200;

    #[test]
    fn testRS() {
        //Sample from Annexe R in ISO/IEC 16022:2000(E)
        let cw: [char; 3] = [142 as char, 164 as char, 186 as char];
        let symbolInfo = SymbolInfoLookup::new()
            .lookup(3)
            .expect("find lookup")
            .unwrap(); //SymbolInfo.lookup(3);
        let s = encodeECC200(&String::from_iter(cw), symbolInfo).expect("encode");
        assert_eq!("142 164 186 114 25 5 88 102", visualize(&s));

        //"A" encoded (ASCII encoding + 2 padding characters)
        let cw: [char; 3] = [66 as char, 129 as char, 70 as char];
        let s = encodeECC200(&String::from_iter(cw), symbolInfo).expect("encode");
        assert_eq!("66 129 70 138 234 82 82 95", visualize(&s));
    }
}
