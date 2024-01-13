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

use std::fmt;

use crate::{
    common::{BitMatrix, Result},
    qrcode::cpp_port::Type,
    Exceptions,
};

use super::{ErrorCorrectionLevel, FormatInformation};

use once_cell::sync::Lazy;

pub type VersionRef = &'static Version;

pub static VERSIONS: Lazy<Vec<Version>> = Lazy::new(Version::buildVersions);
pub static MICRO_VERSIONS: Lazy<Vec<Version>> = Lazy::new(Version::build_micro_versions);
pub static MODEL1_VERSIONS: Lazy<Vec<Version>> = Lazy::new(Version::build_model1_versions);
pub static RMQR_VERSIONS: Lazy<Vec<Version>> = Lazy::new(Version::build_rmqr_versions);

/**
 * See ISO 18004:2006 Annex D.
 * Element i represents the raw version bits that specify version i + 7
 */
pub const VERSION_DECODE_INFO: [u32; 34] = [
    0x07C94, 0x085BC, 0x09A99, 0x0A4D3, 0x0BBF6, 0x0C762, 0x0D847, 0x0E60D, 0x0F928, 0x10B78,
    0x1145D, 0x12A17, 0x13532, 0x149A6, 0x15683, 0x168C9, 0x177EC, 0x18EC4, 0x191E1, 0x1AFAB,
    0x1B08E, 0x1CC1A, 0x1D33F, 0x1ED75, 0x1F250, 0x209D5, 0x216F0, 0x228BA, 0x2379F, 0x24B0B,
    0x2542E, 0x26A64, 0x27541, 0x28C69,
];

// const VERSIONS: &'static[Version] = &Version::buildVersions();
/**
 * See ISO 18004:2006 Annex D
 *
 * @author Sean Owen
 */
#[derive(Debug)]
pub struct Version {
    //   private static final Version[] VERSIONS = buildVersions();
    versionNumber: u32,
    alignmentPatternCenters: Vec<u32>,
    ecBlocks: Vec<ECBlocks>,
    totalCodewords: u32,
    pub(crate) qr_type: Type,
}
impl Version {
    pub(super) fn new(
        versionNumber: u32,
        alignmentPatternCenters: Vec<u32>,
        ecBlocks: [ECBlocks; 4],
    ) -> Self {
        let mut total = 0;
        let ecCodewords = ecBlocks[1].getECCodewordsPerBlock();
        let ecbArray = ecBlocks[1].getECBlocks();
        // let mut i = 0;
        for ecb in ecbArray {
            // while i < ecbArray.len() {
            total += ecb.getCount() * (ecb.getDataCodewords() + ecCodewords);
            // i += 1;
        }

        Self {
            versionNumber,
            alignmentPatternCenters,
            ecBlocks: ecBlocks.to_vec(),
            totalCodewords: total,
            qr_type: if ecBlocks[0].getECCodewordsPerBlock() != 0 {
                Type::Model2
            } else {
                Type::RectMicro
            },
        }
    }

    pub(super) fn new_micro(versionNumber: u32, ecBlocks: Vec<ECBlocks>) -> Self {
        let mut total = 0;
        let ecCodewords = ecBlocks[0].getECCodewordsPerBlock();
        let ecbArray = ecBlocks[0].getECBlocks();
        let mut i = 0;
        while i < ecbArray.len() {
            total += ecbArray[i].getCount() * (ecbArray[i].getDataCodewords() + ecCodewords);
            i += 1;
        }

        Self {
            versionNumber,
            alignmentPatternCenters: Vec::default(),
            ecBlocks,
            totalCodewords: total,
            qr_type: Type::Micro,
        }
    }

    pub(super) fn new_model1(versionNumber: u32, ecBlocks: Vec<ECBlocks>) -> Self {
        let mut total = 0;
        let ecCodewords = ecBlocks[0].getECCodewordsPerBlock();
        let ecbArray = ecBlocks[0].getECBlocks();
        let mut i = 0;
        while i < ecbArray.len() {
            total += ecbArray[i].getCount() * (ecbArray[i].getDataCodewords() + ecCodewords);
            i += 1;
        }

        Self {
            versionNumber,
            alignmentPatternCenters: Vec::default(),
            ecBlocks,
            totalCodewords: total,
            qr_type: Type::Model1,
        }
    }

