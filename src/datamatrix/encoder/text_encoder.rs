/*
 * Copyright 2006-2007 Jeremias Maerki.
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

use super::{high_level_encoder, C40Encoder, Encoder};

pub struct TextEncoder(C40Encoder);
impl Encoder for TextEncoder {
    fn getEncodingMode(&self) -> usize {
        high_level_encoder::TEXT_ENCODATION
    }

    fn encode(&self, context: &mut super::EncoderContext) -> Result<(), crate::Exceptions> {
        self.0
            .encode_with_encode_char_fn(context, &Self::encodeChar, &C40Encoder::handleEOD_c40, &||{self.getEncodingMode()})
    }
}
impl TextEncoder {
    pub fn new() -> Self {
        Self(C40Encoder::new())
    }
    fn encodeChar(c: char, sb: &mut String) -> u32 {
        if c == ' ' {
            sb.push('\u{3}');
            return 1;
        }
        if c >= '0' && c <= '9' {
            sb.push((c as u8 - 48 + 4) as char);
            return 1;
        }
        if c >= 'a' && c <= 'z' {
            sb.push((c as u8 - 97 + 14) as char);
            return 1;
        }
        if c < ' ' {
            sb.push('\0'); //Shift 1 Set
            sb.push(c);
            return 2;
        }
        if c <= '/' {
            sb.push('\u{1}'); //Shift 2 Set
            sb.push((c as u8 - 33) as char);
            return 2;
        }
        if c <= '@' {
            sb.push('\u{1}'); //Shift 2 Set
            sb.push((c as u8 - 58 + 15) as char);
            return 2;
        }
        if c >= '[' && c <= '_' {
            sb.push('\u{1}'); //Shift 2 Set
            sb.push((c as u8 - 91 + 22) as char);
            return 2;
        }
        if c == '`' {
            sb.push('\u{2}'); //Shift 3 Set
            sb.push(0 as char); // '`' - 96 == 0
            return 2;
        }
        if c <= 'Z' {
            sb.push('\u{2}'); //Shift 3 Set
            sb.push((c as u8 - 65 + 1) as char);
            return 2;
        }
        if c as u8 <= 127 {
            sb.push('\u{2}'); //Shift 3 Set
            sb.push((c as u8 - 123 + 27) as char);
            return 2;
        }
        sb.push_str("\u{1}\u{001e}"); //Shift 2, Upper Shift
        let mut len = 2;
        len += Self::encodeChar((c as u8 - 128) as char, sb);
        return len;
    }
}
