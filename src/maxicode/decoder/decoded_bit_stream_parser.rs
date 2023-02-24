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

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    common::{DecoderRXingResult, Result},
    Exceptions,
};
use once_cell::sync::Lazy;

/**
 * <p>MaxiCodes can encode text or structured information as bits in one of several modes,
 * with multiple character sets in one code. This class decodes the bits back into text.</p>
 *
 * @author mike32767
 * @author Manuel Kasten
 */

const SHIFTA: &str = "\u{FFF0}";
const SHIFTB: &str = "\u{FFF1}";
const SHIFTC: &str = "\u{FFF2}";
const SHIFTD: &str = "\u{FFF3}";
const SHIFTE: &str = "\u{FFF4}";
const TWOSHIFTA: &str = "\u{FFF5}";
const THREESHIFTA: &str = "\u{FFF6}";
const LATCHA: &str = "\u{FFF7}";
const LATCHB: &str = "\u{FFF8}";
const LOCK: &str = "\u{FFF9}";
const ECI: &str = "\u{FFFA}";
const NS: &str = "\u{FFFB}";
const PAD: &str = "\u{FFFC}";
const FS: &str = "\u{001C}";
const GS: &str = "\u{001D}";
const RS: &str = "\u{001E}";
const COUNTRY_BYTES: [u8; 10] = [53, 54, 43, 44, 45, 46, 47, 48, 37, 38];
const SERVICE_CLASS_BYTES: [u8; 10] = [55, 56, 57, 58, 59, 60, 49, 50, 51, 52];
const POSTCODE_2_LENGTH_BYTES: [u8; 6] = [39, 40, 41, 42, 31, 32];
const POSTCODE_2_BYTES: [u8; 30] = [
    33, 34, 35, 36, 25, 26, 27, 28, 29, 30, 19, 20, 21, 22, 23, 24, 13, 14, 15, 16, 17, 18, 7, 8,
    9, 10, 11, 12, 1, 2,
];
const POSTCODE_3_BYTES: [[u8; 6]; 6] = [
    [39, 40, 41, 42, 31, 32],
    [33, 34, 35, 36, 25, 26],
    [27, 28, 29, 30, 19, 20],
    [21, 22, 23, 24, 13, 14],
    [15, 16, 17, 18, 7, 8],
    [9, 10, 11, 12, 1, 2],
];

