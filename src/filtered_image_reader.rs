use std::collections::HashMap;

use crate::{Binarizer, BinaryBitmap, Exceptions, Luma8LuminanceSource, LuminanceSource, Reader};

use crate::common::{BitMatrix, HybridBinarizer, Result};

pub const DEFAULT_DOWNSCALE_THRESHHOLD: usize = 500;
pub const DEFAULT_DOWNSCALE_FACTOR: usize = 3;

/// Passed image data is ignored, only the image data
pub struct FilteredImageReader<R: Reader>(R);

impl<R: Reader> FilteredImageReader<R> {
    pub fn new(reader: R) -> Self {
        Self(reader)
    }
}

impl<R: Reader> Reader for FilteredImageReader<R> {
    fn decode<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
    ) -> crate::common::Result<crate::RXingResult> {
        self.decode_with_hints(image, &HashMap::default())
    }

    fn decode_with_hints<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &crate::DecodingHintDictionary,
    ) -> crate::common::Result<crate::RXingResult> {
        let pyramids = LumImagePyramid::new(
            Luma8LuminanceSource::new(
                image.get_source().get_matrix(),
                image.get_source().get_width() as u32,
                image.get_source().get_height() as u32,
            ),
            DEFAULT_DOWNSCALE_THRESHHOLD,
            DEFAULT_DOWNSCALE_FACTOR,
        )
        .ok_or(Exceptions::ILLEGAL_ARGUMENT)?;
        for layer in pyramids.layers {
            let mut b = BinaryBitmap::new(HybridBinarizer::new(layer));
            for close in [false, true] {
                if close {
                    let Ok(_) = b.close() else {
                        continue;
                    };
                }
                if let Ok(res) = self.0.decode_with_hints(&mut b, hints) {
                    return Ok(res);
                } else {
                    continue;
                }
            }
        }
        Err(Exceptions::NOT_FOUND)
    }
}

#[derive(Debug, Clone)]
struct LumImagePyramid {
    buffers: Vec<Luma8LuminanceSource>,
    pub layers: Vec<Luma8LuminanceSource>,
}

impl Default for LumImagePyramid {
    fn default() -> Self {
        Self {
            buffers: Default::default(),
            layers: Default::default(),
        }
    }
}

impl LumImagePyramid {
    pub fn new(image: Luma8LuminanceSource, threshold: usize, factor: usize) -> Option<Self> {
        let mut new_self = Self::default();

        new_self.layers.push(image);
        // TODO: if only matrix codes were considered, then using std::min would be sufficient (see #425)
        while threshold > 0
            && std::cmp::max(
                new_self.layers.last()?.get_width(),
                new_self.layers.last()?.get_height(),
            ) > threshold
            && std::cmp::min(
                new_self.layers.last()?.get_width(),
                new_self.layers.last()?.get_height(),
            ) >= factor
        {
            new_self.add_layer_with_factor(factor).ok()?;
        }

        if false {
            // Reversing the layers means we'd start with the smallest. that can make sense if we are only looking for a
            // single symbol. If we start with the higher resolution, we get better (high res) position information.
            // TODO: see if masking out higher res layers based on found symbols in lower res helps overall performance.
            new_self.layers.reverse();
        }

        Some(new_self)
    }

    fn add_layer<const N: usize>(&mut self) -> Result<()> {
        let siv = self.layers.last().ok_or(Exceptions::ILLEGAL_ARGUMENT)?;

        self.buffers.push(Luma8LuminanceSource::with_empty_image(
            siv.get_width() / N,
            siv.get_height() / N,
        ));

        let div = self
            .buffers
            .last_mut()
            .ok_or(Exceptions::ILLEGAL_ARGUMENT)?;

        let div_height = div.get_height();
        let div_width = div.get_width();

        let d_vec = div.get_matrix_mut();

        for d in d_vec.iter_mut() {
            for dy in 0..div_height {
                // for (int dy = 0; dy < div.height(); ++dy){
                for dx in 0..div_width {
                    // for (int dx = 0; dx < div.width(); ++dx) {
                    let mut sum = (N * N) / 2;
                    for ty in 0..N {
                        // for (int ty = 0; ty < N; ++ty){
                        for tx in 0..N {
                            // for (int tx = 0; tx < N; ++tx) {
                            sum += siv.get_luma8_point(dx * N + tx, dy * N + ty) as usize;
                        }
                    }
                    *d = (sum / (N * N)) as u8;
                }
            }
        }

        self.layers.push(
            self.buffers
                .last()
                .ok_or(Exceptions::ILLEGAL_ARGUMENT)?
                .clone(),
        );

        Ok(())
    }

    fn add_layer_with_factor(&mut self, factor: usize) -> Result<()> {
        // help the compiler's auto-vectorizer by hard-coding the scale factor
        match factor {
            2 => self.add_layer::<2>(),
            3 => self.add_layer::<3>(),
            4 => self.add_layer::<4>(),
            _ => Err(Exceptions::illegal_argument_with(
                "Invalid ReaderOptions::downscaleFactor",
            )),
        }
    }
}

const SET_V: u32 = 0xff; // allows playing with SIMD binarization

impl<B: Binarizer> BinaryBitmap<B> {
    pub fn close(&mut self) -> Result<()> {
        if let Some(mut matrix) = self.matrix.as_mut() {
            // if (_cache->matrix) {
            // auto& matrix = *const_cast<BitMatrix*>(_cache->matrix.get());
            let mut tmp = BitMatrix::new(matrix.width(), matrix.height())?;

            // dilate
            SumFilter(matrix, &mut tmp, |sum| {
                return u32::from(sum > 0 * SET_V) * SET_V;
            });
            // erode
            SumFilter(&tmp, &mut matrix, |sum| {
                return u32::from(sum == 9 * SET_V) * SET_V;
            });
        }
        Ok(())
        // _closed = true;
    }
}

fn SumFilter<F>(input: &BitMatrix, output: &mut BitMatrix, func: F)
where
    F: Fn(u32) -> u32,
{
    for row in 1..output.height() {
        for col in 0..output.width() - 1 {
            let in0 = input.getRow(row - 1); //.row(0).begin();
            let in1 = input.getRow(row); //.row(1).begin();
            let in2 = input.getRow(row + 1); //.row(2).begin();

            let mut sum = 0;
            for j in 0..3 {
                // for (int j = 0; j < 3; ++j){
                sum += in0.get(j) as u32 + in1.get(j) as u32 + in2.get(j) as u32;
            }

            output.set_bool(row, col, func(sum) != 0);
        }
    }
}
