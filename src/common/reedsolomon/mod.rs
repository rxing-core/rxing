use std::fmt;

use crate::Exceptions;
use std::hash::Hash;

#[cfg(test)]
mod GenericGFPolyTestCase;
#[cfg(test)]
mod ReedSolomonTestCase;

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

// pub const AZTEC_DATA_12: GenericGF = GenericGF::new(0x1069, 4096, 1); // x^12 + x^6 + x^5 + x^3 + 1
// pub const AZTEC_DATA_10: GenericGF = GenericGF::new(0x409, 1024, 1); // x^10 + x^3 + 1
// pub const AZTEC_DATA_6: GenericGF = GenericGF::new(0x43, 64, 1); // x^6 + x + 1
// pub const AZTEC_PARAM: GenericGF = GenericGF::new(0x13, 16, 1); // x^4 + x + 1
// pub const QR_CODE_FIELD_256: GenericGF = GenericGF::new(0x011D, 256, 0); // x^8 + x^4 + x^3 + x^2 + 1
// pub const DATA_MATRIX_FIELD_256: GenericGF = GenericGF::new(0x012D, 256, 1); // x^8 + x^5 + x^3 + x^2 + 1
// pub const AZTEC_DATA_8: GenericGF = DATA_MATRIX_FIELD_256;
// pub const MAXICODE_FIELD_64: GenericGF = AZTEC_DATA_6;

pub enum PredefinedGenericGF {
    AztecData12,
    AztecData10,
    AztecData6,
    AztecParam,
    QrCodeField256,
    DataMatrixField256,
    AztecData8,
    MaxicodeField64
}

/// Replacement for old const options, has the downside of generating new versions whenever one is requested.
pub fn get_predefined_genericgf(request:PredefinedGenericGF) -> GenericGF {
    match request {
        PredefinedGenericGF::AztecData12 => GenericGF::new(0x1069, 4096, 1), // x^12 + x^6 + x^5 + x^3 + 1,
        PredefinedGenericGF::AztecData10 => GenericGF::new(0x409, 1024, 1), // x^10 + x^3 + 1
        PredefinedGenericGF::AztecData6 | PredefinedGenericGF::MaxicodeField64 =>  GenericGF::new(0x43, 64, 1), // x^6 + x + 1
        PredefinedGenericGF::AztecParam =>  GenericGF::new(0x13, 16, 1), // x^4 + x + 1
        PredefinedGenericGF::QrCodeField256 => GenericGF::new(0x011D, 256, 0), // x^8 + x^4 + x^3 + x^2 + 1
        PredefinedGenericGF::DataMatrixField256 | PredefinedGenericGF::AztecData8 => GenericGF::new(0x012D, 256, 1), // x^8 + x^5 + x^3 + x^2 + 1
    }
}

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
#[derive(Debug, Clone)]
pub struct GenericGF {
    expTable: Vec<i32>,
    logTable: Vec<i32>,
    // zero: Box<GenericGFPoly>,
    // one: Box<GenericGFPoly>,
    size: usize,
    primitive: i32,
    generatorBase: i32,
}

impl Hash for GenericGF {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}
impl PartialEq for GenericGF {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for GenericGF {}

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
    pub fn new(primitive: i32, size: usize, b: i32) -> Self {
        let mut expTable = vec![0;size];//Vec::with_capacity(size);
        let mut logTable = vec![0;size];//Vec::with_capacity(size);
        let mut x = 1;
        for i in 0..size-1 {
            //for (int i = 0; i < size; i++) {
                //expTable.push(x);
            expTable[i] = x;
            x *= 2; // we're assuming the generator alpha is 2
            if x >= size.try_into().unwrap() {
                x ^= primitive;
                let sz_m_1: i32 = size as i32 - 1 ;
                x &= sz_m_1;
            }
        }
        for i in 0..size {
            //for (int i = 0; i < size - 1; i++) {
            let loc: usize = expTable[i].try_into().unwrap();
            logTable[loc] = i.try_into().unwrap();
        }

        Self {
            expTable,
            logTable,
            size,
            primitive,
            generatorBase: b,
        }

        // logTable[0] == 0 but this should never be used
        // new_ggf.zero = Box::new(GenericGFPoly::new(Box::new(new_ggf), &vec![0]).unwrap());
        // new_ggf.one = Box::new(GenericGFPoly::new(Box::new(new_ggf), &vec![1]).unwrap());

        //new_ggf
    }