static SETS: Lazy<[String; 5]> = Lazy::new(|| {
    [
    format!("\rABCDEFGHIJKLMNOPQRSTUVWXYZ{ECI}{FS}{GS}{RS}{NS} {PAD}\"#$%&'()*+,-./0123456789:{SHIFTB}{SHIFTC}{SHIFTD}{SHIFTE}{LATCHB}"           ),
    format!("`abcdefghijklmnopqrstuvwxyz{ECI}{FS}{GS}{RS}{NS}{{{PAD}}}~\u{007F};<=>?[\\]^_ ,./:@!|{PAD}{TWOSHIFTA}{THREESHIFTA}{PAD}{SHIFTA}{SHIFTC}{SHIFTD}{SHIFTE}{LATCHA}"               ),
        format!("\u{00C0}\u{00C1}\u{00C2}\u{00C3}\u{00C4}\u{00C5}\u{00C6}\u{00C7}\u{00C8}\u{00C9}\u{00CA}\u{00CB}\u{00CC}\u{00CD}\u{00CE}\u{00CF}\u{00D0}\u{00D1}\u{00D2}\u{00D3}\u{00D4}\u{00D5}\u{00D6}\u{00D7}\u{00D8}\u{00D9}\u{00DA}{}{}{}{}{}{}{}{}{}{}{}{}" ,
        ECI , FS , GS , RS , NS ,
        "\u{00DB}\u{00DC}\u{00DD}\u{00DE}\u{00DF}\u{00AA}\u{00AC}\u{00B1}\u{00B2}\u{00B3}\u{00B5}\u{00B9}\u{00BA}\u{00BC}\u{00BD}\u{00BE}\u{0080}\u{0081}\u{0082}\u{0083}\u{0084}\u{0085}\u{0086}\u{0087}\u{0088}\u{0089}" ,
        LATCHA , ' ' , LOCK , SHIFTD , SHIFTE , LATCHB),
    format!("\u{00E0}\u{00E1}\u{00E2}\u{00E3}\u{00E4}\u{00E5}\u{00E6}\u{00E7}\u{00E8}\u{00E9}\u{00EA}\u{00EB}\u{00EC}\u{00ED}\u{00EE}\u{00EF}\u{00F0}\u{00F1}\u{00F2}\u{00F3}\u{00F4}\u{00F5}\u{00F6}\u{00F7}\u{00F8}\u{00F9}\u{00FA}{}{}{}{}{}{}{}{}{}{}{}{}" ,
        ECI , FS , GS , RS , NS ,
        "\u{00FB}\u{00FC}\u{00FD}\u{00FE}\u{00FF}\u{00A1}\u{00A8}\u{00AB}\u{00AF}\u{00B0}\u{00B4}\u{00B7}\u{00B8}\u{00BB}\u{00BF}\u{008A}\u{008B}\u{008C}\u{008D}\u{008E}\u{008F}\u{0090}\u{0091}\u{0092}\u{0093}\u{0094}" ,
        LATCHA , ' ' , SHIFTC , LOCK , SHIFTE , LATCHB),
    format!("\u{0000}\u{0001}\u{0002}\u{0003}\u{0004}\u{0005}\u{0006}\u{0007}\u{0008}\u{0009}\n\u{000B}\u{000C}\r\u{000E}\u{000F}\u{0010}\u{0011}\u{0012}\u{0013}\u{0014}\u{0015}\u{0016}\u{0017}\u{0018}\u{0019}\u{001A}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}" ,
        ECI , PAD , PAD , '\u{001B}' , NS , FS , GS , RS ,
        "\u{001F}\u{009F}\u{00A0}\u{00A2}\u{00A3}\u{00A4}\u{00A5}\u{00A6}\u{00A7}\u{00A9}\u{00AD}\u{00AE}\u{00B6}\u{0095}\u{0096}\u{0097}\u{0098}\u{0099}\u{009A}\u{009B}\u{009C}\u{009D}\u{009E}" ,
        LATCHA , ' ' , SHIFTC , SHIFTD , LOCK , LATCHB),
   ]
});

pub fn decode(bytes: &[u8], mode: u8) -> Result<DecoderRXingResult> {
    let mut result = String::with_capacity(144);
    match mode {
        2 | 3 => {
            let postcode = if mode == 2 {
                let pc = getPostCode2(bytes);
                let ps2Length = getPostCode2Length(bytes) as usize;
                if ps2Length > 10 {
                    return Err(Exceptions::format);
                }
                // NumberFormat df = new DecimalFormat("0000000000".substring(0, ps2Length));
                // postcode = df.format(pc);
                format!("{pc:0>ps2Length$}")
            } else {
                getPostCode3(bytes)
            };
            // NumberFormat threeDigits = new DecimalFormat("000");
            // let country = threeDigits.format(getCountry(bytes));
            // let service = threeDigits.format(getServiceClass(bytes));
            let country = format!("{:0>3}", getCountry(bytes)); //threeDigits.format(getCountry(bytes));
            let service = format!("{:0>3}", getServiceClass(bytes));
            result.push_str(&getMessage(bytes, 10, 84));
            if result.starts_with(&format!("[)>{}{}{}", RS, "01", GS)) {
                result.insert_str(9, &format!("{postcode}{GS}{country}{GS}{service}{GS}"));
            } else {
                result.insert_str(0, &format!("{postcode}{GS}{country}{GS}{service}{GS}"));
            }
        }
        4 => {
            result.push_str(&getMessage(bytes, 1, 93));
        }
        5 => {
            result.push_str(&getMessage(bytes, 1, 77));
        }
        _ => {}
    }
    Ok(DecoderRXingResult::new(
        bytes.to_vec(),
        result,
        Vec::new(),
        mode.to_string(),
    ))
}

