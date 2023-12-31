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

use rxing::{oned::TelepenReader, BarcodeFormat};

mod common;

/**
 * @author Chris Wood
 */
#[test]
fn telepen_black_box1_test_case() {
    const NUMTESTS: u32 = 5;

    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/telepen-1",
        TelepenReader::new(),
        BarcodeFormat::TELEPEN,
    );
    // super("src/test/resources/blackbox/Telepen-1", new MultiFormatReader(), BarcodeFormat.Telepen);
    tester.add_test(NUMTESTS, 2, 0.0);
    tester.add_test(NUMTESTS, 2, 180.0);

    tester.test_black_box();
}
