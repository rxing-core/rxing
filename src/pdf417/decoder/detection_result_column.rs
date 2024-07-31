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

use super::{BoundingBox, Codeword, DetectionRXingResultRowIndicatorColumn};

const MAX_NEARBY_DISTANCE: u32 = 5;

pub trait DetectionRXingResultColumnTrait {
    fn new_column(boundingBox: &BoundingBox) -> DetectionRXingResultColumn
    where
        Self: Sized;
    fn new_with_is_left(boundingBox: &BoundingBox, isLeft: bool) -> DetectionRXingResultColumn
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
    fn new_column(boundingBox: &BoundingBox) -> DetectionRXingResultColumn {
        DetectionRXingResultColumn {
            boundingBox: BoundingBox::from_other(boundingBox),
            codewords: vec![None; (boundingBox.getMaxY() - boundingBox.getMinY() + 1) as usize],
            isLeft: None,
        }
    }

    fn new_with_is_left(boundingBox: &BoundingBox, isLeft: bool) -> DetectionRXingResultColumn {
        DetectionRXingResultColumn {
            boundingBox: BoundingBox::from_other(boundingBox),
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
            let mut nearImageRow = self.imageRowToCodewordIndex(imageRow) as isize - i as isize;
            if nearImageRow >= 0 {
                codeword = &self.codewords[nearImageRow as usize];
                if codeword.is_some() {
                    return codeword;
                }
            }
            nearImageRow = self.imageRowToCodewordIndex(imageRow) as isize + i as isize;
            if nearImageRow < self.codewords.len() as isize {
                codeword = &self.codewords[nearImageRow as usize];
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
}

impl Display for DetectionRXingResultColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(isLeft) = self.isLeft {
            writeln!(f, "IsLeft: {isLeft} ")?;
        }
        for (row, codeword) in self.codewords.iter().enumerate() {
            if let Some(codeword) = codeword {
                writeln!(
                    f,
                    "{:3}: {:3}|{:3}",
                    row,
                    codeword.getRowNumber(),
                    codeword.getValue()
                )?;
            } else {
                writeln!(f, "{row:3}:    |   ")?;
            }
        }
        write!(f, "")
    }
}
