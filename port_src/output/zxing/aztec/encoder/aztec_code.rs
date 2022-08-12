/*
 * Copyright 2013 ZXing authors
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
// package com::google::zxing::aztec::encoder;

/**
 * Aztec 2D code representation
 *
 * @author Rustam Abdullaev
 */
pub struct AztecCode {

     let compact: bool;

     let size: i32;

     let layers: i32;

     let code_words: i32;

     let matrix: BitMatrix;
}

impl AztecCode {

    /**
   * @return {@code true} if compact instead of full mode
   */
    pub fn  is_compact(&self) -> bool  {
        return self.compact;
    }

    pub fn  set_compact(&self,  compact: bool)   {
        self.compact = compact;
    }

    /**
   * @return size in pixels (width and height)
   */
    pub fn  get_size(&self) -> i32  {
        return self.size;
    }

    pub fn  set_size(&self,  size: i32)   {
        self.size = size;
    }

    /**
   * @return number of levels
   */
    pub fn  get_layers(&self) -> i32  {
        return self.layers;
    }

    pub fn  set_layers(&self,  layers: i32)   {
        self.layers = layers;
    }

    /**
   * @return number of data codewords
   */
    pub fn  get_code_words(&self) -> i32  {
        return self.code_words;
    }

    pub fn  set_code_words(&self,  code_words: i32)   {
        self.codeWords = code_words;
    }

    /**
   * @return the symbol image
   */
    pub fn  get_matrix(&self) -> BitMatrix  {
        return self.matrix;
    }

    pub fn  set_matrix(&self,  matrix: &BitMatrix)   {
        self.matrix = matrix;
    }
}