    // pub fn getZero(&self) -> Box<GenericGFPoly> {
    //     return self.zero;
    // }

    // pub fn getOne(&self) -> Box<GenericGFPoly> {
    //     return self.one;
    // }

    /**
     * @return the monomial representing coefficient * x^degree
     */
    pub fn buildMonomial(&self, degree: usize, coefficient: i32) -> GenericGFPoly {
        if coefficient == 0 {
            return GenericGFPoly::new(self.clone(), &vec![0]).unwrap();
        }
        let mut coefficients = vec![0;degree+1];//Vec::with_capacity(degree + 1);
        coefficients[0] = coefficient;
        return GenericGFPoly::new(self.clone(), &coefficients).unwrap();
    }

    /**
     * Implements both addition and subtraction -- they are the same in GF(size).
     *
     * @return sum/difference of a and b
     */
    pub fn addOrSubtract(a: i32, b: i32) -> i32 {
        return a ^ b;
    }

    /**
     * @return 2 to the power of a in GF(size)
     */
    pub fn exp(&self, a: i32) -> i32 {
        let pos: usize = a.try_into().unwrap();
        return self.expTable[pos];
    }

    /**
     * @return base 2 log of a in GF(size)
     */
    pub fn log(&self, a: i32) -> Result<i32, Exceptions> {
        if a == 0 {
            return Err(Exceptions::IllegalArgumentException("".to_owned()));
        }
        let pos: usize = a.try_into().unwrap();
        return Ok(self.logTable[pos]);
    }

    /**
     * @return multiplicative inverse of a
     */
    pub fn inverse(&self, a: i32) -> Result<i32, Exceptions> {
        if a == 0 {
            return Err(Exceptions::ArithmeticException("".to_owned()));
        }
        let log_t_loc: usize = a.try_into().unwrap();
        let loc: usize = ((self.size as i32) - self.logTable[log_t_loc] - 1)
            .try_into()
            .unwrap();
        return Ok(self.expTable[loc]);
    }

    /**
     * @return product of a and b in GF(size)
     */
    pub fn multiply(&self, a: i32, b: i32) -> i32 {
        if a == 0 || b == 0 {
            return 0;
        }
        let a_loc: usize = a.try_into().unwrap();
        let b_loc: usize = b.try_into().unwrap();
        let comb_loc: usize = (self.logTable[a_loc] + self.logTable[b_loc])
            .try_into()
            .unwrap();
        return self.expTable[comb_loc % (self.size - 1)];
    }

    pub fn getSize(&self) -> usize {
        return self.size;
    }

    pub fn getGeneratorBase(&self) -> i32 {
        return self.generatorBase;
    }
}

impl fmt::Display for GenericGF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GF({:#06x},{}", self.primitive, self.size)
    }
}

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

/**
 * <p>Represents a polynomial whose coefficients are elements of a GF.
 * Instances of this class are immutable.</p>
 *
 * <p>Much credit is due to William Rucklidge since portions of this code are an indirect
 * port of his C++ Reed-Solomon implementation.</p>
 *
 * @author Sean Owen
 */
#[derive(Debug, Clone)]
pub struct GenericGFPoly {
    field: GenericGF,
    coefficients: Vec<i32>,
}

