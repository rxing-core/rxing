/*
 * Copyright 2021 ZXing authors
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
// package com::google::zxing::qrcode::encoder;

/**
 * Encoder that encodes minimally
 *
 * Algorithm:
 *
 * The eleventh commandment was "Thou Shalt Compute" or "Thou Shalt Not Compute" - I forget which (Alan Perilis).
 *
 * This implementation computes. As an alternative, the QR-Code specification suggests heuristics like this one:
 *
 * If initial input data is in the exclusive subset of the Alphanumeric character set AND if there are less than
 * [6,7,8] characters followed by data from the remainder of the 8-bit byte character set, THEN select the 8-
 * bit byte mode ELSE select Alphanumeric mode;
 *
 * This is probably right for 99.99% of cases but there is at least this one counter example: The string "AAAAAAa"
 * encodes 2 bits smaller as ALPHANUMERIC(AAAAAA), BYTE(a) than by encoding it as BYTE(AAAAAAa).
 * Perhaps that is the only counter example but without having proof, it remains unclear.
 *
 * ECI switching:
 *
 * In multi language content the algorithm selects the most compact representation using ECI modes.
 * For example the most compact representation of the string "\u0150\u015C" (O-double-acute, S-circumflex) is
 * ECI(UTF-8), BYTE(\u0150\u015C) while prepending one or more times the same leading character as in
 * "\u0150\u0150\u015C", the most compact representation uses two ECIs so that the string is encoded as
 * ECI(ISO-8859-2), BYTE(\u0150\u0150), ECI(ISO-8859-3), BYTE(\u015C).
 *
 * @author Alex Geller
 */
struct MinimalEncoder {

     let string_to_encode: String;

     let is_g_s1: bool;

     let mut encoders: ECIEncoderSet;

     let ec_level: ErrorCorrectionLevel;
}

impl MinimalEncoder {

    enum VersionSize {

        SMALL("version 1-9"), MEDIUM("version 10-26"), LARGE("version 27-40");

         let description: String;

        fn new( description: &String) -> VersionSize {
            let .description = description;
        }

        pub fn  to_string(&self) -> String  {
            return self.description;
        }
    }

    /**
   * Creates a MinimalEncoder
   *
   * @param stringToEncode The string to encode
   * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
   *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
   *   charset to encode any character in the input that can be encoded by it if the charset is among the
   *   supported charsets.
   * @param isGS1 {@code true} if a FNC1 is to be prepended; {@code false} otherwise
   * @param ecLevel The error correction level.
   * @see ResultList#getVersion
   */
    fn new( string_to_encode: &String,  priority_charset: &Charset,  is_g_s1: bool,  ec_level: &ErrorCorrectionLevel) -> MinimalEncoder {
        let .stringToEncode = string_to_encode;
        let .isGS1 = is_g_s1;
        let .encoders = ECIEncoderSet::new(&string_to_encode, &priority_charset, -1);
        let .ecLevel = ec_level;
    }

    /**
   * Encodes the string minimally
   *
   * @param stringToEncode The string to encode
   * @param version The preferred {@link Version}. A minimal version is computed (see
   *   {@link ResultList#getVersion method} when the value of the argument is null
   * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
   *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
   *   charset to encode any character in the input that can be encoded by it if the charset is among the
   *   supported charsets.
   * @param isGS1 {@code true} if a FNC1 is to be prepended; {@code false} otherwise
   * @param ecLevel The error correction level.
   * @return An instance of {@code ResultList} representing the minimal solution.
   * @see ResultList#getBits
   * @see ResultList#getVersion
   * @see ResultList#getSize
   */
    fn  encode( string_to_encode: &String,  version: &Version,  priority_charset: &Charset,  is_g_s1: bool,  ec_level: &ErrorCorrectionLevel) -> /*  throws WriterException */Result<ResultList, Rc<Exception>>   {
        return Ok(MinimalEncoder::new(&string_to_encode, &priority_charset, is_g_s1, ec_level).encode(version));
    }

