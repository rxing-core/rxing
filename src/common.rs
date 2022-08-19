pub mod detector;
pub mod readsolomon;

use std::collections::HashMap;

use crate::{
    Binarizer, Binarizer, FormatException, LuminanceSource, NotFoundException, NotFoundException,
    ResultPoint,
};

// ECIInput.java
/**
 * Interface to navigate a sequence of ECIs and bytes.
 *
 * @author Alex Geller
 */
pub trait ECIInput {
    /**
     * Returns the length of this input.  The length is the number
     * of {@code byte}s in or ECIs in the sequence.
     *
     * @return  the number of {@code char}s in this sequence
     */
    fn length(&self) -> i32;

    /**
     * Returns the {@code byte} value at the specified index.  An index ranges from zero
     * to {@code length() - 1}.  The first {@code byte} value of the sequence is at
     * index zero, the next at index one, and so on, as for array
     * indexing.
     *
     * @param   index the index of the {@code byte} value to be returned
     *
     * @return  the specified {@code byte} value as character or the FNC1 character
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     * @throws  IllegalArgumentException
     *          if the value at the {@code index} argument is an ECI (@see #isECI)
     */
    fn char_at(&self, index: i32) -> char;

    /**
     * Returns a {@code CharSequence} that is a subsequence of this sequence.
     * The subsequence starts with the {@code char} value at the specified index and
     * ends with the {@code char} value at index {@code end - 1}.  The length
     * (in {@code char}s) of the
     * returned sequence is {@code end - start}, so if {@code start == end}
     * then an empty sequence is returned.
     *
     * @param   start   the start index, inclusive
     * @param   end     the end index, exclusive
     *
     * @return  the specified subsequence
     *
     * @throws  IndexOutOfBoundsException
     *          if {@code start} or {@code end} are negative,
     *          if {@code end} is greater than {@code length()},
     *          or if {@code start} is greater than {@code end}
     * @throws  IllegalArgumentException
     *          if a value in the range {@code start}-{@code end} is an ECI (@see #isECI)
     */
    fn sub_sequence(&self, start: i32, end: i32) -> CharSequence;

    /**
     * Determines if a value is an ECI
     *
     * @param   index the index of the value
     *
     * @return  true if the value at position {@code index} is an ECI
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     */
    fn is_e_c_i(&self, index: i32) -> bool;

    /**
     * Returns the {@code int} ECI value at the specified index.  An index ranges from zero
     * to {@code length() - 1}.  The first {@code byte} value of the sequence is at
     * index zero, the next at index one, and so on, as for array
     * indexing.
     *
     * @param   index the index of the {@code int} value to be returned
     *
     * @return  the specified {@code int} ECI value.
     *          The ECI specified the encoding of all bytes with a higher index until the
     *          next ECI or until the end of the input if no other ECI follows.
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     * @throws  IllegalArgumentException
     *          if the value at the {@code index} argument is not an ECI (@see #isECI)
     */
    fn get_e_c_i_value(&self, index: i32) -> i32;

    fn have_n_characters(&self, index: i32, n: i32) -> bool;
}

// GridSampler.java

/**
 * Implementations of this class can, given locations of finder patterns for a QR code in an
 * image, sample the right points in the image to reconstruct the QR code, accounting for
 * perspective distortion. It is abstracted since it is relatively expensive and should be allowed
 * to take advantage of platform-specific optimized implementations, like Sun's Java Advanced
 * Imaging library, but which may not be available in other environments such as J2ME, and vice
 * versa.
 *
 * The implementation used can be controlled by calling {@link #setGridSampler(GridSampler)}
 * with an instance of a class which implements this interface.
 *
 * @author Sean Owen
 */

//let grid_sampler: dyn GridSampler = DefaultGridSampler::new();
pub struct GridSampler {
    grid_sampler: dyn GridSampler,
}

impl GridSampler {
    pub fn new() -> Self {
        Self {
            grid_sampler: DefaultGridSampler::new(),
        }
    }

    /**
     * Sets the implementation of GridSampler used by the library. One global
     * instance is stored, which may sound problematic. But, the implementation provided
     * ought to be appropriate for the entire platform, and all uses of this library
     * in the whole lifetime of the JVM. For instance, an Android activity can swap in
     * an implementation that takes advantage of native platform libraries.
     *
     * @param newGridSampler The platform-specific object to install.
     */
    pub fn set_grid_sampler(new_grid_sampler: &GridSampler) {
        grid_sampler = new_grid_sampler;
    }

    /**
     * @return the current implementation of GridSampler
     */
    pub fn get_instance() -> GridSampler {
        return grid_sampler;
    }

    /**
     * Samples an image for a rectangular matrix of bits of the given dimension. The sampling
     * transformation is determined by the coordinates of 4 points, in the original and transformed
     * image space.
     *
     * @param image image to sample
     * @param dimensionX width of {@link BitMatrix} to sample from image
     * @param dimensionY height of {@link BitMatrix} to sample from image
     * @param p1ToX point 1 preimage X
     * @param p1ToY point 1 preimage Y
     * @param p2ToX point 2 preimage X
     * @param p2ToY point 2 preimage Y
     * @param p3ToX point 3 preimage X
     * @param p3ToY point 3 preimage Y
     * @param p4ToX point 4 preimage X
     * @param p4ToY point 4 preimage Y
     * @param p1FromX point 1 image X
     * @param p1FromY point 1 image Y
     * @param p2FromX point 2 image X
     * @param p2FromY point 2 image Y
     * @param p3FromX point 3 image X
     * @param p3FromY point 3 image Y
     * @param p4FromX point 4 image X
     * @param p4FromY point 4 image Y
     * @return {@link BitMatrix} representing a grid of points sampled from the image within a region
     *   defined by the "from" parameters
     * @throws NotFoundException if image can't be sampled, for example, if the transformation defined
     *   by the given points is invalid or results in sampling outside the image boundaries
     */
    pub fn sample_grid(
        &self,
        image: &BitMatrix,
        dimension_x: i32,
        dimension_y: i32,
        p1_to_x: f32,
        p1_to_y: f32,
        p2_to_x: f32,
        p2_to_y: f32,
        p3_to_x: f32,
        p3_to_y: f32,
        p4_to_x: f32,
        p4_to_y: f32,
        p1_from_x: f32,
        p1_from_y: f32,
        p2_from_x: f32,
        p2_from_y: f32,
        p3_from_x: f32,
        p3_from_y: f32,
        p4_from_x: f32,
        p4_from_y: f32,
    ) -> Result<BitMatrix, NotFoundException>;

    pub fn sample_grid(
        &self,
        image: &BitMatrix,
        dimension_x: i32,
        dimension_y: i32,
        transform: &PerspectiveTransform,
    ) -> Result<BitMatrix, NotFoundException>;

    /**
     * <p>Checks a set of points that have been transformed to sample points on an image against
     * the image's dimensions to see if the point are even within the image.</p>
     *
     * <p>This method will actually "nudge" the endpoints back onto the image if they are found to be
     * barely (less than 1 pixel) off the image. This accounts for imperfect detection of finder
     * patterns in an image where the QR Code runs all the way to the image border.</p>
     *
     * <p>For efficiency, the method will check points from either end of the line until one is found
     * to be within the image. Because the set of points are assumed to be linear, this is valid.</p>
     *
     * @param image image into which the points should map
     * @param points actual points in x1,y1,...,xn,yn form
     * @throws NotFoundException if an endpoint is lies outside the image boundaries
     */
    pub fn check_and_nudge_points(
        image: &BitMatrix,
        points: &Vec<f32>,
    ) -> Result<(), NotFoundException> {
        let width: i32 = image.get_width();
        let height: i32 = image.get_height();
        // Check and nudge points from start until we see some that are OK:
        let mut nudged: bool = true;
        // points.length must be even
        let max_offset: i32 = points.len() - 1;
        {
            let mut offset: i32 = 0;
            while offset < max_offset && nudged {
                {
                    let x: i32 = points[offset] as i32;
                    let y: i32 = points[offset + 1] as i32;
                    if x < -1 || x > width || y < -1 || y > height {
                        return Err(NotFoundException::get_not_found_instance());
                    }
                    nudged = false;
                    if x == -1 {
                        points[offset] = 0.0f32;
                        nudged = true;
                    } else if x == width {
                        points[offset] = width - 1.0;
                        nudged = true;
                    }
                    if y == -1 {
                        points[offset + 1] = 0.0f32;
                        nudged = true;
                    } else if y == height {
                        points[offset + 1] = height - 1.0;
                        nudged = true;
                    }
                }
                offset += 2;
            }
        }

        // Check and nudge points from end:
        nudged = true;
        {
            let mut offset: i32 = points.len() - 2;
            while offset >= 0 && nudged {
                {
                    let x: i32 = points[offset] as i32;
                    let y: i32 = points[offset + 1] as i32;
                    if x < -1 || x > width || y < -1 || y > height {
                        return Err(NotFoundException::get_not_found_instance());
                    }
                    nudged = false;
                    if x == -1 {
                        points[offset] = 0.0f32;
                        nudged = true;
                    } else if x == width {
                        points[offset] = width - 1.0;
                        nudged = true;
                    }
                    if y == -1 {
                        points[offset + 1] = 0.0f32;
                        nudged = true;
                    } else if y == height {
                        points[offset + 1] = height - 1.0;
                        nudged = true;
                    }
                }
                offset -= 2;
            }
        }

        Ok(())
    }
}

// GlobalHistogramBinarizer.java
/**
 * This Binarizer implementation uses the old ZXing global histogram approach. It is suitable
 * for low-end mobile devices which don't have enough CPU or memory to use a local thresholding
 * algorithm. However, because it picks a global black point, it cannot handle difficult shadows
 * and gradients.
 *
 * Faster mobile devices and all desktop applications should probably use HybridBinarizer instead.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 */

const LUMINANCE_BITS: i32 = 5;

const LUMINANCE_SHIFT: i32 = 8 - LUMINANCE_BITS;

const LUMINANCE_BUCKETS: i32 = 1 << LUMINANCE_BITS;

const EMPTY: [i8; 0] = [0; 0];
pub struct GlobalHistogramBinarizer {
    //super: Binarizer;
    luminances: Vec<i8>,

    buckets: Vec<i32>,
}

impl Binarizer for GlobalHistogramBinarizer {
    // Applies simple sharpening to the row data to improve performance of the 1D Readers.
    fn get_black_row(&self, y: i32, row: &BitArray) -> Result<BitArray, NotFoundException> {
        let source: LuminanceSource = get_luminance_source();
        let width: i32 = source.get_width();
        if row == null || row.get_size() < width {
            row = &BitArray::new(None, Some(width));
        } else {
            row.clear();
        }
        self.init_arrays(width);
        let local_luminances: Vec<i8> = source.get_row(y, &self.luminances);
        let local_buckets: Vec<i32> = self.buckets;
        {
            let mut x: i32 = 0;
            while x < width {
                {
                    local_buckets[(local_luminances[x] & 0xff) >> LUMINANCE_SHIFT] += 1;
                }
                x += 1;
            }
        }

        let black_point: i32 = ::estimate_black_point(&local_buckets);
        if width < 3 {
            // Special case for very small images
            {
                let mut x: i32 = 0;
                while x < width {
                    {
                        if (local_luminances[x] & 0xff) < black_point {
                            row.set(x);
                        }
                    }
                    x += 1;
                }
            }
        } else {
            let mut left: i32 = local_luminances[0] & 0xff;
            let mut center: i32 = local_luminances[1] & 0xff;
            {
                let mut x: i32 = 1;
                while x < width - 1 {
                    {
                        let right: i32 = local_luminances[x + 1] & 0xff;
                        // A simple -1 4 -1 box filter with a weight of 2.
                        if ((center * 4) - left - right) / 2 < black_point {
                            row.set(x);
                        }
                        left = center;
                        center = right;
                    }
                    x += 1;
                }
            }
        }
        return Ok(row);
    }

    // Does not sharpen the data, as this call is intended to only be used by 2D Readers.
    fn get_black_matrix(&self) -> Result<BitMatrix, Rc<Exception>> {
        let source: LuminanceSource = get_luminance_source();
        let width: i32 = source.get_width();
        let height: i32 = source.get_height();
        let matrix: BitMatrix = BitMatrix::new(width, height, None, None);
        // Quickly calculates the histogram by sampling four rows from the image. This proved to be
        // more robust on the blackbox tests than sampling a diagonal as we used to do.
        self.init_arrays(width);
        let local_buckets: Vec<i32> = self.buckets;
        {
            let mut y: i32 = 1;
            while y < 5 {
                {
                    let row: i32 = height * y / 5;
                    let local_luminances: Vec<i8> = source.get_row(row, &self.luminances);
                    let right: i32 = (width * 4) / 5;
                    {
                        let mut x: i32 = width / 5;
                        while x < right {
                            {
                                let mut pixel: i32 = local_luminances[x] & 0xff;
                                local_buckets[pixel >> LUMINANCE_SHIFT] += 1;
                            }
                            x += 1;
                        }
                    }
                }
                y += 1;
            }
        }

        let black_point: i32 = ::estimate_black_point(&local_buckets);
        // We delay reading the entire image luminance until the black point estimation succeeds.
        // Although we end up reading four rows twice, it is consistent with our motto of
        // "fail quickly" which is necessary for continuous scanning.
        let local_luminances: Vec<i8> = source.get_matrix();
        {
            let mut y: i32 = 0;
            while y < height {
                {
                    let offset: i32 = y * width;
                    {
                        let mut x: i32 = 0;
                        while x < width {
                            {
                                let pixel: i32 = local_luminances[offset + x] & 0xff;
                                if pixel < black_point {
                                    matrix.set(x, y);
                                }
                            }
                            x += 1;
                        }
                    }
                }
                y += 1;
            }
        }

        return Ok(matrix);
    }

    fn create_binarizer(&self, source: &LuminanceSource) -> Binarizer {
        return GlobalHistogramBinarizer::new(source);
    }
}

impl GlobalHistogramBinarizer {
    pub fn new(source: &LuminanceSource) -> GlobalHistogramBinarizer {
        super(source);
        luminances = EMPTY;
        buckets = [0; LUMINANCE_BUCKETS];
    }

