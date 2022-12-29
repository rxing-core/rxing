/*
 * Copyright 2013 ZXing authors
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
mod common;

use rxing::pdf417::PDF417Reader;

/**
 * This class tests Macro PDF417 barcode specific functionality. It ensures that information, which is split into
 * several barcodes can be properly combined again to yield the original data content.
 *
 * @author Guenther Grau
 */
#[test]
fn pdf417_black_box4_test_case() {
    let mut tester = common::PDF417MultiImageSpanAbstractBlackBoxTestCase::new(
        "test_resources/blackbox/pdf417-4",
        PDF417Reader::default(),
        rxing::BarcodeFormat::PDF_417,
    );

    // super("src/test/resources/blackbox/pdf417-4", null, BarcodeFormat.PDF_417);
    tester.add_test_complex(3, 3, 0, 0, 0.0);

    tester.test_black_box()
}
