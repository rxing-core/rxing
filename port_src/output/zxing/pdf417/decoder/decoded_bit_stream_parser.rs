/*
 * Copyright 2009 ZXing authors
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
// package com::google::zxing::pdf417::decoder;

/**
 * <p>This class contains the methods for decoding the PDF417 codewords.</p>
 *
 * @author SITA Lab (kevin.osullivan@sita.aero)
 * @author Guenther Grau
 */

 const TEXT_COMPACTION_MODE_LATCH: i32 = 900;

 const BYTE_COMPACTION_MODE_LATCH: i32 = 901;

 const NUMERIC_COMPACTION_MODE_LATCH: i32 = 902;

 const BYTE_COMPACTION_MODE_LATCH_6: i32 = 924;

 const ECI_USER_DEFINED: i32 = 925;

 const ECI_GENERAL_PURPOSE: i32 = 926;

 const ECI_CHARSET: i32 = 927;

 const BEGIN_MACRO_PDF417_CONTROL_BLOCK: i32 = 928;

 const BEGIN_MACRO_PDF417_OPTIONAL_FIELD: i32 = 923;

 const MACRO_PDF417_TERMINATOR: i32 = 922;

 const MODE_SHIFT_TO_BYTE_COMPACTION_MODE: i32 = 913;

 const MAX_NUMERIC_CODEWORDS: i32 = 15;

 const MACRO_PDF417_OPTIONAL_FIELD_FILE_NAME: i32 = 0;

 const MACRO_PDF417_OPTIONAL_FIELD_SEGMENT_COUNT: i32 = 1;

 const MACRO_PDF417_OPTIONAL_FIELD_TIME_STAMP: i32 = 2;

 const MACRO_PDF417_OPTIONAL_FIELD_SENDER: i32 = 3;

 const MACRO_PDF417_OPTIONAL_FIELD_ADDRESSEE: i32 = 4;

 const MACRO_PDF417_OPTIONAL_FIELD_FILE_SIZE: i32 = 5;

 const MACRO_PDF417_OPTIONAL_FIELD_CHECKSUM: i32 = 6;

 const PL: i32 = 25;

 const LL: i32 = 27;

 const AS: i32 = 27;

 const ML: i32 = 28;

 const AL: i32 = 28;

 const PS: i32 = 29;

 const PAL: i32 = 29;

 const PUNCT_CHARS: Vec<char> = ";<>@[\\]_`~!\r\t,:\n-.$/\"|*()?{}'".to_char_array();

 const MIXED_CHARS: Vec<char> = "0123456789&\r\t,:#-.$/+%*=^".to_char_array();

/**
   * Table containing values for the exponent of 900.
   * This is used in the numeric compaction decode algorithm.
   */
 const EXP900: Vec<BigInteger>;

 const NUMBER_OF_SEQUENCE_CODEWORDS: i32 = 2;
struct DecodedBitStreamParser {
}

impl DecodedBitStreamParser {

    enum Mode {

        ALPHA(), LOWER(), MIXED(), PUNCT(), ALPHA_SHIFT(), PUNCT_SHIFT()
    }

    static {
        EXP900 = : [Option<BigInteger>; 16] = [None; 16];
        EXP900[0] = BigInteger::ONE;
         let nine_hundred: BigInteger = BigInteger::value_of(900);
        EXP900[1] = nine_hundred;
         {
             let mut i: i32 = 2;
            while i < EXP900.len() {
                {
                    EXP900[i] = EXP900[i - 1]::multiply(&nine_hundred);
                }
                i += 1;
             }
         }

    }

    fn new() -> DecodedBitStreamParser {
    }

