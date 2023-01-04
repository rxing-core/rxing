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

// import java.util.Random;

// /**
//  * @author Sean Owen
//  */
// public final class BitArrayTestCase extends Assert {

use super::BitArray;
use rand::Rng;

#[test]
fn test_get_set() {
    let mut array = BitArray::with_size(33);
    for i in 0..33 {
        // for (int i = 0; i < 33; i++) {
        assert!(!array.get(i));
        array.set(i);
        assert!(array.get(i));
    }
}

#[test]
fn test_get_next_set1() {
    let array = BitArray::with_size(32);
    for i in 0..array.getSize() {
        // for (int i = 0; i < array.getSize(); i++) {
        assert_eq!(32, array.getNextSet(i), "{}", i);
    }
    let array = BitArray::with_size(33);
    for i in 0..array.getSize() {
        // for (int i = 0; i < array.getSize(); i++) {
        assert_eq!(33, array.getNextSet(i), "{}", i);
    }
}

#[test]
fn test_get_next_set2() {
    let mut array = BitArray::with_size(33);
    array.set(31);
    for i in 0..array.getSize() {
        // for (int i = 0; i < array.getSize(); i++) {
        assert_eq!(if i <= 31 { 31 } else { 33 }, array.getNextSet(i), "{}", i);
    }
    array = BitArray::with_size(33);
    array.set(32);
    for i in 0..array.getSize() {
        // for (int i = 0; i < array.getSize(); i++) {
        assert_eq!(32, array.getNextSet(i), "{}", i);
    }
}

#[test]
fn test_get_next_set3() {
    let mut array = BitArray::with_size(63);
    array.set(31);
    array.set(32);
    for i in 0..array.getSize() {
        // for (int i = 0; i < array.getSize(); i++) {
        let expected;
        if i <= 31 {
            expected = 31;
        } else if i == 32 {
            expected = 32;
        } else {
            expected = 63;
        }
        assert_eq!(expected, array.getNextSet(i), "{}", i);
    }
}

#[test]
fn test_get_next_set4() {
    let mut array = BitArray::with_size(63);
    array.set(33);
    array.set(40);
    for i in 0..array.getSize() {
        // for (int i = 0; i < array.getSize(); i++) {
        let expected;
        if i <= 33 {
            expected = 33;
        } else if i <= 40 {
            expected = 40;
        } else {
            expected = 63;
        }
        assert_eq!(expected, array.getNextSet(i), "{}", i);
    }
}

#[test]
fn test_get_next_set5() {
    let mut r = rand::thread_rng();
    for _i in 0..10 {
        // for (int i = 0; i < 10; i++) {
        let mut array = BitArray::with_size(1 + r.gen_range(0..100));
        let numSet = r.gen_range(0..20);
        for _j in 0..numSet {
            // for (int j = 0; j < numSet; j++) {
            array.set(r.gen_range(0..array.getSize()));
        }
        let numQueries = r.gen_range(0..20);
        for _j in 0..numQueries {
            // for (int j = 0; j < numQueries; j++) {
            let query = r.gen_range(0..array.getSize());
            let mut expected = query;
            while expected < array.getSize() && !array.get(expected) {
                expected += 1;
            }
            let actual = array.getNextSet(query);
            assert_eq!(expected, actual);
        }
    }
}

#[test]
fn test_set_bulk() {
    let mut array = BitArray::with_size(64);
    array.setBulk(32, 0xFFFF0000);
    for i in 0..48 {
        // for (int i = 0; i < 48; i++) {
        assert!(!array.get(i));
    }
    for i in 48..64 {
        // for (int i = 48; i < 64; i++) {
        assert!(array.get(i));
    }
}

#[test]
fn test_append_bit() {
    let mut array = BitArray::new();
    array.appendBits(0x000001E, 6).expect("must append)");
    let mut array_2 = BitArray::new();
    array_2.appendBit(false);
    array_2.appendBit(true);
    array_2.appendBit(true);
    array_2.appendBit(true);
    array_2.appendBit(true);
    array_2.appendBit(false);

    assert_eq!(array, array_2)
}

