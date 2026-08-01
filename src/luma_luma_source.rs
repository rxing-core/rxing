use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;

use crate::LuminanceSource;
use crate::common::Result;

/// Backing pixel storage for [`Luma8Source`].
///
/// Not a [`Cow`]: `crop(&self) -> Self` results may outlive `&self`, so an
/// exclusively-owned buffer could never lend itself to a crop and would have to
/// copy the region. Sharing ownership through an [`Arc`] instead makes cropping
/// and cloning refcount bumps for owned sources too.
#[derive(Debug, Clone)]
enum LumaData<'a> {
    /// borrows an external buffer (see [`Luma8Source::new_with_slice`])
    Borrowed(&'a [u8]),
    /// shares an owned buffer with all crops and clones of this source.
    ///
    /// Deliberately `Arc<Vec<u8>>`, not `Arc<Box<[u8]>>` or `Arc<[u8]>`:
    /// `Arc::new(vec)` is always a move, while `into_boxed_slice` reallocates
    /// whenever `len != capacity` and `Arc<[u8]>::from(Vec)` always memcpys —
    /// both would put a hidden full-buffer copy on the ingest path.
    Shared(Arc<Vec<u8>>),
}

impl Deref for LumaData<'_> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        match self {
            LumaData::Borrowed(data) => data,
            LumaData::Shared(data) => data,
        }
    }
}

/// A simple luma8 source for bytes; supports cropping and rotation.
///
/// Build it with [`Luma8Source::new`] to own the buffer, or with
/// [`Luma8Source::new_with_slice`] to borrow an existing frame without copying
/// it. Cropping never copies pixel data — it only narrows the view
/// (`origin`/`row_stride`); owned buffers are shared with crops and clones via
/// [`Arc`], so a retained crop keeps its parent's whole buffer alive. Rotation
/// always materializes a new row-contiguous buffer so that row access stays
/// cheap afterwards.
///
/// [`Luma8LuminanceSource`] is an alias for the owned (`'static`) form.
#[derive(Debug, Clone)]
pub struct Luma8Source<'a> {
    /// backing luma8 pixels; may be borrowed or shared
    data: LumaData<'a>,
    /// top-left corner of the view within the backing buffer, in form (x,y)
    origin: (usize, usize),
    /// view dimension in form (x,y)
    dimensions: (u32, u32),
    /// row-to-row distance in `data` (width of the backing buffer)
    row_stride: usize,
    /// flag indicating if the underlying data needs to be inverted for use
    inverted: bool,
}

/// Owned [`Luma8Source`]; the backing buffer is owned, so no lifetime is involved.
pub type Luma8LuminanceSource = Luma8Source<'static>;

impl LuminanceSource for Luma8Source<'_> {
    const SUPPORTS_CROP: bool = true;
    const SUPPORTS_ROTATION: bool = true;

    fn get_row(&'_ self, y: usize) -> Option<Cow<'_, [u8]>> {
        if y >= self.get_height() {
            return None;
        }
        let row = self.row_slice(y);
        if self.inverted {
            Some(Cow::Owned(self.invert_block_of_bytes(row.to_vec())))
        } else {
            Some(Cow::Borrowed(row))
        }
    }

    fn get_column(&self, x: usize) -> Cow<'_, [u8]> {
        if x >= self.get_width() {
            return Cow::Owned(Vec::new());
        }
        let column = (0..self.get_height())
            .map(|y| Self::invert_if_should(self.data[self.row_start(y) + x], self.inverted))
            .collect();
        Cow::Owned(column)
    }

    fn get_matrix(&self) -> Cow<'_, [u8]> {
        if !self.inverted {
            if let Some(slice) = self.contiguous_slice() {
                return Cow::Borrowed(slice);
            }
        }
        let packed = match self.contiguous_slice() {
            Some(slice) => slice.to_vec(),
            None => self.pack_view(),
        };
        if self.inverted {
            Cow::Owned(self.invert_block_of_bytes(packed))
        } else {
            Cow::Owned(packed)
        }
    }

    fn get_width(&self) -> usize {
        self.dimensions.0 as usize
    }

    fn get_height(&self) -> usize {
        self.dimensions.1 as usize
    }

    fn invert(&mut self) {
        self.inverted = !self.inverted;
    }

    fn crop(&self, left: usize, top: usize, width: usize, height: usize) -> Result<Self> {
        if left + width > self.get_width() || top + height > self.get_height() {
            return Err(crate::Error::illegal_argument_with(
                "Crop rectangle does not fit within image data.",
            ));
        }

        // Narrowing the view is free for both variants: no pixels move.
        Ok(Self {
            data: self.data.clone(),
            origin: (self.origin.0 + left, self.origin.1 + top),
            dimensions: (width as u32, height as u32),
            row_stride: self.row_stride,
            inverted: self.inverted,
        })
    }

    fn rotate_counter_clockwise(&self) -> Result<Self> {
        // Materialize into a fresh row-contiguous buffer: every row of a rotated
        // barcode image gets read at least once, so a lazy transposed view would
        // pay strided access repeatedly instead of one O(n) transpose here.
        let (w, h) = (self.get_width(), self.get_height());
        let mut out = vec![0u8; w * h];
        for y in 0..h {
            for (x, &px) in self.row_slice(y).iter().enumerate() {
                out[(w - 1 - x) * h + y] = px;
            }
        }
        Ok(Self {
            data: LumaData::Shared(Arc::new(out)),
            origin: (0, 0),
            dimensions: (self.dimensions.1, self.dimensions.0),
            row_stride: h,
            inverted: self.inverted,
        })
    }

    fn rotate_counter_clockwise_45(&self) -> Result<Self> {
        Err(crate::Error::unsupported_operation_with(
            "This luminance source does not support rotation by 45 degrees.",
        ))
    }

    fn get_luma8_point(&self, column: usize, row: usize) -> u8 {
        Self::invert_if_should(self.data[self.row_start(row) + column], self.inverted)
    }
}

