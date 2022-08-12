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

