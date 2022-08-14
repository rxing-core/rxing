use crate::{FormatException,NotFoundException};
use crate::common::BitArray;




// NEW FILE: a_i013103decoder.rs
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
 */
struct AI013103decoder {
    super: AI013x0xDecoder;
}

impl AI013103decoder {

    fn new( information: &BitArray) -> AI013103decoder {
        super(information);
    }

    pub fn  add_weight_code(&self,  buf: &StringBuilder,  weight: i32)   {
        buf.append("(3103)");
    }

    pub fn  check_weight(&self,  weight: i32) -> i32  {
        return weight;
    }
}

// NEW FILE: a_i01320x_decoder.rs
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
 */
struct AI01320xDecoder {
    super: AI013x0xDecoder;
}

impl AI01320xDecoder {

    fn new( information: &BitArray) -> AI01320xDecoder {
        super(information);
    }

    pub fn  add_weight_code(&self,  buf: &StringBuilder,  weight: i32)   {
        if weight < 10000 {
            buf.append("(3202)");
        } else {
            buf.append("(3203)");
        }
    }

    pub fn  check_weight(&self,  weight: i32) -> i32  {
        if weight < 10000 {
            return weight;
        }
        return weight - 10000;
    }
}

// NEW FILE: a_i01392x_decoder.rs
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
 */

 const HEADER_SIZE: i32 = 5 + 1 + 2;

 const LAST_DIGIT_SIZE: i32 = 2;
struct AI01392xDecoder {
    super: AI01decoder;
}

impl AI01392xDecoder {

    fn new( information: &BitArray) -> AI01392xDecoder {
        super(information);
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>   {
        if self.get_information().get_size() < HEADER_SIZE + GTIN_SIZE {
            throw NotFoundException::get_not_found_instance();
        }
         let buf: StringBuilder = StringBuilder::new();
        encode_compressed_gtin(&buf, HEADER_SIZE);
         let last_a_idigit: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(HEADER_SIZE + GTIN_SIZE, LAST_DIGIT_SIZE);
        buf.append("(392");
        buf.append(last_a_idigit);
        buf.append(')');
         let decoded_information: DecodedInformation = self.get_general_decoder().decode_general_purpose_field(HEADER_SIZE + GTIN_SIZE + LAST_DIGIT_SIZE, null);
        buf.append(&decoded_information.get_new_string());
        return Ok(buf.to_string());
    }
}

// NEW FILE: a_i01393x_decoder.rs
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
// package com::google::zxing::oned::rss::expanded::decoders;

/**
 * @author Pablo OrduÃ±a, University of Deusto (pablo.orduna@deusto.es)
 */

 const HEADER_SIZE: i32 = 5 + 1 + 2;

 const LAST_DIGIT_SIZE: i32 = 2;

 const FIRST_THREE_DIGITS_SIZE: i32 = 10;
struct AI01393xDecoder {
    super: AI01decoder;
}

impl AI01393xDecoder {

    fn new( information: &BitArray) -> AI01393xDecoder {
        super(information);
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>   {
        if self.get_information().get_size() < HEADER_SIZE + GTIN_SIZE {
            throw NotFoundException::get_not_found_instance();
        }
         let buf: StringBuilder = StringBuilder::new();
        encode_compressed_gtin(&buf, HEADER_SIZE);
         let last_a_idigit: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(HEADER_SIZE + GTIN_SIZE, LAST_DIGIT_SIZE);
        buf.append("(393");
        buf.append(last_a_idigit);
        buf.append(')');
         let first_three_digits: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(HEADER_SIZE + GTIN_SIZE + LAST_DIGIT_SIZE, FIRST_THREE_DIGITS_SIZE);
        if first_three_digits / 100 == 0 {
            buf.append('0');
        }
        if first_three_digits / 10 == 0 {
            buf.append('0');
        }
        buf.append(first_three_digits);
         let general_information: DecodedInformation = self.get_general_decoder().decode_general_purpose_field(HEADER_SIZE + GTIN_SIZE + LAST_DIGIT_SIZE + FIRST_THREE_DIGITS_SIZE, null);
        buf.append(&general_information.get_new_string());
        return Ok(buf.to_string());
    }
}

// NEW FILE: a_i013x0x1x_decoder.rs
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

 const HEADER_SIZE: i32 = 7 + 1;

 const WEIGHT_SIZE: i32 = 20;

 const DATE_SIZE: i32 = 16;
struct AI013x0x1xDecoder {
    super: AI01weightDecoder;

     let date_code: String;

     let first_a_idigits: String;
}

impl AI013x0x1xDecoder {

