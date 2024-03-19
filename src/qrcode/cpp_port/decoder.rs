// /*
// * Copyright 2016 Nu-book Inc.
// * Copyright 2016 ZXing authors
// */
// // SPDX-License-Identifier: Apache-2.0

use crate::common::cpp_essentials::{DecoderResult, StructuredAppendInfo};
use crate::common::reedsolomon::{
    get_predefined_genericgf, PredefinedGenericGF, ReedSolomonDecoder,
};
use crate::common::{
    AIFlag, BitMatrix, BitSource, CharacterSet, ECIStringBuilder, Eci, Result, SymbologyIdentifier,
};
use crate::qrcode::cpp_port::bitmatrix_parser::{
    ReadCodewords, ReadFormatInformation, ReadVersion,
};
use crate::qrcode::decoder::{DataBlock, ErrorCorrectionLevel, Mode, Version};
use crate::Exceptions;

/**
* <p>Given data and error-correction codewords received, possibly corrupted by errors, attempts to
* correct the errors in-place using Reed-Solomon error correction.</p>
*
* @param codewordBytes data and error correction codewords
* @param numDataCodewords number of codewords that are data bytes
* @return false if error correction fails
*/
pub fn CorrectErrors(codewordBytes: &mut [u8], numDataCodewords: u32) -> Result<bool> {
    // First read into an array of ints
    // std::vector<int> codewordsInts(codewordBytes.begin(), codewordBytes.end());
    let mut codewordsInts: Vec<i32> = codewordBytes.iter().copied().map(|b| b as i32).collect();

    let numECCodewords = ((codewordBytes.len() as u32) - numDataCodewords) as i32;
    let rs = ReedSolomonDecoder::new(get_predefined_genericgf(
        PredefinedGenericGF::QrCodeField256,
    ));

    rs.decode(&mut codewordsInts, numECCodewords)?;

    // if rs.decode(&mut codewordsInts, numECCodewords)? != 0
    // // if (!ReedSolomonDecode(GenericGF::QRCodeField256(), codewordsInts, numECCodewords))
    // {
    //     return Ok(false);
    // }

    // Copy back into array of bytes -- only need to worry about the bytes that were data
    // We don't care about errors in the error-correction codewords
    codewordBytes[..numDataCodewords as usize].copy_from_slice(
        &codewordsInts[..numDataCodewords as usize]
            .iter()
            .copied()
            .map(|i| i as u8)
            .collect::<Vec<u8>>(),
    );
    // std::copy_n(codewordsInts.begin(), numDataCodewords, codewordBytes.begin());

    Ok(true)
}

/**
* See specification GBT 18284-2000
*/
pub fn DecodeHanziSegment(
    bits: &mut BitSource,
    count: u32,
    result: &mut ECIStringBuilder,
) -> Result<()> {
    let mut count = count;

    // Each character will require 2 bytes, decode as GB2312
    // There is no ECI value for GB2312, use GB18030 which is a superset
    result.switch_encoding(CharacterSet::GB18030, false);
    result.reserve(2 * count as usize);

    while count > 0 {
        // Each 13 bits encodes a 2-byte character
        let twoBytes = bits.readBits(13)?;
        let mut assembledTwoBytes = ((twoBytes / 0x060) << 8) | (twoBytes % 0x060);
        if assembledTwoBytes < 0x00A00 {
            // In the 0xA1A1 to 0xAAFE range
            assembledTwoBytes += 0x0A1A1;
        } else {
            // In the 0xB0A1 to 0xFAFE range
            assembledTwoBytes += 0x0A6A1;
        }
        *result += ((assembledTwoBytes >> 8) & 0xFF) as u8;
        *result += (assembledTwoBytes & 0xFF) as u8;
        count -= 1;
    }
    Ok(())
}

pub fn DecodeKanjiSegment(
    bits: &mut BitSource,
    count: u32,
    result: &mut ECIStringBuilder,
) -> Result<()> {
    let mut count = count;
    // Each character will require 2 bytes. Read the characters as 2-byte pairs
    // and decode as Shift_JIS afterwards
    result.switch_encoding(CharacterSet::Shift_JIS, false);
    result.reserve(2 * count as usize);

    while count > 0 {
        // Each 13 bits encodes a 2-byte character
        let twoBytes = bits.readBits(13)?;
        let mut assembledTwoBytes = ((twoBytes / 0x0C0) << 8) | (twoBytes % 0x0C0);
        if assembledTwoBytes < 0x01F00 {
            // In the 0x8140 to 0x9FFC range
            assembledTwoBytes += 0x08140;
        } else {
            // In the 0xE040 to 0xEBBF range
            assembledTwoBytes += 0x0C140;
        }
        *result += (assembledTwoBytes >> 8) as u8;
        *result += (assembledTwoBytes) as u8;
        count -= 1;
    }
    Ok(())
}

