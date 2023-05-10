use crate::Binarizer;
use crate::BinaryBitmap;
use crate::common::BitMatrix;
use crate::common::Result;
use crate::Exceptions;
use once_cell::sync::OnceCell;

pub trait Denoiser {
    fn denoise_bitmatrix(&self, _matrix: &BitMatrix) -> Result<&BitMatrix> {
        Err(Exceptions::UNSUPPORTED_OPERATION)
    }
    fn denoise_bitmap<B: Binarizer>(&self, _bitmap: &BinaryBitmap<B>) -> Result<&BinaryBitmap<B>>{
        Err(Exceptions::UNSUPPORTED_OPERATION)
    }
}

#[derive(Debug, Default)]
pub struct DefaultDenoiser<B: Binarizer>{
    cached_result: OnceCell<BitMatrix>,
    cached_bitmap: OnceCell<BinaryBitmap<B>>
}

impl<B: Binarizer> Denoiser for DefaultDenoiser<_> {
    fn denoise_bitmatrix(&self, matrix: &BitMatrix) -> Result<&BitMatrix> {
        self.cached_result.get_or_try_init(|| {
            Self::close(matrix)
        })
    }

    fn denoise_bitmap(&self, _bitmap: &BinaryBitmap<B>) -> Result<&BinaryBitmap<B>>{
        Err(Exceptions::UNSUPPORTED_OPERATION)
    }
    
}

impl<B: Binarizer> DefaultDenoiser<_> {
    fn close(matrix: &BitMatrix) -> Result<BitMatrix>{
        // Dilate the image.
        let mut matrix = matrix.clone();
        let mut dilated_matrix = BitMatrix::new(matrix.width(), matrix.height())?;
        for y in 0..matrix.height() {
          for x in 0..matrix.width() {
            if matrix.get(x, y) {
              for dx in -1_i32..2 {
                for dy in -1_i32..2 {
                  if dx == 0 && dy == 0 {
                    continue;
                  }
                  let xx = x as i32+ dx;
                  let yy = y as i32+ dy;
                  if xx < 0 || xx >= matrix.getWidth() as i32 || yy < 0 || yy >= matrix.getHeight() as i32 {
                    continue;
                  }
                  dilated_matrix.set(xx as u32, yy as u32);
                }
              }
            }
          }
        }
      
        // Erode the image.
        for y in 0..matrix.height() {
          for x in 0..matrix.width() {
            if dilated_matrix.get(x, y) {
              let mut count = 0;
              for dx in -1_i32..2 {
                for dy in -1_i32..2 {
                  if dx == 0 && dy == 0 {
                    continue;
                  }
                  let xx = x as i32 + dx;
                  let yy = y as i32+ dy;
                  if xx < 0 || xx >= matrix.getWidth() as i32 || yy < 0 || yy >= matrix.getHeight() as i32 {
                    continue;
                  }
                  count += dilated_matrix.get(xx as u32, yy as u32) as usize;
                }
              }
              if count == 9 {
                matrix.set(x, y);
              }
            }
          }
        }

        Ok(matrix)
      }
}