    fn  encode(&self,  version: &Version) -> /*  throws WriterException */Result<ResultList, Rc<Exception>>   {
        if version == null {
            // compute minimal encoding trying the three version sizes.
             let versions: vec![Vec<Version>; 3] = vec![::get_version(VersionSize::SMALL), ::get_version(VersionSize::MEDIUM), ::get_version(VersionSize::LARGE), ]
            ;
             let results: vec![Vec<ResultList>; 3] = vec![self.encode_specific_version(versions[0]), self.encode_specific_version(versions[1]), self.encode_specific_version(versions[2]), ]
            ;
             let smallest_size: i32 = Integer::MAX_VALUE;
             let smallest_result: i32 = -1;
             {
                 let mut i: i32 = 0;
                while i < 3 {
                    {
                         let size: i32 = results[i].get_size();
                        if Encoder::will_fit(size, versions[i], self.ec_level) && size < smallest_size {
                            smallest_size = size;
                            smallest_result = i;
                        }
                    }
                    i += 1;
                 }
             }

            if smallest_result < 0 {
                throw WriterException::new("Data too big for any version");
            }
            return Ok(results[smallest_result]);
        } else {
            // compute minimal encoding for a given version
             let result: ResultList = self.encode_specific_version(version);
            if !Encoder::will_fit(&result.get_size(), &::get_version(&::get_version_size(&result.get_version())), self.ec_level) {
                throw WriterException::new(format!("Data too big for version{}", version));
            }
            return Ok(result);
        }
    }

    fn  get_version_size( version: &Version) -> VersionSize  {
        return  if version.get_version_number() <= 9 { VersionSize::SMALL } else {  if version.get_version_number() <= 26 { VersionSize::MEDIUM } else { VersionSize::LARGE } };
    }

    fn  get_version( version_size: &VersionSize) -> Version  {
        match version_size {
              SMALL => 
                 {
                    return Version::get_version_for_number(9);
                }
              MEDIUM => 
                 {
                    return Version::get_version_for_number(26);
                }
              LARGE => 
                 {
                }
            _ => 
                 {
                    return Version::get_version_for_number(40);
                }
        }
    }

    fn  is_numeric( c: char) -> bool  {
        return c >= '0' && c <= '9';
    }

    fn  is_double_byte_kanji( c: char) -> bool  {
        return Encoder::is_only_double_byte_kanji(&String::value_of(c));
    }

    fn  is_alphanumeric( c: char) -> bool  {
        return Encoder::get_alphanumeric_code(c) != -1;
    }

    fn  can_encode(&self,  mode: &Mode,  c: char) -> bool  {
        match mode {
              KANJI => 
                 {
                    return ::is_double_byte_kanji(c);
                }
              ALPHANUMERIC => 
                 {
                    return ::is_alphanumeric(c);
                }
              NUMERIC => 
                 {
                    return ::is_numeric(c);
                }
            // any character can be encoded as byte(s). Up to the caller to manage splitting into
              BYTE => 
                 {
                    return true;
                }
            // multiple bytes when String.getBytes(Charset) return more than one byte.
            _ => 
                 {
                    return false;
                }
        }
    }

    fn  get_compacted_ordinal( mode: &Mode) -> i32  {
        if mode == null {
            return 0;
        }
        match mode {
              KANJI => 
                 {
                    return 0;
                }
              ALPHANUMERIC => 
                 {
                    return 1;
                }
              NUMERIC => 
                 {
                    return 2;
                }
              BYTE => 
                 {
                    return 3;
                }
            _ => 
                 {
                    throw IllegalStateException::new(format!("Illegal mode {}", mode));
                }
        }
    }

    fn  add_edge(&self,  edges: &Vec<Vec<Vec<Edge>>>,  position: i32,  edge: &Edge)   {
         let vertex_index: i32 = position + edge.characterLength;
         let mode_edges: Vec<Edge> = edges[vertex_index][edge.charsetEncoderIndex];
         let mode_ordinal: i32 = ::get_compacted_ordinal(edge.mode);
        if mode_edges[mode_ordinal] == null || mode_edges[mode_ordinal].cachedTotalSize > edge.cachedTotalSize {
            mode_edges[mode_ordinal] = edge;
        }
    }

