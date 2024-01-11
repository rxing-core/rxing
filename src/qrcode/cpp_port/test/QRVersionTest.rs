/*
* Copyright 2017 Huy Cuong Nguyen
* Copyright 2008 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::{
    common::BitMatrix,
    qrcode::decoder::{Version, VersionRef},
};

fn CheckVersion(version: VersionRef, number: u32, dimension: u32) {
    // assert_ne!(version, nullptr);
    assert_eq!(number, version.getVersionNumber());
    if number > 1 && version.isModel2() {
        assert!(!version.getAlignmentPatternCenters().is_empty());
    }
    assert_eq!(dimension, version.getDimensionForVersion());
}

fn DoTestVersion(expectedVersion: u32, mask: i32) {
    let version = Version::DecodeVersionInformation(mask, 0).expect("should exist");
    // assert_ne!(version, nullptr);
    assert_eq!(expectedVersion, version.getVersionNumber());
}

#[test]
fn VersionForNumber() {
    let version = Version::Model2(0);
    assert!(version.is_err(), "There is version with number 0");

    for i in 1..=40 {
        // for (int i = 1; i <= 40; i++) {
        CheckVersion(
            Version::Model2(i).expect("version number found"),
            i,
            4 * i + 17,
        );
    }
}

#[test]
fn GetProvisionalVersionForDimension() {
    for i in 1..=40 {
        // for (int i = 1; i <= 40; i++) {
        // assert_ne!(prov, nullptr);
        assert_eq!(
            i,
            Version::Number(
                &BitMatrix::with_single_dimension(4 * i + 17).expect("must create bitmatrix")
            )
        );
    }
}

#[test]
fn DecodeVersionInformation() {
    // Spot check
    DoTestVersion(7, 0x07C94);
    DoTestVersion(12, 0x0C762);
    DoTestVersion(17, 0x1145D);
    DoTestVersion(22, 0x168C9);
    DoTestVersion(27, 0x1B08E);
    DoTestVersion(32, 0x209D5);
}

#[test]
fn MicroVersionForNumber() {
    let version = Version::Micro(0);
    assert!(version.is_err(), "There is version with number 0");

    for i in 1..=4 {
        // for (int i = 1; i <= 4; i++) {
        CheckVersion(
            Version::Micro(i).unwrap_or_else(|_| panic!("version for {i} should exist")),
            i,
            2 * i + 9,
        );
    }
}

#[test]
fn GetProvisionalMicroVersionForDimension() {
    for i in 1..=4 {
        // for (int i = 1; i <= 4; i++) {
        assert_eq!(
            i,
            Version::Number(
                &BitMatrix::with_single_dimension(2 * i + 9).expect("must create bitmatrix")
            )
        );
    }
}

#[test]
fn FunctionPattern() {
    let testFinderPatternRegion = |bitMatrix: &BitMatrix| {
        for row in 0..9 {
            // for (int row = 0; row < 9; row++){
            for col in 0..9 {
                // for (int col = 0; col < 9; col++) {
                assert!(bitMatrix.get(col, row));
            }
        }
    };
    for i in 1..=4 {
        // for (int i = 1; i <= 4; i++) {
        let version = Version::Micro(i).expect("version must be found");
        let functionPattern = version
            .buildFunctionPattern()
            .expect("function pattern must be found");
        testFinderPatternRegion(&functionPattern);

        // Check timing pattern areas.
        let dimension = version.getDimensionForVersion();
        for row in dimension..functionPattern.height()
        // for (int row = dimension; row < functionPattern.height(); row++)
        {
            assert!(functionPattern.get(0, row));
        }
        for col in dimension..functionPattern.width()
        // for (int col = dimension; col < functionPattern.width(); col++)
        {
            assert!(functionPattern.get(col, 0));
        }
    }
}
