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

use crate::common::Result;
use crate::Exceptions;

use super::high_level_encoder::{
    self, ASCII_ENCODATION, C40_ENCODATION, C40_UNLATCH, LATCH_TO_C40,
};

use super::{Encoder, EncoderContext};

pub struct C40Encoder;

impl Encoder for C40Encoder {
    fn encode(&self, context: &mut super::EncoderContext) -> Result<()> {
        self.encode_with_encode_char_fn(
            context,
            &Self::encodeChar_c40,
            &Self::handleEOD_c40,
            &|| self.getEncodingMode(),
        )
    }

    fn getEncodingMode(&self) -> usize {
        C40_ENCODATION
    }
}

impl C40Encoder {
    pub fn new() -> Self {
        Self
    }

    pub(super) fn encode_with_encode_char_fn(
        &self,
        context: &mut super::EncoderContext,
        encodeChar: &dyn Fn(char, &mut String) -> u32,
        handleEOD: &dyn Fn(&mut EncoderContext, &mut String) -> Result<()>,
        getEncodingMode: &dyn Fn() -> usize,
    ) -> Result<()> {
        //step C
        let mut buffer = String::new();
        while context.hasMoreCharacters() {
            let c = context.getCurrentChar();
            context.pos += 1;

            let mut lastCharSize = encodeChar(c, &mut buffer);

            let unwritten = (buffer.chars().count() / 3) * 2;

            let curCodewordCount = context.getCodewordCount() + unwritten;
            context.updateSymbolInfoWithLength(curCodewordCount);
            let available = context
                .getSymbolInfo()
                .ok_or(Exceptions::ILLEGAL_STATE)?
                .getDataCapacity() as usize
                - curCodewordCount;

            if !context.hasMoreCharacters() {
                //Avoid having a single C40 value in the last triplet
                let mut removed = String::new();
                if (buffer.chars().count() % 3) == 2 && available != 2 {
                    lastCharSize = self.backtrackOneCharacter(
                        context,
                        &mut buffer,
                        &mut removed,
                        lastCharSize,
                        encodeChar,
                    );
                }
                while (buffer.chars().count() % 3) == 1 && (lastCharSize > 3 || available != 1) {
                    lastCharSize = self.backtrackOneCharacter(
                        context,
                        &mut buffer,
                        &mut removed,
                        lastCharSize,
                        encodeChar,
                    );
                }
                break;
            }

            let count = buffer.chars().count();
            if (count % 3) == 0 {
                let newMode = high_level_encoder::lookAheadTest(
                    context.getMessage(),
                    context.pos,
                    getEncodingMode() as u32,
                );
                if newMode != getEncodingMode() {
                    // Return to ASCII encodation, which will actually handle latch to new mode
                    context.signalEncoderChange(ASCII_ENCODATION);
                    break;
                }
            }
        }
        handleEOD(context, &mut buffer)
    }

    pub fn encodeMaximalC40(&self, context: &mut EncoderContext) -> Result<()> {
        self.encodeMaximal(context, &Self::encodeChar_c40, &Self::handleEOD_c40)
    }

    fn encodeMaximal(
        &self,
        context: &mut EncoderContext,
        encodeChar: &dyn Fn(char, &mut String) -> u32,
        handleEOD: &dyn Fn(&mut EncoderContext, &mut String) -> Result<()>,
    ) -> Result<()> {
        let mut buffer = String::new();
        let mut lastCharSize = 0;
        let mut backtrackStartPosition = context.pos;
        let mut backtrackBufferLength = 0;
        while context.hasMoreCharacters() {
            let c = context.getCurrentChar();
            context.pos += 1;
            lastCharSize = encodeChar(c, &mut buffer);
            if buffer.chars().count() % 3 == 0 {
                backtrackStartPosition = context.pos;
                backtrackBufferLength = buffer.chars().count();
            }
        }
        if backtrackBufferLength != buffer.chars().count() {
            let unwritten = (buffer.chars().count() / 3) * 2;

            let curCodewordCount = context.getCodewordCount() + unwritten + 1; // +1 for the latch to C40
            context.updateSymbolInfoWithLength(curCodewordCount);
            let available = context
                .getSymbolInfo()
                .ok_or(Exceptions::ILLEGAL_STATE)?
                .getDataCapacity() as usize
                - curCodewordCount;
            let rest = buffer.chars().count() % 3;
            if (rest == 2 && available != 2) || (rest == 1 && (lastCharSize > 3 || available != 1))
            {
                buffer.truncate(backtrackBufferLength);
                // buffer.setLength(backtrackBufferLength);
                context.pos = backtrackStartPosition;
            }
        }
        if buffer.chars().count() > 0 {
            context.writeCodeword(LATCH_TO_C40);
        }

        handleEOD(context, &mut buffer)?;

        Ok(())
    }

