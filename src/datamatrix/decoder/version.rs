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

use core::fmt;
use once_cell::sync::Lazy;

use crate::common::Result;
use crate::Exceptions;

static VERSIONS: Lazy<Box<[Version]>> = Lazy::new(Version::buildVersions);

pub type VersionRef = &'static Version;

/**
 * The Version object encapsulates attributes about a particular
 * size Data Matrix Code.
 *
 * @author bbrown@google.com (Brian Brown)
 */
pub struct Version {
    versionNumber: u32,
    symbolSizeRows: u32,
    symbolSizeColumns: u32,
    dataRegionSizeRows: u32,
    dataRegionSizeColumns: u32,
    ecBlocks: ECBlocks,
    totalCodewords: u32,
}
impl Version {
    pub fn new(
        versionNumber: u32,
        symbolSizeRows: u32,
        symbolSizeColumns: u32,
        dataRegionSizeRows: u32,
        dataRegionSizeColumns: u32,
        ecBlocks: ECBlocks,
    ) -> Self {
        Self {
            versionNumber,
            symbolSizeRows,
            symbolSizeColumns,
            dataRegionSizeRows,
            dataRegionSizeColumns,
            totalCodewords: {
                let ecCodewords = &ecBlocks.getECCodewords();
                let ecbArray = ecBlocks.getECBlocks();
                ecbArray.iter().fold(0, |acc, ecBlock| {
                    acc + ecBlock.getCount() * (ecBlock.getDataCodewords() + ecCodewords)
                })
            },
            ecBlocks,
        }
    }

    pub fn getVersionNumber(&self) -> u32 {
        self.versionNumber
    }

    pub fn getSymbolSizeRows(&self) -> u32 {
        self.symbolSizeRows
    }

    pub fn getSymbolSizeColumns(&self) -> u32 {
        self.symbolSizeColumns
    }

    pub fn getDataRegionSizeRows(&self) -> u32 {
        self.dataRegionSizeRows
    }

    pub fn getDataRegionSizeColumns(&self) -> u32 {
        self.dataRegionSizeColumns
    }

    pub fn getTotalCodewords(&self) -> u32 {
        self.totalCodewords
    }

    pub fn getECBlocks(&self) -> &ECBlocks {
        &self.ecBlocks
    }

    /**
     * <p>Deduces version information from Data Matrix dimensions.</p>
     *
     * @param numRows Number of rows in modules
     * @param numColumns Number of columns in modules
     * @return Version for a Data Matrix Code of those dimensions
     * @throws FormatException if dimensions do correspond to a valid Data Matrix size
     */
    pub fn getVersionForDimensions(numRows: u32, numColumns: u32) -> Result<&'static Version> {
        if (numRows & 0x01) != 0 || (numColumns & 0x01) != 0 {
            return Err(Exceptions::FORMAT);
        }

        for version in VERSIONS.iter() {
            if version.symbolSizeRows == numRows && version.symbolSizeColumns == numColumns {
                return Ok(version);
            }
        }

