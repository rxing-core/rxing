/*
 * Copyright 2006-2007 Jeremias Maerki.
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
// package com::google::zxing::datamatrix::encoder;

struct TextEncoder {
    super: C40Encoder;
}

impl TextEncoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::TEXT_ENCODATION;
    }

    fn  encode_char(&self,  c: char,  sb: &StringBuilder) -> i32  {
        if c == ' ' {
            sb.append('\3');
            return 1;
        }
        if c >= '0' && c <= '9' {
            sb.append((c - 48 + 4) as char);
            return 1;
        }
        if c >= 'a' && c <= 'z' {
            sb.append((c - 97 + 14) as char);
            return 1;
        }
        if c < ' ' {
            //Shift 1 Set
            sb.append('\0');
            sb.append(c);
            return 2;
        }
        if c <= '/' {
            //Shift 2 Set
            sb.append('\1');
            sb.append((c - 33) as char);
            return 2;
        }
        if c <= '@' {
            //Shift 2 Set
            sb.append('\1');
            sb.append((c - 58 + 15) as char);
            return 2;
        }
        if c >= '[' && c <= '_' {
            //Shift 2 Set
            sb.append('\1');
            sb.append((c - 91 + 22) as char);
            return 2;
        }
        if c == '`' {
            //Shift 3 Set
            sb.append('\2');
            // '`' - 96 == 0
            sb.append(0 as char);
            return 2;
        }
        if c <= 'Z' {
            //Shift 3 Set
            sb.append('\2');
            sb.append((c - 65 + 1) as char);
            return 2;
        }
        if c <= 127 {
            //Shift 3 Set
            sb.append('\2');
            sb.append((c - 123 + 27) as char);
            return 2;
        }
        //Shift 2, Upper Shift
        sb.append("\1");
         let mut len: i32 = 2;
        len += self.encode_char((c - 128) as char, &sb);
        return len;
    }
}

