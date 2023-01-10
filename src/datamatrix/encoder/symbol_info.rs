/*
 * Copyright 2006 Jeremias Maerki
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

use crate::{Dimension, Exceptions};

use super::SymbolShapeHint;
use once_cell::sync::Lazy;

pub(super) static PROD_SYMBOLS: Lazy<Vec<SymbolInfo>> = Lazy::new(|| {
    vec![
        SymbolInfo::new(false, 3, 5, 8, 8, 1),
        SymbolInfo::new(false, 5, 7, 10, 10, 1),
        /*rect*/ SymbolInfo::new(true, 5, 7, 16, 6, 1),
        SymbolInfo::new(false, 8, 10, 12, 12, 1),
        /*rect*/ SymbolInfo::new(true, 10, 11, 14, 6, 2),
        SymbolInfo::new(false, 12, 12, 14, 14, 1),
        /*rect*/ SymbolInfo::new(true, 16, 14, 24, 10, 1),
        SymbolInfo::new(false, 18, 14, 16, 16, 1),
        SymbolInfo::new(false, 22, 18, 18, 18, 1),
        /*rect*/ SymbolInfo::new(true, 22, 18, 16, 10, 2),
        SymbolInfo::new(false, 30, 20, 20, 20, 1),
        /*rect*/ SymbolInfo::new(true, 32, 24, 16, 14, 2),
        SymbolInfo::new(false, 36, 24, 22, 22, 1),
        SymbolInfo::new(false, 44, 28, 24, 24, 1),
        /*rect*/ SymbolInfo::new(true, 49, 28, 22, 14, 2),
        SymbolInfo::new(false, 62, 36, 14, 14, 4),
        SymbolInfo::new(false, 86, 42, 16, 16, 4),
        SymbolInfo::new(false, 114, 48, 18, 18, 4),
        SymbolInfo::new(false, 144, 56, 20, 20, 4),
        SymbolInfo::new(false, 174, 68, 22, 22, 4),
        SymbolInfo::with_details(false, 204, 84, 24, 24, 4, 102, 42),
        SymbolInfo::with_details(false, 280, 112, 14, 14, 16, 140, 56),
        SymbolInfo::with_details(false, 368, 144, 16, 16, 16, 92, 36),
        SymbolInfo::with_details(false, 456, 192, 18, 18, 16, 114, 48),
        SymbolInfo::with_details(false, 576, 224, 20, 20, 16, 144, 56),
        SymbolInfo::with_details(false, 696, 272, 22, 22, 16, 174, 68),
        SymbolInfo::with_details(false, 816, 336, 24, 24, 16, 136, 56),
        SymbolInfo::with_details(false, 1050, 408, 18, 18, 36, 175, 68),
        SymbolInfo::with_details(false, 1304, 496, 20, 20, 36, 163, 62),
        SymbolInfo::new_symbol_info_144(),
    ]
});

/**
 * Symbol info table for DataMatrix.
 *
 * @version $Id$
 */
