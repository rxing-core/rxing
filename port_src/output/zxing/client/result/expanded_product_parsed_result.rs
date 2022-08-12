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
// package com::google::zxing::client::result;

/**
 * Represents a parsed result that encodes extended product information as encoded
 * by the RSS format, like weight, price, dates, etc.
 *
 * @author Antonio Manuel Benjumea Conde, Servinform, S.A.
 * @author AgustÃ­n Delgado, Servinform, S.A.
 */

 const KILOGRAM: &'static str = "KG";

 const POUND: &'static str = "LB";
pub struct ExpandedProductParsedResult {
    super: ParsedResult;

     let raw_text: String;

     let product_i_d: String;

     let sscc: String;

     let lot_number: String;

     let production_date: String;

     let packaging_date: String;

     let best_before_date: String;

     let expiration_date: String;

     let weight: String;

     let weight_type: String;

     let weight_increment: String;

     let price: String;

     let price_increment: String;

     let price_currency: String;

    // For AIS that not exist in this object
     let uncommon_a_is: Map<String, String>;
}

impl ExpandedProductParsedResult {

    pub fn new( raw_text: &String,  product_i_d: &String,  sscc: &String,  lot_number: &String,  production_date: &String,  packaging_date: &String,  best_before_date: &String,  expiration_date: &String,  weight: &String,  weight_type: &String,  weight_increment: &String,  price: &String,  price_increment: &String,  price_currency: &String,  uncommon_a_is: &Map<String, String>) -> ExpandedProductParsedResult {
        super(ParsedResultType::PRODUCT);
        let .rawText = raw_text;
        let .productID = product_i_d;
        let .sscc = sscc;
        let .lotNumber = lot_number;
        let .productionDate = production_date;
        let .packagingDate = packaging_date;
        let .bestBeforeDate = best_before_date;
        let .expirationDate = expiration_date;
        let .weight = weight;
        let .weightType = weight_type;
        let .weightIncrement = weight_increment;
        let .price = price;
        let .priceIncrement = price_increment;
        let .priceCurrency = price_currency;
        let .uncommonAIs = uncommon_a_is;
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof ExpandedProductParsedResult) {
            return false;
        }
         let other: ExpandedProductParsedResult = o as ExpandedProductParsedResult;
        return Objects::equals(&self.product_i_d, other.productID) && Objects::equals(&self.sscc, other.sscc) && Objects::equals(&self.lot_number, other.lotNumber) && Objects::equals(&self.production_date, other.productionDate) && Objects::equals(&self.best_before_date, other.bestBeforeDate) && Objects::equals(&self.expiration_date, other.expirationDate) && Objects::equals(&self.weight, other.weight) && Objects::equals(&self.weight_type, other.weightType) && Objects::equals(&self.weight_increment, other.weightIncrement) && Objects::equals(&self.price, other.price) && Objects::equals(&self.price_increment, other.priceIncrement) && Objects::equals(&self.price_currency, other.priceCurrency) && Objects::equals(&self.uncommon_a_is, other.uncommonAIs);
    }

    pub fn  hash_code(&self) -> i32  {
         let mut hash: i32 = Objects::hash_code(&self.product_i_d);
        hash ^= Objects::hash_code(&self.sscc);
        hash ^= Objects::hash_code(&self.lot_number);
        hash ^= Objects::hash_code(&self.production_date);
        hash ^= Objects::hash_code(&self.best_before_date);
        hash ^= Objects::hash_code(&self.expiration_date);
        hash ^= Objects::hash_code(&self.weight);
        hash ^= Objects::hash_code(&self.weight_type);
        hash ^= Objects::hash_code(&self.weight_increment);
        hash ^= Objects::hash_code(&self.price);
        hash ^= Objects::hash_code(&self.price_increment);
        hash ^= Objects::hash_code(&self.price_currency);
        hash ^= Objects::hash_code(&self.uncommon_a_is);
        return hash;
    }

    pub fn  get_raw_text(&self) -> String  {
        return self.raw_text;
    }

    pub fn  get_product_i_d(&self) -> String  {
        return self.product_i_d;
    }

    pub fn  get_sscc(&self) -> String  {
        return self.sscc;
    }

    pub fn  get_lot_number(&self) -> String  {
        return self.lot_number;
    }

    pub fn  get_production_date(&self) -> String  {
        return self.production_date;
    }

    pub fn  get_packaging_date(&self) -> String  {
        return self.packaging_date;
    }

    pub fn  get_best_before_date(&self) -> String  {
        return self.best_before_date;
    }

    pub fn  get_expiration_date(&self) -> String  {
        return self.expiration_date;
    }

    pub fn  get_weight(&self) -> String  {
        return self.weight;
    }

    pub fn  get_weight_type(&self) -> String  {
        return self.weight_type;
    }

    pub fn  get_weight_increment(&self) -> String  {
        return self.weight_increment;
    }

    pub fn  get_price(&self) -> String  {
        return self.price;
    }

    pub fn  get_price_increment(&self) -> String  {
        return self.price_increment;
    }

    pub fn  get_price_currency(&self) -> String  {
        return self.price_currency;
    }

    pub fn  get_uncommon_a_is(&self) -> Map<String, String>  {
        return self.uncommon_a_is;
    }

    pub fn  get_display_result(&self) -> String  {
        return String::value_of(&self.raw_text);
    }
}