    fn new( information: &BitArray,  first_a_idigits: &String,  date_code: &String) -> AI013x0x1xDecoder {
        super(information);
        let .dateCode = date_code;
        let .firstAIdigits = first_a_idigits;
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
        if self.get_information().get_size() != HEADER_SIZE + GTIN_SIZE + WEIGHT_SIZE + DATE_SIZE {
            throw NotFoundException::get_not_found_instance();
        }
         let buf: StringBuilder = StringBuilder::new();
        encode_compressed_gtin(&buf, HEADER_SIZE);
        encode_compressed_weight(&buf, HEADER_SIZE + GTIN_SIZE, WEIGHT_SIZE);
        self.encode_compressed_date(&buf, HEADER_SIZE + GTIN_SIZE + WEIGHT_SIZE);
        return Ok(buf.to_string());
    }

    fn  encode_compressed_date(&self,  buf: &StringBuilder,  current_pos: i32)   {
         let numeric_date: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(current_pos, DATE_SIZE);
        if numeric_date == 38400 {
            return;
        }
        buf.append('(');
        buf.append(self.dateCode);
        buf.append(')');
         let day: i32 = numeric_date % 32;
        numeric_date /= 32;
         let month: i32 = numeric_date % 12 + 1;
        numeric_date /= 12;
         let year: i32 = numeric_date;
        if year / 10 == 0 {
            buf.append('0');
        }
        buf.append(year);
        if month / 10 == 0 {
            buf.append('0');
        }
        buf.append(month);
        if day / 10 == 0 {
            buf.append('0');
        }
        buf.append(day);
    }

    pub fn  add_weight_code(&self,  buf: &StringBuilder,  weight: i32)   {
        buf.append('(');
        buf.append(self.firstAIdigits);
        buf.append(weight / 100000);
        buf.append(')');
    }

    pub fn  check_weight(&self,  weight: i32) -> i32  {
        return weight % 100000;
    }
}

// NEW FILE: a_i013x0x_decoder.rs
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
 */

 const HEADER_SIZE: i32 = 4 + 1;

 const WEIGHT_SIZE: i32 = 15;
struct AI013x0xDecoder {
    super: AI01weightDecoder;
}

impl AI013x0xDecoder {

    fn new( information: &BitArray) -> AI013x0xDecoder {
        super(information);
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
        if self.get_information().get_size() != HEADER_SIZE + GTIN_SIZE + WEIGHT_SIZE {
            throw NotFoundException::get_not_found_instance();
        }
         let buf: StringBuilder = StringBuilder::new();
        encode_compressed_gtin(&buf, HEADER_SIZE);
        encode_compressed_weight(&buf, HEADER_SIZE + GTIN_SIZE, WEIGHT_SIZE);
        return Ok(buf.to_string());
    }
}

// NEW FILE: a_i01_and_other_a_is.rs
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

//first bit encodes the linkage flag,
 const HEADER_SIZE: i32 = 1 + 1 + 2;
struct AI01AndOtherAIs {
    super: AI01decoder;
}

impl AI01AndOtherAIs {

    //the second one is the encodation method, and the other two are for the variable length
    fn new( information: &BitArray) -> AI01AndOtherAIs {
        super(information);
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>   {
         let buff: StringBuilder = StringBuilder::new();
        buff.append("(01)");
         let initial_gtin_position: i32 = buff.length();
         let first_gtin_digit: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(HEADER_SIZE, 4);
        buff.append(first_gtin_digit);
        self.encode_compressed_gtin_without_a_i(&buff, HEADER_SIZE + 4, initial_gtin_position);
        return Ok(self.get_general_decoder().decode_all_codes(&buff, HEADER_SIZE + 44));
    }
}

// NEW FILE: a_i01decoder.rs
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

 const GTIN_SIZE: i32 = 40;
struct AI01decoder {
    super: AbstractExpandedDecoder;
}

impl AI01decoder {

    fn new( information: &BitArray) -> AI01decoder {
        super(information);
    }

    fn  encode_compressed_gtin(&self,  buf: &StringBuilder,  current_pos: i32)   {
        buf.append("(01)");
         let initial_position: i32 = buf.length();
        buf.append('9');
        self.encode_compressed_gtin_without_a_i(&buf, current_pos, initial_position);
    }

    fn  encode_compressed_gtin_without_a_i(&self,  buf: &StringBuilder,  current_pos: i32,  initial_buffer_position: i32)   {
         {
             let mut i: i32 = 0;
            while i < 4 {
                {
                     let current_block: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(current_pos + 10 * i, 10);
                    if current_block / 100 == 0 {
                        buf.append('0');
                    }
                    if current_block / 10 == 0 {
                        buf.append('0');
                    }
                    buf.append(current_block);
                }
                i += 1;
             }
         }

        ::append_check_digit(&buf, initial_buffer_position);
    }

    fn  append_check_digit( buf: &StringBuilder,  current_pos: i32)   {
         let check_digit: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < 13 {
                {
                     let digit: i32 = buf.char_at(i + current_pos) - '0';
                    check_digit +=  if (i & 0x01) == 0 { 3 * digit } else { digit };
                }
                i += 1;
             }
         }

