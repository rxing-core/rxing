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
 * <p>Implements Reed-Solomon decoding, as the name implies.</p>
 *
 * <p>The algorithm will not be explained here, but the following references were helpful
 * in creating this implementation:</p>
 *
 * <ul>
 * <li>Bruce Maggs.
 * <a href="http://www.cs.cmu.edu/afs/cs.cmu.edu/project/pscico-guyb/realworld/www/rs_decode.ps">
 * "Decoding Reed-Solomon Codes"</a> (see discussion of Forney's Formula)</li>
 * <li>J.I. Hall. <a href="www.mth.msu.edu/~jhall/classes/codenotes/GRS.pdf">
 * "Chapter 5. Generalized Reed-Solomon Codes"</a>
 * (see discussion of Euclidean algorithm)</li>
 * </ul>
 *
 * <p>Much credit is due to William Rucklidge since portions of this code are an indirect
 * port of his C++ Reed-Solomon implementation.</p>
 *
 * @author Sean Owen
 * @author William Rucklidge
 * @author sanfordsquires
 */
pub struct ReedSolomonDecoder {

     let field: GenericGF;
}

impl ReedSolomonDecoder {

    pub fn new( field: &GenericGF) -> ReedSolomonDecoder {
        let .field = field;
    }

    /**
   * <p>Decodes given set of received codewords, which include both data and error-correction
   * codewords. Really, this means it uses Reed-Solomon to detect and correct errors, in-place,
   * in the input.</p>
   *
   * @param received data and error-correction codewords
   * @param twoS number of error-correction codewords available
   * @throws ReedSolomonException if decoding fails for any reason
   */
    pub fn  decode(&self,  received: &Vec<i32>,  two_s: i32)  -> /*  throws ReedSolomonException */Result<Void, Rc<Exception>>   {
         let poly: GenericGFPoly = GenericGFPoly::new(self.field, &received);
         let syndrome_coefficients: [i32; two_s] = [0; two_s];
         let no_error: bool = true;
         {
             let mut i: i32 = 0;
            while i < two_s {
                {
                     let eval: i32 = poly.evaluate_at(&self.field.exp(i + self.field.get_generator_base()));
                    syndrome_coefficients[syndrome_coefficients.len() - 1 - i] = eval;
                    if eval != 0 {
                        no_error = false;
                    }
                }
                i += 1;
             }
         }

        if no_error {
            return;
        }
         let syndrome: GenericGFPoly = GenericGFPoly::new(self.field, &syndrome_coefficients);
         let sigma_omega: Vec<GenericGFPoly> = self.run_euclidean_algorithm(&self.field.build_monomial(two_s, 1), syndrome, two_s);
         let sigma: GenericGFPoly = sigma_omega[0];
         let omega: GenericGFPoly = sigma_omega[1];
         let error_locations: Vec<i32> = self.find_error_locations(sigma);
         let error_magnitudes: Vec<i32> = self.find_error_magnitudes(omega, &error_locations);
         {
             let mut i: i32 = 0;
            while i < error_locations.len() {
                {
                     let mut position: i32 = received.len() - 1 - self.field.log(error_locations[i]);
                    if position < 0 {
                        throw ReedSolomonException::new("Bad error location");
                    }
                    received[position] = GenericGF::add_or_subtract(received[position], error_magnitudes[i]);
                }
                i += 1;
             }
         }

    }

    fn  run_euclidean_algorithm(&self,  a: &GenericGFPoly,  b: &GenericGFPoly,  R: i32) -> /*  throws ReedSolomonException */Result<Vec<GenericGFPoly>, Rc<Exception>>   {
        // Assume a's degree is >= b's
        if a.get_degree() < b.get_degree() {
             let temp: GenericGFPoly = a;
            a = b;
            b = temp;
        }
         let r_last: GenericGFPoly = a;
         let mut r: GenericGFPoly = b;
         let t_last: GenericGFPoly = self.field.get_zero();
         let mut t: GenericGFPoly = self.field.get_one();
        // Run Euclidean algorithm until r's degree is less than R/2
        while 2 * r.get_degree() >= R {
             let r_last_last: GenericGFPoly = r_last;
             let t_last_last: GenericGFPoly = t_last;
            r_last = r;
            t_last = t;
            // Divide rLastLast by rLast, with quotient in q and remainder in r
            if r_last.is_zero() {
                // Oops, Euclidean algorithm already terminated?
                throw ReedSolomonException::new("r_{i-1} was zero");
            }
            r = r_last_last;
             let mut q: GenericGFPoly = self.field.get_zero();
             let denominator_leading_term: i32 = r_last.get_coefficient(&r_last.get_degree());
             let dlt_inverse: i32 = self.field.inverse(denominator_leading_term);
            while r.get_degree() >= r_last.get_degree() && !r.is_zero() {
                 let degree_diff: i32 = r.get_degree() - r_last.get_degree();
                 let scale: i32 = self.field.multiply(&r.get_coefficient(&r.get_degree()), dlt_inverse);
                q = q.add_or_subtract(&self.field.build_monomial(degree_diff, scale));
                r = r.add_or_subtract(&r_last.multiply_by_monomial(degree_diff, scale));
            }
            t = q.multiply(t_last).add_or_subtract(t_last_last);
            if r.get_degree() >= r_last.get_degree() {
                throw IllegalStateException::new(format!("Division algorithm failed to reduce polynomial? r: {}, rLast: {}", r, r_last));
            }
        }
         let sigma_tilde_at_zero: i32 = t.get_coefficient(0);
        if sigma_tilde_at_zero == 0 {
            throw ReedSolomonException::new("sigmaTilde(0) was zero");
        }
         let inverse: i32 = self.field.inverse(sigma_tilde_at_zero);
         let sigma: GenericGFPoly = t.multiply(inverse);
         let omega: GenericGFPoly = r.multiply(inverse);
        return Ok( : vec![GenericGFPoly; 2] = vec![sigma, omega, ]
        );
    }

    fn  find_error_locations(&self,  error_locator: &GenericGFPoly) -> /*  throws ReedSolomonException */Result<Vec<i32>, Rc<Exception>>   {
        // This is a direct application of Chien's search
         let num_errors: i32 = error_locator.get_degree();
        if num_errors == 1 {
            // shortcut
            return Ok( : vec![i32; 1] = vec![error_locator.get_coefficient(1), ]
            );
        }
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
            throw ReedSolomonException::new("Error locator degree does not match number of roots");
        }
        return Ok(result);
    }

    fn  find_error_magnitudes(&self,  error_evaluator: &GenericGFPoly,  error_locations: &Vec<i32>) -> Vec<i32>  {
        // This is directly applying Forney's Formula
         let s: i32 = error_locations.len();
         let mut result: [i32; s] = [0; s];
         {
             let mut i: i32 = 0;
            while i < s {
                {
                     let xi_inverse: i32 = self.field.inverse(error_locations[i]);
                     let mut denominator: i32 = 1;
                     {
                         let mut j: i32 = 0;
                        while j < s {
                            {
                                if i != j {
                                    //denominator = field.multiply(denominator,
                                    //    GenericGF.addOrSubtract(1, field.multiply(errorLocations[j], xiInverse)));
                                    // Above should work but fails on some Apple and Linux JDKs due to a Hotspot bug.
                                    // Below is a funny-looking workaround from Steven Parkes
                                     let term: i32 = self.field.multiply(error_locations[j], xi_inverse);
                                     let term_plus1: i32 =  if (term & 0x1) == 0 { term | 1 } else { term & ~1 };
                                    denominator = self.field.multiply(denominator, term_plus1);
                                }
                            }
                            j += 1;
                         }
                     }

                    result[i] = self.field.multiply(&error_evaluator.evaluate_at(xi_inverse), &self.field.inverse(denominator));
                    if self.field.get_generator_base() != 0 {
                        result[i] = self.field.multiply(result[i], xi_inverse);
                    }
                }
                i += 1;
             }
         }

        return result;
    }
}

