/*
 * Copyright 2006 Jeremias Maerki in part, and ZXing Authors in part
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
// package com::google::zxing::pdf417::encoder;

/**
 * PDF417 high-level encoder following the algorithm described in ISO/IEC 15438:2001(E) in
 * annex P.
 */

/**
   * code for Text compaction
   */
 const TEXT_COMPACTION: i32 = 0;

/**
   * code for Byte compaction
   */
 const BYTE_COMPACTION: i32 = 1;

/**
   * code for Numeric compaction
   */
 const NUMERIC_COMPACTION: i32 = 2;

/**
   * Text compaction submode Alpha
   */
 const SUBMODE_ALPHA: i32 = 0;

/**
   * Text compaction submode Lower
   */
 const SUBMODE_LOWER: i32 = 1;

/**
   * Text compaction submode Mixed
   */
 const SUBMODE_MIXED: i32 = 2;

/**
   * Text compaction submode Punctuation
   */
 const SUBMODE_PUNCTUATION: i32 = 3;

/**
   * mode latch to Text Compaction mode
   */
 const LATCH_TO_TEXT: i32 = 900;

/**
   * mode latch to Byte Compaction mode (number of characters NOT a multiple of 6)
   */
 const LATCH_TO_BYTE_PADDED: i32 = 901;

/**
   * mode latch to Numeric Compaction mode
   */
 const LATCH_TO_NUMERIC: i32 = 902;

/**
   * mode shift to Byte Compaction mode
   */
 const SHIFT_TO_BYTE: i32 = 913;

/**
   * mode latch to Byte Compaction mode (number of characters a multiple of 6)
   */
 const LATCH_TO_BYTE: i32 = 924;

/**
   * identifier for a user defined Extended Channel Interpretation (ECI)
   */
 const ECI_USER_DEFINED: i32 = 925;

/**
   * identifier for a general purpose ECO format
   */
 const ECI_GENERAL_PURPOSE: i32 = 926;

/**
   * identifier for an ECI of a character set of code page
   */
 const ECI_CHARSET: i32 = 927;

/**
   * Raw code table for text compaction Mixed sub-mode
   */
 const TEXT_MIXED_RAW: vec![Vec<i8>; 30] = vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 38, 13, 9, 44, 58, 35, 45, 46, 36, 47, 43, 37, 42, 61, 94, 0, 32, 0, 0, 0, ]
;

/**
   * Raw code table for text compaction: Punctuation sub-mode
   */
 const TEXT_PUNCTUATION_RAW: vec![Vec<i8>; 30] = vec![59, 60, 62, 64, 91, 92, 93, 95, 96, 126, 33, 13, 9, 44, 58, 10, 45, 46, 36, 47, 34, 124, 42, 40, 41, 63, 123, 125, 39, 0, ]
;

 const MIXED: [i8; 128] = [0; 128];

 const PUNCTUATION: [i8; 128] = [0; 128];

 const DEFAULT_ENCODING: Charset = StandardCharsets::ISO_8859_1;
struct PDF417HighLevelEncoder {
}

impl PDF417HighLevelEncoder {

    fn new() -> PDF417HighLevelEncoder {
    }

    static {
        //Construct inverse lookups
        Arrays::fill(&MIXED, -1 as i8);
         {
             let mut i: i32 = 0;
            while i < TEXT_MIXED_RAW.len() {
                {
                     let mut b: i8 = TEXT_MIXED_RAW[i];
                    if b > 0 {
                        MIXED[b] = i as i8;
                    }
                }
                i += 1;
             }
         }

        Arrays::fill(&PUNCTUATION, -1 as i8);
         {
             let mut i: i32 = 0;
            while i < TEXT_PUNCTUATION_RAW.len() {
                {
                     let mut b: i8 = TEXT_PUNCTUATION_RAW[i];
                    if b > 0 {
                        PUNCTUATION[b] = i as i8;
                    }
                }
                i += 1;
             }
         }

    }

