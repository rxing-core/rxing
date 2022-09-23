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

use crate::common::BitMatrix;

/**
 * Aztec 2D code representation
 *
 * @author Rustam Abdullaev
 */
pub struct AztecCode {
    compact: bool,
    size: u32,
    layers: u32,
    code_words: u32,
    matrix: BitMatrix,
}

impl AztecCode {
    pub fn new(compact: bool, size: u32, layers: u32, code_words: u32, matrix: BitMatrix) -> Self {
        Self {
            compact,
            size,
            layers,
            code_words,
            matrix,
        }
    }

    /**
     * @return {@code true} if compact instead of full mode
     */
    pub fn isCompact(&self) -> bool {
        self.compact
    }

    pub fn setCompact(&mut self, compact: bool) {
        self.compact = compact;
    }

    /**
     * @return size in pixels (width and height)
     */
    pub fn getSize(&self) -> u32 {
        self.size
    }

    pub fn setSize(&mut self, size: u32) {
        self.size = size;
    }

    /**
     * @return number of levels
     */
    pub fn getLayers(&self) -> u32 {
        self.layers
    }

    pub fn setLayers(&mut self, layers: u32) {
        self.layers = layers;
    }

    /**
     * @return number of data codewords
     */
    pub fn getCodeWords(&self) -> u32 {
        self.code_words
    }

    pub fn setCodeWords(&mut self, code_words: u32) {
        self.code_words = code_words;
    }

    /**
     * @return the symbol image
     */
    pub fn getMatrix(&self) -> &BitMatrix {
        &self.matrix
    }

    pub fn setMatrix(&mut self, matrix: BitMatrix) {
        self.matrix = matrix;
    }
}