    fn init_arrays(&self, luminance_size: i32) {
        if self.luminances.len() < luminance_size {
            self.luminances = [0; luminance_size];
        }
        {
            let mut x: i32 = 0;
            while x < LUMINANCE_BUCKETS {
                {
                    self.buckets[x] = 0;
                }
                x += 1;
            }
        }
    }

    fn estimate_black_point(buckets: &Vec<i32>) -> Result<i32, NotFoundException> {
        // Find the tallest peak in the histogram.
        let num_buckets: i32 = buckets.len();
        let max_bucket_count: i32 = 0;
        let first_peak: i32 = 0;
        let first_peak_size: i32 = 0;
        {
            let mut x: i32 = 0;
            while x < num_buckets {
                {
                    if buckets[x] > first_peak_size {
                        first_peak = x;
                        first_peak_size = buckets[x];
                    }
                    if buckets[x] > max_bucket_count {
                        max_bucket_count = buckets[x];
                    }
                }
                x += 1;
            }
        }

        // Find the second-tallest peak which is somewhat far from the tallest peak.
        let second_peak: i32 = 0;
        let second_peak_score: i32 = 0;
        {
            let mut x: i32 = 0;
            while x < num_buckets {
                {
                    let distance_to_biggest: i32 = x - first_peak;
                    // Encourage more distant second peaks by multiplying by square of distance.
                    let score: i32 = buckets[x] * distance_to_biggest * distance_to_biggest;
                    if score > second_peak_score {
                        second_peak = x;
                        second_peak_score = score;
                    }
                }
                x += 1;
            }
        }

        // Make sure firstPeak corresponds to the black peak.
        if first_peak > second_peak {
            let temp: i32 = first_peak;
            first_peak = second_peak;
            second_peak = temp;
        }
        // than waste time trying to decode the image, and risk false positives.
        if second_peak - first_peak <= num_buckets / 16 {
            return Err(NotFoundException::get_not_found_instance());
        }
        // Find a valley between them that is low and closer to the white peak.
        let best_valley: i32 = second_peak - 1;
        let best_valley_score: i32 = -1;
        {
            let mut x: i32 = second_peak - 1;
            while x > first_peak {
                {
                    let from_first: i32 = x - first_peak;
                    let score: i32 = from_first
                        * from_first
                        * (second_peak - x)
                        * (max_bucket_count - buckets[x]);
                    if score > best_valley_score {
                        best_valley = x;
                        best_valley_score = score;
                    }
                }
                x -= 1;
            }
        }

        return Ok(best_valley << LUMINANCE_SHIFT);
    }
}

// BitArray.java
/**
 * <p>A simple, fast array of bits, represented compactly by an array of ints internally.</p>
 *
 * @author Sean Owen
 */

const EMPTY_BITS: Vec<i32> = Vec!([]);
const LOAD_FACTOR: f32 = 0.75f32;

#[derive(Cloneable, Eq, Hash)]
pub struct BitArray {
    bits: Vec<i32>,

    size: i32,
}

impl BitArray {
    fn new(bits: Option<&Vec<i32>>, size: Option<i32>) -> Self {
        let mut new_bit_array: Self;

        new_bit_array.size = size.unwrap_or(0);
        new_bit_array.bits = bits.unwrap_or(&BitArray::make_array(new_bit_array.size));

        new_bit_array
    }

    pub fn get_size(&self) -> i32 {
        return self.size;
    }

    pub fn get_size_in_bytes(&self) -> i32 {
        return (self.size + 7) / 8;
    }

    fn ensure_capacity(&self, new_size: i32) {
        if new_size > self.bits.len() * 32 {
            let new_bits: Vec<i32> = ::make_array(Math::ceil(new_size / LOAD_FACTOR) as i32);
            System::arraycopy(&self.bits, 0, &new_bits, 0, self.bits.len());
            self.bits = new_bits;
        }
    }

    /**
     * @param i bit to get
     * @return true iff bit i is set
     */
    pub fn get(&self, i: i32) -> bool {
        return (self.bits[i / 32] & (1 << (i & 0x1F))) != 0;
    }

    /**
     * Sets bit i.
     *
     * @param i bit to set
     */
    pub fn set(&self, i: i32) {
        self.bits[i / 32] |= 1 << (i & 0x1F);
    }

    /**
     * Flips bit i.
     *
     * @param i bit to set
     */
    pub fn flip(&self, i: i32) {
        self.bits[i / 32] ^= 1 << (i & 0x1F);
    }

    /**
     * @param from first bit to check
     * @return index of first bit that is set, starting from the given index, or size if none are set
     *  at or beyond this given index
     * @see #getNextUnset(int)
     */
    pub fn get_next_set(&self, from: i32) -> i32 {
        if from >= self.size {
            return self.size;
        }
        let bits_offset: i32 = from / 32;
        let current_bits: i32 = self.bits[bits_offset];
        // mask off lesser bits first
        current_bits &= -(1 << (from & 0x1F));
        while current_bits == 0 {
            if bits_offset += 1 == self.bits.len() {
                return self.size;
            }
            current_bits = self.bits[bits_offset];
        }
        let result: i32 = (bits_offset * 32) + Integer::number_of_trailing_zeros(current_bits);
        return Math::min(result, self.size);
    }

    /**
     * @param from index to start looking for unset bit
     * @return index of next unset bit, or {@code size} if none are unset until the end
     * @see #getNextSet(int)
     */
    pub fn get_next_unset(&self, from: i32) -> i32 {
        if from >= self.size {
            return self.size;
        }
        let bits_offset: i32 = from / 32;
        let current_bits: i32 = !self.bits[bits_offset];
        // mask off lesser bits first
        current_bits &= -(1 << (from & 0x1F));
        while current_bits == 0 {
            if bits_offset += 1 == self.bits.len() {
                return self.size;
            }
            current_bits = !self.bits[bits_offset];
        }
        let result: i32 = (bits_offset * 32) + Integer::number_of_trailing_zeros(current_bits);
        return Math::min(result, self.size);
    }

    /**
     * Sets a block of 32 bits, starting at bit i.
     *
     * @param i first bit to set
     * @param newBits the new value of the next 32 bits. Note again that the least-significant bit
     * corresponds to bit i, the next-least-significant to i+1, and so on.
     */
    pub fn set_bulk(&self, i: i32, new_bits: i32) {
        self.bits[i / 32] = new_bits;
    }

    /**
     * Sets a range of bits.
     *
     * @param start start of range, inclusive.
     * @param end end of range, exclusive
     */
    pub fn set_range(&self, start: i32, end: i32) -> Result<(), IllegalArgumentException> {
        if end < start || start < 0 || end > self.size {
            return Err(IllegalArgumentException::new());
        }
        if end == start {
            return;
        }
        // will be easier to treat this as the last actually set bit -- inclusive
        end -= 1;
        let first_int: i32 = start / 32;
        let last_int: i32 = end / 32;
        {
            let mut i: i32 = first_int;
            while i <= last_int {
                {
                    let first_bit: i32 = if i > first_int { 0 } else { start & 0x1F };
                    let last_bit: i32 = if i < last_int { 31 } else { end & 0x1F };
                    // Ones from firstBit to lastBit, inclusive
                    let mask: i32 = (2 << last_bit) - (1 << first_bit);
                    self.bits[i] |= mask;
                }
                i += 1;
            }
        }

        Ok(())
    }

    /**
     * Clears all bits (sets to false).
     */
    pub fn clear(&self) {
        let max: i32 = self.bits.len();
        {
            let mut i: i32 = 0;
            while i < max {
                {
                    self.bits[i] = 0;
                }
                i += 1;
            }
        }
    }

    /**
     * Efficient method to check if a range of bits is set, or not set.
     *
     * @param start start of range, inclusive.
     * @param end end of range, exclusive
     * @param value if true, checks that bits in range are set, otherwise checks that they are not set
     * @return true iff all bits are set or not set in range, according to value argument
     * @throws IllegalArgumentException if end is less than start or the range is not contained in the array
     */
    pub fn is_range(
        &self,
        start: i32,
        end: i32,
        value: bool,
    ) -> Result<bool, IllegalArgumentException> {
        if end < start || start < 0 || end > self.size {
            return Err(IllegalArgumentException::new());
        }
        if end == start {
            // empty range matches
            return Ok(true);
        }
        // will be easier to treat this as the last actually set bit -- inclusive
        end -= 1;
        let first_int: i32 = start / 32;
        let last_int: i32 = end / 32;
        {
            let mut i: i32 = first_int;
            while i <= last_int {
                {
                    let first_bit: i32 = if i > first_int { 0 } else { start & 0x1F };
                    let last_bit: i32 = if i < last_int { 31 } else { end & 0x1F };
                    // Ones from firstBit to lastBit, inclusive
                    let mask: i32 = (2 << last_bit) - (1 << first_bit);
                    // equals the mask, or we're looking for 0s and the masked portion is not all 0s
                    if (self.bits[i] & mask) != (if value { mask } else { 0 }) {
                        return Ok(false);
                    }
                }
                i += 1;
            }
        }

        return Ok(true);
    }

    pub fn append_bit(&self, bit: bool) {
        self.ensure_capacity(self.size + 1);
        if bit {
            self.bits[self.size / 32] |= 1 << (self.size & 0x1F);
        }
        self.size += 1;
    }

    /**
     * Appends the least-significant bits, from value, in order from most-significant to
     * least-significant. For example, appending 6 bits from 0x000001E will append the bits
     * 0, 1, 1, 1, 1, 0 in that order.
     *
     * @param value {@code int} containing bits to append
     * @param numBits bits from value to append
     */
    pub fn append_bits(&self, value: i32, num_bits: i32) -> Result<(), IllegalArgumentException> {
        if num_bits < 0 || num_bits > 32 {
            return Err(IllegalArgumentException::new(
                "Num bits must be between 0 and 32",
            ));
        }
        let next_size: i32 = self.size;
        self.ensure_capacity(next_size + num_bits);
        {
            let num_bits_left: i32 = num_bits - 1;
            while num_bits_left >= 0 {
                {
                    if (value & (1 << num_bits_left)) != 0 {
                        self.bits[next_size / 32] |= 1 << (next_size & 0x1F);
                    }
                    next_size += 1;
                }
                num_bits_left -= 1;
            }
        }

        self.size = next_size;
        Ok(())
    }

    pub fn append_bit_array(&self, other: &BitArray) {
        let other_size: i32 = other.size;
        self.ensure_capacity(self.size + other_size);
        {
            let mut i: i32 = 0;
            while i < other_size {
                {
                    self.append_bit(&other.get(i));
                }
                i += 1;
            }
        }
    }

    pub fn xor(&self, other: &BitArray) -> Result((), IllegalArgumentException) {
        if self.size != other.size {
            return Err(IllegalArgumentException::new("Sizes don't match"));
        }
        {
            let mut i: i32 = 0;
            while i < self.bits.len() {
                {
                    // The last int could be incomplete (i.e. not have 32 bits in
                    // it) but there is no problem since 0 XOR 0 == 0.
                    self.bits[i] ^= other.bits[i];
                }
                i += 1;
            }
        }
        Ok(())
    }

    /**
     *
     * @param bitOffset first bit to start writing
     * @param array array to write into. Bytes are written most-significant byte first. This is the opposite
     *  of the internal representation, which is exposed by {@link #getBitArray()}
     * @param offset position in array to start writing
     * @param numBytes how many bytes to write
     */
    pub fn to_bytes(&self, bit_offset: i32, array: &Vec<i8>, offset: i32, num_bytes: i32) {
        {
            let mut i: i32 = 0;
            while i < num_bytes {
                {
                    let the_byte: i32 = 0;
                    {
                        let mut j: i32 = 0;
                        while j < 8 {
                            {
                                if self.get(bit_offset) {
                                    the_byte |= 1 << (7 - j);
                                }
                                bit_offset += 1;
                            }
                            j += 1;
                        }
                    }

                    array[offset + i] = the_byte as i8;
                }
                i += 1;
            }
        }
    }

    /**
     * @return underlying array of ints. The first element holds the first 32 bits, and the least
     *         significant bit is bit 0.
     */
    pub fn get_bit_array(&self) -> Vec<i32> {
        return self.bits;
    }

    /**
     * Reverses all bits in the array.
     */
    pub fn reverse(&self) {
        let new_bits: [i32; self.bits.len()] = [0; self.bits.len()];
        // reverse all int's first
        let mut len: i32 = (self.size - 1) / 32;
        let old_bits_len: i32 = len + 1;
        {
            let mut i: i32 = 0;
            while i < old_bits_len {
                {
                    new_bits[len - i] = Integer::reverse(self.bits[i]);
                }
                i += 1;
            }
        }

        // now correct the int's if the bit size isn't a multiple of 32
        if self.size != old_bits_len * 32 {
            let left_offset: i32 = old_bits_len * 32 - self.size;
            let current_int: i32 = new_bits[0] >> /* >>> */ left_offset;
            {
                let mut i: i32 = 1;
                while i < old_bits_len {
                    {
                        let next_int: i32 = new_bits[i];
                        current_int |= next_int << (32 - left_offset);
                        new_bits[i - 1] = current_int;
                        current_int = next_int >> /* >>> */ left_offset;
                    }
                    i += 1;
                }
            }

            new_bits[old_bits_len - 1] = current_int;
        }
        self.bits = new_bits;
    }

    fn make_array(size: i32) -> Vec<i32> {
        return [0; (size + 31) / 32];
    }

    pub fn to_string(&self) -> String {
        let result: StringBuilder = StringBuilder::new(self.size + (self.size / 8) + 1);
        {
            let mut i: i32 = 0;
            while i < self.size {
                {
                    if (i & 0x07) == 0 {
                        result.append(' ');
                    }
                    result.append(if self.get(i) { 'X' } else { '.' });
                }
                i += 1;
            }
        }

        return result.to_string();
    }

    /*pub fn  clone(&self) -> BitArray  {
        return BitArray::new(&self.bits.clone(), self.size);
    }*/
}