    fn  add_edges(&self,  version: &Version,  edges: &Vec<Vec<Vec<Edge>>>,  from: i32,  previous: &Edge)   {
         let mut start: i32 = 0;
         let mut end: i32 = self.encoders.length();
         let priority_encoder_index: i32 = self.encoders.get_priority_encoder_index();
        if priority_encoder_index >= 0 && self.encoders.can_encode(&self.string_to_encode.char_at(from), priority_encoder_index) {
            start = priority_encoder_index;
            end = priority_encoder_index + 1;
        }
         {
             let mut i: i32 = start;
            while i < end {
                {
                    if self.encoders.can_encode(&self.string_to_encode.char_at(from), i) {
                        self.add_edge(edges, from, Edge::new(Mode::BYTE, from, i, 1, previous, version));
                    }
                }
                i += 1;
             }
         }

        if self.can_encode(Mode::KANJI, &self.string_to_encode.char_at(from)) {
            self.add_edge(edges, from, Edge::new(Mode::KANJI, from, 0, 1, previous, version));
        }
         let input_length: i32 = self.string_to_encode.length();
        if self.can_encode(Mode::ALPHANUMERIC, &self.string_to_encode.char_at(from)) {
            self.add_edge(edges, from, Edge::new(Mode::ALPHANUMERIC, from, 0,  if from + 1 >= input_length || !self.can_encode(Mode::ALPHANUMERIC, &self.string_to_encode.char_at(from + 1)) { 1 } else { 2 }, previous, version));
        }
        if self.can_encode(Mode::NUMERIC, &self.string_to_encode.char_at(from)) {
            self.add_edge(edges, from, Edge::new(Mode::NUMERIC, from, 0,  if from + 1 >= input_length || !self.can_encode(Mode::NUMERIC, &self.string_to_encode.char_at(from + 1)) { 1 } else {  if from + 2 >= input_length || !self.can_encode(Mode::NUMERIC, &self.string_to_encode.char_at(from + 2)) { 2 } else { 3 } }, previous, version));
        }
    }

