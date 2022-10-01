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

use crate::qrcode::decoder::Version;

use super::Mode;


/**
 * @author Sean Owen
 */

  #[test]
  fn testForBits() {
    assert_eq!(Mode::TERMINATOR, Mode::forBits(0x00).unwrap());
    assert_eq!(Mode::NUMERIC, Mode::forBits(0x01).unwrap());
    assert_eq!(Mode::ALPHANUMERIC, Mode::forBits(0x02).unwrap());
    assert_eq!(Mode::BYTE, Mode::forBits(0x04).unwrap());
    assert_eq!(Mode::KANJI, Mode::forBits(0x08).unwrap());
  }

  #[test]
  #[should_panic]
  fn testBadMode() {
    assert!(Mode::forBits(0x10).is_ok());
  }

  #[test]
  fn testCharacterCount() {
    // Spot check a few values
    assert_eq!(10, Mode::NUMERIC.getCharacterCountBits(Version::getVersionForNumber(5).unwrap()));
    assert_eq!(12, Mode::NUMERIC.getCharacterCountBits(Version::getVersionForNumber(26).unwrap()));
    assert_eq!(14, Mode::NUMERIC.getCharacterCountBits(Version::getVersionForNumber(40).unwrap()));
    assert_eq!(9, Mode::ALPHANUMERIC.getCharacterCountBits(Version::getVersionForNumber(6).unwrap()));
    assert_eq!(8, Mode::BYTE.getCharacterCountBits(Version::getVersionForNumber(7).unwrap()));
    assert_eq!(8, Mode::KANJI.getCharacterCountBits(Version::getVersionForNumber(8).unwrap()));
  }

