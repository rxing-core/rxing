use std::borrow::Cow;

use crate::LuminanceSource;
use crate::common::Result;

/// A simple luma8 source for bytes, supports cropping but not rotation
#[derive(Debug, Clone)]
pub struct Luma8LuminanceSource {
    /// image dimension in form (x,y)
    dimensions: (u32, u32),
    /// raw data for luma 8
    data: Box<[u8]>,
    /// flag indicating if the underlying data needs to be inverted for use
    inverted: bool,
}
impl LuminanceSource for Luma8LuminanceSource {
    const SUPPORTS_CROP: bool = true;
    const SUPPORTS_ROTATION: bool = true;

    fn get_row(&'_ self, y: usize) -> Option<Cow<'_, [u8]>> {
        let chunk_size = self.dimensions.0 as usize;
        let row_skip = y; //self.origin.1 as usize;
        let column_skip = 0; //self.origin.0 as usize;
        let column_take = self.dimensions.0 as usize;

        let data_start = (chunk_size * row_skip) + column_skip;
        let data_end = (chunk_size * row_skip) + column_skip + column_take;

        if self.inverted {
            Some(Cow::Owned(self.invert_block_of_bytes(Vec::from(
                &self.data[data_start..data_end],
            ))))
        } else {
            Some(Cow::Borrowed(&self.data[data_start..data_end]))
        }
    }

    fn get_column(&self, x: usize) -> Vec<u8> {
        self.data
            .chunks_exact(self.dimensions.0 as usize)
            // .skip(self.origin.1 as usize)
            .fold(Vec::with_capacity(self.get_height()), |mut acc, e| {
                let byte = e[x];
                acc.push(Self::invert_if_should(byte, self.inverted));
                acc
            })
    }

    fn get_matrix(&self) -> Cow<'_, [u8]> {
        Cow::Borrowed(&self.data)
    }

    fn get_width(&self) -> usize {
        self.dimensions.0 as usize
    }

    fn get_height(&self) -> usize {
        self.dimensions.1 as usize
    }

    fn invert(&mut self) {
        self.inverted = !self.inverted;
    }

    fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> Result<Self> {
        if left + width > self.get_width() || top + height > self.get_height() {
            return Err(crate::Exceptions::illegal_argument_with(
                "Crop rectangle does not fit within image data.",
            ));
        }

        let dimensions = (width as u32, height as u32);
        // origin: (self.origin.0 + left as u32, self.origin.1 + top as u32),
        let data: Box<[u8]> = self
            .data
            .chunks_exact(self.dimensions.0 as usize)
            .skip(top)
            .take(height)
            .flat_map(|f| f.iter().skip(left).take(width))
            .copied()
            .collect();
        let inverted = self.inverted;

        if width * height != data.len() {
            // print!("Crop dimensions do not match the data length. width: {}, height: {}, data.len(): {}", width, height, data.len());
            return Err(crate::Exceptions::illegal_argument_with(
                "Crop dimensions do not match the data length.",
            ));
        }

        Ok(Self {
            dimensions,
            data,
            inverted,
        })
    }

    fn rotate_counter_clockwise(&self) -> Result<Self> {
        let mut new_matrix = Self {
            dimensions: self.dimensions,
            data: self.data.clone(),
            inverted: self.inverted,
        };
        new_matrix.transpose();
        new_matrix.reverseColumns();
        Ok(new_matrix)
    }

    fn rotate_counter_clockwise_45(&self) -> Result<Self> {
        Err(crate::Exceptions::unsupported_operation_with(
            "This luminance source does not support rotation by 45 degrees.",
        ))
    }

    fn get_luma8_point(&self, column: usize, row: usize) -> u8 {
        let chunk_size = self.dimensions.0 as usize;
        let row_skip = row; //row + self.origin.1 as usize;
        let column_skip = 0; //self.origin.0 as usize;

        let data_start = (chunk_size * row_skip) + column_skip;
        let data_point = data_start + column;

        Self::invert_if_should(self.data[data_point], self.inverted)
    }
}