    fn  encode_specific_version(&self,  version: &Version) -> /*  throws WriterException */Result<ResultList, Rc<Exception>>   {
         let input_length: i32 = self.string_to_encode.length();
        // Array that represents vertices. There is a vertex for every character, encoding and mode. The vertex contains
        // a list of all edges that lead to it that have the same encoding and mode.
        // The lists are created lazily
        // The last dimension in the array below encodes the 4 modes KANJI, ALPHANUMERIC, NUMERIC and BYTE via the
        // function getCompactedOrdinal(Mode)
         let edges: [[[Option<Edge>; 4]; self.encoders.length()]; input_length + 1] = [[[None; 4]; self.encoders.length()]; input_length + 1];
        self.add_edges(version, edges, 0, null);
         {
             let mut i: i32 = 1;
            while i <= input_length {
                {
                     {
                         let mut j: i32 = 0;
                        while j < self.encoders.length() {
                            {
                                 {
                                     let mut k: i32 = 0;
                                    while k < 4 {
                                        {
                                            if edges[i][j][k] != null && i < input_length {
                                                self.add_edges(version, edges, i, edges[i][j][k]);
                                            }
                                        }
                                        k += 1;
                                     }
                                 }

                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

         let minimal_j: i32 = -1;
         let minimal_k: i32 = -1;
         let minimal_size: i32 = Integer::MAX_VALUE;
         {
             let mut j: i32 = 0;
            while j < self.encoders.length() {
                {
                     {
                         let mut k: i32 = 0;
                        while k < 4 {
                            {
                                if edges[input_length][j][k] != null {
                                     let edge: Edge = edges[input_length][j][k];
                                    if edge.cachedTotalSize < minimal_size {
                                        minimal_size = edge.cachedTotalSize;
                                        minimal_j = j;
                                        minimal_k = k;
                                    }
                                }
                            }
                            k += 1;
                         }
                     }

                }
                j += 1;
             }
         }

        if minimal_j < 0 {
            throw WriterException::new(format!("Internal error: failed to encode \"{}\"", self.string_to_encode));
        }
        return Ok(ResultList::new(version, edges[input_length][minimal_j][minimal_k]));
    }

    struct Edge {

         let mode: Mode;

         let from_position: i32;

         let charset_encoder_index: i32;

         let character_length: i32;

         let previous: Edge;

         let cached_total_size: i32;
    }
    
    impl Edge {

        fn new( mode: &Mode,  from_position: i32,  charset_encoder_index: i32,  character_length: i32,  previous: &Edge,  version: &Version) -> Edge {
            let .mode = mode;
            let .fromPosition = from_position;
            let .charsetEncoderIndex =  if mode == Mode::BYTE || previous == null { charset_encoder_index } else { // inherit the encoding if not of type BYTE
            previous.charsetEncoderIndex };
            let .characterLength = character_length;
            let .previous = previous;
             let mut size: i32 =  if previous != null { previous.cachedTotalSize } else { 0 };
             let need_e_c_i: bool = mode == Mode::BYTE && // at the beginning and charset is not ISO-8859-1
            (previous == null && let .charsetEncoderIndex != 0) || (previous != null && let .charsetEncoderIndex != previous.charsetEncoderIndex);
            if previous == null || mode != previous.mode || need_e_c_i {
                size += 4 + mode.get_character_count_bits(version);
            }
            match mode {
                  KANJI => 
                     {
                        size += 13;
                        break;
                    }
                  ALPHANUMERIC => 
                     {
                        size +=  if character_length == 1 { 6 } else { 11 };
                        break;
                    }
                  NUMERIC => 
                     {
                        size +=  if character_length == 1 { 4 } else {  if character_length == 2 { 7 } else { 10 } };
                        break;
                    }
                  BYTE => 
                     {
                        size += 8 * encoders.encode(&string_to_encode.substring(from_position, from_position + character_length), charset_encoder_index).len();
                        if need_e_c_i {
                            // the ECI assignment numbers for ISO-8859-x, UTF-8 and UTF-16 are all 8 bit long
                            size += 4 + 8;
                        }
                        break;
                    }
            }
            cached_total_size = size;
        }
    }


    struct ResultList {

         let list: List<ResultList.ResultNode> = ArrayList<>::new();

         let version: Version;
    }
    
    impl ResultList {

        fn new( version: &Version,  solution: &Edge) -> ResultList {
             let mut length: i32 = 0;
             let mut current: Edge = solution;
             let contains_e_c_i: bool = false;
            while current != null {
                length += current.characterLength;
                 let previous: Edge = current.previous;
                 let need_e_c_i: bool = current.mode == Mode::BYTE && // at the beginning and charset is not ISO-8859-1
                (previous == null && current.charsetEncoderIndex != 0) || (previous != null && current.charsetEncoderIndex != previous.charsetEncoderIndex);
                if need_e_c_i {
                    contains_e_c_i = true;
                }
                if previous == null || previous.mode != current.mode || need_e_c_i {
                    list.add(0, ResultNode::new(current.mode, current.fromPosition, current.charsetEncoderIndex, length));
                    length = 0;
                }
                if need_e_c_i {
                    list.add(0, ResultNode::new(Mode::ECI, current.fromPosition, current.charsetEncoderIndex, 0));
                }
                current = previous;
            }
            // If there is no ECI at the beginning then we put an ECI to the default charset (ISO-8859-1)
            if is_g_s1 {
                 let mut first: ResultNode = list.get(0);
                if first != null && first.mode != Mode::ECI && contains_e_c_i {
                    // prepend a default character set ECI
                    list.add(0, ResultNode::new(Mode::ECI, 0, 0, 0));
                }
                first = list.get(0);
                // prepend or insert a FNC1_FIRST_POSITION after the ECI (if any)
                list.add( if first.mode != Mode::ECI { 0 } else { 1 }, ResultNode::new(Mode::FNC1_FIRST_POSITION, 0, 0, 0));
            }
            // set version to smallest version into which the bits fit.
             let version_number: i32 = version.get_version_number();
             let lower_limit: i32;
             let upper_limit: i32;
            match ::get_version_size(version) {
                  SMALL => 
                     {
                        lower_limit = 1;
                        upper_limit = 9;
                        break;
                    }
                  MEDIUM => 
                     {
                        lower_limit = 10;
                        upper_limit = 26;
                        break;
                    }
                  LARGE => 
                     {
                    }
                _ => 
                     {
                        lower_limit = 27;
                        upper_limit = 40;
                        break;
                    }
            }
             let size: i32 = self.get_size(version);
            // increase version if needed
            while version_number < upper_limit && !Encoder::will_fit(size, &Version::get_version_for_number(version_number), ec_level) {
                version_number += 1;
            }
            // shrink version if possible
            while version_number > lower_limit && Encoder::will_fit(size, &Version::get_version_for_number(version_number - 1), ec_level) {
                version_number -= 1;
            }
            let .version = Version::get_version_for_number(version_number);
        }

        /**
     * returns the size in bits
     */
        fn  get_size(&self) -> i32  {
            return self.get_size(self.version);
        }

        fn  get_size(&self,  version: &Version) -> i32  {
             let mut result: i32 = 0;
            for  let result_node: ResultNode in self.list {
                result += result_node.get_size(version);
            }
            return result;
        }

        /**
     * appends the bits
     */
        fn  get_bits(&self,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
            for  let result_node: ResultNode in self.list {
                result_node.get_bits(bits);
            }
        }

        fn  get_version(&self) -> Version  {
            return self.version;
        }

        pub fn  to_string(&self) -> String  {
             let result: StringBuilder = StringBuilder::new();
             let mut previous: ResultNode = null;
            for  let current: ResultNode in self.list {
                if previous != null {
                    result.append(",");
                }
                result.append(&current.to_string());
                previous = current;
            }
            return result.to_string();
        }

        struct ResultNode {

             let mode: Mode;

             let from_position: i32;

             let charset_encoder_index: i32;

             let character_length: i32;
        }
        
        impl ResultNode {

            fn new( mode: &Mode,  from_position: i32,  charset_encoder_index: i32,  character_length: i32) -> ResultNode {
                let .mode = mode;
                let .fromPosition = from_position;
                let .charsetEncoderIndex = charset_encoder_index;
                let .characterLength = character_length;
            }

            /**
       * returns the size in bits
       */
            fn  get_size(&self,  version: &Version) -> i32  {
                 let mut size: i32 = 4 + self.mode.get_character_count_bits(version);
                match self.mode {
                      KANJI => 
                         {
                            size += 13 * self.character_length;
                            break;
                        }
                      ALPHANUMERIC => 
                         {
                            size += (self.character_length / 2) * 11;
                            size +=  if (self.character_length % 2) == 1 { 6 } else { 0 };
                            break;
                        }
                      NUMERIC => 
                         {
                            size += (self.character_length / 3) * 10;
                             let rest: i32 = self.character_length % 3;
                            size +=  if rest == 1 { 4 } else {  if rest == 2 { 7 } else { 0 } };
                            break;
                        }
                      BYTE => 
                         {
                            size += 8 * self.get_character_count_indicator();
                            break;
                        }
                      ECI => 
                         {
                            // the ECI assignment numbers for ISO-8859-x, UTF-8 and UTF-16 are all 8 bit long
                            size += 8;
                        }
                }
                return size;
            }

            /**
       * returns the length in characters according to the specification (differs from getCharacterLength() in BYTE mode
       * for multi byte encoded characters)
       */
            fn  get_character_count_indicator(&self) -> i32  {
                return  if self.mode == Mode::BYTE { self.encoders.encode(&self.string_to_encode.substring(self.from_position, self.from_position + self.character_length), self.charset_encoder_index).len() } else { self.character_length };
            }

            /**
       * appends the bits
       */
            fn  get_bits(&self,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
                bits.append_bits(&self.mode.get_bits(), 4);
                if self.character_length > 0 {
                     let length: i32 = self.get_character_count_indicator();
                    bits.append_bits(length, &self.mode.get_character_count_bits(self.version));
                }
                if self.mode == Mode::ECI {
                    bits.append_bits(&self.encoders.get_e_c_i_value(self.charset_encoder_index), 8);
                } else if self.character_length > 0 {
                    // append data
                    Encoder::append_bytes(&self.string_to_encode.substring(self.from_position, self.from_position + self.character_length), self.mode, bits, &self.encoders.get_charset(self.charset_encoder_index));
                }
            }

            pub fn  to_string(&self) -> String  {
                 let result: StringBuilder = StringBuilder::new();
                result.append(self.mode).append('(');
                if self.mode == Mode::ECI {
                    result.append(&self.encoders.get_charset(self.charset_encoder_index).display_name());
                } else {
                    result.append(&self.make_printable(&self.string_to_encode.substring(self.from_position, self.from_position + self.character_length)));
                }
                result.append(')');
                return result.to_string();
            }

            fn  make_printable(&self,  s: &String) -> String  {
                 let result: StringBuilder = StringBuilder::new();
                 {
                     let mut i: i32 = 0;
                    while i < s.length() {
                        {
                            if s.char_at(i) < 32 || s.char_at(i) > 126 {
                                result.append('.');
                            } else {
                                result.append(&s.char_at(i));
                            }
                        }
                        i += 1;
                     }
                 }

                return result.to_string();
            }
        }

    }

}