        check_digit = 10 - (check_digit % 10);
        if check_digit == 10 {
            check_digit = 0;
        }
        buf.append(check_digit);
    }
}

// NEW FILE: a_i01weight_decoder.rs
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
 */
struct AI01weightDecoder {
    super: AI01decoder;
}

impl AI01weightDecoder {

    fn new( information: &BitArray) -> AI01weightDecoder {
        super(information);
    }

    fn  encode_compressed_weight(&self,  buf: &StringBuilder,  current_pos: i32,  weight_size: i32)   {
         let original_weight_numeric: i32 = self.get_general_decoder().extract_numeric_value_from_bit_array(current_pos, weight_size);
        self.add_weight_code(&buf, original_weight_numeric);
         let weight_numeric: i32 = self.check_weight(original_weight_numeric);
         let current_divisor: i32 = 100000;
         {
             let mut i: i32 = 0;
            while i < 5 {
                {
                    if weight_numeric / current_divisor == 0 {
                        buf.append('0');
                    }
                    current_divisor /= 10;
                }
                i += 1;
             }
         }

        buf.append(weight_numeric);
    }

    pub fn  add_weight_code(&self,  buf: &StringBuilder,  weight: i32)  ;

    pub fn  check_weight(&self,  weight: i32) -> i32 ;
}

// NEW FILE: abstract_expanded_decoder.rs
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

// NEW FILE: any_a_i_decoder.rs
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

 const HEADER_SIZE: i32 = 2 + 1 + 2;
struct AnyAIDecoder {
    super: AbstractExpandedDecoder;
}

impl AnyAIDecoder {

    fn new( information: &BitArray) -> AnyAIDecoder {
        super(information);
    }

    pub fn  parse_information(&self) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>   {
         let buf: StringBuilder = StringBuilder::new();
        return Ok(self.get_general_decoder().decode_all_codes(&buf, HEADER_SIZE));
    }
}

// NEW FILE: block_parsed_result.rs
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
struct BlockParsedResult {

     let decoded_information: DecodedInformation;

     let finished: bool;
}

impl BlockParsedResult {

    fn new() -> BlockParsedResult {
        this(null, false);
    }

    fn new( information: &DecodedInformation,  finished: bool) -> BlockParsedResult {
        let .finished = finished;
        let .decodedInformation = information;
    }

    fn  get_decoded_information(&self) -> DecodedInformation  {
        return self.decodedInformation;
    }

    fn  is_finished(&self) -> bool  {
        return self.finished;
    }
}

// NEW FILE: current_parsing_state.rs
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
 */
struct CurrentParsingState {

     let mut position: i32;

     let mut encoding: State;
}

impl CurrentParsingState {

    enum State {

        NUMERIC(), ALPHA(), ISO_IEC_646()
    }

    fn new() -> CurrentParsingState {
        let .position = 0;
        let .encoding = State::NUMERIC;
    }

    fn  get_position(&self) -> i32  {
        return self.position;
    }

    fn  set_position(&self,  position: i32)   {
        self.position = position;
    }

    fn  increment_position(&self,  delta: i32)   {
        self.position += delta;
    }

    fn  is_alpha(&self) -> bool  {
        return self.encoding == State::ALPHA;
    }

    fn  is_numeric(&self) -> bool  {
        return self.encoding == State::NUMERIC;
    }

    fn  is_iso_iec646(&self) -> bool  {
        return self.encoding == State::ISO_IEC_646;
    }

    fn  set_numeric(&self)   {
        self.encoding = State::NUMERIC;
    }

    fn  set_alpha(&self)   {
        self.encoding = State::ALPHA;
    }

    fn  set_iso_iec646(&self)   {
        self.encoding = State::ISO_IEC_646;
    }
}

// NEW FILE: decoded_char.rs
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

// It's not in Alphanumeric neither in ISO/IEC 646 charset
 const FNC1: char = '$';
struct DecodedChar {
    super: DecodedObject;

     let value: char;
}

impl DecodedChar {

    fn new( new_position: i32,  value: char) -> DecodedChar {
        super(new_position);
        let .value = value;
    }

    fn  get_value(&self) -> char  {
        return self.value;
    }

    fn  is_f_n_c1(&self) -> bool  {
        return self.value == FNC1;
    }
}

// NEW FILE: decoded_information.rs
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
struct DecodedInformation {
    super: DecodedObject;

     let new_string: String;

     let remaining_value: i32;

     let mut remaining: bool;
}

impl DecodedInformation {

    fn new( new_position: i32,  new_string: &String) -> DecodedInformation {
        super(new_position);
        let .newString = new_string;
        let .remaining = false;
        let .remainingValue = 0;
    }

    fn new( new_position: i32,  new_string: &String,  remaining_value: i32) -> DecodedInformation {
        super(new_position);
        let .remaining = true;
        let .remainingValue = remaining_value;
        let .newString = new_string;
    }

    fn  get_new_string(&self) -> String  {
        return self.newString;
    }

    fn  is_remaining(&self) -> bool  {
        return self.remaining;
    }

