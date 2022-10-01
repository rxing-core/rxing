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

use crate::{qrcode::decoder::{Version, ErrorCorrectionLevel}, Exceptions};


/**
 * @author Sean Owen
 */


  #[test]
  #[should_panic]
  fn testBadVersion() {
    assert!(Version::getVersionForNumber(0).is_ok());
  }

  #[test]
  fn testVersionForNumber() {
    for i in 1..=40 {
    // for (int i = 1; i <= 40; i++) {
      checkVersion(Version::getVersionForNumber(i), i, 4 * i + 17);
    }
  }

  fn checkVersion( version:Result<&Version,Exceptions>,  number:u32,  dimension:u32) {
    assert!(version.is_ok());
    let version = version.unwrap();
    assert_eq!(number, version.getVersionNumber());
    // assertNotNull(version.getAlignmentPatternCenters());
    if (number > 1) {
      assert!(version.getAlignmentPatternCenters().len() > 0);
    }
    assert_eq!(dimension, version.getDimensionForVersion());
    let _tmp = version.getECBlocksForLevel(ErrorCorrectionLevel::H);
    let _tmp = version.getECBlocksForLevel(ErrorCorrectionLevel::L);
    let _tmp = version.getECBlocksForLevel(ErrorCorrectionLevel::M);
    let _tmp = version.getECBlocksForLevel(ErrorCorrectionLevel::Q);
    let _tmp = version.buildFunctionPattern();

    // assertNotNull(version.getECBlocksForLevel(ErrorCorrectionLevel::H));
    // assertNotNull(version.getECBlocksForLevel(ErrorCorrectionLevel::L));
    // assertNotNull(version.getECBlocksForLevel(ErrorCorrectionLevel::M));
    // assertNotNull(version.getECBlocksForLevel(ErrorCorrectionLevel::Q));
    // assertNotNull(version.buildFunctionPattern());
  }

  #[test]
  fn testGetProvisionalVersionForDimension()  {
    for i in 1..=40 {
    // for (int i = 1; i <= 40; i++) {
      assert_eq!(i, Version::getProvisionalVersionForDimension(4 * i + 17).expect("must exist for supplied values").getVersionNumber());
    }
  }

  #[test]
  fn testDecodeVersionInformation() {
    // Spot check
    doTestVersion(7, 0x07C94);
    doTestVersion(12, 0x0C762);
    doTestVersion(17, 0x1145D);
    doTestVersion(22, 0x168C9);
    doTestVersion(27, 0x1B08E);
    doTestVersion(32, 0x209D5);
  }
  
  fn doTestVersion( expectedVersion:u32,  mask:u32) {
    let version = Version::decodeVersionInformation(mask);
    assert!(version.is_ok());
    assert_eq!(expectedVersion, version.unwrap().getVersionNumber());
  }

