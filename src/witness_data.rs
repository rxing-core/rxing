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
 * * `image` - The original grayscale luminance values (0-255 per pixel), stored as 2D array [row][col]
 * * `binarized_image` - The binarized black/white BitMatrix after threshold application
 */
#[derive(Clone, Debug)]
pub struct WitnessData {
    /// The width of the image in pixels
    pub width: usize,

    /// The height of the image in pixels
    pub height: usize,

    /// The original grayscale luminance values (0-255 per pixel)
    /// Stored as a 2D array: image[row][col] where row is y-coordinate and col is x-coordinate
    /// Outer vector has `height` elements, each inner vector has `width` elements
    pub image: Vec<Vec<u8>>,

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
    /// Stored as a 2D array: image[row][col] where row is y-coordinate and col is x-coordinate
    /// Outer vector has `height` elements, each inner vector has `width` elements
    pub image: Vec<Vec<u8>>,

    /// The binarized image after applying the threshold
    /// Pixels are represented as bits: 1 = black, 0 = white
    /// Serialized as a 2D array of 0s and 1s: binarized_image[row][col]
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_bitmatrix"))]
    pub binarized_image: BitMatrix,
}

impl FinalizedWitnessData {
    pub fn new(width: usize, height: usize, image: Vec<Vec<u8>>, binarized_image: BitMatrix) -> Self {
        assert_eq!(
            image.len(),
            height,
            "Image height mismatch: expected {} rows, got {}",
            height,
            image.len()
        );
        for (row_idx, row) in image.iter().enumerate() {
            assert_eq!(
                row.len(),
                width,
                "Image width mismatch at row {}: expected {} columns, got {}",
                row_idx,
                width,
                row.len()
            );
        }

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
     * * `image` - The grayscale luminance data as a 2D array [row][col]
     *
     * # Panics
     * Panics if the image dimensions don't match width and height
     */
    pub fn new(width: usize, height: usize, image: Vec<Vec<u8>>) -> Self {
        assert_eq!(
            image.len(),
            height,
            "Image height mismatch: expected {} rows, got {}",
            height,
            image.len()
        );
        for (row_idx, row) in image.iter().enumerate() {
            assert_eq!(
                row.len(),
                width,
                "Image width mismatch at row {}: expected {} columns, got {}",
                row_idx,
                width,
                row.len()
            );
        }

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
     * Returns a reference to the grayscale image data as a 2D array.
     */
    pub fn image(&self) -> &[Vec<u8>] {
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
        self.image[y][x]
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

// Custom serialization for BitMatrix - convert to 2D array of 0s and 1s
// Stored as rows[y][x] where each value is 0 (white) or 1 (black)
#[cfg(feature = "serde")]
fn serialize_bitmatrix<S>(matrix: &BitMatrix, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;

    let width = matrix.getWidth();
    let height = matrix.getHeight();

    // Create outer sequence for rows
    let mut rows = serializer.serialize_seq(Some(height as usize))?;
    for y in 0..height {
        // Build each row as a Vec<u8> of 0s and 1s
        let row: Vec<u8> = (0..width)
            .map(|x| if matrix.get(x, y) { 1 } else { 0 })
            .collect();
        rows.serialize_element(&row)?;
    }
    rows.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_witness_data_creation() {
        // Create a simple 4x4 test image as 2D array
        let image = vec![
            vec![0, 64, 127, 128],
            vec![129, 192, 200, 255],
            vec![50, 100, 150, 200],
            vec![127, 128, 129, 130],
        ];

        let mut binarized = BitMatrix::new(4, 4).unwrap();
        // Set some pixels black (< 128)
        binarized.set(0, 0); // 0
        binarized.set(1, 0); // 64
        binarized.set(2, 0); // 127
        // 128+ stay white

        let mut witness = WitnessData::new(4, 4, image.clone());
        witness.set_binarized_image(binarized);

        assert_eq!(witness.width(), 4);
        assert_eq!(witness.height(), 4);
        assert_eq!(witness.image().len(), 4); // 4 rows
        assert_eq!(witness.image()[0].len(), 4); // 4 columns per row
        assert_eq!(witness.get_pixel(0, 0), 0);
        assert_eq!(witness.get_pixel(3, 1), 255);
        assert_eq!(witness.get_binarized_pixel(0, 0), Some(true)); // black
        assert_eq!(witness.get_binarized_pixel(3, 0), Some(false)); // white
    }

    #[test]
    #[should_panic(expected = "Image height mismatch")]
    fn test_witness_data_size_mismatch() {
        let image = vec![vec![1, 2, 3]]; // Wrong number of rows
        let _witness = WitnessData::new(3, 4, image);
    }
}
