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

use crate::common::BitArray;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */

fn getUnsignedInt(v: &BitArray) -> u64 {
    let mut result = 0u64;
    const OFFSET: usize = 0;
    for i in 0..32 {
        // for (int i = 0, offset = 0; i < 32; i++) {
        if v.get(OFFSET + i) {
            result |= 1 << (31 - i);
        }
    }
    result
}

#[test]
fn testAppendBit() {
    let mut v = BitArray::new();
    assert_eq!(0, v.getSizeInBytes());
    // 1
    v.appendBit(true);
    assert_eq!(1, v.getSize());
    assert_eq!(0x80000000, getUnsignedInt(&v));
    // 10
    v.appendBit(false);
    assert_eq!(2, v.getSize());
    assert_eq!(0x80000000, getUnsignedInt(&v));
    // 101
    v.appendBit(true);
    assert_eq!(3, v.getSize());
    assert_eq!(0xa0000000, getUnsignedInt(&v));
    // 1010
    v.appendBit(false);
    assert_eq!(4, v.getSize());
    assert_eq!(0xa0000000, getUnsignedInt(&v));
    // 10101
    v.appendBit(true);
    assert_eq!(5, v.getSize());
    assert_eq!(0xa8000000, getUnsignedInt(&v));
    // 101010
    v.appendBit(false);
    assert_eq!(6, v.getSize());
    assert_eq!(0xa8000000, getUnsignedInt(&v));
    // 1010101
    v.appendBit(true);
    assert_eq!(7, v.getSize());
    assert_eq!(0xaa000000, getUnsignedInt(&v));
    // 10101010
    v.appendBit(false);
    assert_eq!(8, v.getSize());
    assert_eq!(0xaa000000, getUnsignedInt(&v));
    // 10101010 1
    v.appendBit(true);
    assert_eq!(9, v.getSize());
    assert_eq!(0xaa800000, getUnsignedInt(&v));
    // 10101010 10
    v.appendBit(false);
    assert_eq!(10, v.getSize());
    assert_eq!(0xaa800000, getUnsignedInt(&v));
}

#[test]
fn testAppendBits() {
    let mut v = BitArray::new();
    v.appendBits(0x1, 1).expect("append");
    assert_eq!(1, v.getSize());
    assert_eq!(0x80000000, getUnsignedInt(&v));
    let mut v = BitArray::new();
    v.appendBits(0xff, 8).expect("append");
    assert_eq!(8, v.getSize());
    assert_eq!(0xff000000, getUnsignedInt(&v));
    let mut v = BitArray::new();
    v.appendBits(0xff7, 12).expect("append");
    assert_eq!(12, v.getSize());
    assert_eq!(0xff700000, getUnsignedInt(&v));
}

#[test]
fn testNumBytes() {
    let mut v = BitArray::new();
    assert_eq!(0, v.getSizeInBytes());
    v.appendBit(false);
    // 1 bit was added in the vector, so 1 byte should be consumed.
    assert_eq!(1, v.getSizeInBytes());
    v.appendBits(0, 7).expect("append");
    assert_eq!(1, v.getSizeInBytes());
    v.appendBits(0, 8).expect("append");
    assert_eq!(2, v.getSizeInBytes());
    v.appendBits(0, 1).expect("append");
    // We now have 17 bits, so 3 bytes should be consumed.
    assert_eq!(3, v.getSizeInBytes());
}

#[test]
fn testAppendBitVector() {
    let mut v1 = BitArray::new();
    v1.appendBits(0xbe, 8).expect("append");
    let mut v2 = BitArray::new();
    v2.appendBits(0xef, 8).expect("append");
    v1.appendBitArray(v2);
    // beef = 1011 1110 1110 1111
    assert_eq!(" X.XXXXX. XXX.XXXX", v1.to_string());
}

#[test]
fn testXOR() {
    let mut v1 = BitArray::new();
    v1.appendBits(0x5555aaaa, 32).expect("append");
    let mut v2 = BitArray::new();
    v2.appendBits(0xaaaa5555, 32).expect("append");
    v1.xor(&v2).expect("xor");
    assert_eq!(0xffffffff, getUnsignedInt(&v1));
}

#[test]
fn testXOR2() {
    let mut v1 = BitArray::new();
    v1.appendBits(0x2a, 7).expect("append"); // 010 1010
    let mut v2 = BitArray::new();
    v2.appendBits(0x55, 7).expect("append"); // 101 0101
    v1.xor(&v2).expect("xor");
    assert_eq!(0xfe000000, getUnsignedInt(&v1)); // 1111 1110
}

#[test]
fn testAt() {
    let mut v = BitArray::new();
    v.appendBits(0xdead, 16).expect("append"); // 1101 1110 1010 1101
    assert!(v.get(0));
    assert!(v.get(1));
    assert!(!v.get(2));
    assert!(v.get(3));

    assert!(v.get(4));
    assert!(v.get(5));
    assert!(v.get(6));
    assert!(!v.get(7));

    assert!(v.get(8));
    assert!(!v.get(9));
    assert!(v.get(10));
    assert!(!v.get(11));

    assert!(v.get(12));
    assert!(v.get(13));
    assert!(!v.get(14));
    assert!(v.get(15));
}

#[test]
fn testToString() {
    let mut v = BitArray::new();
    v.appendBits(0xdead, 16).expect("append"); // 1101 1110 1010 1101
    assert_eq!(" XX.XXXX. X.X.XX.X", v.to_string());
}
