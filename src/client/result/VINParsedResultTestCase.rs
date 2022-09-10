/*
 * Copyright 2014 ZXing authors
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

// import com.google.zxing.BarcodeFormat;
// import com.google.zxing.RXingResult;
// import org.junit.Assert;
// import org.junit.Test;

/**
 * Tests {@link VINParsedRXingResult}.
 */
// public final class VINParsedRXingResultTestCase extends Assert {
use crate::{
    client::result::{ParsedClientResult, ParsedRXingResult, ParsedRXingResultType},
    BarcodeFormat, RXingResult,
};

use super::ResultParser;

#[test]
fn testNotVIN() {
    let fakeRXingResult = RXingResult::new(
        "1M8GDM9A1KP042788",
        Vec::new(),
        Vec::new(),
        BarcodeFormat::CODE_39,
    );
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::TEXT, result.getType());
    let fakeRXingResult = RXingResult::new(
        "1M8GDM9AXKP042788",
        Vec::new(),
        Vec::new(),
        BarcodeFormat::CODE_128,
    );
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::TEXT, result.getType());
}

#[test]
fn testVIN() {
    doTest(
        "1M8GDM9AXKP042788",
        "1M8",
        "GDM9AX",
        "KP042788",
        "US",
        "GDM9A",
        1989,
        'P',
        "042788",
    );
    doTest(
        "I1M8GDM9AXKP042788",
        "1M8",
        "GDM9AX",
        "KP042788",
        "US",
        "GDM9A",
        1989,
        'P',
        "042788",
    );
    doTest(
        "LJCPCBLCX11000237",
        "LJC",
        "PCBLCX",
        "11000237",
        "CN",
        "PCBLC",
        2001,
        '1',
        "000237",
    );
}

fn doTest(
    contents: &str,
    wmi: &str,
    vds: &str,
    vis: &str,
    country: &str,
    attributes: &str,
    year: u32,
    plant: char,
    sequential: &str,
) {
    let fakeRXingResult =
        RXingResult::new(contents, Vec::new(), Vec::new(), BarcodeFormat::CODE_39);
    let result = ResultParser::parseRXingResult(&fakeRXingResult);
    assert_eq!(ParsedRXingResultType::VIN, result.getType());
    if let ParsedClientResult::VINResult(vinRXingResult) = result {
        // let vinRXingResult = (VINParsedRXingResult) result;
        assert_eq!(wmi, vinRXingResult.getWorldManufacturerID());
        assert_eq!(vds, vinRXingResult.getVehicleDescriptorSection());
        assert_eq!(vis, vinRXingResult.getVehicleIdentifierSection());
        assert_eq!(country, vinRXingResult.getCountryCode());
        assert_eq!(attributes, vinRXingResult.getVehicleAttributes());
        assert_eq!(year, vinRXingResult.getModelYear());
        assert_eq!(plant, vinRXingResult.getPlantCode());
        assert_eq!(sequential, vinRXingResult.getSequentialNumber());
    } else {
        panic!("Expected VINResult");
    }
}