impl<'a> Luma8Source<'a> {
    pub fn new(source: Vec<u8>, width: u32, height: u32) -> Result<Self> {
        // checked_mul so the product can't silently wrap on 32-bit/wasm targets
        // (where usize is 32-bit) and accept a mismatched buffer.
        if (width as usize).checked_mul(height as usize) != Some(source.len()) {
            return Err(crate::Error::illegal_argument_with(
                "Dimensions do not match the data length.",
            ));
        }
        Ok(Self {
            data: LumaData::Shared(Arc::new(source)),
            origin: (0, 0),
            dimensions: (width, height),
            row_stride: width as usize,
            inverted: false,
        })
    }

    /// Build a source that borrows `source` instead of copying it. The zero-copy
    /// entry point: crops of the result stay views into `source`.
    pub fn new_with_slice(source: &'a [u8], width: u32, height: u32) -> Result<Self> {
        if (width as usize).checked_mul(height as usize) != Some(source.len()) {
            return Err(crate::Error::illegal_argument_with(
                "Dimensions do not match the data length.",
            ));
        }
        Ok(Self {
            data: LumaData::Borrowed(source),
            origin: (0, 0),
            dimensions: (width, height),
            row_stride: width as usize,
            inverted: false,
        })
    }

    pub fn with_empty_image(width: usize, height: usize) -> Self {
        Self {
            data: LumaData::Shared(Arc::new(vec![0u8; width * height])),
            origin: (0, 0),
            dimensions: (width as u32, height as u32),
            row_stride: width,
            inverted: false,
        }
    }

    /// Mutable access to exactly the view's pixels, row-major and contiguous.
    /// Copy-on-write: a borrowed, cropped, shared, or inverted source is
    /// repacked into a fresh uniquely-owned buffer first (with any pending
    /// inversion applied and the flag cleared), so neither the original input
    /// nor any crop/clone sharing the buffer is ever modified through this,
    /// and the returned bytes always match `get_matrix()`.
    pub fn get_matrix_mut(&mut self) -> &mut [u8] {
        let exact = self.origin == (0, 0)
            && self.row_stride == self.get_width()
            && self.data.len() == self.get_width() * self.get_height();
        let unique = match &mut self.data {
            LumaData::Shared(data) => Arc::get_mut(data).is_some(),
            LumaData::Borrowed(_) => false,
        };
        if !(exact && unique) || self.inverted {
            let mut packed = self.pack_view();
            if self.inverted {
                packed = self.invert_block_of_bytes(packed);
                self.inverted = false;
            }
            self.data = LumaData::Shared(Arc::new(packed));
            self.origin = (0, 0);
            self.row_stride = self.get_width();
        }
        match &mut self.data {
            LumaData::Shared(data) => {
                Arc::get_mut(data).expect("buffer is uniquely owned after repack")
            }
            LumaData::Borrowed(_) => unreachable!("repacked into Shared above"),
        }
    }

