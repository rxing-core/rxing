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

// package com.google.zxing.client.result;

use super::{ParsedRXingResult, ParsedRXingResultType};

/**
 * Represents a parsed result that encodes a geographic coordinate, with latitude,
 * longitude and altitude.
 *
 * @author Sean Owen
 */
#[derive(Debug)]
pub struct GeoParsedRXingResult {
    latitude: f64,
    longitude: f64,
    altitude: f64,
    query: String,
}

impl ParsedRXingResult for GeoParsedRXingResult {
    fn getType(&self) -> super::ParsedRXingResultType {
        ParsedRXingResultType::GEO
    }

    fn getDisplayRXingResult(&self) -> String {
        let mut result = String::with_capacity(20);
        // result.push_str(self.latitude);
        // result.push_str(", ");
        // result.push_str(self.longitude);
        result.push_str(&format!("{}, {}", self.latitude, self.longitude));
        if self.altitude > 0.0 {
            result.push_str(&format!(", {}m", self.altitude));
            // result.push_str(", ");
            // result.push_str(self.altitude);
            // result.push('m');
        }
        if !self.query.is_empty() {
            result.push_str(&format!(" ({})", self.query));
            // result.push_str(" (");
            // result.push_str(self.query);
            // result.push(')');
        }
        result
    }
}

impl GeoParsedRXingResult {
    pub fn new(latitude: f64, longitude: f64, altitude: f64, query: String) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
            query,
        }
    }

    pub fn getGeoURI(&self) -> String {
        //StringBuilder result = new StringBuilder();
        let mut result = format!("geo:{},{}", self.latitude, self.longitude);
        // result.append("geo:");
        // result.append(latitude);
        // result.append(',');
        // result.append(longitude);
        if self.altitude > 0.0 {
            result.push_str(&format!(",{}", self.altitude));
            // result.append(',');
            // result.append(altitude);
        }
        if !self.query.is_empty() {
            result.push_str(&format!("?{}", self.query));
            // result.append('?');
            // result.append(query);
        }
        result
    }

    /**
     * @return latitude in degrees
     */
    pub fn getLatitude(&self) -> f64 {
        self.latitude
    }

    /**
     * @return longitude in degrees
     */
    pub fn getLongitude(&self) -> f64 {
        self.longitude
    }

    /**
     * @return altitude in meters. If not specified, in the geo URI, returns 0.0
     */
    pub fn getAltitude(&self) -> f64 {
        self.altitude
    }

    /**
     * @return query string associated with geo URI or null if none exists
     */
    pub fn getQuery(&self) -> &str {
        &self.query
    }
}

impl PartialEq for GeoParsedRXingResult {
    fn eq(&self, other: &Self) -> bool {
        self.latitude == other.latitude
            && self.longitude == other.longitude
            && self.altitude == other.altitude
            && self.query == other.query
    }
}
impl Eq for GeoParsedRXingResult {}
