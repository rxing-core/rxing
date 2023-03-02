use std::{borrow::Cow, rc::Rc};

use image::{DynamicImage, ImageBuffer, Luma};
use once_cell::sync::OnceCell;

use crate::common::Result;
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
    fn generate_threshold_matrix(source: &dyn LuminanceSource) -> Result<BitMatrix> {
        let image_buffer = {
            let Some(buff) : Option<ImageBuffer<Luma<u8>,Vec<u8>>> = ImageBuffer::from_vec(source.get_width() as u32, source.get_height() as u32, source.get_matrix()) else {
                return Err(Exceptions::ILLEGAL_ARGUMENT)
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
            width: source.get_width(),
            height: source.get_height(),
            black_row_cache: vec![OnceCell::default(); source.get_height()],
            source,
            black_matrix: OnceCell::new(),
        }
    }
}

impl Binarizer for OtsuLevelBinarizer {
    fn get_luminance_source(&self) -> &Box<dyn crate::LuminanceSource> {
        &self.source
    }

    fn get_black_row(&self, y: usize) -> Result<std::borrow::Cow<super::BitArray>> {
        let row = self.black_row_cache[y].get_or_try_init(|| {
            let matrix = self.get_black_matrix()?;
            Ok(matrix.getRow(y as u32))
        })?;

        Ok(Cow::Borrowed(row))
    }

    fn get_black_matrix(&self) -> Result<&super::BitMatrix> {
        let matrix = self
            .black_matrix
            .get_or_try_init(|| Self::generate_threshold_matrix(self.source.as_ref()))?;
        Ok(matrix)
    }

    fn create_binarizer(
        &self,
        source: Box<dyn crate::LuminanceSource>,
    ) -> std::rc::Rc<dyn Binarizer> {
        Rc::new(Self::new(source))
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}
