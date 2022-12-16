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

// package com.google.zxing.client.result;

// import java.util.Map;
// import java.util.Objects;

use std::collections::HashMap;

use super::{ParsedRXingResult, ParsedRXingResultType};

/**
 * Represents a parsed result that encodes extended product information as encoded
 * by the RSS format, like weight, price, dates, etc.
 *
 * @author Antonio Manuel Benjumea Conde, Servinform, S.A.
 * @author Agustín Delgado, Servinform, S.A.
 */
#[derive(PartialEq, Eq, Debug)]
pub struct ExpandedProductParsedRXingResult {
    rawText: String,
    productID: String,
    sscc: String,
    lotNumber: String,
    productionDate: String,
    packagingDate: String,
    bestBeforeDate: String,
    expirationDate: String,
    weight: String,
    weightType: String,
    weightIncrement: String,
    price: String,
    priceIncrement: String,
    priceCurrency: String,
    // For AIS that not exist in this object
    uncommonAIs: HashMap<String, String>,
}
impl ParsedRXingResult for ExpandedProductParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::PRODUCT
    }

    fn getDisplayRXingResult(&self) -> String {
        self.rawText.clone()
    }
}

impl ExpandedProductParsedRXingResult {
    pub const KILOGRAM: &'static str = "KG";
    pub const POUND: &'static str = "LB";

    pub fn new(
        rawText: String,
        productID: String,
        sscc: String,
        lotNumber: String,
        productionDate: String,
        packagingDate: String,
        bestBeforeDate: String,
        expirationDate: String,
        weight: String,
        weightType: String,
        weightIncrement: String,
        price: String,
        priceIncrement: String,
        priceCurrency: String,
        uncommonAIs: HashMap<String, String>,
    ) -> Self {
        Self {
            rawText,
            productID,
            sscc,
            lotNumber,
            productionDate,
            packagingDate,
            bestBeforeDate,
            expirationDate,
            weight,
            weightType,
            weightIncrement,
            price,
            priceIncrement,
            priceCurrency,
            uncommonAIs,
        }
    }

    // @Override
    // public boolean equals(Object o) {
    //   if (!(o instanceof ExpandedProductParsedRXingResult)) {
    //     return false;
    //   }

    //   ExpandedProductParsedRXingResult other = (ExpandedProductParsedRXingResult) o;

    //   return Objects.equals(productID, other.productID)
    //       && Objects.equals(sscc, other.sscc)
    //       && Objects.equals(lotNumber, other.lotNumber)
    //       && Objects.equals(productionDate, other.productionDate)
    //       && Objects.equals(bestBeforeDate, other.bestBeforeDate)
    //       && Objects.equals(expirationDate, other.expirationDate)
    //       && Objects.equals(weight, other.weight)
    //       && Objects.equals(weightType, other.weightType)
    //       && Objects.equals(weightIncrement, other.weightIncrement)
    //       && Objects.equals(price, other.price)
    //       && Objects.equals(priceIncrement, other.priceIncrement)
    //       && Objects.equals(priceCurrency, other.priceCurrency)
    //       && Objects.equals(uncommonAIs, other.uncommonAIs);
    // }

    // @Override
    // public int hashCode() {
    //   int hash = Objects.hashCode(productID);
    //   hash ^= Objects.hashCode(sscc);
    //   hash ^= Objects.hashCode(lotNumber);
    //   hash ^= Objects.hashCode(productionDate);
    //   hash ^= Objects.hashCode(bestBeforeDate);
    //   hash ^= Objects.hashCode(expirationDate);
    //   hash ^= Objects.hashCode(weight);
    //   hash ^= Objects.hashCode(weightType);
    //   hash ^= Objects.hashCode(weightIncrement);
    //   hash ^= Objects.hashCode(price);
    //   hash ^= Objects.hashCode(priceIncrement);
    //   hash ^= Objects.hashCode(priceCurrency);
    //   hash ^= Objects.hashCode(uncommonAIs);
    //   return hash;
    // }

    pub fn getRawText(&self) -> &str {
        &self.rawText
    }

    pub fn getProductID(&self) -> &str {
        &self.productID
    }

    pub fn getSscc(&self) -> &str {
        &self.sscc
    }

    pub fn getLotNumber(&self) -> &str {
        &self.lotNumber
    }

    pub fn getProductionDate(&self) -> &str {
        &self.productionDate
    }

    pub fn getPackagingDate(&self) -> &str {
        &self.packagingDate
    }

    pub fn getBestBeforeDate(&self) -> &str {
        &self.bestBeforeDate
    }

    pub fn getExpirationDate(&self) -> &str {
        &self.expirationDate
    }

    pub fn getWeight(&self) -> &str {
        &self.weight
    }

    pub fn getWeightType(&self) -> &str {
        &self.weightType
    }

    pub fn getWeightIncrement(&self) -> &str {
        &self.weightIncrement
    }

    pub fn getPrice(&self) -> &str {
        &self.price
    }

    pub fn getPriceIncrement(&self) -> &str {
        &self.priceIncrement
    }

    pub fn getPriceCurrency(&self) -> &str {
        &self.priceCurrency
    }

    pub fn getUncommonAIs(&self) -> &HashMap<String, String> {
        &self.uncommonAIs
    }
}
