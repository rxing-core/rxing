use std::borrow::Cow;

use image::{DynamicImage, ImageBuffer, Luma};
use once_cell::sync::OnceCell;

use crate::{Binarizer, Exceptions, LuminanceSource};

use super::{BitArray, BitMatrix, Result};

/// The `AdaptiveThresholdBinarizer` works using the `imageproc::contrast::adaptive_threshold`
/// function. This is an alternative to the other Binarizers available. It is largely
/// untested.
pub struct AdaptiveThresholdBinarizer<LS: LuminanceSource> {
    source: LS,
    matrix: OnceCell<BitMatrix>,
    radius: u32,
}

impl<LS: LuminanceSource> AdaptiveThresholdBinarizer<LS> {
    pub fn new(source: LS, radius: u32) -> Self {
        Self {
            source,
            radius,
            matrix: OnceCell::new(),
        }
    }
}

impl<LS: LuminanceSource> AdaptiveThresholdBinarizer<LS> {
    fn buildMatrix(&self) -> Result<BitMatrix> {
        let image_buffer = {
            let Some(buff): Option<ImageBuffer<Luma<u8>, Vec<u8>>> = ImageBuffer::from_vec(
                self.source.get_width() as u32,
                self.source.get_height() as u32,
                self.source.get_matrix(),
            ) else {
                return Err(Exceptions::ILLEGAL_ARGUMENT);
            };
            buff
        };

        let filtered_iamge =
            imageproc::contrast::adaptive_threshold(&image_buffer, self.radius.into());

        let dynamic_filtered = DynamicImage::from(filtered_iamge);

        dynamic_filtered.try_into()
    }
}

impl<LS: LuminanceSource> Binarizer for AdaptiveThresholdBinarizer<LS> {
    type Source = LS;

    fn get_luminance_source(&self) -> &Self::Source {
        &self.source
    }

    fn get_black_row(&self, y: usize) -> Result<Cow<BitArray>> {
        let matrix = self.matrix.get_or_try_init(|| self.buildMatrix())?;
        Ok(Cow::Owned(matrix.getRow(y as u32)))
    }

    fn get_black_matrix(&self) -> super::Result<&BitMatrix> {
        self.matrix.get_or_try_init(|| self.buildMatrix())
    }

    fn get_black_line(&self, l: usize, lt: super::LineOrientation) -> Result<Cow<BitArray>> {
        match lt {
            super::LineOrientation::Row => self.get_black_row(l),
            super::LineOrientation::Column => {
                let matrix = self.matrix.get_or_try_init(|| self.buildMatrix())?;
                Ok(Cow::Owned(matrix.getCol(l as u32)))
            }
        }
    }

    fn create_binarizer(&self, source: Self::Source) -> Self
    where
        Self: Sized,
    {
        Self {
            source: source,
            matrix: OnceCell::new(),
            radius: self.radius,
        }
    }

    fn get_width(&self) -> usize {
        self.source.get_width()
    }

    fn get_height(&self) -> usize {
        self.source.get_height()
    }
}
