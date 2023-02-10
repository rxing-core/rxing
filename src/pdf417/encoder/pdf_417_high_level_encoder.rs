/*
 * Copyright 2006 Jeremias Maerki in part, and ZXing Authors in part
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/*
 * This file has been modified from its original form in Barcode4J.
 */

use std::{any::TypeId, fmt::Display};

use encoding::EncodingRef;

use crate::{
    common::{CharacterSetECI, ECIInput, MinimalECIInput},
    Exceptions,
};

use super::Compaction;

/**
 * PDF417 high-level encoder following the algorithm described in ISO/IEC 15438:2001(E) in
 * annex P.
 */

/**
 * code for Text compaction
 */
const TEXT_COMPACTION: u32 = 0;

/**
 * code for Byte compaction
 */
const BYTE_COMPACTION: u32 = 1;

/**
 * code for Numeric compaction
 */
const NUMERIC_COMPACTION: u32 = 2;

/**
 * Text compaction submode Alpha
 */
const SUBMODE_ALPHA: u32 = 0;

/**
 * Text compaction submode Lower
 */
const SUBMODE_LOWER: u32 = 1;

/**
 * Text compaction submode Mixed
 */
const SUBMODE_MIXED: u32 = 2;

/**
 * Text compaction submode Punctuation
 */
const SUBMODE_PUNCTUATION: u32 = 3;

/**
 * mode latch to Text Compaction mode
 */
const LATCH_TO_TEXT: u32 = 900;

/**
 * mode latch to Byte Compaction mode (number of characters NOT a multiple of 6)
 */
const LATCH_TO_BYTE_PADDED: u32 = 901;

/**
 * mode latch to Numeric Compaction mode
 */
const LATCH_TO_NUMERIC: u32 = 902;

/**
 * mode shift to Byte Compaction mode
 */
const SHIFT_TO_BYTE: u32 = 913;

/**
 * mode latch to Byte Compaction mode (number of characters a multiple of 6)
 */
const LATCH_TO_BYTE: u32 = 924;

/**
 * identifier for a user defined Extended Channel Interpretation (ECI)
 */
const ECI_USER_DEFINED: u32 = 925;

/**
 * identifier for a general purpose ECO format
 */
const ECI_GENERAL_PURPOSE: u32 = 926;

/**
 * identifier for an ECI of a character set of code page
 */
const ECI_CHARSET: u32 = 927;

/**
 * Raw code table for text compaction Mixed sub-mode
 */
const TEXT_MIXED_RAW: [u8; 30] = [
    48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 38, 13, 9, 44, 58, 35, 45, 46, 36, 47, 43, 37, 42, 61,
    94, 0, 32, 0, 0, 0,
];

/**
 * Raw code table for text compaction: Punctuation sub-mode
 */
const TEXT_PUNCTUATION_RAW: [u8; 30] = [
    59, 60, 62, 64, 91, 92, 93, 95, 96, 126, 33, 13, 9, 44, 58, 10, 45, 46, 36, 47, 34, 124, 42,
    40, 41, 63, 123, 125, 39, 0,
];

const DEFAULT_ENCODING: EncodingRef = encoding::all::ISO_8859_1; //StandardCharsets.ISO_8859_1;

const MIXED: [i8; 128] = {
    let mut mixed = [-1_i8; 128];
    let mut i = 0;
    while i < TEXT_MIXED_RAW.len() {
        let b = TEXT_MIXED_RAW[i] as usize;
        if b > 0 {
            mixed[b] = i as i8;
        }
        i += 1;
    }

    mixed
};

const PUNCTUATION: [i8; 128] = {
    let mut punct = [-1_i8; 128];
    let mut i = 0;
    while i < TEXT_PUNCTUATION_RAW.len() {
        let b = TEXT_PUNCTUATION_RAW[i] as usize;
        if b > 0 {
            punct[b] = i as i8;
        }
        i += 1;
    }

    punct
};