pub struct SymbolInfo {
    rectangular: bool,
    dataCapacity: u32,
    errorCodewords: u32,
    pub(crate) matrixWidth: u32,
    pub(crate) matrixHeight: u32,
    dataRegions: u32,
    rsBlockData: i32,
    rsBlockError: u32,
    isSymbolInfo144: bool,
}
impl SymbolInfo {
    pub fn new(
        rectangular: bool,
        dataCapacity: u32,
        errorCodewords: u32,
        matrixWidth: u32,
        matrixHeight: u32,
        dataRegions: u32,
    ) -> Self {
        Self::with_details(
            rectangular,
            dataCapacity,
            errorCodewords,
            matrixWidth,
            matrixHeight,
            dataRegions,
            dataCapacity as i32,
            errorCodewords,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_details(
        rectangular: bool,
        dataCapacity: u32,
        errorCodewords: u32,
        matrixWidth: u32,
        matrixHeight: u32,
        dataRegions: u32,
        rsBlockData: i32,
        rsBlockError: u32,
    ) -> Self {
        Self {
            rectangular,
            dataCapacity,
            errorCodewords,
            matrixWidth,
            matrixHeight,
            dataRegions,
            rsBlockData,
            rsBlockError,
            isSymbolInfo144: false,
        }
    }
    pub fn new_symbol_info_144() -> Self {
        let mut new_symbol = Self::with_details(false, 1558, 620, 22, 22, 36, -1, 62);
        new_symbol.isSymbolInfo144 = true;
        new_symbol
    }

    fn getHorizontalDataRegions(&self) -> Result<u32, Exceptions> {
        match self.dataRegions {
            1 => Ok(1),
            2 | 4 => Ok(2),
            16 => Ok(4),
            36 => Ok(6),
            _ => Err(Exceptions::IllegalStateException(Some(
                "Cannot handle this number of data regions".to_owned(),
            ))),
        }
        // switch (dataRegions) {
        //   case 1:
        //     return 1;
        //   case 2:
        //   case 4:
        //     return 2;
        //   case 16:
        //     return 4;
        //   case 36:
        //     return 6;
        //   default:
        //     throw new IllegalStateException("Cannot handle this number of data regions");
        // }
    }

    fn getVerticalDataRegions(&self) -> Result<u32, Exceptions> {
        match self.dataRegions {
            1 | 2 => Ok(1),
            4 => Ok(2),
            16 => Ok(4),
            36 => Ok(6),
            _ => Err(Exceptions::IllegalStateException(Some(
                "Cannot handle this number of data regions".to_owned(),
            ))),
        }
        // switch (dataRegions) {
        //   case 1:
        //   case 2:
        //     return 1;
        //   case 4:
        //     return 2;
        //   case 16:
        //     return 4;
        //   case 36:
        //     return 6;
        //   default:
        //     throw new IllegalStateException("Cannot handle this number of data regions");
        // }
    }

    pub fn getSymbolDataWidth(&self) -> Result<u32, Exceptions> {
        Ok(self.getHorizontalDataRegions()? * self.matrixWidth)
    }

    pub fn getSymbolDataHeight(&self) -> Result<u32, Exceptions> {
        Ok(self.getVerticalDataRegions()? * self.matrixHeight)
    }

    pub fn getSymbolWidth(&self) -> Result<u32, Exceptions> {
        Ok(self.getSymbolDataWidth()? + (self.getHorizontalDataRegions()? * 2))
    }

    pub fn getSymbolHeight(&self) -> Result<u32, Exceptions> {
        Ok(self.getSymbolDataHeight()? + (self.getVerticalDataRegions()? * 2))
    }

    pub fn getCodewordCount(&self) -> u32 {
        self.dataCapacity + self.errorCodewords
    }

    pub fn getInterleavedBlockCount(&self) -> u32 {
        if self.isSymbolInfo144 {
            10
        } else {
            self.dataCapacity / self.rsBlockData as u32
        }
    }

    pub fn getDataCapacity(&self) -> u32 {
        self.dataCapacity
    }

    pub fn getErrorCodewords(&self) -> u32 {
        self.errorCodewords
    }

    pub fn getDataLengthForInterleavedBlock(&self, index: u32) -> i32 {
        if self.isSymbolInfo144 {
            if index <= 8 {
                156
            } else {
                155
            }
        } else {
            self.rsBlockData
        }
    }

    pub fn getErrorLengthForInterleavedBlock(&self, _index: u32) -> u32 {
        self.rsBlockError
    }

    // @Override
    // public final String toString() {
    //   return (rectangular ? "Rectangular Symbol:" : "Square Symbol:") +
    //       " data region " + matrixWidth + 'x' + matrixHeight +
    //       ", symbol size " + getSymbolWidth() + 'x' + getSymbolHeight() +
    //       ", symbol data size " + getSymbolDataWidth() + 'x' + getSymbolDataHeight() +
    //       ", codewords " + dataCapacity + '+' + errorCodewords;
    // }
}

impl fmt::Display for SymbolInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} data region {}x{}, symbol size {}x{}, symbol data size {}x{}, codewords {}+{}",
            if self.rectangular {
                "Rectangular Symbol:"
            } else {
                "Square Symbol:"
            },
            self.matrixWidth,
            self.matrixHeight,
            self.getSymbolWidth().unwrap_or_default(),
            self.getSymbolHeight().unwrap_or_default(),
            self.getSymbolDataWidth().unwrap_or_default(),
            self.getSymbolDataHeight().unwrap_or_default(),
            self.dataCapacity,
            self.errorCodewords
        )
    }
}

