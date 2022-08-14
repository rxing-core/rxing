// NEW FILE: block_pair.rs
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
// package com::google::zxing::qrcode::encoder;

struct BlockPair {

     let data_bytes: Vec<i8>;

     let error_correction_bytes: Vec<i8>;
}

impl BlockPair {

    fn new( data: &Vec<i8>,  error_correction: &Vec<i8>) -> BlockPair {
        data_bytes = data;
        error_correction_bytes = error_correction;
    }

    pub fn  get_data_bytes(&self) -> Vec<i8>  {
        return self.data_bytes;
    }

    pub fn  get_error_correction_bytes(&self) -> Vec<i8>  {
        return self.error_correction_bytes;
    }
}

// NEW FILE: byte_matrix.rs
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
// package com::google::zxing::qrcode::encoder;

/**
 * JAVAPORT: The original code was a 2D array of ints, but since it only ever gets assigned
 * -1, 0, and 1, I'm going to use less memory and go with bytes.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct ByteMatrix {

     let mut bytes: Vec<Vec<i8>>;

     let width: i32;

     let height: i32;
}

impl ByteMatrix {

    pub fn new( width: i32,  height: i32) -> ByteMatrix {
        bytes = : [[i8; width]; height] = [[0; width]; height];
        let .width = width;
        let .height = height;
    }

    pub fn  get_height(&self) -> i32  {
        return self.height;
    }

    pub fn  get_width(&self) -> i32  {
        return self.width;
    }

    pub fn  get(&self,  x: i32,  y: i32) -> i8  {
        return self.bytes[y][x];
    }

    /**
   * @return an internal representation as bytes, in row-major order. array[y][x] represents point (x,y)
   */
    pub fn  get_array(&self) -> Vec<Vec<i8>>  {
        return self.bytes;
    }

    pub fn  set(&self,  x: i32,  y: i32,  value: i8)   {
        self.bytes[y][x] = value;
    }

    pub fn  set(&self,  x: i32,  y: i32,  value: i32)   {
        self.bytes[y][x] = value as i8;
    }

    pub fn  set(&self,  x: i32,  y: i32,  value: bool)   {
        self.bytes[y][x] = ( if value { 1 } else { 0 }) as i8;
    }

    pub fn  clear(&self,  value: i8)   {
        for  let a_byte: Vec<i8> in self.bytes {
            Arrays::fill(&a_byte, value);
        }
    }

    pub fn  to_string(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(2 * self.width * self.height + 2);
         {
             let mut y: i32 = 0;
            while y < self.height {
                {
                     let bytes_y: Vec<i8> = self.bytes[y];
                     {
                         let mut x: i32 = 0;
                        while x < self.width {
                            {
                                match bytes_y[x] {
                                      0 => 
                                         {
                                            result.append(" 0");
                                            break;
                                        }
                                      1 => 
                                         {
                                            result.append(" 1");
                                            break;
                                        }
                                    _ => 
                                         {
                                            result.append("  ");
                                            break;
                                        }
                                }
                            }
                            x += 1;
                         }
                     }

                    result.append('\n');
                }
                y += 1;
             }
         }

        return result.to_string();
    }
}

// NEW FILE: encoder.rs
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
// package com::google::zxing::qrcode::encoder;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */

// The original table is defined in the table 5 of JISX0510:2004 (p.19).
 const ALPHANUMERIC_TABLE: vec![Vec<i32>; 96] = vec![// 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x00-0x0f
// 0x00-0x0f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x10-0x1f
// 0x10-0x1f
-1, // 0x20-0x2f
36, // 0x20-0x2f
// 0x20-0x2f
-1, // 0x20-0x2f
// 0x20-0x2f
-1, // 0x20-0x2f
// 0x20-0x2f
-1, // 0x20-0x2f
37, // 0x20-0x2f
38, // 0x20-0x2f
// 0x20-0x2f
-1, // 0x20-0x2f
// 0x20-0x2f
-1, // 0x20-0x2f
// 0x20-0x2f
-1, // 0x20-0x2f
// 0x20-0x2f
-1, // 0x20-0x2f
39, // 0x20-0x2f
40, // 0x20-0x2f
// 0x20-0x2f
-1, // 0x20-0x2f
41, // 0x20-0x2f
42, // 0x20-0x2f
43, // 0x30-0x3f
0, // 0x30-0x3f
1, // 0x30-0x3f
2, // 0x30-0x3f
3, // 0x30-0x3f
4, // 0x30-0x3f
5, // 0x30-0x3f
6, // 0x30-0x3f
7, // 0x30-0x3f
8, // 0x30-0x3f
9, // 0x30-0x3f
44, // 0x30-0x3f
// 0x30-0x3f
-1, // 0x30-0x3f
// 0x30-0x3f
-1, // 0x30-0x3f
// 0x30-0x3f
-1, // 0x30-0x3f
// 0x30-0x3f
-1, // 0x30-0x3f
// 0x30-0x3f
-1, // 0x40-0x4f
// 0x40-0x4f
-1, // 0x40-0x4f
10, // 0x40-0x4f
11, // 0x40-0x4f
12, // 0x40-0x4f
13, // 0x40-0x4f
14, // 0x40-0x4f
15, // 0x40-0x4f
16, // 0x40-0x4f
17, // 0x40-0x4f
18, // 0x40-0x4f
19, // 0x40-0x4f
20, // 0x40-0x4f
21, // 0x40-0x4f
22, // 0x40-0x4f
23, // 0x40-0x4f
24, // 0x50-0x5f
25, // 0x50-0x5f
26, // 0x50-0x5f
27, // 0x50-0x5f
28, // 0x50-0x5f
29, // 0x50-0x5f
30, // 0x50-0x5f
31, // 0x50-0x5f
32, // 0x50-0x5f
33, // 0x50-0x5f
34, // 0x50-0x5f
35, // 0x50-0x5f
// 0x50-0x5f
-1, // 0x50-0x5f
// 0x50-0x5f
-1, // 0x50-0x5f
// 0x50-0x5f
-1, // 0x50-0x5f
// 0x50-0x5f
-1, // 0x50-0x5f
// 0x50-0x5f
-1, ]
;

 const DEFAULT_BYTE_MODE_ENCODING: Charset = StandardCharsets::ISO_8859_1;
pub struct Encoder {
}

impl Encoder {

    fn new() -> Encoder {
    }

    // The mask penalty calculation is complicated.  See Table 21 of JISX0510:2004 (p.45) for details.
    // Basically it applies four rules and summate all penalties.
    fn  calculate_mask_penalty( matrix: &ByteMatrix) -> i32  {
        return MaskUtil::apply_mask_penalty_rule1(matrix) + MaskUtil::apply_mask_penalty_rule2(matrix) + MaskUtil::apply_mask_penalty_rule3(matrix) + MaskUtil::apply_mask_penalty_rule4(matrix);
    }

    /**
   * @param content text to encode
   * @param ecLevel error correction level to use
   * @return {@link QRCode} representing the encoded QR code
   * @throws WriterException if encoding can't succeed, because of for example invalid content
   *   or configuration
   */
    pub fn  encode( content: &String,  ec_level: &ErrorCorrectionLevel) -> /*  throws WriterException */Result<QRCode, Rc<Exception>>   {
        return Ok(::encode(&content, ec_level, null));
    }

