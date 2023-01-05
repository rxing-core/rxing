use crate::LuminanceSource;

pub struct Luma8LuminanceSource {
    dimensions: (u32, u32),
    origin: (u32, u32),
    data: Vec<u8>,
    inverted: bool,
    original_dimension: (u32, u32),
}
impl LuminanceSource for Luma8LuminanceSource {
    fn getRow(&self, y: usize) -> Vec<u8> {
        self.data
            .chunks_exact(y * self.dimensions.0 as usize)
            .skip(y)
            .take(1)
            .flatten()
            .map(|byte| Self::invert_if_should(*byte, self.inverted))
            .collect()
    }

    fn getMatrix(&self) -> Vec<u8> {
        self.data
            .iter()
            .map(|byte| Self::invert_if_should(*byte, self.inverted))
            .collect()
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
        false
    }

    fn crop(
        &self,
        left: usize,
        top: usize,
        width: usize,
        height: usize,
    ) -> Result<Box<dyn LuminanceSource>, crate::Exceptions> {
        Ok(Box::new(Self {
            dimensions: (width as u32, height as u32),
            origin: (left as u32, top as u32),
            data: self.data.clone(),
            inverted: self.inverted,
            original_dimension: self.original_dimension,
        }))
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
