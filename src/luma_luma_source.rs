use crate::LuminanceSource;

pub struct Luma8LuminanceSource(u32, u32, Vec<u8>, bool);
impl LuminanceSource for Luma8LuminanceSource {
    fn getRow(&self, y: usize) -> Vec<u8> {
        self.2
            .chunks_exact(y * self.0 as usize)
            .skip(y)
            .take(1)
            .flatten()
            .map(|byte| Self::invert_if_should(*byte, self.3))
            .collect()
    }

    fn getMatrix(&self) -> Vec<u8> {
        self.2
            .iter()
            .map(|byte| Self::invert_if_should(*byte, self.3))
            .collect()
    }

    fn getWidth(&self) -> usize {
        self.0 as usize
    }

    fn getHeight(&self) -> usize {
        self.1 as usize
    }

    fn invert(&mut self) {
        self.3 = !self.3;
    }

    fn isCropSupported(&self) -> bool {
        false
    }

    fn crop(
        &self,
        _left: usize,
        _top: usize,
        _width: usize,
        _height: usize,
    ) -> Result<Box<dyn LuminanceSource>, crate::Exceptions> {
        Err(crate::Exceptions::UnsupportedOperationException(Some(
            "This luminance source does not support cropping.".to_owned(),
        )))
    }
}
impl Luma8LuminanceSource {
    pub fn new(source: Vec<u8>, width: u32, height: u32) -> Self {
        Self(width, height, source, false)
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
