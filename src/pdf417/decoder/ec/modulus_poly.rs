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

use std::rc::Rc;

use crate::Exceptions;

use super::ModulusGF;

/**
 * @author Sean Owen
 */
#[derive(Clone, Debug)]
pub struct ModulusPoly {
    field: &'static ModulusGF,
    coefficients: Vec<u32>,
    // zero: Option<Rc<ModulusPoly>>,
    // one: Option<Rc<ModulusPoly>>,
}
impl ModulusPoly {
    pub fn new(
        field: &'static ModulusGF,
        coefficients: Vec<u32>,
    ) -> Result<ModulusPoly, Exceptions> {
        if coefficients.is_empty() {
            return Err(Exceptions::IllegalArgumentException(None));
        }
        let orig_coefs = coefficients.clone();
        let mut coefficients = coefficients;
        let coefficientsLength = coefficients.len();
        if coefficientsLength > 1 && coefficients[0] == 0 {
            // Leading term must be non-zero for anything except the constant polynomial "0"
            let mut firstNonZero = 1;
            while firstNonZero < coefficientsLength && coefficients[firstNonZero] == 0 {
                firstNonZero += 1;
            }
            if firstNonZero == coefficientsLength {
                coefficients = vec![0];
            } else {
                coefficients = vec![0u32; coefficientsLength - firstNonZero];
                coefficients[..].copy_from_slice(&orig_coefs[firstNonZero..]);
                // System.arraycopy(coefficients,
                //     firstNonZero,
                //     this.coefficients,
                //     0,
                //     this.coefficients.length);
            }
        }
        // } else {
        //     coefficients = coefficients;
        // }

        Ok(ModulusPoly {
            field,
            coefficients,
            // zero: Some(Self::getZero(field.clone())),
            // one: Some(Self::getOne(field.clone())),
        })
    }

    pub fn getCoefficients(&self) -> &[u32] {
        &self.coefficients
    }

    /**
     * @return degree of this polynomial
     */
    pub fn getDegree(&self) -> u32 {
        self.coefficients.len() as u32 - 1
    }

    /**
     * @return true iff this polynomial is the monomial "0"
     */
    pub fn isZero(&self) -> bool {
        self.coefficients[0] == 0
    }

    /**
     * @return coefficient of x^degree term in this polynomial
     */
    pub fn getCoefficient(&self, degree: usize) -> u32 {
        self.coefficients[self.coefficients.len() - 1 - degree]
    }

    /**
     * @return evaluation of this polynomial at a given point
     */
    pub fn evaluateAt(&self, a: u32) -> u32 {
        if a == 0 {
            // Just return the x^0 coefficient
            return self.getCoefficient(0);
        }
        if a == 1 {
            // Just the sum of the coefficients
            let mut result = 0;
            for coefficient in self.coefficients.iter() {
                // for (int coefficient : coefficients) {
                result = self.field.add(result, *coefficient);
            }
            return result;
        }
        let mut result = self.coefficients[0];
        let size = self.coefficients.len();
        for i in 1..size {
            // for (int i = 1; i < size; i++) {
            result = self
                .field
                .add(self.field.multiply(a, result), self.coefficients[i]);
        }
        result
    }

    pub fn add(&self, other: Rc<ModulusPoly>) -> Result<Rc<ModulusPoly>, Exceptions> {
        if self.field != other.field {
            return Err(Exceptions::IllegalArgumentException(Some(
                "ModulusPolys do not have same ModulusGF field".to_owned(),
            )));
        }
        if self.isZero() {
            return Ok(other);
        }
        if other.isZero() {
            return Ok(Rc::new(self.clone()));
        }

        let mut smallerCoefficients = &self.coefficients;
        let mut largerCoefficients = &other.coefficients;
        if smallerCoefficients.len() > largerCoefficients.len() {
            std::mem::swap(&mut smallerCoefficients, &mut largerCoefficients);
        }
        let mut sumDiff = vec![0_u32; largerCoefficients.len()];
        let lengthDiff = largerCoefficients.len() - smallerCoefficients.len();
        // Copy high-order terms only found in higher-degree polynomial's coefficients
        sumDiff[..lengthDiff].copy_from_slice(&largerCoefficients[..lengthDiff]);
        // System.arraycopy(largerCoefficients, 0, sumDiff, 0, lengthDiff);

        for i in lengthDiff..largerCoefficients.len() {
            // for (int i = lengthDiff; i < largerCoefficients.length; i++) {
            sumDiff[i] = self
                .field
                .add(smallerCoefficients[i - lengthDiff], largerCoefficients[i]);
        }

        Ok(Rc::new(
            ModulusPoly::new(self.field, sumDiff)?,
        ))
    }

