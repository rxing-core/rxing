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
 * Represents a parsed result that encodes a geographic coordinate, with latitude,
 * longitude and altitude.
 *
 * @author Sean Owen
 */
pub struct GeoParsedResult {
    super: ParsedResult;

     let latitude: f64;

     let longitude: f64;

     let altitude: f64;

     let query: String;
}

impl GeoParsedResult {

    fn new( latitude: f64,  longitude: f64,  altitude: f64,  query: &String) -> GeoParsedResult {
        super(ParsedResultType::GEO);
        let .latitude = latitude;
        let .longitude = longitude;
        let .altitude = altitude;
        let .query = query;
    }

    pub fn  get_geo_u_r_i(&self) -> String  {
         let result: StringBuilder = StringBuilder::new();
        result.append("geo:");
        result.append(self.latitude);
        result.append(',');
        result.append(self.longitude);
        if self.altitude > 0.0 {
            result.append(',');
            result.append(self.altitude);
        }
        if self.query != null {
            result.append('?');
            result.append(&self.query);
        }
        return result.to_string();
    }

    /**
   * @return latitude in degrees
   */
    pub fn  get_latitude(&self) -> f64  {
        return self.latitude;
    }

    /**
   * @return longitude in degrees
   */
    pub fn  get_longitude(&self) -> f64  {
        return self.longitude;
    }

    /**
   * @return altitude in meters. If not specified, in the geo URI, returns 0.0
   */
    pub fn  get_altitude(&self) -> f64  {
        return self.altitude;
    }

    /**
   * @return query string associated with geo URI or null if none exists
   */
    pub fn  get_query(&self) -> String  {
        return self.query;
    }

    pub fn  get_display_result(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(20);
        result.append(self.latitude);
        result.append(", ");
        result.append(self.longitude);
        if self.altitude > 0.0 {
            result.append(", ");
            result.append(self.altitude);
            result.append('m');
        }
        if self.query != null {
            result.append(" (");
            result.append(&self.query);
            result.append(')');
        }
        return result.to_string();
    }
}

