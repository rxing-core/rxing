/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
* Copyright 2023 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use num::Integer;

use crate::common::{BitMatrix, Result};
use crate::qrcode::cpp_port::Type;
use crate::qrcode::decoder::{
    Version, VersionRef, MICRO_VERSIONS, MODEL1_VERSIONS, RMQR_VERSIONS, VERSIONS,
    VERSION_DECODE_INFO,
};
use crate::{point, Exceptions, PointI};

const dimsVersionRMQR: [PointI; 32] = [
    point(43, 7),
    point(59, 7),
    point(77, 7),
    point(99, 7),
    point(139, 7),
    point(43, 9),
    point(59, 9),
    point(77, 9),
    point(99, 9),
    point(139, 9),
    point(27, 11),
    point(43, 11),
    point(59, 11),
    point(77, 11),
    point(99, 11),
    point(139, 11),
    point(27, 13),
    point(43, 13),
    point(59, 13),
    point(77, 13),
    point(99, 13),
    point(139, 13),
    point(43, 15),
    point(59, 15),
    point(77, 15),
    point(99, 15),
    point(139, 15),
    point(43, 17),
    point(59, 17),
    point(77, 17),
    point(99, 17),
    point(139, 17),
];

impl Version {
    pub fn Model1(version_number: u32) -> Result<VersionRef> {
        if !(1..=14).contains(&version_number) {
            Err(Exceptions::ILLEGAL_ARGUMENT)
        } else {
            Ok(&MODEL1_VERSIONS[version_number as usize - 1])
        }
    }

    pub fn Model2(version_number: u32) -> Result<VersionRef> {
        if !(1..=40).contains(&version_number) {
            Err(Exceptions::ILLEGAL_ARGUMENT)
        } else {
            Ok(&VERSIONS[version_number as usize - 1])
        }
    }

    pub fn Micro(version_number: u32) -> Result<VersionRef> {
        if !(1..=4).contains(&version_number) {
            Err(Exceptions::ILLEGAL_ARGUMENT)
        } else {
            Ok(&MICRO_VERSIONS[version_number as usize - 1])
        }
    }

    pub fn rMQR(version_number: u32) -> Result<VersionRef> {
        let version_number = version_number as usize;
        if (version_number < 1 || version_number > (RMQR_VERSIONS.len())) {
            Err(Exceptions::ILLEGAL_ARGUMENT)
        } else {
            Ok(&RMQR_VERSIONS[version_number as usize - 1])
        }
    }

    pub const fn DimensionOfVersion(version: u32, is_micro: bool) -> u32 {
        Self::DimensionOffset(is_micro) + Self::DimensionStep(is_micro) * version
    }

    pub const fn DimensionOffset(is_micro: bool) -> u32 {
        match is_micro {
            true => 9,
            false => 17,
        }
        // return std::array{17, 9}[isMicro];
    }

    pub const fn DimensionStep(is_micro: bool) -> u32 {
        match is_micro {
            true => 2,
            false => 4,
        }
        // return std::array{4, 2}[isMicro];
    }
    pub fn DecodeVersionInformation(versionBitsA: i32, versionBitsB: i32) -> Result<VersionRef> {
        let mut bestDifference = u32::MAX;
        let mut bestVersion = 0;
        for (i, targetVersion) in VERSION_DECODE_INFO.into_iter().enumerate() {
            for bits in [versionBitsA, versionBitsB] {
                // for (int bits : {versionBitsA, versionBitsB}) {
                let bitsDifference = ((bits as u32) ^ targetVersion).count_ones(); //BitHacks::CountBitsSet(bits ^ targetVersion);
                if bitsDifference < bestDifference {
                    bestVersion = i + 7;
                    bestDifference = bitsDifference;
                }
            }
            if bestDifference == 0 {
                break;
            }
        }
        // We can tolerate up to 3 bits of error since no two version info codewords will
        // differ in less than 8 bits.
        if bestDifference <= 3 {
            return Self::getVersionForNumber(bestVersion as u32);
        }
        // If we didn't find a close enough match, fail
        Err(Exceptions::ILLEGAL_STATE)
    }

    pub const fn isMicro(&self) -> bool {
        Type::const_eq(self.qr_type, Type::Micro)
    }

    pub const fn isModel1(&self) -> bool {
        Type::const_eq(self.qr_type, Type::Model1)
    }

    pub const fn isModel2(&self) -> bool {
        Type::const_eq(self.qr_type, Type::Model2)
    }

    pub const fn isRMQR(&self) -> bool {
        Type::const_eq(self.qr_type, Type::RectMicro)
    }

    pub fn HasMicroSize(bitMatrix: &BitMatrix) -> bool {
        let size = bitMatrix.height();
        size == bitMatrix.width() && size >= 11 && size <= 17 && (size % 2) == 1
    }

    pub fn HasRMQRSize(bitMatrix: &BitMatrix) -> bool {
        Self::getVersionRMQR(bitMatrix) != -1
    }

    pub fn HasValidSize(bitMatrix: &BitMatrix) -> bool {
        let size = bitMatrix.height();
        if bitMatrix.width() != size {
            Self::HasRMQRSize(bitMatrix)
        } else {
            Self::HasMicroSize(bitMatrix) || ((21..=177).contains(&size) && (size % 4) == 1)
        }
    }

    pub fn Number(bitMatrix: &BitMatrix) -> u32 {
        if bitMatrix.width() != bitMatrix.height() {
            Self::getVersionRMQR(bitMatrix) as u32 + 1
        } else if !Self::HasValidSize(bitMatrix) {
            0
        } else {
            let isMicro = Self::HasMicroSize(bitMatrix);
            (bitMatrix.height() - Self::DimensionOffset(isMicro)) / Self::DimensionStep(isMicro)
        }
    }

    pub fn DimensionOfVersionRMQR(version_number: u32) -> PointI {
        if version_number < 1 || version_number as usize > dimsVersionRMQR.len() {
            point(0, 0)
        } else {
            dimsVersionRMQR[version_number as usize - 1]
        }
    }

    fn getVersionRMQR(bitMatrix: &BitMatrix) -> i32 {
        let width = bitMatrix.width() as i32;
        let height = bitMatrix.height() as i32;
        if width != height
            && width.is_odd()
            && height.is_odd()
            && width >= 27
            && width <= 139
            && height >= 7
            && height <= 17
        {
            for i in 0..dimsVersionRMQR.len() {
                // for (int i = 0; i < Size(dimsVersionRMQR); i++){
                if width == dimsVersionRMQR[i].x && height == dimsVersionRMQR[i].y {
                    return i as i32;
                }
            }
        }
        return -1;
    }
}
