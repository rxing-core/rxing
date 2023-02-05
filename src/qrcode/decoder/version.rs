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

use crate::{common::BitMatrix, Exceptions};

use super::{ErrorCorrectionLevel, FormatInformation};

use once_cell::sync::Lazy;

pub type VersionRef = &'static Version;

static VERSIONS: Lazy<Vec<Version>> = Lazy::new(Version::buildVersions);

/**
 * See ISO 18004:2006 Annex D.
 * Element i represents the raw version bits that specify version i + 7
 */
const VERSION_DECODE_INFO: [u32; 34] = [
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
}
impl Version {
    fn new(versionNumber: u32, alignmentPatternCenters: Vec<u32>, ecBlocks: Vec<ECBlocks>) -> Self {
        let mut total = 0;
        let ecCodewords = ecBlocks[0].getECCodewordsPerBlock();
        let ecbArray = ecBlocks[0].getECBlocks();
        let mut i = 0;
        while i < ecbArray.len() {
            //    for ecBlock in ecbArray {
            // for (ECB ecBlock : ecbArray) {
            total += ecbArray[i].getCount() * (ecbArray[i].getDataCodewords() + ecCodewords);
            i += 1;
        }

        Self {
            versionNumber,
            alignmentPatternCenters,
            ecBlocks,
            totalCodewords: total,
        }
    }

    pub fn getVersionNumber(&self) -> u32 {
        self.versionNumber
    }

    pub fn getAlignmentPatternCenters(&self) -> &[u32] {
        &self.alignmentPatternCenters
    }

    pub fn getTotalCodewords(&self) -> u32 {
        self.totalCodewords
    }

    pub fn getDimensionForVersion(&self) -> u32 {
        17 + 4 * self.versionNumber
    }

    pub fn getECBlocksForLevel(&self, ecLevel: ErrorCorrectionLevel) -> &ECBlocks {
        &self.ecBlocks[ecLevel.get_ordinal() as usize]
    }