// BitMatrix.java
/**
 * <p>Represents a 2D matrix of bits. In function arguments below, and throughout the common
 * module, x is the column position, and y is the row position. The ordering is always x, y.
 * The origin is at the top-left.</p>
 *
 * <p>Internally the bits are represented in a 1-D array of 32-bit ints. However, each row begins
 * with a new int. This is done intentionally so that we can copy out a row into a BitArray very
 * efficiently.</p>
 *
 * <p>The ordering of bits is row-major. Within each int, the least significant bits are used first,
 * meaning they represent lower x values. This is compatible with BitArray's implementation.</p>
 *
 * @author Sean Owen
 * @author dswitkin@google.com (Daniel Switkin)
 */
#[derive(Cloneable, Eq, Hash)]
pub struct BitMatrix {
    width: i32,

    height: i32,

    row_size: i32,

    bits: Vec<i32>,
}

impl BitMatrix {
    /**
     * Creates an empty square {@code BitMatrix}.
     *
     * @param dimension height and width
     */

    /**
     * Creates an empty {@code BitMatrix}.
     *
     * @param width bit matrix width
     * @param height bit matrix height
     */

    fn new(
        width: i32,
        height: i32,
        row_size: Option<i32>,
        bits: Option<&Vec<i32>>,
    ) -> Result<Self, IllegalArgumentException> {
        if width < 1 || height < 1 {
            return Err(IllegalArgumentException::new(
                "Both dimensions must be greater than 0",
            ));
        }

        Ok(Self {
            width: width,
            height: height,
            row_size: row_size.unwrap_or((width + 31) / 32),
            bits: bits.unwrap_or([0; row_size * height]),
        })
    }

    fn new_dimension(dimension: i32) {
        BitMatrix::new(dimension, dimension, None, None)
    }

    /**
     * Interprets a 2D array of booleans as a {@code BitMatrix}, where "true" means an "on" bit.
     *
     * @param image bits of the image, as a row-major 2D array. Elements are arrays representing rows
     * @return {@code BitMatrix} representation of image
     */
    pub fn parse(image: &Vec<Vec<bool>>) -> BitMatrix {
        let height: i32 = image.len();
        let width: i32 = image[0].len();
        let bits: BitMatrix = BitMatrix::new(width, height, None, None);
        {
            let mut i: i32 = 0;
            while i < height {
                {
                    let image_i: Vec<bool> = image[i];
                    {
                        let mut j: i32 = 0;
                        while j < width {
                            {
                                if image_i[j] {
                                    bits.set(j, i);
                                }
                            }
                            j += 1;
                        }
                    }
                }
                i += 1;
            }
        }

        return bits;
    }

    pub fn parse(
        string_representation: &String,
        set_string: &String,
        unset_string: &String,
    ) -> Result<BitMatrix, IllegalArgumentException> {
        if string_representation == null {
            return Err(IllegalArgumentException::new());
        }
        let mut bits: [bool; string_representation.length()] =
            [false; string_representation.length()];
        let bits_pos: i32 = 0;
        let row_start_pos: i32 = 0;
        let row_length: i32 = -1;
        let n_rows: i32 = 0;
        let mut pos: i32 = 0;
        while pos < string_representation.length() {
            if string_representation.char_at(pos) == '\n'
                || string_representation.char_at(pos) == '\r'
            {
                if bits_pos > row_start_pos {
                    if row_length == -1 {
                        row_length = bits_pos - row_start_pos;
                    } else if bits_pos - row_start_pos != row_length {
                        return Err(IllegalArgumentException::new("row lengths do not match"));
                    }
                    row_start_pos = bits_pos;
                    n_rows += 1;
                }
                pos += 1;
            } else if string_representation[..pos].starts_with(&set_string) {
                pos += set_string.length();
                bits[bits_pos] = true;
                bits_pos += 1;
            } else if string_representation[..pos].starts_with(&unset_string) {
                pos += unset_string.length();
                bits[bits_pos] = false;
                bits_pos += 1;
            } else {
                return Err(IllegalArgumentException::new(format!(
                    "illegal character encountered: {}",
                    string_representation.substring(pos)
                )));
            }
        }
        // no EOL at end?
        if bits_pos > row_start_pos {
            if row_length == -1 {
                row_length = bits_pos - row_start_pos;
            } else if bits_pos - row_start_pos != row_length {
                return Err(IllegalArgumentException::new("row lengths do not match"));
            }
            n_rows += 1;
        }
        let matrix: BitMatrix = BitMatrix::new(row_length, n_rows, None, None);
        {
            let mut i: i32 = 0;
            while i < bits_pos {
                {
                    if bits[i] {
                        matrix.set(i % row_length, i / row_length);
                    }
                }
                i += 1;
            }
        }

        return Ok(matrix);
    }

    /**
     * <p>Gets the requested bit, where true means black.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     * @return value of given bit in matrix
     */
    pub fn get(&self, x: i32, y: i32) -> bool {
        let offset: i32 = y * self.row_size + (x / 32);
        return ((self.bits[offset] >> /* >>> */ (x & 0x1f)) & 1) != 0;
    }

    /**
     * <p>Sets the given bit to true.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     */
    pub fn set(&self, x: i32, y: i32) {
        let mut offset: i32 = y * self.row_size + (x / 32);
        self.bits[offset] |= 1 << (x & 0x1f);
    }

    pub fn unset(&self, x: i32, y: i32) {
        let mut offset: i32 = y * self.row_size + (x / 32);
        self.bits[offset] &= !(1 << (x & 0x1f));
    }

    /**
     * <p>Flips the given bit.</p>
     *
     * @param x The horizontal component (i.e. which column)
     * @param y The vertical component (i.e. which row)
     */
    pub fn flip(&self, x: i32, y: i32) {
        let mut offset: i32 = y * self.row_size + (x / 32);
        self.bits[offset] ^= 1 << (x & 0x1f);
    }

    /**
     * <p>Flips every bit in the matrix.</p>
     */
    pub fn flip(&self) {
        let max: i32 = self.bits.len();
        {
            let mut i: i32 = 0;
            while i < max {
                {
                    self.bits[i] = !self.bits[i];
                }
                i += 1;
            }
        }
    }

    /**
     * Exclusive-or (XOR): Flip the bit in this {@code BitMatrix} if the corresponding
     * mask bit is set.
     *
     * @param mask XOR mask
     */
    pub fn xor(&self, mask: &BitMatrix) -> Result<(), IllegalArgumentException> {
        if self.width != mask.width || self.height != mask.height || self.row_size != mask.rowSize {
            return Err(IllegalArgumentException::new(
                "input matrix dimensions do not match",
            ));
        }
        let row_array: BitArray = BitArray::new(None, Some(self.width));
        {
            let mut y: i32 = 0;
            while y < self.height {
                {
                    let mut offset: i32 = y * self.row_size;
                    let row: Vec<i32> = mask.get_row(y, &row_array).get_bit_array();
                    {
                        let mut x: i32 = 0;
                        while x < self.row_size {
                            {
                                self.bits[offset + x] ^= row[x];
                            }
                            x += 1;
                        }
                    }
                }
                y += 1;
            }
        }
        Ok(())
    }

    /**
     * Clears all bits (sets to false).
     */
    pub fn clear(&self) {
        let max: i32 = self.bits.len();
        {
            let mut i: i32 = 0;
            while i < max {
                {
                    self.bits[i] = 0;
                }
                i += 1;
            }
        }
    }

    /**
     * <p>Sets a square region of the bit matrix to true.</p>
     *
     * @param left The horizontal position to begin at (inclusive)
     * @param top The vertical position to begin at (inclusive)
     * @param width The width of the region
     * @param height The height of the region
     */
    pub fn set_region(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> Result<(), IllegalArgumentException> {
        if top < 0 || left < 0 {
            return Err(IllegalArgumentException::new(
                "Left and top must be nonnegative",
            ));
        }
        if height < 1 || width < 1 {
            return Err(IllegalArgumentException::new(
                "Height and width must be at least 1",
            ));
        }
        let right: i32 = left + width;
        let bottom: i32 = top + height;
        if bottom > self.height || right > self.width {
            return Err(IllegalArgumentException::new(
                "The region must fit inside the matrix",
            ));
        }
        {
            let mut y: i32 = top;
            while y < bottom {
                {
                    let mut offset: i32 = y * self.row_size;
                    {
                        let mut x: i32 = left;
                        while x < right {
                            {
                                self.bits[offset + (x / 32)] |= 1 << (x & 0x1f);
                            }
                            x += 1;
                        }
                    }
                }
                y += 1;
            }
        }
        Ok(())
    }

    /**
     * A fast method to retrieve one row of data from the matrix as a BitArray.
     *
     * @param y The row to retrieve
     * @param row An optional caller-allocated BitArray, will be allocated if null or too small
     * @return The resulting BitArray - this reference should always be used even when passing
     *         your own row
     */
    pub fn get_row(&self, y: i32, row: &BitArray) -> BitArray {
        if row == null || row.get_size() < self.width {
            row = &BitArray::new(None, Some(self.width));
        } else {
            row.clear();
        }
        let offset: i32 = y * self.row_size;
        {
            let mut x: i32 = 0;
            while x < self.row_size {
                {
                    row.set_bulk(x * 32, self.bits[offset + x]);
                }
                x += 1;
            }
        }

        return row;
    }

    /**
     * @param y row to set
     * @param row {@link BitArray} to copy from
     */
    pub fn set_row(&self, y: i32, row: &BitArray) {
        System::arraycopy(
            &row.get_bit_array(),
            0,
            &self.bits,
            y * self.row_size,
            self.row_size,
        );
    }

    /**
     * Modifies this {@code BitMatrix} to represent the same but rotated the given degrees (0, 90, 180, 270)
     *
     * @param degrees number of degrees to rotate through counter-clockwise (0, 90, 180, 270)
     */
    pub fn rotate(&self, degrees: i32) -> Result<(), IllegalArgumentException> {
        match degrees % 360 {
            0 => Ok(()),
            90 => {
                self.rotate90();
                Ok(())
            }
            180 => {
                self.rotate180();
                Ok(())
            }
            270 => {
                self.rotate90();
                self.rotate180();
                Ok(())
            }
            _ => Err(IllegalArgumentException::new(
                "degrees must be a multiple of 0, 90, 180, or 270",
            )),
        }
    }

    /**
     * Modifies this {@code BitMatrix} to represent the same but rotated 180 degrees
     */
    pub fn rotate180(&self) {
        let top_row: BitArray = BitArray::new(None, Some(self.width));
        let bottom_row: BitArray = BitArray::new(None, Some(self.width));
        let max_height: i32 = (self.height + 1) / 2;
        {
            let mut i: i32 = 0;
            while i < max_height {
                {
                    top_row = self.get_row(i, &top_row);
                    let bottom_row_index: i32 = self.height - 1 - i;
                    bottom_row = self.get_row(bottom_row_index, &bottom_row);
                    top_row.reverse();
                    bottom_row.reverse();
                    self.set_row(i, &bottom_row);
                    self.set_row(bottom_row_index, &top_row);
                }
                i += 1;
            }
        }
    }

    /**
     * Modifies this {@code BitMatrix} to represent the same but rotated 90 degrees counterclockwise
     */
    pub fn rotate90(&self) {
        let new_width: i32 = self.height;
        let new_height: i32 = self.width;
        let new_row_size: i32 = (new_width + 31) / 32;
        let new_bits: [i32; new_row_size * new_height] = [0; new_row_size * new_height];
        {
            let mut y: i32 = 0;
            while y < self.height {
                {
                    {
                        let mut x: i32 = 0;
                        while x < self.width {
                            {
                                let offset: i32 = y * self.row_size + (x / 32);
                                if ((self.bits[offset] >> /* >>> */ (x & 0x1f)) & 1) != 0 {
                                    let new_offset: i32 =
                                        (new_height - 1 - x) * new_row_size + (y / 32);
                                    new_bits[new_offset] |= 1 << (y & 0x1f);
                                }
                            }
                            x += 1;
                        }
                    }
                }
                y += 1;
            }
        }

        self.width = new_width;
        self.height = new_height;
        self.row_size = new_row_size;
        self.bits = new_bits;
    }

    /**
     * This is useful in detecting the enclosing rectangle of a 'pure' barcode.
     *
     * @return {@code left,top,width,height} enclosing rectangle of all 1 bits, or null if it is all white
     */
    pub fn get_enclosing_rectangle(&self) -> Option<Vec<i32>> {
        let mut left: i32 = self.width;
        let mut top: i32 = self.height;
        let mut right: i32 = -1;
        let mut bottom: i32 = -1;
        {
            let mut y: i32 = 0;
            while y < self.height {
                {
                    {
                        let mut x32: i32 = 0;
                        while x32 < self.row_size {
                            {
                                let the_bits: i32 = self.bits[y * self.row_size + x32];
                                if the_bits != 0 {
                                    if y < top {
                                        top = y;
                                    }
                                    if y > bottom {
                                        bottom = y;
                                    }
                                    if x32 * 32 < left {
                                        let mut bit: i32 = 0;
                                        while (the_bits << (31 - bit)) == 0 {
                                            bit += 1;
                                        }
                                        if (x32 * 32 + bit) < left {
                                            left = x32 * 32 + bit;
                                        }
                                    }
                                    if x32 * 32 + 31 > right {
                                        let mut bit: i32 = 31;
                                        while (the_bits >> /* >>> */ bit) == 0 {
                                            bit -= 1;
                                        }
                                        if (x32 * 32 + bit) > right {
                                            right = x32 * 32 + bit;
                                        }
                                    }
                                }
                            }
                            x32 += 1;
                        }
                    }
                }
                y += 1;
            }
        }

        if right < left || bottom < top {
            return null;
        }
        return Some(vec![left, top, right - left + 1, bottom - top + 1]);
    }

    /**
     * This is useful in detecting a corner of a 'pure' barcode.
     *
     * @return {@code x,y} coordinate of top-left-most 1 bit, or null if it is all white
     */
    pub fn get_top_left_on_bit(&self) -> Option<Vec<i32>> {
        let bits_offset: i32 = 0;
        while bits_offset < self.bits.len() && self.bits[bits_offset] == 0 {
            bits_offset += 1;
        }
        if bits_offset == self.bits.len() {
            return null;
        }
        let y: i32 = bits_offset / self.row_size;
        let mut x: i32 = (bits_offset % self.row_size) * 32;
        let the_bits: i32 = self.bits[bits_offset];
        let mut bit: i32 = 0;
        while (the_bits << (31 - bit)) == 0 {
            bit += 1;
        }
        x += bit;
        return Some(vec![x, y]);
    }

    pub fn get_bottom_right_on_bit(&self) -> Vec<i32> {
        let bits_offset: i32 = self.bits.len() - 1;
        while bits_offset >= 0 && self.bits[bits_offset] == 0 {
            bits_offset -= 1;
        }
        if bits_offset < 0 {
            return null;
        }
        let y: i32 = bits_offset / self.row_size;
        let mut x: i32 = (bits_offset % self.row_size) * 32;
        let the_bits: i32 = self.bits[bits_offset];
        let mut bit: i32 = 31;
        while (the_bits >> /* >>> */ bit) == 0 {
            bit -= 1;
        }
        x += bit;
        return vec![x, y];
    }

    /**
     * @return The width of the matrix
     */
    pub fn get_width(&self) -> i32 {
        return self.width;
    }

    /**
     * @return The height of the matrix
     */
    pub fn get_height(&self) -> i32 {
        return self.height;
    }

    /**
     * @return The row size of the matrix
     */
    pub fn get_row_size(&self) -> i32 {
        return self.row_size;
    }

    pub fn hash_code(&self) -> i32 {
        let mut hash: i32 = self.width;
        hash = 31 * hash + self.width;
        hash = 31 * hash + self.height;
        hash = 31 * hash + self.row_size;
        hash = 31 * hash + Arrays::hash_code(&self.bits);
        return hash;
    }

    /**
     * @param setString representation of a set bit
     * @param unsetString representation of an unset bit
     * @param lineSeparator newline character in string representation
     * @return string representation of entire matrix utilizing given strings and line separator
     * @deprecated call {@link #toString(String,String)} only, which uses \n line separator always
     */
    pub fn to_string(
        &self,
        set_string: Option<&str>,
        unset_string: Option<&str>,
        line_separator: Option<&str>,
    ) -> String {
        return self.build_to_string(
            set_string.unwrap_or("X "),
            unset_string.unwrap_or("  "),
            line_separator.unwrap_or("\n"),
        );
    }

    fn build_to_string(
        &self,
        set_string: &String,
        unset_string: &String,
        line_separator: &String,
    ) -> String {
        let result: StringBuilder = StringBuilder::new(self.height * (self.width + 1));
        {
            let mut y: i32 = 0;
            while y < self.height {
                {
                    {
                        let mut x: i32 = 0;
                        while x < self.width {
                            {
                                result.append(if self.get(x, y) {
                                    set_string
                                } else {
                                    unset_string
                                });
                            }
                            x += 1;
                        }
                    }

                    result.append(&line_separator);
                }
                y += 1;
            }
        }

        return result.to_string();
    }

    /*pub fn  clone(&self) -> BitMatrix  {
        return BitMatrix::new(self.width, self.height, self.row_size, &self.bits.clone());
    }*/
}

// BitSource.java
/**
 * <p>This provides an easy abstraction to read bits at a time from a sequence of bytes, where the
 * number of bits read is not often a multiple of 8.</p>
 *
 * <p>This class is thread-safe but not reentrant -- unless the caller modifies the bytes array
 * it passed in, in which case all bets are off.</p>
 *
 * @author Sean Owen
 */
pub struct BitSource {
    bytes: Vec<i8>,

