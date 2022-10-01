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

pub struct BlockPair {
    data_bytes: Vec<u8>,
    error_correction_bytes: Vec<u8>,
}

impl BlockPair {
    pub fn new(data: Vec<u8>, error_correction: Vec<u8>) -> Self {
        Self {
            data_bytes: data,
            error_correction_bytes: error_correction,
        }
    }

    pub fn getDataBytes(&self) -> &[u8] {
        &self.data_bytes
    }

    pub fn getErrorCorrectionBytes(&self) -> &[u8] {
        &self.error_correction_bytes
    }
}
