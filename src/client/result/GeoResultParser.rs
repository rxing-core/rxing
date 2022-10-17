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

// import com.google.zxing.RXingResult;

// import java.util.regex.Matcher;
// import java.util.regex.Pattern;

use super::{GeoParsedRXingResult, ParsedClientResult, ResultParser};

use lazy_static::lazy_static;

lazy_static! {
    static ref GEO_URL: regex::Regex = regex::Regex::new(GEO_URL_PATTERN).unwrap();
}

const GEO_URL_PATTERN: &'static str = "geo:([\\-0-9.]+),([\\-0-9.]+)(?:,([\\-0-9.]+))?(?:\\?(.*))?";

/**
 * Parses a "geo:" URI result, which specifies a location on the surface of
 * the Earth as well as an optional altitude above the surface. See
 * <a href="http://tools.ietf.org/html/draft-mayrhofer-geo-uri-00">
 * http://tools.ietf.org/html/draft-mayrhofer-geo-uri-00</a>.
 *
 * @author Sean Owen
 */
// pub struct GeoRXingResultParser {}

// impl RXingResultParser for GeoRXingResultParser {
pub fn parse(theRXingResult: &crate::RXingResult) -> Option<super::ParsedClientResult> {
    let rawText = ResultParser::getMassagedText(theRXingResult);

    if let Some(captures) = GEO_URL.captures(&rawText.to_lowercase()) {
        let query = if let Some(q) = captures.get(4) {
            q.as_str()
        } else {
            ""
        };

        let latitude = if let Some(la) = captures.get(1) {
            if let Ok(laf64) = la.as_str().parse::<f64>() {
                if laf64 > 90.0 || laf64 < -90.0 {
                    return None;
                }
                laf64
            } else {
                return None;
            }
        } else {
            return None;
        };

        let longitude = if let Some(lo) = captures.get(2) {
            if let Ok(lof64) = lo.as_str().parse::<f64>() {
                if lof64 > 180.0 || lof64 < -180.0 {
                    return None;
                }
                lof64
            } else {
                return None;
            }
        } else {
            return None;
        };

        let altitude = if let Some(al) = captures.get(3) {
            if let Ok(alf64) = al.as_str().parse::<f64>() {
                if alf64 < 0.0 {
                    return None;
                }
                alf64
            } else {
                return None;
            }
        } else {
            0.0
        };
        // let longitude;
        // let altitude;
        // try {
        //   // latitude = Double.parseDouble(matcher.group(1));
        //   // if (latitude > 90.0 || latitude < -90.0) {
        //   //   return null;
        //   // }
        //   // longitude = Double.parseDouble(matcher.group(2));
        //   // if (longitude > 180.0 || longitude < -180.0) {
        //   //   return null;
        //   // }
        //   // if (matcher.group(3) == null) {
        //   //   altitude = 0.0;
        //   // } else {
        //   //   altitude = Double.parseDouble(matcher.group(3));
        //   //   if (altitude < 0.0) {
        //   //     return null;
        //   //   }
        //   // }
        // } catch (NumberFormatException ignored) {
        //   return null;
        // }
        Some(ParsedClientResult::GeoResult(GeoParsedRXingResult::new(
            latitude,
            longitude,
            altitude,
            String::from(query),
        )))
    } else {
        return None;
    }

    // Matcher matcher = GEO_URL_PATTERN.matcher(rawText);
    // if (!matcher.matches()) {
    //   return null;
    // }
}
// }