#[derive(Clone, Copy)]
pub struct SymbolInfoLookup<'a>(Option<&'a Vec<SymbolInfo>>);
impl<'a> SymbolInfoLookup<'a> {
    pub const fn new() -> Self {
        Self(None)
    }
    /**
     * Overrides the symbol info set used by this class. Used for testing purposes.
     *
     * @param override the symbol info set to use
     */
    pub fn overrideSymbolSet(&mut self, override_symbols: &'a Vec<SymbolInfo>) {
        self.0 = Some(override_symbols);
    }

    pub fn lookup(&self, dataCodewords: u32) -> Result<Option<&'a SymbolInfo>, Exceptions> {
        self.lookup_with_codewords_shape_fail(dataCodewords, SymbolShapeHint::FORCE_NONE, true)
    }

    pub fn lookup_with_shape(
        &self,
        dataCodewords: u32,
        shape: SymbolShapeHint,
    ) -> Result<Option<&'a SymbolInfo>, Exceptions> {
        self.lookup_with_codewords_shape_fail(dataCodewords, shape, true)
    }

    pub fn lookup_codwords_rectangule_fail(
        &self,
        dataCodewords: u32,
        allowRectangular: bool,
        fail: bool,
    ) -> Result<Option<&'a SymbolInfo>, Exceptions> {
        let shape = if allowRectangular {
            SymbolShapeHint::FORCE_NONE
        } else {
            SymbolShapeHint::FORCE_SQUARE
        };
        self.lookup_with_codewords_shape_fail(dataCodewords, shape, fail)
    }

    fn lookup_with_codewords_shape_fail(
        &self,
        dataCodewords: u32,
        shape: SymbolShapeHint,
        fail: bool,
    ) -> Result<Option<&'a SymbolInfo>, Exceptions> {
        self.lookup_with_codewords_shape_size_fail(dataCodewords, shape, &None, &None, fail)
    }

    pub fn lookup_with_codewords_shape_size_fail(
        &self,
        dataCodewords: u32,
        shape: SymbolShapeHint,
        minSize: &Option<Dimension>,
        maxSize: &Option<Dimension>,
        fail: bool,
        // alternate_symbols_chart: Option<&'a Vec<SymbolInfo>>,
    ) -> Result<Option<&'a SymbolInfo>, Exceptions> {
        let symbol_search_chart: &Vec<SymbolInfo> = if self.0.is_none() {
            &PROD_SYMBOLS
        } else {
            self.0.as_ref().unwrap()
        };
        for symbol in symbol_search_chart {
            // for (SymbolInfo symbol : symbols) {
            if shape == SymbolShapeHint::FORCE_SQUARE && symbol.rectangular {
                continue;
            }
            if shape == SymbolShapeHint::FORCE_RECTANGLE && !symbol.rectangular {
                continue;
            }
            if minSize.is_some()
                && ((symbol.getSymbolWidth()? as usize) < minSize.as_ref().unwrap().getWidth()
                    || (symbol.getSymbolHeight()? as usize) < minSize.as_ref().unwrap().getHeight())
            {
                continue;
            }
            if maxSize.is_some()
                && ((symbol.getSymbolWidth()? as usize) > maxSize.as_ref().unwrap().getWidth()
                    || (symbol.getSymbolHeight()? as usize) > maxSize.as_ref().unwrap().getHeight())
            {
                continue;
            }
            if dataCodewords <= symbol.dataCapacity {
                return Ok(Some(symbol));
            }
        }
        if fail {
            return Err(Exceptions::IllegalArgumentException(Some(format!(
                "Can't find a symbol arrangement that matches the message. Data codewords: {}",
                dataCodewords
            ))));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {

    use crate::{datamatrix::encoder::SymbolShapeHint, Dimension};

    use super::SymbolInfoLookup;

    #[allow(dead_code)]
    const LOOKUP: SymbolInfoLookup = SymbolInfoLookup::new();

    /**
     * Tests the SymbolInfo class.
     */
    #[test]
    fn testSymbolInfo() {
        let info = LOOKUP.lookup(3).expect("returns").expect("exists");
        assert_eq!(5, info.getErrorCodewords());
        assert_eq!(8, info.matrixWidth);
        assert_eq!(8, info.matrixHeight);
        assert_eq!(10, info.getSymbolWidth().expect("returns"));
        assert_eq!(10, info.getSymbolHeight().expect("returns"));

        let info = LOOKUP
            .lookup_with_shape(3, SymbolShapeHint::FORCE_RECTANGLE)
            .expect("returns")
            .expect("exists");
        assert_eq!(7, info.getErrorCodewords());
        assert_eq!(16, info.matrixWidth);
        assert_eq!(6, info.matrixHeight);
        assert_eq!(18, info.getSymbolWidth().expect("returns"));
        assert_eq!(8, info.getSymbolHeight().expect("returns"));

        let info = LOOKUP.lookup(9).expect("returns").expect("exists");
        assert_eq!(11, info.getErrorCodewords());
        assert_eq!(14, info.matrixWidth);
        assert_eq!(6, info.matrixHeight);
        assert_eq!(32, info.getSymbolWidth().expect("returns"));
        assert_eq!(8, info.getSymbolHeight().expect("returns"));

        let info = LOOKUP
            .lookup_with_shape(9, SymbolShapeHint::FORCE_SQUARE)
            .expect("returns")
            .expect("exists");
        assert_eq!(12, info.getErrorCodewords());
        assert_eq!(14, info.matrixWidth);
        assert_eq!(14, info.matrixHeight);
        assert_eq!(16, info.getSymbolWidth().expect("returns"));
        assert_eq!(16, info.getSymbolHeight().expect("returns"));

        assert!(LOOKUP.lookup(1559).is_err());

        //  try {
        //    SymbolInfo.lookup(1559);
        //    fail("There's no rectangular symbol for more than 1558 data codewords");
        //  } catch (IllegalArgumentException iae) {
        //    //expected
        //  }
        assert!(LOOKUP
            .lookup_with_shape(50, SymbolShapeHint::FORCE_RECTANGLE)
            .is_err());
        //  try {
        //    SymbolInfo.lookup(50, SymbolShapeHint.FORCE_RECTANGLE);
        //    fail("There's no rectangular symbol for 50 data codewords");
        //  } catch (IllegalArgumentException iae) {
        //    //expected
        //  }

        let info = LOOKUP.lookup(35).expect("returns").expect("exists");
        assert_eq!(24, info.getSymbolWidth().expect("return"));
        assert_eq!(24, info.getSymbolHeight().expect("return"));

        let fixedSize = Dimension::new(26, 26).expect("new dimension");
        let info = LOOKUP
            .lookup_with_codewords_shape_size_fail(
                35,
                SymbolShapeHint::FORCE_NONE,
                &Some(fixedSize),
                &Some(fixedSize),
                false,
            )
            .expect("returns");
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(26, info.getSymbolWidth().expect("return"));
        assert_eq!(26, info.getSymbolHeight().expect("return"));

        let info = LOOKUP
            .lookup_with_codewords_shape_size_fail(
                45,
                SymbolShapeHint::FORCE_NONE,
                &Some(fixedSize),
                &Some(fixedSize),
                false,
            )
            .expect("return");
        assert!(info.is_none());

        let minSize = fixedSize;
        let maxSize = Dimension::new(32, 32).expect("new dimension");

        let info = LOOKUP
            .lookup_with_codewords_shape_size_fail(
                35,
                SymbolShapeHint::FORCE_NONE,
                &Some(minSize),
                &Some(maxSize),
                false,
            )
            .expect("return");
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(26, info.getSymbolWidth().expect("return"));
        assert_eq!(26, info.getSymbolHeight().expect("return"));

        let info = LOOKUP
            .lookup_with_codewords_shape_size_fail(
                40,
                SymbolShapeHint::FORCE_NONE,
                &Some(minSize),
                &Some(maxSize),
                false,
            )
            .expect("return");
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(26, info.getSymbolWidth().expect("return"));
        assert_eq!(26, info.getSymbolHeight().expect("return"));

        let info = LOOKUP
            .lookup_with_codewords_shape_size_fail(
                45,
                SymbolShapeHint::FORCE_NONE,
                &Some(minSize),
                &Some(maxSize),
                false,
            )
            .expect("return");
        assert!(info.is_some());
        let info = info.unwrap();
        assert_eq!(32, info.getSymbolWidth().expect("return"));
        assert_eq!(32, info.getSymbolHeight().expect("return"));

        let info = LOOKUP
            .lookup_with_codewords_shape_size_fail(
                63,
                SymbolShapeHint::FORCE_NONE,
                &Some(minSize),
                &Some(maxSize),
                false,
            )
            .expect("return");
        assert!(info.is_none());
    }
}