    /**
     * <p>Deduces version information purely from QR Code dimensions.</p>
     *
     * @param dimension dimension in modules
     * @return Version for a QR Code of that dimension
     * @throws FormatException if dimension is not 1 mod 4
     */
    pub fn getProvisionalVersionForDimension(
        dimension: u32,
    ) -> Result<&'static Version, Exceptions> {
        if dimension % 4 != 1 {
            return Err(Exceptions::FormatException(Some(
                "dimension incorrect".to_owned(),
            )));
        }
        Self::getVersionForNumber((dimension - 17) / 4)
        // try {
        //   return getVersionForNumber((dimension - 17) / 4);
        // } catch (IllegalArgumentException ignored) {
        //   throw FormatException.getFormatInstance();
        // }
    }

    pub fn getVersionForNumber(versionNumber: u32) -> Result<&'static Version, Exceptions> {
        if !(1..=40).contains(&versionNumber) {
            return Err(Exceptions::IllegalArgumentException(Some(
                "version out of spec".to_owned(),
            )));
        }
        Ok(&VERSIONS[versionNumber as usize - 1])
    }

    pub fn decodeVersionInformation(versionBits: u32) -> Result<&'static Version, Exceptions> {
        let mut bestDifference = u32::MAX;
        let mut bestVersion = 0;
        for i in 0..VERSION_DECODE_INFO.len() as u32 {
            // for (int i = 0; i < VERSION_DECODE_INFO.length; i++) {
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
        Err(Exceptions::NotFoundException(None))
    }

    /**
     * See ISO 18004:2006 Annex E
     */
    pub fn buildFunctionPattern(&self) -> Result<BitMatrix, Exceptions> {
        let dimension = self.getDimensionForVersion();
        let mut bitMatrix = BitMatrix::with_single_dimension(dimension)?;

        // Top left finder pattern + separator + format
        bitMatrix.setRegion(0, 0, 9, 9)?;
        // Top right finder pattern + separator + format
        bitMatrix.setRegion(dimension - 8, 0, 8, 9)?;
        // Bottom left finder pattern + separator + format
        bitMatrix.setRegion(0, dimension - 8, 9, 8)?;

        // Alignment patterns
        let max = self.alignmentPatternCenters.len();
        for x in 0..max {
            // for (int x = 0; x < max; x++) {
            let i = self.alignmentPatternCenters[x] - 2;
            for y in 0..max {
                //   for (int y = 0; y < max; y++) {
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

        Ok(bitMatrix)
    }

    /**
     * See ISO 18004:2006 6.5.1 Table 9
     */
    pub fn buildVersions() -> Vec<Version> {
        Vec::from([
            Version::new(
                1,
                Vec::from([]),
                Vec::from([
                    ECBlocks::new(7, Vec::from([ECB::new(1, 19)])),
                    ECBlocks::new(10, Vec::from([ECB::new(1, 16)])),
                    ECBlocks::new(13, Vec::from([ECB::new(1, 13)])),
                    ECBlocks::new(17, Vec::from([ECB::new(1, 9)])),
                ]),
            ),
            Version::new(
                2,
                Vec::from([6, 18]),
                Vec::from([
                    ECBlocks::new(10, Vec::from([ECB::new(1, 34)])),
                    ECBlocks::new(16, Vec::from([ECB::new(1, 28)])),
                    ECBlocks::new(22, Vec::from([ECB::new(1, 22)])),
                    ECBlocks::new(28, Vec::from([ECB::new(1, 16)])),
                ]),
            ),
            Version::new(
                3,
                Vec::from([6, 22]),
                Vec::from([
                    ECBlocks::new(15, Vec::from([ECB::new(1, 55)])),
                    ECBlocks::new(26, Vec::from([ECB::new(1, 44)])),
                    ECBlocks::new(18, Vec::from([ECB::new(2, 17)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 13)])),
                ]),
            ),
            Version::new(
                4,
                Vec::from([6, 26]),
                Vec::from([
                    ECBlocks::new(20, Vec::from([ECB::new(1, 80)])),
                    ECBlocks::new(18, Vec::from([ECB::new(2, 32)])),
                    ECBlocks::new(26, Vec::from([ECB::new(2, 24)])),
                    ECBlocks::new(16, Vec::from([ECB::new(4, 9)])),
                ]),
            ),
            Version::new(
                5,
                Vec::from([6, 30]),
                Vec::from([
                    ECBlocks::new(26, Vec::from([ECB::new(1, 108)])),
                    ECBlocks::new(24, Vec::from([ECB::new(2, 43)])),
                    ECBlocks::new(18, Vec::from([ECB::new(2, 15), ECB::new(2, 16)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 11), ECB::new(2, 12)])),
                ]),
            ),
            Version::new(
                6,
                Vec::from([6, 34]),
                Vec::from([
                    ECBlocks::new(18, Vec::from([ECB::new(2, 68)])),
                    ECBlocks::new(16, Vec::from([ECB::new(4, 27)])),
                    ECBlocks::new(24, Vec::from([ECB::new(4, 19)])),
                    ECBlocks::new(28, Vec::from([ECB::new(4, 15)])),
                ]),
            ),
            Version::new(
                7,
                Vec::from([6, 22, 38]),
                Vec::from([
                    ECBlocks::new(20, Vec::from([ECB::new(2, 78)])),
                    ECBlocks::new(18, Vec::from([ECB::new(4, 31)])),
                    ECBlocks::new(18, Vec::from([ECB::new(2, 14), ECB::new(4, 15)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 13), ECB::new(1, 14)])),
                ]),
            ),
            Version::new(
                8,
                Vec::from([6, 24, 42]),
                Vec::from([
                    ECBlocks::new(24, Vec::from([ECB::new(2, 97)])),
                    ECBlocks::new(22, Vec::from([ECB::new(2, 38), ECB::new(2, 39)])),
                    ECBlocks::new(22, Vec::from([ECB::new(4, 18), ECB::new(2, 19)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 14), ECB::new(2, 15)])),
                ]),
            ),
            Version::new(
                9,
                Vec::from([6, 26, 46]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(2, 116)])),
                    ECBlocks::new(22, Vec::from([ECB::new(3, 36), ECB::new(2, 37)])),
                    ECBlocks::new(20, Vec::from([ECB::new(4, 16), ECB::new(4, 17)])),
                    ECBlocks::new(24, Vec::from([ECB::new(4, 12), ECB::new(4, 13)])),
                ]),
            ),
            Version::new(
                10,
                Vec::from([6, 28, 50]),
                Vec::from([
                    ECBlocks::new(18, Vec::from([ECB::new(2, 68), ECB::new(2, 69)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 43), ECB::new(1, 44)])),
                    ECBlocks::new(24, Vec::from([ECB::new(6, 19), ECB::new(2, 20)])),
                    ECBlocks::new(28, Vec::from([ECB::new(6, 15), ECB::new(2, 16)])),
                ]),
            ),
            Version::new(
                11,
                Vec::from([6, 30, 54]),
                Vec::from([
                    ECBlocks::new(20, Vec::from([ECB::new(4, 81)])),
                    ECBlocks::new(30, Vec::from([ECB::new(1, 50), ECB::new(4, 51)])),
                    ECBlocks::new(28, Vec::from([ECB::new(4, 22), ECB::new(4, 23)])),
                    ECBlocks::new(24, Vec::from([ECB::new(3, 12), ECB::new(8, 13)])),
                ]),
            ),
            Version::new(
                12,
                Vec::from([6, 32, 58]),
                Vec::from([
                    ECBlocks::new(24, Vec::from([ECB::new(2, 92), ECB::new(2, 93)])),
                    ECBlocks::new(22, Vec::from([ECB::new(6, 36), ECB::new(2, 37)])),
                    ECBlocks::new(26, Vec::from([ECB::new(4, 20), ECB::new(6, 21)])),
                    ECBlocks::new(28, Vec::from([ECB::new(7, 14), ECB::new(4, 15)])),
                ]),
            ),
            Version::new(
                13,
                Vec::from([6, 34, 62]),
                Vec::from([
                    ECBlocks::new(26, Vec::from([ECB::new(4, 107)])),
                    ECBlocks::new(22, Vec::from([ECB::new(8, 37), ECB::new(1, 38)])),
                    ECBlocks::new(24, Vec::from([ECB::new(8, 20), ECB::new(4, 21)])),
                    ECBlocks::new(22, Vec::from([ECB::new(12, 11), ECB::new(4, 12)])),
                ]),
            ),
            Version::new(
                14,
                Vec::from([6, 26, 46, 66]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(3, 115), ECB::new(1, 116)])),
                    ECBlocks::new(24, Vec::from([ECB::new(4, 40), ECB::new(5, 41)])),
                    ECBlocks::new(20, Vec::from([ECB::new(11, 16), ECB::new(5, 17)])),
                    ECBlocks::new(24, Vec::from([ECB::new(11, 12), ECB::new(5, 13)])),
                ]),
            ),
            Version::new(
                15,
                Vec::from([6, 26, 48, 70]),
                Vec::from([
                    ECBlocks::new(22, Vec::from([ECB::new(5, 87), ECB::new(1, 88)])),
                    ECBlocks::new(24, Vec::from([ECB::new(5, 41), ECB::new(5, 42)])),
                    ECBlocks::new(30, Vec::from([ECB::new(5, 24), ECB::new(7, 25)])),
                    ECBlocks::new(24, Vec::from([ECB::new(11, 12), ECB::new(7, 13)])),
                ]),
            ),
            Version::new(
                16,
                Vec::from([6, 26, 50, 74]),
                Vec::from([
                    ECBlocks::new(24, Vec::from([ECB::new(5, 98), ECB::new(1, 99)])),
                    ECBlocks::new(28, Vec::from([ECB::new(7, 45), ECB::new(3, 46)])),
                    ECBlocks::new(24, Vec::from([ECB::new(15, 19), ECB::new(2, 20)])),
                    ECBlocks::new(30, Vec::from([ECB::new(3, 15), ECB::new(13, 16)])),
                ]),
            ),
            Version::new(
                17,
                Vec::from([6, 30, 54, 78]),
                Vec::from([
                    ECBlocks::new(28, Vec::from([ECB::new(1, 107), ECB::new(5, 108)])),
                    ECBlocks::new(28, Vec::from([ECB::new(10, 46), ECB::new(1, 47)])),
                    ECBlocks::new(28, Vec::from([ECB::new(1, 22), ECB::new(15, 23)])),
                    ECBlocks::new(28, Vec::from([ECB::new(2, 14), ECB::new(17, 15)])),
                ]),
            ),
            Version::new(
                18,
                Vec::from([6, 30, 56, 82]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(5, 120), ECB::new(1, 121)])),
                    ECBlocks::new(26, Vec::from([ECB::new(9, 43), ECB::new(4, 44)])),
                    ECBlocks::new(28, Vec::from([ECB::new(17, 22), ECB::new(1, 23)])),
                    ECBlocks::new(28, Vec::from([ECB::new(2, 14), ECB::new(19, 15)])),
                ]),
            ),
            Version::new(
                19,
                Vec::from([6, 30, 58, 86]),
                Vec::from([
                    ECBlocks::new(28, Vec::from([ECB::new(3, 113), ECB::new(4, 114)])),
                    ECBlocks::new(26, Vec::from([ECB::new(3, 44), ECB::new(11, 45)])),
                    ECBlocks::new(26, Vec::from([ECB::new(17, 21), ECB::new(4, 22)])),
                    ECBlocks::new(26, Vec::from([ECB::new(9, 13), ECB::new(16, 14)])),
                ]),
            ),
            Version::new(
                20,
                Vec::from([6, 34, 62, 90]),
                Vec::from([
                    ECBlocks::new(28, Vec::from([ECB::new(3, 107), ECB::new(5, 108)])),
                    ECBlocks::new(26, Vec::from([ECB::new(3, 41), ECB::new(13, 42)])),
                    ECBlocks::new(30, Vec::from([ECB::new(15, 24), ECB::new(5, 25)])),
                    ECBlocks::new(28, Vec::from([ECB::new(15, 15), ECB::new(10, 16)])),
                ]),
            ),
            Version::new(
                21,
                Vec::from([6, 28, 50, 72, 94]),
                Vec::from([
                    ECBlocks::new(28, Vec::from([ECB::new(4, 116), ECB::new(4, 117)])),
                    ECBlocks::new(26, Vec::from([ECB::new(17, 42)])),
                    ECBlocks::new(28, Vec::from([ECB::new(17, 22), ECB::new(6, 23)])),
                    ECBlocks::new(30, Vec::from([ECB::new(19, 16), ECB::new(6, 17)])),
                ]),
            ),
            Version::new(
                22,
                Vec::from([6, 26, 50, 74, 98]),
                Vec::from([
                    ECBlocks::new(28, Vec::from([ECB::new(2, 111), ECB::new(7, 112)])),
                    ECBlocks::new(28, Vec::from([ECB::new(17, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(7, 24), ECB::new(16, 25)])),
                    ECBlocks::new(24, Vec::from([ECB::new(34, 13)])),
                ]),
            ),
            Version::new(
                23,
                Vec::from([6, 30, 54, 78, 102]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(4, 121), ECB::new(5, 122)])),
                    ECBlocks::new(28, Vec::from([ECB::new(4, 47), ECB::new(14, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(11, 24), ECB::new(14, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(16, 15), ECB::new(14, 16)])),
                ]),
            ),
            Version::new(
                24,
                Vec::from([6, 28, 54, 80, 106]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(6, 117), ECB::new(4, 118)])),
                    ECBlocks::new(28, Vec::from([ECB::new(6, 45), ECB::new(14, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(11, 24), ECB::new(16, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(30, 16), ECB::new(2, 17)])),
                ]),
            ),
            Version::new(
                25,
                Vec::from([6, 32, 58, 84, 110]),
                Vec::from([
                    ECBlocks::new(26, Vec::from([ECB::new(8, 106), ECB::new(4, 107)])),
                    ECBlocks::new(28, Vec::from([ECB::new(8, 47), ECB::new(13, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(7, 24), ECB::new(22, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(22, 15), ECB::new(13, 16)])),
                ]),
            ),
            Version::new(
                26,
                Vec::from([6, 30, 58, 86, 114]),
                Vec::from([
                    ECBlocks::new(28, Vec::from([ECB::new(10, 114), ECB::new(2, 115)])),
                    ECBlocks::new(28, Vec::from([ECB::new(19, 46), ECB::new(4, 47)])),
                    ECBlocks::new(28, Vec::from([ECB::new(28, 22), ECB::new(6, 23)])),
                    ECBlocks::new(30, Vec::from([ECB::new(33, 16), ECB::new(4, 17)])),
                ]),
            ),
            Version::new(
                27,
                Vec::from([6, 34, 62, 90, 118]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(8, 122), ECB::new(4, 123)])),
                    ECBlocks::new(28, Vec::from([ECB::new(22, 45), ECB::new(3, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(8, 23), ECB::new(26, 24)])),
                    ECBlocks::new(30, Vec::from([ECB::new(12, 15), ECB::new(28, 16)])),
                ]),
            ),
            Version::new(
                28,
                Vec::from([6, 26, 50, 74, 98, 122]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(3, 117), ECB::new(10, 118)])),
                    ECBlocks::new(28, Vec::from([ECB::new(3, 45), ECB::new(23, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(4, 24), ECB::new(31, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(11, 15), ECB::new(31, 16)])),
                ]),
            ),
            Version::new(
                29,
                Vec::from([6, 30, 54, 78, 102, 126]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(7, 116), ECB::new(7, 117)])),
                    ECBlocks::new(28, Vec::from([ECB::new(21, 45), ECB::new(7, 46)])),
                    ECBlocks::new(30, Vec::from([ECB::new(1, 23), ECB::new(37, 24)])),
                    ECBlocks::new(30, Vec::from([ECB::new(19, 15), ECB::new(26, 16)])),
                ]),
            ),
            Version::new(
                30,
                Vec::from([6, 26, 52, 78, 104, 130]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(5, 115), ECB::new(10, 116)])),
                    ECBlocks::new(28, Vec::from([ECB::new(19, 47), ECB::new(10, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(15, 24), ECB::new(25, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(23, 15), ECB::new(25, 16)])),
                ]),
            ),
            Version::new(
                31,
                Vec::from([6, 30, 56, 82, 108, 134]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(13, 115), ECB::new(3, 116)])),
                    ECBlocks::new(28, Vec::from([ECB::new(2, 46), ECB::new(29, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(42, 24), ECB::new(1, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(23, 15), ECB::new(28, 16)])),
                ]),
            ),
            Version::new(
                32,
                Vec::from([6, 34, 60, 86, 112, 138]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(17, 115)])),
                    ECBlocks::new(28, Vec::from([ECB::new(10, 46), ECB::new(23, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(10, 24), ECB::new(35, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(19, 15), ECB::new(35, 16)])),
                ]),
            ),
            Version::new(
                33,
                Vec::from([6, 30, 58, 86, 114, 142]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(17, 115), ECB::new(1, 116)])),
                    ECBlocks::new(28, Vec::from([ECB::new(14, 46), ECB::new(21, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(29, 24), ECB::new(19, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(11, 15), ECB::new(46, 16)])),
                ]),
            ),
            Version::new(
                34,
                Vec::from([6, 34, 62, 90, 118, 146]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(13, 115), ECB::new(6, 116)])),
                    ECBlocks::new(28, Vec::from([ECB::new(14, 46), ECB::new(23, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(44, 24), ECB::new(7, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(59, 16), ECB::new(1, 17)])),
                ]),
            ),
            Version::new(
                35,
                Vec::from([6, 30, 54, 78, 102, 126, 150]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(12, 121), ECB::new(7, 122)])),
                    ECBlocks::new(28, Vec::from([ECB::new(12, 47), ECB::new(26, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(39, 24), ECB::new(14, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(22, 15), ECB::new(41, 16)])),
                ]),
            ),
            Version::new(
                36,
                Vec::from([6, 24, 50, 76, 102, 128, 154]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(6, 121), ECB::new(14, 122)])),
                    ECBlocks::new(28, Vec::from([ECB::new(6, 47), ECB::new(34, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(46, 24), ECB::new(10, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(2, 15), ECB::new(64, 16)])),
                ]),
            ),
            Version::new(
                37,
                Vec::from([6, 28, 54, 80, 106, 132, 158]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(17, 122), ECB::new(4, 123)])),
                    ECBlocks::new(28, Vec::from([ECB::new(29, 46), ECB::new(14, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(49, 24), ECB::new(10, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(24, 15), ECB::new(46, 16)])),
                ]),
            ),
            Version::new(
                38,
                Vec::from([6, 32, 58, 84, 110, 136, 162]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(4, 122), ECB::new(18, 123)])),
                    ECBlocks::new(28, Vec::from([ECB::new(13, 46), ECB::new(32, 47)])),
                    ECBlocks::new(30, Vec::from([ECB::new(48, 24), ECB::new(14, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(42, 15), ECB::new(32, 16)])),
                ]),
            ),
            Version::new(
                39,
                Vec::from([6, 26, 54, 82, 110, 138, 166]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(20, 117), ECB::new(4, 118)])),
                    ECBlocks::new(28, Vec::from([ECB::new(40, 47), ECB::new(7, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(43, 24), ECB::new(22, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(10, 15), ECB::new(67, 16)])),
                ]),
            ),
            Version::new(
                40,
                Vec::from([6, 30, 58, 86, 114, 142, 170]),
                Vec::from([
                    ECBlocks::new(30, Vec::from([ECB::new(19, 118), ECB::new(6, 119)])),
                    ECBlocks::new(28, Vec::from([ECB::new(18, 47), ECB::new(31, 48)])),
                    ECBlocks::new(30, Vec::from([ECB::new(34, 24), ECB::new(34, 25)])),
                    ECBlocks::new(30, Vec::from([ECB::new(20, 15), ECB::new(61, 16)])),
                ]),
            ),
        ]) /*
               new Version(4, new int[]{6, 26},
                   new ECBlocks(20, new ECB::new(1, 80)),
                   new ECBlocks(18, new ECB::new(2, 32)),
                   new ECBlocks(26, new ECB::new(2, 24)),
                   new ECBlocks(16, new ECB::new(4, 9))),
               new Version(5, new int[]{6, 30},
                   new ECBlocks(26, new ECB::new(1, 108)),
                   new ECBlocks(24, new ECB::new(2, 43)),
                   new ECBlocks(18, new ECB::new(2, 15),
                       new ECB::new(2, 16)),
                   new ECBlocks(22, new ECB::new(2, 11),
                       new ECB::new(2, 12))),
               new Version(6, new int[]{6, 34},
                   new ECBlocks(18, new ECB::new(2, 68)),
                   new ECBlocks(16, new ECB::new(4, 27)),
                   new ECBlocks(24, new ECB::new(4, 19)),
                   new ECBlocks(28, new ECB::new(4, 15))),
               new Version(7, new int[]{6, 22, 38},
                   new ECBlocks(20, new ECB::new(2, 78)),
                   new ECBlocks(18, new ECB::new(4, 31)),
                   new ECBlocks(18, new ECB::new(2, 14),
                       new ECB::new(4, 15)),
                   new ECBlocks(26, new ECB::new(4, 13),
                       new ECB::new(1, 14))),
               new Version(8, new int[]{6, 24, 42},
                   new ECBlocks(24, new ECB::new(2, 97)),
                   new ECBlocks(22, new ECB::new(2, 38),
                       new ECB::new(2, 39)),
                   new ECBlocks(22, new ECB::new(4, 18),
                       new ECB::new(2, 19)),
                   new ECBlocks(26, new ECB::new(4, 14),
                       new ECB::new(2, 15))),
               new Version(9, new int[]{6, 26, 46},
                   new ECBlocks(30, new ECB::new(2, 116)),
                   new ECBlocks(22, new ECB::new(3, 36),
                       new ECB::new(2, 37)),
                   new ECBlocks(20, new ECB::new(4, 16),
                       new ECB::new(4, 17)),
                   new ECBlocks(24, new ECB::new(4, 12),
                       new ECB::new(4, 13))),
               new Version(10, new int[]{6, 28, 50},
                   new ECBlocks(18, new ECB::new(2, 68),
                       new ECB::new(2, 69)),
                   new ECBlocks(26, new ECB::new(4, 43),
                       new ECB::new(1, 44)),
                   new ECBlocks(24, new ECB::new(6, 19),
                       new ECB::new(2, 20)),
                   new ECBlocks(28, new ECB::new(6, 15),
                       new ECB::new(2, 16))),
               new Version(11, new int[]{6, 30, 54},
                   new ECBlocks(20, new ECB::new(4, 81)),
                   new ECBlocks(30, new ECB::new(1, 50),
                       new ECB::new(4, 51)),
                   new ECBlocks(28, new ECB::new(4, 22),
                       new ECB::new(4, 23)),
                   new ECBlocks(24, new ECB::new(3, 12),
                       new ECB::new(8, 13))),
               new Version(12, new int[]{6, 32, 58},
                   new ECBlocks(24, new ECB::new(2, 92),
                       new ECB::new(2, 93)),
                   new ECBlocks(22, new ECB::new(6, 36),
                       new ECB::new(2, 37)),
                   new ECBlocks(26, new ECB::new(4, 20),
                       new ECB::new(6, 21)),
                   new ECBlocks(28, new ECB::new(7, 14),
                       new ECB::new(4, 15))),
               new Version(13, new int[]{6, 34, 62},
                   new ECBlocks(26, new ECB::new(4, 107)),
                   new ECBlocks(22, new ECB::new(8, 37),
                       new ECB::new(1, 38)),
                   new ECBlocks(24, new ECB::new(8, 20),
                       new ECB::new(4, 21)),
                   new ECBlocks(22, new ECB::new(12, 11),
                       new ECB::new(4, 12))),
               new Version(14, new int[]{6, 26, 46, 66},
                   new ECBlocks(30, new ECB::new(3, 115),
                       new ECB::new(1, 116)),
                   new ECBlocks(24, new ECB::new(4, 40),
                       new ECB::new(5, 41)),
                   new ECBlocks(20, new ECB::new(11, 16),
                       new ECB::new(5, 17)),
                   new ECBlocks(24, new ECB::new(11, 12),
                       new ECB::new(5, 13))),
               new Version(15, new int[]{6, 26, 48, 70},
                   new ECBlocks(22, new ECB::new(5, 87),
                       new ECB::new(1, 88)),
                   new ECBlocks(24, new ECB::new(5, 41),
                       new ECB::new(5, 42)),
                   new ECBlocks(30, new ECB::new(5, 24),
                       new ECB::new(7, 25)),
                   new ECBlocks(24, new ECB::new(11, 12),
                       new ECB::new(7, 13))),
               new Version(16, new int[]{6, 26, 50, 74},
                   new ECBlocks(24, new ECB::new(5, 98),
                       new ECB::new(1, 99)),
                   new ECBlocks(28, new ECB::new(7, 45),
                       new ECB::new(3, 46)),
                   new ECBlocks(24, new ECB::new(15, 19),
                       new ECB::new(2, 20)),
                   new ECBlocks(30, new ECB::new(3, 15),
                       new ECB::new(13, 16))),
               new Version(17, new int[]{6, 30, 54, 78},
                   new ECBlocks(28, new ECB::new(1, 107),
                       new ECB::new(5, 108)),
                   new ECBlocks(28, new ECB::new(10, 46),
                       new ECB::new(1, 47)),
                   new ECBlocks(28, new ECB::new(1, 22),
                       new ECB::new(15, 23)),
                   new ECBlocks(28, new ECB::new(2, 14),
                       new ECB::new(17, 15))),
               new Version(18, new int[]{6, 30, 56, 82},
                   new ECBlocks(30, new ECB::new(5, 120),
                       new ECB::new(1, 121)),
                   new ECBlocks(26, new ECB::new(9, 43),
                       new ECB::new(4, 44)),
                   new ECBlocks(28, new ECB::new(17, 22),
                       new ECB::new(1, 23)),
                   new ECBlocks(28, new ECB::new(2, 14),
                       new ECB::new(19, 15))),
               new Version(19, new int[]{6, 30, 58, 86},
                   new ECBlocks(28, new ECB::new(3, 113),
                       new ECB::new(4, 114)),
                   new ECBlocks(26, new ECB::new(3, 44),
                       new ECB::new(11, 45)),
                   new ECBlocks(26, new ECB::new(17, 21),
                       new ECB::new(4, 22)),
                   new ECBlocks(26, new ECB::new(9, 13),
                       new ECB::new(16, 14))),
               new Version(20, new int[]{6, 34, 62, 90},
                   new ECBlocks(28, new ECB::new(3, 107),
                       new ECB::new(5, 108)),
                   new ECBlocks(26, new ECB::new(3, 41),
                       new ECB::new(13, 42)),
                   new ECBlocks(30, new ECB::new(15, 24),
                       new ECB::new(5, 25)),
                   new ECBlocks(28, new ECB::new(15, 15),
                       new ECB::new(10, 16))),
               new Version(21, new int[]{6, 28, 50, 72, 94},
                   new ECBlocks(28, new ECB::new(4, 116),
                       new ECB::new(4, 117)),
                   new ECBlocks(26, new ECB::new(17, 42)),
                   new ECBlocks(28, new ECB::new(17, 22),
                       new ECB::new(6, 23)),
                   new ECBlocks(30, new ECB::new(19, 16),
                       new ECB::new(6, 17))),
               new Version(22, new int[]{6, 26, 50, 74, 98},
                   new ECBlocks(28, new ECB::new(2, 111),
                       new ECB::new(7, 112)),
                   new ECBlocks(28, new ECB::new(17, 46)),
                   new ECBlocks(30, new ECB::new(7, 24),
                       new ECB::new(16, 25)),
                   new ECBlocks(24, new ECB::new(34, 13))),
               new Version(23, new int[]{6, 30, 54, 78, 102},
                   new ECBlocks(30, new ECB::new(4, 121),
                       new ECB::new(5, 122)),
                   new ECBlocks(28, new ECB::new(4, 47),
                       new ECB::new(14, 48)),
                   new ECBlocks(30, new ECB::new(11, 24),
                       new ECB::new(14, 25)),
                   new ECBlocks(30, new ECB::new(16, 15),
                       new ECB::new(14, 16))),
               new Version(24, new int[]{6, 28, 54, 80, 106},
                   new ECBlocks(30, new ECB::new(6, 117),
                       new ECB::new(4, 118)),
                   new ECBlocks(28, new ECB::new(6, 45),
                       new ECB::new(14, 46)),
                   new ECBlocks(30, new ECB::new(11, 24),
                       new ECB::new(16, 25)),
                   new ECBlocks(30, new ECB::new(30, 16),
                       new ECB::new(2, 17))),
               new Version(25, new int[]{6, 32, 58, 84, 110},
                   new ECBlocks(26, new ECB::new(8, 106),
                       new ECB::new(4, 107)),
                   new ECBlocks(28, new ECB::new(8, 47),
                       new ECB::new(13, 48)),
                   new ECBlocks(30, new ECB::new(7, 24),
                       new ECB::new(22, 25)),
                   new ECBlocks(30, new ECB::new(22, 15),
                       new ECB::new(13, 16))),
               new Version(26, new int[]{6, 30, 58, 86, 114},
                   new ECBlocks(28, new ECB::new(10, 114),
                       new ECB::new(2, 115)),
                   new ECBlocks(28, new ECB::new(19, 46),
                       new ECB::new(4, 47)),
                   new ECBlocks(28, new ECB::new(28, 22),
                       new ECB::new(6, 23)),
                   new ECBlocks(30, new ECB::new(33, 16),
                       new ECB::new(4, 17))),
               new Version(27, new int[]{6, 34, 62, 90, 118},
                   new ECBlocks(30, new ECB::new(8, 122),
                       new ECB::new(4, 123)),
                   new ECBlocks(28, new ECB::new(22, 45),
                       new ECB::new(3, 46)),
                   new ECBlocks(30, new ECB::new(8, 23),
                       new ECB::new(26, 24)),
                   new ECBlocks(30, new ECB::new(12, 15),
                       new ECB::new(28, 16))),
               new Version(28, new int[]{6, 26, 50, 74, 98, 122},
                   new ECBlocks(30, new ECB::new(3, 117),
                       new ECB::new(10, 118)),
                   new ECBlocks(28, new ECB::new(3, 45),
                       new ECB::new(23, 46)),
                   new ECBlocks(30, new ECB::new(4, 24),
                       new ECB::new(31, 25)),
                   new ECBlocks(30, new ECB::new(11, 15),
                       new ECB::new(31, 16))),
               new Version(29, new int[]{6, 30, 54, 78, 102, 126},
                   new ECBlocks(30, new ECB::new(7, 116),
                       new ECB::new(7, 117)),
                   new ECBlocks(28, new ECB::new(21, 45),
                       new ECB::new(7, 46)),
                   new ECBlocks(30, new ECB::new(1, 23),
                       new ECB::new(37, 24)),
                   new ECBlocks(30, new ECB::new(19, 15),
                       new ECB::new(26, 16))),
               new Version(30, new int[]{6, 26, 52, 78, 104, 130},
                   new ECBlocks(30, new ECB::new(5, 115),
                       new ECB::new(10, 116)),
                   new ECBlocks(28, new ECB::new(19, 47),
                       new ECB::new(10, 48)),
                   new ECBlocks(30, new ECB::new(15, 24),
                       new ECB::new(25, 25)),
                   new ECBlocks(30, new ECB::new(23, 15),
                       new ECB::new(25, 16))),
               new Version(31, new int[]{6, 30, 56, 82, 108, 134},
                   new ECBlocks(30, new ECB::new(13, 115),
                       new ECB::new(3, 116)),
                   new ECBlocks(28, new ECB::new(2, 46),
                       new ECB::new(29, 47)),
                   new ECBlocks(30, new ECB::new(42, 24),
                       new ECB::new(1, 25)),
                   new ECBlocks(30, new ECB::new(23, 15),
                       new ECB::new(28, 16))),
               new Version(32, new int[]{6, 34, 60, 86, 112, 138},
                   new ECBlocks(30, new ECB::new(17, 115)),
                   new ECBlocks(28, new ECB::new(10, 46),
                       new ECB::new(23, 47)),
                   new ECBlocks(30, new ECB::new(10, 24),
                       new ECB::new(35, 25)),
                   new ECBlocks(30, new ECB::new(19, 15),
                       new ECB::new(35, 16))),
               new Version(33, new int[]{6, 30, 58, 86, 114, 142},
                   new ECBlocks(30, new ECB::new(17, 115),
                       new ECB::new(1, 116)),
                   new ECBlocks(28, new ECB::new(14, 46),
                       new ECB::new(21, 47)),
                   new ECBlocks(30, new ECB::new(29, 24),
                       new ECB::new(19, 25)),
                   new ECBlocks(30, new ECB::new(11, 15),
                       new ECB::new(46, 16))),
               new Version(34, new int[]{6, 34, 62, 90, 118, 146},
                   new ECBlocks(30, new ECB::new(13, 115),
                       new ECB::new(6, 116)),
                   new ECBlocks(28, new ECB::new(14, 46),
                       new ECB::new(23, 47)),
                   new ECBlocks(30, new ECB::new(44, 24),
                       new ECB::new(7, 25)),
                   new ECBlocks(30, new ECB::new(59, 16),
                       new ECB::new(1, 17))),
               new Version(35, new int[]{6, 30, 54, 78, 102, 126, 150},
                   new ECBlocks(30, new ECB::new(12, 121),
                       new ECB::new(7, 122)),
                   new ECBlocks(28, new ECB::new(12, 47),
                       new ECB::new(26, 48)),
                   new ECBlocks(30, new ECB::new(39, 24),
                       new ECB::new(14, 25)),
                   new ECBlocks(30, new ECB::new(22, 15),
                       new ECB::new(41, 16))),
               new Version(36, new int[]{6, 24, 50, 76, 102, 128, 154},
                   new ECBlocks(30, new ECB::new(6, 121),
                       new ECB::new(14, 122)),
                   new ECBlocks(28, new ECB::new(6, 47),
                       new ECB::new(34, 48)),
                   new ECBlocks(30, new ECB::new(46, 24),
                       new ECB::new(10, 25)),
                   new ECBlocks(30, new ECB::new(2, 15),
                       new ECB::new(64, 16))),
               new Version(37, new int[]{6, 28, 54, 80, 106, 132, 158},
                   new ECBlocks(30, new ECB::new(17, 122),
                       new ECB::new(4, 123)),
                   new ECBlocks(28, new ECB::new(29, 46),
                       new ECB::new(14, 47)),
                   new ECBlocks(30, new ECB::new(49, 24),
                       new ECB::new(10, 25)),
                   new ECBlocks(30, new ECB::new(24, 15),
                       new ECB::new(46, 16))),
               new Version(38, new int[]{6, 32, 58, 84, 110, 136, 162},
                   new ECBlocks(30, new ECB::new(4, 122),
                       new ECB::new(18, 123)),
                   new ECBlocks(28, new ECB::new(13, 46),
                       new ECB::new(32, 47)),
                   new ECBlocks(30, new ECB::new(48, 24),
                       new ECB::new(14, 25)),
                   new ECBlocks(30, new ECB::new(42, 15),
                       new ECB::new(32, 16))),
               new Version(39, new int[]{6, 26, 54, 82, 110, 138, 166},
                   new ECBlocks(30, new ECB::new(20, 117),
                       new ECB::new(4, 118)),
                   new ECBlocks(28, new ECB::new(40, 47),
                       new ECB::new(7, 48)),
                   new ECBlocks(30, new ECB::new(43, 24),
                       new ECB::new(22, 25)),
                   new ECBlocks(30, new ECB::new(10, 15),
                       new ECB::new(67, 16))),
               new Version(40, new int[]{6, 30, 58, 86, 114, 142, 170},
                   new ECBlocks(30, new ECB::new(19, 118),
                       new ECB::new(6, 119)),
                   new ECBlocks(28, new ECB::new(18, 47),
                       new ECB::new(31, 48)),
                   new ECBlocks(30, new ECB::new(34, 24),
                       new ECB::new(34, 25)),
                   new ECBlocks(30, new ECB::new(20, 15),
                       new ECB::new(61, 16)))
           ]*/
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
#[derive(Debug)]
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

    pub fn getECCodewordsPerBlock(&self) -> u32 {
        self.ecCodewordsPerBlock
    }

    pub fn getNumBlocks(&self) -> u32 {
        let mut total = 0;
        for ecBlock in &self.ecBlocks {
            //   for (ECB ecBlock : ecBlocks) {
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
#[derive(Debug)]
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

    pub fn getCount(&self) -> u32 {
        self.count
    }

    pub fn getDataCodewords(&self) -> u32 {
        self.dataCodewords
    }
}