/**
 * Performs high-level encoding of a PDF417 message using the algorithm described in annex P
 * of ISO/IEC 15438:2001(E). If byte compaction has been selected, then only byte compaction
 * is used.
 *
 * @param msg the message
 * @param compaction compaction mode to use
 * @param encoding character encoding used to encode in default or byte compaction
 *  or {@code null} for default / not applicable
 * @param autoECI encode input minimally using multiple ECIs if needed
 *   If autoECI encoding is specified and additionally {@code encoding} is specified, then the encoder
 *   will use the specified {@link Charset} for any character that can be encoded by it, regardless
 *   if a different encoding would lead to a more compact encoding. When no {@code encoding} is specified
 *   then charsets will be chosen so that the byte representation is minimal.
 * @return the encoded message (the char values range from 0 to 928)
 */
pub fn encodeHighLevel(
    msg: &str,
    compaction: Compaction,
    encoding: Option<EncodingRef>,
    autoECI: bool,
) -> Result<String, Exceptions> {
    let mut encoding = encoding;
    if msg.is_empty() {
        return Err(Exceptions::WriterException(Some(
            "Empty message not allowed".to_owned(),
        )));
    }

    if encoding.is_none() && !autoECI {
            for ch in msg.chars() {
            if ch as u32 > 255 {
                return Err(Exceptions::WriterException(Some(format!("Non-encodable character detected: {} (Unicode: {}). Consider specifying EncodeHintType.PDF417_AUTO_ECI and/or EncodeTypeHint.CHARACTER_SET.",ch as u32,ch))));
            }
        }
    }
    //the codewords 0..928 are encoded as Unicode characters
    let mut sb = String::with_capacity(msg.len());

    let input: Box<dyn ECIInput>;
    if autoECI {
        input = Box::new(MinimalECIInput::new(msg, encoding, None));
    } else {
        input = Box::new(NoECIInput::new(msg.to_owned()));
        if encoding.is_none() {
            encoding = Some(DEFAULT_ENCODING);
        } else if DEFAULT_ENCODING.name() != encoding.as_ref().ok_or(Exceptions::IllegalStateException(None))?.name() {
            if let Some(eci) = CharacterSetECI::getCharacterSetECI(encoding.ok_or(Exceptions::IllegalStateException(None))?) {
                encodingECI(CharacterSetECI::getValue(&eci) as i32, &mut sb)?;
            }
        }
    }

    let len = input.length();
    let mut p = 0;
    let mut textSubMode = SUBMODE_ALPHA;

    // User selected encoding mode
    match compaction {
        Compaction::TEXT => {
            encodeText(&input, p, len as u32, &mut sb, textSubMode)?;
        }
        Compaction::BYTE if autoECI => {
            encodeMultiECIBinary(&input, 0, input.length() as u32, TEXT_COMPACTION, &mut sb)?
        }
        Compaction::BYTE => {
            let msgBytes = encoding
                .as_ref()
                .ok_or(Exceptions::IllegalStateException(None))?
                .encode(&input.to_string(), encoding::EncoderTrap::Strict)
                .unwrap_or_default(); //input.to_string().getBytes(encoding);
            encodeBinary(
                &msgBytes,
                p,
                msgBytes.len() as u32,
                BYTE_COMPACTION,
                &mut sb,
            );
        }
        Compaction::NUMERIC => {
            sb.push(char::from_u32(LATCH_TO_NUMERIC).ok_or(Exceptions::ParseException(None))?);
            encodeNumeric(&input, p, len as u32, &mut sb);
        }
        _ => {
            let mut encodingMode = TEXT_COMPACTION; //Default mode, see 4.4.2.1
            while p < len as u32 {
                while p < len as u32 && input.isECI(p)? {
                    encodingECI(input.getECIValue(p as usize)?, &mut sb)?;
                    p += 1;
                }
                if p >= len as u32 {
                    break;
                }
                let n = determineConsecutiveDigitCount(&input, p);
                if n >= 13 {
                    sb.push(char::from_u32(LATCH_TO_NUMERIC).ok_or(Exceptions::ParseException(None))?);
                    encodingMode = NUMERIC_COMPACTION;
                    textSubMode = SUBMODE_ALPHA; //Reset after latch
                    encodeNumeric(&input, p, n, &mut sb);
                    p += n;
                } else {
                    let t = determineConsecutiveTextCount(&input, p);
                    if t >= 5 || n == len as u32 {
                        if encodingMode != TEXT_COMPACTION {
                            sb.push(char::from_u32(LATCH_TO_TEXT).ok_or(Exceptions::ParseException(None))?);
                            encodingMode = TEXT_COMPACTION;
                            textSubMode = SUBMODE_ALPHA; //start with submode alpha after latch
                        }
                        textSubMode = encodeText(&input, p, t, &mut sb, textSubMode)?;
                        p += t;
                    } else {
                        let mut b = determineConsecutiveBinaryCount(
                            &input,
                            p,
                            if autoECI { None } else { encoding },
                        )?;
                        if b == 0 {
                            b = 1;
                        }
                        let bytes = if autoECI {
                            None
                        } else {
                            let str = input
                                .subSequence(p as usize, (p + b) as usize)?
                                .iter()
                                .collect::<String>();
                            if let Ok(enc_str) = encoding
                                .as_ref()
                                .ok_or(Exceptions::IllegalStateException(None))?
                                .encode(&str, encoding::EncoderTrap::Strict)
                            {
                                Some(enc_str)
                            } else {
                                None
                            }
                        }; 
                        
                        let bytes_ok = bytes.is_some(); 
                        if (bytes_ok && b == 1)
                            && (encodingMode == TEXT_COMPACTION)
                        {
                            //Switch for one byte (instead of latch)
                            if autoECI {
                                encodeMultiECIBinary(&input, p, 1, TEXT_COMPACTION, &mut sb)?;
                            } else {
                                encodeBinary(
                                    bytes.as_ref().ok_or(Exceptions::IllegalStateException(None))?,
                                    0,
                                    1,
                                    TEXT_COMPACTION,
                                    &mut sb,
                                );
                            }
                        } else {
                            //Mode latch performed by encodeBinary()
                            if autoECI {
                                encodeMultiECIBinary(&input, p, p + b, encodingMode, &mut sb)?;
                            } else {
                                encodeBinary(
                                    bytes.as_ref().ok_or(Exceptions::IllegalStateException(None))?,
                                    0,
                                    bytes.as_ref().ok_or(Exceptions::IllegalStateException(None))?.len() as u32,
                                    encodingMode,
                                    &mut sb,
                                );
                            }
                            encodingMode = BYTE_COMPACTION;
                            textSubMode = SUBMODE_ALPHA; //Reset after latch
                        }
                        p += b;
                    }
                }
            }
        }
    }

    Ok(sb)
}

