use crate::common::{BitMatrix, HybridBinarizer, Result};
use crate::{
    Binarizer, BinaryBitmap, DecodeHints, Exceptions, Luma8LuminanceSource, LuminanceSource, Reader,
};

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
        self.decode_with_hints(image, &DecodeHints::default())
    }

    fn decode_with_hints<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
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
                if let Ok(mut res) = self.0.decode_with_hints(&mut b, hints) {
                    res.putMetadata(
                        crate::RXingResultMetadataType::FILTERED_CLOSED,
                        crate::RXingResultMetadataValue::FilteredClosed(close),
                    );
                    let resolution = (b.get_width(), b.get_height());
                    res.putMetadata(
                        crate::RXingResultMetadataType::FILTERED_RESOLUTION,
                        crate::RXingResultMetadataValue::FilteredResolution(resolution),
                    );
                    return Ok(res);
                } else {
                    continue;
                }
            }
        }
        Err(Exceptions::NOT_FOUND)
    }
}

#[derive(Debug, Clone, Default)]
struct LumImagePyramid {
    pub layers: Vec<Luma8LuminanceSource>,
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

        #[cfg(feature = "reverse_pyramid_layers")]
        // Reversing the layers means we'd start with the smallest. that can make sense if we are only looking for a
        // single symbol. If we start with the higher resolution, we get better (high res) position information.
        // TODO: see if masking out higher res layers based on found symbols in lower res helps overall performance.
        new_self.layers.reverse();

        Some(new_self)
    }

    fn add_layer<const N: usize>(&mut self) -> Result<()> {
        let siv = self.layers.last().ok_or(Exceptions::ILLEGAL_ARGUMENT)?;

        let mut div =
            Luma8LuminanceSource::with_empty_image(siv.get_width() / N, siv.get_height() / N);

        let div_height = div.get_height();
        let div_width = div.get_width();

        let mut d_vec_it = div.get_matrix_mut().iter_mut();

        'main: for dy in 0..div_height {
            // for (int dy = 0; dy < div.height(); ++dy)
            for dx in 0..div_width {
                // for (int dx = 0; dx < div.width(); ++dx) {
                let mut sum = (N * N) / 2;
                for ty in 0..N {
                    // for (int ty = 0; ty < N; ++ty){
                    for tx in 0..N {
                        // for (int tx = 0; tx < N; ++tx){
                        sum += siv.get_luma8_point(dx * N + tx, dy * N + ty) as usize;
                    }
                }
                if let Some(d) = d_vec_it.next() {
                    *d = (sum / (N * N)) as u8;
                } else {
                    break 'main;
                }
            }
        }

        self.layers.push(div);

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

impl<B: Binarizer> BinaryBitmap<B> {
    pub fn close(&mut self) -> Result<()> {
        if let Some(matrix) = self.matrix.get_mut() {
            let mut tmp = BitMatrix::new(matrix.width(), matrix.height())?;
            // dilate
            SumFilter(matrix, &mut tmp, |sum| sum > 0);
            // erode
            SumFilter(&tmp, matrix, |sum| sum == 9);
        }
        Ok(())
    }
}

fn SumFilter<F>(input: &BitMatrix, output: &mut BitMatrix, func: F)
where
    F: Fn(u8) -> bool,
{
    for row in 1..output.height() - 1 {
        for col in 0..output.width() - 1 {
            let mut sum = 0;
            for j in 0..3 {
                sum += input.try_get(col + j, row - 1).unwrap_or_default() as u8
                    + input.try_get(col + j, row).unwrap_or_default() as u8
                    + input.try_get(col + j, row + 1).unwrap_or_default() as u8;
            }
            output.set_bool(col, row, func(sum));
        }
    }
}
