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

const RMQR_SIZES: [PointI; 32] = [
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
        if version_number < 1 || version_number > (RMQR_VERSIONS.len()) {
            Err(Exceptions::ILLEGAL_ARGUMENT)
        } else {
            Ok(&RMQR_VERSIONS[version_number - 1])
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

    pub fn SymbolSize(version: u32, qr_type: Type) -> PointI {
        let version = version as i32;

        let square = |s: i32| point(s, s);
        let valid = |v: i32, max: i32| v >= 1 && v <= max;

        match qr_type {
            Type::Model1 => {
                if valid(version, 32) {
                    square(17 + 4 * version)
                } else {
                    PointI::default()
                }
            }
            Type::Model2 => {
                if valid(version, 40) {
                    square(17 + 4 * version)
                } else {
                    PointI::default()
                }
            }
            Type::Micro => {
                if valid(version, 4) {
                    square(9 + 2 * version)
                } else {
                    PointI::default()
                }
            }
            Type::RectMicro => {
                if valid(version, 32) {
                    RMQR_SIZES[(version - 1) as usize]
                } else {
                    PointI::default()
                }
            }
        }
    }

    pub fn IsValidSize(size: PointI, qr_type: Type) -> bool {
        match qr_type {
            Type::Model1 => size.x == size.y && size.x >= 21 && size.x <= 145 && (size.x % 4 == 1),
            Type::Model2 => size.x == size.y && size.x >= 21 && size.x <= 177 && (size.x % 4 == 1),
            Type::Micro => size.x == size.y && size.x >= 11 && size.x <= 17 && (size.x % 2 == 1),
            Type::RectMicro => {
                size.x != size.y
                    && size.x.is_odd()
                    && size.y.is_odd()
                    && size.x >= 27
                    && size.x <= 139
                    && size.y >= 7
                    && size.y <= 17
                    && Self::IndexOf(&RMQR_SIZES, size) != -1
            }
        }
    }
    pub fn HasValidSizeType(bitMatrix: &BitMatrix, qr_type: Type) -> bool {
        Self::IsValidSize(
            point(bitMatrix.width() as i32, bitMatrix.height() as i32),
            qr_type,
        )
    }

    pub fn HasValidSize(matrix: &BitMatrix) -> bool {
        Self::HasValidSizeType(matrix, Type::Model1)
            || Self::HasValidSizeType(matrix, Type::Model2)
            || Self::HasValidSizeType(matrix, Type::Micro)
            || Self::HasValidSizeType(matrix, Type::RectMicro)
    }

    fn IndexOf(_points: &[PointI], search: PointI) -> i32 {
        RMQR_SIZES
            .iter()
            .position(|p| *p == search)
            .map(|x| x as i32)
            .unwrap_or(-1)
    }

    pub fn NumberPoint(size: PointI) -> u32 {
        if size.x != size.y {
            (Self::IndexOf(&RMQR_SIZES, size) + 1) as u32
        } else if Self::IsValidSize(size, Type::Model2) {
            ((size.x - 17) / 4) as u32
        } else if Self::IsValidSize(size, Type::Micro) {
            ((size.x - 9) / 2) as u32
        } else {
            0
        }
    }

    pub fn Number(bitMatrix: &BitMatrix) -> u32 {
        Self::NumberPoint(point(bitMatrix.width() as i32, bitMatrix.height() as i32))
    }
}
