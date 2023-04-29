/*
* Copyright 2017 Huy Cuong Nguyen
* Copyright 2007 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::{common::BitMatrix, qrcode::cpp_port::data_mask::GetMaskedBit};

#[test]
fn Mask0() {
    TestMaskAcrossDimensions(0, |i, j| (i + j) % 2 == 0);
}

#[test]
fn Mask1() {
    TestMaskAcrossDimensions(1, |i, _| i % 2 == 0);
}

#[test]
fn Mask2() {
    TestMaskAcrossDimensions(2, |_, j| j % 3 == 0);
}

#[test]
fn Mask3() {
    TestMaskAcrossDimensions(3, |i, j| (i + j) % 3 == 0);
}

#[test]
fn Mask4() {
    TestMaskAcrossDimensions(4, |i, j| (i / 2 + j / 3) % 2 == 0);
}

#[test]
fn Mask5() {
    TestMaskAcrossDimensions(5, |i, j| (i * j) % 2 + (i * j) % 3 == 0);
}

#[test]
fn Mask6() {
    TestMaskAcrossDimensions(6, |i, j| ((i * j) % 2 + (i * j) % 3) % 2 == 0);
}

#[test]
fn Mask7() {
    TestMaskAcrossDimensions(7, |i, j| ((i + j) % 2 + (i * j) % 3) % 2 == 0);
}

#[test]
fn MicroMask0() {
    TestMicroMaskAcrossDimensions(0, |i, _| i % 2 == 0);
}

#[test]
fn MicroMask1() {
    TestMicroMaskAcrossDimensions(1, |i, j| (i / 2 + j / 3) % 2 == 0);
}

#[test]
fn MicroMask2() {
    TestMicroMaskAcrossDimensions(2, |i, j| ((i * j) % 2 + (i * j) % 3) % 2 == 0);
}

#[test]
fn MicroMask3() {
    TestMicroMaskAcrossDimensions(3, |i, j| ((i + j) % 2 + (i * j) % 3) % 2 == 0);
}

fn TestMaskAcrossDimensionsImpl<F>(
    maskIndex: u32,
    isMicro: bool,
    versionMax: u32,
    dimensionStart: u32,
    dimensionStep: u32,
    condition: F,
) where
    F: Fn(u32, u32) -> bool,
{
    for version in 1..=versionMax {
        // for (int version = 1; version <= versionMax; version++) {
        let dimension = dimensionStart + dimensionStep * version;
        let bits = BitMatrix::with_single_dimension(dimension).expect("couldn't create bitmatrix");

        for i in 0..dimension {
            // for (int i = 0; i < dimension; i++)
            for j in 0..dimension {
                // for (int j = 0; j < dimension; j++)
                assert_eq!(
                    GetMaskedBit(&bits, j, i, maskIndex, Some(isMicro))
                        .expect("could not get mask bit"),
                    condition(i, j),
                    "({i},{j})"
                );
            }
        }
    }
}

fn TestMaskAcrossDimensions<F>(maskIndex: u32, condition: F)
where
    F: Fn(u32, u32) -> bool,
{
    TestMaskAcrossDimensionsImpl(maskIndex, false, 40, 17, 4, condition);
}

fn TestMicroMaskAcrossDimensions<F>(maskIndex: u32, condition: F)
where
    F: Fn(u32, u32) -> bool,
{
    TestMaskAcrossDimensionsImpl(maskIndex, true, 4, 9, 2, condition);
}
