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

fn CheckRMQRVersion(version: VersionRef, number: u32) {
    assert_eq!(number, version.getVersionNumber());
    assert_eq!(
        Version::DimensionOfVersionRMQR(number).x == 27,
        version.getAlignmentPatternCenters().is_empty()
    );
}

#[test]
fn RMQRVersionForNumber() {
    let version = Version::rMQR(0);
    assert!(version.is_err(), "There is version with number 0");

    for i in 1..=32 {
        // for (int i = 1; i <= 32; i++) {
        CheckRMQRVersion(Version::rMQR(i).expect("version {i} should exist"), i);
    }
}

#[test]
fn RMQRFunctionPattern1() {
    {
        let expected = BitMatrix::parse_strings(
            r"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
XXXXXXXXXXXX        XXX            XXXXXXXX
XXXXXXXXXXXX        XXX            XXXXXXXX
XXXXXXXXXXXX         X             XXXXXXXX
XXXXXXXXXXX         XXX            XXXXXXXX
XXXXXXXXXXX         XXX            XXXXXXXX
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
",
            "X",
            " ",
        )
        .unwrap();
        let version = Version::rMQR(1).unwrap(); // R7x43
        let functionPattern = version.buildFunctionPattern().unwrap();
        assert_eq!(expected, functionPattern);
    }
    {
        let expected = BitMatrix::parse_strings(
            r"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
XXXXXXXXXXXX        XXX                  XX
XXXXXXXXXXXX        XXX                   X
XXXXXXXXXXXX         X             XXXXXX X
XXXXXXXXXXX          X             XXXXXXXX
XXXXXXXXXXX          X             XXXXXXXX
XXXXXXXX            XXX            XXXXXXXX
XXXXXXXX            XXX            XXXXXXXX
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
",
            "X",
            " ",
        )
        .unwrap();
        let version = Version::rMQR(6).unwrap(); // R9x43
        let functionPattern = version.buildFunctionPattern().unwrap();
        assert_eq!(expected, functionPattern);
    }
    {
        let expected = BitMatrix::parse_strings(
            r"XXXXXXXXXXXXXXXXXXXXXXXXXXX
XXXXXXXXXXXX             XX
XXXXXXXXXXXX              X
XXXXXXXXXXXX              X
XXXXXXXXXXX               X
XXXXXXXXXXX        XXXXXX X
XXXXXXXX           XXXXXXXX
XXXXXXXX           XXXXXXXX
X                  XXXXXXXX
XX                 XXXXXXXX
XXXXXXXXXXXXXXXXXXXXXXXXXXX
",
            "X",
            " ",
        )
        .unwrap();
        let version = Version::rMQR(11).unwrap(); // R11x27
        let functionPattern = version.buildFunctionPattern().unwrap();
        assert_eq!(expected, functionPattern);
    }
    {
        let expected = BitMatrix::parse_strings(
            r"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
XXXXXXXXXXXX        XXX                  XX
XXXXXXXXXXXX        XXX                   X
XXXXXXXXXXXX         X                    X
XXXXXXXXXXX          X                    X
XXXXXXXXXXX          X             XXXXXX X
XXXXXXXX             X             XXXXXXXX
XXXXXXXX             X             XXXXXXXX
X                   XXX            XXXXXXXX
XX                  XXX            XXXXXXXX
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
",
            "X",
            " ",
        )
        .unwrap();
        let version = Version::rMQR(12).unwrap(); // R11x43
        let functionPattern = version.buildFunctionPattern().unwrap();
        assert_eq!(expected, functionPattern);
    }
    {
        let expected = BitMatrix::parse_strings(
            "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
XXXXXXXXXXXX      XXX                 XXX                XX
XXXXXXXXXXXX      XXX                 XXX                 X
XXXXXXXXXXXX       X                   X                  X
XXXXXXXXXXX        X                   X                  X
XXXXXXXXXXX        X                   X           XXXXXX X
XXXXXXXX           X                   X           XXXXXXXX
XXXXXXXX           X                   X           XXXXXXXX
X                 XXX                 XXX          XXXXXXXX
XX                XXX                 XXX          XXXXXXXX
XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
",
            "X",
            " ",
        )
        .unwrap();
        let version = Version::rMQR(13).unwrap(); // R11x59
        let functionPattern = version.buildFunctionPattern().unwrap();
        assert_eq!(expected, functionPattern);
    }
}