/**
 * Encode parts of the message using Text Compaction as described in ISO/IEC 15438:2001(E),
 * chapter 4.4.2.
 *
 * @param input          the input
 * @param startpos       the start position within the message
 * @param count          the number of characters to encode
 * @param sb             receives the encoded codewords
 * @param initialSubmode should normally be SUBMODE_ALPHA
 * @return the text submode in which this method ends
 */
fn encodeText<T: ECIInput + ?Sized>(
    input: &Box<T>,
    startpos: u32,
    count: u32,
    sb: &mut String,
    initialSubmode: u32,
) -> Result<u32, Exceptions> {
    let mut tmp = String::with_capacity(count as usize);
    let mut submode = initialSubmode;
    let mut idx = 0;
    loop {
        if input.isECI(startpos + idx)? {
            encodingECI(input.getECIValue((startpos + idx) as usize)?, sb)?;
            idx += 1;
        } else {
            let ch = input.charAt((startpos + idx) as usize)?;
            match submode {
                SUBMODE_ALPHA => {
                    if isAlphaUpper(ch) {
                        if ch == ' ' {
                            tmp.push(26 as char); //space
                        } else {
                            tmp.push(char::from_u32(ch as u32 - 65).ok_or(Exceptions::ParseException(None))?);
                        }
                    } else if isAlphaLower(ch) {
                        submode = SUBMODE_LOWER;
                        tmp.push(27 as char); //ll
                        continue;
                    } else if isMixed(ch) {
                        submode = SUBMODE_MIXED;
                        tmp.push(28 as char); //ml
                        continue;
                    } else {
                        tmp.push(29 as char); //ps
                        tmp.push(char::from_u32(PUNCTUATION[ch as usize] as u32).ok_or(Exceptions::ParseException(None))?);
                    }
                }
                
                SUBMODE_LOWER => {
                    if isAlphaLower(ch) {
                        if ch == ' ' {
                            tmp.push(26 as char); //space
                        } else {
                            tmp.push(char::from_u32(ch as u32 - 97).ok_or(Exceptions::ParseException(None))?);
                        }
                    } else if isAlphaUpper(ch) {
                        tmp.push(27 as char); //as
                        tmp.push(char::from_u32(ch as u32 - 65).ok_or(Exceptions::ParseException(None))?);
                        //space cannot happen here, it is also in "Lower"
                    } else if isMixed(ch) {
                        submode = SUBMODE_MIXED;
                        tmp.push(28 as char); //ml
                        continue;
                    } else {
                        tmp.push(29 as char); //ps
                        tmp.push(char::from_u32(PUNCTUATION[ch as usize] as u32).ok_or(Exceptions::ParseException(None))?);
                    }
                }
                // break;
                SUBMODE_MIXED => {
                    if isMixed(ch) {
                        tmp.push(char::from_u32(MIXED[ch as usize] as u32).ok_or(Exceptions::ParseException(None))?);
                    } else if isAlphaUpper(ch) {
                        submode = SUBMODE_ALPHA;
                        tmp.push(28 as char); //al
                        continue;
                    } else if isAlphaLower(ch) {
                        submode = SUBMODE_LOWER;
                        tmp.push(27 as char); //ll
                        continue;
                    } else {
                        if startpos + idx + 1 < count
                            && !input.isECI(startpos + idx + 1)?
                            && isPunctuation(input.charAt((startpos + idx + 1) as usize)?)
                        {
                            submode = SUBMODE_PUNCTUATION;
                            tmp.push(25 as char); //pl
                            continue;
                        }
                        tmp.push(29 as char); //ps
                        tmp.push(char::from_u32(PUNCTUATION[ch as usize] as u32).ok_or(Exceptions::ParseException(None))?);
                    }
                }
                _ =>
                //SUBMODE_PUNCTUATION
                {
                    if isPunctuation(ch) {
                        tmp.push(char::from_u32(PUNCTUATION[ch as usize] as u32).ok_or(Exceptions::ParseException(None))?);
                    } else {
                        submode = SUBMODE_ALPHA;
                        tmp.push(29 as char); //al
                        continue;
                    }
                }
            }
            idx += 1;
            if idx >= count {
                break;
            }
        }
    }
    let mut h = 0 as char;
    let len = tmp.chars().count();
    for i in 0..len {
        let odd = (i % 2) != 0;
        if odd {
            h = char::from_u32((h as u32 * 30) + tmp.chars().nth(i).ok_or(Exceptions::IndexOutOfBoundsException(None))? as u32).ok_or(Exceptions::ParseException(None))?;
            sb.push(h);
        } else {
            h = tmp.chars().nth(i).ok_or(Exceptions::IndexOutOfBoundsException(None))?;
        }
    }
    if (len % 2) != 0 {
        sb.push(char::from_u32((h as u32 * 30) + 29).ok_or(Exceptions::ParseException(None))?); //ps
    }
    Ok(submode)
}