    #[inline(always)]
    fn invert_if_should(byte: u8, invert: bool) -> u8 {
        if invert { 255 - byte } else { byte }
    }

    /// Offset into `data` where row `y` of the view begins.
    #[inline(always)]
    fn row_start(&self, y: usize) -> usize {
        (self.origin.1 + y) * self.row_stride + self.origin.0
    }

    /// The pixels of view row `y`, uninverted. Always contiguous.
    #[inline(always)]
    fn row_slice(&self, y: usize) -> &[u8] {
        let start = self.row_start(y);
        &self.data[start..start + self.get_width()]
    }

    /// The whole view as one slice of `data`, if the view rows are adjacent.
    fn contiguous_slice(&self) -> Option<&[u8]> {
        let len = self.get_width() * self.get_height();
        // A zero-area view (e.g. an edge crop with zero width or height) is
        // trivially contiguous and empty; `row_start(0)` may point one past the
        // buffer, so don't index with it.
        if len == 0 {
            return Some(&[]);
        }
        if self.row_stride == self.get_width() || self.get_height() <= 1 {
            let start = self.row_start(0);
            Some(&self.data[start..start + len])
        } else {
            None
        }
    }

    /// Pack the view into a fresh row-major buffer, uninverted.
    fn pack_view(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.get_width() * self.get_height());
        for y in 0..self.get_height() {
            out.extend_from_slice(self.row_slice(y));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{Luma8LuminanceSource, Luma8Source, LuminanceSource};

    /// 4x4 test image:
    ///  0  1  2  3
    ///  4  5  6  7
    ///  8  9 10 11
    /// 12 13 14 15
    fn buf_4x4() -> Vec<u8> {
        (0..16).collect()
    }

    #[test]
    fn test_rotate() {
        let src_square = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let src_rect = vec![0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0];

        let square = Luma8LuminanceSource::new(src_square, 3, 3).unwrap();
        let rect_tall = Luma8LuminanceSource::new(src_rect.clone(), 3, 4).unwrap();
        let rect_wide = Luma8LuminanceSource::new(src_rect, 4, 3).unwrap();

        let rotated_square = square.rotate_counter_clockwise().expect("rotate");
        let rotated_wide_rect = rect_wide.rotate_counter_clockwise().expect("rotate");
        let rotated_tall_rect = rect_tall.rotate_counter_clockwise().expect("rotate");

        assert_eq!(rotated_square.get_width(), square.get_width());
        assert_eq!(rotated_square.get_height(), square.get_height());
        assert_eq!(rotated_tall_rect.get_width(), rect_tall.get_height());
        assert_eq!(rotated_tall_rect.get_height(), rect_tall.get_width());
        assert_eq!(rotated_wide_rect.get_width(), rect_wide.get_height());
        assert_eq!(rotated_wide_rect.get_height(), rect_wide.get_width());

        assert_eq!(&*rotated_square.get_matrix(), &[3, 6, 9, 2, 5, 8, 1, 4, 7]);
        assert_eq!(
            &*rotated_wide_rect.get_matrix(),
            &[1, 1, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1]
        );
        assert_eq!(
            &*rotated_tall_rect.get_matrix(),
            &[0, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0]
        );
    }

    #[test]
    fn new_with_slice_borrows_input_buffer() {
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let matrix = src.get_matrix();
        let Cow::Borrowed(m) = matrix else {
            panic!("full-view matrix of a borrowed source must be Cow::Borrowed");
        };
        assert_eq!(m.as_ptr(), buf.as_ptr(), "must point at the input buffer");
        assert_eq!(m, &buf[..]);
    }

    #[test]
    fn crop_of_borrowed_source_is_zero_copy() {
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let cropped = src.crop(1, 1, 2, 2).expect("crop");

        assert_eq!(cropped.get_width(), 2);
        assert_eq!(cropped.get_height(), 2);

        let base = buf.as_ptr() as usize;
        for (y, expected) in [[5u8, 6], [9, 10]].iter().enumerate() {
            let row = cropped.get_row(y).expect("row in bounds");
            let Cow::Borrowed(s) = row else {
                panic!("cropped row must be Cow::Borrowed");
            };
            assert_eq!(s, expected);
            let p = s.as_ptr() as usize;
            assert!(
                p >= base && p < base + buf.len(),
                "cropped row must point into the original buffer"
            );
        }
    }

    #[test]
    fn crop_of_owned_source_has_correct_values() {
        let src = Luma8LuminanceSource::new(buf_4x4(), 4, 4).unwrap();
        let cropped = src.crop(1, 1, 2, 2).expect("crop");
        assert_eq!(&*cropped.get_matrix(), &[5, 6, 9, 10]);
        // rows of a cropped owned source stay borrowed (point into self)
        assert!(matches!(cropped.get_row(1), Some(Cow::Borrowed(_))));
    }

    #[test]
    fn crop_of_owned_source_is_zero_copy() {
        let src = Luma8LuminanceSource::new(buf_4x4(), 4, 4).unwrap();
        let cropped = src.crop(1, 1, 2, 2).expect("crop");

        let parent = src.get_matrix();
        let Cow::Borrowed(parent_slice) = parent else {
            panic!("full-view matrix of an owned source must be Cow::Borrowed");
        };
        let row = cropped.get_row(0).expect("row in bounds");
        let Cow::Borrowed(row_slice) = row else {
            panic!("cropped row must be Cow::Borrowed");
        };

        let base = parent_slice.as_ptr() as usize;
        let p = row_slice.as_ptr() as usize;
        assert!(
            p >= base && p < base + parent_slice.len(),
            "owned crop must share the parent's buffer, not copy the region"
        );
    }

    #[test]
    fn clone_of_owned_source_shares_buffer() {
        let src = Luma8LuminanceSource::new(buf_4x4(), 4, 4).unwrap();
        let cloned = src.clone();
        let Some(Cow::Borrowed(a)) = src.get_row(0) else {
            panic!("row must be borrowed");
        };
        let Some(Cow::Borrowed(b)) = cloned.get_row(0) else {
            panic!("row must be borrowed");
        };
        assert_eq!(a.as_ptr(), b.as_ptr(), "clone must share, not copy");
    }

    #[test]
    fn shared_buffers_are_isolated_on_mutation() {
        let mut parent = Luma8LuminanceSource::new(buf_4x4(), 4, 4).unwrap();
        let mut child = parent.crop(1, 1, 2, 2).expect("crop");

        // mutating the parent must not be visible through the child
        parent.get_matrix_mut()[5] = 99; // parent's (1,1) == child's (0,0)
        assert_eq!(parent.get_luma8_point(1, 1), 99);
        assert_eq!(child.get_luma8_point(0, 0), 5);

        // and mutating the child must not be visible through the parent
        child.get_matrix_mut()[3] = 77; // child's (1,1) == parent's (2,2)
        assert_eq!(child.get_luma8_point(1, 1), 77);
        assert_eq!(parent.get_luma8_point(2, 2), 10);
    }

    #[test]
    fn vertical_crop_get_matrix_is_borrowed() {
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let cropped = src.crop(0, 1, 4, 2).expect("crop");
        let matrix = cropped.get_matrix();
        let Cow::Borrowed(m) = matrix else {
            panic!("full-width crop keeps a contiguous, borrowable matrix");
        };
        assert_eq!(m, &buf[4..12]);
    }

    #[test]
    fn crop_of_crop_composes_origins() {
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let inner = src
            .crop(1, 1, 3, 3)
            .expect("outer crop")
            .crop(1, 1, 2, 2)
            .expect("inner crop");
        assert_eq!(&*inner.get_matrix(), &[10, 11, 14, 15]);
        assert!(matches!(inner.get_row(0), Some(Cow::Borrowed(_))));
    }

    #[test]
    fn zero_area_bottom_edge_crop_does_not_panic() {
        // A zero-height crop at the bottom edge with a nonzero left origin is a
        // valid rectangle (multi-barcode readers produce these) and its matrix
        // must be empty, not a panic from indexing past the buffer.
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let cropped = src.crop(1, 4, 3, 0).expect("zero-height crop is valid");
        assert_eq!(cropped.get_width(), 3);
        assert_eq!(cropped.get_height(), 0);
        assert!(cropped.get_matrix().is_empty());
    }

    #[test]
    fn zero_width_right_edge_crop_does_not_panic() {
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let cropped = src.crop(4, 1, 0, 2).expect("zero-width crop is valid");
        assert_eq!(cropped.get_width(), 0);
        assert_eq!(cropped.get_height(), 2);
        assert!(cropped.get_matrix().is_empty());
    }

    #[test]
    fn crop_out_of_bounds_errors() {
        let src = Luma8LuminanceSource::new(buf_4x4(), 4, 4).unwrap();
        assert!(src.crop(2, 2, 3, 3).is_err());
        assert!(src.crop(0, 0, 5, 4).is_err());
        assert!(src.crop(4, 0, 1, 1).is_err());
    }

    #[test]
    fn get_row_out_of_bounds_returns_none() {
        let src = Luma8LuminanceSource::new(buf_4x4(), 4, 4).unwrap();
        assert!(src.get_row(3).is_some());
        assert!(src.get_row(4).is_none());
    }

    #[test]
    fn rows_are_borrowed_and_contiguous_after_rotate() {
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let rotated = src.rotate_counter_clockwise().expect("rotate");
        // CCW: first row of the rotated image is the last column, top to bottom
        let row = rotated.get_row(0).expect("row in bounds");
        assert!(
            matches!(row, Cow::Borrowed(_)),
            "rotation must materialize to standard layout so rows stay borrowed"
        );
        assert_eq!(&*row, &[3, 7, 11, 15]);
    }

    #[test]
    fn rotate_of_cropped_view_respects_stride() {
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let rotated = src
            .crop(1, 1, 2, 2)
            .expect("crop")
            .rotate_counter_clockwise()
            .expect("rotate");
        assert_eq!(&*rotated.get_matrix(), &[6, 10, 5, 9]);
    }

    #[test]
    fn get_column_returns_cow_with_correct_values() {
        let buf = buf_4x4();
        let src = Luma8Source::new_with_slice(&buf, 4, 4).unwrap();
        let col: Cow<'_, [u8]> = src.get_column(1);
        assert_eq!(&*col, &[1, 5, 9, 13]);

        let cropped = src.crop(1, 1, 2, 2).expect("crop");
        assert_eq!(&*cropped.get_column(0), &[5, 9]);

        assert!(src.get_column(4).is_empty());
    }

    #[test]
    fn inverted_source_inverts_all_accessors() {
        let mut src = Luma8LuminanceSource::new(vec![0, 10, 100, 255], 2, 2).unwrap();
        src.invert();
        let row = src.get_row(0).expect("row in bounds");
        assert!(matches!(row, Cow::Owned(_)), "inverted rows must be owned");
        assert_eq!(&*row, &[255, 245]);
        assert_eq!(&*src.get_matrix(), &[255, 245, 155, 0]);
        assert_eq!(&*src.get_column(0), &[255, 155]);
        assert_eq!(src.get_luma8_point(1, 1), 0);
        // double inversion round-trips
        src.invert();
        assert_eq!(&*src.get_matrix(), &[0, 10, 100, 255]);
    }

    #[test]
    fn get_matrix_mut_returns_exact_view_pixels() {
        let buf = buf_4x4();
        let mut cropped = Luma8Source::new_with_slice(&buf, 4, 4)
            .unwrap()
            .crop(1, 1, 2, 2)
            .expect("crop");
        {
            let m = cropped.get_matrix_mut();
            assert_eq!(m, &[5, 6, 9, 10][..]);
            m[0] = 42;
        }
        assert_eq!(cropped.get_luma8_point(0, 0), 42);
        // copy-on-write: the original buffer is untouched
        assert_eq!(buf, buf_4x4());
    }

    #[test]
    fn dimension_mismatch_errors() {
        assert!(Luma8LuminanceSource::new(vec![0; 5], 2, 2).is_err());
        let buf = vec![0; 5];
        assert!(Luma8Source::new_with_slice(&buf, 2, 2).is_err());
        // u32 overflow must not wrap into a false match (65536 * 65536 wraps to 0)
        assert!(Luma8LuminanceSource::new(Vec::new(), 65536, 65536).is_err());
        assert!(Luma8Source::new_with_slice(&[], 65536, 65536).is_err());
    }

    #[test]
    fn owned_alias_is_static() {
        fn assert_static<T: 'static>(_: &T) {}
        let src = Luma8LuminanceSource::new(vec![0; 4], 2, 2).unwrap();
        assert_static(&src);
    }

    #[test]
    fn get_matrix_mut_materializes_inversion() {
        let mut src = Luma8LuminanceSource::new(vec![0, 10, 100, 255], 2, 2).unwrap();
        src.invert();
        assert_eq!(src.get_matrix_mut(), &[255, 245, 155, 0][..]);
        // the flag was materialized: the read view agrees and borrows again
        let m = src.get_matrix();
        assert!(matches!(m, Cow::Borrowed(_)));
        assert_eq!(&*m, &[255, 245, 155, 0]);
        assert_eq!(src.get_luma8_point(0, 0), 255);
        // inverting again returns the original values
        src.invert();
        assert_eq!(&*src.get_matrix(), &[0, 10, 100, 255]);
    }
}
