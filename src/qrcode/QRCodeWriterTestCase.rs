/*
 * Copyright 2008 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::{collections::HashMap, path::PathBuf};

use image::DynamicImage;

use crate::{
    common::BitMatrix, qrcode::QRCodeWriter, BarcodeFormat,
    EncodeHintType, EncodeHintValue, Writer,
};

use super::decoder::ErrorCorrectionLevel;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported and expanded from C++
 */

const BASE_IMAGE_PATH: &str = "test_resources/golden/qrcode/";

fn loadImage(fileName: &str) -> DynamicImage {
    let mut file = PathBuf::from(BASE_IMAGE_PATH);
    file.push(fileName);
    if !file.exists() {
        // try starting with 'core' since the test base is often given as the project root
        file = PathBuf::from("core/");
        file.push(BASE_IMAGE_PATH);
        file.push(fileName); //Paths.get("core/").resolve(BASE_IMAGE_PATH).resolve(fileName);
    }
    assert!(
        file.exists(),
        "Please download and install test images, and run from the 'core' directory"
    );
    image::io::Reader::open(file)
        .expect("image should load")
        .decode()
        .expect("decode")
}

// In case the golden images are not monochromatic, convert the RGB values to greyscale.
fn createMatrixFromImage(image: DynamicImage) -> BitMatrix {
    let width = image.width() as usize;
    let height = image.height() as usize;
    // let pixels = vec![0u32; width * height]; //new int[width * height];
    // image.getRGB(0, 0, width, height, pixels, 0, width);
    let img_src = image.into_rgb8(); //image.as_rgb8().unwrap().as_bytes();
                                     // let pixels = img_src.as_bytes();

    let mut matrix = BitMatrix::new(width as u32, height as u32).expect("create new bitmatrix");
    for y in 0..height as u32 {
        // for (int y = 0; y < height; y++) {
        for x in 0..width as u32 {
            // for (int x = 0; x < width; x++) {
            //let pixel = pixels[y * width + x] as u32;
            let [red, green, blue] = img_src.get_pixel(x, y).0;
            let luminance = (306 * red as u32 + 601 * green as u32 + 117 * blue as u32) >> 10;
            if luminance <= 0x7F {
                matrix.set(x, y);
            }
        }
    }
    matrix
}

#[test]
fn testQRCodeWriter() {
    // The QR should be multiplied up to fit, with extra padding if necessary
    let bigEnough = 256;
    let writer = QRCodeWriter {};
    let matrix = writer.encode_with_hints(
        "http://www.google.com/",
        &BarcodeFormat::QR_CODE,
        bigEnough,
        bigEnough,
        &HashMap::new(),
    );
    assert!(matrix.is_ok());
    let mut matrix = matrix.unwrap();
    assert_eq!(bigEnough as u32, matrix.getWidth());
    assert_eq!(bigEnough as u32, matrix.getHeight());

    // The QR will not fit in this size, so the matrix should come back bigger
    let tooSmall = 20;
    matrix = writer
        .encode_with_hints(
            "http://www.google.com/",
            &BarcodeFormat::QR_CODE,
            tooSmall,
            tooSmall,
            &HashMap::new(),
        )
        .expect("should encode");
    // assertNotNull(matrix);
    assert!((tooSmall as u32) < matrix.getWidth());
    assert!((tooSmall as u32) < matrix.getHeight());

    // We should also be able to handle non-square requests by padding them
    let strangeWidth = 500;
    let strangeHeight = 100;
    matrix = writer
        .encode_with_hints(
            "http://www.google.com/",
            &BarcodeFormat::QR_CODE,
            strangeWidth,
            strangeHeight,
            &HashMap::new(),
        )
        .expect("should encode");
    // assertNotNull(matrix);
    assert_eq!(strangeWidth as u32, matrix.getWidth());
    assert_eq!(strangeHeight as u32, matrix.getHeight());
}

fn compareToGoldenFile(
    contents: &str,
    ecLevel: &ErrorCorrectionLevel,
    resolution: u32,
    fileName: &str,
) {
    let image = loadImage(fileName);
    // assertNotNull(image);
    let goldenRXingResult = createMatrixFromImage(image);
    // assertNotNull(goldenRXingResult);

    let mut hints = HashMap::new();
    hints.insert(
        EncodeHintType::ERROR_CORRECTION,
        EncodeHintValue::ErrorCorrection(ecLevel.get_value().to_string()),
    );
    let writer = QRCodeWriter {};
    let generatedRXingResult = writer
        .encode_with_hints(
            contents,
            &BarcodeFormat::QR_CODE,
            resolution as i32,
            resolution as i32,
            &hints,
        )
        .expect("should encode");

    assert_eq!(resolution, generatedRXingResult.getWidth());
    assert_eq!(resolution, generatedRXingResult.getHeight());
    assert_eq!(goldenRXingResult, generatedRXingResult);
}

// Golden images are generated with "qrcode_sample.cc". The images are checked with both eye balls
// and cell phones. We expect pixel-perfect results, because the error correction level is known,
// and the pixel dimensions matches exactly.
#[test]
fn testRegressionTest() {
    compareToGoldenFile(
        "http://www.google.com/",
        &ErrorCorrectionLevel::M,
        99,
        "renderer-test-01.png",
    );
}
