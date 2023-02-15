/*
 * Copyright (C) 2010 ZXing authors
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

/*
 * These authors would like to acknowledge the Spanish Ministry of Industry,
 * Tourism and Trade, for the support in the project TSI020301-2008-2
 * "PIRAmIDE: Personalizable Interactions with Resources on AmI-enabled
 * Mobile Dynamic Environments", led by Treelogic
 * ( http://www.treelogic.com/ ):
 *
 *   http://www.piramidepse.com/
 */

use crate::{common::BitArray, Exceptions};

use super::{
    field_parser, BlockParsedRXingResult, CurrentParsingState, DecodedChar, DecodedInformation,
    DecodedNumeric, DecodedObject,
};

/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
pub struct GeneralAppIdDecoder<'a> {
    information: &'a BitArray,
    current: CurrentParsingState, //= new CurrentParsingState();
    buffer: String,               //= new StringBuilder();
}

impl<'a> GeneralAppIdDecoder<'_> {
    pub fn new(information: &'a BitArray) -> GeneralAppIdDecoder<'a> {
        GeneralAppIdDecoder {
            information,
            current: CurrentParsingState::new(),
            buffer: String::new(),
        }
    }

    pub fn decodeAllCodes(
        &mut self,
        buff: String,
        initialPosition: usize,
    ) -> Result<String, Exceptions> {
        let mut buff = buff;
        let mut currentPosition = initialPosition;
        let mut remaining = String::default();
        loop {
            let info = self.decodeGeneralPurposeField(currentPosition, &remaining)?;
            let parsedFields = field_parser::parseFieldsInGeneralPurpose(info.getNewString())?;
            if !parsedFields.is_empty() {
                buff.push_str(&parsedFields);
            }
            if info.isRemaining() {
                remaining = info.getRemainingValue().to_string();
            } else {
                remaining = String::default();
            }

            if currentPosition == info.getNewPosition() {
                // No step forward!
                break;
            }
            currentPosition = info.getNewPosition();
        }

        Ok(buff)
    }

    fn isStillNumeric(&self, pos: usize) -> bool {
        // It's numeric if it still has 7 positions
        // and one of the first 4 bits is "1".
        if pos + 7 > self.information.getSize() {
            return pos + 4 <= self.information.getSize();
        }

        for i in pos..pos + 3 {
            // for (int i = pos; i < pos + 3; ++i) {
            if self.information.get(i) {
                return true;
            }
        }

        self.information.get(pos + 3)
    }

    fn decodeNumeric(&self, pos: usize) -> Result<DecodedNumeric, Exceptions> {
        if pos + 7 > self.information.getSize() {
            let numeric = self.extractNumericValueFromBitArray(pos, 4);
            if numeric == 0 {
                return DecodedNumeric::new(
                    self.information.getSize(),
                    DecodedNumeric::FNC1,
                    DecodedNumeric::FNC1,
                );
            }
            return DecodedNumeric::new(
                self.information.getSize(),
                numeric - 1,
                DecodedNumeric::FNC1,
            );
        }
        let numeric = self.extractNumericValueFromBitArray(pos, 7);

        let digit1 = (numeric - 8) / 11;
        let digit2 = (numeric - 8) % 11;

        DecodedNumeric::new(pos + 7, digit1, digit2)
    }

    pub fn extractNumericValueFromBitArray(&self, pos: usize, bits: u32) -> u32 {
        Self::extractNumericValueFromBitArrayWithInformation(self.information, pos, bits)
    }

    pub fn extractNumericValueFromBitArrayWithInformation(
        information: &BitArray,
        pos: usize,
        bits: u32,
    ) -> u32 {
        let mut value = 0;
        for i in 0..bits {
            if information.get(pos + i as usize) {
                value |= 1 << (bits - i - 1);
            }
        }

        value
    }

    pub fn decodeGeneralPurposeField(
        &mut self,
        pos: usize,
        remaining: &str,
    ) -> Result<DecodedInformation, Exceptions> {
        self.buffer.clear();

        if !remaining.is_empty() {
            self.buffer.push_str(remaining);
        }

        self.current.setPosition(pos);

        if let Ok(lastDecoded) = self.parseBlocks() {
            if lastDecoded.isRemaining() {
                return Ok(DecodedInformation::with_remaining_value(
                    self.current.getPosition(),
                    self.buffer.clone(),
                    lastDecoded.getRemainingValue(),
                ));
            }
        }
        Ok(DecodedInformation::new(
            self.current.getPosition(),
            self.buffer.clone(),
        ))
    }

    fn parseBlocks(&mut self) -> Result<DecodedInformation, Exceptions> {
        let mut isFinished;
        let mut result: BlockParsedRXingResult;
        loop {
            let initialPosition = self.current.getPosition();

            if self.current.isAlpha() {
                result = self.parseAlphaBlock()?;
                isFinished = result.isFinished();
            } else if self.current.isIsoIec646() {
                result = self.parseIsoIec646Block()?;
                isFinished = result.isFinished();
            } else {
                // it must be numeric
                result = self.parseNumericBlock()?;
                isFinished = result.isFinished();
            }

            let positionChanged = initialPosition != self.current.getPosition();
            if !positionChanged && !isFinished {
                break;
            }

            if isFinished {
                break;
            }
        } //while (!isFinished);

        if let Some(r) = result.getDecodedInformation() {
            Ok(r.clone())
        } else {
            Err(Exceptions::notFound)
        }
    }

    fn parseNumericBlock(&mut self) -> Result<BlockParsedRXingResult, Exceptions> {
        while self.isStillNumeric(self.current.getPosition()) {
            let numeric = self.decodeNumeric(self.current.getPosition())?;
            self.current.setPosition(numeric.getNewPosition());

            if numeric.isFirstDigitFNC1() {
                let information = if numeric.isSecondDigitFNC1() {
                    DecodedInformation::new(self.current.getPosition(), self.buffer.clone())
                } else {
                    DecodedInformation::with_remaining_value(
                        self.current.getPosition(),
                        self.buffer.clone(),
                        numeric.getSecondDigit(),
                    )
                };
                return Ok(BlockParsedRXingResult::with_information(
                    Some(information),
                    true,
                ));
            }
            self.buffer.push_str(&numeric.getFirstDigit().to_string());

            if numeric.isSecondDigitFNC1() {
                let information =
                    DecodedInformation::new(self.current.getPosition(), self.buffer.clone());
                return Ok(BlockParsedRXingResult::with_information(
                    Some(information),
                    true,
                ));
            }
            self.buffer.push_str(&numeric.getSecondDigit().to_string());
        }

        if self.isNumericToAlphaNumericLatch(self.current.getPosition()) {
            self.current.setAlpha();
            self.current.incrementPosition(4);
        }

        Ok(BlockParsedRXingResult::new())
    }

    fn parseIsoIec646Block(&mut self) -> Result<BlockParsedRXingResult, Exceptions> {
        while self.isStillIsoIec646(self.current.getPosition()) {
            let iso = self.decodeIsoIec646(self.current.getPosition())?;
            self.current.setPosition(iso.getNewPosition());

            if iso.isFNC1() {
                let information =
                    DecodedInformation::new(self.current.getPosition(), self.buffer.clone());
                return Ok(BlockParsedRXingResult::with_information(
                    Some(information),
                    true,
                ));
            }
            self.buffer.push_str(&iso.getValue().to_string());
        }

        if self.isAlphaOr646ToNumericLatch(self.current.getPosition()) {
            self.current.incrementPosition(3);
            self.current.setNumeric();
        } else if self.isAlphaTo646ToAlphaLatch(self.current.getPosition()) {
            if self.current.getPosition() + 5 < self.information.getSize() {
                self.current.incrementPosition(5);
            } else {
                self.current.setPosition(self.information.getSize());
            }

            self.current.setAlpha();
        }
        Ok(BlockParsedRXingResult::new())
    }

    fn parseAlphaBlock(&mut self) -> Result<BlockParsedRXingResult, Exceptions> {
        while self.isStillAlpha(self.current.getPosition()) {
            let alpha = self.decodeAlphanumeric(self.current.getPosition())?;
            self.current.setPosition(alpha.getNewPosition());

            if alpha.isFNC1() {
                let information =
                    DecodedInformation::new(self.current.getPosition(), self.buffer.clone());
                return Ok(BlockParsedRXingResult::with_information(
                    Some(information),
                    true,
                )); //end of the char block
            }

            self.buffer.push_str(&alpha.getValue().to_string());
        }

        if self.isAlphaOr646ToNumericLatch(self.current.getPosition()) {
            self.current.incrementPosition(3);
            self.current.setNumeric();
        } else if self.isAlphaTo646ToAlphaLatch(self.current.getPosition()) {
            if self.current.getPosition() + 5 < self.information.getSize() {
                self.current.incrementPosition(5);
            } else {
                self.current.setPosition(self.information.getSize());
            }

            self.current.setIsoIec646();
        }

        Ok(BlockParsedRXingResult::new())
    }

    fn isStillIsoIec646(&self, pos: usize) -> bool {
        if pos + 5 > self.information.getSize() {
            return false;
        }

        let fiveBitValue = self.extractNumericValueFromBitArray(pos, 5);
        if (5..16).contains(&fiveBitValue) {
            return true;
        }

        if pos + 7 > self.information.getSize() {
            return false;
        }

        let sevenBitValue = self.extractNumericValueFromBitArray(pos, 7);
        if (64..116).contains(&sevenBitValue) {
            return true;
        }

        if pos + 8 > self.information.getSize() {
            return false;
        }

        let eightBitValue = self.extractNumericValueFromBitArray(pos, 8);

        (232..253).contains(&eightBitValue)
    }

    fn decodeIsoIec646(&self, pos: usize) -> Result<DecodedChar, Exceptions> {
        let fiveBitValue = self.extractNumericValueFromBitArray(pos, 5);
        if fiveBitValue == 15 {
            return Ok(DecodedChar::new(pos + 5, DecodedChar::FNC1));
        }

        if (5..15).contains(&fiveBitValue) {
            return Ok(DecodedChar::new(
                pos + 5,
                char::from_u32('0' as u32 + fiveBitValue - 5).ok_or(Exceptions::parse)?,
            ));
        }

        let sevenBitValue = self.extractNumericValueFromBitArray(pos, 7);

        if (64..90).contains(&sevenBitValue) {
            return Ok(DecodedChar::new(
                pos + 7,
                char::from_u32(sevenBitValue + 1).ok_or(Exceptions::parse)?,
            ));
        }

        if (90..116).contains(&sevenBitValue) {
            return Ok(DecodedChar::new(
                pos + 7,
                char::from_u32(sevenBitValue + 7).ok_or(Exceptions::parse)?,
            ));
        }

        let eightBitValue = self.extractNumericValueFromBitArray(pos, 8);
        let c = match eightBitValue {
            232 => '!',
            233 => '"',
            234 => '%',
            235 => '&',
            236 => '\'',
            237 => '(',
            238 => ')',
            239 => '*',
            240 => '+',
            241 => ',',
            242 => '-',
            243 => '.',
            244 => '/',
            245 => ':',
            246 => ';',
            247 => '<',
            248 => '=',
            249 => '>',
            250 => '?',
            251 => '_',
            252 => ' ',
            _ => return Err(Exceptions::format),
        };

        Ok(DecodedChar::new(pos + 8, c))
    }

    fn isStillAlpha(&self, pos: usize) -> bool {
        if pos + 5 > self.information.getSize() {
            return false;
        }

        // We now check if it's a valid 5-bit value (0..9 and FNC1)
        let fiveBitValue = self.extractNumericValueFromBitArray(pos, 5);
        if (5..16).contains(&fiveBitValue) {
            return true;
        }

        if pos + 6 > self.information.getSize() {
            return false;
        }

        let sixBitValue = self.extractNumericValueFromBitArray(pos, 6);

        (16..63).contains(&sixBitValue) // 63 not included
    }

    fn decodeAlphanumeric(&self, pos: usize) -> Result<DecodedChar, Exceptions> {
        let fiveBitValue = self.extractNumericValueFromBitArray(pos, 5);
        if fiveBitValue == 15 {
            return Ok(DecodedChar::new(pos + 5, DecodedChar::FNC1));
        }

        if (5..15).contains(&fiveBitValue) {
            return Ok(DecodedChar::new(
                pos + 5,
                char::from_u32('0' as u32 + fiveBitValue - 5).ok_or(Exceptions::parse)?,
            ));
        }

        let sixBitValue = self.extractNumericValueFromBitArray(pos, 6);

        if (32..58).contains(&sixBitValue) {
            return Ok(DecodedChar::new(
                pos + 6,
                char::from_u32(sixBitValue + 33).ok_or(Exceptions::parse)?,
            ));
        }

        let c = match sixBitValue {
            58 => '*',
            59 => ',',
            60 => '-',
            61 => '.',
            62 => '/',
            _ => {
                return Err(Exceptions::illegalStateWith(format!(
                    "Decoding invalid alphanumeric value: {sixBitValue}"
                )))
            }
        };

        Ok(DecodedChar::new(pos + 6, c))
    }

    fn isAlphaTo646ToAlphaLatch(&self, pos: usize) -> bool {
        if pos + 1 > self.information.getSize() {
            return false;
        }

        let mut i = 0;
        while i < 5 && i + pos < self.information.getSize() {
            if i == 2 {
                if !self.information.get(pos + 2) {
                    return false;
                }
            } else if self.information.get(pos + i) {
                return false;
            }

            i += 1;
        }

        true
    }

    fn isAlphaOr646ToNumericLatch(&self, pos: usize) -> bool {
        // Next is alphanumeric if there are 3 positions and they are all zeros
        if pos + 3 > self.information.getSize() {
            return false;
        }

        for i in pos..pos + 3 {
            if self.information.get(i) {
                return false;
            }
        }

        true
    }

    fn isNumericToAlphaNumericLatch(&self, pos: usize) -> bool {
        // Next is alphanumeric if there are 4 positions and they are all zeros, or
        // if there is a subset of this just before the end of the symbol
        if pos + 1 > self.information.getSize() {
            return false;
        }

        let mut i = 0;
        while i < 4 && i + pos < self.information.getSize() {
            if self.information.get(pos + i) {
                return false;
            }
            i += 1;
        }

        true
    }
}
