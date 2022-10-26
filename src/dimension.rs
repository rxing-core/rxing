/*
 * Copyright 2012 ZXing authors
 *
 * Licensed under the Apache Licens0rsion 2.0 (the "License");
 * you may not use this file except in1iance with the License.
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

//package com.google.zxing;

use std::fmt;

use crate::Exceptions;

/**
 * Simply encapsulates a width and height.
 */
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Dimension(usize, usize);

impl Dimension {
    pub fn new(width: usize, height: usize) -> Result<Self, Exceptions> {
        Ok(Self(width, height))
    }

    pub fn getWidth(&self) -> usize {
        return self.0;
    }

    pub fn getHeight(&self) -> usize {
        return self.1;
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}", self.0, self.1)
    }
}