fn getBit(bit: u8, bytes: &[u8]) -> u8 {
    let bit = bit - 1;
    u8::from((bytes[bit as usize / 6] & (1 << (5 - (bit % 6)))) != 0)
    // if (bytes[bit as usize / 6] & (1 << (5 - (bit % 6)))) == 0 {
    //     0
    // } else {
    //     1
    // }
}

fn getInt(bytes: &[u8], x: &[u8]) -> u32 {
    let mut val: u32 = 0;
    for i in 0..x.len() {
        // for (int i = 0; i < x.length; i++) {
        val += (getBit(x[i], bytes) as u32) << ((x.len() - i - 1) as u32);
    }
    val
}

fn getCountry(bytes: &[u8]) -> u32 {
    getInt(bytes, &COUNTRY_BYTES)
}

fn getServiceClass(bytes: &[u8]) -> u32 {
    getInt(bytes, &SERVICE_CLASS_BYTES)
}

fn getPostCode2Length(bytes: &[u8]) -> u32 {
    getInt(bytes, &POSTCODE_2_LENGTH_BYTES)
}

fn getPostCode2(bytes: &[u8]) -> u32 {
    getInt(bytes, &POSTCODE_2_BYTES)
}

fn getPostCode3(bytes: &[u8]) -> String {
    let mut sb = String::with_capacity(POSTCODE_3_BYTES.len());
    let mut graphemes = SETS[0].graphemes(true);
    for p3bytes in &POSTCODE_3_BYTES {
        // for (byte[] p3bytes : POSTCODE_3_BYTES) {
        if let Some(c) = graphemes.nth(getInt(bytes, p3bytes) as usize) {
            sb.push_str(c);
        }
    }
    sb
}

fn getMessage(bytes: &[u8], start: u32, len: u32) -> String {
    let mut sb = String::new();
    let mut shift = -1;
    let mut set = 0;
    let mut lastset = 0;

    let mut i = start;
    while i < start + len {
        // for i in start..(start+len) {
        // for (int i = start; i < start + len; i++) {
        let mut set_graphemes = SETS[set].graphemes(true);
        let Some(c) = set_graphemes.nth(bytes[i as usize] as usize) else { break; };
        match c {
            LATCHA => {
                set = 0;
                shift = -1;
            }
            LATCHB => {
                set = 1;
                shift = -1;
            }
            SHIFTA | SHIFTB | SHIFTC | SHIFTD | SHIFTE => {
                lastset = set;
                // set = c - SHIFTA;
                set = subtract_two_single_char_strings(c, SHIFTA);
                shift = 1;
            }
            TWOSHIFTA => {
                lastset = set;
                set = 0;
                shift = 2;
            }
            THREESHIFTA => {
                lastset = set;
                set = 0;
                shift = 3;
            }
            NS => {
                // let nsval = (bytes[++i] << 24) + (bytes[++i] << 18) + (bytes[++i] << 12) + (bytes[++i] << 6) + bytes[++i];
                i += 1;
                let mut nsval = (bytes[i as usize] as u32) << 24;
                i += 1;
                nsval += (bytes[i as usize] as u32) << 18;
                i += 1;
                nsval += (bytes[i as usize] as u32) << 12;
                i += 1;
                nsval += (bytes[i as usize] as u32) << 6;
                i += 1;
                nsval += bytes[i as usize] as u32;
                // sb.append(new DecimalFormat("000000000").format(nsval));},
                sb.push_str(&format!("{nsval:0>9}"));
            }
            LOCK => {
                shift = -1;
            }
            _ => {
                sb.push_str(c);
            }
        }
        if shift == 0 {
            set = lastset;
        }
        shift -= 1;
        i += 1;
    }

    String::from(sb.trim_end_matches(PAD))
}

fn subtract_two_single_char_strings(str1: &str, str2: &str) -> usize {
    let str1_bytes = str1.as_bytes();
    let str2_bytes = str2.as_bytes();

    let str1_u16 =
        ((str1_bytes[0] as usize) << 4) + ((str1_bytes[1] as usize) << 2) + str1_bytes[2] as usize;
    let str2_u16 =
        ((str2_bytes[0] as usize) << 4) + ((str2_bytes[1] as usize) << 2) + str2_bytes[2] as usize;

    str1_u16 - str2_u16
}
