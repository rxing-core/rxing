/*
 * Witness Data for Zero-Knowledge Proofs
 *
 * This module provides a structure to capture intermediate processing data
 * from barcode decoding for use in zero-knowledge proof generation.
 */

use crate::common::BitMatrix;

#[cfg(feature = "serde")]
use serde::Serialize;

/**
 * Holds witness data for zero-knowledge proof generation during barcode processing.
 *
 * # Fields
 * * `width` - The width of the image in pixels
 * * `height` - The height of the image in pixels
 * * `image` - The original grayscale luminance values (0-255 per pixel), stored row-major
 * * `binarized_image` - The binarized black/white BitMatrix after threshold application
 */
#[derive(Clone, Debug)]
pub struct WitnessData {
    /// The width of the image in pixels
    pub width: usize,

    /// The height of the image in pixels
    pub height: usize,

    /// The original grayscale luminance values (0-255 per pixel)
    /// Stored in row-major order: pixels are stored row by row, left to right, top to bottom
    /// Total size: width * height bytes
    pub image: Vec<u8>,

    /// The binarized image after applying the threshold
    /// Pixels are represented as bits: true/1 = black, false/0 = white
    pub binarized_image: Option<BitMatrix>,
}

/**
 * WitnessData with no optional fields
 */

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Clone, Debug)]
pub struct FinalizedWitnessData {
    /// The width of the image in pixels
    pub width: usize,

    /// The height of the image in pixels
    pub height: usize,

    /// The original grayscale luminance values (0-255 per pixel)
    /// Stored in row-major order: pixels are stored row by row, left to right, top to bottom
    /// Total size: width * height bytes
    pub image: Vec<u8>,

    /// The binarized image after applying the threshold
    /// Pixels are represented as bits: true/1 = black, false/0 = white
    /// Serialized as a flattened 1D array of booleans in row-major order
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_bitmatrix"))]
    pub binarized_image: BitMatrix,
}

impl FinalizedWitnessData {
    pub fn new(width: usize, height: usize, image: Vec<u8>, binarized_image: BitMatrix) -> Self {
        assert_eq!(
            image.len(),
            width * height,
            "Image size mismatch: expected {} bytes, got {}",
            width * height,
            image.len()
        );

        Self {
            width,
            height,
            image,
            binarized_image,
        }
    }

    pub fn from_witness_data(witness_data: &WitnessData) -> Result<Self, String> {
        let binarized_image = Option::ok_or(
            witness_data.binarized_image.clone(),
            "no binarized image data",
        )?;

        Ok(Self {
            width: witness_data.width,
            height: witness_data.height,
            image: witness_data.image.clone(),
            binarized_image,
        })
    }

    /**
     * Saves this WitnessData to a JSON file.
     *
     * # Arguments
     * * `path` - The file path to write to
     *
     * # Returns
     * Result indicating success or error
     */
    #[cfg(feature = "serde")]
    pub fn save_to_json(&self, path: &str) -> Result<(), String> {
        use std::fs::File;
        use std::io::Write;

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;

        let mut file =
            File::create(path).map_err(|e| format!("Failed to create file '{}': {}", path, e))?;

        file.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write to file '{}': {}", path, e))?;

        Ok(())
    }
}

impl WitnessData {
    /**
     * Creates a new WitnessData instance.
     *
     * # Arguments
     * * `width` - The width of the image in pixels
     * * `height` - The height of the image in pixels
     * * `image` - The grayscale luminance data (must be width * height bytes)
     * * `binarized_image` - The binarized BitMatrix
     *
     * # Panics
     * Panics if `image.len()` does not equal `width * height`
     */
    pub fn new(width: usize, height: usize, image: Vec<u8>) -> Self {
        assert_eq!(
            image.len(),
            width * height,
            "Image size mismatch: expected {} bytes, got {}",
            width * height,
            image.len()
        );

        Self {
            width,
            height,
            image,
            binarized_image: None,
        }
    }

    /**
     * Returns the width of the image in pixels.
     */
    pub fn width(&self) -> usize {
        self.width
    }

    /**
     * Returns the height of the image in pixels.
     */
    pub fn height(&self) -> usize {
        self.height
    }

    /**
     * Returns a reference to the grayscale image data.
     */
    pub fn image(&self) -> &[u8] {
        &self.image
    }

    /**
     * Returns a reference to the binarized image.
     */
    pub fn binarized_image(&self) -> &Option<BitMatrix> {
        &self.binarized_image
    }

    pub fn set_binarized_image(&mut self, binarized_image: BitMatrix) {
        self.binarized_image = Some(binarized_image)
    }

    /**
     * Makes sure that all optional fields have data in them.
     */
    pub fn finalize(&self) -> Result<FinalizedWitnessData, String> {
        FinalizedWitnessData::from_witness_data(self)
    }

    /**
     * Gets the grayscale pixel value at position (x, y).
     *
     * # Arguments
     * * `x` - The x coordinate (column)
     * * `y` - The y coordinate (row)
     *
     * # Panics
     * Panics if x >= width or y >= height
     */
    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        assert!(
            x < self.width && y < self.height,
            "Pixel coordinates out of bounds"
        );
        self.image[y * self.width + x]
    }

    /**
     * Gets the binarized bit value at position (x, y).
     *
     * # Arguments
     * * `x` - The x coordinate (column)
     * * `y` - The y coordinate (row)
     *
     * # Returns
     * `true` if the pixel is black, `false` if white
     */
    pub fn get_binarized_pixel(&self, x: usize, y: usize) -> Option<bool> {
        match &self.binarized_image {
            Some(binarized_image) => Some(binarized_image.get(x as u32, y as u32)),
            None => None,
        }
    }
}

// Custom serialization for BitMatrix - convert to flattened 1D array of booleans
// Stored in row-major order: row 0 from left to right, then row 1, etc.
#[cfg(feature = "serde")]
fn serialize_bitmatrix<S>(matrix: &BitMatrix, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;

    let width = matrix.getWidth();
    let height = matrix.getHeight();
    let total_pixels = (width * height) as usize;

    let mut seq = serializer.serialize_seq(Some(total_pixels))?;
    for y in 0..height {
        for x in 0..width {
            seq.serialize_element(&matrix.get(x, y))?;
        }
    }
    seq.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_data_creation() {
        // Create a simple 4x4 test image
        let image = vec![
            0, 64, 127, 128, 129, 192, 200, 255, 50, 100, 150, 200, 127, 128, 129, 130,
        ];

        let mut binarized = BitMatrix::new(4, 4).unwrap();
        // Set some pixels black (< 128)
        binarized.set(0, 0); // 0
        binarized.set(1, 0); // 64
        binarized.set(2, 0); // 127
        // 128+ stay white

        let witness = WitnessData::new(4, 4, image.clone(), binarized);

        assert_eq!(witness.width(), 4);
        assert_eq!(witness.height(), 4);
        assert_eq!(witness.image().len(), 16);
        assert_eq!(witness.get_pixel(0, 0), 0);
        assert_eq!(witness.get_pixel(3, 1), 255);
        assert_eq!(witness.get_binarized_pixel(0, 0), true); // black
        assert_eq!(witness.get_binarized_pixel(3, 0), false); // white
    }

    #[test]
    #[should_panic(expected = "Image size mismatch")]
    fn test_witness_data_size_mismatch() {
        let image = vec![1, 2, 3]; // Wrong size
        let binarized = BitMatrix::new(4, 4).unwrap();
        let _witness = WitnessData::new(4, 4, image, binarized);
    }
}
