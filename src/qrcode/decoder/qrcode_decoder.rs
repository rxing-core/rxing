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

use std::{collections::HashMap, sync::Arc};

/**
 * <p>The main class which implements QR Code decoding -- as opposed to locating and extracting
 * the QR Code from an image.</p>
 *
 * @author Sean Owen
 */
use once_cell::sync::Lazy;

use crate::{
    common::{
        reedsolomon::get_predefined_genericgf, reedsolomon::PredefinedGenericGF,
        reedsolomon::ReedSolomonDecoder, BitMatrix, DecoderRXingResult, Result,
    },
    DecodingHintDictionary, Exceptions,
};

use super::{decoded_bit_stream_parser, BitMatrixParser, DataBlock, QRCodeDecoderMetaData};

//rsDecoder = new ReedSolomonDecoder(GenericGF.QR_CODE_FIELD_256);
static RS_DECODER: Lazy<ReedSolomonDecoder> = Lazy::new(|| {
    ReedSolomonDecoder::new(get_predefined_genericgf(
        PredefinedGenericGF::QrCodeField256,
    ))
});

pub fn decode_bool_array(image: &[Vec<bool>]) -> Result<DecoderRXingResult> {
    decode_bool_array_with_hints(image, &HashMap::new())
}

/**
 * <p>Convenience method that can decode a QR Code represented as a 2D array of booleans.
 * "true" is taken to mean a black module.</p>
 *
 * @param image booleans representing white/black QR Code modules
 * @param hints decoding hints that should be used to influence decoding
 * @return text and bytes encoded within the QR Code
 * @throws FormatException if the QR Code cannot be decoded
 * @throws ChecksumException if error correction fails
 */
pub fn decode_bool_array_with_hints(
    image: &[Vec<bool>],
    hints: &DecodingHintDictionary,
) -> Result<DecoderRXingResult> {
    decode_bitmatrix_with_hints(&BitMatrix::parse_bools(image), hints)
}

pub fn decode_bitmatrix(bits: &BitMatrix) -> Result<DecoderRXingResult> {
    decode_bitmatrix_with_hints(bits, &HashMap::new())
}

/**
 * <p>Decodes a QR Code represented as a {@link BitMatrix}. A 1 or "true" is taken to mean a black module.</p>
 *
 * @param bits booleans representing white/black QR Code modules
 * @param hints decoding hints that should be used to influence decoding
 * @return text and bytes encoded within the QR Code
 * @throws FormatException if the QR Code cannot be decoded
 * @throws ChecksumException if error correction fails
 */
pub fn decode_bitmatrix_with_hints(
    bits: &BitMatrix,
    hints: &DecodingHintDictionary,
) -> Result<DecoderRXingResult> {
    // Construct a parser and read version, error-correction level
    let mut parser = BitMatrixParser::new(bits.clone())?;
    let mut fe = None;
    let mut ce = None;
    match decode_bitmatrix_parser_with_hints(&mut parser, hints) {
        Ok(ok) => return Ok(ok),
        Err(er) => match er {
            Exceptions::FormatException(_) => fe = Some(er),
            Exceptions::ChecksumException(_) => ce = Some(er),
            _ => return Err(er),
        },
    }

    let mut trying = || -> Result<DecoderRXingResult> {
        // Revert the bit matrix
        parser.remask()?;

        // Will be attempting a mirrored reading of the version and format info.
        parser.setMirror(true);

        // Preemptively read the version.
        parser.readVersion()?;

        // Preemptively read the format information.
        parser.readFormatInformation()?;

        /*
         * Since we're here, this means we have successfully detected some kind
         * of version and format information when mirrored. This is a good sign,
         * that the QR code may be mirrored, and we should try once more with a
         * mirrored content.
         */
        // Prepare for a mirrored reading.
        parser.mirror();

        let mut result = decode_bitmatrix_parser_with_hints(&mut parser, hints)?;

        // Success! Notify the caller that the code was mirrored.
        result.setOther(Some(Arc::new(QRCodeDecoderMetaData::new(true))));

        Ok(result)
    };

    match trying() {
        Ok(res) => Ok(res),
        Err(er) => match er {
            Exceptions::FormatException(_) | Exceptions::ChecksumException(_) => {
                if let Some(fe) = fe {
                    Err(fe)
                } else {
                    Err(ce.unwrap_or(Exceptions::CHECKSUM))
                }
            }
            _ => Err(er),
        },
    }
}

fn decode_bitmatrix_parser_with_hints(
    parser: &mut BitMatrixParser,
    hints: &DecodingHintDictionary,
) -> Result<DecoderRXingResult> {
    let version = parser.readVersion()?;
    let ecLevel = parser.readFormatInformation()?.getErrorCorrectionLevel();

    // Read codewords
    let codewords = parser.readCodewords()?;
    // Separate into data blocks
    let dataBlocks = DataBlock::getDataBlocks(&codewords, version, ecLevel)?;

    // Count total number of data bytes
    let totalBytes = dataBlocks.iter().fold(0, |acc, dataBlock| {
        acc + dataBlock.getNumDataCodewords() as usize
    });

    let mut resultBytes = vec![0u8; totalBytes];
    let mut resultOffset = 0;

    // Error-correct and copy data blocks together into a stream of bytes
    for dataBlock in &dataBlocks {
        let mut codewordBytes = dataBlock.getCodewords().to_vec();
        let numDataCodewords = dataBlock.getNumDataCodewords() as usize;
        correctErrors(&mut codewordBytes, numDataCodewords)?;
        for codeword_byte in codewordBytes.iter().take(numDataCodewords) {
            resultBytes[resultOffset] = *codeword_byte;
            resultOffset += 1;
        }
    }

    // Decode the contents of that stream of bytes
    decoded_bit_stream_parser::decode(&resultBytes, version, ecLevel, hints)
}

/**
 * <p>Given data and error-correction codewords received, possibly corrupted by errors, attempts to
 * correct the errors in-place using Reed-Solomon error correction.</p>
 *
 * @param codewordBytes data and error correction codewords
 * @param numDataCodewords number of codewords that are data bytes
 * @throws ChecksumException if error correction fails
 */
fn correctErrors(codewordBytes: &mut [u8], numDataCodewords: usize) -> Result<()> {
    let numCodewords = codewordBytes.len();
    // First read into an array of ints
    let mut codewordsInts = vec![0u8; numCodewords];
    codewordsInts[..numCodewords].copy_from_slice(&codewordBytes[..numCodewords]);

    let mut sending_code_words: Vec<i32> = codewordsInts.iter().map(|x| *x as i32).collect();

    if let Err(Exceptions::ReedSolomonException(error_str)) = RS_DECODER.decode(
        &mut sending_code_words,
        (codewordBytes.len() - numDataCodewords) as i32,
    ) {
        return Err(Exceptions::ChecksumException(error_str));
    }

    // Copy back into array of bytes -- only need to worry about the bytes that were data
    // We don't care about errors in the error-correction codewords
    for (code_word, sent_code_word) in codewordBytes
        .iter_mut()
        .zip(sending_code_words.iter())
        .take(numDataCodewords)
    {
        *code_word = *sent_code_word as u8;
    }

    Ok(())
}
