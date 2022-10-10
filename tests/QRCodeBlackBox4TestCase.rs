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

use rxing::{qrcode::QRCodeReader, BarcodeFormat};

mod common;

/**
 * Tests of various QR Codes from t-shirts, which are notoriously not flat.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[test]
fn  QRCodeBlackBox4TestCase() {
  let mut tester = common::AbstractBlackBoxTestCase::new("test_resources/blackbox/qrcode-4", Box::new(QRCodeReader{}), BarcodeFormat::QR_CODE);
  tester.addTest(36, 36, 0.0);
  tester.addTest(35, 35, 90.0);
  tester.addTest(35, 35, 180.0);
  tester.addTest(35, 35, 270.0);

  tester.testBlackBox();
  }