    fn  get_remaining_value(&self) -> i32  {
        return self.remainingValue;
    }
}

// NEW FILE: decoded_numeric.rs
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

 const FNC1: i32 = 10;
struct DecodedNumeric {
    super: DecodedObject;

     let first_digit: i32;

     let second_digit: i32;
}

impl DecodedNumeric {

    fn new( new_position: i32,  first_digit: i32,  second_digit: i32) -> DecodedNumeric throws FormatException {
        super(new_position);
        if first_digit < 0 || first_digit > 10 || second_digit < 0 || second_digit > 10 {
            throw FormatException::get_format_instance();
        }
        let .firstDigit = first_digit;
        let .secondDigit = second_digit;
    }

    fn  get_first_digit(&self) -> i32  {
        return self.firstDigit;
    }

    fn  get_second_digit(&self) -> i32  {
        return self.secondDigit;
    }

    fn  get_value(&self) -> i32  {
        return self.firstDigit * 10 + self.secondDigit;
    }

    fn  is_first_digit_f_n_c1(&self) -> bool  {
        return self.firstDigit == FNC1;
    }

    fn  is_second_digit_f_n_c1(&self) -> bool  {
        return self.secondDigit == FNC1;
    }
}

// NEW FILE: decoded_object.rs
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
 */
struct DecodedObject {

     let new_position: i32;
}

impl DecodedObject {

    fn new( new_position: i32) -> DecodedObject {
        let .newPosition = new_position;
    }

    fn  get_new_position(&self) -> i32  {
        return self.newPosition;
    }
}

// NEW FILE: field_parser.rs
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

 const TWO_DIGIT_DATA_LENGTH: Map<String, DataLength> = HashMap<>::new();

 const THREE_DIGIT_DATA_LENGTH: Map<String, DataLength> = HashMap<>::new();

 const THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH: Map<String, DataLength> = HashMap<>::new();

 const FOUR_DIGIT_DATA_LENGTH: Map<String, DataLength> = HashMap<>::new();
struct FieldParser {
}

impl FieldParser {

