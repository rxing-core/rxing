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

use crate::common::Result;
use crate::Exceptions;

use super::{GenericGF, GenericGFPoly, GenericGFRef};

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
    field: GenericGFRef,
}

impl ReedSolomonDecoder {
    pub fn new(field: GenericGFRef) -> Self {
        Self { field }
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
    pub fn decode(&self, received: &mut Vec<i32>, twoS: i32) -> Result<usize> {
        let poly = GenericGFPoly::new(self.field, received)?;
        let mut syndromeCoefficients = vec![0; twoS as usize];
        let mut noError = true;
        for i in 0..twoS {
            //for (int i = 0; i < twoS; i++) {
            let eval = poly.evaluateAt(self.field.exp(i + self.field.getGeneratorBase()) as usize);
            let len = syndromeCoefficients.len();
            syndromeCoefficients[len - 1 - i as usize] = eval;
            if eval != 0 {
                noError = false;
            }
        }
        if noError {
            return Ok(0);
        }
        let Ok(syndrome) = GenericGFPoly::new(self.field, &syndromeCoefficients) else {
             return Err(Exceptions::ReedSolomonException(None));
        };
        let sigmaOmega = self.runEuclideanAlgorithm(
            &GenericGF::buildMonomial(self.field, twoS as usize, 1),
            &syndrome,
            twoS as usize,
        )?;
        let sigma = &sigmaOmega[0];
        let omega = &sigmaOmega[1];
        let errorLocations = self.findErrorLocations(sigma)?;
        let errorMagnitudes = self.findErrorMagnitudes(omega, &errorLocations)?;
        for i in 0..errorLocations.len() {
            //for (int i = 0; i < errorLocations.length; i++) {
            let log_value = self.field.log(errorLocations[i] as i32)?;
            if log_value > received.len() as i32 - 1 {
                return Err(Exceptions::ReedSolomonException(Some(
                    "Bad error location".to_owned(),
                )));
            }
            let position: isize = received.len() as isize - 1 - log_value as isize;
            if position < 0 {
                return Err(Exceptions::ReedSolomonException(Some(
                    "Bad error location".to_owned(),
                )));
            }
            received[position as usize] =
                GenericGF::addOrSubtract(received[position as usize], errorMagnitudes[i]);
        }
        Ok(errorLocations.len())
    }

    fn runEuclideanAlgorithm(
        &self,
        a: &GenericGFPoly,
        b: &GenericGFPoly,
        R: usize,
    ) -> Result<Vec<GenericGFPoly>> {
        // Assume a's degree is >= b's
        let mut a = a.clone();
        let mut b = b.clone();
        if a.getDegree() < b.getDegree() {
            std::mem::swap(&mut a, &mut b);
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
                return Err(Exceptions::ReedSolomonException(Some(
                    "r_{i-1} was zero".to_owned(),
                )));
            }
            r = rLastLast;
            let mut q = r.getZero();
            let denominatorLeadingTerm = rLast.getCoefficient(rLast.getDegree());
            let dltInverse = self.field.inverse(denominatorLeadingTerm)?;
            while r.getDegree() >= rLast.getDegree() && !r.isZero() {
                let degreeDiff = r.getDegree() - rLast.getDegree();
                let scale = self
                    .field
                    .multiply(r.getCoefficient(r.getDegree()), dltInverse);
                q = q.addOrSubtract(&GenericGF::buildMonomial(self.field, degreeDiff, scale))?;
                r = r.addOrSubtract(&rLast.multiply_by_monomial(degreeDiff, scale)?)?;
            }

            t = (q.multiply(&tLast)?).addOrSubtract(&tLastLast)?;

            if r.getDegree() >= rLast.getDegree() {
                return Err(Exceptions::ReedSolomonException(Some(format!(
                    "Division algorithm failed to reduce polynomial? r: {r}, rLast: {rLast}"
                ))));
            }
        }

        let sigmaTildeAtZero = t.getCoefficient(0);
        if sigmaTildeAtZero == 0 {
            return Err(Exceptions::ReedSolomonException(Some(
                "sigmaTilde(0) was zero".to_owned(),
            )));
        }

        let inverse = match self.field.inverse(sigmaTildeAtZero) {
            Ok(res) => res,
            Err(_err) => {
                return Err(Exceptions::ReedSolomonException(Some(
                    "ArithmetricException".to_owned(),
                )))
            }
        };
        let sigma = t.multiply_with_scalar(inverse);
        let omega = r.multiply_with_scalar(inverse);
        Ok(vec![sigma, omega])
    }

    fn findErrorLocations(&self, errorLocator: &GenericGFPoly) -> Result<Vec<usize>> {
        // This is a direct application of Chien's search
        let numErrors = errorLocator.getDegree();
        if numErrors == 1 {
            // shortcut
            return Ok(vec![errorLocator.getCoefficient(1) as usize]);
        }

        let mut result: Vec<usize> = vec![0; numErrors];
        let mut e = 0;
        for i in 1..self.field.getSize() {
            //for (int i = 1; i < field.getSize() && e < numErrors; i++) {
            if e >= numErrors {
                break;
            }
            if errorLocator.evaluateAt(i) == 0 {
                result[e] = self.field.inverse(i as i32)? as usize;
                e += 1;
            }
        }
        if e != numErrors {
            return Err(Exceptions::ReedSolomonException(Some(
                "Error locator degree does not match number of roots".to_owned(),
            )));
        }
        Ok(result)
    }

    fn findErrorMagnitudes(
        &self,
        errorEvaluator: &GenericGFPoly,
        errorLocations: &Vec<usize>,
    ) -> Result<Vec<i32>> {
        // This is directly applying Forney's Formula
        let s = errorLocations.len();
        let mut result = vec![0; s];
        for i in 0..s {
            //for (int i = 0; i < s; i++) {
            let xiInverse = self.field.inverse(errorLocations[i] as i32)?;
            let mut denominator = 1;
            for (j, loc) in errorLocations.iter().enumerate().take(s) {
                // for j in 0..s {
                //for (int j = 0; j < s; j++) {
                if i != j {
                    //denominator = field.multiply(denominator,
                    //    GenericGF.addOrSubtract(1, field.multiply(errorLocations[j], xiInverse)));
                    // Above should work but fails on some Apple and Linux JDKs due to a Hotspot bug.
                    // Below is a funny-looking workaround from Steven Parkes
                    let term = self.field.multiply(*loc as i32, xiInverse);
                    let termPlus1 = if (term & 0x1) == 0 {
                        term | 1
                    } else {
                        term & !1
                    };
                    denominator = self.field.multiply(denominator, termPlus1);
                }
            }
            result[i] = self.field.multiply(
                errorEvaluator.evaluateAt(xiInverse as usize),
                self.field.inverse(denominator)?,
            );
            if self.field.getGeneratorBase() != 0 {
                result[i] = self.field.multiply(result[i], xiInverse);
            }
        }
        Ok(result)
    }
}