    pub fn  encode( content: &String,  ec_level: &ErrorCorrectionLevel,  hints: &Map<EncodeHintType, ?>) -> /*  throws WriterException */Result<QRCode, Rc<Exception>>   {
         let mut version: Version;
         let header_and_data_bits: BitArray;
         let mut mode: Mode;
         let has_g_s1_format_hint: bool = hints != null && hints.contains_key(EncodeHintType::GS1_FORMAT) && Boolean::parse_boolean(&hints.get(EncodeHintType::GS1_FORMAT).to_string());
         let has_compaction_hint: bool = hints != null && hints.contains_key(EncodeHintType::QR_COMPACT) && Boolean::parse_boolean(&hints.get(EncodeHintType::QR_COMPACT).to_string());
        // Determine what character encoding has been specified by the caller, if any
         let mut encoding: Charset = DEFAULT_BYTE_MODE_ENCODING;
         let has_encoding_hint: bool = hints != null && hints.contains_key(EncodeHintType::CHARACTER_SET);
        if has_encoding_hint {
            encoding = Charset::for_name(&hints.get(EncodeHintType::CHARACTER_SET).to_string());
        }
        if has_compaction_hint {
            mode = Mode::BYTE;
             let priority_encoding: Charset =  if encoding.equals(&DEFAULT_BYTE_MODE_ENCODING) { null } else { encoding };
             let rn: MinimalEncoder.ResultList = MinimalEncoder::encode(&content, null, &priority_encoding, has_g_s1_format_hint, ec_level);
            header_and_data_bits = BitArray::new();
            rn.get_bits(header_and_data_bits);
            version = rn.get_version();
        } else {
            // Pick an encoding mode appropriate for the content. Note that this will not attempt to use
            // multiple modes / segments even if that were more efficient.
            mode = ::choose_mode(&content, &encoding);
            // This will store the header information, like mode and
            // length, as well as "header" segments like an ECI segment.
             let header_bits: BitArray = BitArray::new();
            // Append ECI segment if applicable
            if mode == Mode::BYTE && has_encoding_hint {
                 let eci: CharacterSetECI = CharacterSetECI::get_character_set_e_c_i(&encoding);
                if eci != null {
                    ::append_e_c_i(eci, header_bits);
                }
            }
            // Append the FNC1 mode header for GS1 formatted data if applicable
            if has_g_s1_format_hint {
                // GS1 formatted codes are prefixed with a FNC1 in first position mode header
                ::append_mode_info(Mode::FNC1_FIRST_POSITION, header_bits);
            }
            // (With ECI in place,) Write the mode marker
            ::append_mode_info(mode, header_bits);
            // Collect data within the main segment, separately, to count its size if needed. Don't add it to
            // main payload yet.
             let data_bits: BitArray = BitArray::new();
            ::append_bytes(&content, mode, data_bits, &encoding);
            if hints != null && hints.contains_key(EncodeHintType::QR_VERSION) {
                 let version_number: i32 = Integer::parse_int(&hints.get(EncodeHintType::QR_VERSION).to_string());
                version = Version::get_version_for_number(version_number);
                 let bits_needed: i32 = ::calculate_bits_needed(mode, header_bits, data_bits, version);
                if !::will_fit(bits_needed, version, ec_level) {
                    throw WriterException::new("Data too big for requested version");
                }
            } else {
                version = ::recommend_version(ec_level, mode, header_bits, data_bits);
            }
            header_and_data_bits = BitArray::new();
            header_and_data_bits.append_bit_array(header_bits);
            // Find "length" of main segment and write it
             let num_letters: i32 =  if mode == Mode::BYTE { data_bits.get_size_in_bytes() } else { content.length() };
            ::append_length_info(num_letters, version, mode, header_and_data_bits);
            // Put data together into the overall payload
            header_and_data_bits.append_bit_array(data_bits);
        }
         let ec_blocks: Version.ECBlocks = version.get_e_c_blocks_for_level(ec_level);
         let num_data_bytes: i32 = version.get_total_codewords() - ec_blocks.get_total_e_c_codewords();
        // Terminate the bits properly.
        ::terminate_bits(num_data_bytes, header_and_data_bits);
        // Interleave data bits with error correction code.
         let final_bits: BitArray = ::interleave_with_e_c_bytes(header_and_data_bits, &version.get_total_codewords(), num_data_bytes, &ec_blocks.get_num_blocks());
         let qr_code: QRCode = QRCode::new();
        qr_code.set_e_c_level(ec_level);
        qr_code.set_mode(mode);
        qr_code.set_version(version);
        //  Choose the mask pattern and set to "qrCode".
         let dimension: i32 = version.get_dimension_for_version();
         let matrix: ByteMatrix = ByteMatrix::new(dimension, dimension);
        // Enable manual selection of the pattern to be used via hint
         let mask_pattern: i32 = -1;
        if hints != null && hints.contains_key(EncodeHintType::QR_MASK_PATTERN) {
             let hint_mask_pattern: i32 = Integer::parse_int(&hints.get(EncodeHintType::QR_MASK_PATTERN).to_string());
            mask_pattern =  if QRCode::is_valid_mask_pattern(hint_mask_pattern) { hint_mask_pattern } else { -1 };
        }
        if mask_pattern == -1 {
            mask_pattern = ::choose_mask_pattern(final_bits, ec_level, version, matrix);
        }
        qr_code.set_mask_pattern(mask_pattern);
        // Build the matrix and set it to "qrCode".
        MatrixUtil::build_matrix(final_bits, ec_level, version, mask_pattern, matrix);
        qr_code.set_matrix(matrix);
        return Ok(qr_code);
    }

    /**
   * Decides the smallest version of QR code that will contain all of the provided data.
   *
   * @throws WriterException if the data cannot fit in any version
   */
    fn  recommend_version( ec_level: &ErrorCorrectionLevel,  mode: &Mode,  header_bits: &BitArray,  data_bits: &BitArray) -> /*  throws WriterException */Result<Version, Rc<Exception>>   {
        // Hard part: need to know version to know how many bits length takes. But need to know how many
        // bits it takes to know version. First we take a guess at version by assuming version will be
        // the minimum, 1:
         let provisional_bits_needed: i32 = ::calculate_bits_needed(mode, header_bits, data_bits, &Version::get_version_for_number(1));
         let provisional_version: Version = ::choose_version(provisional_bits_needed, ec_level);
        // Use that guess to calculate the right version. I am still not sure this works in 100% of cases.
         let bits_needed: i32 = ::calculate_bits_needed(mode, header_bits, data_bits, provisional_version);
        return Ok(::choose_version(bits_needed, ec_level));
    }

    fn  calculate_bits_needed( mode: &Mode,  header_bits: &BitArray,  data_bits: &BitArray,  version: &Version) -> i32  {
        return header_bits.get_size() + mode.get_character_count_bits(version) + data_bits.get_size();
    }

    /**
   * @return the code point of the table used in alphanumeric mode or
   *  -1 if there is no corresponding code in the table.
   */
    fn  get_alphanumeric_code( code: i32) -> i32  {
        if code < ALPHANUMERIC_TABLE.len() {
            return ALPHANUMERIC_TABLE[code];
        }
        return -1;
    }

    pub fn  choose_mode( content: &String) -> Mode  {
        return ::choose_mode(&content, null);
    }

    /**
   * Choose the best mode by examining the content. Note that 'encoding' is used as a hint;
   * if it is Shift_JIS, and the input is only double-byte Kanji, then we return {@link Mode#KANJI}.
   */
    fn  choose_mode( content: &String,  encoding: &Charset) -> Mode  {
        if StringUtils::SHIFT_JIS_CHARSET::equals(&encoding) && ::is_only_double_byte_kanji(&content) {
            // Choose Kanji mode if all input are double-byte characters
            return Mode::KANJI;
        }
         let has_numeric: bool = false;
         let has_alphanumeric: bool = false;
         {
             let mut i: i32 = 0;
            while i < content.length() {
                {
                     let c: char = content.char_at(i);
                    if c >= '0' && c <= '9' {
                        has_numeric = true;
                    } else if ::get_alphanumeric_code(c) != -1 {
                        has_alphanumeric = true;
                    } else {
                        return Mode::BYTE;
                    }
                }
                i += 1;
             }
         }

        if has_alphanumeric {
            return Mode::ALPHANUMERIC;
        }
        if has_numeric {
            return Mode::NUMERIC;
        }
        return Mode::BYTE;
    }