    byte_offset: i32,

    bit_offset: i32,
}

impl BitSource {
    /**
     * @param bytes bytes from which this will read bits. Bits will be read from the first byte first.
     * Bits are read within a byte from most-significant to least-significant bit.
     */
    pub fn new(bytes: &Vec<i8>) -> Self {
        let mut new_bs;
        new_bs.bytes = bytes;

        new_bs
    }

    /**
     * @return index of next bit in current byte which would be read by the next call to {@link #readBits(int)}.
     */
    pub fn get_bit_offset(&self) -> i32 {
        return self.bit_offset;
    }

    /**
     * @return index of next byte in input byte array which would be read by the next call to {@link #readBits(int)}.
     */
    pub fn get_byte_offset(&self) -> i32 {
        return self.byte_offset;
    }

    /**
     * @param numBits number of bits to read
     * @return int representing the bits read. The bits will appear as the least-significant
     *         bits of the int
     * @throws IllegalArgumentException if numBits isn't in [1,32] or more than is available
     */
    pub fn read_bits(&self, num_bits: i32) -> Result<i32, IllegalArgumentException> {
        if num_bits < 1 || num_bits > 32 || num_bits > self.available() {
            return Err(IllegalArgumentException::new(&String::value_of(num_bits)));
        }
        let mut result: i32 = 0;
        // First, read remainder from current byte
        if self.bit_offset > 0 {
            let bits_left: i32 = 8 - self.bit_offset;
            let to_read: i32 = Math::min(num_bits, bits_left);
            let bits_to_not_read: i32 = bits_left - to_read;
            let mask: i32 = (0xFF >> (8 - to_read)) << bits_to_not_read;
            result = (self.bytes[self.byte_offset] & mask) >> bits_to_not_read;
            num_bits -= to_read;
            self.bit_offset += to_read;
            if self.bit_offset == 8 {
                self.bit_offset = 0;
                self.byte_offset += 1;
            }
        }
        // Next read whole bytes
        if num_bits > 0 {
            while num_bits >= 8 {
                result = (result << 8) | (self.bytes[self.byte_offset] & 0xFF);
                self.byte_offset += 1;
                num_bits -= 8;
            }
            // Finally read a partial byte
            if num_bits > 0 {
                let bits_to_not_read: i32 = 8 - num_bits;
                let mask: i32 = (0xFF >> bits_to_not_read) << bits_to_not_read;
                result = (result << num_bits)
                    | ((self.bytes[self.byte_offset] & mask) >> bits_to_not_read);
                self.bit_offset += num_bits;
            }
        }
        return Ok(result);
    }

    /**
     * @return number of bits that can be read successfully
     */
    pub fn available(&self) -> i32 {
        return 8 * (self.bytes.len() - self.byte_offset) - self.bit_offset;
    }
}

// CharacterSetECI.java
/**
 * Encapsulates a Character Set ECI, according to "Extended Channel Interpretations" 5.3.1.1
 * of ISO 18004.
 *
 * @author Sean Owen
 */
pub enum CharacterSetECI {
    // Enum name is a Java encoding valid for java.lang and java.io
    Cp437,
    ISO8859_1,
    ISO8859_2,
    ISO8859_3,
    ISO8859_4,
    ISO8859_5,
    // ISO8859_6(8, "ISO-8859-6"),
    ISO8859_7,
    // ISO8859_8(10, "ISO-8859-8"),
    ISO8859_9,
    // ISO8859_10(12, "ISO-8859-10"),
    // ISO8859_11(13, "ISO-8859-11"),
    ISO8859_13,
    // ISO8859_14(16, "ISO-8859-14"),
    ISO8859_15,
    ISO8859_16,
    SJIS,
    Cp1250,
    Cp1251,
    Cp1252,
    Cp1256,
    UnicodeBigUnmarked,
    UTF8,
    ASCII,
    Big5,
    GB18030,
    EUC_KR, /*

            // Enum name is a Java encoding valid for java.lang and java.io
            Cp437(new int[]{0,2}),
            ISO8859_1(new int[]{1,3}, "ISO-8859-1"),
            ISO8859_2(4, "ISO-8859-2"),
            ISO8859_3(5, "ISO-8859-3"),
            ISO8859_4(6, "ISO-8859-4"),
            ISO8859_5(7, "ISO-8859-5"),
            // ISO8859_6(8, "ISO-8859-6"),
            ISO8859_7(9, "ISO-8859-7"),
            // ISO8859_8(10, "ISO-8859-8"),
            ISO8859_9(11, "ISO-8859-9"),
            // ISO8859_10(12, "ISO-8859-10"),
            // ISO8859_11(13, "ISO-8859-11"),
            ISO8859_13(15, "ISO-8859-13"),
            // ISO8859_14(16, "ISO-8859-14"),
            ISO8859_15(17, "ISO-8859-15"),
            ISO8859_16(18, "ISO-8859-16"),
            SJIS(20, "Shift_JIS"),
            Cp1250(21, "windows-1250"),
            Cp1251(22, "windows-1251"),
            Cp1252(23, "windows-1252"),
            Cp1256(24, "windows-1256"),
            UnicodeBigUnmarked(25, "UTF-16BE", "UnicodeBig"),
            UTF8(26, "UTF-8"),
            ASCII(new int[] {27, 170}, "US-ASCII"),
            Big5(28),
            GB18030(29, "GB2312", "EUC_CN", "GBK"),
            EUC_KR(30, "EUC-KR");

            */
}

impl CharacterSetECI {
    /*
    fn new( value: i32) -> CharacterSetECI {
        this( : vec![i32; 1] = vec![value, ]
        );
    }

    fn new( value: i32,  other_encoding_names: &String) -> CharacterSetECI {
        let .values =  : vec![i32; 1] = vec![value, ]
        ;
        let .otherEncodingNames = other_encoding_names;
    }

    fn new( values: &Vec<i32>,  other_encoding_names: &String) -> CharacterSetECI {
        let .values = values;
        let .otherEncodingNames = other_encoding_names;
    }

    pub fn  get_charset(&self) -> Charset  {
        return Charset::for_name(&name());
    }
    */

    /**
     * @param charset Java character set object
     * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
     *   but unsupported
     */
    pub fn get_character_set_e_c_i(charset: &str) -> Result<Option<CharacterSetECI>, &'static str> {
        //return NAME_TO_ECI::get(&charset.name());
        let eci = match charset {
            "Cp437" => Self::Cp437,
            "ISO-8859-1" => Self::ISO8859_1,
            "ISO-8859-2" => Self::ISO8859_2,
            "ISO-8859-3" => Self::ISO8859_3,
            "ISO-8859-4" => Self::ISO8859_4,
            "ISO-8859-5" => Self::ISO8859_5,
            "ISO-8859-7" => Self::ISO8859_7,
            "ISO-8859-9" => Self::ISO8859_9,
            "ISO-8859-13" => Self::ISO8859_13,
            "ISO-8859-15" => Self::ISO8859_15,
            "ISO-8859-16" => Self::ISO8859_16,
            "Shift_JIS" => Self::SJIS,
            "windows-1250" => Self::Cp1250,
            "windows-1251" => Self::Cp1251,
            "windows-1252" => Self::Cp1252,
            "windows-1256" => Self::Cp1256,
            "UTF-16BE" | "UnicodeBig" => Self::UnicodeBigUnmarked,
            "UTF-8" => Self::UTF8,
            "US-ASCII" => Self::ASCII,
            "Big5" => Self::Big5,
            "GB2312" | "EUC_CN" | "GBK" => Self::GB18030,
            "EUC-KR" => Self::EUC_KR,
            _ => return Err("Invalid charset"),
        };
        Ok(Some(eci))
    }

    /**
     * @param value character set ECI value
     * @return {@code CharacterSetECI} representing ECI of given value, or null if it is legal but
     *   unsupported
     * @throws FormatException if ECI value is invalid
     */
    pub fn get_character_set_e_c_i_by_value(
        value: i32,
    ) -> Result<Option<CharacterSetECI>, FormatException> {
        if value < 0 || value >= 900 {
            return Err(FormatException::get_format_instance());
        }
        let eci = match value {
            0 | 2 => Self::Cp437,
            1 | 3 => Self::ISO8859_1,
            4 => Self::ISO8859_2,
            5 => Self::ISO8859_3,
            6 => Self::ISO8859_4,
            7 => Self::ISO8859_5,
            9 => Self::ISO8859_7,
            11 => Self::ISO8859_9,
            15 => Self::ISO8859_13,
            17 => Self::ISO8859_15,
            18 => Self::ISO8859_16,
            20 => Self::SJIS,
            21 => Self::Cp1250,
            22 => Self::Cp1251,
            23 => Self::Cp1252,
            24 => Self::Cp1256,
            25 => Self::UnicodeBigUnmarked,
            26 => Self::UTF8,
            27 | 170 => Self::ASCII,
            28 => Self::Big5,
            29 => Self::GB18030,
            30 => Self::EUC_KR,
            _ => return Err(FormatException::get_format_instance()),
        };
        return Ok(Some(eci));
    }

    pub fn get_value(v: Self) -> i32 {
        match v {
            CharacterSetECI::Cp437 => 0,
            CharacterSetECI::ISO8859_1 => 1,
            CharacterSetECI::ISO8859_2 => 4,
            CharacterSetECI::ISO8859_3 => 5,
            CharacterSetECI::ISO8859_4 => 6,
            CharacterSetECI::ISO8859_5 => 7,
            CharacterSetECI::ISO8859_7 => 9,
            CharacterSetECI::ISO8859_9 => 11,
            CharacterSetECI::ISO8859_13 => 15,
            CharacterSetECI::ISO8859_15 => 17,
            CharacterSetECI::ISO8859_16 => 18,
            CharacterSetECI::SJIS => 20,
            CharacterSetECI::Cp1250 => 21,
            CharacterSetECI::Cp1251 => 22,
            CharacterSetECI::Cp1252 => 23,
            CharacterSetECI::Cp1256 => 24,
            CharacterSetECI::UnicodeBigUnmarked => 25,
            CharacterSetECI::UTF8 => 26,
            CharacterSetECI::ASCII => 27,
            CharacterSetECI::Big5 => 28,
            CharacterSetECI::GB18030 => 29,
            CharacterSetECI::EUC_KR => 30,
        }
    }

    /*
     * @param name character set ECI encoding name
     * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
     *   but unsupported
     */
    /*
    pub fn  get_character_set_e_c_i_by_name( name: &str) -> Result<CharacterSetECI, &'static str>  {
         return NAME_TO_ECI::get(&name);
     }
      */
}

