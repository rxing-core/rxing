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

use rxing::qrcode::QRCodeReader;

mod common;

/**
 * @author Sean Owen
 */

#[test]
fn QRCodeBlackBox1TestCase() {
    let mut tester = common::AbstractBlackBoxTestCase::new(
        "test_resources/blackbox/qrcode-1",
        Box::new(QRCodeReader {}),
        rxing::BarcodeFormat::QR_CODE,
    );
    // super("src/test/resources/blackbox/qrcode-1", new MultiFormatReader(), BarcodeFormat.QR_CODE);
    tester.addTest(17, 17, 0.0);
    tester.addTest(14, 14, 90.0);
    tester.addTest(17, 17, 180.0);
    tester.addTest(14, 14, 270.0);

    tester.testBlackBox();
}


// TEST CASE 15 FAILING AT allignment patter finder! (line 88) FROM detector 486

/*
 JAVA RESULTS::::

Atul  Starting src/test/resources/blackbox/qrcode-1/2.png
  Starting src/test/resources/blackbox/qrcode-1/9.png
  Starting src/test/resources/blackbox/qrcode-1/16.png
  could not read at rotation 90.000000
  could not read at rotation 90.000000 w/TH
  could not read at rotation 270.000000
  could not read at rotation 270.000000 w/TH
  Starting src/test/resources/blackbox/qrcode-1/11.png
  Starting src/test/resources/blackbox/qrcode-1/5.png
  Starting src/test/resources/blackbox/qrcode-1/3.png
  Starting src/test/resources/blackbox/qrcode-1/6.png
  Starting src/test/resources/blackbox/qrcode-1/14.png
  could not read at rotation 0.000000
  could not read at rotation 0.000000 w/TH
  could not read at rotation 90.000000
  could not read at rotation 90.000000 w/TH
  could not read at rotation 180.000000
  could not read at rotation 180.000000 w/TH
  could not read at rotation 270.000000
  could not read at rotation 270.000000 w/TH
  Starting src/test/resources/blackbox/qrcode-1/8.png
  Starting src/test/resources/blackbox/qrcode-1/13.png
  could not read at rotation 0.000000
  could not read at rotation 0.000000 w/TH
  could not read at rotation 90.000000
  could not read at rotation 90.000000 w/TH
  could not read at rotation 180.000000
  could not read at rotation 180.000000 w/TH
  could not read at rotation 270.000000
  could not read at rotation 270.000000 w/TH
  Starting src/test/resources/blackbox/qrcode-1/17.png
  could not read at rotation 90.000000
  could not read at rotation 90.000000 w/TH
  Starting src/test/resources/blackbox/qrcode-1/12.png
  Starting src/test/resources/blackbox/qrcode-1/7.png
  Starting src/test/resources/blackbox/qrcode-1/4.png
  Starting src/test/resources/blackbox/qrcode-1/19.png
  Starting src/test/resources/blackbox/qrcode-1/18.png
  could not read at rotation 0.000000
  could not read at rotation 0.000000 w/TH
  could not read at rotation 90.000000
  could not read at rotation 90.000000 w/TH
  could not read at rotation 180.000000
  could not read at rotation 180.000000 w/TH
  could not read at rotation 270.000000
  could not read at rotation 270.000000 w/TH
  Starting src/test/resources/blackbox/qrcode-1/1.png
  Starting src/test/resources/blackbox/qrcode-1/15.png
  could not read at rotation 90.000000
  could not read at rotation 90.000000 w/TH
  could not read at rotation 270.000000
  could not read at rotation 270.000000 w/TH
  Starting src/test/resources/blackbox/qrcode-1/10.png
  Starting src/test/resources/blackbox/qrcode-1/20.png
  could not read at rotation 270.000000
  could not read at rotation 270.000000 w/TH
  Rotation 0 degrees:
   17 of 20 images passed (17 required)
   0 failed due to misreads, 3 not detected
   17 of 20 images passed with try harder (17 required)
   0 failed due to misreads, 3 not detected
  Rotation 90 degrees:
   14 of 20 images passed (14 required)
   0 failed due to misreads, 6 not detected
   14 of 20 images passed with try harder (14 required)
   0 failed due to misreads, 6 not detected
  Rotation 180 degrees:
   17 of 20 images passed (17 required)
   0 failed due to misreads, 3 not detected
   17 of 20 images passed with try harder (17 required)
   0 failed due to misreads, 3 not detected
  Rotation 270 degrees:
   14 of 20 images passed (14 required)
   0 failed due to misreads, 6 not detected
   14 of 20 images passed with try harder (14 required)
   0 failed due to misreads, 6 not detected
  Decoded 124 images out of 160 (77%, 124 required)



*/