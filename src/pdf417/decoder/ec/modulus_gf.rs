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

//public static final ModulusGF PDF417_GF = new ModulusGF(PDF417Common.NUMBER_OF_CODEWORDS, 3);

use crate::Error;
use crate::common::Result;

/**
 * <p>A field based on powers of a generator integer, modulo some modulus.</p>
 *
 * @author Sean Owen
 * @see com.google.zxing.common.reedsolomon.GenericGF
 */
#[derive(Debug, Clone)]
pub struct ModulusGF<const MODULUS: usize> {
    expTable: [u32; MODULUS],
    logTable: [u32; MODULUS],
    // zero: Option<Arc<ModulusPoly<'a>>>,
    // one: Option<Arc<ModulusPoly<'a>>>,
    modulus: u32,
    generator: u32,
}
impl<const MODULUS: usize> ModulusGF<MODULUS> {
    pub const fn new(generator: u32) -> Self {
        let mut expTable = [0u32; MODULUS];
        let mut logTable = [0u32; MODULUS];
        let mut x = 1;

        let mut i = 0;
        while i < MODULUS {
            expTable[i] = x;
            x = (x * generator) % MODULUS as u32;
            i += 1;
        }
        let mut i = 0;
        while i < MODULUS - 1 {
            logTable[expTable[i] as usize] = i as u32;

            i += 1;
        }

        Self {
            expTable,
            logTable,
            modulus: MODULUS as u32,
            generator,
        }
    }

    pub const fn add(&self, a: u32, b: u32) -> u32 {
        (a + b) % self.modulus
    }

    pub const fn subtract(&self, a: u32, b: u32) -> u32 {
        (self.modulus + a - b) % self.modulus
    }

    pub const fn exp(&self, a: u32) -> u32 {
        self.expTable[a as usize]
    }

    pub const fn log(&self, a: u32) -> Result<u32> {
        if a == 0 {
            Err(Error::CHECKSUM)
        } else {
            Ok(self.logTable[a as usize])
        }
    }

    pub const fn inverse(&self, a: u32) -> Result<u32> {
        if a == 0 {
            Err(Error::CHECKSUM)
        } else {
            Ok(self.expTable[self.modulus as usize - self.logTable[a as usize] as usize - 1])
        }
    }

    pub const fn multiply(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            0
        } else {
            self.expTable[(self.logTable[a as usize] + self.logTable[b as usize]) as usize
                % (self.modulus - 1) as usize]
        }
    }

    pub const fn getSize(&self) -> u32 {
        self.modulus
    }
}

impl<const MODULUS: usize> PartialEq for ModulusGF<MODULUS> {
    fn eq(&self, other: &Self) -> bool {
        self.modulus == other.modulus && self.generator == other.generator
    }
}
impl<const MODULUS: usize> Eq for ModulusGF<MODULUS> {}
