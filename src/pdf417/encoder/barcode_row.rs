/*
 * Copyright 2011 ZXing authors
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

/**
 * @author Jacob Haynes
 */
pub struct BarcodeRow {
    row: Vec<u8>,
    //A tacker for position in the bar
    currentLocation: usize,
}

impl BarcodeRow {
    /**
     * Creates a Barcode row of the width
     */
    pub fn new(width: usize) -> Self {
        Self {
            row: vec![0; width],
            currentLocation: 0,
        }
    }

    /**
     * Sets a specific location in the bar
     *
     * @param x The location in the bar
     * @param value Black if true, white if false;
     */
    pub fn set(&mut self, x: usize, value: u8) {
        self.row[x] = value
    }

    /**
     * Sets a specific location in the bar
     *
     * @param x The location in the bar
     * @param black Black if true, white if false;
     */
    fn set_bool(&mut self, x: usize, black: bool) {
        self.row[x] = if black { 1 } else { 0 }
    }

    /**
     * @param black A boolean which is true if the bar black false if it is white
     * @param width How many spots wide the bar is.
     */
    pub fn addBar(&mut self, black: bool, width: usize) {
        for _ii in 0..width {
            // for (int ii = 0; ii < width; ii++) {
            self.set_bool(self.currentLocation, black);
            self.currentLocation += 1;
        }
    }

    /**
     * This function scales the row
     *
     * @param scale How much you want the image to be scaled, must be greater than or equal to 1.
     * @return the scaled row
     */
    pub fn getScaledRow(&self, scale: usize) -> Vec<u8> {
        let mut output = vec![0; self.row.len() * scale];
        for i in 0..output.len() {
            // for (int i = 0; i < output.length; i++) {
            output[i] = self.row[i / scale];
        }

        output
    }
}
