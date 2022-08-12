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
 * A simple result type encapsulating a string that has no further
 * interpretation.
 * 
 * @author Sean Owen
 */
pub struct TextParsedResult {
    super: ParsedResult;

     let text: String;

     let language: String;
}

impl TextParsedResult {

    pub fn new( text: &String,  language: &String) -> TextParsedResult {
        super(ParsedResultType::TEXT);
        let .text = text;
        let .language = language;
    }

    pub fn  get_text(&self) -> String  {
        return self.text;
    }

    pub fn  get_language(&self) -> String  {
        return self.language;
    }

    pub fn  get_display_result(&self) -> String  {
        return self.text;
    }
}

