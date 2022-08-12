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
// package com::google::zxing::pdf417::decoder::ec;

/**
 * <p>A field based on powers of a generator integer, modulo some modulus.</p>
 *
 * @author Sean Owen
 * @see com.google.zxing.common.reedsolomon.GenericGF
 */

 const PDF417_GF: ModulusGF = ModulusGF::new(PDF417Common.NUMBER_OF_CODEWORDS, 3);
pub struct ModulusGF {

     let exp_table: Vec<i32>;

     let log_table: Vec<i32>;

     let mut zero: ModulusPoly;

     let mut one: ModulusPoly;

     let modulus: i32;
}

impl ModulusGF {

    fn new( modulus: i32,  generator: i32) -> ModulusGF {
        let .modulus = modulus;
        exp_table = : [i32; modulus] = [0; modulus];
        log_table = : [i32; modulus] = [0; modulus];
         let mut x: i32 = 1;
         {
             let mut i: i32 = 0;
            while i < modulus {
                {
                    exp_table[i] = x;
                    x = (x * generator) % modulus;
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 0;
            while i < modulus - 1 {
                {
                    log_table[exp_table[i]] = i;
                }
                i += 1;
             }
         }

        // logTable[0] == 0 but this should never be used
        zero = ModulusPoly::new(let ,  : vec![i32; 1] = vec![0, ]
        );
        one = ModulusPoly::new(let ,  : vec![i32; 1] = vec![1, ]
        );
    }

    fn  get_zero(&self) -> ModulusPoly  {
        return self.zero;
    }

    fn  get_one(&self) -> ModulusPoly  {
        return self.one;
    }

    fn  build_monomial(&self,  degree: i32,  coefficient: i32) -> ModulusPoly  {
        if degree < 0 {
            throw IllegalArgumentException::new();
        }
        if coefficient == 0 {
            return self.zero;
        }
         let mut coefficients: [i32; degree + 1] = [0; degree + 1];
        coefficients[0] = coefficient;
        return ModulusPoly::new(self, &coefficients);
    }

    fn  add(&self,  a: i32,  b: i32) -> i32  {
        return (a + b) % self.modulus;
    }

    fn  subtract(&self,  a: i32,  b: i32) -> i32  {
        return (self.modulus + a - b) % self.modulus;
    }

    fn  exp(&self,  a: i32) -> i32  {
        return self.exp_table[a];
    }

    fn  log(&self,  a: i32) -> i32  {
        if a == 0 {
            throw IllegalArgumentException::new();
        }
        return self.log_table[a];
    }

    fn  inverse(&self,  a: i32) -> i32  {
        if a == 0 {
            throw ArithmeticException::new();
        }
        return self.exp_table[self.modulus - self.log_table[a] - 1];
    }

    fn  multiply(&self,  a: i32,  b: i32) -> i32  {
        if a == 0 || b == 0 {
            return 0;
        }
        return self.exp_table[(self.log_table[a] + self.log_table[b]) % (self.modulus - 1)];
    }

    fn  get_size(&self) -> i32  {
        return self.modulus;
    }
}

