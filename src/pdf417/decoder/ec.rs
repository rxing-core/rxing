use crate::ChecksumException;
use crate::pdf417::PDF417Common;



// NEW FILE: error_correction.rs
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
 * <p>PDF417 error correction implementation.</p>
 *
 * <p>This <a href="http://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction#Example">example</a>
 * is quite useful in understanding the algorithm.</p>
 *
 * @author Sean Owen
 * @see com.google.zxing.common.reedsolomon.ReedSolomonDecoder
 */
pub struct ErrorCorrection {

     let mut field: ModulusGF;
}

impl ErrorCorrection {

    pub fn new() -> ErrorCorrection {
        let .field = ModulusGF::PDF417_GF;
    }

    /**
   * @param received received codewords
   * @param numECCodewords number of those codewords used for EC
   * @param erasures location of erasures
   * @return number of errors
   * @throws ChecksumException if errors cannot be corrected, maybe because of too many errors
   */
    pub fn  decode(&self,  received: &Vec<i32>,  num_e_c_codewords: i32,  erasures: &Vec<i32>) -> /*  throws ChecksumException */Result<i32, Rc<Exception>>   {
         let poly: ModulusPoly = ModulusPoly::new(self.field, &received);
         const S: [i32; num_e_c_codewords] = [0; num_e_c_codewords];
         let mut error: bool = false;
         {
             let mut i: i32 = num_e_c_codewords;
            while i > 0 {
                {
                     let eval: i32 = poly.evaluate_at(&self.field.exp(i));
                    S[num_e_c_codewords - i] = eval;
                    if eval != 0 {
                        error = true;
                    }
                }
                i -= 1;
             }
         }

        if !error {
            return Ok(0);
        }
         let known_errors: ModulusPoly = self.field.get_one();
        if erasures != null {
            for  let erasure: i32 in erasures {
                 let b: i32 = self.field.exp(received.len() - 1 - erasure);
                // Add (1 - bx) term:
                 let term: ModulusPoly = ModulusPoly::new(self.field,  : vec![i32; 2] = vec![self.field.subtract(0, b), 1, ]
                );
                known_errors = known_errors.multiply(term);
            }
        }
         let syndrome: ModulusPoly = ModulusPoly::new(self.field, &S);
        //syndrome = syndrome.multiply(knownErrors);
         let sigma_omega: Vec<ModulusPoly> = self.run_euclidean_algorithm(&self.field.build_monomial(num_e_c_codewords, 1), syndrome, num_e_c_codewords);
         let sigma: ModulusPoly = sigma_omega[0];
         let omega: ModulusPoly = sigma_omega[1];
        //sigma = sigma.multiply(knownErrors);
         let error_locations: Vec<i32> = self.find_error_locations(sigma);
         let error_magnitudes: Vec<i32> = self.find_error_magnitudes(omega, sigma, &error_locations);
         {
             let mut i: i32 = 0;
            while i < error_locations.len() {
                {
                     let mut position: i32 = received.len() - 1 - self.field.log(error_locations[i]);
                    if position < 0 {
                        throw ChecksumException::get_checksum_instance();
                    }
                    received[position] = self.field.subtract(received[position], error_magnitudes[i]);
                }
                i += 1;
             }
         }

        return Ok(error_locations.len());
    }

    fn  run_euclidean_algorithm(&self,  a: &ModulusPoly,  b: &ModulusPoly,  R: i32) -> /*  throws ChecksumException */Result<Vec<ModulusPoly>, Rc<Exception>>   {
        // Assume a's degree is >= b's
        if a.get_degree() < b.get_degree() {
             let temp: ModulusPoly = a;
            a = b;
            b = temp;
        }
         let r_last: ModulusPoly = a;
         let mut r: ModulusPoly = b;
         let t_last: ModulusPoly = self.field.get_zero();
         let mut t: ModulusPoly = self.field.get_one();
        // Run Euclidean algorithm until r's degree is less than R/2
        while r.get_degree() >= R / 2 {
             let r_last_last: ModulusPoly = r_last;
             let t_last_last: ModulusPoly = t_last;
            r_last = r;
            t_last = t;
            // Divide rLastLast by rLast, with quotient in q and remainder in r
            if r_last.is_zero() {
                // Oops, Euclidean algorithm already terminated?
                throw ChecksumException::get_checksum_instance();
            }
            r = r_last_last;
             let mut q: ModulusPoly = self.field.get_zero();
             let denominator_leading_term: i32 = r_last.get_coefficient(&r_last.get_degree());
             let dlt_inverse: i32 = self.field.inverse(denominator_leading_term);
            while r.get_degree() >= r_last.get_degree() && !r.is_zero() {
                 let degree_diff: i32 = r.get_degree() - r_last.get_degree();
                 let scale: i32 = self.field.multiply(&r.get_coefficient(&r.get_degree()), dlt_inverse);
                q = q.add(&self.field.build_monomial(degree_diff, scale));
                r = r.subtract(&r_last.multiply_by_monomial(degree_diff, scale));
            }
            t = q.multiply(t_last).subtract(t_last_last).negative();
        }
         let sigma_tilde_at_zero: i32 = t.get_coefficient(0);
        if sigma_tilde_at_zero == 0 {
            throw ChecksumException::get_checksum_instance();
        }
         let inverse: i32 = self.field.inverse(sigma_tilde_at_zero);
         let sigma: ModulusPoly = t.multiply(inverse);
         let omega: ModulusPoly = r.multiply(inverse);
        return Ok( : vec![ModulusPoly; 2] = vec![sigma, omega, ]
        );
    }