    pub const fn getVersionNumber(&self) -> u32 {
        self.versionNumber
    }

    pub fn getAlignmentPatternCenters(&self) -> &[u32] {
        &self.alignmentPatternCenters
    }

    pub fn getTotalCodewords(&self) -> u32 {
        self.totalCodewords
    }

    pub fn getDimensionForVersion(&self) -> u32 {
        Self::DimensionOfVersion(self.versionNumber, self.qr_type == Type::Micro)
        // 17 + 4 * self.versionNumber
    }

    pub fn getECBlocksForLevel(&self, ecLevel: ErrorCorrectionLevel) -> &ECBlocks {
        if ecLevel.get_ordinal() as usize >= self.ecBlocks.len() {
            return &self.ecBlocks[ecLevel.get_ordinal() as usize % self.ecBlocks.len()];
        }
        &self.ecBlocks[ecLevel.get_ordinal() as usize]
    }

    /**
     * <p>Deduces version information purely from QR Code dimensions.</p>
     *
     * @param dimension dimension in modules
     * @return Version for a QR Code of that dimension
     * @throws FormatException if dimension is not 1 mod 4
     */
    pub fn getProvisionalVersionForDimension(dimension: u32) -> Result<VersionRef> {
        if dimension % 4 != 1 {
            return Err(Exceptions::format_with("dimension incorrect"));
        }
        Self::getVersionForNumber((dimension - 17) / 4)
    }

    pub fn getVersionForNumber(versionNumber: u32) -> Result<VersionRef> {
        if !(1..=40).contains(&versionNumber) {
            return Err(Exceptions::illegal_argument_with("version out of spec"));
        }
        Ok(&VERSIONS[versionNumber as usize - 1])
    }

    pub fn decodeVersionInformation(versionBits: u32) -> Result<VersionRef> {
        let mut bestDifference = u32::MAX;
        let mut bestVersion = 0;
        for i in 0..VERSION_DECODE_INFO.len() as u32 {
            let targetVersion = VERSION_DECODE_INFO[i as usize];
            // Do the version info bits match exactly? done.
            if targetVersion == versionBits {
                return Self::getVersionForNumber(i + 7);
            }
            // Otherwise see if this is the closest to a real version info bit string
            // we have seen so far
            let bitsDifference = FormatInformation::numBitsDiffering(versionBits, targetVersion);
            if bitsDifference < bestDifference {
                bestVersion = i + 7;
                bestDifference = bitsDifference;
            }
        }
        // We can tolerate up to 3 bits of error since no two version info codewords will
        // differ in less than 8 bits.
        if bestDifference <= 3 {
            return Self::getVersionForNumber(bestVersion);
        }
        // If we didn't find a close enough match, fail
        Err(Exceptions::NOT_FOUND)
    }

