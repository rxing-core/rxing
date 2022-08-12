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
// package com::google::zxing::datamatrix::encoder;

/**
 * Symbol info table for DataMatrix.
 *
 * @version $Id$
 */

 const PROD_SYMBOLS: vec![Vec<SymbolInfo>; 30] = vec![SymbolInfo::new(false, 3, 5, 8, 8, 1), SymbolInfo::new(false, 5, 7, 10, 10, 1), /*rect*/
SymbolInfo::new(true, 5, 7, 16, 6, 1), SymbolInfo::new(false, 8, 10, 12, 12, 1), /*rect*/
SymbolInfo::new(true, 10, 11, 14, 6, 2), SymbolInfo::new(false, 12, 12, 14, 14, 1), /*rect*/
SymbolInfo::new(true, 16, 14, 24, 10, 1), SymbolInfo::new(false, 18, 14, 16, 16, 1), SymbolInfo::new(false, 22, 18, 18, 18, 1), /*rect*/
SymbolInfo::new(true, 22, 18, 16, 10, 2), SymbolInfo::new(false, 30, 20, 20, 20, 1), /*rect*/
SymbolInfo::new(true, 32, 24, 16, 14, 2), SymbolInfo::new(false, 36, 24, 22, 22, 1), SymbolInfo::new(false, 44, 28, 24, 24, 1), /*rect*/
SymbolInfo::new(true, 49, 28, 22, 14, 2), SymbolInfo::new(false, 62, 36, 14, 14, 4), SymbolInfo::new(false, 86, 42, 16, 16, 4), SymbolInfo::new(false, 114, 48, 18, 18, 4), SymbolInfo::new(false, 144, 56, 20, 20, 4), SymbolInfo::new(false, 174, 68, 22, 22, 4), SymbolInfo::new(false, 204, 84, 24, 24, 4, 102, 42), SymbolInfo::new(false, 280, 112, 14, 14, 16, 140, 56), SymbolInfo::new(false, 368, 144, 16, 16, 16, 92, 36), SymbolInfo::new(false, 456, 192, 18, 18, 16, 114, 48), SymbolInfo::new(false, 576, 224, 20, 20, 16, 144, 56), SymbolInfo::new(false, 696, 272, 22, 22, 16, 174, 68), SymbolInfo::new(false, 816, 336, 24, 24, 16, 136, 56), SymbolInfo::new(false, 1050, 408, 18, 18, 36, 175, 68), SymbolInfo::new(false, 1304, 496, 20, 20, 36, 163, 62), DataMatrixSymbolInfo144::new(), ]
;

 let mut symbols: Vec<SymbolInfo> = PROD_SYMBOLS;
pub struct SymbolInfo {

     let rectangular: bool;

     let data_capacity: i32;

     let error_codewords: i32;

     let matrix_width: i32;

     let matrix_height: i32;

     let data_regions: i32;

     let rs_block_data: i32;

     let rs_block_error: i32;
}

impl SymbolInfo {

    /**
   * Overrides the symbol info set used by this class. Used for testing purposes.
   *
   * @param override the symbol info set to use
   */
    pub fn  override_symbol_set( override: &Vec<SymbolInfo>)   {
        symbols = override;
    }

    pub fn new( rectangular: bool,  data_capacity: i32,  error_codewords: i32,  matrix_width: i32,  matrix_height: i32,  data_regions: i32) -> SymbolInfo {
        this(rectangular, data_capacity, error_codewords, matrix_width, matrix_height, data_regions, data_capacity, error_codewords);
    }

    fn new( rectangular: bool,  data_capacity: i32,  error_codewords: i32,  matrix_width: i32,  matrix_height: i32,  data_regions: i32,  rs_block_data: i32,  rs_block_error: i32) -> SymbolInfo {
        let .rectangular = rectangular;
        let .dataCapacity = data_capacity;
        let .errorCodewords = error_codewords;
        let .matrixWidth = matrix_width;
        let .matrixHeight = matrix_height;
        let .dataRegions = data_regions;
        let .rsBlockData = rs_block_data;
        let .rsBlockError = rs_block_error;
    }

    pub fn  lookup( data_codewords: i32) -> SymbolInfo  {
        return ::lookup(data_codewords, SymbolShapeHint::FORCE_NONE, true);
    }

