/*
 * Copyright 2016 ZXing authors
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

use std::{path::PathBuf, rc::Rc};

use crate::{BufferedImageLuminanceSource, BinaryBitmap, common::HybridBinarizer, MultiFormatReader, BarcodeFormat};

use super::{GenericMultipleBarcodeReader, MultipleBarcodeReader};

/**
 * Tests {@link MultipleBarcodeReader}.
 */

  #[test]
  fn testMulti()  {
    // Very basic test for now
    let mut testBase = PathBuf::from("test_resources/blackbox/multi-1");

    testBase.push("1.png");
    let image = image::io::Reader::open(testBase)
            .expect("image must open")
            .decode()
            .expect("must decode");
    let source =  BufferedImageLuminanceSource::new(image);
    let bitmap =  BinaryBitmap::new( Rc::new(HybridBinarizer::new(Box::new(source))));

    let mut reader =  GenericMultipleBarcodeReader::new( MultiFormatReader::default());
    let results = reader.decodeMultiple(&bitmap).expect("must decode multi");
    // assertNotNull(results);
    assert_eq!(2, results.len());

    assert_eq!("031415926531", results[0].getText());
    assert_eq!(&BarcodeFormat::UPC_A, results[0].getBarcodeFormat());

    assert_eq!("www.airtable.com/jobs", results[1].getText());
    assert_eq!(&BarcodeFormat::QR_CODE, results[1].getBarcodeFormat());
  }