impl Luma8LuminanceSource {
    fn reverseColumns(&mut self) {
        for col in 0..(self.get_width()) {
            let mut a = 0;
            let mut b = self.get_height() - 1;
            while a < b {
                let offset_a = a * self.get_width() + col;
                let offset_b = b * self.get_width() + col;
                self.data.swap(offset_a, offset_b);

                a += 1;
                b -= 1;
            }
        }
    }

    fn transpose_square(&mut self) {
        for i in 0..self.get_height() {
            for j in i..self.get_width() {
                let offset_a = (self.get_width() * i) + j;
                let offset_b = (self.get_width() * j) + i;
                self.data.swap(offset_a, offset_b);
            }
        }
    }

    fn transpose_rect(&mut self) {
        let mut new_data = vec![0; self.data.len()];
        let new_dim = (self.dimensions.1, self.dimensions.0);
        for i in 0..self.get_height() {
            for j in 0..self.get_width() {
                let offset_a = (self.get_width() * i) + j;
                let offset_b = (self.get_height() * j) + i;
                new_data[offset_b] = self.data[offset_a];
            }
        }
        self.data = new_data.into_boxed_slice();
        self.dimensions = new_dim;
    }

    fn transpose(&mut self) {
        if self.get_width() == self.get_height() {
            self.transpose_square()
        } else {
            self.transpose_rect()
        }
        // print_matrix(&self.data, self.get_width(), self.get_height());
    }
}

impl Luma8LuminanceSource {
    pub fn new(source: Vec<u8>, width: u32, height: u32) -> Result<Self> {
        if width * height != source.len() as u32 {
            return Err(crate::Exceptions::illegal_argument_with(
                "Dimensions do not match the data length.",
            ));
        }
        Ok(Self {
            dimensions: (width, height),
            data: source.into_boxed_slice(),
            inverted: false,
        })
    }

    pub fn with_empty_image(width: usize, height: usize) -> Self {
        Self {
            dimensions: (width as u32, height as u32),
            data: vec![0u8; width * height].into_boxed_slice(),
            inverted: false,
        }
    }

    pub fn get_matrix_mut(&mut self) -> &mut Box<[u8]> {
        &mut self.data
    }

    #[inline(always)]
    fn invert_if_should(byte: u8, invert: bool) -> u8 {
        if invert { 255 - byte } else { byte }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Luma8LuminanceSource, LuminanceSource};

    #[test]
    fn test_rotate() {
        let src_square = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let src_rect = vec![0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0];

        let square = Luma8LuminanceSource::new(src_square, 3, 3).unwrap();
        let rect_tall = Luma8LuminanceSource::new(src_rect.clone(), 3, 4).unwrap();
        let rect_wide = Luma8LuminanceSource::new(src_rect, 4, 3).unwrap();

        let rotated_square = square.rotate_counter_clockwise().expect("rotate");
        // print_matrix(&src_rect, 4, 3);
        let rotated_wide_rect = rect_wide.rotate_counter_clockwise().expect("rotate");
        // print_matrix(&src_rect, 3, 4);
        let rotated_tall_rect = rect_tall.rotate_counter_clockwise().expect("rotate");

        assert_eq!(rotated_square.dimensions, square.dimensions);
        assert_eq!(
            rotated_tall_rect.dimensions,
            (rect_tall.dimensions.1, rect_tall.dimensions.0)
        );
        assert_eq!(
            rotated_wide_rect.dimensions,
            (rect_wide.dimensions.1, rect_wide.dimensions.0)
        );

        assert_eq!(
            rotated_square.data,
            vec![3, 6, 9, 2, 5, 8, 1, 4, 7].into_boxed_slice()
        );

        assert_eq!(
            rotated_wide_rect.data,
            vec![1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1].into_boxed_slice()
        );

        assert_eq!(
            rotated_tall_rect.data,
            vec![0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0].into_boxed_slice()
        );
    }
}
