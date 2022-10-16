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

//package com.google.zxing.common.reedsolomon;

use std::fmt;

use crate::Exceptions;

use super::{GenericGFRef, GenericGF};

/**
 * <p>Represents a polynomial whose coefficients are elements of a GF.
 * Instances of this class are immutable.</p>
 *
 * <p>Much credit is due to William Rucklidge since portions of this code are an indirect
 * port of his C++ Reed-Solomon implementation.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericGFPoly {
    field: GenericGFRef,
    coefficients: Vec<i32>,
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
    pub fn new(field: GenericGFRef, coefficients: &Vec<i32>) -> Result<Self, Exceptions> {
        if coefficients.len() == 0 {
            return Err(Exceptions::IllegalArgumentException(
                "coefficients.len()".to_owned(),
            ));
        }
        Ok(Self {
            field: field,
            coefficients: {
                let coefficients_length = coefficients.len();
                if coefficients_length > 1 && coefficients[0] == 0 {
                    // Leading term must be non-zero for anything except the constant polynomial "0"
                    let mut first_non_zero = 1;
                    while first_non_zero < coefficients_length && coefficients[first_non_zero] == 0
                    {
                        first_non_zero += 1;
                    }
                    if first_non_zero == coefficients_length {
                        vec![0]
                    } else {
                        let mut new_coefficients = vec![0; coefficients_length - first_non_zero];
                        let l = new_coefficients.len() - 1;
                        new_coefficients[0..=l].clone_from_slice(&coefficients[first_non_zero..]);
                        // System.arraycopy(coefficients,
                        //     firstNonZero,
                        //     this.coefficients,
                        //     0,
                        //     this.coefficients.length);
                        new_coefficients
                    }
                } else {
                    coefficients.to_vec()
                }
            },
        })
    }

    pub fn getCoefficients(&self) -> &Vec<i32> {
        return &self.coefficients;
    }

    /**
     * @return degree of this polynomial
     */
    pub fn getDegree(&self) -> usize {
        return self.coefficients.len() - 1;
    }

    /**
     * @return true iff this polynomial is the monomial "0"
     */
    pub fn isZero(&self) -> bool {
        return self.coefficients[0] == 0;
    }

    /**
     * @return coefficient of x^degree term in this polynomial
     */
    pub fn getCoefficient(&self, degree: usize) -> i32 {
        return self.coefficients[self.coefficients.len() - 1 - degree];
    }

    /**
     * @return evaluation of this polynomial at a given point
     */
    pub fn evaluateAt(&self, a: usize) -> i32 {
        if a == 0 {
            // Just return the x^0 coefficient
            return self.getCoefficient(0);
        }
        if a == 1 {
            // Just the sum of the coefficients
            let mut result = 0;
            for coefficient in &self.coefficients {
                //for (int coefficient : coefficients) {
                result = GenericGF::addOrSubtract(result, *coefficient);
            }
            return result;
        }
        let mut result = self.coefficients[0];
        let size = self.coefficients.len();
        for i in 1..size {
            //for (int i = 1; i < size; i++) {
            result = GenericGF::addOrSubtract(
                self.field.multiply(a as i32, result as i32),
                self.coefficients[i],
            );
        }
        return result;
    }

    pub fn addOrSubtract(&self, other: &GenericGFPoly) -> Result<GenericGFPoly, Exceptions> {
        if self.field != other.field {
            return Err(Exceptions::IllegalArgumentException(
                "GenericGFPolys do not have same GenericGF field".to_owned(),
            ));
        }
        if self.isZero() {
            return Ok(other.clone());
        }
        if other.isZero() {
            return Ok(self.clone());
        }

        let mut smallerCoefficients = self.coefficients.clone();
        let mut largerCoefficients = other.coefficients.clone();
        if smallerCoefficients.len() > largerCoefficients.len() {
            let temp = smallerCoefficients;
            smallerCoefficients = largerCoefficients;
            largerCoefficients = temp;
        }

        let mut sumDiff = vec![0; largerCoefficients.len()];
        let lengthDiff = largerCoefficients.len() - smallerCoefficients.len();
        // Copy high-order terms only found in higher-degree polynomial's coefficients
        sumDiff[0..lengthDiff].clone_from_slice(&largerCoefficients[0..lengthDiff]);
        //System.arraycopy(largerCoefficients, 0, sumDiff, 0, lengthDiff);

        for i in lengthDiff..largerCoefficients.len() {
            //for (int i = lengthDiff; i < largerCoefficients.length; i++) {
            sumDiff[i] = GenericGF::addOrSubtract(
                smallerCoefficients[i - lengthDiff],
                largerCoefficients[i],
            );
        }

        return Ok(GenericGFPoly::new(self.field, &sumDiff)?);
    }

    pub fn multiply(&self, other: &GenericGFPoly) -> Result<GenericGFPoly, Exceptions> {
        if self.field != other.field {
            //if (!field.equals(other.field)) {
            return Err(Exceptions::IllegalArgumentException(
                "GenericGFPolys do not have same GenericGF field".to_owned(),
            ));
        }
        if self.isZero() || other.isZero() {
            return Ok(self.getZero());
        }
        let aCoefficients = self.coefficients.clone();
        let aLength = aCoefficients.len();
        let bCoefficients = other.coefficients.clone();
        let bLength = bCoefficients.len();
        let mut product = vec![0; aLength + bLength - 1];
        for i in 0..aLength {
            //for (int i = 0; i < aLength; i++) {
            let aCoeff = aCoefficients[i];
            for j in 0..bLength {
                //for (int j = 0; j < bLength; j++) {
                product[i + j] = GenericGF::addOrSubtract(
                    product[i + j],
                    self.field.multiply(aCoeff, bCoefficients[j]),
                );
            }
        }
        return Ok(GenericGFPoly::new(self.field, &product)?);
    }

    pub fn multiply_with_scalar(&self, scalar: i32) -> GenericGFPoly {
        if scalar == 0 {
            return self.getZero();
        }
        if scalar == 1 {
            return self.clone();
        }
        let size = self.coefficients.len();

        let mut product = vec![0; size];
        for i in 0..size {
            //for (int i = 0; i < size; i++) {
            product[i] = self.field.multiply(self.coefficients[i], scalar);
        }
        return GenericGFPoly::new(self.field, &product).unwrap();
    }

    pub fn getZero(&self) -> Self {
        GenericGFPoly::new(self.field, &vec![0]).unwrap()
    }

    pub fn getOne(&self) -> Self {
        GenericGFPoly::new(self.field, &vec![1]).unwrap()
    }

    pub fn multiply_by_monomial(
        &self,
        degree: usize,
        coefficient: i32,
    ) -> Result<GenericGFPoly, Exceptions> {
        if coefficient == 0 {
            return Ok(self.getZero());
        }
        let size = self.coefficients.len();
        let mut product = vec![0; size + degree];
        for i in 0..size {
            //for (int i = 0; i < size; i++) {
            product[i] = self.field.multiply(self.coefficients[i], coefficient);
        }
        return Ok(GenericGFPoly::new(self.field, &product)?);
    }

    pub fn divide(
        &self,
        other: &GenericGFPoly,
    ) -> Result<(GenericGFPoly, GenericGFPoly), Exceptions> {
        if self.field != other.field {
            return Err(Exceptions::IllegalArgumentException(
                "GenericGFPolys do not have same GenericGF field".to_owned(),
            ));
        }
        if other.isZero() {
            return Err(Exceptions::IllegalArgumentException(
                "Divide by 0".to_owned(),
            ));
        }

        let mut quotient = self.getZero();
        let mut remainder = self.clone();

        let denominator_leading_term = other.getCoefficient(other.getDegree());
        let inverse_denominator_leading_term = match self.field.inverse(denominator_leading_term) {
            Ok(val) => val,
            Err(_issue) => {
                return Err(Exceptions::IllegalArgumentException(
                    "arithmetic issue".to_owned(),
                ))
            }
        };

        while remainder.getDegree() >= other.getDegree() && !remainder.isZero() {
            let degree_difference = remainder.getDegree() - other.getDegree();
            let scale = self.field.multiply(
                remainder.getCoefficient(remainder.getDegree()),
                inverse_denominator_leading_term,
            );
            let term = other.multiply_by_monomial(degree_difference, scale)?;
            let iteration_quotient = GenericGF::buildMonomial(self.field, degree_difference, scale);
            quotient = quotient.addOrSubtract(&iteration_quotient)?;
            remainder = remainder.addOrSubtract(&term)?;
        }

        return Ok((quotient, remainder));
    }
}

impl fmt::Display for GenericGFPoly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.isZero() {
            return write!(f, "0");
        }
        let mut result = String::with_capacity(8 * self.getDegree());
        for degree in (0..=self.getDegree()).rev() {
            //for (int degree = getDegree(); degree >= 0; degree--) {
            let mut coefficient = self.getCoefficient(degree);
            if coefficient != 0 {
                if coefficient < 0 {
                    if degree == self.getDegree() {
                        result.push_str("-");
                    } else {
                        result.push_str(" - ");
                    }
                    coefficient = -coefficient;
                } else {
                    if result.len() > 0 {
                        result.push_str(" + ");
                    }
                }
                if degree == 0 || coefficient != 1 {
                    if let Ok(alpha_power) = self.field.log(coefficient) {
                        if alpha_power == 0 {
                            result.push_str("1");
                        } else if alpha_power == 1 {
                            result.push_str("a");
                        } else {
                            result.push_str("a^");
                            result.push_str(&format!("{}", alpha_power));
                        }
                    }
                }
                if degree != 0 {
                    if degree == 1 {
                        result.push_str("x");
                    } else {
                        result.push_str("x^");
                        result.push_str(&format!("{}", degree));
                    }
                }
            }
        }
        write!(f, "{}", result)
    }
}