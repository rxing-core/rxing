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

use std::{fmt::Display, rc::Rc};

use super::{BoundingBox, Codeword, DetectionRXingResultRowIndicatorColumn};

const MAX_NEARBY_DISTANCE: u32 = 5;

pub trait DetectionRXingResultColumnTrait {
    fn new(boundingBox: Rc<BoundingBox>) -> DetectionRXingResultColumn
    where
        Self: Sized;
    fn new_with_is_left(boundingBox: Rc<BoundingBox>, isLeft: bool) -> DetectionRXingResultColumn
    where
        Self: Sized;
    fn getCodewordNearby(&self, imageRow: u32) -> &Option<Codeword>;
    fn imageRowToCodewordIndex(&self, imageRow: u32) -> usize;
    fn setCodeword(&mut self, imageRow: u32, codeword: Codeword);
    fn getCodeword(&self, imageRow: u32) -> &Option<Codeword>;
    fn getBoundingBox(&self) -> &BoundingBox;
    fn getCodewords(&self) -> &[Option<Codeword>];
    fn getCodewordsMut(&mut self) -> &mut [Option<Codeword>];
    fn as_indicator_row(&mut self) -> &mut dyn DetectionRXingResultRowIndicatorColumn;
}

/**
 * @author Guenther Grau
 */
#[derive(Clone)]
pub struct DetectionRXingResultColumn {
    boundingBox: BoundingBox,
    codewords: Vec<Option<Codeword>>,
    pub(super) isLeft: Option<bool>,
}

impl DetectionRXingResultColumnTrait for DetectionRXingResultColumn {
    fn new(boundingBox: Rc<BoundingBox>) -> DetectionRXingResultColumn {
        DetectionRXingResultColumn {
            boundingBox: BoundingBox::from_other(boundingBox.clone()),
            codewords: vec![None; (boundingBox.getMaxY() - boundingBox.getMinY() + 1) as usize],
            isLeft: None,
        }
        // this.boundingBox = new BoundingBox(boundingBox);
        // codewords = new Codeword[boundingBox.getMaxY() - boundingBox.getMinY() + 1];
    }

    fn new_with_is_left(boundingBox: Rc<BoundingBox>, isLeft: bool) -> DetectionRXingResultColumn {
        DetectionRXingResultColumn {
            boundingBox: BoundingBox::from_other(boundingBox.clone()),
            codewords: vec![None; (boundingBox.getMaxY() - boundingBox.getMinY() + 1) as usize],
            isLeft: Some(isLeft),
        }
    }

    fn getCodewordNearby(&self, imageRow: u32) -> &Option<Codeword> {
        let mut codeword = self.getCodeword(imageRow);
        if codeword.is_some() {
            return codeword;
        }
        for i in 1..MAX_NEARBY_DISTANCE as usize {
            // for (int i = 1; i < MAX_NEARBY_DISTANCE; i++) {
            let mut nearImageRow = self.imageRowToCodewordIndex(imageRow) - i;
            if nearImageRow >= 0 {
                codeword = &self.codewords[nearImageRow];
                if codeword.is_some() {
                    return codeword;
                }
            }
            nearImageRow = self.imageRowToCodewordIndex(imageRow) + i;
            if nearImageRow < self.codewords.len() {
                codeword = &self.codewords[nearImageRow];
                if codeword.is_some() {
                    return codeword;
                }
            }
        }
        &None
    }

    fn imageRowToCodewordIndex(&self, imageRow: u32) -> usize {
        (imageRow - self.boundingBox.getMinY()) as usize
    }

    fn setCodeword(&mut self, imageRow: u32, codeword: Codeword) {
        let pos = self.imageRowToCodewordIndex(imageRow);
        self.codewords[pos] = Some(codeword);
    }

    fn getCodeword(&self, imageRow: u32) -> &Option<Codeword> {
        &self.codewords[self.imageRowToCodewordIndex(imageRow)]
    }

    fn getBoundingBox(&self) -> &BoundingBox {
        &self.boundingBox
    }

    fn getCodewords(&self) -> &[Option<Codeword>] {
        &self.codewords
    }

    fn getCodewordsMut(&mut self) -> &mut [Option<Codeword>] {
        &mut self.codewords
    }

    fn as_indicator_row(&mut self) -> &mut dyn DetectionRXingResultRowIndicatorColumn {
        self as &mut dyn DetectionRXingResultRowIndicatorColumn
    }
    // pub fn as_row_indicator(&self) -> DetectionRXingResultRowIndicatorColumn {
    //     DetectionRXingResultRowIndicatorColumn::new(&self.boundingBox, false)
    // }
}

impl Display for DetectionRXingResultColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.isLeft.is_some() {
            write!(f, "IsLeft: {} \n", self.isLeft.as_ref().unwrap());
        }
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
