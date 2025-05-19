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

// import java.util.Locale;

// import com.google.zxing.BarcodeFormat;
// import com.google.zxing.RXingResult;
// import org.junit.Assert;
// import org.junit.Test;

/**
 * Tests {@link GeoParsedRXingResult}.
 *
 * @author Sean Owen
 */
// public final class GeoParsedRXingResultTestCase extends Assert {
use crate::{
    client::result::{ParsedClientResult, ParsedRXingResult, ParsedRXingResultType, ResultParser},
    BarcodeFormat, RXingResult,
};

const EPSILON: f64 = 1.0E-10;

#[test]
pub fn testGeo1() {
    //doTest("geo:1,2", 1.0, 2.0, 0.0, "", "geo:1.0,2.0");
    // I think 1.0 and 1 and 2.0 and 2 are similar enough here, correct me if i'm wrong.
    doTest("geo:1,2", 1.0, 2.0, 0.0, "", "geo:1,2");
}
#[test]
pub fn testGeo2() {
    doTest("geo:80.33,-32.3344,3.35", 80.33, -32.3344, 3.35, "", "");
}
#[test]
pub fn testGeo() {
    doTest("geo:-20.33,132.3344,0.01", -20.33, 132.3344, 0.01, "", "");
}
#[test]
pub fn testGeo3() {
    doTest(
        "geo:-20.33,132.3344,0.01?q=foobar",
        -20.33,
        132.3344,
        0.01,
        "q=foobar",
        "",
    );
}
#[test]
pub fn testGeo4() {
    doTest(
        "GEO:-20.33,132.3344,0.01?q=foobar",
        -20.33,
        132.3344,
        0.01,
        "q=foobar",
        "",
    );
}

fn doTest(contents: &str, latitude: f64, longitude: f64, altitude: f64, query: &str, uri: &str) {
    let fakeRXingResult =
        RXingResult::new(contents, Vec::new(), Vec::new(), BarcodeFormat::QR_CODE);
    let result = ResultParser::parseRXingResult(&fakeRXingResult);

    assert_eq!(ParsedRXingResultType::Geo, result.getType());

    if let ParsedClientResult::GeoResult(geoRXingResult) = result {
        assert!(within_range(
            latitude,
            geoRXingResult.getLatitude(),
            EPSILON
        ));
        assert!(within_range(
            longitude,
            geoRXingResult.getLongitude(),
            EPSILON
        ));
        assert!(within_range(
            altitude,
            geoRXingResult.getAltitude(),
            EPSILON
        ));
        assert_eq!(query, geoRXingResult.getQuery());
        assert_eq!(
            if uri.is_empty() {
                contents.to_lowercase()
            } else {
                String::from(uri)
            },
            geoRXingResult.getGeoURI()
        );
    } else {
        panic!("Expected ParsedClientResult::GeoResult");
    }
}

fn within_range(a: f64, b: f64, delta: f64) -> bool {
    a - b < delta || b - a < delta
}

// }
