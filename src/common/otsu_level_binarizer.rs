use std::borrow::Cow;

use image::{DynamicImage, ImageBuffer, Luma};
use once_cell::sync::OnceCell;

use crate::common::Result;
use crate::{Binarizer, Exceptions, LuminanceSource};

use super::{BitArray, BitMatrix};

pub struct OtsuLevelBinarizer<LS: LuminanceSource> {
    width: usize,
    height: usize,
    source: LS,
    black_matrix: OnceCell<BitMatrix>,
    black_row_cache: Vec<OnceCell<BitArray>>,
}

impl<LS: LuminanceSource> OtsuLevelBinarizer<LS> {
    fn generate_threshold_matrix<LS2: LuminanceSource>(source: &LS2) -> Result<BitMatrix> {
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

    pub fn new(source: LS) -> Self {
        Self {
            width: source.get_width(),
            height: source.get_height(),
            black_row_cache: vec![OnceCell::default(); source.get_height()],
            source,
            black_matrix: OnceCell::new(),
        }
    }
}

impl<LS: LuminanceSource> Binarizer for OtsuLevelBinarizer<LS> {
    type Source = LS;

    fn get_luminance_source(&self) -> &LS {
        &self.source
    }

    fn get_black_row(&self, y: usize) -> Result<Cow<BitArray>> {
        let row = self.black_row_cache[y].get_or_try_init(|| {
            let matrix = self.get_black_matrix()?;
            Ok(matrix.getRow(y as u32))
        })?;

        Ok(Cow::Borrowed(row))
    }

    fn get_black_matrix(&self) -> Result<&BitMatrix> {
        let matrix = self
            .black_matrix
            .get_or_try_init(|| Self::generate_threshold_matrix(&self.source))?;
        Ok(matrix)
    }

    fn create_binarizer(&self, source: LS) -> Self {
        Self::new(source)
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}
