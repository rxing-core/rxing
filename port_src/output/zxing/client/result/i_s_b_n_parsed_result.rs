/*
 * Copyright 2008 ZXing authors
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
 * Represents a parsed result that encodes a product ISBN number.
 *
 * @author jbreiden@google.com (Jeff Breidenbach)
 */
pub struct ISBNParsedResult {
    super: ParsedResult;

     let isbn: String;
}

impl ISBNParsedResult {

    fn new( isbn: &String) -> ISBNParsedResult {
        super(ParsedResultType::ISBN);
        let .isbn = isbn;
    }

    pub fn  get_i_s_b_n(&self) -> String  {
        return self.isbn;
    }

    pub fn  get_display_result(&self) -> String  {
        return self.isbn;
    }
}