    fn backtrackOneCharacter(
        &self,
        context: &mut EncoderContext,
        buffer: &mut String,
        removed: &mut String,
        lastCharSize: u32,
        encodeChar: &dyn Fn(char, &mut String) -> u32,
    ) -> u32 {
        let count = buffer.chars().count();
        // buffer.delete(count - lastCharSize, count);
        buffer.replace_range((count - lastCharSize as usize)..count, "");
        context.pos -= 1;
        let c = context.getCurrentChar();
        let lastCharSize = encodeChar(c, removed);
        context.resetSymbolInfo(); //Deal with possible reduction in symbol size
        lastCharSize
    }

    pub(super) fn writeNextTriplet(
        context: &mut EncoderContext,
        buffer: &mut String,
    ) -> Result<()> {
        context.writeCodewords(&Self::encodeToCodewords(buffer).ok_or(Exceptions::FORMAT)?);
        buffer.replace_range(0..3, "");
        // buffer.delete(0, 3);
        Ok(())
    }

    /**
     * Handle "end of data" situations
     *
     * @param context the encoder context
     * @param buffer  the buffer with the remaining encoded characters
     */
    pub fn handleEOD_c40(context: &mut EncoderContext, buffer: &mut String) -> Result<()> {
        let unwritten = (buffer.chars().count() / 3) * 2;
        let rest = buffer.chars().count() % 3;

        let curCodewordCount = context.getCodewordCount() + unwritten;
        context.updateSymbolInfoWithLength(curCodewordCount);
        let available = context
            .getSymbolInfo()
            .ok_or(Exceptions::ILLEGAL_STATE)?
            .getDataCapacity() as usize
            - curCodewordCount;

        if rest == 2 {
            buffer.push('\0'); //Shift 1
            while buffer.chars().count() >= 3 {
                C40Encoder::writeNextTriplet(context, buffer)?;
            }
            if context.hasMoreCharacters() {
                context.writeCodeword(C40_UNLATCH);
            }
        } else if available == 1 && rest == 1 {
            while buffer.chars().count() >= 3 {
                C40Encoder::writeNextTriplet(context, buffer)?;
            }
            if context.hasMoreCharacters() {
                context.writeCodeword(C40_UNLATCH);
            }
            // else no unlatch
            context.pos -= 1;
        } else if rest == 0 {
            while buffer.chars().count() >= 3 {
                C40Encoder::writeNextTriplet(context, buffer)?;
            }
            if available > 0 || context.hasMoreCharacters() {
                context.writeCodeword(C40_UNLATCH);
            }
        } else {
            return Err(Exceptions::illegal_state_with(
                "Unexpected case. Please report!",
            ));
        }
        context.signalEncoderChange(ASCII_ENCODATION);

        Ok(())
    }

    fn encodeChar_c40(c: char, sb: &mut String) -> u32 {
        if c == ' ' {
            sb.push('\u{3}');
            return 1;
        }
        if c.is_ascii_digit() {
            sb.push((c as u8 - 48 + 4) as char);
            return 1;
        }
        if c.is_ascii_uppercase() {
            sb.push((c as u8 - 65 + 14) as char);
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
        if c <= '_' {
            sb.push('\u{1}'); //Shift 2 Set
            sb.push((c as u8 - 91 + 22) as char);
            return 2;
        }
        if (c as u8) <= 127 {
            sb.push('\u{2}'); //Shift 3 Set
            sb.push((c as u8 - 96) as char);
            return 2;
        }
        sb.push_str("\u{1}\u{001e}"); //Shift 2, Upper Shift
        let mut len = 2;
        len += Self::encodeChar_c40((c as u8 - 128) as char, sb);

        len
    }

    fn encodeToCodewords(sb: &str) -> Option<String> {
        let v = (1600 * sb.chars().next()? as u32)
            + (40 * sb.chars().nth(1)? as u32)
            + sb.chars().nth(2)? as u32
            + 1;
        let cw1 = v / 256;
        let cw2 = v % 256;
        Some(
            [char::from_u32(cw1)?, char::from_u32(cw2)?]
                .into_iter()
                .collect(),
        )
        // return new String(new char[] {cw1, cw2});
    }
}

impl Default for C40Encoder {
    fn default() -> Self {
        Self::new()
    }
}