    /**
   * Performs high-level encoding of a PDF417 message using the algorithm described in annex P
   * of ISO/IEC 15438:2001(E). If byte compaction has been selected, then only byte compaction
   * is used.
   *
   * @param msg the message
   * @param compaction compaction mode to use
   * @param encoding character encoding used to encode in default or byte compaction
   *  or {@code null} for default / not applicable
   * @param autoECI encode input minimally using multiple ECIs if needed
   *   If autoECI encoding is specified and additionally {@code encoding} is specified, then the encoder
   *   will use the specified {@link Charset} for any character that can be encoded by it, regardless
   *   if a different encoding would lead to a more compact encoding. When no {@code encoding} is specified
   *   then charsets will be chosen so that the byte representation is minimal.
   * @return the encoded message (the char values range from 0 to 928)
   */
    fn  encode_high_level( msg: &String,  compaction: &Compaction,  encoding: &Charset,  auto_e_c_i: bool) -> /*  throws WriterException */Result<String, Rc<Exception>>   {
        if msg.is_empty() {
            throw WriterException::new("Empty message not allowed");
        }
        if encoding == null && !auto_e_c_i {
             {
                 let mut i: i32 = 0;
                while i < msg.length() {
                    {
                        if msg.char_at(i) > 255 {
                            throw WriterException::new(format!("Non-encodable character detected: {} (Unicode: {}). Consider specifying EncodeHintType.PDF417_AUTO_ECI and/or EncodeTypeHint.CHARACTER_SET.", msg.char_at(i), msg.char_at(i) as i32));
                        }
                    }
                    i += 1;
                 }
             }

        }
        //the codewords 0..928 are encoded as Unicode characters
         let sb: StringBuilder = StringBuilder::new(&msg.length());
         let mut input: ECIInput;
        if auto_e_c_i {
            input = MinimalECIInput::new(&msg, &encoding, -1);
        } else {
            input = NoECIInput::new(&msg);
            if encoding == null {
                encoding = DEFAULT_ENCODING;
            } else if !DEFAULT_ENCODING::equals(&encoding) {
                 let eci: CharacterSetECI = CharacterSetECI::get_character_set_e_c_i(&encoding);
                if eci != null {
                    ::encoding_e_c_i(&eci.get_value(), &sb);
                }
            }
        }
         let len: i32 = input.length();
         let mut p: i32 = 0;
         let text_sub_mode: i32 = SUBMODE_ALPHA;
        // User selected encoding mode
        match compaction {
              TEXT => 
                 {
                    ::encode_text(input, p, len, &sb, text_sub_mode);
                    break;
                }
              BYTE => 
                 {
                    if auto_e_c_i {
                        ::encode_multi_e_c_i_binary(input, 0, &input.length(), TEXT_COMPACTION, &sb);
                    } else {
                         let msg_bytes: Vec<i8> = input.to_string().get_bytes(&encoding);
                        ::encode_binary(&msg_bytes, p, msg_bytes.len(), BYTE_COMPACTION, &sb);
                    }
                    break;
                }
              NUMERIC => 
                 {
                    sb.append(LATCH_TO_NUMERIC as char);
                    ::encode_numeric(input, p, len, &sb);
                    break;
                }
            _ => 
                 {
                    //Default mode, see 4.4.2.1
                     let encoding_mode: i32 = TEXT_COMPACTION;
                    while p < len {
                        while p < len && input.is_e_c_i(p) {
                            ::encoding_e_c_i(&input.get_e_c_i_value(p), &sb);
                            p += 1;
                        }
                        if p >= len {
                            break;
                        }
                         let n: i32 = ::determine_consecutive_digit_count(input, p);
                        if n >= 13 {
                            sb.append(LATCH_TO_NUMERIC as char);
                            encoding_mode = NUMERIC_COMPACTION;
                            //Reset after latch
                            text_sub_mode = SUBMODE_ALPHA;
                            ::encode_numeric(input, p, n, &sb);
                            p += n;
                        } else {
                             let t: i32 = ::determine_consecutive_text_count(input, p);
                            if t >= 5 || n == len {
                                if encoding_mode != TEXT_COMPACTION {
                                    sb.append(LATCH_TO_TEXT as char);
                                    encoding_mode = TEXT_COMPACTION;
                                    //start with submode alpha after latch
                                    text_sub_mode = SUBMODE_ALPHA;
                                }
                                text_sub_mode = ::encode_text(input, p, t, &sb, text_sub_mode);
                                p += t;
                            } else {
                                 let mut b: i32 = ::determine_consecutive_binary_count(input, p,  if auto_e_c_i { null } else { encoding });
                                if b == 0 {
                                    b = 1;
                                }
                                 let bytes: Vec<i8> =  if auto_e_c_i { null } else { input.sub_sequence(p, p + b).to_string().get_bytes(&encoding) };
                                if ((bytes == null && b == 1) || (bytes != null && bytes.len() == 1)) && encoding_mode == TEXT_COMPACTION {
                                    //Switch for one byte (instead of latch)
                                    if auto_e_c_i {
                                        ::encode_multi_e_c_i_binary(input, p, 1, TEXT_COMPACTION, &sb);
                                    } else {
                                        ::encode_binary(&bytes, 0, 1, TEXT_COMPACTION, &sb);
                                    }
                                } else {
                                    //Mode latch performed by encodeBinary()
                                    if auto_e_c_i {
                                        ::encode_multi_e_c_i_binary(input, p, p + b, encoding_mode, &sb);
                                    } else {
                                        ::encode_binary(&bytes, 0, bytes.len(), encoding_mode, &sb);
                                    }
                                    encoding_mode = BYTE_COMPACTION;
                                    //Reset after latch
                                    text_sub_mode = SUBMODE_ALPHA;
                                }
                                p += b;
                            }
                        }
                    }
                    break;
                }
        }
        return Ok(sb.to_string());
    }

