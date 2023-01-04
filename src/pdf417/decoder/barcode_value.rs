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

use std::collections::HashMap;

/**
 * @author Guenther Grau
 */
#[derive(Clone, Default)]
pub struct BarcodeValue(HashMap<u32, u32>);
// private final Map<Integer,Integer> values = new HashMap<>();

impl BarcodeValue {
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * Add an occurrence of a value
     */
    pub fn setValue(&mut self, value: u32) {
        let mut confidence = if let Some(value) = self.0.get(&value) {
            *value
        } else {
            0
        };
        confidence += 1;
        self.0.insert(value, confidence);
    }

    /**
     * Determines the maximum occurrence of a set value and returns all values which were set with this occurrence.
     * @return an array of int, containing the values with the highest occurrence, or null, if no value was set
     */
    pub fn getValue(&self) -> Vec<u32> {
        let mut maxConfidence = -1_i32;
        let mut result = Vec::new();
        for (key, value) in &self.0 {
            // for (Entry<Integer,Integer> entry : values.entrySet()) {
            if *value as i32 > maxConfidence {
                maxConfidence = *value as i32;
                result.clear();
                result.push(*key);
            } else if *value as i32 == maxConfidence {
                result.push(*key);
            }
        }

        result
    }

    pub fn getConfidence(&self, value: u32) -> u32 {
        if let Some(v) = self.0.get(&value) {
            *v
        } else {
            0
        }
    }
}
