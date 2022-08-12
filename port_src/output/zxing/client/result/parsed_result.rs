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
// package com::google::zxing::client::result;

/**
 * <p>Abstract class representing the result of decoding a barcode, as more than
 * a String -- as some type of structured data. This might be a subclass which represents
 * a URL, or an e-mail address. {@link ResultParser#parseResult(com.google.zxing.Result)} will turn a raw
 * decoded string into the most appropriate type of structured representation.</p>
 *
 * <p>Thanks to Jeff Griffin for proposing rewrite of these classes that relies less
 * on exception-based mechanisms during parsing.</p>
 *
 * @author Sean Owen
 */
pub struct ParsedResult {

     let type: ParsedResultType;
}

impl ParsedResult {

    pub fn new( type: &ParsedResultType) -> ParsedResult {
        let .type = type;
    }

    pub fn  get_type(&self) -> ParsedResultType  {
        return self.type;
    }

    pub fn  get_display_result(&self) -> String ;

    pub fn  to_string(&self) -> String  {
        return self.get_display_result();
    }

    pub fn  maybe_append( value: &String,  result: &StringBuilder)   {
        if value != null && !value.is_empty() {
            // Don't add a newline before the first value
            if result.length() > 0 {
                result.append('\n');
            }
            result.append(&value);
        }
    }

    pub fn  maybe_append( values: &Vec<String>,  result: &StringBuilder)   {
        if values != null {
            for  let value: String in values {
                ::maybe_append(&value, &result);
            }
        }
    }
}

