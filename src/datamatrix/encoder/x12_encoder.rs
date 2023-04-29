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

use super::{high_level_encoder, C40Encoder, Encoder, EncoderContext};

pub struct X12Encoder(C40Encoder);
impl Encoder for X12Encoder {
    fn getEncodingMode(&self) -> usize {
        high_level_encoder::X12_ENCODATION
    }

    fn encode(&self, context: &mut super::EncoderContext) -> Result<()> {
        //step C
        let mut buffer = String::new();
        while context.hasMoreCharacters() {
            let c = context.getCurrentChar();
            context.pos += 1;

            Self::encodeChar(c, &mut buffer)?;

            let count = buffer.chars().count();
            if (count % 3) == 0 {
                C40Encoder::writeNextTriplet(context, &mut buffer)?;

                let newMode = high_level_encoder::lookAheadTest(
                    context.getMessage(),
                    context.pos,
                    self.getEncodingMode() as u32,
                );
                if newMode != self.getEncodingMode() {
                    // Return to ASCII encodation, which will actually handle latch to new mode
                    context.signalEncoderChange(high_level_encoder::ASCII_ENCODATION);
                    break;
                }
            }
        }
        Self::handleEOD(context, &mut buffer)?;
        Ok(())
    }
}
impl X12Encoder {
    pub fn new() -> Self {
        Self(C40Encoder::new())
    }

    fn encodeChar(c: char, sb: &mut String) -> Result<u32> {
        match c {
            '\r' => sb.push('\0'),
            '*' => sb.push('\u{1}'),
            '>' => sb.push('\u{2}'),
            ' ' => sb.push('\u{3}'),
            _ => {
                if c.is_ascii_digit() {
                    sb.push((c as u8 - 48 + 4) as char);
                } else if c.is_ascii_uppercase() {
                    sb.push((c as u8 - 65 + 14) as char);
                } else {
                    high_level_encoder::illegalCharacter(c)?;
                }
            }
        }
        Ok(1)
    }

    fn handleEOD(context: &mut EncoderContext, buffer: &mut str) -> Result<()> {
        context.updateSymbolInfo();
        let available = context
            .getSymbolInfo()
            .ok_or(Exceptions::ILLEGAL_STATE)?
            .getDataCapacity()
            - context.getCodewordCount() as u32;
        let count = buffer.chars().count();
        context.pos -= count as u32;
        if context.getRemainingCharacters() > 1
            || available > 1
            || context.getRemainingCharacters() != available
        {
            context.writeCodeword(high_level_encoder::X12_UNLATCH);
        }
        if context.getNewEncoding().is_none() {
            context.signalEncoderChange(high_level_encoder::ASCII_ENCODATION);
        }
        Ok(())
    }
}

impl Default for X12Encoder {
    fn default() -> Self {
        Self::new()
    }
}