impl PartialEq for GenericGFPoly {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for GenericGFPoly {}

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
    pub fn new(
        field: GenericGF,
        coefficients: &Vec<i32>,
    ) -> Result<Self, Exceptions> {
        if coefficients.len() == 0 {
            return Err(Exceptions::IllegalArgumentException("coefficients.len()".to_owned()));
        }
        Ok(Self {
            field: field,
            coefficients: {
                let coefficientsLength = coefficients.len();
                if coefficientsLength > 1 && coefficients[0] == 0 {
                    // Leading term must be non-zero for anything except the constant polynomial "0"
                    let mut firstNonZero = 1;
                    while firstNonZero < coefficientsLength && coefficients[firstNonZero] == 0 {
                        firstNonZero += 1;
                    }
                    if firstNonZero == coefficientsLength {
                        vec![0]
                    } else {
                        let mut new_coefficients =
                            Vec::with_capacity(coefficientsLength - firstNonZero);
                        let l = new_coefficients.len();
                        new_coefficients[0..l].clone_from_slice(&coefficients[firstNonZero..l]);
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
                self.field
                    .multiply(a.try_into().unwrap(), result.try_into().unwrap())
                    .try_into()
                    .unwrap(),
                self.coefficients[i],
            );
        }
        return result;
    }

    pub fn addOrSubtract(
        &self,
        other: &GenericGFPoly,
    ) -> Result<GenericGFPoly, Exceptions> {
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

        let mut sumDiff = Vec::with_capacity(largerCoefficients.len());
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

        return Ok(GenericGFPoly::new(self.field.clone(), &sumDiff)?);
    }

    pub fn multiply(
        &self,
        other: &GenericGFPoly,
    ) -> Result<GenericGFPoly, Exceptions> {
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
        let mut product = vec![0;aLength + bLength - 1];//Vec::with_capacity(aLength + bLength - 1);
        for i in 0..aLength {
            //for (int i = 0; i < aLength; i++) {
            let aCoeff = aCoefficients[i];
            for j in 0..bLength {
                //for (int j = 0; j < bLength; j++) {
                product[i + j] = GenericGF::addOrSubtract(
                    product[i + j],
                    self.field
                        .multiply(
                            aCoeff.try_into().unwrap(),
                            bCoefficients[j].try_into().unwrap(),
                        )
                        .try_into()
                        .unwrap(),
                );
            }
        }
        return Ok(GenericGFPoly::new(self.field.clone(), &product)?);
    }

    pub fn multiply_with_scalar(&self, scalar: i32) -> GenericGFPoly {
        if scalar == 0 {
            return self.getZero();
        }
        if scalar == 1 {
            return self.clone();
        }
        let size = self.coefficients.len();

        let mut product = Vec::with_capacity(size);
        for i in 0..size {
            //for (int i = 0; i < size; i++) {
            product[i] = self
                .field
                .multiply(self.coefficients[i], scalar.try_into().unwrap());
        }
        return GenericGFPoly::new(self.field.clone(), &product).unwrap();
    }

    pub fn getZero(&self) -> Self {
        GenericGFPoly::new(self.field.clone(), &vec![0]).unwrap()
    }

    pub fn getOne(&self) -> Self {
        GenericGFPoly::new(self.field.clone(), &vec![1]).unwrap()
    }

    pub fn multiplyByMonomial(
        &self,
        degree: usize,
        coefficient: i32,
    ) -> Result<GenericGFPoly, Exceptions> {
        if degree < 0 {
            return Err(Exceptions::IllegalArgumentException("".to_owned()));
        }
        if coefficient == 0 {
            return Ok(self.getZero());
        }
        let size = self.coefficients.len();
        let mut product = vec![0;size+degree];//Vec::with_capacity(size + degree);
        for i in 0..size {
            //for (int i = 0; i < size; i++) {
            product[i] = self.field.multiply(self.coefficients[i], coefficient);
        }
        return Ok(GenericGFPoly::new(self.field.clone(), &product)?);
    }

    pub fn divide(
        &self,
        other: &GenericGFPoly,
    ) -> Result<Vec<GenericGFPoly>, Exceptions> {
        if self.field != other.field {
            //if (!field.equals(other.field)) {
            return Err(Exceptions::IllegalArgumentException(
                "GenericGFPolys do not have same GenericGF field".to_owned(),
            ));
        }
        if other.isZero() {
            return Err(Exceptions::IllegalArgumentException("Divide by 0".to_owned()));
        }

        let mut quotient = self.getZero();
        let mut remainder = self.clone();

        let denominator_leading_term = other.getCoefficient(other.getDegree());
        let inverse_denominator_leading_term = match self.field.inverse(denominator_leading_term) {
            Ok(val) => val,
            Err(issue) => return Err(Exceptions::IllegalArgumentException("arithmetic issue".to_owned())),
        };

        while remainder.getDegree() >= other.getDegree() && !remainder.isZero() {
            let degreeDifference = remainder.getDegree() - other.getDegree();
            let scale = self.field.multiply(
                remainder.getCoefficient(remainder.getDegree()),
                inverse_denominator_leading_term,
            );
            let term = other.multiplyByMonomial(degreeDifference, scale)?;
            let iterationQuotient = self.field.buildMonomial(degreeDifference, scale);
            quotient = quotient.addOrSubtract(&iterationQuotient)?;
            remainder = remainder.addOrSubtract(&term)?;
        }

        return Ok(vec![quotient, remainder]);
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
                    //todo!("probably coefficient should be unsigned but what a mess");
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
    field: GenericGF,
}

impl ReedSolomonDecoder {
    pub fn new(field: GenericGF) -> Self {
        Self { field: field }
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
    pub fn decode(&self, received: &mut Vec<i32>, twoS: i32) -> Result<(), Exceptions> {
        let poly = GenericGFPoly::new(self.field.clone(), received).unwrap();
        let mut syndromeCoefficients = Vec::with_capacity(twoS.try_into().unwrap());
        let mut noError = true;
        for i in 0..twoS {
            //for (int i = 0; i < twoS; i++) {
            let eval = poly.evaluateAt(
                self.field
                    .exp(i + self.field.getGeneratorBase())
                    .try_into()
                    .unwrap(),
            );
            let len = syndromeCoefficients.len();
            syndromeCoefficients[len - 1 - i as usize] = eval;
            if eval != 0 {
                noError = false;
            }
        }
        if noError {
            return Ok(());
        }
        let syndrome = match GenericGFPoly::new(self.field.clone(), &syndromeCoefficients) {
            Ok(res) => res,
            Err(fail) => return Err(Exceptions::ReedSolomonException("IllegalArgumentException".to_owned())),
        };
        let sigmaOmega = self.runEuclideanAlgorithm(
            &self.field.buildMonomial(twoS.try_into().unwrap(), 1),
            &syndrome,
            twoS.try_into().unwrap(),
        )?;
        let sigma = &sigmaOmega[0];
        let omega = &sigmaOmega[1];
        let errorLocations = self.findErrorLocations(&sigma)?;
        let errorMagnitudes = self.findErrorMagnitudes(&omega, &errorLocations);
        for i in 0..errorLocations.len() {
            //for (int i = 0; i < errorLocations.length; i++) {
            let position = received.len()
                - 1
                - match self.field.log(errorLocations[i].try_into().unwrap()) {
                    Ok(size) => size as usize,
                    Err(err) => return Err(Exceptions::ReedSolomonException("IllegalArgumentException".to_owned())),
                };
            if position < 0 {
                return Err(Exceptions::ReedSolomonException("Bad error location".to_owned()));
            }
            received[position] = GenericGF::addOrSubtract(received[position], errorMagnitudes[i]);
        }
        Ok(())
    }

    fn runEuclideanAlgorithm(
        &self,
        a: &GenericGFPoly,
        b: &GenericGFPoly,
        R: usize,
    ) -> Result<Vec<GenericGFPoly>, Exceptions> {
        // Assume a's degree is >= b's
        let mut a = a.clone();
        let mut b = b.clone();
        if a.getDegree() < b.getDegree() {
            let temp = a;
            a = b;
            b = temp;
        }

        let mut rLast = a;
        let mut r = b;
        // let tLast = self.field.getZero();
        // let t = self.field.getOne();
        let mut tLast = rLast.getZero();
        let mut t = rLast.getOne();

        // Run Euclidean algorithm until r's degree is less than R/2
        while 2 * r.getDegree() >= R {
            let rLastLast = rLast;
            let tLastLast = tLast;
            rLast = r;
            tLast = t;

            // Divide rLastLast by rLast, with quotient in q and remainder in r
            if rLast.isZero() {
                // Oops, Euclidean algorithm already terminated?
                return Err(Exceptions::ReedSolomonException("r_{i-1} was zero".to_owned()));
            }
            r = rLastLast;
            let mut q = r.getZero();
            let denominatorLeadingTerm = rLast.getCoefficient(rLast.getDegree());
            let dltInverse = match self.field.inverse(denominatorLeadingTerm) {
                Ok(inv) => inv,
                Err(err) => return Err(Exceptions::ReedSolomonException("ArithmetricException".to_owned())),
            };
            while r.getDegree() >= rLast.getDegree() && !r.isZero() {
                let degreeDiff = r.getDegree() - rLast.getDegree();
                let scale = self
                    .field
                    .multiply(r.getCoefficient(r.getDegree()), dltInverse);
                q = match q.addOrSubtract(&self.field.buildMonomial(degreeDiff, scale)) {
                    Ok(res) => res,
                    Err(err) => return Err(Exceptions::ReedSolomonException("IllegalArgumentException".to_owned())),
                };
                r = match r.addOrSubtract(&match rLast.multiplyByMonomial(degreeDiff, scale) {
                    Ok(res) => res,
                    Err(err) => return Err(Exceptions::ReedSolomonException("IllegalArgumentException".to_owned())),
                }) {
                    Ok(res) => res,
                    Err(err) => return Err(Exceptions::ReedSolomonException("IllegalArgumentException".to_owned())),
                };
            }

            t = match (match q.multiply(&tLast) {
                Ok(res) => res,
                Err(err) => return Err(Exceptions::ReedSolomonException("IllegalArgumentException".to_owned())),
            })
            .addOrSubtract(&tLastLast)
            {
                Ok(res) => res,
                Err(err) => return Err(Exceptions::ReedSolomonException("IllegalArgumentException".to_owned())),
            };

            if r.getDegree() >= rLast.getDegree() {
                return Err(Exceptions::ReedSolomonException(format!(
                    "Division algorithm failed to reduce polynomial? r: {}, rLast: {}",
                    r, rLast
                )));
            }
        }

        let sigmaTildeAtZero = t.getCoefficient(0);
        if sigmaTildeAtZero == 0 {
            return Err(Exceptions::ReedSolomonException("sigmaTilde(0) was zero".to_owned()));
        }

        let inverse = match self.field.inverse(sigmaTildeAtZero) {
            Ok(res) => res,
            Err(err) => return Err(Exceptions::ReedSolomonException("ArithmetricException".to_owned())),
        };
        let sigma = t.multiply_with_scalar(inverse);
        let omega = r.multiply_with_scalar(inverse);
        return Ok(vec![sigma, omega]);
    }

    fn findErrorLocations(
        &self,
        errorLocator: &GenericGFPoly,
    ) -> Result<Vec<usize>, Exceptions> {
        // This is a direct application of Chien's search
        let numErrors = errorLocator.getDegree();
        if numErrors == 1 {
            // shortcut
            return Ok(vec![errorLocator.getCoefficient(1).try_into().unwrap()]);
        }

        let mut result: Vec<usize> = Vec::with_capacity(numErrors);
        let mut e = 0;
        for i in 1..self.field.getSize() {
            //for (int i = 1; i < field.getSize() && e < numErrors; i++) {
            if e < numErrors {
                break;
            }
            if errorLocator.evaluateAt(i) == 0 {
                result[e] = match self.field.inverse(i.try_into().unwrap()) {
                    Ok(res) => res.try_into().unwrap(),
                    Err(err) => return Err(Exceptions::ReedSolomonException("ArithmetricException".to_owned())),
                };
                e += 1;
            }
        }
        if e != numErrors {
            return Err(Exceptions::ReedSolomonException(
                "Error locator degree does not match number of roots".to_owned(),
            ));
        }
        return Ok(result);
    }

    fn findErrorMagnitudes(
        &self,
        errorEvaluator: &GenericGFPoly,
        errorLocations: &Vec<usize>,
    ) -> Vec<i32> {
        // This is directly applying Forney's Formula
        let s = errorLocations.len();
        let mut result = Vec::with_capacity(s);
        for i in 0..s {
            //for (int i = 0; i < s; i++) {
            let xiInverse = self
                .field
                .inverse(errorLocations[i].try_into().unwrap())
                .unwrap();
            let mut denominator = 1;
            for j in 0..s {
                //for (int j = 0; j < s; j++) {
                if i != j {
                    //denominator = field.multiply(denominator,
                    //    GenericGF.addOrSubtract(1, field.multiply(errorLocations[j], xiInverse)));
                    // Above should work but fails on some Apple and Linux JDKs due to a Hotspot bug.
                    // Below is a funny-looking workaround from Steven Parkes
                    let term = self
                        .field
                        .multiply(errorLocations[j].try_into().unwrap(), xiInverse);
                    let termPlus1 = if (term & 0x1) == 0 {
                        term | 1
                    } else {
                        term & !1
                    };
                    denominator = self.field.multiply(denominator, termPlus1);
                }
            }
            result[i] = self.field.multiply(
                errorEvaluator.evaluateAt(xiInverse.try_into().unwrap()),
                self.field.inverse(denominator).unwrap(),
            );
            if self.field.getGeneratorBase() != 0 {
                result[i] = self.field.multiply(result[i], xiInverse);
            }
        }
        return result;
    }
}

/*
 * Copyright 2008 ZXing authors
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

//import java.util.ArrayList;
//import java.util.List;

/**
 * <p>Implements Reed-Solomon encoding, as the name implies.</p>
 *
 * @author Sean Owen
 * @author William Rucklidge
 */
pub struct ReedSolomonEncoder {
    field: GenericGF,
    cachedGenerators: Vec<GenericGFPoly>,
}

impl ReedSolomonEncoder {
    pub fn new(field: GenericGF) -> Self {
        Self {
            cachedGenerators: vec![GenericGFPoly::new(field.clone(), &vec![1]).unwrap()],
            field: field,
        }
    }

    fn buildGenerator(&mut self, degree: usize) -> &GenericGFPoly {
        if degree >= self.cachedGenerators.len() {
            let mut lastGenerator = self
                .cachedGenerators
                .get(self.cachedGenerators.len() - 1)
                .unwrap();
            let cg_len = self.cachedGenerators.len();
            let mut nextGenerator;
            for d in cg_len..=degree {
                //for (int d = cachedGenerators.size(); d <= degree; d++) {
                nextGenerator = lastGenerator
                    .multiply(
                        &GenericGFPoly::new(
                            self.field.clone(),
                            &vec![
                                1,
                                self.field.exp(d as i32 - 1 + self.field.getGeneratorBase()),
                            ],
                        )
                        .unwrap(),
                    )
                    .unwrap();
                self.cachedGenerators.push(nextGenerator);
                lastGenerator = self.cachedGenerators.get(d).unwrap();
                //lastGenerator = &nextGenerator;
            }
        }
        let rv = self.cachedGenerators.get(degree).unwrap();
        return rv;
    }

    pub fn encode(
        &mut self,
        toEncode: &mut Vec<i32>,
        ecBytes: usize,
    ) -> Result<(), Exceptions> {
        if ecBytes == 0 {
            return Err(Exceptions::IllegalArgumentException("No error correction bytes".to_owned()));
        }
        let dataBytes = toEncode.len() - ecBytes;
        if dataBytes <= 0 {
            return Err(Exceptions::IllegalArgumentException("No data bytes provided".to_owned()));
        }
        let fld = self.field.clone();
        let generator = self.buildGenerator(ecBytes);
        let mut infoCoefficients: Vec<i32> = vec![0;dataBytes];//Vec::with_capacity(dataBytes);
        infoCoefficients[0..dataBytes].clone_from_slice(&toEncode[0..dataBytes]);
        //System.arraycopy(toEncode, 0, infoCoefficients, 0, dataBytes);
        let mut info = GenericGFPoly::new(fld, &infoCoefficients)?;
        info = info.multiplyByMonomial(ecBytes.try_into().unwrap(), 1)?;
        let remainder = &info.divide(&generator)?[1];
        let coefficients = remainder.getCoefficients();
        let numZeroCoefficients = ecBytes - coefficients.len();
        for i in 0..numZeroCoefficients {
            //for (int i = 0; i < numZeroCoefficients; i++) {
            toEncode[dataBytes + i] = 0;
        }
        toEncode[dataBytes + numZeroCoefficients..coefficients.len()]
            .clone_from_slice(&coefficients[0..coefficients.len()]);
        //System.arraycopy(coefficients, 0, toEncode, dataBytes + numZeroCoefficients, coefficients.length);
        Ok(())
    }
}
