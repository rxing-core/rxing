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
 * @author dswitkin@google.com (Daniel Switkin)
 */

#[test]
fn QRCodeBlackBox3TestCase() {
  let mut tester = common::AbstractBlackBoxTestCase::new("test_resources/blackbox/qrcode-3", Box::new(QRCodeReader{}), BarcodeFormat::QR_CODE);
  tester.addTest(38, 38, 0.0);
  tester.addTest(39, 39, 90.0);
  tester.addTest(36, 36, 180.0);
  tester.addTest(39, 39, 270.0);

  tester.testBlackBox();
  }

// 11 - 90
// 13 - 90
