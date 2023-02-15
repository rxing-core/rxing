use std::{borrow::Cow, rc::Rc};

use image::{DynamicImage, ImageBuffer, Luma};
use once_cell::sync::OnceCell;

use crate::{Binarizer, Exceptions, LuminanceSource};

use super::{BitArray, BitMatrix};

pub struct OtsuLevelBinarizer {
    width: usize,
    height: usize,
    source: Box<dyn LuminanceSource>,
    black_matrix: OnceCell<BitMatrix>,
    black_row_cache: Vec<OnceCell<BitArray>>,
}

impl OtsuLevelBinarizer {
    fn generate_threshold_matrix(source: &dyn LuminanceSource) -> Result<BitMatrix, Exceptions> {
        let image_buffer = {
            let Some(buff) : Option<ImageBuffer<Luma<u8>,Vec<u8>>> = ImageBuffer::from_vec(source.getWidth() as u32, source.getHeight() as u32, source.getMatrix()) else {
                return Err(Exceptions::illegalArgumentEmpty())
            };
            buff
        };

        let otsu_level = imageproc::contrast::otsu_level(&image_buffer);

        let filtered_iamge = imageproc::contrast::threshold(&image_buffer, otsu_level);

        let dynamic_filtered = DynamicImage::from(filtered_iamge);

        dynamic_filtered.try_into()
    }

    pub fn new(source: Box<dyn LuminanceSource>) -> Self {
        Self {
            width: source.getWidth(),
            height: source.getHeight(),
            black_row_cache: vec![OnceCell::default(); source.getHeight()],
            source,
            black_matrix: OnceCell::new(),
        }
    }
}

impl Binarizer for OtsuLevelBinarizer {
    fn getLuminanceSource(&self) -> &Box<dyn crate::LuminanceSource> {
        &self.source
    }

    fn getBlackRow(
        &self,
        y: usize,
    ) -> Result<std::borrow::Cow<super::BitArray>, crate::Exceptions> {
        let row = self.black_row_cache[y].get_or_try_init(|| {
            let matrix = self.getBlackMatrix()?;
            Ok(matrix.getRow(y as u32))
        })?;

        Ok(Cow::Borrowed(row))
    }

    fn getBlackMatrix(&self) -> Result<&super::BitMatrix, crate::Exceptions> {
        let matrix = self
            .black_matrix
            .get_or_try_init(|| Self::generate_threshold_matrix(self.source.as_ref()))?;
        Ok(matrix)
    }

    fn createBinarizer(
        &self,
        source: Box<dyn crate::LuminanceSource>,
    ) -> std::rc::Rc<dyn Binarizer> {
        Rc::new(Self::new(source))
    }

    fn getWidth(&self) -> usize {
        self.width
    }

    fn getHeight(&self) -> usize {
        self.height
    }
}