// DecoderResult.java
/**
 * <p>Encapsulates the result of decoding a matrix of bits. This typically
 * applies to 2D barcode formats. For now it contains the raw bytes obtained,
 * as well as a String interpretation of those bytes, if applicable.</p>
 *
 * @author Sean Owen
 */
pub struct DecoderResult {
    raw_bytes: Vec<i8>,

    num_bits: i32,

    text: String,

    byte_segments: List<Vec<i8>>,

    ec_level: String,

    errors_corrected: Integer,

    erasures: Integer,

    other: Object,

    structured_append_parity: i32,

    structured_append_sequence_number: i32,

    symbology_modifier: i32,
}

impl DecoderResult {
    pub fn new(
        raw_bytes: &Vec<i8>,
        text: &String,
        byte_segments: &Vec<Vec<i8>>,
        ec_level: &String,
        sa_sequence: Option<i32>,
        sa_parity: Option<i32>,
        symbology_modifier: Option<i32>,
    ) -> Self {
        let mut new_dr: Self;

        new_dr.raw_bytes = raw_bytes;
        new_dr.text = text;
        new_dr.byte_segments = byte_segments;
        new_dr.ec_level = ec_level;

        new_dr.symbology_modifier = symbology_modifier.unwrap_or(0);

        new_dr.structured_append_parity = sa_parity.unwrap_or(-1);
        new_dr.structured_append_sequence_number = sa_sequence.unwrap_or(-1);

        new_dr.num_bits = raw_bytes.len() * 8;

        new_dr
    }
    /**
     * @return raw bytes representing the result, or {@code null} if not applicable
     */
    pub fn get_raw_bytes(&self) -> Option<Vec<i8>> {
        return Some(self.raw_bytes);
    }

    /**
     * @return how many bits of {@link #getRawBytes()} are valid; typically 8 times its length
     * @since 3.3.0
     */
    pub fn get_num_bits(&self) -> i32 {
        return self.num_bits;
    }

    /**
     * @param numBits overrides the number of bits that are valid in {@link #getRawBytes()}
     * @since 3.3.0
     */
    pub fn set_num_bits(&self, num_bits: i32) {
        self.numBits = num_bits;
    }

    /**
     * @return text representation of the result
     */
    pub fn get_text(&self) -> String {
        return self.text;
    }

    /**
     * @return list of byte segments in the result, or {@code null} if not applicable
     */
    pub fn get_byte_segments(&self) -> Option<Vector<Vec<i8>>> {
        return self.byte_segments;
    }

    /**
     * @return name of error correction level used, or {@code null} if not applicable
     */
    pub fn get_e_c_level(&self) -> Option<String> {
        return Some(self.ec_level);
    }

    /**
     * @return number of errors corrected, or {@code null} if not applicable
     */
    pub fn get_errors_corrected(&self) -> Option<Integer> {
        return self.errors_corrected;
    }

    pub fn set_errors_corrected(&self, errors_corrected: &Integer) {
        self.errorsCorrected = errors_corrected;
    }

    /**
     * @return number of erasures corrected, or {@code null} if not applicable
     */
    pub fn get_erasures(&self) -> Option<Integer> {
        return self.erasures;
    }

    pub fn set_erasures(&self, erasures: &Integer) {
        self.erasures = erasures;
    }

    /**
     * @return arbitrary additional metadata
     */
    pub fn get_other(&self) -> Object {
        return self.other;
    }

    pub fn set_other(&self, other: &Object) {
        self.other = other;
    }

    pub fn has_structured_append(&self) -> bool {
        return self.structured_append_parity >= 0 && self.structured_append_sequence_number >= 0;
    }

    pub fn get_structured_append_parity(&self) -> i32 {
        return self.structured_append_parity;
    }

    pub fn get_structured_append_sequence_number(&self) -> i32 {
        return self.structured_append_sequence_number;
    }

    pub fn get_symbology_modifier(&self) -> i32 {
        return self.symbology_modifier;
    }
}

// DefaultGridSampler.java

/**
 * @author Sean Owen
 */
pub struct DefaultGridSampler {
    //super: GridSampler;
}

impl GridSampler for DefaultGridSampler {
    fn sample_grid(
        &self,
        image: &BitMatrix,
        dimension_x: i32,
        dimension_y: i32,
        p1_to_x: f32,
        p1_to_y: f32,
        p2_to_x: f32,
        p2_to_y: f32,
        p3_to_x: f32,
        p3_to_y: f32,
        p4_to_x: f32,
        p4_to_y: f32,
        p1_from_x: f32,
        p1_from_y: f32,
        p2_from_x: f32,
        p2_from_y: f32,
        p3_from_x: f32,
        p3_from_y: f32,
        p4_from_x: f32,
        p4_from_y: f32,
    ) -> Result<BitMatrix, NotFoundException> {
        let transform: PerspectiveTransform = PerspectiveTransform::quadrilateral_to_quadrilateral(
            p1_to_x, p1_to_y, p2_to_x, p2_to_y, p3_to_x, p3_to_y, p4_to_x, p4_to_y, p1_from_x,
            p1_from_y, p2_from_x, p2_from_y, p3_from_x, p3_from_y, p4_from_x, p4_from_y,
        );
        return Ok(self.sample_grid(image, dimension_x, dimension_y, transform));
    }

    fn sample_grid(
        &self,
        image: &BitMatrix,
        dimension_x: i32,
        dimension_y: i32,
        transform: &PerspectiveTransform,
    ) -> Result<BitMatrix, NotFoundException> {
        if dimension_x <= 0 || dimension_y <= 0 {
            return Err(NotFoundException::get_not_found_instance());
        }
        let bits: BitMatrix = BitMatrix::new(dimension_x, dimension_y, None, None);
        let mut points: [f32; 2.0 * dimension_x] = [0.0; 2.0 * dimension_x];
        {
            let mut y: i32 = 0;
            while y < dimension_y {
                {
                    let max: i32 = points.len();
                    let i_value: f32 = y + 0.5f32;
                    {
                        let mut x: i32 = 0;
                        while x < max {
                            {
                                points[x] = (x / 2.0) as f32 + 0.5f32;
                                points[x + 1] = i_value;
                            }
                            x += 2;
                        }
                    }

                    transform.transform_points(&points);
                    // Quick check to see if points transformed to something inside the image;
                    // sufficient to check the endpoints
                    check_and_nudge_points(image, &points);
                    let tryResult1 = 0;
                    //'try1: loop {
                    //{
                    {
                        let mut x: i32 = 0;
                        while x < max {
                            {
                                if image.get(points[x] as i32, points[x + 1] as i32) {
                                    // Black(-ish) pixel
                                    bits.set(x / 2, y);
                                }
                            }
                            x += 2;
                        }
                    }

                    //}
                    //break 'try1
                    //}
                    //match tryResult1 {
                    //     catch ( aioobe: &ArrayIndexOutOfBoundsException) {
                    //        return Err( NotFoundException::get_not_found_instance());
                    //    }  0 => break
                    //}
                }
                y += 1;
            }
        }

        return Ok(bits);
    }
}

// DetectorResult.java
/**
 * <p>Encapsulates the result of detecting a barcode in an image. This includes the raw
 * matrix of black/white pixels corresponding to the barcode, and possibly points of interest
 * in the image, like the location of finder patterns or corners of the barcode in the image.</p>
 *
 * @author Sean Owen
 */

/* pub struct DetectorResult {
    bits: BitMatrix,

    points: Vec<ResultPoint>,
}

impl DetectorResult {
    pub fn new(bits: &BitMatrix, points: &Vec<ResultPoint>) -> Self {
        Self {
            bits: bits,
            points: points,
        }
    }

    pub fn get_bits(&self) -> BitMatrix {
        return self.bits;
    }

    pub fn get_points(&self) -> Vec<ResultPoint> {
        return self.points;
    }
}
*/

pub trait DetectorResult {
    //pub fn new(bits: &BitMatrix, points: &Vec<ResultPoint>) -> Self;
    pub fn get_bits(&self) -> BitMatrix;
    pub fn get_points(&self) -> Vec<ResultPoint>;
}

// ECIEncoderSet.java
/**
 * Set of CharsetEncoders for a given input string
 *
 * Invariants:
 * - The list contains only encoders from CharacterSetECI (list is shorter then the list of encoders available on
 *   the platform for which ECI values are defined).
 * - The list contains encoders at least one encoder for every character in the input.
 * - The first encoder in the list is always the ISO-8859-1 encoder even of no character in the input can be encoded
 *       by it.
 * - If the input contains a character that is not in ISO-8859-1 then the last two entries in the list will be the
 *   UTF-8 encoder and the UTF-16BE encoder.
 *
 * @author Alex Geller
 */

// List of encoders that potentially encode characters not in ISO-8859-1 in one byte.
//const ENCODERS: List<CharsetEncoder> = ArrayList<>::new();
pub struct ECIEncoderSet {
    encoders: Vec<CharsetEncoder>,

    priority_encoder_index: i32,
}

impl ECIEncoderSet {
    /*static {
         let names: vec![Vec<String>; 20] = vec!["IBM437", "ISO-8859-2", "ISO-8859-3", "ISO-8859-4", "ISO-8859-5", "ISO-8859-6", "ISO-8859-7", "ISO-8859-8", "ISO-8859-9", "ISO-8859-10", "ISO-8859-11", "ISO-8859-13", "ISO-8859-14", "ISO-8859-15", "ISO-8859-16", "windows-1250", "windows-1251", "windows-1252", "windows-1256", "Shift_JIS", ]
        ;
        for  let name: String in names {
            if CharacterSetECI::get_character_set_e_c_i_by_name(&name) != null {
                let tryResult1 = 0;
                'try1: loop {
                {
                    ENCODERS::add(&Charset::for_name(&name)::new_encoder());
                }
                break 'try1
                }
                match tryResult1 {
                     catch ( e: &UnsupportedCharsetException) {
                    }  0 => break
                }

            }
        }
    }*/

    /**
     * Constructs an encoder set
     *
     * @param stringToEncode the string that needs to be encoded
     * @param priorityCharset The preferred {@link Charset} or null.
     * @param fnc1 fnc1 denotes the character in the input that represents the FNC1 character or -1 for a non-GS1 bar
     * code. When specified, it is considered an error to pass it as argument to the methods canEncode() or encode().
     */
    pub fn new(string_to_encode: &str, priority_charset: &Charset, fnc1: i32) -> ECIEncoderSet {
        let needed_encoders: Vec<CharsetEncoder> = Vec::new();
        //we always need the ISO-8859-1 encoder. It is the default encoding
        needed_encoders.add(&StandardCharsets::ISO_8859_1::new_encoder());
        let need_unicode_encoder: bool =
            priority_charset != null && priority_charset.name().starts_with("UTF");
        //Walk over the input string and see if all characters can be encoded with the list of encoders
        {
            let mut i: i32 = 0;
            while i < string_to_encode.length() {
                {
                    let can_encode: bool = false;
                    for encoder in needed_encoders {
                        let c: char = string_to_encode.char_at(i);
                        if c == fnc1 || encoder.can_encode(c) {
                            can_encode = true;
                            break;
                        }
                    }
                    if !can_encode {
                        //for the character at position i we don't yet have an encoder in the list
                        for encoder in ENCODERS {
                            if encoder.can_encode(&string_to_encode.char_at(i)) {
                                //Good, we found an encoder that can encode the character. We add him to the list and continue scanning
                                //the input
                                needed_encoders.add(&encoder);
                                can_encode = true;
                                break;
                            }
                        }
                    }
                    if !can_encode {
                        //The character is not encodeable by any of the single byte encoders so we remember that we will need a
                        //Unicode encoder.
                        need_unicode_encoder = true;
                    }
                }
                i += 1;
            }
        }

        if needed_encoders.size() == 1 && !need_unicode_encoder {
            //the entire input can be encoded by the ISO-8859-1 encoder
            encoders = vec![needed_encoders.get(0)];
        } else {
            // we need more than one single byte encoder or we need a Unicode encoder.
            // In this case we append a UTF-8 and UTF-16 encoder to the list
            encoders = [None; needed_encoders.size() + 2];
            let mut index: i32 = 0;
            for encoder in needed_encoders {
                encoders[index += 1] = encoder;
            }
            encoders[index] = StandardCharsets::UTF_8::new_encoder();
            encoders[index + 1] = StandardCharsets::UTF_16BE::new_encoder();
        }
        //Compute priorityEncoderIndex by looking up priorityCharset in encoders
        let priority_encoder_index_value: i32 = -1;
        if priority_charset != null {
            {
                let mut i: i32 = 0;
                while i < encoders.len() {
                    {
                        if encoders[i] != null
                            && priority_charset
                                .name()
                                .equals(&encoders[i].charset().name())
                        {
                            priority_encoder_index_value = i;
                            break;
                        }
                    }
                    i += 1;
                }
            }
        }
        priority_encoder_index = priority_encoder_index_value;
        //invariants
        assert!(encoders[0].charset().equals(StandardCharsets::ISO_8859_1));
    }

    pub fn length(&self) -> i32 {
        return self.encoders.len();
    }

    pub fn get_charset_name(&self, index: i32) -> String {
        assert!(index < self.length());
        return self.encoders[index].charset().name();
    }

    pub fn get_charset(&self, index: i32) -> Charset {
        assert!(index < self.length());
        return self.encoders[index].charset();
    }

    pub fn get_e_c_i_value(&self, encoder_index: i32) -> i32 {
        return CharacterSetECI::get_value(CharacterSetECI::get_character_set_e_c_i(
            &self.encoders[encoder_index].charset(),
        ));
    }

    /*
     *  returns -1 if no priority charset was defined
     */
    pub fn get_priority_encoder_index(&self) -> i32 {
        return self.priority_encoder_index;
    }

    pub fn can_encode(&self, c: char, encoder_index: i32) -> bool {
        assert!(encoder_index < self.length());
        let encoder: CharsetEncoder = self.encoders[encoder_index];
        return encoder.can_encode(format!("{}", c));
    }