    fn  is_only_double_byte_kanji( content: &String) -> bool  {
         let bytes: Vec<i8> = content.get_bytes(StringUtils::SHIFT_JIS_CHARSET);
         let length: i32 = bytes.len();
        if length % 2 != 0 {
            return false;
        }
         {
             let mut i: i32 = 0;
            while i < length {
                {
                     let byte1: i32 = bytes[i] & 0xFF;
                    if (byte1 < 0x81 || byte1 > 0x9F) && (byte1 < 0xE0 || byte1 > 0xEB) {
                        return false;
                    }
                }
                i += 2;
             }
         }

        return true;
    }

    fn  choose_mask_pattern( bits: &BitArray,  ec_level: &ErrorCorrectionLevel,  version: &Version,  matrix: &ByteMatrix) -> /*  throws WriterException */Result<i32, Rc<Exception>>   {
        // Lower penalty is better.
         let min_penalty: i32 = Integer::MAX_VALUE;
         let best_mask_pattern: i32 = -1;
        // We try all mask patterns to choose the best one.
         {
             let mask_pattern: i32 = 0;
            while mask_pattern < QRCode.NUM_MASK_PATTERNS {
                {
                    MatrixUtil::build_matrix(bits, ec_level, version, mask_pattern, matrix);
                     let penalty: i32 = ::calculate_mask_penalty(matrix);
                    if penalty < min_penalty {
                        min_penalty = penalty;
                        best_mask_pattern = mask_pattern;
                    }
                }
                mask_pattern += 1;
             }
         }

        return Ok(best_mask_pattern);
    }

    fn  choose_version( num_input_bits: i32,  ec_level: &ErrorCorrectionLevel) -> /*  throws WriterException */Result<Version, Rc<Exception>>   {
         {
             let version_num: i32 = 1;
            while version_num <= 40 {
                {
                     let version: Version = Version::get_version_for_number(version_num);
                    if ::will_fit(num_input_bits, version, ec_level) {
                        return Ok(version);
                    }
                }
                version_num += 1;
             }
         }

        throw WriterException::new("Data too big");
    }

    /**
   * @return true if the number of input bits will fit in a code with the specified version and
   * error correction level.
   */
    fn  will_fit( num_input_bits: i32,  version: &Version,  ec_level: &ErrorCorrectionLevel) -> bool  {
        // In the following comments, we use numbers of Version 7-H.
        // numBytes = 196
         let num_bytes: i32 = version.get_total_codewords();
        // getNumECBytes = 130
         let ec_blocks: Version.ECBlocks = version.get_e_c_blocks_for_level(ec_level);
         let num_ec_bytes: i32 = ec_blocks.get_total_e_c_codewords();
        // getNumDataBytes = 196 - 130 = 66
         let num_data_bytes: i32 = num_bytes - num_ec_bytes;
         let total_input_bytes: i32 = (num_input_bits + 7) / 8;
        return num_data_bytes >= total_input_bytes;
    }

    /**
   * Terminate bits as described in 8.4.8 and 8.4.9 of JISX0510:2004 (p.24).
   */
    fn  terminate_bits( num_data_bytes: i32,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let capacity: i32 = num_data_bytes * 8;
        if bits.get_size() > capacity {
            throw WriterException::new(format!("data bits cannot fit in the QR Code{} > {}", bits.get_size(), capacity));
        }
        // Append Mode.TERMINATE if there is enough space (value is 0000)
         {
             let mut i: i32 = 0;
            while i < 4 && bits.get_size() < capacity {
                {
                    bits.append_bit(false);
                }
                i += 1;
             }
         }

        // Append termination bits. See 8.4.8 of JISX0510:2004 (p.24) for details.
        // If the last byte isn't 8-bit aligned, we'll add padding bits.
         let num_bits_in_last_byte: i32 = bits.get_size() & 0x07;
        if num_bits_in_last_byte > 0 {
             {
                 let mut i: i32 = num_bits_in_last_byte;
                while i < 8 {
                    {
                        bits.append_bit(false);
                    }
                    i += 1;
                 }
             }

        }
        // If we have more space, we'll fill the space with padding patterns defined in 8.4.9 (p.24).
         let num_padding_bytes: i32 = num_data_bytes - bits.get_size_in_bytes();
         {
             let mut i: i32 = 0;
            while i < num_padding_bytes {
                {
                    bits.append_bits( if (i & 0x01) == 0 { 0xEC } else { 0x11 }, 8);
                }
                i += 1;
             }
         }

        if bits.get_size() != capacity {
            throw WriterException::new("Bits size does not equal capacity");
        }
    }