    fn  decode( codewords: &Vec<i32>,  ec_level: &String) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
         let result: ECIStringBuilder = ECIStringBuilder::new(codewords.len() * 2);
         let code_index: i32 = ::text_compaction(&codewords, 1, result);
         let result_metadata: PDF417ResultMetadata = PDF417ResultMetadata::new();
        while code_index < codewords[0] {
             let code: i32 = codewords[code_index += 1 !!!check!!! post increment];
            match code {
                  TEXT_COMPACTION_MODE_LATCH => 
                     {
                        code_index = ::text_compaction(&codewords, code_index, result);
                        break;
                    }
                  BYTE_COMPACTION_MODE_LATCH => 
                     {
                    }
                  BYTE_COMPACTION_MODE_LATCH_6 => 
                     {
                        code_index = ::byte_compaction(code, &codewords, code_index, result);
                        break;
                    }
                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                     {
                        result.append(codewords[code_index += 1 !!!check!!! post increment] as char);
                        break;
                    }
                  NUMERIC_COMPACTION_MODE_LATCH => 
                     {
                        code_index = ::numeric_compaction(&codewords, code_index, result);
                        break;
                    }
                  ECI_CHARSET => 
                     {
                        result.append_e_c_i(codewords[code_index += 1 !!!check!!! post increment]);
                        break;
                    }
                  ECI_GENERAL_PURPOSE => 
                     {
                        // Can't do anything with generic ECI; skip its 2 characters
                        code_index += 2;
                        break;
                    }
                  ECI_USER_DEFINED => 
                     {
                        // Can't do anything with user ECI; skip its 1 character
                        code_index += 1;
                        break;
                    }
                  BEGIN_MACRO_PDF417_CONTROL_BLOCK => 
                     {
                        code_index = ::decode_macro_block(&codewords, code_index, result_metadata);
                        break;
                    }
                  BEGIN_MACRO_PDF417_OPTIONAL_FIELD => 
                     {
                    }
                  MACRO_PDF417_TERMINATOR => 
                     {
                        // Should not see these outside a macro block
                        throw FormatException::get_format_instance();
                    }
                _ => 
                     {
                        // Default to text compaction. During testing numerous barcodes
                        // appeared to be missing the starting mode. In these cases defaulting
                        // to text compaction seems to work.
                        code_index -= 1;
                        code_index = ::text_compaction(&codewords, code_index, result);
                        break;
                    }
            }
        }
        if result.is_empty() && result_metadata.get_file_id() == null {
            throw FormatException::get_format_instance();
        }
         let decoder_result: DecoderResult = DecoderResult::new(null, &result.to_string(), null, &ec_level);
        decoder_result.set_other(result_metadata);
        return Ok(decoder_result);
    }

    fn  decode_macro_block( codewords: &Vec<i32>,  code_index: i32,  result_metadata: &PDF417ResultMetadata) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
        if code_index + NUMBER_OF_SEQUENCE_CODEWORDS > codewords[0] {
            // we must have at least two bytes left for the segment index
            throw FormatException::get_format_instance();
        }
         let segment_index_array: [i32; NUMBER_OF_SEQUENCE_CODEWORDS] = [0; NUMBER_OF_SEQUENCE_CODEWORDS];
         {
             let mut i: i32 = 0;
            while i < NUMBER_OF_SEQUENCE_CODEWORDS {
                {
                    segment_index_array[i] = codewords[code_index];
                }
                i += 1;
                code_index += 1;
             }
         }

         let segment_index_string: String = ::decode_base900to_base10(&segment_index_array, NUMBER_OF_SEQUENCE_CODEWORDS);
        if segment_index_string.is_empty() {
            result_metadata.set_segment_index(0);
        } else {
            let tryResult1 = 0;
            'try1: loop {
            {
                result_metadata.set_segment_index(&Integer::parse_int(&segment_index_string));
            }
            break 'try1
            }
            match tryResult1 {
                 catch ( nfe: &NumberFormatException) {
                    throw FormatException::get_format_instance();
                }  0 => break
            }

        }
        // Decoding the fileId codewords as 0-899 numbers, each 0-filled to width 3. This follows the spec
        // (See ISO/IEC 15438:2015 Annex H.6) and preserves all info, but some generators (e.g. TEC-IT) write
        // the fileId using text compaction, so in those cases the fileId will appear mangled.
         let file_id: StringBuilder = StringBuilder::new();
        while code_index < codewords[0] && code_index < codewords.len() && codewords[code_index] != MACRO_PDF417_TERMINATOR && codewords[code_index] != BEGIN_MACRO_PDF417_OPTIONAL_FIELD {
            file_id.append(&String::format("%03d", codewords[code_index]));
            code_index += 1;
        }
        if file_id.length() == 0 {
            // at least one fileId codeword is required (Annex H.2)
            throw FormatException::get_format_instance();
        }
        result_metadata.set_file_id(&file_id.to_string());
         let optional_fields_start: i32 = -1;
        if codewords[code_index] == BEGIN_MACRO_PDF417_OPTIONAL_FIELD {
            optional_fields_start = code_index + 1;
        }
        while code_index < codewords[0] {
            match codewords[code_index] {
                  BEGIN_MACRO_PDF417_OPTIONAL_FIELD => 
                     {
                        code_index += 1;
                        match codewords[code_index] {
                              MACRO_PDF417_OPTIONAL_FIELD_FILE_NAME => 
                                 {
                                     let file_name: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::text_compaction(&codewords, code_index + 1, file_name);
                                    result_metadata.set_file_name(&file_name.to_string());
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_SENDER => 
                                 {
                                     let sender: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::text_compaction(&codewords, code_index + 1, sender);
                                    result_metadata.set_sender(&sender.to_string());
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_ADDRESSEE => 
                                 {
                                     let addressee: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::text_compaction(&codewords, code_index + 1, addressee);
                                    result_metadata.set_addressee(&addressee.to_string());
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_SEGMENT_COUNT => 
                                 {
                                     let segment_count: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::numeric_compaction(&codewords, code_index + 1, segment_count);
                                    result_metadata.set_segment_count(&Integer::parse_int(&segment_count.to_string()));
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_TIME_STAMP => 
                                 {
                                     let timestamp: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::numeric_compaction(&codewords, code_index + 1, timestamp);
                                    result_metadata.set_timestamp(&Long::parse_long(&timestamp.to_string()));
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_CHECKSUM => 
                                 {
                                     let checksum: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::numeric_compaction(&codewords, code_index + 1, checksum);
                                    result_metadata.set_checksum(&Integer::parse_int(&checksum.to_string()));
                                    break;
                                }
                              MACRO_PDF417_OPTIONAL_FIELD_FILE_SIZE => 
                                 {
                                     let file_size: ECIStringBuilder = ECIStringBuilder::new();
                                    code_index = ::numeric_compaction(&codewords, code_index + 1, file_size);
                                    result_metadata.set_file_size(&Long::parse_long(&file_size.to_string()));
                                    break;
                                }
                            _ => 
                                 {
                                    throw FormatException::get_format_instance();
                                }
                        }
                        break;
                    }
                  MACRO_PDF417_TERMINATOR => 
                     {
                        code_index += 1;
                        result_metadata.set_last_segment(true);
                        break;
                    }
                _ => 
                     {
                        throw FormatException::get_format_instance();
                    }
            }
        }
        // copy optional fields to additional options
        if optional_fields_start != -1 {
             let optional_fields_length: i32 = code_index - optional_fields_start;
            if result_metadata.is_last_segment() {
                // do not include terminator
                optional_fields_length -= 1;
            }
            result_metadata.set_optional_data(&Arrays::copy_of_range(&codewords, optional_fields_start, optional_fields_start + optional_fields_length));
        }
        return Ok(code_index);
    }

    /**
   * Text Compaction mode (see 5.4.1.5) permits all printable ASCII characters to be
   * encoded, i.e. values 32 - 126 inclusive in accordance with ISO/IEC 646 (IRV), as
   * well as selected control characters.
   *
   * @param codewords The array of codewords (data + error)
   * @param codeIndex The current index into the codeword array.
   * @param result    The decoded data is appended to the result.
   * @return The next index into the codeword array.
   */
    fn  text_compaction( codewords: &Vec<i32>,  code_index: i32,  result: &ECIStringBuilder) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
        // 2 character per codeword
         let text_compaction_data: [i32; (codewords[0] - code_index) * 2] = [0; (codewords[0] - code_index) * 2];
        // Used to hold the byte compaction value if there is a mode shift
         let byte_compaction_data: [i32; (codewords[0] - code_index) * 2] = [0; (codewords[0] - code_index) * 2];
         let mut index: i32 = 0;
         let mut end: bool = false;
         let sub_mode: Mode = Mode::ALPHA;
        while (code_index < codewords[0]) && !end {
             let mut code: i32 = codewords[code_index += 1 !!!check!!! post increment];
            if code < TEXT_COMPACTION_MODE_LATCH {
                text_compaction_data[index] = code / 30;
                text_compaction_data[index + 1] = code % 30;
                index += 2;
            } else {
                match code {
                      TEXT_COMPACTION_MODE_LATCH => 
                         {
                            // reinitialize text compaction mode to alpha sub mode
                            text_compaction_data[index += 1 !!!check!!! post increment] = TEXT_COMPACTION_MODE_LATCH;
                            break;
                        }
                      BYTE_COMPACTION_MODE_LATCH => 
                         {
                        }
                      BYTE_COMPACTION_MODE_LATCH_6 => 
                         {
                        }
                      NUMERIC_COMPACTION_MODE_LATCH => 
                         {
                        }
                      BEGIN_MACRO_PDF417_CONTROL_BLOCK => 
                         {
                        }
                      BEGIN_MACRO_PDF417_OPTIONAL_FIELD => 
                         {
                        }
                      MACRO_PDF417_TERMINATOR => 
                         {
                            code_index -= 1;
                            end = true;
                            break;
                        }
                      MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                         {
                            // The Mode Shift codeword 913 shall cause a temporary
                            // switch from Text Compaction mode to Byte Compaction mode.
                            // This switch shall be in effect for only the next codeword,
                            // after which the mode shall revert to the prevailing sub-mode
                            // of the Text Compaction mode. Codeword 913 is only available
                            // in Text Compaction mode; its use is described in 5.4.2.4.
                            text_compaction_data[index] = MODE_SHIFT_TO_BYTE_COMPACTION_MODE;
                            code = codewords[code_index += 1 !!!check!!! post increment];
                            byte_compaction_data[index] = code;
                            index += 1;
                            break;
                        }
                      ECI_CHARSET => 
                         {
                            sub_mode = ::decode_text_compaction(&text_compaction_data, &byte_compaction_data, index, result, sub_mode);
                            result.append_e_c_i(codewords[code_index += 1 !!!check!!! post increment]);
                            text_compaction_data = : [i32; (codewords[0] - code_index) * 2] = [0; (codewords[0] - code_index) * 2];
                            byte_compaction_data = : [i32; (codewords[0] - code_index) * 2] = [0; (codewords[0] - code_index) * 2];
                            index = 0;
                            break;
                        }
                }
            }
        }
        ::decode_text_compaction(&text_compaction_data, &byte_compaction_data, index, result, sub_mode);
        return Ok(code_index);
    }

    /**
   * The Text Compaction mode includes all the printable ASCII characters
   * (i.e. values from 32 to 126) and three ASCII control characters: HT or tab
   * (ASCII value 9), LF or line feed (ASCII value 10), and CR or carriage
   * return (ASCII value 13). The Text Compaction mode also includes various latch
   * and shift characters which are used exclusively within the mode. The Text
   * Compaction mode encodes up to 2 characters per codeword. The compaction rules
   * for converting data into PDF417 codewords are defined in 5.4.2.2. The sub-mode
   * switches are defined in 5.4.2.3.
   *
   * @param textCompactionData The text compaction data.
   * @param byteCompactionData The byte compaction data if there
   *                           was a mode shift.
   * @param length             The size of the text compaction and byte compaction data.
   * @param result             The decoded data is appended to the result.
   * @param startMode          The mode in which decoding starts
   * @return The mode in which decoding ended
   */
    fn  decode_text_compaction( text_compaction_data: &Vec<i32>,  byte_compaction_data: &Vec<i32>,  length: i32,  result: &ECIStringBuilder,  start_mode: &Mode) -> Mode  {
        // Beginning from an initial state
        // The default compaction mode for PDF417 in effect at the start of each symbol shall always be Text
        // Compaction mode Alpha sub-mode (uppercase alphabetic). A latch codeword from another mode to the Text
        // Compaction mode shall always switch to the Text Compaction Alpha sub-mode.
         let sub_mode: Mode = start_mode;
         let prior_to_shift_mode: Mode = start_mode;
         let latched_mode: Mode = start_mode;
         let mut i: i32 = 0;
        while i < length {
             let sub_mode_ch: i32 = text_compaction_data[i];
             let mut ch: char = 0;
            match sub_mode {
                  ALPHA => 
                     {
                        // Alpha (uppercase alphabetic)
                        if sub_mode_ch < 26 {
                            // Upper case Alpha Character
                            ch = ('A' + sub_mode_ch) as char;
                        } else {
                            match sub_mode_ch {
                                  26 => 
                                     {
                                        ch = ' ';
                                        break;
                                    }
                                  LL => 
                                     {
                                        sub_mode = Mode::LOWER;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  ML => 
                                     {
                                        sub_mode = Mode::MIXED;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  PS => 
                                     {
                                        // Shift to punctuation
                                        prior_to_shift_mode = sub_mode;
                                        sub_mode = Mode::PUNCT_SHIFT;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  LOWER => 
                     {
                        // Lower (lowercase alphabetic)
                        if sub_mode_ch < 26 {
                            ch = ('a' + sub_mode_ch) as char;
                        } else {
                            match sub_mode_ch {
                                  26 => 
                                     {
                                        ch = ' ';
                                        break;
                                    }
                                  AS => 
                                     {
                                        // Shift to alpha
                                        prior_to_shift_mode = sub_mode;
                                        sub_mode = Mode::ALPHA_SHIFT;
                                        break;
                                    }
                                  ML => 
                                     {
                                        sub_mode = Mode::MIXED;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  PS => 
                                     {
                                        // Shift to punctuation
                                        prior_to_shift_mode = sub_mode;
                                        sub_mode = Mode::PUNCT_SHIFT;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  MIXED => 
                     {
                        // Mixed (numeric and some punctuation)
                        if sub_mode_ch < PL {
                            ch = MIXED_CHARS[sub_mode_ch];
                        } else {
                            match sub_mode_ch {
                                  PL => 
                                     {
                                        sub_mode = Mode::PUNCT;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  26 => 
                                     {
                                        ch = ' ';
                                        break;
                                    }
                                  LL => 
                                     {
                                        sub_mode = Mode::LOWER;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  AL => 
                                     {
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  PS => 
                                     {
                                        // Shift to punctuation
                                        prior_to_shift_mode = sub_mode;
                                        sub_mode = Mode::PUNCT_SHIFT;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  PUNCT => 
                     {
                        // Punctuation
                        if sub_mode_ch < PAL {
                            ch = PUNCT_CHARS[sub_mode_ch];
                        } else {
                            match sub_mode_ch {
                                  PAL => 
                                     {
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        latched_mode = sub_mode;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  ALPHA_SHIFT => 
                     {
                        // Restore sub-mode
                        sub_mode = prior_to_shift_mode;
                        if sub_mode_ch < 26 {
                            ch = ('A' + sub_mode_ch) as char;
                        } else {
                            match sub_mode_ch {
                                  26 => 
                                     {
                                        ch = ' ';
                                        break;
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        break;
                                    }
                            }
                        }
                        break;
                    }
                  PUNCT_SHIFT => 
                     {
                        // Restore sub-mode
                        sub_mode = prior_to_shift_mode;
                        if sub_mode_ch < PAL {
                            ch = PUNCT_CHARS[sub_mode_ch];
                        } else {
                            match sub_mode_ch {
                                  PAL => 
                                     {
                                    }
                                  TEXT_COMPACTION_MODE_LATCH => 
                                     {
                                        sub_mode = Mode::ALPHA;
                                        break;
                                    }
                                  MODE_SHIFT_TO_BYTE_COMPACTION_MODE => 
                                     {
                                        // PS before Shift-to-Byte is used as a padding character,
                                        // see 5.4.2.4 of the specification
                                        result.append(byte_compaction_data[i] as char);
                                        break;
                                    }
                            }
                        }
                        break;
                    }
            }
            if ch != 0 {
                // Append decoded character to result
                result.append(ch);
            }
            i += 1;
        }
        return latched_mode;
    }

    /**
   * Byte Compaction mode (see 5.4.3) permits all 256 possible 8-bit byte values to be encoded.
   * This includes all ASCII characters value 0 to 127 inclusive and provides for international
   * character set support.
   *
   * @param mode      The byte compaction mode i.e. 901 or 924
   * @param codewords The array of codewords (data + error)
   * @param codeIndex The current index into the codeword array.
   * @param result    The decoded data is appended to the result.
   * @return The next index into the codeword array.
   */
    fn  byte_compaction( mode: i32,  codewords: &Vec<i32>,  code_index: i32,  result: &ECIStringBuilder) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
         let mut end: bool = false;
        while code_index < codewords[0] && !end {
            //handle leading ECIs
            while code_index < codewords[0] && codewords[code_index] == ECI_CHARSET {
                result.append_e_c_i(codewords[code_index += 1]);
                code_index += 1;
            }
            if code_index >= codewords[0] || codewords[code_index] >= TEXT_COMPACTION_MODE_LATCH {
                end = true;
            } else {
                //decode one block of 5 codewords to 6 bytes
                 let mut value: i64 = 0;
                 let mut count: i32 = 0;
                loop { {
                    value = 900 * value + codewords[code_index += 1 !!!check!!! post increment];
                    count += 1;
                }if !(count < 5 && code_index < codewords[0] && codewords[code_index] < TEXT_COMPACTION_MODE_LATCH) break;}
                if count == 5 && (mode == BYTE_COMPACTION_MODE_LATCH_6 || code_index < codewords[0] && codewords[code_index] < TEXT_COMPACTION_MODE_LATCH) {
                     {
                         let mut i: i32 = 0;
                        while i < 6 {
                            {
                                result.append((value >> (8 * (5 - i))) as i8);
                            }
                            i += 1;
                         }
                     }

                } else {
                    code_index -= count;
                    while (code_index < codewords[0]) && !end {
                         let code: i32 = codewords[code_index += 1 !!!check!!! post increment];
                        if code < TEXT_COMPACTION_MODE_LATCH {
                            result.append(code as i8);
                        } else if code == ECI_CHARSET {
                            result.append_e_c_i(codewords[code_index += 1 !!!check!!! post increment]);
                        } else {
                            code_index -= 1;
                            end = true;
                        }
                    }
                }
            }
        }
        return Ok(code_index);
    }

    /**
   * Numeric Compaction mode (see 5.4.4) permits efficient encoding of numeric data strings.
   *
   * @param codewords The array of codewords (data + error)
   * @param codeIndex The current index into the codeword array.
   * @param result    The decoded data is appended to the result.
   * @return The next index into the codeword array.
   */
    fn  numeric_compaction( codewords: &Vec<i32>,  code_index: i32,  result: &ECIStringBuilder) -> /*  throws FormatException */Result<i32, Rc<Exception>>   {
         let mut count: i32 = 0;
         let mut end: bool = false;
         let numeric_codewords: [i32; MAX_NUMERIC_CODEWORDS] = [0; MAX_NUMERIC_CODEWORDS];
        while code_index < codewords[0] && !end {
             let code: i32 = codewords[code_index += 1 !!!check!!! post increment];
            if code_index == codewords[0] {
                end = true;
            }
            if code < TEXT_COMPACTION_MODE_LATCH {
                numeric_codewords[count] = code;
                count += 1;
            } else {
                match code {
                      TEXT_COMPACTION_MODE_LATCH => 
                         {
                        }
                      BYTE_COMPACTION_MODE_LATCH => 
                         {
                        }
                      BYTE_COMPACTION_MODE_LATCH_6 => 
                         {
                        }
                      BEGIN_MACRO_PDF417_CONTROL_BLOCK => 
                         {
                        }
                      BEGIN_MACRO_PDF417_OPTIONAL_FIELD => 
                         {
                        }
                      MACRO_PDF417_TERMINATOR => 
                         {
                        }
                      ECI_CHARSET => 
                         {
                            code_index -= 1;
                            end = true;
                            break;
                        }
                }
            }
            if (count % MAX_NUMERIC_CODEWORDS == 0 || code == NUMERIC_COMPACTION_MODE_LATCH || end) && count > 0 {
                // Re-invoking Numeric Compaction mode (by using codeword 902
                // while in Numeric Compaction mode) serves  to terminate the
                // current Numeric Compaction mode grouping as described in 5.4.4.2,
                // and then to start a new one grouping.
                result.append(&::decode_base900to_base10(&numeric_codewords, count));
                count = 0;
            }
        }
        return Ok(code_index);
    }

    /**
   * Convert a list of Numeric Compacted codewords from Base 900 to Base 10.
   *
   * @param codewords The array of codewords
   * @param count     The number of codewords
   * @return The decoded string representing the Numeric data.
   */
    /*
     EXAMPLE
     Encode the fifteen digit numeric string 000213298174000
     Prefix the numeric string with a 1 and set the initial value of
     t = 1 000 213 298 174 000
     Calculate codeword 0
     d0 = 1 000 213 298 174 000 mod 900 = 200

     t = 1 000 213 298 174 000 div 900 = 1 111 348 109 082
     Calculate codeword 1
     d1 = 1 111 348 109 082 mod 900 = 282

     t = 1 111 348 109 082 div 900 = 1 234 831 232
     Calculate codeword 2
     d2 = 1 234 831 232 mod 900 = 632

     t = 1 234 831 232 div 900 = 1 372 034
     Calculate codeword 3
     d3 = 1 372 034 mod 900 = 434

     t = 1 372 034 div 900 = 1 524
     Calculate codeword 4
     d4 = 1 524 mod 900 = 624

     t = 1 524 div 900 = 1
     Calculate codeword 5
     d5 = 1 mod 900 = 1
     t = 1 div 900 = 0
     Codeword sequence is: 1, 624, 434, 632, 282, 200

     Decode the above codewords involves
       1 x 900 power of 5 + 624 x 900 power of 4 + 434 x 900 power of 3 +
     632 x 900 power of 2 + 282 x 900 power of 1 + 200 x 900 power of 0 = 1000213298174000

     Remove leading 1 =>  Result is 000213298174000
   */
    fn  decode_base900to_base10( codewords: &Vec<i32>,  count: i32) -> /*  throws FormatException */Result<String, Rc<Exception>>   {
         let mut result: BigInteger = BigInteger::ZERO;
         {
             let mut i: i32 = 0;
            while i < count {
                {
                    result = result.add(&EXP900[count - i - 1]::multiply(&BigInteger::value_of(codewords[i])));
                }
                i += 1;
             }
         }

         let result_string: String = result.to_string();
        if result_string.char_at(0) != '1' {
            throw FormatException::get_format_instance();
        }
        return Ok(result_string.substring(1));
    }
}

