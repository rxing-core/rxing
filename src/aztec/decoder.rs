use create::FormatException;
use crate::aztec::AztecDetectorResult;
use crate::common::{BitMatrix,CharacterSetECI,DecoderResult};
use crate::common::reedsolomon::{GenericGF,ReedSolomonDecoder,ReedSolomonException};

/**
 * <p>The main class which implements Aztec Code decoding -- as opposed to locating and extracting
 * the Aztec Code from an image.</p>
 *
 * @author David Olivier
 */

const UPPER_TABLE: vec![Vec<String>; 32] = vec!["CTRL_PS", " ", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "CTRL_LL", "CTRL_ML", "CTRL_DL", "CTRL_BS", ]
;

 const LOWER_TABLE: vec![Vec<String>; 32] = vec!["CTRL_PS", " ", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "CTRL_US", "CTRL_ML", "CTRL_DL", "CTRL_BS", ]
;

 const MIXED_TABLE: vec![Vec<String>; 32] = vec!["CTRL_PS", " ", "\1", "\2", "\3", "\4", "\5", "\6", "\7", "\b", "\t", "\n", "\13", "\f", "\r", "\33", "\34", "\35", "\36", "\37", "@", "\\", "^", "_", "`", "|", "~", "\177", "CTRL_LL", "CTRL_UL", "CTRL_PL", "CTRL_BS", ]