    /**
   * Get number of data bytes and number of error correction bytes for block id "blockID". Store
   * the result in "numDataBytesInBlock", and "numECBytesInBlock". See table 12 in 8.5.1 of
   * JISX0510:2004 (p.30)
   */
    fn  get_num_data_bytes_and_num_e_c_bytes_for_block_i_d( num_total_bytes: i32,  num_data_bytes: i32,  num_r_s_blocks: i32,  block_i_d: i32,  num_data_bytes_in_block: &Vec<i32>,  num_e_c_bytes_in_block: &Vec<i32>)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        if block_i_d >= num_r_s_blocks {
            throw WriterException::new("Block ID too large");
        }
        // numRsBlocksInGroup2 = 196 % 5 = 1
         let num_rs_blocks_in_group2: i32 = num_total_bytes % num_r_s_blocks;
        // numRsBlocksInGroup1 = 5 - 1 = 4
         let num_rs_blocks_in_group1: i32 = num_r_s_blocks - num_rs_blocks_in_group2;
        // numTotalBytesInGroup1 = 196 / 5 = 39
         let num_total_bytes_in_group1: i32 = num_total_bytes / num_r_s_blocks;
        // numTotalBytesInGroup2 = 39 + 1 = 40
         let num_total_bytes_in_group2: i32 = num_total_bytes_in_group1 + 1;
        // numDataBytesInGroup1 = 66 / 5 = 13
         let num_data_bytes_in_group1: i32 = num_data_bytes / num_r_s_blocks;
        // numDataBytesInGroup2 = 13 + 1 = 14
         let num_data_bytes_in_group2: i32 = num_data_bytes_in_group1 + 1;
        // numEcBytesInGroup1 = 39 - 13 = 26
         let num_ec_bytes_in_group1: i32 = num_total_bytes_in_group1 - num_data_bytes_in_group1;
        // numEcBytesInGroup2 = 40 - 14 = 26
         let num_ec_bytes_in_group2: i32 = num_total_bytes_in_group2 - num_data_bytes_in_group2;
        // 26 = 26
        if num_ec_bytes_in_group1 != num_ec_bytes_in_group2 {
            throw WriterException::new("EC bytes mismatch");
        }
        // 5 = 4 + 1.
        if num_r_s_blocks != num_rs_blocks_in_group1 + num_rs_blocks_in_group2 {
            throw WriterException::new("RS blocks mismatch");
        }
        // 196 = (13 + 26) * 4 + (14 + 26) * 1
        if num_total_bytes != ((num_data_bytes_in_group1 + num_ec_bytes_in_group1) * num_rs_blocks_in_group1) + ((num_data_bytes_in_group2 + num_ec_bytes_in_group2) * num_rs_blocks_in_group2) {
            throw WriterException::new("Total bytes mismatch");
        }
        if block_i_d < num_rs_blocks_in_group1 {
            num_data_bytes_in_block[0] = num_data_bytes_in_group1;
            num_e_c_bytes_in_block[0] = num_ec_bytes_in_group1;
        } else {
            num_data_bytes_in_block[0] = num_data_bytes_in_group2;
            num_e_c_bytes_in_block[0] = num_ec_bytes_in_group2;
        }
    }

    /**
   * Interleave "bits" with corresponding error correction bytes. On success, store the result in
   * "result". The interleave rule is complicated. See 8.6 of JISX0510:2004 (p.37) for details.
   */
    fn  interleave_with_e_c_bytes( bits: &BitArray,  num_total_bytes: i32,  num_data_bytes: i32,  num_r_s_blocks: i32) -> /*  throws WriterException */Result<BitArray, Rc<Exception>>   {
        // "bits" must have "getNumDataBytes" bytes of data.
        if bits.get_size_in_bytes() != num_data_bytes {
            throw WriterException::new("Number of bits and data bytes does not match");
        }
        // Step 1.  Divide data bytes into blocks and generate error correction bytes for them. We'll
        // store the divided data bytes blocks and error correction bytes blocks into "blocks".
         let data_bytes_offset: i32 = 0;
         let max_num_data_bytes: i32 = 0;
         let max_num_ec_bytes: i32 = 0;
        // Since, we know the number of reedsolmon blocks, we can initialize the vector with the number.
         let blocks: Collection<BlockPair> = ArrayList<>::new(num_r_s_blocks);
         {
             let mut i: i32 = 0;
            while i < num_r_s_blocks {
                {
                     let num_data_bytes_in_block: [i32; 1] = [0; 1];
                     let num_ec_bytes_in_block: [i32; 1] = [0; 1];
                    ::get_num_data_bytes_and_num_e_c_bytes_for_block_i_d(num_total_bytes, num_data_bytes, num_r_s_blocks, i, &num_data_bytes_in_block, &num_ec_bytes_in_block);
                     let size: i32 = num_data_bytes_in_block[0];
                     let data_bytes: [i8; size] = [0; size];
                    bits.to_bytes(8 * data_bytes_offset, &data_bytes, 0, size);
                     let ec_bytes: Vec<i8> = ::generate_e_c_bytes(&data_bytes, num_ec_bytes_in_block[0]);
                    blocks.add(BlockPair::new(&data_bytes, &ec_bytes));
                    max_num_data_bytes = Math::max(max_num_data_bytes, size);
                    max_num_ec_bytes = Math::max(max_num_ec_bytes, ec_bytes.len());
                    data_bytes_offset += num_data_bytes_in_block[0];
                }
                i += 1;
             }
         }

        if num_data_bytes != data_bytes_offset {
            throw WriterException::new("Data bytes does not match offset");
        }
         let result: BitArray = BitArray::new();
        // First, place data blocks.
         {
             let mut i: i32 = 0;
            while i < max_num_data_bytes {
                {
                    for  let block: BlockPair in blocks {
                         let data_bytes: Vec<i8> = block.get_data_bytes();
                        if i < data_bytes.len() {
                            result.append_bits(data_bytes[i], 8);
                        }
                    }
                }
                i += 1;
             }
         }

        // Then, place error correction blocks.
         {
             let mut i: i32 = 0;
            while i < max_num_ec_bytes {
                {
                    for  let block: BlockPair in blocks {
                         let ec_bytes: Vec<i8> = block.get_error_correction_bytes();
                        if i < ec_bytes.len() {
                            result.append_bits(ec_bytes[i], 8);
                        }
                    }
                }
                i += 1;
             }
         }

        if num_total_bytes != result.get_size_in_bytes() {
            // Should be same.
            throw WriterException::new(format!("Interleaving error: {} and {} differ.", num_total_bytes, result.get_size_in_bytes()));
        }
        return Ok(result);
    }

    fn  generate_e_c_bytes( data_bytes: &Vec<i8>,  num_ec_bytes_in_block: i32) -> Vec<i8>  {
         let num_data_bytes: i32 = data_bytes.len();
         let to_encode: [i32; num_data_bytes + num_ec_bytes_in_block] = [0; num_data_bytes + num_ec_bytes_in_block];
         {
             let mut i: i32 = 0;
            while i < num_data_bytes {
                {
                    to_encode[i] = data_bytes[i] & 0xFF;
                }
                i += 1;
             }
         }

        ReedSolomonEncoder::new(GenericGF::QR_CODE_FIELD_256).encode(&to_encode, num_ec_bytes_in_block);
         let ec_bytes: [i8; num_ec_bytes_in_block] = [0; num_ec_bytes_in_block];
         {
             let mut i: i32 = 0;
            while i < num_ec_bytes_in_block {
                {
                    ec_bytes[i] = to_encode[num_data_bytes + i] as i8;
                }
                i += 1;
             }
         }

        return ec_bytes;
    }

    /**
   * Append mode info. On success, store the result in "bits".
   */
    fn  append_mode_info( mode: &Mode,  bits: &BitArray)   {
        bits.append_bits(&mode.get_bits(), 4);
    }

    /**
   * Append length info. On success, store the result in "bits".
   */
    fn  append_length_info( num_letters: i32,  version: &Version,  mode: &Mode,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let num_bits: i32 = mode.get_character_count_bits(version);
        if num_letters >= (1 << num_bits) {
            throw WriterException::new(format!("{} is bigger than {}", num_letters, ((1 << num_bits) - 1)));
        }
        bits.append_bits(num_letters, num_bits);
    }

    /**
   * Append "bytes" in "mode" mode (encoding) into "bits". On success, store the result in "bits".
   */
    fn  append_bytes( content: &String,  mode: &Mode,  bits: &BitArray,  encoding: &Charset)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        match mode {
              NUMERIC => 
                 {
                    ::append_numeric_bytes(&content, bits);
                    break;
                }
              ALPHANUMERIC => 
                 {
                    ::append_alphanumeric_bytes(&content, bits);
                    break;
                }
              BYTE => 
                 {
                    ::append8_bit_bytes(&content, bits, &encoding);
                    break;
                }
              KANJI => 
                 {
                    ::append_kanji_bytes(&content, bits);
                    break;
                }
            _ => 
                 {
                    throw WriterException::new(format!("Invalid mode: {}", mode));
                }
        }
    }

    fn  append_numeric_bytes( content: &CharSequence,  bits: &BitArray)   {
         let length: i32 = content.length();
         let mut i: i32 = 0;
        while i < length {
             let num1: i32 = content.char_at(i) - '0';
            if i + 2 < length {
                // Encode three numeric letters in ten bits.
                 let num2: i32 = content.char_at(i + 1) - '0';
                 let num3: i32 = content.char_at(i + 2) - '0';
                bits.append_bits(num1 * 100 + num2 * 10 + num3, 10);
                i += 3;
            } else if i + 1 < length {
                // Encode two numeric letters in seven bits.
                 let num2: i32 = content.char_at(i + 1) - '0';
                bits.append_bits(num1 * 10 + num2, 7);
                i += 2;
            } else {
                // Encode one numeric letter in four bits.
                bits.append_bits(num1, 4);
                i += 1;
            }
        }
    }

    fn  append_alphanumeric_bytes( content: &CharSequence,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let length: i32 = content.length();
         let mut i: i32 = 0;
        while i < length {
             let code1: i32 = ::get_alphanumeric_code(&content.char_at(i));
            if code1 == -1 {
                throw WriterException::new();
            }
            if i + 1 < length {
                 let code2: i32 = ::get_alphanumeric_code(&content.char_at(i + 1));
                if code2 == -1 {
                    throw WriterException::new();
                }
                // Encode two alphanumeric letters in 11 bits.
                bits.append_bits(code1 * 45 + code2, 11);
                i += 2;
            } else {
                // Encode one alphanumeric letter in six bits.
                bits.append_bits(code1, 6);
                i += 1;
            }
        }
    }

    fn  append8_bit_bytes( content: &String,  bits: &BitArray,  encoding: &Charset)   {
         let bytes: Vec<i8> = content.get_bytes(&encoding);
        for  let b: i8 in bytes {
            bits.append_bits(b, 8);
        }
    }

    fn  append_kanji_bytes( content: &String,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let bytes: Vec<i8> = content.get_bytes(StringUtils::SHIFT_JIS_CHARSET);
        if bytes.len() % 2 != 0 {
            throw WriterException::new("Kanji byte size not even");
        }
        // bytes.length must be even
         let max_i: i32 = bytes.len() - 1;
         {
             let mut i: i32 = 0;
            while i < max_i {
                {
                     let byte1: i32 = bytes[i] & 0xFF;
                     let byte2: i32 = bytes[i + 1] & 0xFF;
                     let code: i32 = (byte1 << 8) | byte2;
                     let mut subtracted: i32 = -1;
                    if code >= 0x8140 && code <= 0x9ffc {
                        subtracted = code - 0x8140;
                    } else if code >= 0xe040 && code <= 0xebbf {
                        subtracted = code - 0xc140;
                    }
                    if subtracted == -1 {
                        throw WriterException::new("Invalid byte sequence");
                    }
                     let encoded: i32 = ((subtracted >> 8) * 0xc0) + (subtracted & 0xff);
                    bits.append_bits(encoded, 13);
                }
                i += 2;
             }
         }

    }

    fn  append_e_c_i( eci: &CharacterSetECI,  bits: &BitArray)   {
        bits.append_bits(&Mode::ECI::get_bits(), 4);
        // This is correct for values up to 127, which is all we need now.
        bits.append_bits(&eci.get_value(), 8);
    }
}

