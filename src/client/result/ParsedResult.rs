/*
 * Copyright 2007 ZXing authors
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

// package com.google.zxing.client.result;

use std::any::Any;

use super::ParsedRXingResultType;

/**
 * <p>Abstract class representing the result of decoding a barcode, as more than
 * a String -- as some type of structured data. This might be a subclass which represents
 * a URL, or an e-mail address. {@link RXingResultParser#parseRXingResult(com.google.zxing.RXingResult)} will turn a raw
 * decoded string into the most appropriate type of structured representation.</p>
 *
 * <p>Thanks to Jeff Griffin for proposing rewrite of these classes that relies less
 * on exception-based mechanisms during parsing.</p>
 *
 * @author Sean Owen
 */
pub trait ParsedRXingResult {
    // private final ParsedRXingResultType type;

    // protected ParsedRXingResult(ParsedRXingResultType type) {
    //   this.type = type;
    // }

    fn getType(&self) -> ParsedRXingResultType;
    // fn as_any(&self) -> &dyn Any;

    fn getDisplayRXingResult(&self) -> String;

    fn maybe_append(&self, value: &str, result: &mut String) {
        if !value.is_empty() {
            // Don't add a newline before the first value
            if result.len() > 0 {
                result.push('\n');
            }
            result.push_str(value);
        }
    }

    fn maybe_append_multiple(&self, values: &[&str], result: &mut String) {
        if !values.is_empty() {
            for value in values {
                // for (String value : values) {
                self.maybe_append(value, result);
            }
        }
    }
}
