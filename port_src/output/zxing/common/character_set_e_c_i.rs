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
// package com::google::zxing::common;

/**
 * Encapsulates a Character Set ECI, according to "Extended Channel Interpretations" 5.3.1.1
 * of ISO 18004.
 *
 * @author Sean Owen
 */
pub enum CharacterSetECI {

    // Enum name is a Java encoding valid for java.lang and java.io
    Cp437( : vec![i32; 2] = vec![0, 2, ]
    ), ISO8859_1( : vec![i32; 2] = vec![1, 3, ]
    , "ISO-8859-1"), ISO8859_2(4, "ISO-8859-2"), ISO8859_3(5, "ISO-8859-3"), ISO8859_4(6, "ISO-8859-4"), ISO8859_5(7, "ISO-8859-5"), // ISO8859_6(8, "ISO-8859-6"),
    ISO8859_7(9, "ISO-8859-7"), // ISO8859_8(10, "ISO-8859-8"),
    ISO8859_9(11, "ISO-8859-9"), // ISO8859_11(13, "ISO-8859-11"),
    ISO8859_13(15, "ISO-8859-13"), // ISO8859_14(16, "ISO-8859-14"),
    ISO8859_15(17, "ISO-8859-15"), ISO8859_16(18, "ISO-8859-16"), SJIS(20, "Shift_JIS"), Cp1250(21, "windows-1250"), Cp1251(22, "windows-1251"), Cp1252(23, "windows-1252"), Cp1256(24, "windows-1256"), UnicodeBigUnmarked(25, "UTF-16BE", "UnicodeBig"), UTF8(26, "UTF-8"), ASCII( : vec![i32; 2] = vec![27, 170, ]
    , "US-ASCII"), Big5(28), GB18030(29, "GB2312", "EUC_CN", "GBK"), EUC_KR(30, "EUC-KR");

     const VALUE_TO_ECI: Map<Integer, CharacterSetECI> = HashMap<>::new();

     const NAME_TO_ECI: Map<String, CharacterSetECI> = HashMap<>::new();

    static {
        for  let eci: CharacterSetECI in self.values() {
            for  let value: i32 in eci.values {
                VALUE_TO_ECI::put(value, eci);
            }
            NAME_TO_ECI::put(&eci.name(), eci);
            for  let name: String in eci.otherEncodingNames {
                NAME_TO_ECI::put(&name, eci);
            }
        }
    }

     let mut values: Vec<i32>;

     let other_encoding_names: Vec<String>;

    fn new( value: i32) -> CharacterSetECI {
        this( : vec![i32; 1] = vec![value, ]
        );
    }

    fn new( value: i32,  other_encoding_names: &String) -> CharacterSetECI {
        let .values =  : vec![i32; 1] = vec![value, ]
        ;
        let .otherEncodingNames = other_encoding_names;
    }

    fn new( values: &Vec<i32>,  other_encoding_names: &String) -> CharacterSetECI {
        let .values = values;
        let .otherEncodingNames = other_encoding_names;
    }

    pub fn  get_value(&self) -> i32  {
        return self.values[0];
    }

    pub fn  get_charset(&self) -> Charset  {
        return Charset::for_name(&name());
    }

    /**
   * @param charset Java character set object
   * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
   *   but unsupported
   */
    pub fn  get_character_set_e_c_i( charset: &Charset) -> CharacterSetECI  {
        return NAME_TO_ECI::get(&charset.name());
    }

    /**
   * @param value character set ECI value
   * @return {@code CharacterSetECI} representing ECI of given value, or null if it is legal but
   *   unsupported
   * @throws FormatException if ECI value is invalid
   */
    pub fn  get_character_set_e_c_i_by_value( value: i32) -> /*  throws FormatException */Result<CharacterSetECI, Rc<Exception>>   {
        if value < 0 || value >= 900 {
            throw FormatException::get_format_instance();
        }
        return Ok(VALUE_TO_ECI::get(value));
    }

    /**
   * @param name character set ECI encoding name
   * @return CharacterSetECI representing ECI for character encoding, or null if it is legal
   *   but unsupported
   */
    pub fn  get_character_set_e_c_i_by_name( name: &String) -> CharacterSetECI  {
        return NAME_TO_ECI::get(&name);
    }
}
