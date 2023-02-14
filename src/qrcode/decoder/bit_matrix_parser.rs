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

use crate::{
    common::{BitMatrix, Result},
    Exceptions,
};

use super::{DataMask, FormatInformation, Version, VersionRef};

/**
 * @author Sean Owen
 */
pub struct BitMatrixParser {
    bitMatrix: BitMatrix,
    parsedVersion: Option<VersionRef>,
    parsedFormatInfo: Option<FormatInformation>,
    mirror: bool,
}

impl BitMatrixParser {
    /**
     * @param bitMatrix {@link BitMatrix} to parse
     * @throws FormatException if dimension is not >= 21 and 1 mod 4
     */
    pub fn new(bit_matrix: BitMatrix) -> Result<Self> {
        let dimension = bit_matrix.getHeight();
        if dimension < 21 || (dimension & 0x03) != 1 {
            Err(Exceptions::FormatException(Some(format!(
                "{dimension} < 21 || ({dimension} % 0x03) != 1"
            ))))
        } else {
            Ok(Self {
                bitMatrix: bit_matrix,
                parsedVersion: None,
                parsedFormatInfo: None,
                mirror: false,
            })
        }
    }

    /**
     * <p>Reads format information from one of its two locations within the QR Code.</p>
     *
     * @return {@link FormatInformation} encapsulating the QR Code's format info
     * @throws FormatException if both format information locations cannot be parsed as
     * the valid encoding of format information
     */
    pub fn readFormatInformation(&mut self) -> Result<&FormatInformation> {
        if self.parsedFormatInfo.is_some() {
            return self
                .parsedFormatInfo
                .as_ref()
                .ok_or(Exceptions::ParseException(None));
        }

        // Read top-left format info bits
        let mut formatInfoBits1 = 0;
        for i in 0..6 {
            formatInfoBits1 = self.copyBit(i, 8, formatInfoBits1);
        }
        // .. and skip a bit in the timing pattern ...
        formatInfoBits1 = self.copyBit(7, 8, formatInfoBits1);
        formatInfoBits1 = self.copyBit(8, 8, formatInfoBits1);
        formatInfoBits1 = self.copyBit(8, 7, formatInfoBits1);
        // .. and skip a bit in the timing pattern ...
        for j in (0..=5).rev() {
            formatInfoBits1 = self.copyBit(8, j, formatInfoBits1);
        }

        // Read the top-right/bottom-left pattern too
        let dimension = self.bitMatrix.getHeight();
        let mut formatInfoBits2 = 0;
        let jMin = dimension - 7;
        for j in (jMin..=dimension - 1).rev() {
            formatInfoBits2 = self.copyBit(8, j, formatInfoBits2);
        }
        for i in (dimension - 8)..dimension {
            formatInfoBits2 = self.copyBit(i, 8, formatInfoBits2);
        }

        self.parsedFormatInfo =
            FormatInformation::decodeFormatInformation(formatInfoBits1, formatInfoBits2);

        self.parsedFormatInfo
            .as_ref()
            .ok_or(Exceptions::FormatException(None))
    }

    /**
     * <p>Reads version information from one of its two locations within the QR Code.</p>
     *
     * @return {@link Version} encapsulating the QR Code's version
     * @throws FormatException if both version information locations cannot be parsed as
     * the valid encoding of version information
     */
    pub fn readVersion(&mut self) -> Result<VersionRef> {
        if let Some(pv) = self.parsedVersion {
            return Ok(pv);
        }

        let dimension = self.bitMatrix.getHeight();

        let provisionalVersion = (dimension - 17) / 4;
        if provisionalVersion <= 6 {
            return Version::getVersionForNumber(provisionalVersion);
        }

        // Read top-right version info: 3 wide by 6 tall
        let mut versionBits = 0;
        let ijMin = dimension - 11;
        for j in (0..=5).rev() {
            for i in (ijMin..(dimension - 8)).rev() {
                versionBits = self.copyBit(i, j, versionBits);
            }
        }

        if let Ok(theParsedVersion) = Version::decodeVersionInformation(versionBits) {
            if theParsedVersion.getDimensionForVersion() == dimension {
                self.parsedVersion = Some(theParsedVersion);
                return Ok(theParsedVersion);
            }
        }

        // Hmm, failed. Try bottom left: 6 wide by 3 tall
        versionBits = 0;
        for i in (0..=5).rev() {
            for j in (ijMin..(dimension - 5)).rev() {
                versionBits = self.copyBit(i, j, versionBits);
            }
        }

        if let Ok(theParsedVersion) = Version::decodeVersionInformation(versionBits) {
            if theParsedVersion.getDimensionForVersion() == dimension {
                self.parsedVersion = Some(theParsedVersion);
                return Ok(theParsedVersion);
            }
        }
        Err(Exceptions::FormatException(None))
    }

