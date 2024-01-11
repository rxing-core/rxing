/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
* Copyright 2023 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use crate::common::{BitMatrix, Result};
use crate::qrcode::cpp_port::Type;
use crate::qrcode::decoder::{
    Version, VersionRef, MICRO_VERSIONS, MODEL1_VERSIONS, VERSIONS, VERSION_DECODE_INFO,
};
use crate::Exceptions;

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

    pub fn DimensionOfVersion(version: u32, is_micro: bool) -> u32 {
        Self::DimensionOffset(is_micro) + Self::DimensionStep(is_micro) * version
    }

    pub fn DimensionOffset(is_micro: bool) -> u32 {
        match is_micro {
            true => 9,
            false => 17,
        }
        // return std::array{17, 9}[isMicro];
    }

    pub fn DimensionStep(is_micro: bool) -> u32 {
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

    pub fn HasMicroSize(bitMatrix: &BitMatrix) -> bool {
        let size = bitMatrix.height();
        (11..=17).contains(&size) && (size % 2) == 1
    }

    pub fn HasValidSize(bitMatrix: &BitMatrix) -> bool {
        let size = bitMatrix.height();
        Self::HasMicroSize(bitMatrix) || ((21..=177).contains(&size) && (size % 4) == 1)
    }

    pub fn Number(bitMatrix: &BitMatrix) -> u32 {
        if !Self::HasValidSize(bitMatrix) {
            0
        } else {
            let isMicro = Self::HasMicroSize(bitMatrix);
            (bitMatrix.height() - Self::DimensionOffset(isMicro)) / Self::DimensionStep(isMicro)
        }
    }
}