    pub fn  lookup( data_codewords: i32,  shape: &SymbolShapeHint) -> SymbolInfo  {
        return ::lookup(data_codewords, shape, true);
    }

    pub fn  lookup( data_codewords: i32,  allow_rectangular: bool,  fail: bool) -> SymbolInfo  {
         let shape: SymbolShapeHint =  if allow_rectangular { SymbolShapeHint::FORCE_NONE } else { SymbolShapeHint::FORCE_SQUARE };
        return ::lookup(data_codewords, shape, fail);
    }

    fn  lookup( data_codewords: i32,  shape: &SymbolShapeHint,  fail: bool) -> SymbolInfo  {
        return ::lookup(data_codewords, shape, null, null, fail);
    }

    pub fn  lookup( data_codewords: i32,  shape: &SymbolShapeHint,  min_size: &Dimension,  max_size: &Dimension,  fail: bool) -> SymbolInfo  {
        for  let symbol: SymbolInfo in symbols {
            if shape == SymbolShapeHint::FORCE_SQUARE && symbol.rectangular {
                continue;
            }
            if shape == SymbolShapeHint::FORCE_RECTANGLE && !symbol.rectangular {
                continue;
            }
            if min_size != null && (symbol.get_symbol_width() < min_size.get_width() || symbol.get_symbol_height() < min_size.get_height()) {
                continue;
            }
            if max_size != null && (symbol.get_symbol_width() > max_size.get_width() || symbol.get_symbol_height() > max_size.get_height()) {
                continue;
            }
            if data_codewords <= symbol.dataCapacity {
                return symbol;
            }
        }
        if fail {
            throw IllegalArgumentException::new(format!("Can't find a symbol arrangement that matches the message. Data codewords: {}", data_codewords));
        }
        return null;
    }

    fn  get_horizontal_data_regions(&self) -> i32  {
        match self.data_regions {
              1 => 
                 {
                    return 1;
                }
              2 => 
                 {
                }
              4 => 
                 {
                    return 2;
                }
              16 => 
                 {
                    return 4;
                }
              36 => 
                 {
                    return 6;
                }
            _ => 
                 {
                    throw IllegalStateException::new("Cannot handle this number of data regions");
                }
        }
    }

    fn  get_vertical_data_regions(&self) -> i32  {
        match self.data_regions {
              1 => 
                 {
                }
              2 => 
                 {
                    return 1;
                }
              4 => 
                 {
                    return 2;
                }
              16 => 
                 {
                    return 4;
                }
              36 => 
                 {
                    return 6;
                }
            _ => 
                 {
                    throw IllegalStateException::new("Cannot handle this number of data regions");
                }
        }
    }

    pub fn  get_symbol_data_width(&self) -> i32  {
        return self.get_horizontal_data_regions() * self.matrix_width;
    }

    pub fn  get_symbol_data_height(&self) -> i32  {
        return self.get_vertical_data_regions() * self.matrix_height;
    }

    pub fn  get_symbol_width(&self) -> i32  {
        return self.get_symbol_data_width() + (self.get_horizontal_data_regions() * 2);
    }

    pub fn  get_symbol_height(&self) -> i32  {
        return self.get_symbol_data_height() + (self.get_vertical_data_regions() * 2);
    }

    pub fn  get_codeword_count(&self) -> i32  {
        return self.data_capacity + self.error_codewords;
    }

    pub fn  get_interleaved_block_count(&self) -> i32  {
        return self.data_capacity / self.rs_block_data;
    }

    pub fn  get_data_capacity(&self) -> i32  {
        return self.data_capacity;
    }

    pub fn  get_error_codewords(&self) -> i32  {
        return self.error_codewords;
    }

    pub fn  get_data_length_for_interleaved_block(&self,  index: i32) -> i32  {
        return self.rs_block_data;
    }

    pub fn  get_error_length_for_interleaved_block(&self,  index: i32) -> i32  {
        return self.rs_block_error;
    }

    pub fn  to_string(&self) -> String  {
        return format!("{} data region {}x{}, symbol size {}x{}, symbol data size {}x{}, codewords {}+{}", ( if self.rectangular { "Rectangular Symbol:" } else { "Square Symbol:" }), self.matrix_width, self.matrix_height, self.get_symbol_width(), self.get_symbol_height(), self.get_symbol_data_width(), self.get_symbol_data_height(), self.data_capacity, self.error_codewords);
    }
}

