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
 * Represents a parsed result that encodes a product by an identifier of some kind.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct ProductParsedResult {
    super: ParsedResult;

     let product_i_d: String;

     let normalized_product_i_d: String;
}

impl ProductParsedResult {

    fn new( product_i_d: &String) -> ProductParsedResult {
        this(&product_i_d, &product_i_d);
    }

    fn new( product_i_d: &String,  normalized_product_i_d: &String) -> ProductParsedResult {
        super(ParsedResultType::PRODUCT);
        let .productID = product_i_d;
        let .normalizedProductID = normalized_product_i_d;
    }

    pub fn  get_product_i_d(&self) -> String  {
        return self.product_i_d;
    }

    pub fn  get_normalized_product_i_d(&self) -> String  {
        return self.normalized_product_i_d;
    }

    pub fn  get_display_result(&self) -> String  {
        return self.product_i_d;
    }
}