    pub fn encode(&self, c: char, encoder_index: i32) -> Vec<i8> {
        assert!(encoder_index < self.length());
        let encoder: CharsetEncoder = self.encoders[encoder_index];
        assert!(encoder.can_encode(format!("{}", c)));
        return (format!("{}", c)).get_bytes(&encoder.charset());
    }

    pub fn encode(&self, s: &String, encoder_index: i32) -> Vec<i8> {
        assert!(encoder_index < self.length());
        let encoder: CharsetEncoder = self.encoders[encoder_index];
        return s.get_bytes(&encoder.charset());
    }
}

// ECIStringBuilder.java
/**
 * Class that converts a sequence of ECIs and bytes into a string
 *
 * @author Alex Geller
 */
pub struct ECIStringBuilder {
    current_bytes: StringBuilder,

    result: StringBuilder,

    current_charset: Charset,
}

impl ECIStringBuilder {
    pub fn new() -> Self {
        let mut neweci_sb;
        neweci_sb.current_bytes = StringBuilder::new(initial_capacity.unwrape_or(0));

        neweci_sb
    }

    /**
     * Appends {@code value} as a byte value
     *
     * @param value character whose lowest byte is to be appended
     */
    pub fn append(&self, value: char) {
        self.current_bytes.append((value & 0xff) as char);
    }

    /**
     * Appends {@code value} as a byte value
     *
     * @param value byte to append
     */
    pub fn append(&self, value: i8) {
        self.current_bytes.append((value & 0xff) as char);
    }

    /**
     * Appends the characters in {@code value} as bytes values
     *
     * @param value string to append
     */
    pub fn append(&self, value: &String) {
        self.current_bytes.append(&value);
    }

    /**
     * Append the string repesentation of {@code value} (short for {@code append(String.valueOf(value))})
     *
     * @param value int to append as a string
     */
    pub fn append(&self, value: i32) {
        self.append(&String::value_of(value));
    }

    /**
     * Appends ECI value to output.
     *
     * @param value ECI value to append, as an int
     * @throws FormatException on invalid ECI value
     */
    pub fn append_e_c_i(&self, value: i32) -> Result<(), FormatException> {
        self.encode_current_bytes_if_any();
        let character_set_e_c_i: CharacterSetECI =
            CharacterSetECI::get_character_set_e_c_i_by_value(value);
        if character_set_e_c_i == null {
            return Err(FormatException::get_format_instance());
        }
        self.current_charset = character_set_e_c_i.get_charset();
        Ok(())
    }

    fn encode_current_bytes_if_any(&self) {
        if self.current_charset.equals(StandardCharsets::ISO_8859_1) {
            if self.current_bytes.length() > 0 {
                if self.result == null {
                    self.result = self.current_bytes;
                    self.current_bytes = StringBuilder::new();
                } else {
                    self.result.append(&self.current_bytes);
                    self.current_bytes = StringBuilder::new();
                }
            }
        } else if self.current_bytes.length() > 0 {
            let bytes: Vec<i8> = self
                .current_bytes
                .to_string()
                .get_bytes(StandardCharsets::ISO_8859_1);
            self.current_bytes = StringBuilder::new();
            if self.result == null {
                //self.result = StringBuilder::new(String::new(&bytes, &self.current_charset));
                self.result = StringBuilder::new(String::from(&bytes));
            } else {
                //self.result.append(String::new(&bytes, &self.current_charset));
                self.result.append(String::from(&bytes));
            }
        }
    }

    /**
     * Appends the characters from {@code value} (unlike all other append methods of this class who append bytes)
     *
     * @param value characters to append
     */
    pub fn append_characters(&self, value: &StringBuilder) {
        self.encode_current_bytes_if_any();
        self.result.append(&value);
    }

    /**
     * Short for {@code toString().length()} (if possible, use {@link #isEmpty()} instead)
     *
     * @return length of string representation in characters
     */
    pub fn length(&self) -> i32 {
        return self.to_string().length();
    }

    /**
     * @return true iff nothing has been appended
     */
    pub fn is_empty(&self) -> bool {
        return self.current_bytes.length() == 0
            && (self.result == null || self.result.length() == 0);
    }

    pub fn to_string(&self) -> String {
        self.encode_current_bytes_if_any();
        return if self.result == null {
            "".to_owned()
        } else {
            self.result.to_string()
        };
    }
}

// HybridBinarizer.java
/**
 * This class implements a local thresholding algorithm, which while slower than the
 * GlobalHistogramBinarizer, is fairly efficient for what it does. It is designed for
 * high frequency images of barcodes with black data on white backgrounds. For this application,
 * it does a much better job than a global blackpoint with severe shadows and gradients.
 * However it tends to produce artifacts on lower frequency images and is therefore not
 * a good general purpose binarizer for uses outside ZXing.
 *
 * This class extends GlobalHistogramBinarizer, using the older histogram approach for 1D readers,
 * and the newer local approach for 2D readers. 1D decoding using a per-row histogram is already
 * inherently local, and only fails for horizontal gradients. We can revisit that problem later,
 * but for now it was not a win to use local blocks for 1D.
 *
 * This Binarizer is the default for the unit tests and the recommended class for library users.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */

// This class uses 5x5 blocks to compute local luminance, where each block is 8x8 pixels.
// So this is the smallest dimension in each axis we can accept.
const BLOCK_SIZE_POWER: i32 = 3;

// ...0100...00
const BLOCK_SIZE: i32 = 1 << BLOCK_SIZE_POWER;

// ...0011...11
const BLOCK_SIZE_MASK: i32 = BLOCK_SIZE - 1;

const MINIMUM_DIMENSION: i32 = BLOCK_SIZE * 5;

const MIN_DYNAMIC_RANGE: i32 = 24;
pub struct HybridBinarizer {
    //super: GlobalHistogramBinarizer;
    matrix: BitMatrix,
}

impl GlobalHistogramBinarizer for HybridBinarizer {
    /**
     * Calculates the final BitMatrix once for all requests. This could be called once from the
     * constructor instead, but there are some advantages to doing it lazily, such as making
     * profiling easier, and not doing heavy lifting when callers don't expect it.
     */
    fn get_black_matrix(&self) -> Result<BitMatrix, NotFoundException> {
        if self.matrix != null {
            return Ok(self.matrix);
        }
        let source: LuminanceSource = get_luminance_source();
        let width: i32 = source.get_width();
        let height: i32 = source.get_height();
        if width >= MINIMUM_DIMENSION && height >= MINIMUM_DIMENSION {
            let luminances: Vec<i8> = source.get_matrix();
            let sub_width: i32 = width >> BLOCK_SIZE_POWER;
            if (width & BLOCK_SIZE_MASK) != 0 {
                sub_width += 1;
            }
            let sub_height: i32 = height >> BLOCK_SIZE_POWER;
            if (height & BLOCK_SIZE_MASK) != 0 {
                sub_height += 1;
            }
            let black_points: Vec<Vec<i32>> =
                ::calculate_black_points(&luminances, sub_width, sub_height, width, height);
            let new_matrix: BitMatrix = BitMatrix::new(width, height, None, None);
            ::calculate_threshold_for_block(
                &luminances,
                sub_width,
                sub_height,
                width,
                height,
                &black_points,
                new_matrix,
            );
            self.matrix = new_matrix;
        } else {
            // If the image is too small, fall back to the global histogram approach.
            self.matrix = super.get_black_matrix();
        }
        return Ok(self.matrix);
    }

    fn create_binarizer(&self, source: &LuminanceSource) -> Binarizer {
        return HybridBinarizer::new(source);
    }
}

impl HybridBinarizer {
    pub fn new(source: &LuminanceSource) -> Self {
        //super(source);
        HybridBinarizer::new(source)
    }

    /**
     * For each block in the image, calculate the average black point using a 5x5 grid
     * of the blocks around it. Also handles the corner cases (fractional blocks are computed based
     * on the last pixels in the row/column which are also used in the previous block).
     */
    fn calculate_threshold_for_block(
        luminances: &Vec<i8>,
        sub_width: i32,
        sub_height: i32,
        width: i32,
        height: i32,
        black_points: &Vec<Vec<i32>>,
        matrix: &BitMatrix,
    ) {
        let max_y_offset: i32 = height - BLOCK_SIZE;
        let max_x_offset: i32 = width - BLOCK_SIZE;
        {
            let mut y: i32 = 0;
            while y < sub_height {
                {
                    let mut yoffset: i32 = y << BLOCK_SIZE_POWER;
                    if yoffset > max_y_offset {
                        yoffset = max_y_offset;
                    }
                    let top: i32 = ::cap(y, sub_height - 3);
                    {
                        let mut x: i32 = 0;
                        while x < sub_width {
                            {
                                let mut xoffset: i32 = x << BLOCK_SIZE_POWER;
                                if xoffset > max_x_offset {
                                    xoffset = max_x_offset;
                                }
                                let left: i32 = ::cap(x, sub_width - 3);
                                let mut sum: i32 = 0;
                                {
                                    let mut z: i32 = -2;
                                    while z <= 2 {
                                        {
                                            let black_row: Vec<i32> = black_points[top + z];
                                            sum += black_row[left - 2]
                                                + black_row[left - 1]
                                                + black_row[left]
                                                + black_row[left + 1]
                                                + black_row[left + 2];
                                        }
                                        z += 1;
                                    }
                                }

                                let average: i32 = sum / 25;
                                ::threshold_block(
                                    &luminances,
                                    xoffset,
                                    yoffset,
                                    average,
                                    width,
                                    matrix,
                                );
                            }
                            x += 1;
                        }
                    }
                }
                y += 1;
            }
        }
    }

    fn cap(value: i32, max: i32) -> i32 {
        return if value < 2 { 2 } else { Math::min(value, max) };
    }

    /**
     * Applies a single threshold to a block of pixels.
     */
    fn threshold_block(
        luminances: &Vec<i8>,
        xoffset: i32,
        yoffset: i32,
        threshold: i32,
        stride: i32,
        matrix: &BitMatrix,
    ) {
        {
            let mut y: i32 = 0;
            let mut offset: i32 = yoffset * stride + xoffset;
            while y < BLOCK_SIZE {
                {
                    {
                        let mut x: i32 = 0;
                        while x < BLOCK_SIZE {
                            {
                                // Comparison needs to be <= so that black == 0 pixels are black even if the threshold is 0.
                                if (luminances[offset + x] & 0xFF) <= threshold {
                                    matrix.set(xoffset + x, yoffset + y);
                                }
                            }
                            x += 1;
                        }
                    }
                }
                y += 1;
                offset += stride;
            }
        }
    }

    /**
     * Calculates a single black point for each block of pixels and saves it away.
     * See the following thread for a discussion of this algorithm:
     *  http://groups.google.com/group/zxing/browse_thread/thread/d06efa2c35a7ddc0
     */
    fn calculate_black_points(
        luminances: &Vec<i8>,
        sub_width: i32,
        sub_height: i32,
        width: i32,
        height: i32,
    ) -> Vec<Vec<i32>> {
        let max_y_offset: i32 = height - BLOCK_SIZE;
        let max_x_offset: i32 = width - BLOCK_SIZE;
        let black_points: [[i32; sub_width]; sub_height] = [[0; sub_width]; sub_height];
        {
            let mut y: i32 = 0;
            while y < sub_height {
                {
                    let mut yoffset: i32 = y << BLOCK_SIZE_POWER;
                    if yoffset > max_y_offset {
                        yoffset = max_y_offset;
                    }
                    {
                        let mut x: i32 = 0;
                        while x < sub_width {
                            {
                                let mut xoffset: i32 = x << BLOCK_SIZE_POWER;
                                if xoffset > max_x_offset {
                                    xoffset = max_x_offset;
                                }
                                let mut sum: i32 = 0;
                                let mut min: i32 = 0xFF;
                                let mut max: i32 = 0;
                                {
                                    let mut yy: i32 = 0;
                                    let mut offset: i32 = yoffset * width + xoffset;
                                    while yy < BLOCK_SIZE {
                                        {
                                            {
                                                let mut xx: i32 = 0;
                                                while xx < BLOCK_SIZE {
                                                    {
                                                        let pixel: i32 =
                                                            luminances[offset + xx] & 0xFF;
                                                        sum += pixel;
                                                        // still looking for good contrast
                                                        if pixel < min {
                                                            min = pixel;
                                                        }
                                                        if pixel > max {
                                                            max = pixel;
                                                        }
                                                    }
                                                    xx += 1;
                                                }
                                            }

                                            // short-circuit min/max tests once dynamic range is met
                                            if max - min > MIN_DYNAMIC_RANGE {
                                                // finish the rest of the rows quickly
                                                {
                                                    yy += 1;
                                                    offset += width;
                                                    while yy < BLOCK_SIZE {
                                                        {
                                                            {
                                                                let mut xx: i32 = 0;
                                                                while xx < BLOCK_SIZE {
                                                                    {
                                                                        sum += luminances
                                                                            [offset + xx]
                                                                            & 0xFF;
                                                                    }
                                                                    xx += 1;
                                                                }
                                                            }
                                                        }
                                                        yy += 1;
                                                        offset += width;
                                                    }
                                                }
                                            }
                                        }
                                        yy += 1;
                                        offset += width;
                                    }
                                }

                                // The default estimate is the average of the values in the block.
                                let mut average: i32 = sum >> (BLOCK_SIZE_POWER * 2);
                                if max - min <= MIN_DYNAMIC_RANGE {
                                    // If variation within the block is low, assume this is a block with only light or only
                                    // dark pixels. In that case we do not want to use the average, as it would divide this
                                    // low contrast area into black and white pixels, essentially creating data out of noise.
                                    //
                                    // The default assumption is that the block is light/background. Since no estimate for
                                    // the level of dark pixels exists locally, use half the min for the block.
                                    average = min / 2;
                                    if y > 0 && x > 0 {
                                        // Correct the "white background" assumption for blocks that have neighbors by comparing
                                        // the pixels in this block to the previously calculated black points. This is based on
                                        // the fact that dark barcode symbology is always surrounded by some amount of light
                                        // background for which reasonable black point estimates were made. The bp estimated at
                                        // the boundaries is used for the interior.
                                        // The (min < bp) is arbitrary but works better than other heuristics that were tried.
                                        let average_neighbor_black_point: i32 = (black_points
                                            [y - 1][x]
                                            + (2 * black_points[y][x - 1])
                                            + black_points[y - 1][x - 1])
                                            / 4;
                                        if min < average_neighbor_black_point {
                                            average = average_neighbor_black_point;
                                        }
                                    }
                                }
                                black_points[y][x] = average;
                            }
                            x += 1;
                        }
                    }
                }
                y += 1;
            }
        }

        return black_points;
    }
}