/**
 * Encode all of the message using Byte Compaction as described in ISO/IEC 15438:2001(E)
 *
 * @param input     the input
 * @param startpos  the start position within the message
 * @param count     the number of bytes to encode
 * @param startmode the mode from which this method starts
 * @param sb        receives the encoded codewords
 */
fn encodeMultiECIBinary<T: ECIInput + ?Sized>(
    input: &Box<T>,
    startpos: u32,
    count: u32,
    startmode: u32,
    sb: &mut String,
) -> Result<(), Exceptions> {
    let end = (startpos + count).min(input.length() as u32);
    let mut localStart = startpos;
    loop {
        //encode all leading ECIs and advance localStart
        while localStart < end && input.isECI(localStart)? {
            encodingECI(input.getECIValue(localStart as usize)?, sb)?;
            localStart += 1;
        }
        let mut localEnd = localStart;
        //advance end until before the next ECI
        while localEnd < end && !input.isECI(localEnd)? {
            localEnd += 1;
        }

        let localCount = localEnd as i32 - localStart as i32;
        if localCount <= 0 {
            //done
            break;
        } else {
            //encode the segment
            encodeBinary(
                &subBytes(input, localStart, localEnd)?,
                0,
                localCount as u32,
                if localStart == startpos {
                    startmode
                } else {
                    BYTE_COMPACTION
                },
                sb,
            );
            localStart = localEnd;
        }
    }

    Ok(())
}