    static {
        TWO_DIGIT_DATA_LENGTH::put("00", &DataLength::fixed(18));
        TWO_DIGIT_DATA_LENGTH::put("01", &DataLength::fixed(14));
        TWO_DIGIT_DATA_LENGTH::put("02", &DataLength::fixed(14));
        TWO_DIGIT_DATA_LENGTH::put("10", &DataLength::variable(20));
        TWO_DIGIT_DATA_LENGTH::put("11", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("12", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("13", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("15", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("17", &DataLength::fixed(6));
        TWO_DIGIT_DATA_LENGTH::put("20", &DataLength::fixed(2));
        TWO_DIGIT_DATA_LENGTH::put("21", &DataLength::variable(20));
        TWO_DIGIT_DATA_LENGTH::put("22", &DataLength::variable(29));
        TWO_DIGIT_DATA_LENGTH::put("30", &DataLength::variable(8));
        TWO_DIGIT_DATA_LENGTH::put("37", &DataLength::variable(8));
        //internal company codes
         {
             let mut i: i32 = 90;
            while i <= 99 {
                {
                    TWO_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::variable(30));
                }
                i += 1;
             }
         }

    }

    static {
        THREE_DIGIT_DATA_LENGTH::put("240", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("241", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("242", &DataLength::variable(6));
        THREE_DIGIT_DATA_LENGTH::put("250", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("251", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("253", &DataLength::variable(17));
        THREE_DIGIT_DATA_LENGTH::put("254", &DataLength::variable(20));
        THREE_DIGIT_DATA_LENGTH::put("400", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("401", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("402", &DataLength::fixed(17));
        THREE_DIGIT_DATA_LENGTH::put("403", &DataLength::variable(30));
        THREE_DIGIT_DATA_LENGTH::put("410", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("411", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("412", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("413", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("414", &DataLength::fixed(13));
        THREE_DIGIT_DATA_LENGTH::put("420", &DataLength::variable(20));
        THREE_DIGIT_DATA_LENGTH::put("421", &DataLength::variable(15));
        THREE_DIGIT_DATA_LENGTH::put("422", &DataLength::fixed(3));
        THREE_DIGIT_DATA_LENGTH::put("423", &DataLength::variable(15));
        THREE_DIGIT_DATA_LENGTH::put("424", &DataLength::fixed(3));
        THREE_DIGIT_DATA_LENGTH::put("425", &DataLength::fixed(3));
        THREE_DIGIT_DATA_LENGTH::put("426", &DataLength::fixed(3));
    }

    static {
         {
             let mut i: i32 = 310;
            while i <= 316 {
                {
                    THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::fixed(6));
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 320;
            while i <= 336 {
                {
                    THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::fixed(6));
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 340;
            while i <= 357 {
                {
                    THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::fixed(6));
                }
                i += 1;
             }
         }

         {
             let mut i: i32 = 360;
            while i <= 369 {
                {
                    THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put(&String::value_of(i), &DataLength::fixed(6));
                }
                i += 1;
             }
         }

        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("390", &DataLength::variable(15));
        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("391", &DataLength::variable(18));
        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("392", &DataLength::variable(15));
        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("393", &DataLength::variable(18));
        THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::put("703", &DataLength::variable(30));
    }

    static {
        FOUR_DIGIT_DATA_LENGTH::put("7001", &DataLength::fixed(13));
        FOUR_DIGIT_DATA_LENGTH::put("7002", &DataLength::variable(30));
        FOUR_DIGIT_DATA_LENGTH::put("7003", &DataLength::fixed(10));
        FOUR_DIGIT_DATA_LENGTH::put("8001", &DataLength::fixed(14));
        FOUR_DIGIT_DATA_LENGTH::put("8002", &DataLength::variable(20));
        FOUR_DIGIT_DATA_LENGTH::put("8003", &DataLength::variable(30));
        FOUR_DIGIT_DATA_LENGTH::put("8004", &DataLength::variable(30));
        FOUR_DIGIT_DATA_LENGTH::put("8005", &DataLength::fixed(6));
        FOUR_DIGIT_DATA_LENGTH::put("8006", &DataLength::fixed(18));
        FOUR_DIGIT_DATA_LENGTH::put("8007", &DataLength::variable(30));
        FOUR_DIGIT_DATA_LENGTH::put("8008", &DataLength::variable(12));
        FOUR_DIGIT_DATA_LENGTH::put("8018", &DataLength::fixed(18));
        FOUR_DIGIT_DATA_LENGTH::put("8020", &DataLength::variable(25));
        FOUR_DIGIT_DATA_LENGTH::put("8100", &DataLength::fixed(6));
        FOUR_DIGIT_DATA_LENGTH::put("8101", &DataLength::fixed(10));
        FOUR_DIGIT_DATA_LENGTH::put("8102", &DataLength::fixed(2));
        FOUR_DIGIT_DATA_LENGTH::put("8110", &DataLength::variable(70));
        FOUR_DIGIT_DATA_LENGTH::put("8200", &DataLength::variable(70));
    }

    fn new() -> FieldParser {
    }

    fn  parse_fields_in_general_purpose( raw_information: &String) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
        if raw_information.is_empty() {
            return Ok(null);
        }
        if raw_information.length() < 2 {
            throw NotFoundException::get_not_found_instance();
        }
         let two_digit_data_length: DataLength = TWO_DIGIT_DATA_LENGTH::get(&raw_information.substring(0, 2));
        if two_digit_data_length != null {
            if two_digit_data_length.variable {
                return Ok(::process_variable_a_i(2, two_digit_data_length.len(), &raw_information));
            }
            return Ok(::process_fixed_a_i(2, two_digit_data_length.len(), &raw_information));
        }
        if raw_information.length() < 3 {
            throw NotFoundException::get_not_found_instance();
        }
         let first_three_digits: String = raw_information.substring(0, 3);
         let three_digit_data_length: DataLength = THREE_DIGIT_DATA_LENGTH::get(&first_three_digits);
        if three_digit_data_length != null {
            if three_digit_data_length.variable {
                return Ok(::process_variable_a_i(3, three_digit_data_length.len(), &raw_information));
            }
            return Ok(::process_fixed_a_i(3, three_digit_data_length.len(), &raw_information));
        }
        if raw_information.length() < 4 {
            throw NotFoundException::get_not_found_instance();
        }
         let three_digit_plus_digit_data_length: DataLength = THREE_DIGIT_PLUS_DIGIT_DATA_LENGTH::get(&first_three_digits);
        if three_digit_plus_digit_data_length != null {
            if three_digit_plus_digit_data_length.variable {
                return Ok(::process_variable_a_i(4, three_digit_plus_digit_data_length.len(), &raw_information));
            }
            return Ok(::process_fixed_a_i(4, three_digit_plus_digit_data_length.len(), &raw_information));
        }
         let first_four_digit_length: DataLength = FOUR_DIGIT_DATA_LENGTH::get(&raw_information.substring(0, 4));
        if first_four_digit_length != null {
            if first_four_digit_length.variable {
                return Ok(::process_variable_a_i(4, first_four_digit_length.len(), &raw_information));
            }
            return Ok(::process_fixed_a_i(4, first_four_digit_length.len(), &raw_information));
        }
        throw NotFoundException::get_not_found_instance();
    }

    fn  process_fixed_a_i( ai_size: i32,  field_size: i32,  raw_information: &String) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
        if raw_information.length() < ai_size {
            throw NotFoundException::get_not_found_instance();
        }
         let ai: String = raw_information.substring(0, ai_size);
        if raw_information.length() < ai_size + field_size {
            throw NotFoundException::get_not_found_instance();
        }
         let field: String = raw_information.substring(ai_size, ai_size + field_size);
         let remaining: String = raw_information.substring(ai_size + field_size);
         let result: String = format!("({}){}", ai, field);
         let parsed_a_i: String = ::parse_fields_in_general_purpose(&remaining);
        return Ok( if parsed_a_i == null { result } else { format!("{}{}", result, parsed_a_i) });
    }

    fn  process_variable_a_i( ai_size: i32,  variable_field_size: i32,  raw_information: &String) -> /*  throws NotFoundException */Result<String, Rc<Exception>>   {
         let ai: String = raw_information.substring(0, ai_size);
         let max_size: i32 = Math::min(&raw_information.length(), ai_size + variable_field_size);
         let field: String = raw_information.substring(ai_size, max_size);
         let remaining: String = raw_information.substring(max_size);
         let result: String = format!("({}){}", ai, field);
         let parsed_a_i: String = ::parse_fields_in_general_purpose(&remaining);
        return Ok( if parsed_a_i == null { result } else { format!("{}{}", result, parsed_a_i) });
    }

    struct DataLength {

         let variable: bool;

         let length: i32;
    }
    
    impl DataLength {

        fn new( variable: bool,  length: i32) -> DataLength {
            let .variable = variable;
            let .len() = length;
        }

        fn  fixed( length: i32) -> DataLength  {
            return DataLength::new(false, length);
        }

        fn  variable( length: i32) -> DataLength  {
            return DataLength::new(true, length);
        }
    }

}

// NEW FILE: general_app_id_decoder.rs
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
struct GeneralAppIdDecoder {

     let information: BitArray;

     let current: CurrentParsingState = CurrentParsingState::new();

     let buffer: StringBuilder = StringBuilder::new();
}

impl GeneralAppIdDecoder {

    fn new( information: &BitArray) -> GeneralAppIdDecoder {
        let .information = information;
    }

    fn  decode_all_codes(&self,  buff: &StringBuilder,  initial_position: i32) -> /*  throws NotFoundException, FormatException */Result<String, Rc<Exception>>   {
         let current_position: i32 = initial_position;
         let mut remaining: String = null;
        loop { {
             let info: DecodedInformation = self.decode_general_purpose_field(current_position, &remaining);
             let parsed_fields: String = FieldParser::parse_fields_in_general_purpose(&info.get_new_string());
            if parsed_fields != null {
                buff.append(&parsed_fields);
            }
            if info.is_remaining() {
                remaining = String::value_of(&info.get_remaining_value());
            } else {
                remaining = null;
            }
            if current_position == info.get_new_position() {
                // No step forward!
                break;
            }
            current_position = info.get_new_position();
        }if !(true) break;}
        return Ok(buff.to_string());
    }

    fn  is_still_numeric(&self,  pos: i32) -> bool  {
        // and one of the first 4 bits is "1".
        if pos + 7 > self.information.get_size() {
            return pos + 4 <= self.information.get_size();
        }
         {
             let mut i: i32 = pos;
            while i < pos + 3 {
                {
                    if self.information.get(i) {
                        return true;
                    }
                }
                i += 1;
             }
         }

        return self.information.get(pos + 3);
    }

    fn  decode_numeric(&self,  pos: i32) -> /*  throws FormatException */Result<DecodedNumeric, Rc<Exception>>   {
        if pos + 7 > self.information.get_size() {
             let numeric: i32 = ::extract_numeric_value_from_bit_array(pos, 4);
            if numeric == 0 {
                return Ok(DecodedNumeric::new(&self.information.get_size(), DecodedNumeric::FNC1, DecodedNumeric::FNC1));
            }
            return Ok(DecodedNumeric::new(&self.information.get_size(), numeric - 1, DecodedNumeric::FNC1));
        }
         let numeric: i32 = ::extract_numeric_value_from_bit_array(pos, 7);
         let digit1: i32 = (numeric - 8) / 11;
         let digit2: i32 = (numeric - 8) % 11;
        return Ok(DecodedNumeric::new(pos + 7, digit1, digit2));
    }

    fn  extract_numeric_value_from_bit_array(&self,  pos: i32,  bits: i32) -> i32  {
        return ::extract_numeric_value_from_bit_array(self.information, pos, bits);
    }

    fn  extract_numeric_value_from_bit_array( information: &BitArray,  pos: i32,  bits: i32) -> i32  {
         let mut value: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < bits {
                {
                    if information.get(pos + i) {
                        value |= 1 << (bits - i - 1);
                    }
                }
                i += 1;
             }
         }

        return value;
    }

    fn  decode_general_purpose_field(&self,  pos: i32,  remaining: &String) -> /*  throws FormatException */Result<DecodedInformation, Rc<Exception>>   {
        self.buffer.set_length(0);
        if remaining != null {
            self.buffer.append(&remaining);
        }
        self.current.set_position(pos);
         let last_decoded: DecodedInformation = self.parse_blocks();
        if last_decoded != null && last_decoded.is_remaining() {
            return Ok(DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string(), &last_decoded.get_remaining_value()));
        }
        return Ok(DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string()));
    }

    fn  parse_blocks(&self) -> /*  throws FormatException */Result<DecodedInformation, Rc<Exception>>   {
         let is_finished: bool;
         let mut result: BlockParsedResult;
        loop { {
             let initial_position: i32 = self.current.get_position();
            if self.current.is_alpha() {
                result = self.parse_alpha_block();
                is_finished = result.is_finished();
            } else if self.current.is_iso_iec646() {
                result = self.parse_iso_iec646_block();
                is_finished = result.is_finished();
            } else {
                // it must be numeric
                result = self.parse_numeric_block();
                is_finished = result.is_finished();
            }
             let position_changed: bool = initial_position != self.current.get_position();
            if !position_changed && !is_finished {
                break;
            }
        }if !(!is_finished) break;}
        return Ok(result.get_decoded_information());
    }

    fn  parse_numeric_block(&self) -> /*  throws FormatException */Result<BlockParsedResult, Rc<Exception>>   {
        while self.is_still_numeric(&self.current.get_position()) {
             let numeric: DecodedNumeric = self.decode_numeric(&self.current.get_position());
            self.current.set_position(&numeric.get_new_position());
            if numeric.is_first_digit_f_n_c1() {
                 let mut information: DecodedInformation;
                if numeric.is_second_digit_f_n_c1() {
                    information = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string());
                } else {
                    information = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string(), &numeric.get_second_digit());
                }
                return Ok(BlockParsedResult::new(information, true));
            }
            self.buffer.append(&numeric.get_first_digit());
            if numeric.is_second_digit_f_n_c1() {
                 let information: DecodedInformation = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string());
                return Ok(BlockParsedResult::new(information, true));
            }
            self.buffer.append(&numeric.get_second_digit());
        }
        if self.is_numeric_to_alpha_numeric_latch(&self.current.get_position()) {
            self.current.set_alpha();
            self.current.increment_position(4);
        }
        return Ok(BlockParsedResult::new());
    }

    fn  parse_iso_iec646_block(&self) -> /*  throws FormatException */Result<BlockParsedResult, Rc<Exception>>   {
        while self.is_still_iso_iec646(&self.current.get_position()) {
             let iso: DecodedChar = self.decode_iso_iec646(&self.current.get_position());
            self.current.set_position(&iso.get_new_position());
            if iso.is_f_n_c1() {
                 let information: DecodedInformation = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string());
                return Ok(BlockParsedResult::new(information, true));
            }
            self.buffer.append(&iso.get_value());
        }
        if self.is_alpha_or646_to_numeric_latch(&self.current.get_position()) {
            self.current.increment_position(3);
            self.current.set_numeric();
        } else if self.is_alpha_to646_to_alpha_latch(&self.current.get_position()) {
            if self.current.get_position() + 5 < self.information.get_size() {
                self.current.increment_position(5);
            } else {
                self.current.set_position(&self.information.get_size());
            }
            self.current.set_alpha();
        }
        return Ok(BlockParsedResult::new());
    }

    fn  parse_alpha_block(&self) -> BlockParsedResult  {
        while self.is_still_alpha(&self.current.get_position()) {
             let alpha: DecodedChar = self.decode_alphanumeric(&self.current.get_position());
            self.current.set_position(&alpha.get_new_position());
            if alpha.is_f_n_c1() {
                 let information: DecodedInformation = DecodedInformation::new(&self.current.get_position(), &self.buffer.to_string());
                //end of the char block
                return BlockParsedResult::new(information, true);
            }
            self.buffer.append(&alpha.get_value());
        }
        if self.is_alpha_or646_to_numeric_latch(&self.current.get_position()) {
            self.current.increment_position(3);
            self.current.set_numeric();
        } else if self.is_alpha_to646_to_alpha_latch(&self.current.get_position()) {
            if self.current.get_position() + 5 < self.information.get_size() {
                self.current.increment_position(5);
            } else {
                self.current.set_position(&self.information.get_size());
            }
            self.current.set_iso_iec646();
        }
        return BlockParsedResult::new();
    }

    fn  is_still_iso_iec646(&self,  pos: i32) -> bool  {
        if pos + 5 > self.information.get_size() {
            return false;
        }
         let five_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 5);
        if five_bit_value >= 5 && five_bit_value < 16 {
            return true;
        }
        if pos + 7 > self.information.get_size() {
            return false;
        }
         let seven_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 7);
        if seven_bit_value >= 64 && seven_bit_value < 116 {
            return true;
        }
        if pos + 8 > self.information.get_size() {
            return false;
        }
         let eight_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 8);
        return eight_bit_value >= 232 && eight_bit_value < 253;
    }

    fn  decode_iso_iec646(&self,  pos: i32) -> /*  throws FormatException */Result<DecodedChar, Rc<Exception>>   {
         let five_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 5);
        if five_bit_value == 15 {
            return Ok(DecodedChar::new(pos + 5, DecodedChar::FNC1));
        }
        if five_bit_value >= 5 && five_bit_value < 15 {
            return Ok(DecodedChar::new(pos + 5, ('0' + five_bit_value - 5) as char));
        }
         let seven_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 7);
        if seven_bit_value >= 64 && seven_bit_value < 90 {
            return Ok(DecodedChar::new(pos + 7, (seven_bit_value + 1) as char));
        }
        if seven_bit_value >= 90 && seven_bit_value < 116 {
            return Ok(DecodedChar::new(pos + 7, (seven_bit_value + 7) as char));
        }
         let eight_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 8);
         let mut c: char;
        match eight_bit_value {
              232 => 
                 {
                    c = '!';
                    break;
                }
              233 => 
                 {
                    c = '"';
                    break;
                }
              234 => 
                 {
                    c = '%';
                    break;
                }
              235 => 
                 {
                    c = '&';
                    break;
                }
              236 => 
                 {
                    c = '\'';
                    break;
                }
              237 => 
                 {
                    c = '(';
                    break;
                }
              238 => 
                 {
                    c = ')';
                    break;
                }
              239 => 
                 {
                    c = '*';
                    break;
                }
              240 => 
                 {
                    c = '+';
                    break;
                }
              241 => 
                 {
                    c = ',';
                    break;
                }
              242 => 
                 {
                    c = '-';
                    break;
                }
              243 => 
                 {
                    c = '.';
                    break;
                }
              244 => 
                 {
                    c = '/';
                    break;
                }
              245 => 
                 {
                    c = ':';
                    break;
                }
              246 => 
                 {
                    c = ';';
                    break;
                }
              247 => 
                 {
                    c = '<';
                    break;
                }
              248 => 
                 {
                    c = '=';
                    break;
                }
              249 => 
                 {
                    c = '>';
                    break;
                }
              250 => 
                 {
                    c = '?';
                    break;
                }
              251 => 
                 {
                    c = '_';
                    break;
                }
              252 => 
                 {
                    c = ' ';
                    break;
                }
            _ => 
                 {
                    throw FormatException::get_format_instance();
                }
        }
        return Ok(DecodedChar::new(pos + 8, c));
    }