;

 const PUNCT_TABLE: vec![Vec<String>; 32] = vec!["FLG(n)", "\r", "\r\n", ". ", ", ", ": ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "[", "]", "{", "}", "CTRL_UL", ]
;

 const DIGIT_TABLE: vec![Vec<String>; 16] = vec!["CTRL_PS", " ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ",", ".", "CTRL_UL", "CTRL_US", ]
;

 const DEFAULT_ENCODING: Charset = StandardCharsets::ISO_8859_1;
pub struct Decoder {

     let mut ddata: AztecDetectorResult;
}

impl Decoder {

    enum Table {

        UPPER(), LOWER(), MIXED(), DIGIT(), PUNCT(), BINARY()
    }

    pub fn  decode(&self,  detector_result: &AztecDetectorResult) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
        self.ddata = detector_result;
         let matrix: BitMatrix = detector_result.get_bits();
         let rawbits: Vec<bool> = self.extract_bits(matrix);
         let corrected_bits: CorrectedBitsResult = self.correct_bits(&rawbits);
         let raw_bytes: Vec<i8> = ::convert_bool_array_to_byte_array(corrected_bits.correctBits);
         let result: String = ::get_encoded_data(corrected_bits.correctBits);
         let decoder_result: DecoderResult = DecoderResult::new(&raw_bytes, &result, null, &String::format("%d%%", corrected_bits.ecLevel));
        decoder_result.set_num_bits(corrected_bits.correctBits.len());
        return Ok(decoder_result);
    }

    // This method is used for testing the high-level encoder
    pub fn  high_level_decode( corrected_bits: &Vec<bool>) -> /*  throws FormatException */Result<String, Rc<Exception>>   {
        return Ok(::get_encoded_data(&corrected_bits));
    }

    /**
   * Gets the string encoded in the aztec code bits
   *
   * @return the decoded string
   */
    fn  get_encoded_data( corrected_bits: &Vec<bool>) -> /*  throws FormatException */Result<String, Rc<Exception>>   {
         let end_index: i32 = corrected_bits.len();
        // table most recently latched to
         let latch_table: Table = Table::UPPER;
        // table to use for the next read
         let shift_table: Table = Table::UPPER;
        // Final decoded string result
        // (correctedBits-5) / 4 is an upper bound on the size (all-digit result)
         let result: StringBuilder = StringBuilder::new((corrected_bits.len() - 5) / 4);
        // Intermediary buffer of decoded bytes, which is decoded into a string and flushed
        // when character encoding changes (ECI) or input ends.
         let decoded_bytes: ByteArrayOutputStream = ByteArrayOutputStream::new();
         let mut encoding: Charset = DEFAULT_ENCODING;
         let mut index: i32 = 0;
        while index < end_index {
            if shift_table == Table::BINARY {
                if end_index - index < 5 {
                    break;
                }
                 let mut length: i32 = ::read_code(&corrected_bits, index, 5);
                index += 5;
                if length == 0 {
                    if end_index - index < 11 {
                        break;
                    }
                    length = ::read_code(&corrected_bits, index, 11) + 31;
                    index += 11;
                }
                 {
                     let char_count: i32 = 0;
                    while char_count < length {
                        {
                            if end_index - index < 8 {
                                // Force outer loop to exit
                                index = end_index;
                                break;
                            }
                             let code: i32 = ::read_code(&corrected_bits, index, 8);
                            decoded_bytes.write(code as i8);
                            index += 8;
                        }
                        char_count += 1;
                     }
                 }

                // Go back to whatever mode we had been in
                shift_table = latch_table;
            } else {
                 let size: i32 =  if shift_table == Table::DIGIT { 4 } else { 5 };
                if end_index - index < size {
                    break;
                }
                 let code: i32 = ::read_code(&corrected_bits, index, size);
                index += size;
                 let str: String = ::get_character(shift_table, code);
                if "FLG(n)".equals(&str) {
                    if end_index - index < 3 {
                        break;
                    }
                     let mut n: i32 = ::read_code(&corrected_bits, index, 3);
                    index += 3;
                    //  flush bytes, FLG changes state
                    let tryResult1 = 0;
                    'try1: loop {
                    {
                        result.append(&decoded_bytes.to_string(&encoding.name()));
                    }
                    break 'try1
                    }
                    match tryResult1 {
                         catch ( uee: &UnsupportedEncodingException) {
                            throw IllegalStateException::new(&uee);
                        }  0 => break
                    }

                    decoded_bytes.reset();
                    match n {
                          0 => 
                             {
                                // translate FNC1 as ASCII 29
                                result.append(29 as char);
                                break;
                            }
                          7 => 
                             {
                                // FLG(7) is reserved and illegal
                                throw FormatException::get_format_instance();
                            }
                        _ => 
                             {
                                // ECI is decimal integer encoded as 1-6 codes in DIGIT mode
                                 let mut eci: i32 = 0;
                                if end_index - index < 4 * n {
                                    break;
                                }
                                while n -= 1 !!!check!!! post decrement > 0 {
                                     let next_digit: i32 = ::read_code(&corrected_bits, index, 4);
                                    index += 4;
                                    if next_digit < 2 || next_digit > 11 {
                                        // Not a decimal digit
                                        throw FormatException::get_format_instance();
                                    }
                                    eci = eci * 10 + (next_digit - 2);
                                }
                                 let charset_e_c_i: CharacterSetECI = CharacterSetECI::get_character_set_e_c_i_by_value(eci);
                                if charset_e_c_i == null {
                                    throw FormatException::get_format_instance();
                                }
                                encoding = charset_e_c_i.get_charset();
                            }
                    }
                    // Go back to whatever mode we had been in
                    shift_table = latch_table;
                } else if str.starts_with("CTRL_") {
                    // Table changes
                    // ISO/IEC 24778:2008 prescribes ending a shift sequence in the mode from which it was invoked.
                    // That's including when that mode is a shift.
                    // Our test case dlusbs.png for issue #642 exercises that.
                    // Latch the current mode, so as to return to Upper after U/S B/S
                    latch_table = shift_table;
                    shift_table = ::get_table(&str.char_at(5));
                    if str.char_at(6) == 'L' {
                        latch_table = shift_table;
                    }
                } else {
                    // Though stored as a table of strings for convenience, codes actually represent 1 or 2 *bytes*.
                     let b: Vec<i8> = str.get_bytes(StandardCharsets::US_ASCII);
                    decoded_bytes.write(&b, 0, b.len());
                    // Go back to whatever mode we had been in
                    shift_table = latch_table;
                }
            }
        }
        let tryResult1 = 0;
        'try1: loop {
        {
            result.append(&decoded_bytes.to_string(&encoding.name()));
        }
        break 'try1
        }
        match tryResult1 {
             catch ( uee: &UnsupportedEncodingException) {
                throw IllegalStateException::new(&uee);
            }  0 => break
        }

        return Ok(result.to_string());
    }

    /**
   * gets the table corresponding to the char passed
   */
    fn  get_table( t: char) -> Table  {
        match t {
              'L' => 
                 {
                    return Table::LOWER;
                }
              'P' => 
                 {
                    return Table::PUNCT;
                }
              'M' => 
                 {
                    return Table::MIXED;
                }
              'D' => 
                 {
                    return Table::DIGIT;
                }
              'B' => 
                 {
                    return Table::BINARY;
                }
              'U' => 
                 {
                }
            _ => 
                 {
                    return Table::UPPER;
                }
        }
    }

    /**
   * Gets the character (or string) corresponding to the passed code in the given table
   *
   * @param table the table used
   * @param code the code of the character
   */
    fn  get_character( table: &Table,  code: i32) -> String  {
        match table {
              UPPER => 
                 {
                    return UPPER_TABLE[code];
                }
              LOWER => 
                 {
                    return LOWER_TABLE[code];
                }
              MIXED => 
                 {
                    return MIXED_TABLE[code];
                }
              PUNCT => 
                 {
                    return PUNCT_TABLE[code];
                }
              DIGIT => 
                 {
                    return DIGIT_TABLE[code];
                }
            _ => 
                 {
                    // Should not reach here.
                    throw IllegalStateException::new("Bad table");
                }
        }
    }

    struct CorrectedBitsResult {

         let correct_bits: Vec<bool>;

         let ec_level: i32;
    }
    
    impl CorrectedBitsResult {

        fn new( correct_bits: &Vec<bool>,  ec_level: i32) -> CorrectedBitsResult {
            let .correctBits = correct_bits;
            let .ecLevel = ec_level;
        }
    }


    /**
   * <p>Performs RS error correction on an array of bits.</p>
   *
   * @return the corrected array
   * @throws FormatException if the input contains too many errors
   */
    fn  correct_bits(&self,  rawbits: &Vec<bool>) -> /*  throws FormatException */Result<CorrectedBitsResult, Rc<Exception>>   {
         let mut gf: GenericGF;
         let codeword_size: i32;
        if self.ddata.get_nb_layers() <= 2 {
            codeword_size = 6;
            gf = GenericGF::AZTEC_DATA_6;
        } else if self.ddata.get_nb_layers() <= 8 {
            codeword_size = 8;
            gf = GenericGF::AZTEC_DATA_8;
        } else if self.ddata.get_nb_layers() <= 22 {
            codeword_size = 10;
            gf = GenericGF::AZTEC_DATA_10;
        } else {
            codeword_size = 12;
            gf = GenericGF::AZTEC_DATA_12;
        }
         let num_data_codewords: i32 = self.ddata.get_nb_datablocks();
         let num_codewords: i32 = rawbits.len() / codeword_size;
        if num_codewords < num_data_codewords {
            throw FormatException::get_format_instance();
        }
         let mut offset: i32 = rawbits.len() % codeword_size;
         let data_words: [i32; num_codewords] = [0; num_codewords];
         {
             let mut i: i32 = 0;
            while i < num_codewords {
                {
                    data_words[i] = ::read_code(&rawbits, offset, codeword_size);
                }
                i += 1;
                offset += codeword_size;
             }
         }

        let tryResult1 = 0;
        'try1: loop {
        {
             let rs_decoder: ReedSolomonDecoder = ReedSolomonDecoder::new(gf);
            rs_decoder.decode(&data_words, num_codewords - num_data_codewords);
        }
        break 'try1
        }
        match tryResult1 {
             catch ( ex: &ReedSolomonException) {
                throw FormatException::get_format_instance(ex);
            }  0 => break
        }

        // Now perform the unstuffing operation.
        // First, count how many bits are going to be thrown out as stuffing
         let mask: i32 = (1 << codeword_size) - 1;
         let stuffed_bits: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < num_data_codewords {
                {
                     let data_word: i32 = data_words[i];
                    if data_word == 0 || data_word == mask {
                        throw FormatException::get_format_instance();
                    } else if data_word == 1 || data_word == mask - 1 {
                        stuffed_bits += 1;
                    }
                }
                i += 1;
             }
         }

        // Now, actually unpack the bits and remove the stuffing
         let corrected_bits: [bool; num_data_codewords * codeword_size - stuffed_bits] = [false; num_data_codewords * codeword_size - stuffed_bits];
         let mut index: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < num_data_codewords {
                {
                     let data_word: i32 = data_words[i];
                    if data_word == 1 || data_word == mask - 1 {
                        // next codewordSize-1 bits are all zeros or all ones
                        Arrays::fill(&corrected_bits, index, index + codeword_size - 1, data_word > 1);
                        index += codeword_size - 1;
                    } else {
                         {
                             let mut bit: i32 = codeword_size - 1;
                            while bit >= 0 {
                                {
                                    corrected_bits[index += 1 !!!check!!! post increment] = (data_word & (1 << bit)) != 0;
                                }
                                bit -= 1;
                             }
                         }

                    }
                }
                i += 1;
             }
         }

        return Ok(CorrectedBitsResult::new(&corrected_bits, 100 * (num_codewords - num_data_codewords) / num_codewords));
    }

    /**
   * Gets the array of bits from an Aztec Code matrix
   *
   * @return the array of bits
   */
    fn  extract_bits(&self,  matrix: &BitMatrix) -> Vec<bool>  {
         let compact: bool = self.ddata.is_compact();
         let layers: i32 = self.ddata.get_nb_layers();
        // not including alignment lines
         let base_matrix_size: i32 = ( if compact { 11 } else { 14 }) + layers * 4;
         let alignment_map: [i32; base_matrix_size] = [0; base_matrix_size];
         let mut rawbits: [bool; ::total_bits_in_layer(layers, compact)] = [false; ::total_bits_in_layer(layers, compact)];
        if compact {
             {
                 let mut i: i32 = 0;
                while i < alignment_map.len() {
                    {
                        alignment_map[i] = i;
                    }
                    i += 1;
                 }
             }

        } else {
             let matrix_size: i32 = base_matrix_size + 1 + 2 * ((base_matrix_size / 2 - 1) / 15);
             let orig_center: i32 = base_matrix_size / 2;
             let center: i32 = matrix_size / 2;
             {
                 let mut i: i32 = 0;
                while i < orig_center {
                    {
                         let new_offset: i32 = i + i / 15;
                        alignment_map[orig_center - i - 1] = center - new_offset - 1;
                        alignment_map[orig_center + i] = center + new_offset + 1;
                    }
                    i += 1;
                 }
             }

        }
         {
             let mut i: i32 = 0, let row_offset: i32 = 0;
            while i < layers {
                {
                     let row_size: i32 = (layers - i) * 4 + ( if compact { 9 } else { 12 });
                    // The top-left most point of this layer is <low, low> (not including alignment lines)
                     let low: i32 = i * 2;
                    // The bottom-right most point of this layer is <high, high> (not including alignment lines)
                     let high: i32 = base_matrix_size - 1 - low;
                    // We pull bits from the two 2 x rowSize columns and two rowSize x 2 rows
                     {
                         let mut j: i32 = 0;
                        while j < row_size {
                            {
                                 let column_offset: i32 = j * 2;
                                 {
                                     let mut k: i32 = 0;
                                    while k < 2 {
                                        {
                                            // left column
                                            rawbits[row_offset + column_offset + k] = matrix.get(alignment_map[low + k], alignment_map[low + j]);
                                            // bottom row
                                            rawbits[row_offset + 2 * row_size + column_offset + k] = matrix.get(alignment_map[low + j], alignment_map[high - k]);
                                            // right column
                                            rawbits[row_offset + 4 * row_size + column_offset + k] = matrix.get(alignment_map[high - k], alignment_map[high - j]);
                                            // top row
                                            rawbits[row_offset + 6 * row_size + column_offset + k] = matrix.get(alignment_map[high - j], alignment_map[low + k]);
                                        }
                                        k += 1;
                                     }
                                 }

                            }
                            j += 1;
                         }
                     }

                    row_offset += row_size * 8;
                }
                i += 1;
             }
         }

        return rawbits;
    }

    /**
   * Reads a code of given length and at given index in an array of bits
   */
    fn  read_code( rawbits: &Vec<bool>,  start_index: i32,  length: i32) -> i32  {
         let mut res: i32 = 0;
         {
             let mut i: i32 = start_index;
            while i < start_index + length {
                {
                    res <<= 1;
                    if rawbits[i] {
                        res |= 0x01;
                    }
                }
                i += 1;
             }
         }

        return res;
    }

    /**
   * Reads a code of length 8 in an array of bits, padding with zeros
   */
    fn  read_byte( rawbits: &Vec<bool>,  start_index: i32) -> i8  {
         let n: i32 = rawbits.len() - start_index;
        if n >= 8 {
            return ::read_code(&rawbits, start_index, 8) as i8;
        }
        return (::read_code(&rawbits, start_index, n) << (8 - n)) as i8;
    }

    /**
   * Packs a bit array into bytes, most significant bit first
   */
    fn  convert_bool_array_to_byte_array( bool_arr: &Vec<bool>) -> Vec<i8>  {
         let byte_arr: [i8; (bool_arr.len() + 7) / 8] = [0; (bool_arr.len() + 7) / 8];
         {
             let mut i: i32 = 0;
            while i < byte_arr.len() {
                {
                    byte_arr[i] = ::read_byte(&bool_arr, 8 * i);
                }
                i += 1;
             }
         }

        return byte_arr;
    }

    fn  total_bits_in_layer( layers: i32,  compact: bool) -> i32  {
        return (( if compact { 88 } else { 112 }) + 16 * layers) * layers;
    }
}