    /**
   * Encode parts of the message using Text Compaction as described in ISO/IEC 15438:2001(E),
   * chapter 4.4.2.
   *
   * @param input          the input
   * @param startpos       the start position within the message
   * @param count          the number of characters to encode
   * @param sb             receives the encoded codewords
   * @param initialSubmode should normally be SUBMODE_ALPHA
   * @return the text submode in which this method ends
   */
    fn  encode_text( input: &ECIInput,  startpos: i32,  count: i32,  sb: &StringBuilder,  initial_submode: i32) -> /*  throws WriterException */Result<i32, Rc<Exception>>   {
         let tmp: StringBuilder = StringBuilder::new(count);
         let mut submode: i32 = initial_submode;
         let mut idx: i32 = 0;
        while true {
            if input.is_e_c_i(startpos + idx) {
                ::encoding_e_c_i(&input.get_e_c_i_value(startpos + idx), &sb);
                idx += 1;
            } else {
                 let ch: char = input.char_at(startpos + idx);
                match submode {
                      SUBMODE_ALPHA => 
                         {
                            if ::is_alpha_upper(ch) {
                                if ch == ' ' {
                                    //space
                                    tmp.append(26 as char);
                                } else {
                                    tmp.append((ch - 65) as char);
                                }
                            } else {
                                if ::is_alpha_lower(ch) {
                                    submode = SUBMODE_LOWER;
                                    //ll
                                    tmp.append(27 as char);
                                    continue;
                                } else if ::is_mixed(ch) {
                                    submode = SUBMODE_MIXED;
                                    //ml
                                    tmp.append(28 as char);
                                    continue;
                                } else {
                                    //ps
                                    tmp.append(29 as char);
                                    tmp.append(PUNCTUATION[ch] as char);
                                    break;
                                }
                            }
                            break;
                        }
                      SUBMODE_LOWER => 
                         {
                            if ::is_alpha_lower(ch) {
                                if ch == ' ' {
                                    //space
                                    tmp.append(26 as char);
                                } else {
                                    tmp.append((ch - 97) as char);
                                }
                            } else {
                                if ::is_alpha_upper(ch) {
                                    //as
                                    tmp.append(27 as char);
                                    tmp.append((ch - 65) as char);
                                    //space cannot happen here, it is also in "Lower"
                                    break;
                                } else if ::is_mixed(ch) {
                                    submode = SUBMODE_MIXED;
                                    //ml
                                    tmp.append(28 as char);
                                    continue;
                                } else {
                                    //ps
                                    tmp.append(29 as char);
                                    tmp.append(PUNCTUATION[ch] as char);
                                    break;
                                }
                            }
                            break;
                        }
                      SUBMODE_MIXED => 
                         {
                            if ::is_mixed(ch) {
                                tmp.append(MIXED[ch] as char);
                            } else {
                                if ::is_alpha_upper(ch) {
                                    submode = SUBMODE_ALPHA;
                                    //al
                                    tmp.append(28 as char);
                                    continue;
                                } else if ::is_alpha_lower(ch) {
                                    submode = SUBMODE_LOWER;
                                    //ll
                                    tmp.append(27 as char);
                                    continue;
                                } else {
                                    if startpos + idx + 1 < count {
                                        if !input.is_e_c_i(startpos + idx + 1) && ::is_punctuation(&input.char_at(startpos + idx + 1)) {
                                            submode = SUBMODE_PUNCTUATION;
                                            //pl
                                            tmp.append(25 as char);
                                            continue;
                                        }
                                    }
                                    //ps
                                    tmp.append(29 as char);
                                    tmp.append(PUNCTUATION[ch] as char);
                                }
                            }
                            break;
                        }
                    _ => 
                         {
                            //SUBMODE_PUNCTUATION
                            if ::is_punctuation(ch) {
                                tmp.append(PUNCTUATION[ch] as char);
                            } else {
                                submode = SUBMODE_ALPHA;
                                //al
                                tmp.append(29 as char);
                                continue;
                            }
                        }
                }
                idx += 1;
                if idx >= count {
                    break;
                }
            }
        }
         let mut h: char = 0;
         let len: i32 = tmp.length();
         {
             let mut i: i32 = 0;
            while i < len {
                {
                     let odd: bool = (i % 2) != 0;
                    if odd {
                        h = ((h * 30) + tmp.char_at(i)) as char;
                        sb.append(h);
                    } else {
                        h = tmp.char_at(i);
                    }
                }
                i += 1;
             }
         }

        if (len % 2) != 0 {
            //ps
            sb.append(((h * 30) + 29) as char);
        }
        return Ok(submode);
    }

