/*
 * Copyright 2012 ZXing authors
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

use rand::Rng;

use crate::{datamatrix::encoder::error_correction, Exceptions};

use super::{
    abstract_error_correction_test_case::{corrupt, getRandom},
    error_correction::decode,
};

/**
 * @author Sean Owen
 */

const PDF417_TEST: [u32; 48] = [
    48, 901, 56, 141, 627, 856, 330, 69, 244, 900, 852, 169, 843, 895, 852, 895, 913, 154, 845,
    778, 387, 89, 869, 901, 219, 474, 543, 650, 169, 201, 9, 160, 35, 70, 900, 900, 900, 900, 900,
    900, 900, 900, 900, 900, 900, 900, 900, 900,
];
const PDF417_TEST_WITH_EC: [u32; 112] = [
    48, 901, 56, 141, 627, 856, 330, 69, 244, 900, 852, 169, 843, 895, 852, 895, 913, 154, 845,
    778, 387, 89, 869, 901, 219, 474, 543, 650, 169, 201, 9, 160, 35, 70, 900, 900, 900, 900, 900,
    900, 900, 900, 900, 900, 900, 900, 900, 900, 769, 843, 591, 910, 605, 206, 706, 917, 371, 469,
    79, 718, 47, 777, 249, 262, 193, 620, 597, 477, 450, 806, 908, 309, 153, 871, 686, 838, 185,
    674, 68, 679, 691, 794, 497, 479, 234, 250, 496, 43, 347, 582, 882, 536, 322, 317, 273, 194,
    917, 237, 420, 859, 340, 115, 222, 808, 866, 836, 417, 121, 833, 459, 64, 159,
];
const ECC_BYTES: usize = PDF417_TEST_WITH_EC.len() - PDF417_TEST.len();
const ERROR_LIMIT: usize = ECC_BYTES;
const MAX_ERRORS: usize = ERROR_LIMIT / 2;
const _MAX_ERASURES: usize = ERROR_LIMIT;

// private final ErrorCorrection ec = new ErrorCorrection();

#[test]
fn testNoError() {
    let mut received = PDF417_TEST_WITH_EC.clone();
    // no errors
    checkDecode(&mut received).expect("ok");
}

#[test]
fn testExplicitError() {
    let mut random = getRandom();
    for i in 0..PDF417_TEST_WITH_EC.len() {
        // for (int i = 0; i < PDF417_TEST_WITH_EC.length; i++) {
        let mut received = PDF417_TEST_WITH_EC.clone();
        received[i] = 610; //random.gen_range(0..256);// random.nextInt(256);
        checkDecode(&mut received).expect("ok");
    }
}

#[test]
fn testOneError() {
    let mut random = getRandom();
    for i in 0..PDF417_TEST_WITH_EC.len() {
        // for (int i = 0; i < PDF417_TEST_WITH_EC.length; i++) {
        let mut received = PDF417_TEST_WITH_EC.clone();
        received[i] = random.gen_range(0..256); //random.gen_range(0..256);// random.nextInt(256);
        checkDecode(&mut received).expect("ok");
    }
}

#[test]
fn testMaxErrors() {
    let mut random = getRandom();
    for _testIterations in 0..100 {
        // for (int testIterations = 0; testIterations < 100; testIterations++) { // # iterations is kind of arbitrary
        let mut received = PDF417_TEST_WITH_EC.clone();
        corrupt(&mut received, MAX_ERRORS as u32, &mut random);
        checkDecode(&mut received).expect("ok");
    }
}

#[test]
fn testTooManyErrors() {
    let mut received = PDF417_TEST_WITH_EC.clone();
    let mut random = getRandom();
    corrupt(&mut received, MAX_ERRORS as u32 + 1, &mut random);
    // try {
    assert!(checkDecode(&mut received).is_err());
    //   fail("Should not have decoded");
    // } catch (ChecksumException ce) {
    //   // good
    // }
}

fn checkDecode(received: &mut [u32]) -> Result<(), Exceptions> {
    checkDecodeErasures(received, &mut [0_u32; 0])
}

fn checkDecodeErasures(received: &mut [u32], erasures: &mut [u32]) -> Result<(), Exceptions> {
    decode(received, ECC_BYTES as u32, erasures)?;
    // ec.decode(received, ECC_BYTES, erasures);
    for i in 0..PDF417_TEST.len() {
        // for (int i = 0; i < PDF417_TEST.length; i++) {
        assert_eq!(received[i], PDF417_TEST[i]);
    }
    Ok(())
}