    /**
     * See ISO 18004:2006 Annex E
     */
    pub fn buildFunctionPattern(&self) -> Result<BitMatrix> {
        if self.isRMQR() {
            let size = Version::SymbolSize(self.versionNumber, Type::RectMicro);
            let mut bitMatrix = BitMatrix::new(size.x as u32, size.y as u32)?;

            // Set edge timing patterns
            bitMatrix.setRegion(0, 0, size.x as u32, 1)?; // Top
            bitMatrix.setRegion(0, (size.y - 1) as u32, size.x as u32, 1)?; // Bottom
            bitMatrix.setRegion(0, 1, 1, (size.y - 2) as u32)?; // Left
            bitMatrix.setRegion((size.x - 1) as u32, 1, 1, (size.y - 2) as u32)?; // Right

            // Set vertical timing and alignment patterns
            let max = self.alignmentPatternCenters.len(); // Same as vertical timing column
            for x in 0..max {
                // for (size_t x = 0; x < max; ++x) {
                let cx = self.alignmentPatternCenters[x];
                bitMatrix.setRegion(cx - 1, 1, 3, 2)?; // Top alignment pattern
                bitMatrix.setRegion(cx - 1, (size.y - 3) as u32, 3, 2)?; // Bottom alignment pattern
                bitMatrix.setRegion(cx, 3, 1, (size.y - 6) as u32)?; // Vertical timing pattern
            }

            // Top left finder pattern + separator
            bitMatrix.setRegion(1, 1, 8 - 1, 8 - 1 - u32::from(size.y == 7))?; // R7 finder bottom flush with edge
                                                                                    // Top left format
            bitMatrix.setRegion(8, 1, 3, 5)?;
            bitMatrix.setRegion(11, 1, 1, 3)?;

            // Bottom right finder subpattern
            bitMatrix.setRegion(
                (size.x - 5) as u32,
                (size.y - 5) as u32,
                5 - 1,
                5 - 1,
            )?;
            // Bottom right format
            bitMatrix.setRegion((size.x - 8) as u32, (size.y - 6) as u32, 3, 5)?;
            bitMatrix.setRegion((size.x - 5) as u32, (size.y - 6) as u32, 3, 1)?;

            // Top right corner finder
            bitMatrix.set((size.x - 2) as u32, 1);
            if size.y > 9 {
                // Bottom left corner finder
                bitMatrix.set(1, (size.y - 2) as u32);
            }

            return Ok(bitMatrix);
        }

        let dimension = self.getDimensionForVersion();
        let mut bitMatrix = BitMatrix::with_single_dimension(dimension)?;

        // Top left finder pattern + separator + format
        bitMatrix.setRegion(0, 0, 9, 9)?;

        if self.qr_type != Type::Micro {
            // Top right finder pattern + separator + format
            bitMatrix.setRegion(dimension - 8, 0, 8, 9)?;
            // Bottom left finder pattern + separator + format
            bitMatrix.setRegion(0, dimension - 8, 9, 8)?;

            // Alignment patterns
            let max = self.alignmentPatternCenters.len();
            for x in 0..max {
                let i = self.alignmentPatternCenters[x] - 2;
                for y in 0..max {
                    if (x != 0 || (y != 0 && y != max - 1)) && (x != max - 1 || y != 0) {
                        bitMatrix.setRegion(self.alignmentPatternCenters[y] - 2, i, 5, 5)?;
                    }
                    // else no o alignment patterns near the three finder patterns
                }
            }

            // Vertical timing pattern
            bitMatrix.setRegion(6, 9, 1, dimension - 17)?;
            // Horizontal timing pattern
            bitMatrix.setRegion(9, 6, dimension - 17, 1)?;

            if self.versionNumber > 6 {
                // Version info, top right
                bitMatrix.setRegion(dimension - 11, 0, 3, 6)?;
                // Version info, bottom left
                bitMatrix.setRegion(0, dimension - 11, 6, 3)?;
            }
        } else {
            // Vertical timing pattern
            bitMatrix.setRegion(9, 0, dimension - 9, 1)?;

            // Horizontal timing pattern
            bitMatrix.setRegion(0, 9, 1, dimension - 9)?;
        }

        Ok(bitMatrix)
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
#[derive(Debug, Clone)]
pub struct ECBlocks {
    ecCodewordsPerBlock: u32,
    ecBlocks: Vec<ECB>,
}

impl ECBlocks {
    pub const fn new(ecCodewordsPerBlock: u32, ecBlocks: Vec<ECB>) -> Self {
        Self {
            ecCodewordsPerBlock,
            ecBlocks,
        }
    }

    pub const fn getECCodewordsPerBlock(&self) -> u32 {
        self.ecCodewordsPerBlock
    }

    pub fn getNumBlocks(&self) -> u32 {
        let mut total = 0;
        for ecBlock in &self.ecBlocks {
            total += ecBlock.getCount();
        }
        total
    }

    pub fn getTotalECCodewords(&self) -> u32 {
        self.ecCodewordsPerBlock * self.getNumBlocks()
    }

    pub fn getECBlocks(&self) -> &[ECB] {
        &self.ecBlocks
    }
}

/**
 * <p>Encapsulates the parameters for one error-correction block in one symbol version.
 * This includes the number of data codewords, and the number of times a block with these
 * parameters is used consecutively in the QR code version's format.</p>
 */
#[derive(Debug, Clone, Copy)]
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
