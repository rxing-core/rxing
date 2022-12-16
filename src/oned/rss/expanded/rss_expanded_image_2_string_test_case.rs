/*
 * Copyright (C) 2010 ZXing authors
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

/*
 * These authors would like to acknowledge the Spanish Ministry of Industry,
 * Tourism and Trade, for the support in the project TSI020301-2008-2
 * "PIRAmIDE: Personalizable Interactions with Resources on AmI-enabled
 * Mobile Dynamic Environments", led by Treelogic
 * ( http://www.treelogic.com/ ):
 *
 *   http://www.piramidepse.com/
 */

use std::{rc::Rc, collections::HashMap};

use crate::{BinaryBitmap, common::{GlobalHistogramBinarizer, BitArray}, BufferedImageLuminanceSource, oned::{rss::expanded::RSSExpandedReader, OneDReader}, BarcodeFormat};


/**
 * @author Pablo Orduña, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */

 #[test]
 fn  testDecodeRow2string1()  {
    assertCorrectImage2string("1.png", "(11)100224(17)110224(3102)000100");
  }

  #[test]
  fn  testDecodeRow2string2()  {
    assertCorrectImage2string("2.png", "(01)90012345678908(3103)001750");
  }

  #[test]
  fn  testDecodeRow2string3()  {
    assertCorrectImage2string("3.png", "(10)12A");
  }

  #[test]
  fn  testDecodeRow2string4()  {
    assertCorrectImage2string("4.png", "(01)98898765432106(3202)012345(15)991231");
  }

  #[test]
  fn testDecodeRow2string5()  {
    assertCorrectImage2string("5.png", "(01)90614141000015(3202)000150");
  }

  #[test]
  fn  testDecodeRow2string7()  {
    assertCorrectImage2string("7.png", "(10)567(11)010101");
  }

  #[test]
  fn testDecodeRow2string10()  {
    let expected = "(01)98898765432106(15)991231(3103)001750(10)12A(422)123(21)123456(423)012345678901";
    assertCorrectImage2string("10.png", expected);
  }

  #[test]
  fn  testDecodeRow2string11()  {
    assertCorrectImage2string("11.png", "(01)98898765432106(15)991231(3103)001750(10)12A(422)123(21)123456");
  }

  #[test]
  fn  testDecodeRow2string12()  {
    assertCorrectImage2string("12.png", "(01)98898765432106(3103)001750");
  }

  #[test]
  fn  testDecodeRow2string13()  {
    assertCorrectImage2string("13.png", "(01)90012345678908(3922)795");
  }

  #[test]
  fn  testDecodeRow2string14()  {
    assertCorrectImage2string("14.png", "(01)90012345678908(3932)0401234");
  }

  #[test]
  fn  testDecodeRow2string15()  {
    assertCorrectImage2string("15.png", "(01)90012345678908(3102)001750(11)100312");
  }

  #[test]
  fn  testDecodeRow2string16()  {
    assertCorrectImage2string("16.png", "(01)90012345678908(3202)001750(11)100312");
  }

  #[test]
  fn  testDecodeRow2string17()  {
    assertCorrectImage2string("17.png", "(01)90012345678908(3102)001750(13)100312");
  }

  #[test]
  fn  testDecodeRow2string18()  {
    assertCorrectImage2string("18.png", "(01)90012345678908(3202)001750(13)100312");
  }

  #[test]
  fn  testDecodeRow2string19()  {
    assertCorrectImage2string("19.png", "(01)90012345678908(3102)001750(15)100312");
  }

  #[test]
  fn  testDecodeRow2string20()  {
    assertCorrectImage2string("20.png", "(01)90012345678908(3202)001750(15)100312");
  }

  #[test]
  fn  testDecodeRow2string21()  {
    assertCorrectImage2string("21.png", "(01)90012345678908(3102)001750(17)100312");
  }

  #[test]
  fn  testDecodeRow2string22()  {
    assertCorrectImage2string("22.png", "(01)90012345678908(3202)001750(17)100312");
  }

  #[test]
  fn  testDecodeRow2string25()  {
    assertCorrectImage2string("25.png", "(10)123");
  }

  #[test]
  fn  testDecodeRow2string26()  {
    assertCorrectImage2string("26.png", "(10)5678(11)010101");
  }

  #[test]
  fn  testDecodeRow2string27()  {
    assertCorrectImage2string("27.png", "(10)1098-1234");
  }

  #[test]
  fn  testDecodeRow2string28()  {
    assertCorrectImage2string("28.png", "(10)1098/1234");
  }

  #[test]
  fn  testDecodeRow2string29()  {
    assertCorrectImage2string("29.png", "(10)1098.1234");
  }

  #[test]
  fn  testDecodeRow2string30()  {
    assertCorrectImage2string("30.png", "(10)1098*1234");
  }

  #[test]
  fn  testDecodeRow2string31()  {
    assertCorrectImage2string("31.png", "(10)1098,1234");
  }

  #[test]
  fn testDecodeRow2string32()  {
    assertCorrectImage2string("32.png", "(15)991231(3103)001750(10)12A(422)123(21)123456(423)0123456789012");
  }

  fn assertCorrectImage2string( fileName:&str, expected:&str)
      {
    let path = format!("test_resources/blackbox/rssexpanded-1/{}",fileName);

    let image = image::open(path).expect("load image");
    let binaryMap =
         BinaryBitmap::new(Rc::new(GlobalHistogramBinarizer::new(Box::new(BufferedImageLuminanceSource::new(image)))));
    let rowNumber = binaryMap.getHeight() / 2;
    let row = binaryMap.getBlackRow(rowNumber, &mut BitArray::new()).expect("get row");

    let mut rssExpandedReader =  RSSExpandedReader::new();
    let result = rssExpandedReader.decodeRow(rowNumber as u32, &row, &HashMap::new()).expect("should decode");



    assert_eq!(&BarcodeFormat::RSS_EXPANDED, result.getBarcodeFormat());
    assert_eq!(expected, result.getText());
  }