    fn  find_error_locations(&self,  error_locator: &ModulusPoly) -> /*  throws ChecksumException */Result<Vec<i32>, Rc<Exception>>   {
        // This is a direct application of Chien's search
         let num_errors: i32 = error_locator.get_degree();
         let mut result: [i32; num_errors] = [0; num_errors];
         let mut e: i32 = 0;
         {
             let mut i: i32 = 1;
            while i < self.field.get_size() && e < num_errors {
                {
                    if error_locator.evaluate_at(i) == 0 {
                        result[e] = self.field.inverse(i);
                        e += 1;
                    }
                }
                i += 1;
             }
         }

        if e != num_errors {
            throw ChecksumException::get_checksum_instance();
        }
        return Ok(result);
    }

    fn  find_error_magnitudes(&self,  error_evaluator: &ModulusPoly,  error_locator: &ModulusPoly,  error_locations: &Vec<i32>) -> Vec<i32>  {
         let error_locator_degree: i32 = error_locator.get_degree();
        if error_locator_degree < 1 {
            return : [i32; 0] = [0; 0];
        }
         let formal_derivative_coefficients: [i32; error_locator_degree] = [0; error_locator_degree];
         {
             let mut i: i32 = 1;
            while i <= error_locator_degree {
                {
                    formal_derivative_coefficients[error_locator_degree - i] = self.field.multiply(i, &error_locator.get_coefficient(i));
                }
                i += 1;
             }
         }

         let formal_derivative: ModulusPoly = ModulusPoly::new(self.field, &formal_derivative_coefficients);
        // This is directly applying Forney's Formula
         let s: i32 = error_locations.len();
         let mut result: [i32; s] = [0; s];
         {
             let mut i: i32 = 0;
            while i < s {
                {
                     let xi_inverse: i32 = self.field.inverse(error_locations[i]);
                     let numerator: i32 = self.field.subtract(0, &error_evaluator.evaluate_at(xi_inverse));
                     let denominator: i32 = self.field.inverse(&formal_derivative.evaluate_at(xi_inverse));
                    result[i] = self.field.multiply(numerator, denominator);
                }
                i += 1;
             }
         }

        return result;
    }
}

// NEW FILE: modulus_g_f.rs
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

// NEW FILE: modulus_poly.rs
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
 * @author Sean Owen
 */
struct ModulusPoly {

     let field: ModulusGF;

     let coefficients: Vec<i32>;
}

impl ModulusPoly {

    fn new( field: &ModulusGF,  coefficients: &Vec<i32>) -> ModulusPoly {
        if coefficients.len() == 0 {
            throw IllegalArgumentException::new();
        }
        let .field = field;
         let coefficients_length: i32 = coefficients.len();
        if coefficients_length > 1 && coefficients[0] == 0 {
            // Leading term must be non-zero for anything except the constant polynomial "0"
             let first_non_zero: i32 = 1;
            while first_non_zero < coefficients_length && coefficients[first_non_zero] == 0 {
                first_non_zero += 1;
            }
            if first_non_zero == coefficients_length {
                let .coefficients =  : vec![i32; 1] = vec![0, ]
                ;
            } else {
                let .coefficients = : [i32; coefficients_length - first_non_zero] = [0; coefficients_length - first_non_zero];
                System::arraycopy(&coefficients, first_non_zero, let .coefficients, 0, let .coefficients.len());
            }
        } else {
            let .coefficients = coefficients;
        }
    }

    fn  get_coefficients(&self) -> Vec<i32>  {
        return self.coefficients;
    }

    /**
   * @return degree of this polynomial
   */
    fn  get_degree(&self) -> i32  {
        return self.coefficients.len() - 1;
    }

    /**
   * @return true iff this polynomial is the monomial "0"
   */
    fn  is_zero(&self) -> bool  {
        return self.coefficients[0] == 0;
    }

    /**
   * @return coefficient of x^degree term in this polynomial
   */
    fn  get_coefficient(&self,  degree: i32) -> i32  {
        return self.coefficients[self.coefficients.len() - 1 - degree];
    }

    /**
   * @return evaluation of this polynomial at a given point
   */
    fn  evaluate_at(&self,  a: i32) -> i32  {
        if a == 0 {
            // Just return the x^0 coefficient
            return self.get_coefficient(0);
        }
        if a == 1 {
            // Just the sum of the coefficients
             let mut result: i32 = 0;
            for  let coefficient: i32 in self.coefficients {
                result = self.field.add(result, coefficient);
            }
            return result;
        }
         let mut result: i32 = self.coefficients[0];
         let size: i32 = self.coefficients.len();
         {
             let mut i: i32 = 1;
            while i < size {
                {
                    result = self.field.add(&self.field.multiply(a, result), self.coefficients[i]);
                }
                i += 1;
             }
         }

        return result;
    }

