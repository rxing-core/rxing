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

use super::{high_level_encoder, Encoder};

#[derive(Debug, Default)]
pub struct ASCIIEncoder;

impl Encoder for ASCIIEncoder {
    fn encode(&self, context: &mut super::EncoderContext) -> Result<()> {
        //step B
        let n =
            high_level_encoder::determineConsecutiveDigitCount(context.getMessage(), context.pos);
        if n >= 2 {
            context.writeCodeword(Self::encodeASCIIDigits(
                context
                    .getMessage()
                    .chars()
                    .nth(context.pos as usize)
                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
                context
                    .getMessage()
                    .chars()
                    .nth(context.pos as usize + 1)
                    .ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?,
            )? as u8);
            context.pos += 2;
        } else {
            let c = context.getCurrentChar();
            let newMode = high_level_encoder::lookAheadTest(
                context.getMessage(),
                context.pos,
                self.getEncodingMode() as u32,
            );
            if newMode != self.getEncodingMode() {
                match newMode {
                    high_level_encoder::BASE256_ENCODATION => {
                        context.writeCodeword(high_level_encoder::LATCH_TO_BASE256);
                        context.signalEncoderChange(high_level_encoder::BASE256_ENCODATION);
                        return Ok(());
                    }
                    high_level_encoder::C40_ENCODATION => {
                        context.writeCodeword(high_level_encoder::LATCH_TO_C40);
                        context.signalEncoderChange(high_level_encoder::C40_ENCODATION);
                        return Ok(());
                    }
                    high_level_encoder::X12_ENCODATION => {
                        context.writeCodeword(high_level_encoder::LATCH_TO_ANSIX12);
                        context.signalEncoderChange(high_level_encoder::X12_ENCODATION);
                    }
                    high_level_encoder::TEXT_ENCODATION => {
                        context.writeCodeword(high_level_encoder::LATCH_TO_TEXT);
                        context.signalEncoderChange(high_level_encoder::TEXT_ENCODATION);
                    }

                    high_level_encoder::EDIFACT_ENCODATION => {
                        context.writeCodeword(high_level_encoder::LATCH_TO_EDIFACT);
                        context.signalEncoderChange(high_level_encoder::EDIFACT_ENCODATION);
                    }

                    _ => {
                        return Err(Exceptions::illegal_state_with(format!(
                            "Illegal mode: {newMode}"
                        )));
                    }
                }
            } else if high_level_encoder::isExtendedASCII(c) {
                context.writeCodeword(high_level_encoder::UPPER_SHIFT);
                context.writeCodeword(c as u8 - 128 + 1);
                context.pos += 1;
            } else {
                context.writeCodeword(c as u8 + 1);
                context.pos += 1;
            }
        }
        Ok(())
    }

    fn getEncodingMode(&self) -> usize {
        high_level_encoder::ASCII_ENCODATION
    }
}

impl ASCIIEncoder {
    pub fn new() -> Self {
        Self
    }
    fn encodeASCIIDigits(digit1: char, digit2: char) -> Result<char> {
        if high_level_encoder::isDigit(digit1) && high_level_encoder::isDigit(digit2) {
            let num = (digit1 as u8 - 48) * 10 + (digit2 as u8 - 48);
            Ok((num + 130) as char)
        } else {
            Err(Exceptions::illegal_argument_with(format!(
                "not digits: {digit1}{digit2}"
            )))
        }
    }
}
