use crate::common::Result;
use crate::qrcode::decoder::{Version, VersionRef, MICRO_VERSIONS, VERSIONS, VERSION_DECODE_INFO};
use crate::Exceptions;

// const Version* Version::AllMicroVersions()
// {
// 	/**
// 	 * See ISO 18004:2006 6.5.1 Table 9
// 	 */
// 	static const Version allVersions[] = {
// 		{1, {2, 1, 3, 0, 0}},
// 		{2, {5, 1, 5, 0, 0, 6, 1, 4, 0, 0}},
// 		{3, {6, 1, 11, 0, 0, 8, 1, 9, 0, 0}},
// 		{4, {8, 1, 16, 0, 0, 10, 1, 14, 0, 0, 14, 1, 10, 0, 0}}};
// 	return allVersions;
// }

impl Version {
    pub fn FromDimension(dimension: u32) -> Result<VersionRef> {
        let isMicro = dimension < 21;
        if dimension % Self::DimensionStep(isMicro) != 1 {
            //throw std::invalid_argument("Unexpected dimension");
            return Err(Exceptions::ILLEGAL_ARGUMENT);
        }
        return Self::FromNumber(
            (dimension - Self::DimensionOffset(isMicro)) / Self::DimensionStep(isMicro),
            isMicro,
        );
    }

    pub fn FromNumber(versionNumber: u32, is_micro: bool) -> Result<VersionRef> {
        if versionNumber < 1 || versionNumber > (if is_micro { 4 } else { 40 }) {
            //throw std::invalid_argument("Version should be in range [1-40].");
            return Err(Exceptions::ILLEGAL_ARGUMENT);
        }

        Ok(if is_micro {
            &MICRO_VERSIONS[versionNumber as usize - 1]
        } else {
            &VERSIONS[versionNumber as usize - 1]
        })
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
        let mut i = 0;
        for targetVersion in VERSION_DECODE_INFO {
            // for (int targetVersion : VERSION_DECODE_INFO) {
            // Do the version info bits match exactly? done.
            if targetVersion == versionBitsA as u32 || targetVersion == versionBitsB as u32 {
                return Self::getVersionForNumber(i + 7);
            }
            // Otherwise see if this is the closest to a real version info bit string
            // we have seen so far
            for bits in [versionBitsA, versionBitsB] {
                // for (int bits : {versionBitsA, versionBitsB}) {
                let bitsDifference = ((bits as u32) ^ targetVersion).count_ones(); //BitHacks::CountBitsSet(bits ^ targetVersion);
                if bitsDifference < bestDifference {
                    bestVersion = i + 7;
                    bestDifference = bitsDifference;
                }
            }
            i += 1;
        }
        // We can tolerate up to 3 bits of error since no two version info codewords will
        // differ in less than 8 bits.
        if bestDifference <= 3 {
            return Self::getVersionForNumber(bestVersion);
        }
        // If we didn't find a close enough match, fail
        Err(Exceptions::ILLEGAL_STATE)
    }

    pub const fn isMicroQRCode(&self) -> bool {
        self.is_micro
    }
}