    fn  add(&self,  other: &ModulusPoly) -> ModulusPoly  {
        if !self.field.equals(other.field) {
            throw IllegalArgumentException::new("ModulusPolys do not have same ModulusGF field");
        }
        if self.is_zero() {
            return other;
        }
        if other.is_zero() {
            return self;
        }
         let smaller_coefficients: Vec<i32> = self.coefficients;
         let larger_coefficients: Vec<i32> = other.coefficients;
        if smaller_coefficients.len() > larger_coefficients.len() {
             let temp: Vec<i32> = smaller_coefficients;
            smaller_coefficients = larger_coefficients;
            larger_coefficients = temp;
        }
         let sum_diff: [i32; larger_coefficients.len()] = [0; larger_coefficients.len()];
         let length_diff: i32 = larger_coefficients.len() - smaller_coefficients.len();
        // Copy high-order terms only found in higher-degree polynomial's coefficients
        System::arraycopy(&larger_coefficients, 0, &sum_diff, 0, length_diff);
         {
             let mut i: i32 = length_diff;
            while i < larger_coefficients.len() {
                {
                    sum_diff[i] = self.field.add(smaller_coefficients[i - length_diff], larger_coefficients[i]);
                }
                i += 1;
             }
         }

        return ModulusPoly::new(self.field, &sum_diff);
    }

    fn  subtract(&self,  other: &ModulusPoly) -> ModulusPoly  {
        if !self.field.equals(other.field) {
            throw IllegalArgumentException::new("ModulusPolys do not have same ModulusGF field");
        }
        if other.is_zero() {
            return self;
        }
        return self.add(&other.negative());
    }

    fn  multiply(&self,  other: &ModulusPoly) -> ModulusPoly  {
        if !self.field.equals(other.field) {
            throw IllegalArgumentException::new("ModulusPolys do not have same ModulusGF field");
        }
        if self.is_zero() || other.is_zero() {
            return self.field.get_zero();
        }
         let a_coefficients: Vec<i32> = self.coefficients;
         let a_length: i32 = a_coefficients.len();
         let b_coefficients: Vec<i32> = other.coefficients;
         let b_length: i32 = b_coefficients.len();
         let mut product: [i32; a_length + b_length - 1] = [0; a_length + b_length - 1];
         {
             let mut i: i32 = 0;
            while i < a_length {
                {
                     let a_coeff: i32 = a_coefficients[i];
                     {
                         let mut j: i32 = 0;
                        while j < b_length {
                            {
                                product[i + j] = self.field.add(product[i + j], &self.field.multiply(a_coeff, b_coefficients[j]));
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        return ModulusPoly::new(self.field, &product);
    }

    fn  negative(&self) -> ModulusPoly  {
         let size: i32 = self.coefficients.len();
         let negative_coefficients: [i32; size] = [0; size];
         {
             let mut i: i32 = 0;
            while i < size {
                {
                    negative_coefficients[i] = self.field.subtract(0, self.coefficients[i]);
                }
                i += 1;
             }
         }

        return ModulusPoly::new(self.field, &negative_coefficients);
    }

    fn  multiply(&self,  scalar: i32) -> ModulusPoly  {
        if scalar == 0 {
            return self.field.get_zero();
        }
        if scalar == 1 {
            return self;
        }
         let size: i32 = self.coefficients.len();
         let mut product: [i32; size] = [0; size];
         {
             let mut i: i32 = 0;
            while i < size {
                {
                    product[i] = self.field.multiply(self.coefficients[i], scalar);
                }
                i += 1;
             }
         }

        return ModulusPoly::new(self.field, &product);
    }

    fn  multiply_by_monomial(&self,  degree: i32,  coefficient: i32) -> ModulusPoly  {
        if degree < 0 {
            throw IllegalArgumentException::new();
        }
        if coefficient == 0 {
            return self.field.get_zero();
        }
         let size: i32 = self.coefficients.len();
         let mut product: [i32; size + degree] = [0; size + degree];
         {
             let mut i: i32 = 0;
            while i < size {
                {
                    product[i] = self.field.multiply(self.coefficients[i], coefficient);
                }
                i += 1;
             }
         }

        return ModulusPoly::new(self.field, &product);
    }

    pub fn  to_string(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(8 * self.get_degree());
         {
             let mut degree: i32 = self.get_degree();
            while degree >= 0 {
                {
                     let mut coefficient: i32 = self.get_coefficient(degree);
                    if coefficient != 0 {
                        if coefficient < 0 {
                            result.append(" - ");
                            coefficient = -coefficient;
                        } else {
                            if result.length() > 0 {
                                result.append(" + ");
                            }
                        }
                        if degree == 0 || coefficient != 1 {
                            result.append(coefficient);
                        }
                        if degree != 0 {
                            if degree == 1 {
                                result.append('x');
                            } else {
                                result.append("x^");
                                result.append(degree);
                            }
                        }
                    }
                }
                degree -= 1;
             }
         }

        return result.to_string();
    }
}

