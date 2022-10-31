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

use crate::Exceptions;

use super::{high_level_encoder, Encoder, EncoderContext};

pub struct EdifactEncoder;
impl Encoder for EdifactEncoder {
    fn getEncodingMode(&self) -> usize {
        high_level_encoder::EDIFACT_ENCODATION
    }

    fn encode(&self, context: &mut super::EncoderContext) -> Result<(), crate::Exceptions> {
        //step F
        let mut buffer = String::new();
        while context.hasMoreCharacters() {
            let c = context.getCurrentChar();
            Self::encodeChar(c, &mut buffer);
            context.pos += 1;

            let count = buffer.chars().count();
            if count >= 4 {
                context.writeCodewords(&Self::encodeToCodewords(&buffer)?);
                // buffer.delete(0, 4);
                buffer.replace_range(0..4, "");

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
        buffer.push(31 as char); //Unlatch
        Self::handleEOD(context, &mut buffer)?;
        Ok(())
    }
}
impl EdifactEncoder {
    pub fn new() -> Self {
        Self
    }
    /**
     * Handle "end of data" situations
     *
     * @param context the encoder context
     * @param buffer  the buffer with the remaining encoded characters
     */
    fn handleEOD(context: &mut EncoderContext, buffer: &mut String) -> Result<(), Exceptions> {
        let mut runner = || -> Result<(), Exceptions> {
            let count = buffer.chars().count();
            if count == 0 {
                return Ok(()); //Already finished
            }
            if count == 1 {
                //Only an unlatch at the end
                context.updateSymbolInfo();
                let mut available = context.getSymbolInfo().unwrap().getDataCapacity()
                    - context.getCodewordCount() as u32;
                let remaining = context.getRemainingCharacters();
                // The following two lines are a hack inspired by the 'fix' from https://sourceforge.net/p/barcode4j/svn/221/
                if remaining > available {
                    context.updateSymbolInfoWithLength(context.getCodewordCount() + 1);
                    available = context.getSymbolInfo().unwrap().getDataCapacity()
                        - context.getCodewordCount() as u32;
                }
                if remaining <= available && available <= 2 {
                    return Ok(()); //No unlatch
                }
            }

            if count > 4 {
                return Err(Exceptions::IllegalStateException(
                    "Count must not exceed 4".to_owned(),
                ));
            }
            let restChars = count - 1;
            let encoded = Self::encodeToCodewords(buffer)?;
            let endOfSymbolReached = !context.hasMoreCharacters();
            let mut restInAscii = endOfSymbolReached && restChars <= 2;

            if restChars <= 2 {
                context.updateSymbolInfoWithLength(context.getCodewordCount() + restChars);
                let available = context.getSymbolInfo().unwrap().getDataCapacity()
                    - context.getCodewordCount() as u32;
                if available >= 3 {
                    restInAscii = false;
                    context.updateSymbolInfoWithLength(
                        context.getCodewordCount() + encoded.chars().count(),
                    );
                    //available = context.symbolInfo.dataCapacity - context.getCodewordCount();
                }
            }

            if restInAscii {
                context.resetSymbolInfo();
                context.pos -= restChars as u32;
            } else {
                context.writeCodewords(&encoded);
            }
            Ok(())
        };

        let res = runner();
        context.signalEncoderChange(high_level_encoder::ASCII_ENCODATION);

        res
    }

    fn encodeChar(c: char, sb: &mut String) {
        if c >= ' ' && c <= '?' {
            sb.push(c);
        } else if c >= '@' && c <= '^' {
            sb.push((c as u8 - 64) as char);
        } else {
            high_level_encoder::illegalCharacter(c);
        }
    }

    fn encodeToCodewords(sb: &str) -> Result<String, Exceptions> {
        let len = sb.chars().count();
        if len == 0 {
            return Err(Exceptions::IllegalStateException(
                "StringBuilder must not be empty".to_owned(),
            ));
        }
        let c1 = sb.chars().nth(0).unwrap();
        let c2 = if len >= 2 {
            sb.chars().nth(1).unwrap()
        } else {
            0 as char
        };
        let c3 = if len >= 3 {
            sb.chars().nth(2).unwrap()
        } else {
            0 as char
        };
        let c4 = if len >= 4 {
            sb.chars().nth(3).unwrap()
        } else {
            0 as char
        };

        let v: u32 = ((c1 as u32) << 18) + ((c2 as u32) << 12) + ((c3 as u32) << 6) + c4 as u32;
        let cw1 = (v as u32 >> 16) & 255;
        let cw2 = (v as u32 >> 8) & 255;
        let cw3 = v as u32 & 255;
        let mut res = String::with_capacity(3);
        res.push(char::from_u32(cw1).unwrap());
        if len >= 2 {
            res.push(char::from_u32(cw2).unwrap());
        }
        if len >= 3 {
            res.push(char::from_u32(cw3).unwrap());
        }

        Ok(res)
    }
}
