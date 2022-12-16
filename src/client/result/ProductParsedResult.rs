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

use super::{ParsedRXingResult, ParsedRXingResultType};

/**
 * Represents a parsed result that encodes a product by an identifier of some kind.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct ProductParsedRXingResult {
    product_id: String,
    normalized_product_id: String,
}
impl ParsedRXingResult for ProductParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::PRODUCT
    }

    fn getDisplayRXingResult(&self) -> String {
        self.product_id.clone()
    }
}
impl ProductParsedRXingResult {
    pub fn new(product_id: String) -> Self {
        Self::with_normalized_id(product_id.clone(), product_id)
    }

    pub fn with_normalized_id(product_id: String, normalized_product_id: String) -> Self {
        Self {
            product_id,
            normalized_product_id,
        }
    }

    pub fn getProductID(&self) -> &str {
        &self.product_id
    }

    pub fn getNormalizedProductID(&self) -> &str {
        &self.normalized_product_id
    }
}
