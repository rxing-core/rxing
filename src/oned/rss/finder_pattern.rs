/*
 * Copyright 2009 ZXing authors
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

use std::hash::Hash;

use crate::{point, Point};

/**
 * Encapsulates an RSS barcode finder pattern, including its start/end position and row.
 */
#[derive(Clone)]
pub struct FinderPattern {
    value: u32,
    startEnd: [usize; 2],
    resultPoints: Vec<Point>,
}

impl FinderPattern {
    pub fn new(value: u32, startEnd: [usize; 2], start: usize, end: usize, rowNumber: u32) -> Self {
        Self {
            value,
            startEnd,
            resultPoints: vec![
                point(start as f32, rowNumber as f32),
                point(end as f32, rowNumber as f32),
            ],
        }
    }

    pub const fn getValue(&self) -> u32 {
        self.value
    }

    pub fn getStartEnd(&self) -> &[usize] {
        &self.startEnd
    }

    #[cfg(all(test, feature = "image"))]
    pub(crate) fn getStartEndMut(&mut self) -> &mut [usize] {
        &mut self.startEnd
    }

    pub fn getPoints(&self) -> &[Point] {
        &self.resultPoints
    }
}

impl PartialEq for FinderPattern {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for FinderPattern {}
impl Hash for FinderPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