pub fn DecodeByteSegment(
    bits: &mut BitSource,
    count: u32,
    result: &mut ECIStringBuilder,
) -> Result<()> {
    result.switch_encoding(CharacterSet::Unknown, false);
    result.reserve(count as usize);

    for _i in 0..count {
        // for (int i = 0; i < count; i++)
        *result += (bits.readBits(8)?) as u8;
    }
    Ok(())
}

pub fn ToAlphaNumericChar(value: u32) -> Result<char> {
    let value = value as usize;
    /**
     * See ISO 18004:2006, 6.4.4 Table 5
     */
    const ALPHANUMERIC_CHARS: [char; 45] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ' ', '$', '%', '*', '+', '-', '.', '/', ':',
    ];

    if value >= (ALPHANUMERIC_CHARS.len()) {
        return Err(Exceptions::index_out_of_bounds_with(
            "oAlphaNumericChar: out of range",
        ));
    }

    Ok(ALPHANUMERIC_CHARS[value])
}

pub fn DecodeAlphanumericSegment(
    bits: &mut BitSource,
    count: u32,
    result: &mut ECIStringBuilder,
) -> Result<()> {
    let mut count = count;

    // Read two characters at a time
    let mut buffer = String::new();

    while count > 1 {
        let nextTwoCharsBits = bits.readBits(11)?;
        buffer.push(ToAlphaNumericChar(nextTwoCharsBits / 45)?);
        buffer.push(ToAlphaNumericChar(nextTwoCharsBits % 45)?);
        count -= 2;
    }
    if count == 1 {
        // special case: one character left
        buffer.push(ToAlphaNumericChar(bits.readBits(6)?)?);
    }
    // See section 6.4.8.1, 6.4.8.2
    if result.symbology.aiFlag != AIFlag::None {
        // We need to massage the result a bit if in an FNC1 mode:
        for i in 0..buffer.len() {
            // for (size_t i = 0; i < buffer.length(); i++) {
            if buffer
                .chars()
                .nth(i)
                .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                == '%'
            {
                if i < buffer.len() - 1
                    && buffer
                        .chars()
                        .nth(i + 1)
                        .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?
                        == '%'
                {
                    // %% is rendered as %
                    buffer.remove(i + 1);
                // buffer.erase(i + 1);
                } else {
                    // In alpha mode, % should be converted to FNC1 separator 0x1D
                    buffer.replace_range(i..i, &char::from(0x1D).to_string());
                    // buffer[i] = static_cast<char>(0x1D);
                }
            }
        }
    }

    result.switch_encoding(CharacterSet::ISO8859_1, false);
    *result += buffer;

    Ok(())
}

pub fn DecodeNumericSegment(
    bits: &mut BitSource,
    count: u32,
    result: &mut ECIStringBuilder,
) -> Result<()> {
    let mut count = count;

    result.switch_encoding(CharacterSet::ISO8859_1, false);
    result.reserve(count as usize);

    while count > 0 {
        let n = std::cmp::min(count, 3);
        let nDigits = bits.readBits(1 + 3 * n as usize)?; // read 4, 7 or 10 bits into 1, 2 or 3 digits
        result.append_string(&crate::common::cpp_essentials::util::ToString(
            nDigits as usize,
            n as usize,
        )?);
        count -= n;
    }

    Ok(())
}

pub fn ParseECIValue(bits: &mut BitSource) -> Result<Eci> {
    let firstByte = bits.readBits(8)?;
    if (firstByte & 0x80) == 0 {
        // just one byte
        return Ok(Eci::from(firstByte & 0x7F));
    }
    if (firstByte & 0xC0) == 0x80 {
        // two bytes
        let secondByte = bits.readBits(8)?;
        return Ok(Eci::from(((firstByte & 0x3F) << 8) | secondByte));
    }
    if (firstByte & 0xE0) == 0xC0 {
        // three bytes
        let secondThirdBytes = bits.readBits(16)?;
        return Ok(Eci::from(((firstByte & 0x1F) << 16) | secondThirdBytes));
    }
    Err(Exceptions::format_with("ParseECIValue: invalid value"))
}