pub fn subBytes<T: ECIInput + ?Sized>(input: &Box<T>, start: u32, end: u32) -> Result<Vec<u8>,Exceptions> {
    let count = (end - start) as usize;
    let mut result = vec![0_u8; count];
    for i in start as usize..end as usize {
        result[i - start as usize] = input.charAt(i)? as u8;
    }
    Ok(result)
}

/**
 * Encode parts of the message using Byte Compaction as described in ISO/IEC 15438:2001(E),
 * chapter 4.4.3. The Unicode characters will be converted to binary using the cp437
 * codepage.
 *
 * @param bytes     the message converted to a byte array
 * @param startpos  the start position within the message
 * @param count     the number of bytes to encode
 * @param startmode the mode from which this method starts
 * @param sb        receives the encoded codewords
 */
fn encodeBinary(bytes: &[u8], startpos: u32, count: u32, startmode: u32, sb: &mut String) -> Result<(),Exceptions> {
    if count == 1 && startmode == TEXT_COMPACTION {
        sb.push(char::from_u32(SHIFT_TO_BYTE).unwrap());
    } else if (count % 6) == 0 {
        sb.push(char::from_u32(LATCH_TO_BYTE).unwrap());
    } else {
        sb.push(char::from_u32(LATCH_TO_BYTE_PADDED).unwrap());
    }

    let mut idx = startpos;
    // Encode sixpacks
    if count >= 6 {
        let mut chars = [0 as char; 5]; //new char[5];
        while (startpos + count - idx) >= 6 {
            let mut t: i64 = 0;
            for i in 0..6 {
                // for (int i = 0; i < 6; i++) {
                t <<= 8;
                t += bytes[idx as usize + i as usize] as i64;
            }
            for ch in &mut chars {
                // for i in 0..5 {
                // for (int i = 0; i < 5; i++) {
                *ch = char::from_u32((t % 900) as u32).unwrap();
                t /= 900;
            }
            for ch in chars.into_iter().rev() {
                // for i in (0..chars.len()).rev() {
                // for (int i = chars.length - 1; i >= 0; i--) {
                sb.push(ch);
            }
            idx += 6;
        }
    }
    //Encode rest (remaining n<5 bytes if any)
    for i in idx..(startpos + count) {
        // for (int i = idx; i < startpos + count; i++) {
        let ch = bytes[i as usize];
        sb.push(ch as char);
    }

    Ok(())
}

fn encodeNumeric<T: ECIInput + ?Sized>(input: &Box<T>, startpos: u32, count: u32, sb: &mut String) {
    let mut idx = 0;
    let mut tmp = String::with_capacity(count as usize / 3 + 1);
    let num900: u128 = 900;
    let num0: u128 = 0;
    while idx < count {
        tmp.clear();
        let len = 44.min(count as isize - idx as isize);
        let part = format!(
            "1{}",
            input
                .subSequence(
                    (startpos + idx) as usize,
                    (startpos + idx + len as u32) as usize
                )
                .unwrap()
                .iter()
                .collect::<String>()
        );
        let mut bigint: u128 = part.parse().unwrap();
        loop {
            tmp.push(char::from_u32((bigint % num900) as u32).unwrap());
            bigint /= num900;

            if bigint == num0 {
                break;
            }
        } //while (!bigint.equals(num0));

        //Reverse temporary string
        let mut i = tmp.chars().count() as isize - 1;
        while i >= 0 {
            // for (int i = tmp.length() - 1; i >= 0; i--) {
            sb.push(tmp.chars().nth(i as usize).unwrap());

            i -= 1;
        }
        idx += len as u32;
    }
}

