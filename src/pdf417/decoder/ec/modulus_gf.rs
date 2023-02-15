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

use crate::Exceptions;

/**
 * <p>A field based on powers of a generator integer, modulo some modulus.</p>
 *
 * @author Sean Owen
 * @see com.google.zxing.common.reedsolomon.GenericGF
 */
#[derive(Debug, Clone)]
pub struct ModulusGF {
    expTable: Vec<u32>,
    logTable: Vec<u32>,
    // zero: Option<Rc<ModulusPoly<'a>>>,
    // one: Option<Rc<ModulusPoly<'a>>>,
    modulus: u32,
    generator: u32,
}
impl ModulusGF {
    pub fn new(modulus: u32, generator: u32) -> Self {
        let mut expTable = vec![0u32; modulus as usize]; //new int[modulus];
        let mut logTable = vec![0u32; modulus as usize]; //new int[modulus];
        let mut x = 1;
        for table_entry in expTable.iter_mut().take(modulus as usize) {
            // for i in 0..modulus as usize {
            // for (int i = 0; i < modulus; i++) {
            *table_entry = x;
            x = (x * generator) % modulus;
        }
        for i in 0..modulus as usize - 1 {
            // for (int i = 0; i < modulus - 1; i++) {
            logTable[expTable[i] as usize] = i as u32;
        }
        // logTable[0] == 0 but this should never be used

        // zero = new ModulusPoly(this, new int[]{0});
        // one = new ModulusPoly(this, new int[]{1});

        Self {
            expTable,
            logTable,
            // zero: None,
            // one: None,
            modulus,
            generator,
        }
    }

    pub fn add(&self, a: u32, b: u32) -> u32 {
        (a + b) % self.modulus
    }

    pub fn subtract(&self, a: u32, b: u32) -> u32 {
        (self.modulus + a - b) % self.modulus
    }

    pub fn exp(&self, a: u32) -> u32 {
        self.expTable[a as usize]
    }

    pub fn log(&self, a: u32) -> Result<u32, Exceptions> {
        if a == 0 {
            Err(Exceptions::arithmetic)
        } else {
            Ok(self.logTable[a as usize])
        }
    }

    pub fn inverse(&self, a: u32) -> Result<u32, Exceptions> {
        if a == 0 {
            Err(Exceptions::arithmetic)
        } else {
            Ok(self.expTable[self.modulus as usize - self.logTable[a as usize] as usize - 1])
        }
    }

    pub fn multiply(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            0
        } else {
            self.expTable[(self.logTable[a as usize] + self.logTable[b as usize]) as usize
                % (self.modulus - 1) as usize]
        }
    }

    pub fn getSize(&self) -> u32 {
        self.modulus
    }
}

impl PartialEq for ModulusGF {
    fn eq(&self, other: &Self) -> bool {
        self.modulus == other.modulus && self.generator == other.generator
    }
}
impl Eq for ModulusGF {}
