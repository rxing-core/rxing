/*
 * Copyright 2009 ZXing authors
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
// package com::google::zxing::oned::rss;

/**
 * Encapsulates a since character value in an RSS barcode, including its checksum information.
 */
pub struct DataCharacter {

     let value: i32;

     let checksum_portion: i32;
}

impl DataCharacter {

    pub fn new( value: i32,  checksum_portion: i32) -> DataCharacter {
        let .value = value;
        let .checksumPortion = checksum_portion;
    }

    pub fn  get_value(&self) -> i32  {
        return self.value;
    }

    pub fn  get_checksum_portion(&self) -> i32  {
        return self.checksum_portion;
    }

    pub fn  to_string(&self) -> String  {
        return format!("{}({})", self.value, self.checksum_portion);
    }

    pub fn  equals(&self,  o: &Object) -> bool  {
        if !(o instanceof DataCharacter) {
            return false;
        }
         let that: DataCharacter = o as DataCharacter;
        return self.value == that.value && self.checksum_portion == that.checksumPortion;
    }

    pub fn  hash_code(&self) -> i32  {
        return self.value ^ self.checksum_portion;
    }
}