fn isDigit(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}

fn isAlphaUpper(ch: char) -> bool {
    ch == ' ' || ('A'..='Z').contains(&ch)
}

fn isAlphaLower(ch: char) -> bool {
    ch == ' ' || ('a'..='z').contains(&ch)
}

fn isMixed(ch: char) -> bool {
    MIXED[ch as usize] != -1
}

fn isPunctuation(ch: char) -> bool {
    PUNCTUATION[ch as usize] != -1
}

fn isText(ch: char) -> bool {
    ch == '\t' || ch == '\n' || ch == '\r' || (ch as u32 >= 32 && ch as u32 <= 126)
}

/**
 * Determines the number of consecutive characters that are encodable using numeric compaction.
 *
 * @param input      the input
 * @param startpos the start position within the input
 * @return the requested character count
 */
fn determineConsecutiveDigitCount<T: ECIInput + ?Sized>(input: &Box<T>, startpos: u32) -> u32 {
    let mut count = 0;
    let len = input.length();
    let mut idx = startpos as usize;
    if idx < len {
        while idx < len && !input.isECI(idx as u32).unwrap() && isDigit(input.charAt(idx).unwrap())
        {
            count += 1;
            idx += 1;
        }
    }

    count
}

/**
 * Determines the number of consecutive characters that are encodable using text compaction.
 *
 * @param input      the input
 * @param startpos the start position within the input
 * @return the requested character count
 */
fn determineConsecutiveTextCount<T: ECIInput + ?Sized>(input: &Box<T>, startpos: u32) -> u32 {
    let len = input.length();
    let mut idx = startpos as usize;
    while idx < len {
        let mut numericCount = 0;
        while numericCount < 13
            && idx < len
            && !input.isECI(idx as u32).unwrap()
            && isDigit(input.charAt(idx).unwrap())
        {
            numericCount += 1;
            idx += 1;
        }
        if numericCount >= 13 {
            return (idx - startpos as usize - numericCount) as u32;
        }
        if numericCount > 0 {
            //Heuristic: All text-encodable chars or digits are binary encodable
            continue;
        }

        //Check if character is encodable
        if input.isECI(idx as u32).unwrap() || !isText(input.charAt(idx).unwrap()) {
            break;
        }
        idx += 1;
    }
    (idx - startpos as usize) as u32
}

/**
 * Determines the number of consecutive characters that are encodable using binary compaction.
 *
 * @param input    the input
 * @param startpos the start position within the message
 * @param encoding the charset used to convert the message to a byte array
 * @return the requested character count
 */
fn determineConsecutiveBinaryCount<T: ECIInput + ?Sized + 'static>(
    input: &Box<T>,
    startpos: u32,
    encoding: Option<EncodingRef>,
) -> Result<u32, Exceptions> {
    // CharsetEncoder encoder = encoding == null ? null : encoding.newEncoder();
    let len = input.length();
    let mut idx = startpos as usize;
    while idx < len {
        let mut numericCount = 0;

        let mut i = idx;
        while numericCount < 13
            && !input.isECI(i as u32).unwrap()
            && isDigit(input.charAt(i).unwrap())
        {
            numericCount += 1;
            //textCount++;
            i = idx + numericCount;
            if i >= len {
                break;
            }
        }
        if numericCount >= 13 {
            return Ok(idx as u32 - startpos);
        }

        if let Some(encoder) = encoding {
            let can_encode = encoder
                .encode(
                    &input.charAt(idx)?.to_string(),
                    encoding::EncoderTrap::Strict,
                )
                .is_ok();

            // if encoder != null && !encoder.canEncode(input.charAt(idx)) {
            if !can_encode {
                assert!(TypeId::of::<T>() == TypeId::of::<NoECIInput>());
                // assert!(input instanceof NoECIInput);
                let ch = input.charAt(idx).unwrap();
                return Err(Exceptions::WriterException(Some(format!(
                    "Non-encodable character detected: {} (Unicode: {})",
                    ch, ch as u32
                ))));
            }
        }
        idx += 1;
    }
    Ok(idx as u32 - startpos)
}

