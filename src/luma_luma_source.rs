use crate::LuminanceSource;

pub struct Luma8LuminanceSource((u32, u32), (u32, u32), Vec<u8>, bool, (u32, u32));
impl LuminanceSource for Luma8LuminanceSource {
    fn getRow(&self, y: usize) -> Vec<u8> {
        self.2
            .chunks_exact(y * self.0 .0 as usize)
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
        self.0 .0 as usize
    }

    fn getHeight(&self) -> usize {
        self.0 .1 as usize
    }

    fn invert(&mut self) {
        self.3 = !self.3;
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
        Ok(Box::new(Self(
            (width as u32, height as u32),
            (left as u32, top as u32),
            self.2.clone(),
            self.3,
            self.4,
        )))
    }
}
impl Luma8LuminanceSource {
    pub fn new(source: Vec<u8>, width: u32, height: u32) -> Self {
        Self((width, height), (0, 0), source, false, (width, height))
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