/**
 * QR codes encode mode indicators and terminator codes into a constant bit length of 4.
 * Micro QR codes have terminator codes that vary in bit length but are always longer than
 * the mode indicators.
 * M1 - 0 length mode code, 3 bits terminator code
 * M2 - 1 bit mode code, 5 bits terminator code
 * M3 - 2 bit mode code, 7 bits terminator code
 * M4 - 3 bit mode code, 9 bits terminator code
 * IsTerminator peaks into the bit stream to see if the current position is at the start of
 * a terminator code.  If true, then the decoding can finish. If false, then the decoding
 * can read off the next mode code.
 *
 * See ISO 18004:2015, 7.4.1 Table 2
 *
 * @param bits the stream of bits that might have a terminator code
 * @param version the QR or micro QR code version
 */
pub fn IsEndOfStream(bits: &mut BitSource, version: &Version) -> Result<bool> {
    let bitsRequired = Mode::get_terminator_bit_length(version); //super::qr_codec_mode::TerminatorBitsLength(version);
    let bitsAvailable = std::cmp::min(bits.available(), bitsRequired as usize);
    Ok(bitsAvailable == 0 || bits.peak_bits(bitsAvailable)? == 0)
}

/**
* <p>QR Codes can encode text as bits in one of several modes, and can use multiple modes
* in one QR Code. This method decodes the bits back into text.</p>
*
* <p>See ISO 18004:2006, 6.4.3 - 6.4.7</p>
*/
// ZXING_EXPORT_TEST_ONLY
pub fn DecodeBitStream(
    bytes: &[u8],
    version: &Version,
    ecLevel: ErrorCorrectionLevel,
) -> Result<DecoderResult<bool>> {
    let mut bits = BitSource::new(bytes.to_vec());
    let mut result = ECIStringBuilder::default();
    // Error error;
    result.symbology = SymbologyIdentifier {
        code: b'Q',
        modifier: b'1',
        eciModifierOffset: 1,
        aiFlag: AIFlag::None,
    }; //{'Q', '1', 1};
    let mut structuredAppend = StructuredAppendInfo::default();
    let modeBitLength = Mode::get_codec_mode_bits_length(version);

    if version.isModel1() {
        bits.readBits(4)?; /* Model 1 is leading with 4 0-bits -> drop them */
    }

    let res = (|| {
        while !IsEndOfStream(&mut bits, version)? {
            let mode: Mode = if modeBitLength == 0 {
                Mode::NUMERIC // MicroQRCode version 1 is always NUMERIC and modeBitLength is 0
            } else {
                Mode::CodecModeForBits(
                    bits.readBits(modeBitLength as usize)?,
                    Some(version.qr_type),
                )?
            };

            match mode {
                Mode::FNC1_FIRST_POSITION => {
                    //				if (!result.empty()) // uncomment to enforce specification
                    //					throw FormatError("GS1 Indicator (FNC1 in first position) at illegal position");
                    result.symbology.modifier = b'3';
                    result.symbology.aiFlag = AIFlag::GS1; // In Alphanumeric mode undouble doubled '%' and treat single '%' as <GS>
                }
                Mode::FNC1_SECOND_POSITION => {
                    if !result.is_empty() {
                        return Err(Exceptions::format_with("AIM Application Indicator (FNC1 in second position) at illegal position"));
                        // throw FormatError("AIM Application Indicator (FNC1 in second position) at illegal position");
                    }
                    result.symbology.modifier = b'5'; // As above
                                                      // ISO/IEC 18004:2015 7.4.8.3 AIM Application Indicator (FNC1 in second position), "00-99" or "A-Za-z"
                    let appInd = bits.readBits(8)?;
                    if appInd < 100
                    // "00-09"
                    {
                        result +=
                            crate::common::cpp_essentials::util::ToString(appInd as usize, 2)?;
                    } else if (165..=190).contains(&appInd) || (197..=222).contains(&appInd)
                    // "A-Za-z"
                    {
                        result += (appInd - 100) as u8;
                    } else {
                        return Err(Exceptions::format_with("Invalid AIM Application Indicator"));
                        // throw FormatError("Invalid AIM Application Indicator");
                    }
                    result.symbology.aiFlag = AIFlag::AIM; // see also above
                }
                Mode::STRUCTURED_APPEND => {
                    // sequence number and parity is added later to the result metadata
                    // Read next 4 bits of index, 4 bits of symbol count, and 8 bits of parity data, then continue
                    structuredAppend.index = bits.readBits(4)? as i32;
                    structuredAppend.count = bits.readBits(4)? as i32 + 1;
                    structuredAppend.id = (bits.readBits(8)?).to_string(); //std::to_string(bits.readBits(8));
                }
                Mode::ECI => {
                    // Count doesn't apply to ECI
                    result.switch_encoding(ParseECIValue(&mut bits)?.into(), true);
                }
                Mode::HANZI => {
                    // First handle Hanzi mode which does not start with character count
                    // chinese mode contains a sub set indicator right after mode indicator
                    let subset = bits.readBits(4)?;
                    if subset != 1
                    // GB2312_SUBSET is the only supported one right now
                    {
                        return Err(Exceptions::format_with("Unsupported HANZI subset"));
                        // throw FormatError("Unsupported HANZI subset");
                    }
                    let count = bits.readBits(mode.CharacterCountBits(version) as usize)?;
                    DecodeHanziSegment(&mut bits, count, &mut result)?;
                }
                _ => {
                    // "Normal" QR code modes:
                    // How many characters will follow, encoded in this mode?
                    let count = bits.readBits(mode.CharacterCountBits(version) as usize)?;
                    match mode {
                        Mode::NUMERIC => DecodeNumericSegment(&mut bits, count, &mut result)?,
                        Mode::ALPHANUMERIC => {
                            DecodeAlphanumericSegment(&mut bits, count, &mut result)?
                        }
                        Mode::BYTE => DecodeByteSegment(&mut bits, count, &mut result)?,
                        Mode::KANJI => DecodeKanjiSegment(&mut bits, count, &mut result)?,
                        _ => return Err(Exceptions::format_with("Invalid CodecMode")), //throw FormatError("Invalid CodecMode");
                    };
                }
            }
        }
        Ok(())
    })();

    Ok(DecoderResult::with_eci_string_builder(result)
        .withError(res.err())
        .withEcLevel(ecLevel.to_string())
        .withVersionNumber(version.getVersionNumber())
        .withStructuredAppend(structuredAppend)
        .withIsModel1(version.isModel1()))
}

