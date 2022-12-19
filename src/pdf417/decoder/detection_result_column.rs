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

use std::fmt::Display;

use super::{BoundingBox, Codeword};

 const MAX_NEARBY_DISTANCE :u32 = 5;

/**
 * @author Guenther Grau
 */
pub struct DetectionRXingResultColumn<'a> {
   boundingBox: BoundingBox<'a>,
    codewords:Vec<Option<Codeword>>,
}

impl<'a> DetectionRXingResultColumn<'_> {

  pub fn new( boundingBox:&'a BoundingBox) -> DetectionRXingResultColumn<'a> {
    DetectionRXingResultColumn {
        boundingBox: BoundingBox::from_other(boundingBox),
        codewords: vec![None;(boundingBox.getMaxY() - boundingBox.getMinY() + 1) as usize],
    }
    // this.boundingBox = new BoundingBox(boundingBox);
    // codewords = new Codeword[boundingBox.getMaxY() - boundingBox.getMinY() + 1];
  }

  pub fn getCodewordNearby(&self, imageRow:u32) -> &Option<Codeword> {
    let mut codeword = self.getCodeword(imageRow);
    if codeword.is_some() {
      return codeword;
    }
    for i in 1..MAX_NEARBY_DISTANCE as usize{
    // for (int i = 1; i < MAX_NEARBY_DISTANCE; i++) {
      let mut nearImageRow = self.imageRowToCodewordIndex(imageRow) - i;
      if nearImageRow >= 0 {
        codeword = &self.codewords[nearImageRow];
        if codeword.is_some() {
          return codeword;
        }
      }
      nearImageRow = self.imageRowToCodewordIndex(imageRow) + i ;
      if nearImageRow < self.codewords.len() {
        codeword = &self.codewords[nearImageRow];
        if codeword.is_some() {
          return codeword;
        }
      }
    }
    &None
  }

  pub fn imageRowToCodewordIndex(&self,  imageRow:u32) -> usize{
     (imageRow - self.boundingBox.getMinY()) as usize
  }

  pub fn setCodeword(&mut self,  imageRow:u32,  codeword:Codeword) {
    let pos = self.imageRowToCodewordIndex(imageRow);
    self.codewords[pos] = Some(codeword);
  }

  pub fn getCodeword(&self,  imageRow:u32) -> &Option<Codeword>{
    &self. codewords[self.imageRowToCodewordIndex(imageRow)]
  }

  pub fn getBoundingBox(&self) -> &BoundingBox {
    &self. boundingBox
  }

  pub fn getCodewords(&self) -> &[Option<Codeword>] {
    &self.codewords
  }



}

impl Display for DetectionRXingResultColumn<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }

    // @Override
    // public String toString() {
    //   try (Formatter formatter = new Formatter()) {
    //     int row = 0;
    //     for (Codeword codeword : codewords) {
    //       if (codeword == null) {
    //         formatter.format("%3d:    |   %n", row++);
    //         continue;
    //       }
    //       formatter.format("%3d: %3d|%3d%n", row++, codeword.getRowNumber(), codeword.getValue());
    //     }
    //     return formatter.toString();
    //   }
    // }
}