    pub fn subtract(&self, other: Rc<ModulusPoly>) -> Result<Rc<ModulusPoly>, Exceptions> {
        if self.field != other.field {
            return Err(Exceptions::IllegalArgumentException(Some(
                "ModulusPolys do not have same ModulusGF field".to_owned(),
            )));
        }
        if other.isZero() {
            return Ok(Rc::new(self.clone()));
        };
        self.add(other.negative())
    }

    pub fn multiply(&self, other: Rc<ModulusPoly>) -> Result<Rc<ModulusPoly>, Exceptions> {
        if !(self.field == other.field) {
            return Err(Exceptions::IllegalArgumentException(Some(
                "ModulusPolys do not have same ModulusGF field".to_owned(),
            )));
        }
        if self.isZero() || other.isZero() {
            return Ok(Self::getZero(self.field));
        }
        let aCoefficients = &self.coefficients;
        let aLength = aCoefficients.len();
        let bCoefficients = &other.coefficients;
        let bLength = bCoefficients.len();
        let mut product = vec![0u32; aLength + bLength - 1];
        for i in 0..aLength {
            // for (int i = 0; i < aLength; i++) {
            let aCoeff = aCoefficients[i];
            for j in 0..bLength {
                // for (int j = 0; j < bLength; j++) {
                product[i + j] = self.field.add(
                    product[i + j],
                    self.field.multiply(aCoeff, bCoefficients[j]),
                );
            }
        }

        Ok(Rc::new(
            ModulusPoly::new(self.field, product)?,
        ))
    }

    pub fn negative(&self) -> Rc<ModulusPoly> {
        let size = self.coefficients.len();
        let mut negativeCoefficients = vec![0u32; size];
        for (i, neg_coef) in negativeCoefficients.iter_mut().enumerate().take(size) {
            // for (int i = 0; i < size; i++) {
            *neg_coef = self.field.subtract(0, self.coefficients[i]);
        }
        Rc::new(
            ModulusPoly::new(self.field, negativeCoefficients)
                .expect("should always generate with known goods"),
        )
    }

    pub fn multiplyByScaler(&self, scalar: u32) -> Rc<ModulusPoly> {
        if scalar == 0 {
            return Self::getZero(self.field);
        }
        if scalar == 1 {
            return Rc::new(self.clone());
        }
        let size = self.coefficients.len();
        let mut product = vec![0u32; size];
        for (i, prod) in product.iter_mut().enumerate().take(size) {
            // for (int i = 0; i < size; i++) {
            *prod = self.field.multiply(self.coefficients[i], scalar);
        }

        Rc::new(
            ModulusPoly::new(self.field, product).expect("should always generate with known goods"),
        )
    }

    pub fn multiplyByMonomial(&self, degree: usize, coefficient: u32) -> Rc<ModulusPoly> {
        if coefficient == 0 {
            return Self::getZero(self.field);
        }
        let size = self.coefficients.len();
        let mut product = vec![0u32; size + degree];
        for (i, prod) in product.iter_mut().enumerate().take(size) {
            // for (int i = 0; i < size; i++) {
            *prod = self.field.multiply(self.coefficients[i], coefficient);
        }

        Rc::new(
            ModulusPoly::new(self.field, product).expect("should always generate with known goods"),
        )
    }

    pub fn getZero(field: &'static ModulusGF) -> Rc<ModulusPoly> {
        Rc::new(ModulusPoly::new(field, vec![0]).expect("should always generate with known goods"))
    }

    pub fn getOne(field: &'static ModulusGF) -> Rc<ModulusPoly> {
        Rc::new(ModulusPoly::new(field, vec![1]).expect("should always generate with known goods"))
    }

    pub fn buildMonomial(
        field: &'static ModulusGF,
        degree: usize,
        coefficient: u32,
    ) -> Rc<ModulusPoly> {
        // if degree < 0 {
        //   throw new IllegalArgumentException();
        // }
        if coefficient == 0 {
            return Self::getZero(field);
        }
        let mut coefficients = vec![0_u32; degree + 1];
        coefficients[0] = coefficient;
        Rc::new(
            ModulusPoly::new(field, coefficients).expect("should always generate with known goods"),
        )
    }

    // @Override
    // public String toString() {
    //   StringBuilder result = new StringBuilder(8 * getDegree());
    //   for (int degree = getDegree(); degree >= 0; degree--) {
    //     int coefficient = getCoefficient(degree);
    //     if (coefficient != 0) {
    //       if (coefficient < 0) {
    //         result.append(" - ");
    //         coefficient = -coefficient;
    //       } else {
    //         if (result.length() > 0) {
    //           result.append(" + ");
    //         }
    //       }
    //       if (degree == 0 || coefficient != 1) {
    //         result.append(coefficient);
    //       }
    //       if (degree != 0) {
    //         if (degree == 1) {
    //           result.append('x');
    //         } else {
    //           result.append("x^");
    //           result.append(degree);
    //         }
    //       }
    //     }
    //   }
    //   return result.toString();
    // }
}
