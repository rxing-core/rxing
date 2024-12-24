use std::borrow::Cow;

use crate::{Binarizer, LuminanceSource};

use super::{BitArray, BitMatrix, LineOrientation};

pub struct BitMatrixSource {
    base_bitmatrix: BitMatrix,
    byte_array: Box<[u8]>,
}

impl BitMatrixSource {
    pub fn new(base_bitmatrix: BitMatrix) -> Self {
        let mut tmp =
            Vec::with_capacity((base_bitmatrix.getWidth() * base_bitmatrix.getHeight()) as usize);
        for y in 0..base_bitmatrix.getHeight() {
            for x in 0..base_bitmatrix.getWidth() {
                tmp.push(if base_bitmatrix.get(x, y) { 255 } else { 0 });
            }
        }
        Self {
            base_bitmatrix,
            byte_array: tmp.into_boxed_slice(),
        }
    }
}

impl LuminanceSource for BitMatrixSource {
    fn get_row(&self, y: usize) -> Vec<u8> {
        self.base_bitmatrix.getRow(y as u32).into()
    }

    fn get_column(&self, x: usize) -> Vec<u8> {
        self.base_bitmatrix.getCol(x as u32).into()
    }

    fn get_matrix(&self) -> Vec<u8> {
        self.byte_array.to_vec()
    }

    fn get_width(&self) -> usize {
        self.base_bitmatrix.getWidth() as usize
    }

    fn get_height(&self) -> usize {
        self.base_bitmatrix.getHeight() as usize
    }

    fn invert(&mut self) {
        for byte in self.byte_array.iter_mut() {
            match byte {
                0 => *byte = 255,
                255 => *byte = 0,
                _ => unreachable!(),
            }
        }
    }

    fn get_luma8_point(&self, x: usize, y: usize) -> u8 {
        if !self.base_bitmatrix.get(x as u32, y as u32) {
            0
        } else {
            255
        }
    }
}

pub struct BitMatrixBinarizer(BitMatrixSource);
impl Binarizer for BitMatrixBinarizer {
    type Source = BitMatrixSource;

    fn get_luminance_source(&self) -> &Self::Source {
        &self.0
    }

    fn get_black_row(&self, y: usize) -> super::Result<std::borrow::Cow<BitArray>> {
        Ok(Cow::Owned(self.0.get_row(y).into()))
    }

    fn get_black_row_from_matrix(
        &self,
        y: usize,
    ) -> super::Result<std::borrow::Cow<super::BitArray>> {
        self.get_black_row(y)
    }

    fn get_black_matrix(&self) -> super::Result<&super::BitMatrix> {
        Ok(&self.0.base_bitmatrix)
    }

    fn get_black_line(
        &self,
        l: usize,
        lt: super::LineOrientation,
    ) -> super::Result<std::borrow::Cow<super::BitArray>> {
        match lt {
            LineOrientation::Row => self.get_black_row(l),
            LineOrientation::Column => Ok(Cow::Owned(self.0.get_column(l).into())),
        }
    }

    fn create_binarizer(&self, source: Self::Source) -> Self
    where
        Self: Sized,
    {
        Self(source)
    }

    fn get_width(&self) -> usize {
        self.0.get_width()
    }

    fn get_height(&self) -> usize {
        self.0.get_height()
    }
}