// MinimalECIInput.java
/**
 * Class that converts a character string into a sequence of ECIs and bytes
 *
 * The implementation uses the Dijkstra algorithm to produce minimal encodings
 *
 * @author Alex Geller
 */

// approximated (latch + 2 codewords)
const COST_PER_ECI: i32 = 3;
pub struct MinimalECIInput {
    bytes: Vec<i32>,

    fnc1: i32,
}

impl ECIInput for MinimalECIInput {
    fn have_n_characters(&self, index: i32, n: i32) -> bool {
        if index + n - 1 >= self.bytes.len() {
            return false;
        }
        {
            let mut i: i32 = 0;
            while i < n {
                {
                    if self.is_e_c_i(index + i) {
                        return false;
                    }
                }
                i += 1;
            }
        }

        return true;
    }

    /**
     * Returns the {@code int} ECI value at the specified index.  An index ranges from zero
     * to {@code length() - 1}.  The first {@code byte} value of the sequence is at
     * index zero, the next at index one, and so on, as for array
     * indexing.
     *
     * @param   index the index of the {@code int} value to be returned
     *
     * @return  the specified {@code int} ECI value.
     *          The ECI specified the encoding of all bytes with a higher index until the
     *          next ECI or until the end of the input if no other ECI follows.
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     * @throws  IllegalArgumentException
     *          if the value at the {@code index} argument is not an ECI (@see #isECI)
     */
    fn get_e_c_i_value(
        &self,
        index: i32,
    ) -> Result<i32, IndexOutOfBoundsException + IllegalArgumentException> {
        if index < 0 || index >= self.length() {
            return Err(IndexOutOfBoundsException::new(format!("{}", index)));
        }
        if !self.is_e_c_i(index) {
            return Err(IllegalArgumentException::new(format!(
                "value at {} is not an ECI but a character",
                index
            )));
        }
        return self.bytes[index] - 256;
    }

    /**
     * Determines if a value is an ECI
     *
     * @param   index the index of the value
     *
     * @return  true if the value at position {@code index} is an ECI
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     */
    fn is_e_c_i(&self, index: i32) -> Result<bool, IndexOutOfBoundsException> {
        if index < 0 || index >= self.length() {
            return Err(IndexOutOfBoundsException::new(format!("{}", index)));
        }
        return Ok(self.bytes[index] > 255 && self.bytes[index] <= 999);
    }

    /**
     * Returns a {@code CharSequence} that is a subsequence of this sequence.
     * The subsequence starts with the {@code char} value at the specified index and
     * ends with the {@code char} value at index {@code end - 1}.  The length
     * (in {@code char}s) of the
     * returned sequence is {@code end - start}, so if {@code start == end}
     * then an empty sequence is returned.
     *
     * @param   start   the start index, inclusive
     * @param   end     the end index, exclusive
     *
     * @return  the specified subsequence
     *
     * @throws  IndexOutOfBoundsException
     *          if {@code start} or {@code end} are negative,
     *          if {@code end} is greater than {@code length()},
     *          or if {@code start} is greater than {@code end}
     * @throws  IllegalArgumentException
     *          if a value in the range {@code start}-{@code end} is an ECI (@see #isECI)
     */
    fn sub_sequence(
        &self,
        start: i32,
        end: i32,
    ) -> Result<CharSequence, IndexOutOfBoundsException + IllegalArgumentException> {
        if start < 0 || start > end || end > self.length() {
            return Err(IndexOutOfBoundsException::new(format!("{}", start)));
        }
        let result: StringBuilder = StringBuilder::new();
        {
            let mut i: i32 = start;
            while i < end {
                {
                    if self.is_e_c_i(i) {
                        return Err(IllegalArgumentException::new(format!(
                            "value at {} is not a character but an ECI",
                            i
                        )));
                    }
                    result.append(&self.char_at(i));
                }
                i += 1;
            }
        }

        return result;
    }

    /**
     * Returns the {@code byte} value at the specified index.  An index ranges from zero
     * to {@code length() - 1}.  The first {@code byte} value of the sequence is at
     * index zero, the next at index one, and so on, as for array
     * indexing.
     *
     * @param   index the index of the {@code byte} value to be returned
     *
     * @return  the specified {@code byte} value as character or the FNC1 character
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     * @throws  IllegalArgumentException
     *          if the value at the {@code index} argument is an ECI (@see #isECI)
     */
    fn char_at(
        &self,
        index: i32,
    ) -> Result<char, IndexOutOfBoundsException + IllegalArgumentException> {
        if index < 0 || index >= self.length() {
            return Err(IndexOutOfBoundsException::new(format!("{}", index)));
        }
        if self.is_e_c_i(index) {
            return Err(IllegalArgumentException::new(format!(
                "value at {} is not a character but an ECI",
                index
            )));
        }
        return if self.is_f_n_c1(index) {
            Ok(self.fnc1 as char)
        } else {
            Ok(self.bytes[index] as char)
        };
    }

    /**
     * Returns the length of this input.  The length is the number
     * of {@code byte}s, FNC1 characters or ECIs in the sequence.
     *
     * @return  the number of {@code char}s in this sequence
     */
    fn length(&self) -> i32 {
        return self.bytes.len();
    }
}

impl MinimalECIInput {
    /**
     * Constructs a minimal input
     *
     * @param stringToEncode the character string to encode
     * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
     *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
     *   charset to encode any character in the input that can be encoded by it if the charset is among the
     *   supported charsets.
     * @param fnc1 denotes the character in the input that represents the FNC1 character or -1 if this is not GS1
     *   input.
     */
    pub fn new(string_to_encode: &String, priority_charset: &Charset, fnc1: i32) -> Self {
        let mut new_mecii: Self;
        new_mecii.fnc1 = fnc1;
        let encoder_set: ECIEncoderSet =
            ECIEncoderSet::new(&string_to_encode, &priority_charset, fnc1);
        if encoder_set.length() == 1 {
            //optimization for the case when all can be encoded without ECI in ISO-8859-1
            bytes = [0; string_to_encode.length()];
            {
                let mut i: i32 = 0;
                while i < bytes.len() {
                    {
                        let c: char = string_to_encode.char_at(i);
                        bytes[i] = if c == fnc1 { 1000 } else { c as i32 };
                    }
                    i += 1;
                }
            }
        } else {
            bytes = ::encode_minimally(&string_to_encode, encoder_set, fnc1);
        }
    }

    pub fn get_f_n_c1_character(&self) -> i32 {
        return self.fnc1;
    }

    /**
     * Determines if a value is the FNC1 character
     *
     * @param   index the index of the value
     *
     * @return  true if the value at position {@code index} is the FNC1 character
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     */
    pub fn is_f_n_c1(&self, index: i32) -> Result<bool, IndexOutOfBoundsException> {
        if index < 0 || index >= self.length() {
            return Err(IndexOutOfBoundsException::new(format!("{}", index)));
        }
        return Ok(self.bytes[index] == 1000);
    }

    pub fn to_string(&self) -> String {
        let result: StringBuilder = StringBuilder::new();
        {
            let mut i: i32 = 0;
            while i < self.length() {
                {
                    if i > 0 {
                        result.append(", ");
                    }
                    if self.is_e_c_i(i) {
                        result.append("ECI(");
                        result.append(&self.get_e_c_i_value(i));
                        result.append(')');
                    } else if self.char_at(i) < 128 {
                        result.append('\'');
                        result.append(&self.char_at(i));
                        result.append('\'');
                    } else {
                        result.append(self.char_at(i) as i32);
                    }
                }
                i += 1;
            }
        }

        return result.to_string();
    }

    fn add_edge(edges: &Vec<Vec<InputEdge>>, to: i32, edge: &InputEdge) {
        if edges[to][edge.encoderIndex] == null
            || edges[to][edge.encoderIndex].cachedTotalSize > edge.cachedTotalSize
        {
            edges[to][edge.encoderIndex] = edge;
        }
    }

    fn add_edges(
        string_to_encode: &String,
        encoder_set: &ECIEncoderSet,
        edges: &Vec<Vec<InputEdge>>,
        from: i32,
        previous: &InputEdge,
        fnc1: i32,
    ) {
        let ch: char = string_to_encode.char_at(from);
        let mut start: i32 = 0;
        let mut end: i32 = encoder_set.length();
        if encoder_set.get_priority_encoder_index() >= 0
            && (ch == fnc1 || encoder_set.can_encode(ch, &encoder_set.get_priority_encoder_index()))
        {
            start = encoder_set.get_priority_encoder_index();
            end = start + 1;
        }
        {
            let mut i: i32 = start;
            while i < end {
                {
                    if ch == fnc1 || encoder_set.can_encode(ch, i) {
                        ::add_edge(
                            edges,
                            from + 1,
                            InputEdge::new(ch, encoder_set, i, previous, fnc1),
                        );
                    }
                }
                i += 1;
            }
        }
    }

    fn encode_minimally(
        string_to_encode: &String,
        encoder_set: &ECIEncoderSet,
        fnc1: i32,
    ) -> Result<Vec<i32>, RuntimeException> {
        let input_length: i32 = string_to_encode.length();
        // Array that represents vertices. There is a vertex for every character and encoding.
        let mut edges: [[Option<InputEdge>; encoder_set.length()]; input_length + 1] =
            [[None; encoder_set.length()]; input_length + 1];
        ::add_edges(&string_to_encode, encoder_set, edges, 0, null, fnc1);
        {
            let mut i: i32 = 1;
            while i <= input_length {
                {
                    {
                        let mut j: i32 = 0;
                        while j < encoder_set.length() {
                            {
                                if edges[i][j] != null && i < input_length {
                                    ::add_edges(
                                        &string_to_encode,
                                        encoder_set,
                                        edges,
                                        i,
                                        edges[i][j],
                                        fnc1,
                                    );
                                }
                            }
                            j += 1;
                        }
                    }

                    //optimize memory by removing edges that have been passed.
                    {
                        let mut j: i32 = 0;
                        while j < encoder_set.length() {
                            {
                                edges[i - 1][j] = null;
                            }
                            j += 1;
                        }
                    }
                }
                i += 1;
            }
        }

        let minimal_j: i32 = -1;
        let minimal_size: i32 = Integer::MAX_VALUE;
        {
            let mut j: i32 = 0;
            while j < encoder_set.length() {
                {
                    if edges[input_length][j] != null {
                        let edge: InputEdge = edges[input_length][j];
                        if edge.cachedTotalSize < minimal_size {
                            minimal_size = edge.cachedTotalSize;
                            minimal_j = j;
                        }
                    }
                }
                j += 1;
            }
        }

        if minimal_j < 0 {
            return Err(RuntimeException::new(format!(
                "Internal error: failed to encode \"{}\"",
                string_to_encode
            )));
        }
        let ints_a_l: List<Integer> = Vec::new();
        let mut current: InputEdge = edges[input_length][minimal_j];
        while current != null {
            if current.is_f_n_c1() {
                ints_a_l.add(0, 1000);
            } else {
                let bytes: Vec<i8> = encoder_set.encode(current.c, current.encoderIndex);
                {
                    let mut i: i32 = bytes.len() - 1;
                    while i >= 0 {
                        {
                            ints_a_l.add(0, (bytes[i] & 0xFF));
                        }
                        i -= 1;
                    }
                }
            }
            let previous_encoder_index: i32 = if current.previous == null {
                0
            } else {
                current.previous.encoderIndex
            };
            if previous_encoder_index != current.encoderIndex {
                ints_a_l.add(0, 256 + encoder_set.get_e_c_i_value(current.encoderIndex));
            }
            current = current.previous;
        }
        let mut ints: [i32; ints_a_l.size()] = [0; ints_a_l.size()];
        {
            let mut i: i32 = 0;
            while i < ints.len() {
                {
                    ints[i] = ints_a_l.get(i);
                }
                i += 1;
            }
        }

        return ints;
    }
}

struct InputEdge {
    c: char,

    //the encoding of this edge
    encoder_index: i32,

    previous: InputEdge,

    cached_total_size: i32,
}

impl InputEdge {
    fn new(
        c: char,
        encoder_set: &ECIEncoderSet,
        encoder_index: i32,
        previous: &InputEdge,
        fnc1: i32,
    ) -> Self {
        let mut new_ie: Self;
        new_ie.c = if c == fnc1 { 1000 } else { c };
        new_ie.encoderIndex = encoder_index;
        new_ie.previous = previous;
        let mut size: i32 = if new_ie.c == 1000 {
            1
        } else {
            encoder_set.encode(c, encoder_index).len()
        };
        let previous_encoder_index: i32 = if previous == null {
            0
        } else {
            previous.encoderIndex
        };
        if previous_encoder_index != encoder_index {
            size += COST_PER_ECI;
        }
        if previous != null {
            size += previous.cachedTotalSize;
        }
        new_ie.cachedTotalSize = size;

        new_ie
    }

    fn is_f_n_c1(&self) -> bool {
        return self.c == 1000;
    }
}

// PerspectiveTransform.java
/**
 * <p>This class implements a perspective transform in two dimensions. Given four source and four
 * destination points, it will compute the transformation implied between them. The code is based
 * directly upon section 3.4.2 of George Wolberg's "Digital Image Warping"; see pages 54-56.</p>
 *
 * @author Sean Owen
 */
pub struct PerspectiveTransform {
    a11: f32,

    a12: f32,

    a13: f32,

    a21: f32,

    a22: f32,

    a23: f32,

    a31: f32,

    a32: f32,

    a33: f32,
}