    /**
   * Encode all of the message using Byte Compaction as described in ISO/IEC 15438:2001(E)
   *
   * @param input     the input
   * @param startpos  the start position within the message
   * @param count     the number of bytes to encode
   * @param startmode the mode from which this method starts
   * @param sb        receives the encoded codewords
   */
    fn  encode_multi_e_c_i_binary( input: &ECIInput,  startpos: i32,  count: i32,  startmode: i32,  sb: &StringBuilder)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
         let end: i32 = Math::min(startpos + count, &input.length());
         let local_start: i32 = startpos;
        while true {
            //encode all leading ECIs and advance localStart
            while local_start < end && input.is_e_c_i(local_start) {
                ::encoding_e_c_i(&input.get_e_c_i_value(local_start), &sb);
                local_start += 1;
            }
             let local_end: i32 = local_start;
            //advance end until before the next ECI
            while local_end < end && !input.is_e_c_i(local_end) {
                local_end += 1;
            }
             let local_count: i32 = local_end - local_start;
            if local_count <= 0 {
                //done
                break;
            } else {
                //encode the segment
                ::encode_binary(&::sub_bytes(input, local_start, local_end), 0, local_count,  if local_start == startpos { startmode } else { BYTE_COMPACTION }, &sb);
                local_start = local_end;
            }
        }
    }

    fn  sub_bytes( input: &ECIInput,  start: i32,  end: i32) -> Vec<i8>  {
         let count: i32 = end - start;
         let mut result: [i8; count] = [0; count];
         {
             let mut i: i32 = start;
            while i < end {
                {
                    result[i - start] = (input.char_at(i) & 0xff) as i8;
                }
                i += 1;
             }
         }

        return result;
    }

    /**
   * Encode parts of the message using Byte Compaction as described in ISO/IEC 15438:2001(E),
   * chapter 4.4.3. The Unicode characters will be converted to binary using the cp437
   * codepage.
   *
   * @param bytes     the message converted to a byte array
   * @param startpos  the start position within the message
   * @param count     the number of bytes to encode
   * @param startmode the mode from which this method starts
   * @param sb        receives the encoded codewords
   */
    fn  encode_binary( bytes: &Vec<i8>,  startpos: i32,  count: i32,  startmode: i32,  sb: &StringBuilder)   {
        if count == 1 && startmode == TEXT_COMPACTION {
            sb.append(SHIFT_TO_BYTE as char);
        } else {
            if (count % 6) == 0 {
                sb.append(LATCH_TO_BYTE as char);
            } else {
                sb.append(LATCH_TO_BYTE_PADDED as char);
            }
        }
         let mut idx: i32 = startpos;
        // Encode sixpacks
        if count >= 6 {
             let mut chars: [Option<char>; 5] = [None; 5];
            while (startpos + count - idx) >= 6 {
                 let mut t: i64 = 0;
                 {
                     let mut i: i32 = 0;
                    while i < 6 {
                        {
                            t <<= 8;
                            t += bytes[idx + i] & 0xff;
                        }
                        i += 1;
                     }
                 }

                 {
                     let mut i: i32 = 0;
                    while i < 5 {
                        {
                            chars[i] = (t % 900) as char;
                            t /= 900;
                        }
                        i += 1;
                     }
                 }

                 {
                     let mut i: i32 = chars.len() - 1;
                    while i >= 0 {
                        {
                            sb.append(chars[i]);
                        }
                        i -= 1;
                     }
                 }

                idx += 6;
            }
        }
        //Encode rest (remaining n<5 bytes if any)
         {
             let mut i: i32 = idx;
            while i < startpos + count {
                {
                     let ch: i32 = bytes[i] & 0xff;
                    sb.append(ch as char);
                }
                i += 1;
             }
         }

    }

    fn  encode_numeric( input: &ECIInput,  startpos: i32,  count: i32,  sb: &StringBuilder)   {
         let mut idx: i32 = 0;
         let tmp: StringBuilder = StringBuilder::new(count / 3 + 1);
         let num900: BigInteger = BigInteger::value_of(900);
         let num0: BigInteger = BigInteger::value_of(0);
        while idx < count {
            tmp.set_length(0);
             let len: i32 = Math::min(44, count - idx);
             let part: String = format!("1{}", input.sub_sequence(startpos + idx, startpos + idx + len));
             let mut bigint: BigInteger = BigInteger::new(&part);
            loop { {
                tmp.append(bigint.mod(&num900).int_value() as char);
                bigint = bigint.divide(&num900);
            }if !(!bigint.equals(&num0)) break;}
            //Reverse temporary string
             {
                 let mut i: i32 = tmp.length() - 1;
                while i >= 0 {
                    {
                        sb.append(&tmp.char_at(i));
                    }
                    i -= 1;
                 }
             }

            idx += len;
        }
    }

    fn  is_digit( ch: char) -> bool  {
        return ch >= '0' && ch <= '9';
    }

    fn  is_alpha_upper( ch: char) -> bool  {
        return ch == ' ' || (ch >= 'A' && ch <= 'Z');
    }

    fn  is_alpha_lower( ch: char) -> bool  {
        return ch == ' ' || (ch >= 'a' && ch <= 'z');
    }

    fn  is_mixed( ch: char) -> bool  {
        return MIXED[ch] != -1;
    }

    fn  is_punctuation( ch: char) -> bool  {
        return PUNCTUATION[ch] != -1;
    }

    fn  is_text( ch: char) -> bool  {
        return ch == '\t' || ch == '\n' || ch == '\r' || (ch >= 32 && ch <= 126);
    }

    /**
   * Determines the number of consecutive characters that are encodable using numeric compaction.
   *
   * @param input      the input
   * @param startpos the start position within the input
   * @return the requested character count
   */
    fn  determine_consecutive_digit_count( input: &ECIInput,  startpos: i32) -> i32  {
         let mut count: i32 = 0;
         let len: i32 = input.length();
         let mut idx: i32 = startpos;
        if idx < len {
            while idx < len && !input.is_e_c_i(idx) && ::is_digit(&input.char_at(idx)) {
                count += 1;
                idx += 1;
            }
        }
        return count;
    }

    /**
   * Determines the number of consecutive characters that are encodable using text compaction.
   *
   * @param input      the input
   * @param startpos the start position within the input
   * @return the requested character count
   */
    fn  determine_consecutive_text_count( input: &ECIInput,  startpos: i32) -> i32  {
         let len: i32 = input.length();
         let mut idx: i32 = startpos;
        while idx < len {
             let numeric_count: i32 = 0;
            while numeric_count < 13 && idx < len && !input.is_e_c_i(idx) && ::is_digit(&input.char_at(idx)) {
                numeric_count += 1;
                idx += 1;
            }
            if numeric_count >= 13 {
                return idx - startpos - numeric_count;
            }
            if numeric_count > 0 {
                //Heuristic: All text-encodable chars or digits are binary encodable
                continue;
            }
            //Check if character is encodable
            if input.is_e_c_i(idx) || !::is_text(&input.char_at(idx)) {
                break;
            }
            idx += 1;
        }
        return idx - startpos;
    }

    /**
   * Determines the number of consecutive characters that are encodable using binary compaction.
   *
   * @param input    the input
   * @param startpos the start position within the message
   * @param encoding the charset used to convert the message to a byte array
   * @return the requested character count
   */
    fn  determine_consecutive_binary_count( input: &ECIInput,  startpos: i32,  encoding: &Charset) -> /*  throws WriterException */Result<i32, Rc<Exception>>   {
         let encoder: CharsetEncoder =  if encoding == null { null } else { encoding.new_encoder() };
         let len: i32 = input.length();
         let mut idx: i32 = startpos;
        while idx < len {
             let numeric_count: i32 = 0;
             let mut i: i32 = idx;
            while numeric_count < 13 && !input.is_e_c_i(i) && ::is_digit(&input.char_at(i)) {
                numeric_count += 1;
                //textCount++;
                i = idx + numeric_count;
                if i >= len {
                    break;
                }
            }
            if numeric_count >= 13 {
                return Ok(idx - startpos);
            }
            if encoder != null && !encoder.can_encode(&input.char_at(idx)) {
                assert!( input instanceof NoECIInput);
                 let ch: char = input.char_at(idx);
                throw WriterException::new(format!("Non-encodable character detected: {} (Unicode: {})", ch, ch as i32));
            }
            idx += 1;
        }
        return Ok(idx - startpos);
    }

    fn  encoding_e_c_i( eci: i32,  sb: &StringBuilder)  -> /*  throws WriterException */Result<Void, Rc<Exception>>   {
        if eci >= 0 && eci < 900 {
            sb.append(ECI_CHARSET as char);
            sb.append(eci as char);
        } else if eci < 810900 {
            sb.append(ECI_GENERAL_PURPOSE as char);
            sb.append((eci / 900 - 1) as char);
            sb.append((eci % 900) as char);
        } else if eci < 811800 {
            sb.append(ECI_USER_DEFINED as char);
            sb.append((810900 - eci) as char);
        } else {
            throw WriterException::new(format!("ECI number not in valid range from 0..811799, but was {}", eci));
        }
    }

    #[derive(ECIInput)]
    struct NoECIInput {

         let input: String;
    }
    
    impl NoECIInput {

        fn new( input: &String) -> NoECIInput {
            let .input = input;
        }

        pub fn  length(&self) -> i32  {
            return self.input.length();
        }

        pub fn  char_at(&self,  index: i32) -> char  {
            return self.input.char_at(index);
        }

        pub fn  is_e_c_i(&self,  index: i32) -> bool  {
            return false;
        }

        pub fn  get_e_c_i_value(&self,  index: i32) -> i32  {
            return -1;
        }

        pub fn  have_n_characters(&self,  index: i32,  n: i32) -> bool  {
            return index + n <= self.input.length();
        }

        pub fn  sub_sequence(&self,  start: i32,  end: i32) -> CharSequence  {
            return self.input.sub_sequence(start, end);
        }

        pub fn  to_string(&self) -> String  {
            return self.input;
        }
    }

}

