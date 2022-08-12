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

