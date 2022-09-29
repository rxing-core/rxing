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

/**
 * Encapsulates the result of one test over a batch of black-box images.
 */
pub struct TestRXingResult {
    must_pass_count: u32,
    try_harder_count: u32,
    max_misreads: u32,
    max_try_harder_misreads: u32,
    rotation: f32,
}

impl TestRXingResult {
    pub fn new(
        must_pass_count: u32,
        try_harder_count: u32,
        max_misreads: u32,
        max_try_harder_misreads: u32,
        rotation: f32,
    ) -> Self {
        Self {
            must_pass_count,
            try_harder_count,
            max_misreads,
            max_try_harder_misreads,
            rotation,
        }
    }

    pub fn getMustPassCount(&self) -> u32 {
        self.must_pass_count
    }

    pub fn getTryHarderCount(&self) -> u32 {
        self.try_harder_count
    }

    pub fn getMaxMisreads(&self) -> u32 {
        self.max_misreads
    }

    pub fn getMaxTryHarderMisreads(&self) -> u32 {
        self.max_try_harder_misreads
    }

    pub fn getRotation(&self) -> f32 {
        self.rotation
    }
}
