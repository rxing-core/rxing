use crate::common::Result;
use crate::LuminanceSource;

/// A simple luma8 source for bytes, supports cropping but not rotation
pub struct Luma8LuminanceSource {
    /// image dimension in form (x,y)
    dimensions: (u32, u32),
    /// image origin in the form (x,y)
    origin: (u32, u32),
    /// raw data for luma 8
    data: Vec<u8>,
    /// flag indicating if the underlying data needs to be inverted for use
    inverted: bool,
    /// original dimensions of the data, used to manage crop
    original_dimension: (u32, u32),
}
impl LuminanceSource for Luma8LuminanceSource {
    fn getRow(&self, y: usize) -> Vec<u8> {
        self.data
            .chunks_exact(self.original_dimension.0 as usize)
            .skip(y + self.origin.1 as usize)
            .take(1)
            .flatten()
            .skip(self.origin.0 as usize)
            .take(self.dimensions.0 as usize)
            .map(|byte| Self::invert_if_should(*byte, self.inverted))
            .collect()
    }

    fn getMatrix(&self) -> Vec<u8> {
        self.data
            .iter()
            .skip((self.original_dimension.0 * self.origin.1) as usize)
            .take((self.dimensions.1 * self.original_dimension.0) as usize)
            .collect::<Vec<&u8>>()
            .chunks_exact(self.original_dimension.0 as usize)
            .into_iter()
            .flat_map(|f| {
                f.iter()
                    .skip((self.origin.0) as usize)
                    .take(self.getWidth())
                    .copied()
            }) // flatten this all out
            .copied() // copy it over so that it's u8
            .map(|byte| Self::invert_if_should(byte, self.inverted))
            .collect() // collect into a vec
    }

    fn getWidth(&self) -> usize {
        self.dimensions.0 as usize
    }

    fn getHeight(&self) -> usize {
        self.dimensions.1 as usize
    }

    fn invert(&mut self) {
        self.inverted = !self.inverted;
    }

    fn isCropSupported(&self) -> bool {
        true
    }

    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>> {
        Ok(Box::new(Self {
            dimensions: (width as u32, height as u32),
            origin: (left as u32, top as u32),
            data: self.data.clone(),
            inverted: self.inverted,
            original_dimension: self.original_dimension,
        }))
    }

    fn isRotateSupported(&self) -> bool {
        true
    }

    fn rotateCounterClockwise(&self) -> Result<Box<dyn LuminanceSource>> {
        let mut new_matrix = Self {
            dimensions: self.dimensions,
            origin: self.origin,
            data: self.data.clone(),
            inverted: self.inverted,
            original_dimension: self.original_dimension,
        };
        new_matrix.transpose();
        new_matrix.reverseColumns();
        Ok(Box::new(new_matrix))
    }

    fn rotateCounterClockwise45(&self) -> Result<Box<dyn LuminanceSource>> {
        Err(crate::Exceptions::UnsupportedOperationException(Some(
            "This luminance source does not support rotation by 45 degrees.".to_owned(),
        )))
    }
}

impl Luma8LuminanceSource {
    fn reverseColumns(&mut self) {
        for i in 0..self.getWidth() {
            let mut j = 0;
            let mut k = self.getWidth() - 1;
            while j < k {
                let offset_a = (self.getWidth() * i) + j;
                let offset_b = (self.getWidth() * i) + k;
                self.data.swap(offset_a, offset_b);
                j += 1;
                k -= 1;
            }
        }
    }

    fn transpose(&mut self) {
        for i in 0..self.getHeight() {
            for j in i..self.getWidth() {
                let offset_a = (self.getWidth() * i) + j;
                let offset_b = (self.getWidth() * j) + i;
                self.data.swap(offset_a, offset_b);
            }
        }
    }
}

impl Luma8LuminanceSource {
    pub fn new(source: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            dimensions: (width, height),
            origin: (0, 0),
            data: source,
            inverted: false,
            original_dimension: (width, height),
        }
    }

    #[inline(always)]
    fn invert_if_should(byte: u8, invert: bool) -> u8 {
        if invert {
            255 - byte
        } else {
            byte
        }
    }
}
