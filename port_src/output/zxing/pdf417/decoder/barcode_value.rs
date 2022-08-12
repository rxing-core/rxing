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
// package com::google::zxing::pdf417::decoder;

/**
 * @author Guenther Grau
 */
struct BarcodeValue {

     let values: Map<Integer, Integer> = HashMap<>::new();
}

impl BarcodeValue {

    /**
   * Add an occurrence of a value
   */
    fn  set_value(&self,  value: i32)   {
         let mut confidence: Integer = self.values.get(value);
        if confidence == null {
            confidence = 0;
        }
        confidence += 1;
        self.values.put(value, &confidence);
    }

    /**
   * Determines the maximum occurrence of a set value and returns all values which were set with this occurrence.
   * @return an array of int, containing the values with the highest occurrence, or null, if no value was set
   */
    fn  get_value(&self) -> Vec<i32>  {
         let max_confidence: i32 = -1;
         let result: Collection<Integer> = ArrayList<>::new();
        for  let entry: Entry<Integer, Integer> in self.values.entry_set() {
            if entry.get_value() > max_confidence {
                max_confidence = entry.get_value();
                result.clear();
                result.add(&entry.get_key());
            } else if entry.get_value() == max_confidence {
                result.add(&entry.get_key());
            }
        }
        return PDF417Common::to_int_array(&result);
    }

    fn  get_confidence(&self,  value: i32) -> Integer  {
        return self.values.get(value);
    }
}