pub fn Decode(bits: &BitMatrix) -> Result<DecoderResult<bool>> {
    if !Version::HasValidSize(bits) {
        return Err(Exceptions::format_with("Invalid symbol size"));
    }
    let Ok(formatInfo) = ReadFormatInformation(bits) else {
        return Err(Exceptions::format_with("Invalid format information"));
    };

    let Ok(pversion) = ReadVersion(bits, formatInfo.qr_type()) else {
        return Err(Exceptions::format_with("Invalid version"));
    };
    let version = pversion;

    let Ok(formatInfo) = ReadFormatInformation(bits) else {
        return Err(Exceptions::format_with("Invalid format information"));
    };

    // Read codewords
    let codewords = ReadCodewords(bits, version, &formatInfo)?;
    if codewords.is_empty() {
        return Err(Exceptions::format_with("Failed to read codewords"));
    }

    // Separate into data blocks
    let dataBlocks: Vec<DataBlock> =
        DataBlock::getDataBlocks(&codewords, version, formatInfo.error_correction_level)?;
    if dataBlocks.is_empty() {
        return Err(Exceptions::format_with("Failed to get data blocks"));
    }

    // Count total number of data bytes
    let op = |totalBytes, dataBlock: &DataBlock| totalBytes + dataBlock.getNumDataCodewords();
    let totalBytes = dataBlocks.iter().fold(0, op); // std::accumulate(std::begin(dataBlocks), std::end(dataBlocks), int{}, op);
    let mut resultBytes = vec![0u8; totalBytes as usize];
    let mut resultIterator = 0; //resultBytes.begin();

    // Error-correct and copy data blocks together into a stream of bytes
    for dataBlock in dataBlocks.iter() {
        let mut codewordBytes = dataBlock.getCodewords().to_vec();
        let numDataCodewords = dataBlock.getNumDataCodewords() as usize;

        if !CorrectErrors(&mut codewordBytes, numDataCodewords as u32)? {
            return Err(Exceptions::CHECKSUM);
        }

        // resultIterator = std::copy_n(codewordBytes.begin(), numDataCodewords, resultIterator);
        resultBytes[resultIterator..(resultIterator + numDataCodewords)]
            .copy_from_slice(&codewordBytes[..numDataCodewords]);
        resultIterator += numDataCodewords;
    }

    // Decode the contents of that stream of bytes
    Ok(
        DecodeBitStream(&resultBytes, version, formatInfo.error_correction_level)?
            .withIsMirrored(formatInfo.isMirrored),
    )
}

// } // namespace ZXing::QRCode
