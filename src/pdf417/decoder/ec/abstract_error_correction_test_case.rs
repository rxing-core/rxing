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

use crate::common::reedsolomon::ReedSolomonTestCase;

/**
 * @author Sean Owen
 */

pub fn corrupt(received: &mut [u32], howMany: u32, random: &mut rand::rngs::ThreadRng) {
    let mut pass: Vec<i32> = received.iter().map(|x| *x as i32).collect();
    ReedSolomonTestCase::corrupt(&mut pass, howMany as i32, random, 929);
    for i in 0..received.len() {
        received[i] = pass[i] as u32;
    }
}

#[allow(dead_code)]
pub fn erase(received: &mut [u32], howMany: u32, random: &mut rand::rngs::ThreadRng) -> Vec<u32> {
    let mut erased = vec![false; received.len()]; //BitSet::new(received.len());
    let mut erasures = vec![0_u32; howMany as usize];
    let mut erasureOffset = 0;
    let mut j = 0;
    while j < howMany {
        // for (int j = 0; j < howMany; j++) {
        let location = random.gen_range(0..received.len()); //random.nextInt(received.len());
        if *erased.get(location).unwrap() {
            j -= 1;
        } else {
            erased[location] = true;
            received[location] = 0;
            erasures[erasureOffset] = location as u32;
            erasureOffset += 1;
        }
        j += 1;
    }
    erasures
}

pub fn getRandom() -> rand::rngs::ThreadRng {
    rand::thread_rng()
}
