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
// package com::google::zxing::oned::rss::expanded::decoders;

/**
 * @author Pablo OrduÃ±a, University of Deusto (pablo.orduna@deusto.es)
 * @author Eduardo Castillejo, University of Deusto (eduardo.castillejo@deusto.es)
 */
pub struct AbstractExpandedDecoder {

     let information: BitArray;

     let general_decoder: GeneralAppIdDecoder;
}

impl AbstractExpandedDecoder {

    fn new( information: &BitArray) -> AbstractExpandedDecoder {
        let .information = information;
        let .generalDecoder = GeneralAppIdDecoder::new(information);
    }

    pub fn  get_information(&self) -> BitArray  {
        return self.information;
    }

    pub fn  get_general_decoder(&self) -> GeneralAppIdDecoder  {
        return self.general_decoder;
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>  ;

    pub fn  create_decoder( information: &BitArray) -> AbstractExpandedDecoder  {
        if information.get(1) {
            return AI01AndOtherAIs::new(information);
        }
        if !information.get(2) {
            return AnyAIDecoder::new(information);
        }
         let four_bit_encodation_method: i32 = GeneralAppIdDecoder::extract_numeric_value_from_bit_array(information, 1, 4);
        match four_bit_encodation_method {
              4 => 
                 {
                    return AI013103decoder::new(information);
                }
              5 => 
                 {
                    return AI01320xDecoder::new(information);
                }
        }
         let five_bit_encodation_method: i32 = GeneralAppIdDecoder::extract_numeric_value_from_bit_array(information, 1, 5);
        match five_bit_encodation_method {
              12 => 
                 {
                    return AI01392xDecoder::new(information);
                }
              13 => 
                 {
                    return AI01393xDecoder::new(information);
                }
        }
         let seven_bit_encodation_method: i32 = GeneralAppIdDecoder::extract_numeric_value_from_bit_array(information, 1, 7);
        match seven_bit_encodation_method {
              56 => 
                 {
                    return AI013x0x1xDecoder::new(information, "310", "11");
                }
              57 => 
                 {
                    return AI013x0x1xDecoder::new(information, "320", "11");
                }
              58 => 
                 {
                    return AI013x0x1xDecoder::new(information, "310", "13");
                }
              59 => 
                 {
                    return AI013x0x1xDecoder::new(information, "320", "13");
                }
              60 => 
                 {
                    return AI013x0x1xDecoder::new(information, "310", "15");
                }
              61 => 
                 {
                    return AI013x0x1xDecoder::new(information, "320", "15");
                }
              62 => 
                 {
                    return AI013x0x1xDecoder::new(information, "310", "17");
                }
              63 => 
                 {
                    return AI013x0x1xDecoder::new(information, "320", "17");
                }
        }
        throw IllegalStateException::new(format!("unknown decoder: {}", information));
    }
}

