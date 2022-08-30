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

// package com.google.zxing.common;

// import org.junit.Assert;
// import org.junit.Test;

// /**
//  * @author Sean Owen
//  */
// public final class BitSourceTestCase extends Assert {

use super::BitSource;

  #[test]
  fn test_source() {
    let bytes:Vec<u8> = vec![ 1,  2,  3,  4,  5];
    let mut source =  BitSource::new(bytes);
    assert_eq!(40, source.available());
    assert_eq!(0, source.readBits(1).unwrap());
    assert_eq!(39, source.available());
    assert_eq!(0, source.readBits(6).unwrap());
    assert_eq!(33, source.available());
    assert_eq!(1, source.readBits(1).unwrap());
    assert_eq!(32, source.available());
    assert_eq!(2, source.readBits(8).unwrap());
    assert_eq!(24, source.available());
    assert_eq!(12, source.readBits(10).unwrap());
    assert_eq!(14, source.available());
    assert_eq!(16, source.readBits(8).unwrap());
    assert_eq!(6, source.available());
    assert_eq!(5, source.readBits(6).unwrap());
    assert_eq!(0, source.available());
  }

// }