    fn  is_still_alpha(&self,  pos: i32) -> bool  {
        if pos + 5 > self.information.get_size() {
            return false;
        }
        // We now check if it's a valid 5-bit value (0..9 and FNC1)
         let five_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 5);
        if five_bit_value >= 5 && five_bit_value < 16 {
            return true;
        }
        if pos + 6 > self.information.get_size() {
            return false;
        }
         let six_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 6);
        // 63 not included
        return six_bit_value >= 16 && six_bit_value < 63;
    }

    fn  decode_alphanumeric(&self,  pos: i32) -> DecodedChar  {
         let five_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 5);
        if five_bit_value == 15 {
            return DecodedChar::new(pos + 5, DecodedChar::FNC1);
        }
        if five_bit_value >= 5 && five_bit_value < 15 {
            return DecodedChar::new(pos + 5, ('0' + five_bit_value - 5) as char);
        }
         let six_bit_value: i32 = ::extract_numeric_value_from_bit_array(pos, 6);
        if six_bit_value >= 32 && six_bit_value < 58 {
            return DecodedChar::new(pos + 6, (six_bit_value + 33) as char);
        }
         let mut c: char;
        match six_bit_value {
              58 => 
                 {
                    c = '*';
                    break;
                }
              59 => 
                 {
                    c = ',';
                    break;
                }
              60 => 
                 {
                    c = '-';
                    break;
                }
              61 => 
                 {
                    c = '.';
                    break;
                }
              62 => 
                 {
                    c = '/';
                    break;
                }
            _ => 
                 {
                    throw IllegalStateException::new(format!("Decoding invalid alphanumeric value: {}", six_bit_value));
                }
        }
        return DecodedChar::new(pos + 6, c);
    }

    fn  is_alpha_to646_to_alpha_latch(&self,  pos: i32) -> bool  {
        if pos + 1 > self.information.get_size() {
            return false;
        }
         {
             let mut i: i32 = 0;
            while i < 5 && i + pos < self.information.get_size() {
                {
                    if i == 2 {
                        if !self.information.get(pos + 2) {
                            return false;
                        }
                    } else if self.information.get(pos + i) {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    fn  is_alpha_or646_to_numeric_latch(&self,  pos: i32) -> bool  {
        // Next is alphanumeric if there are 3 positions and they are all zeros
        if pos + 3 > self.information.get_size() {
            return false;
        }
         {
             let mut i: i32 = pos;
            while i < pos + 3 {
                {
                    if self.information.get(i) {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    fn  is_numeric_to_alpha_numeric_latch(&self,  pos: i32) -> bool  {
        // if there is a subset of this just before the end of the symbol
        if pos + 1 > self.information.get_size() {
            return false;
        }
         {
             let mut i: i32 = 0;
            while i < 4 && i + pos < self.information.get_size() {
                {
                    if self.information.get(pos + i) {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }
}

