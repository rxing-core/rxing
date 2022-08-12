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
 * <p>Represents a polynomial whose coefficients are elements of a GF.
 * Instances of this class are immutable.</p>
 *
 * <p>Much credit is due to William Rucklidge since portions of this code are an indirect
 * port of his C++ Reed-Solomon implementation.</p>
 *
 * @author Sean Owen
 */
struct GenericGFPoly {

     let field: GenericGF;

     let coefficients: Vec<i32>;
}

impl GenericGFPoly {

    /**
   * @param field the {@link GenericGF} instance representing the field to use
   * to perform computations
   * @param coefficients coefficients as ints representing elements of GF(size), arranged
   * from most significant (highest-power term) coefficient to least significant
   * @throws IllegalArgumentException if argument is null or empty,
   * or if leading coefficient is 0 and this is not a
   * constant polynomial (that is, it is not the monomial "0")
   */
    fn new( field: &GenericGF,  coefficients: &Vec<i32>) -> GenericGFPoly {
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
                result = GenericGF::add_or_subtract(result, coefficient);
            }
            return result;
        }
         let mut result: i32 = self.coefficients[0];
         let size: i32 = self.coefficients.len();
         {
             let mut i: i32 = 1;
            while i < size {
                {
                    result = GenericGF::add_or_subtract(&self.field.multiply(a, result), self.coefficients[i]);
                }
                i += 1;
             }
         }

        return result;
    }

    fn  add_or_subtract(&self,  other: &GenericGFPoly) -> GenericGFPoly  {
        if !self.field.equals(other.field) {
            throw IllegalArgumentException::new("GenericGFPolys do not have same GenericGF field");
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
                    sum_diff[i] = GenericGF::add_or_subtract(smaller_coefficients[i - length_diff], larger_coefficients[i]);
                }
                i += 1;
             }
         }

        return GenericGFPoly::new(self.field, &sum_diff);
    }

    fn  multiply(&self,  other: &GenericGFPoly) -> GenericGFPoly  {
        if !self.field.equals(other.field) {
            throw IllegalArgumentException::new("GenericGFPolys do not have same GenericGF field");
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
                                product[i + j] = GenericGF::add_or_subtract(product[i + j], &self.field.multiply(a_coeff, b_coefficients[j]));
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

        return GenericGFPoly::new(self.field, &product);
    }

    fn  multiply(&self,  scalar: i32) -> GenericGFPoly  {
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

        return GenericGFPoly::new(self.field, &product);
    }

    fn  multiply_by_monomial(&self,  degree: i32,  coefficient: i32) -> GenericGFPoly  {
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

        return GenericGFPoly::new(self.field, &product);
    }

    fn  divide(&self,  other: &GenericGFPoly) -> Vec<GenericGFPoly>  {
        if !self.field.equals(other.field) {
            throw IllegalArgumentException::new("GenericGFPolys do not have same GenericGF field");
        }
        if other.is_zero() {
            throw IllegalArgumentException::new("Divide by 0");
        }
         let mut quotient: GenericGFPoly = self.field.get_zero();
         let mut remainder: GenericGFPoly = self;
         let denominator_leading_term: i32 = other.get_coefficient(&other.get_degree());
         let inverse_denominator_leading_term: i32 = self.field.inverse(denominator_leading_term);
        while remainder.get_degree() >= other.get_degree() && !remainder.is_zero() {
             let degree_difference: i32 = remainder.get_degree() - other.get_degree();
             let scale: i32 = self.field.multiply(&remainder.get_coefficient(&remainder.get_degree()), inverse_denominator_leading_term);
             let term: GenericGFPoly = other.multiply_by_monomial(degree_difference, scale);
             let iteration_quotient: GenericGFPoly = self.field.build_monomial(degree_difference, scale);
            quotient = quotient.add_or_subtract(iteration_quotient);
            remainder = remainder.add_or_subtract(term);
        }
        return  : vec![GenericGFPoly; 2] = vec![quotient, remainder, ]
        ;
    }

    pub fn  to_string(&self) -> String  {
        if self.is_zero() {
            return "0";
        }
         let result: StringBuilder = StringBuilder::new(8 * self.get_degree());
         {
             let mut degree: i32 = self.get_degree();
            while degree >= 0 {
                {
                     let mut coefficient: i32 = self.get_coefficient(degree);
                    if coefficient != 0 {
                        if coefficient < 0 {
                            if degree == self.get_degree() {
                                result.append("-");
                            } else {
                                result.append(" - ");
                            }
                            coefficient = -coefficient;
                        } else {
                            if result.length() > 0 {
                                result.append(" + ");
                            }
                        }
                        if degree == 0 || coefficient != 1 {
                             let alpha_power: i32 = self.field.log(coefficient);
                            if alpha_power == 0 {
                                result.append('1');
                            } else if alpha_power == 1 {
                                result.append('a');
                            } else {
                                result.append("a^");
                                result.append(alpha_power);
                            }
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

