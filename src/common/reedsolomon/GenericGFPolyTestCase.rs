/*
 * Copyright 2018 ZXing authors
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

//import org.junit.Assert;
//import org.junit.Test;

use std::rc::Rc;

use super::{GenericGF, GenericGFPoly};

/**
 * Tests {@link GenericGFPoly}.
 */

//const FIELD: GenericGF = super::QR_CODE_FIELD_256;

#[test]
fn testPolynomialString() {
    let FIELD = Rc::new(super::get_predefined_genericgf(super::PredefinedGenericGF::QrCodeField256));
    let fz = super::GenericGFPoly::new(FIELD.clone(), &vec![0; 1]).unwrap();

    assert_eq!("0", fz.getZero().to_string());
    let n1mono = GenericGF::buildMonomial(FIELD.clone(), 0, -1);
    assert_eq!("-1", n1mono.to_string());
    let p = GenericGFPoly::new(FIELD.clone(), &vec![3, 0, -2, 1, 1]).unwrap();
    assert_eq!("a^25x^4 - ax^2 + x + 1", p.to_string());
    let p = GenericGFPoly::new(FIELD.clone(), &vec![3]).unwrap();
    assert_eq!("a^25", p.to_string());
}

#[test]
fn testZero() {
    let FIELD = Rc::new(super::get_predefined_genericgf(super::PredefinedGenericGF::QrCodeField256));
    let fz = super::GenericGFPoly::new(FIELD.clone(), &vec![0; 1]).unwrap();

    assert_eq!(fz.getZero(), GenericGF::buildMonomial(FIELD.clone(), 1, 0));
    assert_eq!(
        fz.getZero(),
        GenericGF::buildMonomial(FIELD.clone(), 1, 2).multiply_with_scalar(0)
    );
}

#[test]
fn testEvaluate() {
    let FIELD = Rc::new(super::get_predefined_genericgf(super::PredefinedGenericGF::QrCodeField256));

    assert_eq!(3, GenericGF::buildMonomial(FIELD.clone(), 0, 3).evaluateAt(0));
}
