/*
 * Copyright 2007 ZXing authors
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
// package com::google::zxing::common::reedsolomon;

/**
 * <p>This class contains utility methods for performing mathematical operations over
 * the Galois Fields. Operations use a given primitive polynomial in calculations.</p>
 *
 * <p>Throughout this package, elements of the GF are represented as an {@code int}
 * for convenience and speed (but at the cost of memory).
 * </p>
 *
 * @author Sean Owen
 * @author David Olivier
 */

// x^12 + x^6 + x^5 + x^3 + 1
 const AZTEC_DATA_12: GenericGF = GenericGF::new(0x1069, 4096, 1);

// x^10 + x^3 + 1
 const AZTEC_DATA_10: GenericGF = GenericGF::new(0x409, 1024, 1);

// x^6 + x + 1
 const AZTEC_DATA_6: GenericGF = GenericGF::new(0x43, 64, 1);

// x^4 + x + 1
 const AZTEC_PARAM: GenericGF = GenericGF::new(0x13, 16, 1);

// x^8 + x^4 + x^3 + x^2 + 1
 const QR_CODE_FIELD_256: GenericGF = GenericGF::new(0x011D, 256, 0);

// x^8 + x^5 + x^3 + x^2 + 1
 const DATA_MATRIX_FIELD_256: GenericGF = GenericGF::new(0x012D, 256, 1);

 const AZTEC_DATA_8: GenericGF = DATA_MATRIX_FIELD_256;

 const MAXICODE_FIELD_64: GenericGF = AZTEC_DATA_6;
pub struct GenericGF {

     let exp_table: Vec<i32>;

     let log_table: Vec<i32>;

     let mut zero: GenericGFPoly;

     let mut one: GenericGFPoly;

     let size: i32;

     let primitive: i32;

     let generator_base: i32;
}

impl GenericGF {

    /**
   * Create a representation of GF(size) using the given primitive polynomial.
   *
   * @param primitive irreducible polynomial whose coefficients are represented by
   *  the bits of an int, where the least-significant bit represents the constant
   *  coefficient
   * @param size the size of the field
   * @param b the factor b in the generator polynomial can be 0- or 1-based
   *  (g(x) = (x+a^b)(x+a^(b+1))...(x+a^(b+2t-1))).
   *  In most cases it should be 1, but for QR code it is 0.
   */
    pub fn new( primitive: i32,  size: i32,  b: i32) -> GenericGF {
        let .primitive = primitive;
        let .size = size;
        let .generatorBase = b;
        exp_table = : [i32; size] = [0; size];
        log_table = : [i32; size] = [0; size];
         let mut x: i32 = 1;
         {
             let mut i: i32 = 0;
            while i < size {
                {
                    exp_table[i] = x;
                    // we're assuming the generator alpha is 2
                    x *= 2;
                    if x >= size {
                        x ^= primitive;
                        x &= size - 1;
                    }
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 0;
            while i < size - 1 {
                {
                    log_table[exp_table[i]] = i;
                }
                i += 1;
             }
         }

        // logTable[0] == 0 but this should never be used
        zero = GenericGFPoly::new(let ,  : vec![i32; 1] = vec![0, ]
        );
        one = GenericGFPoly::new(let ,  : vec![i32; 1] = vec![1, ]
        );
    }

    fn  get_zero(&self) -> GenericGFPoly  {
        return self.zero;
    }

    fn  get_one(&self) -> GenericGFPoly  {
        return self.one;
    }

    /**
   * @return the monomial representing coefficient * x^degree
   */
    fn  build_monomial(&self,  degree: i32,  coefficient: i32) -> GenericGFPoly  {
        if degree < 0 {
            throw IllegalArgumentException::new();
        }
        if coefficient == 0 {
            return self.zero;
        }
         let mut coefficients: [i32; degree + 1] = [0; degree + 1];
        coefficients[0] = coefficient;
        return GenericGFPoly::new(self, &coefficients);
    }

    /**
   * Implements both addition and subtraction -- they are the same in GF(size).
   *
   * @return sum/difference of a and b
   */
    fn  add_or_subtract( a: i32,  b: i32) -> i32  {
        return a ^ b;
    }

    /**
   * @return 2 to the power of a in GF(size)
   */
    fn  exp(&self,  a: i32) -> i32  {
        return self.exp_table[a];
    }

    /**
   * @return base 2 log of a in GF(size)
   */
    fn  log(&self,  a: i32) -> i32  {
        if a == 0 {
            throw IllegalArgumentException::new();
        }
        return self.log_table[a];
    }

    /**
   * @return multiplicative inverse of a
   */
    fn  inverse(&self,  a: i32) -> i32  {
        if a == 0 {
            throw ArithmeticException::new();
        }
        return self.exp_table[self.size - self.log_table[a] - 1];
    }

    /**
   * @return product of a and b in GF(size)
   */
    fn  multiply(&self,  a: i32,  b: i32) -> i32  {
        if a == 0 || b == 0 {
            return 0;
        }
        return self.exp_table[(self.log_table[a] + self.log_table[b]) % (self.size - 1)];
    }

    pub fn  get_size(&self) -> i32  {
        return self.size;
    }

    pub fn  get_generator_base(&self) -> i32  {
        return self.generator_base;
    }

    pub fn  to_string(&self) -> String  {
        return format!("GF(0x{},{})", Integer::to_hex_string(self.primitive), self.size);
    }
}