    fn copyBit(&self, i: u32, j: u32, versionBits: u32) -> u32 {
        let bit = if self.mirror {
            self.bitMatrix.get(j, i)
        } else {
            self.bitMatrix.get(i, j)
        };

        if bit {
            (versionBits << 1) | 0x1
        } else {
            versionBits << 1
        }
    }

    /**
     * <p>Reads the bits in the {@link BitMatrix} representing the finder pattern in the
     * correct order in order to reconstruct the codewords bytes contained within the
     * QR Code.</p>
     *
     * @return bytes encoded within the QR Code
     * @throws FormatException if the exact number of bytes expected is not read
     */
    pub fn readCodewords(&mut self) -> Result<Vec<u8>> {
        let version = self.readVersion()?;

        // Get the data mask for the format used in this QR Code. This will exclude
        // some bits from reading as we wind through the bit matrix.
        let dataMask: DataMask = self.readFormatInformation()?.getDataMask().try_into()?;
        let dimension = self.bitMatrix.getHeight();
        dataMask.unmaskBitMatrix(&mut self.bitMatrix, dimension);

        let functionPattern = version.buildFunctionPattern()?;

        let mut readingUp = true;
        let mut result = vec![0u8; version.getTotalCodewords() as usize];
        let mut resultOffset = 0;
        let mut currentByte = 0;
        let mut bitsRead = 0;
        // Read columns in pairs, from right to left
        let mut j = dimension as i32 - 1;
        while j > 0 {
            if j == 6 {
                // Skip whole column with vertical alignment pattern;
                // saves time and makes the other code proceed more cleanly
                j -= 1;
            }
            // Read alternatingly from bottom to top then top to bottom
            for count in 0..dimension {
                let i = if readingUp {
                    dimension - 1 - count
                } else {
                    count
                };
                for col in 0..2 {
                    // Ignore bits covered by the function pattern
                    if !functionPattern.get(j as u32 - col, i) {
                        // Read a bit
                        bitsRead += 1;
                        currentByte <<= 1;
                        if self.bitMatrix.get(j as u32 - col, i) {
                            currentByte |= 1;
                        }
                        // If we've made a whole byte, save it off
                        if bitsRead == 8 {
                            result[resultOffset] = currentByte;
                            resultOffset += 1;
                            bitsRead = 0;
                            currentByte = 0;
                        }
                    }
                }
            }
            readingUp ^= true; // readingUp = !readingUp; // switch directions

            j -= 2;
        }

        if resultOffset != version.getTotalCodewords() as usize {
            return Err(Exceptions::FormatException(None));
        }
        Ok(result)
    }

    /**
     * Revert the mask removal done while reading the code words. The bit matrix should revert to its original state.
     */
    pub fn remask(&mut self) -> Result<()> {
        if let Some(pfi) = &self.parsedFormatInfo {
            let dataMask: DataMask = pfi.getDataMask().try_into()?;
            let dimension = self.bitMatrix.getHeight();
            dataMask.unmaskBitMatrix(&mut self.bitMatrix, dimension);
        } else {
            // We have no format information, and have no data mask
        }
        Ok(())
    }

    /**
     * Prepare the parser for a mirrored operation.
     * This flag has effect only on the {@link #readFormatInformation()} and the
     * {@link #readVersion()}. Before proceeding with {@link #readCodewords()} the
     * {@link #mirror()} method should be called.
     *
     * @param mirror Whether to read version and format information mirrored.
     */
    pub fn setMirror(&mut self, mirror: bool) {
        self.parsedVersion = None;
        self.parsedFormatInfo = None;
        self.mirror = mirror;
    }

    /** Mirror the bit matrix in order to attempt a second reading. */
    pub fn mirror(&mut self) {
        for x in 0..self.bitMatrix.getWidth() {
            for y in (x + 1)..self.bitMatrix.getHeight() {
                if self.bitMatrix.get(x, y) != self.bitMatrix.get(y, x) {
                    self.bitMatrix.flip_coords(y, x);
                    self.bitMatrix.flip_coords(x, y);
                }
            }
        }
    }
}
