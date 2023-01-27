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

use std::collections::HashMap;

use crate::{
    common::BitMatrix, BarcodeFormat, EncodeHintType, EncodeHintValue, Exceptions, Writer,
};

use once_cell::sync::Lazy;
use regex::Regex;

pub static NUMERIC: Lazy<Regex> = Lazy::new(|| Regex::new("[0-9]+").unwrap());

/**
 * <p>Encapsulates functionality and implementation that is common to one-dimensional barcodes.</p>
 *
 * @author dsbnatut@gmail.com (Kazuki Nishiura)
 */
pub trait OneDimensionalCodeWriter: Writer {
    // private static final Pattern NUMERIC = Pattern.compile("[0-9]+");

    /**
     * Encode the contents to boolean array expression of one-dimensional barcode.
     * Start code and end code should be included in result, and side margins should not be included.
     *
     * @param contents barcode contents to encode
     * @return a {@code boolean[]} of horizontal pixels (false = white, true = black)
     */
    fn encode_oned(&self, contents: &str) -> Result<Vec<bool>, Exceptions>;

    /**
     * Can be overwritten if the encode requires to read the hints map. Otherwise it defaults to {@code encode}.
     * @param contents barcode contents to encode
     * @param hints encoding hints
     * @return a {@code boolean[]} of horizontal pixels (false = white, true = black)
     */
    fn encode_oned_with_hints(
        &self,
        contents: &str,
        _hints: &crate::EncodingHintDictionary,
    ) -> Result<Vec<bool>, Exceptions> {
        self.encode_oned(contents)
    }

    fn getSupportedWriteFormats(&self) -> Option<Vec<BarcodeFormat>> {
        None
    }

    /**
     * @return a byte array of horizontal pixels (0 = white, 1 = black)
     */
    fn renderRXingResult(
        code: &[bool],
        width: i32,
        height: i32,
        sidesMargin: u32,
    ) -> Result<BitMatrix, Exceptions> {
        let inputWidth = code.len();
        // Add quiet zone on both sides.
        let fullWidth = inputWidth + sidesMargin as usize;
        let outputWidth = width.max(fullWidth as i32);
        let outputHeight = 1.max(height);

        let multiple = outputWidth as usize / fullWidth;
        let leftPadding = (outputWidth as isize - (inputWidth as isize * multiple as isize)) / 2;

        let mut output = BitMatrix::new(outputWidth as u32, outputHeight as u32)?;

        let mut inputX = 0;
        let mut outputX = leftPadding;

        while inputX < inputWidth {
            // for (int inputX = 0, outputX = leftPadding; inputX < inputWidth; inputX++, outputX += multiple) {
            if code[inputX] {
                output.setRegion(outputX as u32, 0, multiple as u32, outputHeight as u32)?;
            }

            inputX += 1;
            outputX += multiple as isize;
        }
        Ok(output)
    }

    /**
     * @param contents string to check for numeric characters
     * @throws IllegalArgumentException if input contains characters other than digits 0-9.
     */
    fn checkNumeric(contents: &str) -> Result<(), Exceptions> {
        if !NUMERIC.is_match(contents) {
            Err(Exceptions::IllegalArgumentException(Some(
                "Input should only contain digits 0-9".to_owned(),
            )))
        } else {
            Ok(())
        }
    }

    /**
     * @param target encode black/white pattern into this array
     * @param pos position to start encoding at in {@code target}
     * @param pattern lengths of black/white runs to encode
     * @param startColor starting color - false for white, true for black
     * @return the number of elements added to target.
     */
    fn appendPattern<T: TryInto<usize> + Copy>(
        target: &mut [bool],
        pos: usize,
        pattern: &[T],
        startColor: bool,
    ) -> u32 {
        let mut color = startColor;
        let mut numAdded = 0;
        let mut pos = pos;
        for len in pattern {
            // for (int len : pattern) {
            for _j in 0..TryInto::<usize>::try_into(*len).unwrap_or_default() {
                // for (int j = 0; j < len; j++) {
                target[pos] = color;
                pos += 1;
            }
            numAdded += TryInto::<usize>::try_into(*len).unwrap_or_default();
            color = !color; // flip color after each segment
        }
        numAdded as u32
    }

    fn getDefaultMargin(&self) -> u32 {
        // CodaBar spec requires a side margin to be more than ten times wider than narrow space.
        // This seems like a decent idea for a default for all formats.
        10
    }
}

struct L;
impl Writer for L {
    fn encode(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
    ) -> Result<crate::common::BitMatrix, crate::Exceptions> {
        self.encode_with_hints(contents, format, width, height, &HashMap::new())
    }

    fn encode_with_hints(
        &self,
        contents: &str,
        format: &crate::BarcodeFormat,
        width: i32,
        height: i32,
        hints: &crate::EncodingHintDictionary,
    ) -> Result<crate::common::BitMatrix, crate::Exceptions> {
        if contents.is_empty() {
            return Err(Exceptions::IllegalArgumentException(Some(
                "Found empty contents".to_owned(),
            )));
        }

        if width < 0 || height < 0 {
            return Err(Exceptions::IllegalArgumentException(Some(format!(
                "Negative size is not allowed. Input: {width}x{height}"
            ))));
        }
        if let Some(supportedFormats) = self.getSupportedWriteFormats() {
            if !supportedFormats.contains(format) {
                return Err(Exceptions::IllegalArgumentException(Some(format!(
                    "Can only encode {supportedFormats:?}, but got {format:?}"
                ))));
            }
        }

        let mut sidesMargin = self.getDefaultMargin();
        if let Some(EncodeHintValue::Margin(margin)) = hints.get(&EncodeHintType::MARGIN) {
            sidesMargin = margin.parse::<u32>().unwrap();
        }
        // if  hints.contains_key(&EncodeHintType::MARGIN) {
        //   sidesMargin = Integer.parseInt(hints.get(EncodeHintType.MARGIN).toString());
        // }

        let code = self.encode_oned_with_hints(contents, hints)?;

        Self::renderRXingResult(&code, width, height, sidesMargin)
    }
}
impl OneDimensionalCodeWriter for L {
    fn encode_oned(&self, _contents: &str) -> Result<Vec<bool>, Exceptions> {
        todo!()
    }
}
