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

use crate::{
    common::Result,
    pdf417::{decoder::ec::ModulusGF, pdf_417_common::NUMBER_OF_CODEWORDS},
    Exceptions,
};

use super::ModulusPoly;

use once_cell::sync::Lazy;

// static ref PDF417_GF : Rc<&ModulusGF> =  Rc::new(&ModulusGF::new(NUMBER_OF_CODEWORDS, 3));
static FLD_INTERIOR: Lazy<ModulusGF> = Lazy::new(|| ModulusGF::new(NUMBER_OF_CODEWORDS, 3));

/**
 * <p>PDF417 error correction implementation.</p>
 *
 * <p>This <a href="http://en.wikipedia.org/wiki/Reed%E2%80%93Solomon_error_correction#Example">example</a>
 * is quite useful in understanding the algorithm.</p>
 *
 * @author Sean Owen
 * @see com.google.zxing.common.reedsolomon.ReedSolomonDecoder
 */

/**
 * @param received received codewords
 * @param numECCodewords number of those codewords used for EC
 * @param erasures location of erasures
 * @return number of errors
 * @throws ChecksumException if errors cannot be corrected, maybe because of too many errors
 */
pub fn decode(received: &mut [u32], numECCodewords: u32, erasures: &mut [u32]) -> Result<usize> {
    let field: &'static ModulusGF = &FLD_INTERIOR;
    let poly = ModulusPoly::new(field, received.to_vec())?;
    let mut S = vec![0u32; numECCodewords as usize];
    let mut error = false;
    for i in (1..=numECCodewords).rev() {
        // for (int i = numECCodewords; i > 0; i--) {
        let eval = poly.evaluateAt(field.exp(i));
        S[(numECCodewords - i) as usize] = eval;
        if eval != 0 {
            error = true;
        }
    }

    if !error {
        return Ok(0);
    }

    let mut knownErrors: Rc<ModulusPoly> = ModulusPoly::getOne(field);
    let mut b;
    let mut term;
    let mut kE: Rc<ModulusPoly>;
    if !erasures.is_empty() {
        for erasure in erasures {
            // for (int erasure : erasures) {
            b = field.exp(received.len() as u32 - 1 - *erasure);
            // Add (1 - bx) term:
            term = ModulusPoly::new(field, vec![field.subtract(0, b), 1])?;
            kE = knownErrors.clone();
            knownErrors = kE.multiply(Rc::new(term))?;
        }
    }

    let syndrome = Rc::new(ModulusPoly::new(field, S)?);
    //syndrome = syndrome.multiply(knownErrors);

    let sigmaOmega = runEuclideanAlgorithm(
        ModulusPoly::buildMonomial(field, numECCodewords as usize, 1),
        syndrome,
        numECCodewords,
        field,
    )?;
    let sigma = sigmaOmega[0].clone();
    let omega = sigmaOmega[1].clone();

    //sigma = sigma.multiply(knownErrors);

    let mut errorLocations = findErrorLocations(sigma.clone(), field)?;
    let errorMagnitudes = findErrorMagnitudes(omega, sigma, &mut errorLocations, field);

    for i in 0..errorLocations.len() {
        // for (int i = 0; i < errorLocations.length; i++) {
        let position = received.len() as isize - 1 - field.log(errorLocations[i])? as isize;
        if position < 0 {
            return Err(Exceptions::checksumWith(file!()));
        }
        received[position as usize] =
            field.subtract(received[position as usize], errorMagnitudes[i]);
    }

    Ok(errorLocations.len())
}