fn encodingECI(eci: i32, sb: &mut String) -> Result<(), Exceptions> {
    if (0..900).contains(&eci) {
        sb.push(char::from_u32(ECI_CHARSET).unwrap());
        sb.push(char::from_u32(eci as u32).unwrap());
    } else if eci < 810900 {
        sb.push(char::from_u32(ECI_GENERAL_PURPOSE).unwrap());
        sb.push(char::from_u32((eci / 900 - 1) as u32).unwrap());
        sb.push(char::from_u32((eci % 900) as u32).unwrap());
    } else if eci < 811800 {
        sb.push(char::from_u32(ECI_USER_DEFINED).unwrap());
        sb.push(char::from_u32((810900 - eci) as u32).unwrap());
    } else {
        return Err(Exceptions::WriterException(Some(format!(
            "ECI number not in valid range from 0..811799, but was {eci}"
        ))));
    }
    Ok(())
}

struct NoECIInput(String);
impl ECIInput for NoECIInput {
    fn length(&self) -> usize {
        self.0.chars().count()
    }

    fn charAt(&self, index: usize) -> Result<char, Exceptions> {
        self.0
            .chars()
            .nth(index)
            .ok_or(Exceptions::IndexOutOfBoundsException(None))
    }

    fn subSequence(&self, start: usize, end: usize) -> Result<Vec<char>, Exceptions> {
        let res: Vec<char> = self.0.chars().skip(start).take(end - start).collect();
        Ok(res)
    }

    fn isECI(&self, _index: u32) -> Result<bool, Exceptions> {
        Ok(false)
    }

    fn getECIValue(&self, _index: usize) -> Result<i32, Exceptions> {
        Ok(-1)
    }

    fn haveNCharacters(&self, index: usize, n: usize) -> Result<bool, Exceptions> {
        Ok(index + n <= self.0.len())
    }
}
impl NoECIInput {
    pub fn new(input: String) -> Self {
        Self(input)
    }
}
impl Display for NoECIInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/**
 * Tests {@link PDF417HighLevelEncoder}.
 */
#[cfg(test)]
mod PDF417EncoderTestCase {
    use crate::pdf417::encoder::{pdf_417_high_level_encoder::encodeHighLevel, Compaction};

    #[test]
    fn testEncodeAuto() {
        let encoded = encodeHighLevel("ABCD", Compaction::AUTO, Some(encoding::all::UTF_8), false)
            .expect("encode");
        assert_eq!("\u{039f}\u{001A}\u{0385}ABCD", encoded);
    }

    #[test]
    fn testEncodeAutoWithSpecialChars() {
        // Just check if this does not throw an exception
        encodeHighLevel(
            "1%§s ?aG$",
            Compaction::AUTO,
            Some(encoding::all::UTF_8),
            false,
        )
        .expect("encode");
    }

    #[test]
    fn testEncodeIso88591WithSpecialChars() {
        // Just check if this does not throw an exception
        encodeHighLevel(
            "asdfg§asd",
            Compaction::AUTO,
            Some(encoding::all::ISO_8859_1),
            false,
        )
        .expect("encode");
    }

    #[test]
    fn testEncodeText() {
        let encoded = encodeHighLevel("ABCD", Compaction::TEXT, Some(encoding::all::UTF_8), false)
            .expect("encode");
        assert_eq!("Ο\u{001A}\u{0001}?", encoded);
    }

    #[test]
    fn testEncodeNumeric() {
        let encoded = encodeHighLevel(
            "1234",
            Compaction::NUMERIC,
            Some(encoding::all::UTF_8),
            false,
        )
        .expect("encode");
        assert_eq!("\u{039f}\u{001A}\u{0386}\u{C}\u{01b2}", encoded);
        // converted \f to \u{0046}
    }

    #[test]
    fn testEncodeByte() {
        let encoded = encodeHighLevel("abcd", Compaction::BYTE, Some(encoding::all::UTF_8), false)
            .expect("encode");
        assert_eq!("\u{039f}\u{001A}\u{0385}abcd", encoded);
    }

    #[test]
    #[should_panic]
    fn testEncodeEmptyString() {
        encodeHighLevel("", Compaction::AUTO, None, false).expect("encode");
    }
}