impl PerspectiveTransform {
    fn new(
        a11: f32,
        a21: f32,
        a31: f32,
        a12: f32,
        a22: f32,
        a32: f32,
        a13: f32,
        a23: f32,
        a33: f32,
    ) -> Self {
        Self {
            a11: a11,
            a12: a12,
            a13: a13,
            a21: a21,
            a22: a22,
            a23: a23,
            a31: a31,
            a32: a32,
            a33: a33,
        }
    }

    pub fn quadrilateral_to_quadrilateral(
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        x0p: f32,
        y0p: f32,
        x1p: f32,
        y1p: f32,
        x2p: f32,
        y2p: f32,
        x3p: f32,
        y3p: f32,
    ) -> PerspectiveTransform {
        let q_to_s: PerspectiveTransform =
            ::quadrilateral_to_square(x0, y0, x1, y1, x2, y2, x3, y3);
        let s_to_q: PerspectiveTransform =
            ::square_to_quadrilateral(x0p, y0p, x1p, y1p, x2p, y2p, x3p, y3p);
        return s_to_q.times(&q_to_s);
    }

    pub fn transform_points(&self, points: &Vec<f32>) {
        let a11: f32 = self.a11;
        let a12: f32 = self.a12;
        let a13: f32 = self.a13;
        let a21: f32 = self.a21;
        let a22: f32 = self.a22;
        let a23: f32 = self.a23;
        let a31: f32 = self.a31;
        let a32: f32 = self.a32;
        let a33: f32 = self.a33;
        // points.length must be even
        let max_i: i32 = points.len() - 1;
        {
            let mut i: i32 = 0;
            while i < max_i {
                {
                    let x: f32 = points[i];
                    let y: f32 = points[i + 1];
                    let denominator: f32 = a13 * x + a23 * y + a33;
                    points[i] = (a11 * x + a21 * y + a31) / denominator;
                    points[i + 1] = (a12 * x + a22 * y + a32) / denominator;
                }
                i += 2;
            }
        }
    }

    pub fn transform_points(&self, x_values: &Vec<f32>, y_values: &Vec<f32>) {
        let n: i32 = x_values.len();
        {
            let mut i: i32 = 0;
            while i < n {
                {
                    let x: f32 = x_values[i];
                    let y: f32 = y_values[i];
                    let denominator: f32 = self.a13 * x + self.a23 * y + self.a33;
                    x_values[i] = (self.a11 * x + self.a21 * y + self.a31) / denominator;
                    y_values[i] = (self.a12 * x + self.a22 * y + self.a32) / denominator;
                }
                i += 1;
            }
        }
    }

    pub fn square_to_quadrilateral(
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> PerspectiveTransform {
        let dx3: f32 = x0 - x1 + x2 - x3;
        let dy3: f32 = y0 - y1 + y2 - y3;
        if dx3 == 0.0f32 && dy3 == 0.0f32 {
            // Affine
            return PerspectiveTransform::new(
                x1 - x0,
                x2 - x1,
                x0,
                y1 - y0,
                y2 - y1,
                y0,
                0.0f32,
                0.0f32,
                1.0f32,
            );
        } else {
            let dx1: f32 = x1 - x2;
            let dx2: f32 = x3 - x2;
            let dy1: f32 = y1 - y2;
            let dy2: f32 = y3 - y2;
            let denominator: f32 = dx1 * dy2 - dx2 * dy1;
            let a13: f32 = (dx3 * dy2 - dx2 * dy3) / denominator;
            let a23: f32 = (dx1 * dy3 - dx3 * dy1) / denominator;
            return PerspectiveTransform::new(
                x1 - x0 + a13 * x1,
                x3 - x0 + a23 * x3,
                x0,
                y1 - y0 + a13 * y1,
                y3 - y0 + a23 * y3,
                y0,
                a13,
                a23,
                1.0f32,
            );
        }
    }

    pub fn quadrilateral_to_square(
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
    ) -> PerspectiveTransform {
        // Here, the adjoint serves as the inverse:
        return ::square_to_quadrilateral(x0, y0, x1, y1, x2, y2, x3, y3).build_adjoint();
    }

    fn build_adjoint(&self) -> PerspectiveTransform {
        // Adjoint is the transpose of the cofactor matrix:
        return PerspectiveTransform::new(
            self.a22 * self.a33 - self.a23 * self.a32,
            self.a23 * self.a31 - self.a21 * self.a33,
            self.a21 * self.a32 - self.a22 * self.a31,
            self.a13 * self.a32 - self.a12 * self.a33,
            self.a11 * self.a33 - self.a13 * self.a31,
            self.a12 * self.a31 - self.a11 * self.a32,
            self.a12 * self.a23 - self.a13 * self.a22,
            self.a13 * self.a21 - self.a11 * self.a23,
            self.a11 * self.a22 - self.a12 * self.a21,
        );
    }

    fn times(&self, other: &PerspectiveTransform) -> PerspectiveTransform {
        return PerspectiveTransform::new(
            self.a11 * other.a11 + self.a21 * other.a12 + self.a31 * other.a13,
            self.a11 * other.a21 + self.a21 * other.a22 + self.a31 * other.a23,
            self.a11 * other.a31 + self.a21 * other.a32 + self.a31 * other.a33,
            self.a12 * other.a11 + self.a22 * other.a12 + self.a32 * other.a13,
            self.a12 * other.a21 + self.a22 * other.a22 + self.a32 * other.a23,
            self.a12 * other.a31 + self.a22 * other.a32 + self.a32 * other.a33,
            self.a13 * other.a11 + self.a23 * other.a12 + self.a33 * other.a13,
            self.a13 * other.a21 + self.a23 * other.a22 + self.a33 * other.a23,
            self.a13 * other.a31 + self.a23 * other.a32 + self.a33 * other.a33,
        );
    }
}

// StringUtils.java
/**
 * Common string-related functions.
 *
 * @author Sean Owen
 * @author Alex Dupre
 */

const PLATFORM_DEFAULT_ENCODING: Charset = Charset::default_charset();

const SHIFT_JIS_CHARSET: Charset = Charset::for_name("SJIS");

const GB2312_CHARSET: Charset = Charset::for_name("GB2312");

const EUC_JP: Charset = Charset::for_name("EUC_JP");

const ASSUME_SHIFT_JIS: bool = SHIFT_JIS_CHARSET::equals(&PLATFORM_DEFAULT_ENCODING)
    || EUC_JP::equals(&PLATFORM_DEFAULT_ENCODING);

// Retained for ABI compatibility with earlier versions
const SHIFT_JIS: &'static str = "SJIS";

const GB2312: &'static str = "GB2312";
pub struct StringUtils {}

impl StringUtils {
    fn new() -> StringUtils {}

    /**
     * @param bytes bytes encoding a string, whose encoding should be guessed
     * @param hints decode hints if applicable
     * @return name of guessed encoding; at the moment will only guess one of:
     *  "SJIS", "UTF8", "ISO8859_1", or the platform default encoding if none
     *  of these can possibly be correct
     */
    pub fn guess_encoding(bytes: &Vec<i8>, hints: &HashMap<DecodeHintType, _>) -> &str {
        let c: Charset = ::guess_charset(&bytes, &hints);
        if c == SHIFT_JIS_CHARSET {
            return "SJIS";
        } else if c == StandardCharsets::UTF_8 {
            return "UTF8";
        } else if c == StandardCharsets::ISO_8859_1 {
            return "ISO8859_1";
        }
        return c.name();
    }

    /**
     * @param bytes bytes encoding a string, whose encoding should be guessed
     * @param hints decode hints if applicable
     * @return Charset of guessed encoding; at the moment will only guess one of:
     *  {@link #SHIFT_JIS_CHARSET}, {@link StandardCharsets#UTF_8},
     *  {@link StandardCharsets#ISO_8859_1}, {@link StandardCharsets#UTF_16},
     *  or the platform default encoding if
     *  none of these can possibly be correct
     */
    pub fn guess_charset(bytes: &Vec<i8>, hints: &HashMap<DecodeHintType, _>) -> Charset {
        if hints != null && hints.contains_key(DecodeHintType::CHARACTER_SET) {
            return Charset::for_name(&hints.get(DecodeHintType::CHARACTER_SET).to_string());
        }
        // First try UTF-16, assuming anything with its BOM is UTF-16
        if bytes.len() > 2
            && ((bytes[0] == 0xFE as i8 && bytes[1] == 0xFF as i8)
                || (bytes[0] == 0xFF as i8 && bytes[1] == 0xFE as i8))
        {
            return StandardCharsets::UTF_16;
        }
        // For now, merely tries to distinguish ISO-8859-1, UTF-8 and Shift_JIS,
        // which should be by far the most common encodings.
        let length: i32 = bytes.len();
        let can_be_i_s_o88591: bool = true;
        let can_be_shift_j_i_s: bool = true;
        let can_be_u_t_f8: bool = true;
        let utf8_bytes_left: i32 = 0;
        let utf2_bytes_chars: i32 = 0;
        let utf3_bytes_chars: i32 = 0;
        let utf4_bytes_chars: i32 = 0;
        let sjis_bytes_left: i32 = 0;
        let sjis_katakana_chars: i32 = 0;
        let sjis_cur_katakana_word_length: i32 = 0;
        let sjis_cur_double_bytes_word_length: i32 = 0;
        let sjis_max_katakana_word_length: i32 = 0;
        let sjis_max_double_bytes_word_length: i32 = 0;
        let iso_high_other: i32 = 0;
        let utf8bom: bool = bytes.len() > 3
            && bytes[0] == 0xEF as i8
            && bytes[1] == 0xBB as i8
            && bytes[2] == 0xBF as i8;
        {
            let mut i: i32 = 0;
            while i < length && (can_be_i_s_o88591 || can_be_shift_j_i_s || can_be_u_t_f8) {
                {
                    let value: i32 = bytes[i] & 0xFF;
                    // UTF-8 stuff
                    if can_be_u_t_f8 {
                        if utf8_bytes_left > 0 {
                            if (value & 0x80) == 0 {
                                can_be_u_t_f8 = false;
                            } else {
                                utf8_bytes_left -= 1;
                            }
                        } else if (value & 0x80) != 0 {
                            if (value & 0x40) == 0 {
                                can_be_u_t_f8 = false;
                            } else {
                                utf8_bytes_left += 1;
                                if (value & 0x20) == 0 {
                                    utf2_bytes_chars += 1;
                                } else {
                                    utf8_bytes_left += 1;
                                    if (value & 0x10) == 0 {
                                        utf3_bytes_chars += 1;
                                    } else {
                                        utf8_bytes_left += 1;
                                        if (value & 0x08) == 0 {
                                            utf4_bytes_chars += 1;
                                        } else {
                                            can_be_u_t_f8 = false;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // ISO-8859-1 stuff
                    if can_be_i_s_o88591 {
                        if value > 0x7F && value < 0xA0 {
                            can_be_i_s_o88591 = false;
                        } else if value > 0x9F && (value < 0xC0 || value == 0xD7 || value == 0xF7) {
                            iso_high_other += 1;
                        }
                    }
                    // Shift_JIS stuff
                    if can_be_shift_j_i_s {
                        if sjis_bytes_left > 0 {
                            if value < 0x40 || value == 0x7F || value > 0xFC {
                                can_be_shift_j_i_s = false;
                            } else {
                                sjis_bytes_left -= 1;
                            }
                        } else if value == 0x80 || value == 0xA0 || value > 0xEF {
                            can_be_shift_j_i_s = false;
                        } else if value > 0xA0 && value < 0xE0 {
                            sjis_katakana_chars += 1;
                            sjis_cur_double_bytes_word_length = 0;
                            sjis_cur_katakana_word_length += 1;
                            if sjis_cur_katakana_word_length > sjis_max_katakana_word_length {
                                sjis_max_katakana_word_length = sjis_cur_katakana_word_length;
                            }
                        } else if value > 0x7F {
                            sjis_bytes_left += 1;
                            //sjisDoubleBytesChars++;
                            sjis_cur_katakana_word_length = 0;
                            sjis_cur_double_bytes_word_length += 1;
                            if sjis_cur_double_bytes_word_length > sjis_max_double_bytes_word_length
                            {
                                sjis_max_double_bytes_word_length =
                                    sjis_cur_double_bytes_word_length;
                            }
                        } else {
                            //sjisLowChars++;
                            sjis_cur_katakana_word_length = 0;
                            sjis_cur_double_bytes_word_length = 0;
                        }
                    }
                }
                i += 1;
            }
        }

        if can_be_u_t_f8 && utf8_bytes_left > 0 {
            can_be_u_t_f8 = false;
        }
        if can_be_shift_j_i_s && sjis_bytes_left > 0 {
            can_be_shift_j_i_s = false;
        }
        // Easy -- if there is BOM or at least 1 valid not-single byte character (and no evidence it can't be UTF-8), done
        if can_be_u_t_f8 && (utf8bom || utf2_bytes_chars + utf3_bytes_chars + utf4_bytes_chars > 0)
        {
            return StandardCharsets::UTF_8;
        }
        // Easy -- if assuming Shift_JIS or >= 3 valid consecutive not-ascii characters (and no evidence it can't be), done
        if can_be_shift_j_i_s
            && (ASSUME_SHIFT_JIS
                || sjis_max_katakana_word_length >= 3
                || sjis_max_double_bytes_word_length >= 3)
        {
            return SHIFT_JIS_CHARSET;
        }
        // - then we conclude Shift_JIS, else ISO-8859-1
        if can_be_i_s_o88591 && can_be_shift_j_i_s {
            return if (sjis_max_katakana_word_length == 2 && sjis_katakana_chars == 2)
                || iso_high_other * 10 >= length
            {
                SHIFT_JIS_CHARSET
            } else {
                StandardCharsets::ISO_8859_1
            };
        }
        // Otherwise, try in order ISO-8859-1, Shift JIS, UTF-8 and fall back to default platform encoding
        if can_be_i_s_o88591 {
            return StandardCharsets::ISO_8859_1;
        }
        if can_be_shift_j_i_s {
            return SHIFT_JIS_CHARSET;
        }
        if can_be_u_t_f8 {
            return StandardCharsets::UTF_8;
        }
        // Otherwise, we take a wild guess with platform encoding
        return PLATFORM_DEFAULT_ENCODING;
    }
}