fn runEuclideanAlgorithm(
    a: Rc<ModulusPoly>,
    b: Rc<ModulusPoly>,
    R: u32,
    field: &'static ModulusGF,
) -> Result<[Rc<ModulusPoly>; 2]> {
    // Assume a's degree is >= b's
    let mut a = a;
    let mut b = b;
    if a.getDegree() < b.getDegree() {
        std::mem::swap(&mut a, &mut b);
    }

    let mut rLast = a;
    let mut r = b;
    let mut tLast = ModulusPoly::getZero(field);
    let mut t = ModulusPoly::getOne(field);

    // Run Euclidean algorithm until r's degree is less than R/2
    while r.getDegree() >= R / 2 {
        let rLastLast = rLast.clone();
        let tLastLast = tLast.clone();
        rLast = r;
        tLast = t;

        // Divide rLastLast by rLast, with quotient in q and remainder in r
        if rLast.isZero() {
            // Oops, Euclidean algorithm already terminated?
            return Err(Exceptions::checksumWith(file!()));
        }
        r = rLastLast;
        let mut q = ModulusPoly::getZero(field); //field.getZero();
        let denominatorLeadingTerm = rLast.getCoefficient(rLast.getDegree() as usize);
        let dltInverse = field.inverse(denominatorLeadingTerm)?;
        while r.getDegree() >= rLast.getDegree() && !r.isZero() {
            let degreeDiff = r.getDegree() - rLast.getDegree();
            let scale = field.multiply(r.getCoefficient(r.getDegree() as usize), dltInverse);
            q = q.add(ModulusPoly::buildMonomial(
                field,
                degreeDiff as usize,
                scale,
            ))?;
            r = r.subtract(rLast.multiplyByMonomial(degreeDiff as usize, scale))?;
        }

        t = q.multiply(tLast.clone())?.subtract(tLastLast)?.negative();
    }

    let sigmaTildeAtZero = t.getCoefficient(0);
    if sigmaTildeAtZero == 0 {
        return Err(Exceptions::checksumWith(file!()));
    }

    let inverse = field.inverse(sigmaTildeAtZero)?;
    let sigma = t.multiplyByScaler(inverse);
    let omega = r.multiplyByScaler(inverse);

    Ok([sigma, omega])
}

fn findErrorLocations(errorLocator: Rc<ModulusPoly>, field: &ModulusGF) -> Result<Vec<u32>> {
    // This is a direct application of Chien's search
    let numErrors = errorLocator.getDegree();
    let mut result = vec![0u32; numErrors as usize];
    let mut e = 0;
    let mut i = 1;
    while i < field.getSize() && e < numErrors {
        // for (int i = 1; i < PDF417_GF.getSize() && e < numErrors; i++) {
        if errorLocator.evaluateAt(i) == 0 {
            result[e as usize] = field.inverse(i)?;
            e += 1;
        }
        i += 1;
    }
    if e != numErrors {
        return Err(Exceptions::checksumWith(file!()));
    }
    Ok(result)
}

fn findErrorMagnitudes(
    errorEvaluator: Rc<ModulusPoly>,
    errorLocator: Rc<ModulusPoly>,
    errorLocations: &mut [u32],
    field: &'static ModulusGF,
) -> Vec<u32> {
    let errorLocatorDegree = errorLocator.getDegree();
    if errorLocatorDegree < 1 {
        return vec![0; 0];
    }
    let mut formalDerivativeCoefficients = vec![0u32; errorLocatorDegree as usize];
    for i in 1..=errorLocatorDegree {
        // for (int i = 1; i <= errorLocatorDegree; i++) {
        formalDerivativeCoefficients[errorLocatorDegree as usize - i as usize] =
            field.multiply(i, errorLocator.getCoefficient(i as usize));
    }
    let formalDerivative =
        ModulusPoly::new(field, formalDerivativeCoefficients).expect("should generate good poly");

    // This is directly applying Forney's Formula
    let s = errorLocations.len();
    let mut result = vec![0u32; s];
    for i in 0..s {
        // for (int i = 0; i < s; i++) {
        let xiInverse = field.inverse(errorLocations[i]).expect("must invert");
        let numerator = field.subtract(0, errorEvaluator.evaluateAt(xiInverse));
        let denominator = field
            .inverse(formalDerivative.evaluateAt(xiInverse))
            .expect("must invert");
        result[i] = field.multiply(numerator, denominator);
    }

    result
}