        Err(Exceptions::FORMAT)
    }

    /**
     * See ISO 16022:2006 5.5.1 Table 7
     */
    fn buildVersions() -> Box<[Version]> {
        Box::new([
            Version::new(1, 10, 10, 8, 8, ECBlocks::new(5, ECB::new(1, 3))),
            Version::new(2, 12, 12, 10, 10, ECBlocks::new(7, ECB::new(1, 5))),
            Version::new(3, 14, 14, 12, 12, ECBlocks::new(10, ECB::new(1, 8))),
            Version::new(4, 16, 16, 14, 14, ECBlocks::new(12, ECB::new(1, 12))),
            Version::new(5, 18, 18, 16, 16, ECBlocks::new(14, ECB::new(1, 18))),
            Version::new(6, 20, 20, 18, 18, ECBlocks::new(18, ECB::new(1, 22))),
            Version::new(7, 22, 22, 20, 20, ECBlocks::new(20, ECB::new(1, 30))),
            Version::new(8, 24, 24, 22, 22, ECBlocks::new(24, ECB::new(1, 36))),
            Version::new(9, 26, 26, 24, 24, ECBlocks::new(28, ECB::new(1, 44))),
            Version::new(10, 32, 32, 14, 14, ECBlocks::new(36, ECB::new(1, 62))),
            Version::new(11, 36, 36, 16, 16, ECBlocks::new(42, ECB::new(1, 86))),
            Version::new(12, 40, 40, 18, 18, ECBlocks::new(48, ECB::new(1, 114))),
            Version::new(13, 44, 44, 20, 20, ECBlocks::new(56, ECB::new(1, 144))),
            Version::new(14, 48, 48, 22, 22, ECBlocks::new(68, ECB::new(1, 174))),
            Version::new(15, 52, 52, 24, 24, ECBlocks::new(42, ECB::new(2, 102))),
            Version::new(16, 64, 64, 14, 14, ECBlocks::new(56, ECB::new(2, 140))),
            Version::new(17, 72, 72, 16, 16, ECBlocks::new(36, ECB::new(4, 92))),
            Version::new(18, 80, 80, 18, 18, ECBlocks::new(48, ECB::new(4, 114))),
            Version::new(19, 88, 88, 20, 20, ECBlocks::new(56, ECB::new(4, 144))),
            Version::new(20, 96, 96, 22, 22, ECBlocks::new(68, ECB::new(4, 174))),
            Version::new(21, 104, 104, 24, 24, ECBlocks::new(56, ECB::new(6, 136))),
            Version::new(22, 120, 120, 18, 18, ECBlocks::new(68, ECB::new(6, 175))),
            Version::new(23, 132, 132, 20, 20, ECBlocks::new(62, ECB::new(8, 163))),
            Version::new(
                24,
                144,
                144,
                22,
                22,
                ECBlocks::new2(62, ECB::new(8, 156), ECB::new(2, 155)),
            ),
            Version::new(25, 8, 18, 6, 16, ECBlocks::new(7, ECB::new(1, 5))),
            Version::new(26, 8, 32, 6, 14, ECBlocks::new(11, ECB::new(1, 10))),
            Version::new(27, 12, 26, 10, 24, ECBlocks::new(14, ECB::new(1, 16))),
            Version::new(28, 12, 36, 10, 16, ECBlocks::new(18, ECB::new(1, 22))),
            Version::new(29, 16, 36, 14, 16, ECBlocks::new(24, ECB::new(1, 32))),
            Version::new(30, 16, 48, 14, 22, ECBlocks::new(28, ECB::new(1, 49))),
            // extended forms as specified in
            // ISO 21471:2020 (DMRE) 5.5.1 Table 7
            Version::new(31, 8, 48, 6, 22, ECBlocks::new(15, ECB::new(1, 18))),
            Version::new(32, 8, 64, 6, 14, ECBlocks::new(18, ECB::new(1, 24))),
            Version::new(33, 8, 80, 6, 18, ECBlocks::new(22, ECB::new(1, 32))),
            Version::new(34, 8, 96, 6, 22, ECBlocks::new(28, ECB::new(1, 38))),
            Version::new(35, 8, 120, 6, 18, ECBlocks::new(32, ECB::new(1, 49))),
            Version::new(36, 8, 144, 6, 22, ECBlocks::new(36, ECB::new(1, 63))),
            Version::new(37, 12, 64, 10, 14, ECBlocks::new(27, ECB::new(1, 43))),
            Version::new(38, 12, 88, 10, 20, ECBlocks::new(36, ECB::new(1, 64))),
            Version::new(39, 16, 64, 14, 14, ECBlocks::new(36, ECB::new(1, 62))),
            Version::new(40, 20, 36, 18, 16, ECBlocks::new(28, ECB::new(1, 44))),
            Version::new(41, 20, 44, 18, 20, ECBlocks::new(34, ECB::new(1, 56))),
            Version::new(42, 20, 64, 18, 14, ECBlocks::new(42, ECB::new(1, 84))),
            Version::new(43, 22, 48, 20, 22, ECBlocks::new(38, ECB::new(1, 72))),
            Version::new(44, 24, 48, 22, 22, ECBlocks::new(41, ECB::new(1, 80))),
            Version::new(45, 24, 64, 22, 14, ECBlocks::new(46, ECB::new(1, 108))),
            Version::new(46, 26, 40, 24, 18, ECBlocks::new(38, ECB::new(1, 70))),
            Version::new(47, 26, 48, 24, 22, ECBlocks::new(42, ECB::new(1, 90))),
            Version::new(48, 26, 64, 24, 14, ECBlocks::new(50, ECB::new(1, 118))),
        ])
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.versionNumber)
    }
}

/**
 * <p>Encapsulates a set of error-correction blocks in one symbol version. Most versions will
 * use blocks of differing sizes within one version, so, this encapsulates the parameters for
 * each set of blocks. It also holds the number of error-correction codewords per block since it
 * will be the same across all blocks within one version.</p>
 */
pub struct ECBlocks {
    ecCodewords: u32,
    ecBlocks: Box<[ECB]>,
}
impl ECBlocks {
    pub fn new(ecCodewords: u32, ecBlocks: ECB) -> Self {
        Self {
            ecCodewords,
            ecBlocks: Box::new([ecBlocks]),
        }
    }

    pub fn new2(ecCodewords: u32, ecBlocks1: ECB, ecBlocks2: ECB) -> Self {
        Self {
            ecCodewords,
            ecBlocks: Box::new([ecBlocks1, ecBlocks2]),
        }
    }

    pub fn getECCodewords(&self) -> u32 {
        self.ecCodewords
    }

    pub fn getECBlocks(&self) -> &[ECB] {
        &self.ecBlocks
    }
}

/**
 * <p>Encapsulates the parameters for one error-correction block in one symbol version.
 * This includes the number of data codewords, and the number of times a block with these
 * parameters is used consecutively in the Data Matrix code version's format.</p>
 */
pub struct ECB {
    count: u32,
    dataCodewords: u32,
}
impl ECB {
    pub const fn new(count: u32, dataCodewords: u32) -> Self {
        Self {
            count,
            dataCodewords,
        }
    }

    pub const fn getCount(&self) -> u32 {
        self.count
    }

    pub const fn getDataCodewords(&self) -> u32 {
        self.dataCodewords
    }
}