#[test]
fn test_set_range() {
    let mut array = BitArray::with_size(64);
    array.setRange(28, 36).unwrap();
    assert!(!array.get(27));
    for i in 28..36 {
        // for (int i = 28; i < 36; i++) {
        assert!(array.get(i));
    }
    assert!(!array.get(36));
}

#[test]
fn test_clear() {
    let mut array = BitArray::with_size(32);
    for i in 0..32 {
        // for (int i = 0; i < 32; i++) {
        array.set(i);
    }
    array.clear();
    for i in 0..32 {
        // for (int i = 0; i < 32; i++) {
        assert!(!array.get(i));
    }
}

#[test]
fn test_flip() {
    let mut array = BitArray::with_size(32);
    assert!(!array.get(5));
    array.flip(5);
    assert!(array.get(5));
    array.flip(5);
    assert!(!array.get(5));
}

#[test]
fn test_get_array() {
    let mut array = BitArray::with_size(64);
    array.set(0);
    array.set(63);
    let ints = array.getBitArray();
    assert_eq!(1, ints[0]);
    assert_eq!(0b10_00_00_00_00_00_00_00_00_00_00_00_00_00_00_00, ints[1]);
}

#[test]
fn test_is_range() {
    let mut array = BitArray::with_size(64);
    assert!(array.isRange(0, 64, false).unwrap());
    assert!(!array.isRange(0, 64, true).unwrap());
    array.set(32);
    assert!(array.isRange(32, 33, true).unwrap());
    array.set(31);
    assert!(array.isRange(31, 33, true).unwrap());
    array.set(34);
    assert!(!array.isRange(31, 35, true).unwrap());
    for i in 0..31 {
        // for (int i = 0; i < 31; i++) {
        array.set(i);
    }
    assert!(array.isRange(0, 33, true).unwrap());
    for i in 33..64 {
        // for (int i = 33; i < 64; i++) {
        array.set(i);
    }
    assert!(array.isRange(0, 64, true).unwrap());
    assert!(!array.isRange(0, 64, false).unwrap());
}

#[test]
fn reverse_algorithm_test() {
    let oldBits: Vec<u32> = vec![128, 256, 512, 6453324, 50934953];
    for size in 1..160 {
        // for (int size = 1; size < 160; size++) {
        let newBitsOriginal = reverse_original(&oldBits.clone(), size);
        let mut newBitArray = BitArray::with_initial_values(oldBits.clone(), size);
        newBitArray.reverse();
        let newBitsNew = newBitArray.getBitArray();
        assert!(arrays_are_equal(
            &newBitsOriginal,
            newBitsNew,
            size / 32 + 1
        ));
    }
}

#[test]
fn test_clone() {
    let array = BitArray::with_size(32);
    array.clone().set(0);
    assert!(!array.get(0));
}

#[test]
fn test_equals() {
    let mut a = BitArray::with_size(32);
    let mut b = BitArray::with_size(32);
    assert_eq!(a, b);
    // assert_eq!(a.hash(), b.hash());
    assert_ne!(a, BitArray::with_size(31));
    a.set(16);
    assert_ne!(a, b);
    // assert_ne!(a.hash(), b.hash());
    b.set(16);
    assert_eq!(a, b);
    // assert_eq!(a.hash(), b.hash());
}

fn reverse_original(oldBits: &[u32], size: usize) -> Vec<u32> {
    let mut newBits = vec![0; oldBits.len()];
    for i in 0..size {
        // for (int i = 0; i < size; i++) {
        if bit_set(oldBits, size - i - 1) {
            newBits[i / 32_usize] |= 1 << (i & 0x1F);
        }
    }
    newBits
}

fn bit_set(bits: &[u32], i: usize) -> bool {
    (bits[i / 32] & (1 << (i & 0x1F))) != 0
}

fn arrays_are_equal(left: &[u32], right: &[u32], size: usize) -> bool {
    for i in 0..size {
        // for (int i = 0; i < size; i++) {
        if left[i] != right[i] {
            return false;
        }
    }
    true
}

// }