// NEW FILE: mask_util.rs
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
// package com::google::zxing::qrcode::encoder;

/**
 * @author Satoru Takabayashi
 * @author Daniel Switkin
 * @author Sean Owen
 */

// Penalty weights from section 6.8.2.1
 const N1: i32 = 3;

 const N2: i32 = 3;

 const N3: i32 = 40;

 const N4: i32 = 10;
struct MaskUtil {
}

impl MaskUtil {

    fn new() -> MaskUtil {
    // do nothing
    }

    /**
   * Apply mask penalty rule 1 and return the penalty. Find repetitive cells with the same color and
   * give penalty to them. Example: 00000 or 11111.
   */
    fn  apply_mask_penalty_rule1( matrix: &ByteMatrix) -> i32  {
        return ::apply_mask_penalty_rule1_internal(matrix, true) + ::apply_mask_penalty_rule1_internal(matrix, false);
    }

    /**
   * Apply mask penalty rule 2 and return the penalty. Find 2x2 blocks with the same color and give
   * penalty to them. This is actually equivalent to the spec's rule, which is to find MxN blocks and give a
   * penalty proportional to (M-1)x(N-1), because this is the number of 2x2 blocks inside such a block.
   */
    fn  apply_mask_penalty_rule2( matrix: &ByteMatrix) -> i32  {
         let mut penalty: i32 = 0;
         let array: Vec<Vec<i8>> = matrix.get_array();
         let width: i32 = matrix.get_width();
         let height: i32 = matrix.get_height();
         {
             let mut y: i32 = 0;
            while y < height - 1 {
                {
                     let array_y: Vec<i8> = array[y];
                     {
                         let mut x: i32 = 0;
                        while x < width - 1 {
                            {
                                 let value: i32 = array_y[x];
                                if value == array_y[x + 1] && value == array[y + 1][x] && value == array[y + 1][x + 1] {
                                    penalty += 1;
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        return N2 * penalty;
    }

    /**
   * Apply mask penalty rule 3 and return the penalty. Find consecutive runs of 1:1:3:1:1:4
   * starting with black, or 4:1:1:3:1:1 starting with white, and give penalty to them.  If we
   * find patterns like 000010111010000, we give penalty once.
   */
    fn  apply_mask_penalty_rule3( matrix: &ByteMatrix) -> i32  {
         let num_penalties: i32 = 0;
         let array: Vec<Vec<i8>> = matrix.get_array();
         let width: i32 = matrix.get_width();
         let height: i32 = matrix.get_height();
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     {
                         let mut x: i32 = 0;
                        while x < width {
                            {
                                // We can at least optimize this access
                                 let array_y: Vec<i8> = array[y];
                                if x + 6 < width && array_y[x] == 1 && array_y[x + 1] == 0 && array_y[x + 2] == 1 && array_y[x + 3] == 1 && array_y[x + 4] == 1 && array_y[x + 5] == 0 && array_y[x + 6] == 1 && (::is_white_horizontal(&array_y, x - 4, x) || ::is_white_horizontal(&array_y, x + 7, x + 11)) {
                                    num_penalties += 1;
                                }
                                if y + 6 < height && array[y][x] == 1 && array[y + 1][x] == 0 && array[y + 2][x] == 1 && array[y + 3][x] == 1 && array[y + 4][x] == 1 && array[y + 5][x] == 0 && array[y + 6][x] == 1 && (::is_white_vertical(&array, x, y - 4, y) || ::is_white_vertical(&array, x, y + 7, y + 11)) {
                                    num_penalties += 1;
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        return num_penalties * N3;
    }

    fn  is_white_horizontal( row_array: &Vec<i8>,  from: i32,  to: i32) -> bool  {
        if from < 0 || row_array.len() < to {
            return false;
        }
         {
             let mut i: i32 = from;
            while i < to {
                {
                    if row_array[i] == 1 {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    fn  is_white_vertical( array: &Vec<Vec<i8>>,  col: i32,  from: i32,  to: i32) -> bool  {
        if from < 0 || array.len() < to {
            return false;
        }
         {
             let mut i: i32 = from;
            while i < to {
                {
                    if array[i][col] == 1 {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
    }

    /**
   * Apply mask penalty rule 4 and return the penalty. Calculate the ratio of dark cells and give
   * penalty if the ratio is far from 50%. It gives 10 penalty for 5% distance.
   */
    fn  apply_mask_penalty_rule4( matrix: &ByteMatrix) -> i32  {
         let num_dark_cells: i32 = 0;
         let array: Vec<Vec<i8>> = matrix.get_array();
         let width: i32 = matrix.get_width();
         let height: i32 = matrix.get_height();
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let array_y: Vec<i8> = array[y];
                     {
                         let mut x: i32 = 0;
                        while x < width {
                            {
                                if array_y[x] == 1 {
                                    num_dark_cells += 1;
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

         let num_total_cells: i32 = matrix.get_height() * matrix.get_width();
         let five_percent_variances: i32 = Math::abs(num_dark_cells * 2 - num_total_cells) * 10 / num_total_cells;
        return five_percent_variances * N4;
    }

    /**
   * Return the mask bit for "getMaskPattern" at "x" and "y". See 8.8 of JISX0510:2004 for mask
   * pattern conditions.
   */
    fn  get_data_mask_bit( mask_pattern: i32,  x: i32,  y: i32) -> bool  {
         let mut intermediate: i32;
         let mut temp: i32;
        match mask_pattern {
              0 => 
                 {
                    intermediate = (y + x) & 0x1;
                    break;
                }
              1 => 
                 {
                    intermediate = y & 0x1;
                    break;
                }
              2 => 
                 {
                    intermediate = x % 3;
                    break;
                }
              3 => 
                 {
                    intermediate = (y + x) % 3;
                    break;
                }
              4 => 
                 {
                    intermediate = ((y / 2) + (x / 3)) & 0x1;
                    break;
                }
              5 => 
                 {
                    temp = y * x;
                    intermediate = (temp & 0x1) + (temp % 3);
                    break;
                }
              6 => 
                 {
                    temp = y * x;
                    intermediate = ((temp & 0x1) + (temp % 3)) & 0x1;
                    break;
                }
              7 => 
                 {
                    temp = y * x;
                    intermediate = ((temp % 3) + ((y + x) & 0x1)) & 0x1;
                    break;
                }
            _ => 
                 {
                    throw IllegalArgumentException::new(format!("Invalid mask pattern: {}", mask_pattern));
                }
        }
        return intermediate == 0;
    }

    /**
   * Helper function for applyMaskPenaltyRule1. We need this for doing this calculation in both
   * vertical and horizontal orders respectively.
   */
    fn  apply_mask_penalty_rule1_internal( matrix: &ByteMatrix,  is_horizontal: bool) -> i32  {
         let mut penalty: i32 = 0;
         let i_limit: i32 =  if is_horizontal { matrix.get_height() } else { matrix.get_width() };
         let j_limit: i32 =  if is_horizontal { matrix.get_width() } else { matrix.get_height() };
         let array: Vec<Vec<i8>> = matrix.get_array();
         {
             let mut i: i32 = 0;
            while i < i_limit {
                {
                     let num_same_bit_cells: i32 = 0;
                     let prev_bit: i32 = -1;
                     {
                         let mut j: i32 = 0;
                        while j < j_limit {
                            {
                                 let bit: i32 =  if is_horizontal { array[i][j] } else { array[j][i] };
                                if bit == prev_bit {
                                    num_same_bit_cells += 1;
                                } else {
                                    if num_same_bit_cells >= 5 {
                                        penalty += N1 + (num_same_bit_cells - 5);
                                    }
                                    // Include the cell itself.
                                    num_same_bit_cells = 1;
                                    prev_bit = bit;
                                }
                            }
                            j += 1;
                         }
                     }

                    if num_same_bit_cells >= 5 {
                        penalty += N1 + (num_same_bit_cells - 5);
                    }
                }
                i += 1;
             }
         }

        return penalty;
    }
}

// NEW FILE: matrix_util.rs
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
// package com::google::zxing::qrcode::encoder;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */

 const POSITION_DETECTION_PATTERN: vec![vec![Vec<Vec<i32>>; 7]; 7] = vec![vec![1, 1, 1, 1, 1, 1, 1, ]
, vec![1, 0, 0, 0, 0, 0, 1, ]
, vec![1, 0, 1, 1, 1, 0, 1, ]
, vec![1, 0, 1, 1, 1, 0, 1, ]
, vec![1, 0, 1, 1, 1, 0, 1, ]
, vec![1, 0, 0, 0, 0, 0, 1, ]
, vec![1, 1, 1, 1, 1, 1, 1, ]
, ]
;

 const POSITION_ADJUSTMENT_PATTERN: vec![vec![Vec<Vec<i32>>; 5]; 5] = vec![vec![1, 1, 1, 1, 1, ]
, vec![1, 0, 0, 0, 1, ]
, vec![1, 0, 1, 0, 1, ]
, vec![1, 0, 0, 0, 1, ]
, vec![1, 1, 1, 1, 1, ]
, ]
;

// From Appendix E. Table 1, JIS0510X:2004 (p 71). The table was double-checked by komatsu.
 const POSITION_ADJUSTMENT_PATTERN_COORDINATE_TABLE: vec![vec![Vec<Vec<i32>>; 7]; 40] = vec![// Version 1
vec![-1, -1, -1, -1, -1, -1, -1, ]
, // Version 2
vec![6, 18, -1, -1, -1, -1, -1, ]
, // Version 3
vec![6, 22, -1, -1, -1, -1, -1, ]
, // Version 4
vec![6, 26, -1, -1, -1, -1, -1, ]
, // Version 5
vec![6, 30, -1, -1, -1, -1, -1, ]
, // Version 6
vec![6, 34, -1, -1, -1, -1, -1, ]
, // Version 7
vec![6, 22, 38, -1, -1, -1, -1, ]
, // Version 8
vec![6, 24, 42, -1, -1, -1, -1, ]
, // Version 9
vec![6, 26, 46, -1, -1, -1, -1, ]
, // Version 10
vec![6, 28, 50, -1, -1, -1, -1, ]
, // Version 11
vec![6, 30, 54, -1, -1, -1, -1, ]
, // Version 12
vec![6, 32, 58, -1, -1, -1, -1, ]
, // Version 13
vec![6, 34, 62, -1, -1, -1, -1, ]
, // Version 14
vec![6, 26, 46, 66, -1, -1, -1, ]
, // Version 15
vec![6, 26, 48, 70, -1, -1, -1, ]
, // Version 16
vec![6, 26, 50, 74, -1, -1, -1, ]
, // Version 17
vec![6, 30, 54, 78, -1, -1, -1, ]
, // Version 18
vec![6, 30, 56, 82, -1, -1, -1, ]
, // Version 19
vec![6, 30, 58, 86, -1, -1, -1, ]
, // Version 20
vec![6, 34, 62, 90, -1, -1, -1, ]
, // Version 21
vec![6, 28, 50, 72, 94, -1, -1, ]
, // Version 22
vec![6, 26, 50, 74, 98, -1, -1, ]
, // Version 23
vec![6, 30, 54, 78, 102, -1, -1, ]
, // Version 24
vec![6, 28, 54, 80, 106, -1, -1, ]
, // Version 25
vec![6, 32, 58, 84, 110, -1, -1, ]
, // Version 26
vec![6, 30, 58, 86, 114, -1, -1, ]
, // Version 27
vec![6, 34, 62, 90, 118, -1, -1, ]
, // Version 28
vec![6, 26, 50, 74, 98, 122, -1, ]
, // Version 29
vec![6, 30, 54, 78, 102, 126, -1, ]
, // Version 30
vec![6, 26, 52, 78, 104, 130, -1, ]
, // Version 31
vec![6, 30, 56, 82, 108, 134, -1, ]
, // Version 32
vec![6, 34, 60, 86, 112, 138, -1, ]
, // Version 33
vec![6, 30, 58, 86, 114, 142, -1, ]
, // Version 34
vec![6, 34, 62, 90, 118, 146, -1, ]
, // Version 35
vec![6, 30, 54, 78, 102, 126, 150, ]
, // Version 36
vec![6, 24, 50, 76, 102, 128, 154, ]
, // Version 37
vec![6, 28, 54, 80, 106, 132, 158, ]
, // Version 38
vec![6, 32, 58, 84, 110, 136, 162, ]
, // Version 39
vec![6, 26, 54, 82, 110, 138, 166, ]
, // Version 40
vec![6, 30, 58, 86, 114, 142, 170, ]
, ]
;

// Type info cells at the left top corner.
 const TYPE_INFO_COORDINATES: vec![vec![Vec<Vec<i32>>; 2]; 15] = vec![vec![8, 0, ]
, vec![8, 1, ]
, vec![8, 2, ]
, vec![8, 3, ]
, vec![8, 4, ]
, vec![8, 5, ]
, vec![8, 7, ]
, vec![8, 8, ]
, vec![7, 8, ]
, vec![5, 8, ]
, vec![4, 8, ]
, vec![3, 8, ]
, vec![2, 8, ]
, vec![1, 8, ]
, vec![0, 8, ]
, ]
;

// From Appendix D in JISX0510:2004 (p. 67)
// 1 1111 0010 0101
 const VERSION_INFO_POLY: i32 = 0x1f25;

// From Appendix C in JISX0510:2004 (p.65).
 const TYPE_INFO_POLY: i32 = 0x537;

 const TYPE_INFO_MASK_PATTERN: i32 = 0x5412;
struct MatrixUtil {
}

impl MatrixUtil {

    fn new() -> MatrixUtil {
    // do nothing
    }

    // Set all cells to -1.  -1 means that the cell is empty (not set yet).
    //
    // JAVAPORT: We shouldn't need to do this at all. The code should be rewritten to begin encoding
    // with the ByteMatrix initialized all to zero.
    fn  clear_matrix( matrix: &ByteMatrix)   {
        matrix.clear(-1 as i8);
    }

    // Build 2D matrix of QR Code from "dataBits" with "ecLevel", "version" and "getMaskPattern". On
    // success, store the result in "matrix" and return true.
    fn  build_matrix( data_bits: &BitArray,  ec_level: &ErrorCorrectionLevel,  version: &Version,  mask_pattern: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        ::clear_matrix(matrix);
        ::embed_basic_patterns(version, matrix);
        // Type information appear with any version.
        ::embed_type_info(ec_level, mask_pattern, matrix);
        // Version info appear if version >= 7.
        ::maybe_embed_version_info(version, matrix);
        // Data should be embedded at end.
        ::embed_data_bits(data_bits, mask_pattern, matrix);
    }

    // Embed basic patterns. On success, modify the matrix and return true.
    // The basic patterns are:
    // - Position detection patterns
    // - Timing patterns
    // - Dark dot at the left bottom corner
    // - Position adjustment patterns, if need be
    fn  embed_basic_patterns( version: &Version,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        // Let's get started with embedding big squares at corners.
        ::embed_position_detection_patterns_and_separators(matrix);
        // Then, embed the dark dot at the left bottom corner.
        ::embed_dark_dot_at_left_bottom_corner(matrix);
        // Position adjustment patterns appear if version >= 2.
        ::maybe_embed_position_adjustment_patterns(version, matrix);
        // Timing patterns should be embedded after position adj. patterns.
        ::embed_timing_patterns(matrix);
    }

    // Embed type information. On success, modify the matrix.
    fn  embed_type_info( ec_level: &ErrorCorrectionLevel,  mask_pattern: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let type_info_bits: BitArray = BitArray::new();
        ::make_type_info_bits(ec_level, mask_pattern, type_info_bits);
         {
             let mut i: i32 = 0;
            while i < type_info_bits.get_size() {
                {
                    // Place bits in LSB to MSB order.  LSB (least significant bit) is the last value in
                    // "typeInfoBits".
                     let bit: bool = type_info_bits.get(type_info_bits.get_size() - 1 - i);
                    // Type info bits at the left top corner. See 8.9 of JISX0510:2004 (p.46).
                     let coordinates: Vec<i32> = TYPE_INFO_COORDINATES[i];
                     let x1: i32 = coordinates[0];
                     let y1: i32 = coordinates[1];
                    matrix.set(x1, y1, bit);
                     let mut x2: i32;
                     let mut y2: i32;
                    if i < 8 {
                        // Right top corner.
                        x2 = matrix.get_width() - i - 1;
                        y2 = 8;
                    } else {
                        // Left bottom corner.
                        x2 = 8;
                        y2 = matrix.get_height() - 7 + (i - 8);
                    }
                    matrix.set(x2, y2, bit);
                }
                i += 1;
             }
         }

    }

    // Embed version information if need be. On success, modify the matrix and return true.
    // See 8.10 of JISX0510:2004 (p.47) for how to embed version information.
    fn  maybe_embed_version_info( version: &Version,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        if version.get_version_number() < 7 {
            // Don't need version info.
            return;
        }
         let version_info_bits: BitArray = BitArray::new();
        ::make_version_info_bits(version, version_info_bits);
        // It will decrease from 17 to 0.
         let bit_index: i32 = 6 * 3 - 1;
         {
             let mut i: i32 = 0;
            while i < 6 {
                {
                     {
                         let mut j: i32 = 0;
                        while j < 3 {
                            {
                                // Place bits in LSB (least significant bit) to MSB order.
                                 let bit: bool = version_info_bits.get(bit_index);
                                bit_index -= 1;
                                // Left bottom corner.
                                matrix.set(i, matrix.get_height() - 11 + j, bit);
                                // Right bottom corner.
                                matrix.set(matrix.get_height() - 11 + j, i, bit);
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

    }

    // Embed "dataBits" using "getMaskPattern". On success, modify the matrix and return true.
    // For debugging purposes, it skips masking process if "getMaskPattern" is -1.
    // See 8.7 of JISX0510:2004 (p.38) for how to embed data bits.
    fn  embed_data_bits( data_bits: &BitArray,  mask_pattern: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let bit_index: i32 = 0;
         let mut direction: i32 = -1;
        // Start from the right bottom cell.
         let mut x: i32 = matrix.get_width() - 1;
         let mut y: i32 = matrix.get_height() - 1;
        while x > 0 {
            // Skip the vertical timing pattern.
            if x == 6 {
                x -= 1;
            }
            while y >= 0 && y < matrix.get_height() {
                 {
                     let mut i: i32 = 0;
                    while i < 2 {
                        {
                             let xx: i32 = x - i;
                            // Skip the cell if it's not empty.
                            if !::is_empty(&matrix.get(xx, y)) {
                                continue;
                            }
                             let mut bit: bool;
                            if bit_index < data_bits.get_size() {
                                bit = data_bits.get(bit_index);
                                bit_index += 1;
                            } else {
                                // Padding bit. If there is no bit left, we'll fill the left cells with 0, as described
                                // in 8.4.9 of JISX0510:2004 (p. 24).
                                bit = false;
                            }
                            // Skip masking if mask_pattern is -1.
                            if mask_pattern != -1 && MaskUtil::get_data_mask_bit(mask_pattern, xx, y) {
                                bit = !bit;
                            }
                            matrix.set(xx, y, bit);
                        }
                        i += 1;
                     }
                 }

                y += direction;
            }
            // Reverse the direction.
            direction = -direction;
            y += direction;
            // Move to the left.
            x -= 2;
        }
        // All bits should be consumed.
        if bit_index != data_bits.get_size() {
            throw WriterException::new(format!("Not all bits consumed: {}/{}", bit_index, data_bits.get_size()));
        }
    }

    // Return the position of the most significant bit set (to one) in the "value". The most
    // significant bit is position 32. If there is no bit set, return 0. Examples:
    // - findMSBSet(0) => 0
    // - findMSBSet(1) => 1
    // - findMSBSet(255) => 8
    fn  find_m_s_b_set( value: i32) -> i32  {
        return 32 - Integer::number_of_leading_zeros(value);
    }

    // Calculate BCH (Bose-Chaudhuri-Hocquenghem) code for "value" using polynomial "poly". The BCH
    // code is used for encoding type information and version information.
    // Example: Calculation of version information of 7.
    // f(x) is created from 7.
    //   - 7 = 000111 in 6 bits
    //   - f(x) = x^2 + x^1 + x^0
    // g(x) is given by the standard (p. 67)
    //   - g(x) = x^12 + x^11 + x^10 + x^9 + x^8 + x^5 + x^2 + 1
    // Multiply f(x) by x^(18 - 6)
    //   - f'(x) = f(x) * x^(18 - 6)
    //   - f'(x) = x^14 + x^13 + x^12
    // Calculate the remainder of f'(x) / g(x)
    //         x^2
    //         __________________________________________________
    //   g(x) )x^14 + x^13 + x^12
    //         x^14 + x^13 + x^12 + x^11 + x^10 + x^7 + x^4 + x^2
    //         --------------------------------------------------
    //                              x^11 + x^10 + x^7 + x^4 + x^2
    //
    // The remainder is x^11 + x^10 + x^7 + x^4 + x^2
    // Encode it in binary: 110010010100
    // The return value is 0xc94 (1100 1001 0100)
    //
    // Since all coefficients in the polynomials are 1 or 0, we can do the calculation by bit
    // operations. We don't care if coefficients are positive or negative.
    fn  calculate_b_c_h_code( value: i32,  poly: i32) -> i32  {
        if poly == 0 {
            throw IllegalArgumentException::new("0 polynomial");
        }
        // If poly is "1 1111 0010 0101" (version info poly), msbSetInPoly is 13. We'll subtract 1
        // from 13 to make it 12.
         let msb_set_in_poly: i32 = ::find_m_s_b_set(poly);
        value <<= msb_set_in_poly - 1;
        // Do the division business using exclusive-or operations.
        while ::find_m_s_b_set(value) >= msb_set_in_poly {
            value ^= poly << (::find_m_s_b_set(value) - msb_set_in_poly);
        }
        // Now the "value" is the remainder (i.e. the BCH code)
        return value;
    }

    // Make bit vector of type information. On success, store the result in "bits" and return true.
    // Encode error correction level and mask pattern. See 8.9 of
    // JISX0510:2004 (p.45) for details.
    fn  make_type_info_bits( ec_level: &ErrorCorrectionLevel,  mask_pattern: i32,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        if !QRCode::is_valid_mask_pattern(mask_pattern) {
            throw WriterException::new("Invalid mask pattern");
        }
         let type_info: i32 = (ec_level.get_bits() << 3) | mask_pattern;
        bits.append_bits(type_info, 5);
         let bch_code: i32 = ::calculate_b_c_h_code(type_info, TYPE_INFO_POLY);
        bits.append_bits(bch_code, 10);
         let mask_bits: BitArray = BitArray::new();
        mask_bits.append_bits(TYPE_INFO_MASK_PATTERN, 15);
        bits.xor(mask_bits);
        if bits.get_size() != 15 {
            // Just in case.
            throw WriterException::new(format!("should not happen but we got: {}", bits.get_size()));
        }
    }

    // Make bit vector of version information. On success, store the result in "bits" and return true.
    // See 8.10 of JISX0510:2004 (p.45) for details.
    fn  make_version_info_bits( version: &Version,  bits: &BitArray)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        bits.append_bits(&version.get_version_number(), 6);
         let bch_code: i32 = ::calculate_b_c_h_code(&version.get_version_number(), VERSION_INFO_POLY);
        bits.append_bits(bch_code, 12);
        if bits.get_size() != 18 {
            // Just in case.
            throw WriterException::new(format!("should not happen but we got: {}", bits.get_size()));
        }
    }

    // Check if "value" is empty.
    fn  is_empty( value: i32) -> bool  {
        return value == -1;
    }

    fn  embed_timing_patterns( matrix: &ByteMatrix)   {
        // separation patterns (size 1). Thus, 8 = 7 + 1.
         {
             let mut i: i32 = 8;
            while i < matrix.get_width() - 8 {
                {
                     let bit: i32 = (i + 1) % 2;
                    // Horizontal line.
                    if ::is_empty(&matrix.get(i, 6)) {
                        matrix.set(i, 6, bit);
                    }
                    // Vertical line.
                    if ::is_empty(&matrix.get(6, i)) {
                        matrix.set(6, i, bit);
                    }
                }
                i += 1;
             }
         }

    }

    // Embed the lonely dark dot at left bottom corner. JISX0510:2004 (p.46)
    fn  embed_dark_dot_at_left_bottom_corner( matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        if matrix.get(8, matrix.get_height() - 8) == 0 {
            throw WriterException::new();
        }
        matrix.set(8, matrix.get_height() - 8, 1);
    }

    fn  embed_horizontal_separation_pattern( x_start: i32,  y_start: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         {
             let mut x: i32 = 0;
            while x < 8 {
                {
                    if !::is_empty(&matrix.get(x_start + x, y_start)) {
                        throw WriterException::new();
                    }
                    matrix.set(x_start + x, y_start, 0);
                }
                x += 1;
             }
         }

    }

    fn  embed_vertical_separation_pattern( x_start: i32,  y_start: i32,  matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         {
             let mut y: i32 = 0;
            while y < 7 {
                {
                    if !::is_empty(&matrix.get(x_start, y_start + y)) {
                        throw WriterException::new();
                    }
                    matrix.set(x_start, y_start + y, 0);
                }
                y += 1;
             }
         }

    }

    fn  embed_position_adjustment_pattern( x_start: i32,  y_start: i32,  matrix: &ByteMatrix)   {
         {
             let mut y: i32 = 0;
            while y < 5 {
                {
                     let pattern_y: Vec<i32> = POSITION_ADJUSTMENT_PATTERN[y];
                     {
                         let mut x: i32 = 0;
                        while x < 5 {
                            {
                                matrix.set(x_start + x, y_start + y, pattern_y[x]);
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

    }

    fn  embed_position_detection_pattern( x_start: i32,  y_start: i32,  matrix: &ByteMatrix)   {
         {
             let mut y: i32 = 0;
            while y < 7 {
                {
                     let pattern_y: Vec<i32> = POSITION_DETECTION_PATTERN[y];
                     {
                         let mut x: i32 = 0;
                        while x < 7 {
                            {
                                matrix.set(x_start + x, y_start + y, pattern_y[x]);
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

    }

    // Embed position detection patterns and surrounding vertical/horizontal separators.
    fn  embed_position_detection_patterns_and_separators( matrix: &ByteMatrix)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        // Embed three big squares at corners.
         let pdp_width: i32 = POSITION_DETECTION_PATTERN[0].len();
        // Left top corner.
        ::embed_position_detection_pattern(0, 0, matrix);
        // Right top corner.
        ::embed_position_detection_pattern(matrix.get_width() - pdp_width, 0, matrix);
        // Left bottom corner.
        ::embed_position_detection_pattern(0, matrix.get_width() - pdp_width, matrix);
        // Embed horizontal separation patterns around the squares.
         let hsp_width: i32 = 8;
        // Left top corner.
        ::embed_horizontal_separation_pattern(0, hsp_width - 1, matrix);
        // Right top corner.
        ::embed_horizontal_separation_pattern(matrix.get_width() - hsp_width, hsp_width - 1, matrix);
        // Left bottom corner.
        ::embed_horizontal_separation_pattern(0, matrix.get_width() - hsp_width, matrix);
        // Embed vertical separation patterns around the squares.
         let vsp_size: i32 = 7;
        // Left top corner.
        ::embed_vertical_separation_pattern(vsp_size, 0, matrix);
        // Right top corner.
        ::embed_vertical_separation_pattern(matrix.get_height() - vsp_size - 1, 0, matrix);
        // Left bottom corner.
        ::embed_vertical_separation_pattern(vsp_size, matrix.get_height() - vsp_size, matrix);
    }

    // Embed position adjustment patterns if need be.
    fn  maybe_embed_position_adjustment_patterns( version: &Version,  matrix: &ByteMatrix)   {
        if version.get_version_number() < 2 {
            // The patterns appear if version >= 2
            return;
        }
         let index: i32 = version.get_version_number() - 1;
         let coordinates: Vec<i32> = POSITION_ADJUSTMENT_PATTERN_COORDINATE_TABLE[index];
        for  let y: i32 in coordinates {
            if y >= 0 {
                for  let x: i32 in coordinates {
                    if x >= 0 && ::is_empty(&matrix.get(x, y)) {
                        // If the cell is unset, we embed the position adjustment pattern here.
                        // -2 is necessary since the x/y coordinates point to the center of the pattern, not the
                        // left top corner.
                        ::embed_position_adjustment_pattern(x - 2, y - 2, matrix);
                    }
                }
            }
        }
    }
}

// NEW FILE: minimal_encoder.rs
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

// NEW FILE: q_r_code.rs
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
// package com::google::zxing::qrcode::encoder;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author dswitkin@google.com (Daniel Switkin) - ported from C++
 */

 const NUM_MASK_PATTERNS: i32 = 8;
pub struct QRCode {

     let mut mode: Mode;

     let ec_level: ErrorCorrectionLevel;

     let version: Version;

     let mask_pattern: i32;

     let mut matrix: ByteMatrix;
}

impl QRCode {

    pub fn new() -> QRCode {
        mask_pattern = -1;
    }

    /**
   * @return the mode. Not relevant if {@link com.google.zxing.EncodeHintType#QR_COMPACT} is selected.
   */
    pub fn  get_mode(&self) -> Mode  {
        return self.mode;
    }

    pub fn  get_e_c_level(&self) -> ErrorCorrectionLevel  {
        return self.ec_level;
    }

    pub fn  get_version(&self) -> Version  {
        return self.version;
    }

    pub fn  get_mask_pattern(&self) -> i32  {
        return self.mask_pattern;
    }

    pub fn  get_matrix(&self) -> ByteMatrix  {
        return self.matrix;
    }

    pub fn  to_string(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(200);
        result.append("<<\n");
        result.append(" mode: ");
        result.append(self.mode);
        result.append("\n ecLevel: ");
        result.append(self.ec_level);
        result.append("\n version: ");
        result.append(self.version);
        result.append("\n maskPattern: ");
        result.append(self.mask_pattern);
        if self.matrix == null {
            result.append("\n matrix: null\n");
        } else {
            result.append("\n matrix:\n");
            result.append(self.matrix);
        }
        result.append(">>\n");
        return result.to_string();
    }

    pub fn  set_mode(&self,  value: &Mode)   {
        self.mode = value;
    }

    pub fn  set_e_c_level(&self,  value: &ErrorCorrectionLevel)   {
        self.ec_level = value;
    }

    pub fn  set_version(&self,  version: &Version)   {
        self.version = version;
    }

    pub fn  set_mask_pattern(&self,  value: i32)   {
        self.mask_pattern = value;
    }

    pub fn  set_matrix(&self,  value: &ByteMatrix)   {
        self.matrix = value;
    }

    // Check if "mask_pattern" is valid.
    pub fn  is_valid_mask_pattern( mask_pattern: i32) -> bool  {
        return mask_pattern >= 0 && mask_pattern < NUM_MASK_PATTERNS;
    }
}

