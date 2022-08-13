use crate::common::{BitArray,BitMatrix,CharacterSetECI};
use crate::common::reedsolomon{GenericGF,ReedSolomonEncoder};

// Token.java
const EMPTY: Token = SimpleToken::new(null, 0, 0);

pub trait Token {

    fn new( previous: &Token) -> Token {
        let .previous = previous;
    }

    fn  get_previous(&self) -> Token  {
        return self.previous;
    }

    fn  add(&self,  value: i32,  bit_count: i32) -> Token  {
        return SimpleToken::new(self, value, bit_count);
    }

    fn  add_binary_shift(&self,  start: i32,  byte_count: i32) -> Token  {
        //int bitCount = (byteCount * 8) + (byteCount <= 31 ? 10 : byteCount <= 62 ? 20 : 21);
        return BinaryShiftToken::new(self, start, byte_count);
    }

    fn  append_to(&self,  bit_array: &BitArray,  text: &Vec<i8>)  ;
}

// AztecCode.java
/**
 * Aztec 2D code representation
 *
 * @author Rustam Abdullaev
 */
pub struct AztecCode {

    let compact: bool;

    let size: i32;

    let layers: i32;

    let code_words: i32;

    let matrix: BitMatrix;
}

impl AztecCode {

   /**
  * @return {@code true} if compact instead of full mode
  */
   pub fn  is_compact(&self) -> bool  {
       return self.compact;
   }

   pub fn  set_compact(&self,  compact: bool)   {
       self.compact = compact;
   }

   /**
  * @return size in pixels (width and height)
  */
   pub fn  get_size(&self) -> i32  {
       return self.size;
   }

   pub fn  set_size(&self,  size: i32)   {
       self.size = size;
   }

   /**
  * @return number of levels
  */
   pub fn  get_layers(&self) -> i32  {
       return self.layers;
   }

   pub fn  set_layers(&self,  layers: i32)   {
       self.layers = layers;
   }

   /**
  * @return number of data codewords
  */
   pub fn  get_code_words(&self) -> i32  {
       return self.code_words;
   }

   pub fn  set_code_words(&self,  code_words: i32)   {
       self.codeWords = code_words;
   }

   /**
  * @return the symbol image
  */
   pub fn  get_matrix(&self) -> BitMatrix  {
       return self.matrix;
   }

   pub fn  set_matrix(&self,  matrix: &BitMatrix)   {
       self.matrix = matrix;
   }
}

// Encoder.java

/**
 * Generates Aztec 2D barcodes.
 *
 * @author Rustam Abdullaev
 */

// default minimal percentage of error check words
const DEFAULT_EC_PERCENT: i32 = 33;

const DEFAULT_AZTEC_LAYERS: i32 = 0;

const MAX_NB_BITS: i32 = 32;

const MAX_NB_BITS_COMPACT: i32 = 4;

const WORD_SIZE: vec![Vec<i32>; 33] = vec![4, 6, 6, 8, 8, 8, 8, 8, 8, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 12, 12, 12, 12, 12, 12, 12, 12, 12, 12, ]
;
pub struct Encoder {
}

impl Encoder {

   fn new() -> Encoder {
   }

   /**
  * Encodes the given string content as an Aztec symbol (without ECI code)
  *
  * @param data input data string; must be encodable as ISO/IEC 8859-1 (Latin-1)
  * @return Aztec symbol matrix with metadata
  */
   pub fn  encode( data: &String) -> AztecCode  {
       return ::encode(&data.get_bytes(StandardCharsets::ISO_8859_1));
   }

   /**
  * Encodes the given string content as an Aztec symbol (without ECI code)
  *
  * @param data input data string; must be encodable as ISO/IEC 8859-1 (Latin-1)
  * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
  *                      a minimum of 23% + 3 words is recommended)
  * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
  * @return Aztec symbol matrix with metadata
  */
   pub fn  encode( data: &String,  min_e_c_c_percent: i32,  user_specified_layers: i32) -> AztecCode  {
       return ::encode(&data.get_bytes(StandardCharsets::ISO_8859_1), min_e_c_c_percent, user_specified_layers, null);
   }

   /**
  * Encodes the given string content as an Aztec symbol
  *
  * @param data input data string
  * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
  *                      a minimum of 23% + 3 words is recommended)
  * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
  * @param charset character set in which to encode string using ECI; if null, no ECI code
  *                will be inserted, and the string must be encodable as ISO/IEC 8859-1
  *                (Latin-1), the default encoding of the symbol.
  * @return Aztec symbol matrix with metadata
  */
   pub fn  encode( data: &String,  min_e_c_c_percent: i32,  user_specified_layers: i32,  charset: &Charset) -> AztecCode  {
        let bytes: Vec<i8> = data.get_bytes( if null != charset { charset } else { StandardCharsets::ISO_8859_1 });
       return ::encode(&bytes, min_e_c_c_percent, user_specified_layers, &charset);
   }

   /**
  * Encodes the given binary content as an Aztec symbol (without ECI code)
  *
  * @param data input data string
  * @return Aztec symbol matrix with metadata
  */
   pub fn  encode( data: &Vec<i8>) -> AztecCode  {
       return ::encode(&data, DEFAULT_EC_PERCENT, DEFAULT_AZTEC_LAYERS, null);
   }

   /**
  * Encodes the given binary content as an Aztec symbol (without ECI code)
  *
  * @param data input data string
  * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
  *                      a minimum of 23% + 3 words is recommended)
  * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
  * @return Aztec symbol matrix with metadata
  */
   pub fn  encode( data: &Vec<i8>,  min_e_c_c_percent: i32,  user_specified_layers: i32) -> AztecCode  {
       return ::encode(&data, min_e_c_c_percent, user_specified_layers, null);
   }

   /**
  * Encodes the given binary content as an Aztec symbol
  *
  * @param data input data string
  * @param minECCPercent minimal percentage of error check words (According to ISO/IEC 24778:2008,
  *                      a minimum of 23% + 3 words is recommended)
  * @param userSpecifiedLayers if non-zero, a user-specified value for the number of layers
  * @param charset character set to mark using ECI; if null, no ECI code will be inserted, and the
  *                default encoding of ISO/IEC 8859-1 will be assuming by readers.
  * @return Aztec symbol matrix with metadata
  */
   pub fn  encode( data: &Vec<i8>,  min_e_c_c_percent: i32,  user_specified_layers: i32,  charset: &Charset) -> AztecCode  {
       // High-level encode
        let bits: BitArray = HighLevelEncoder::new(&data, &charset).encode();
       // stuff bits and choose symbol size
        let ecc_bits: i32 = bits.get_size() * min_e_c_c_percent / 100 + 11;
        let total_size_bits: i32 = bits.get_size() + ecc_bits;
        let mut compact: bool;
        let mut layers: i32;
        let total_bits_in_layer: i32;
        let word_size: i32;
        let stuffed_bits: BitArray;
       if user_specified_layers != DEFAULT_AZTEC_LAYERS {
           compact = user_specified_layers < 0;
           layers = Math::abs(user_specified_layers);
           if layers > ( if compact { MAX_NB_BITS_COMPACT } else { MAX_NB_BITS }) {
               throw IllegalArgumentException::new(&String::format("Illegal value %s for layers", user_specified_layers));
           }
           total_bits_in_layer = self.total_bits_in_layer(layers, compact);
           word_size = WORD_SIZE[layers];
            let usable_bits_in_layers: i32 = total_bits_in_layer - (total_bits_in_layer % word_size);
           stuffed_bits = ::stuff_bits(bits, word_size);
           if stuffed_bits.get_size() + ecc_bits > usable_bits_in_layers {
               throw IllegalArgumentException::new("Data to large for user specified layer");
           }
           if compact && stuffed_bits.get_size() > word_size * 64 {
               // Compact format only allows 64 data words, though C4 can hold more words than that
               throw IllegalArgumentException::new("Data to large for user specified layer");
           }
       } else {
           word_size = 0;
           stuffed_bits = null;
           // is the same size, but has more data.
            {
                let mut i: i32 = 0;
               loop  {
                   {
                       if i > MAX_NB_BITS {
                           throw IllegalArgumentException::new("Data too large for an Aztec code");
                       }
                       compact = i <= 3;
                       layers =  if compact { i + 1 } else { i };
                       total_bits_in_layer = self.total_bits_in_layer(layers, compact);
                       if total_size_bits > total_bits_in_layer {
                           continue;
                       }
                       // wordSize has changed
                       if stuffed_bits == null || word_size != WORD_SIZE[layers] {
                           word_size = WORD_SIZE[layers];
                           stuffed_bits = ::stuff_bits(bits, word_size);
                       }
                        let usable_bits_in_layers: i32 = total_bits_in_layer - (total_bits_in_layer % word_size);
                       if compact && stuffed_bits.get_size() > word_size * 64 {
                           // Compact format only allows 64 data words, though C4 can hold more words than that
                           continue;
                       }
                       if stuffed_bits.get_size() + ecc_bits <= usable_bits_in_layers {
                           break;
                       }
                   }
                   i += 1;
                }
            }

       }
        let message_bits: BitArray = ::generate_check_words(stuffed_bits, total_bits_in_layer, word_size);
       // generate mode message
        let message_size_in_words: i32 = stuffed_bits.get_size() / word_size;
        let mode_message: BitArray = ::generate_mode_message(compact, layers, message_size_in_words);
       // allocate symbol
       // not including alignment lines
        let base_matrix_size: i32 = ( if compact { 11 } else { 14 }) + layers * 4;
        let alignment_map: [i32; base_matrix_size] = [0; base_matrix_size];
        let matrix_size: i32;
       if compact {
           // no alignment marks in compact mode, alignmentMap is a no-op
           matrix_size = base_matrix_size;
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
           matrix_size = base_matrix_size + 1 + 2 * ((base_matrix_size / 2 - 1) / 15);
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
        let matrix: BitMatrix = BitMatrix::new(matrix_size);
       // draw data bits
        {
            let mut i: i32 = 0, let row_offset: i32 = 0;
           while i < layers {
               {
                    let row_size: i32 = (layers - i) * 4 + ( if compact { 9 } else { 12 });
                    {
                        let mut j: i32 = 0;
                       while j < row_size {
                           {
                                let column_offset: i32 = j * 2;
                                {
                                    let mut k: i32 = 0;
                                   while k < 2 {
                                       {
                                           if message_bits.get(row_offset + column_offset + k) {
                                               matrix.set(alignment_map[i * 2 + k], alignment_map[i * 2 + j]);
                                           }
                                           if message_bits.get(row_offset + row_size * 2 + column_offset + k) {
                                               matrix.set(alignment_map[i * 2 + j], alignment_map[base_matrix_size - 1 - i * 2 - k]);
                                           }
                                           if message_bits.get(row_offset + row_size * 4 + column_offset + k) {
                                               matrix.set(alignment_map[base_matrix_size - 1 - i * 2 - k], alignment_map[base_matrix_size - 1 - i * 2 - j]);
                                           }
                                           if message_bits.get(row_offset + row_size * 6 + column_offset + k) {
                                               matrix.set(alignment_map[base_matrix_size - 1 - i * 2 - j], alignment_map[i * 2 + k]);
                                           }
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

       // draw mode message
       ::draw_mode_message(matrix, compact, matrix_size, mode_message);
       // draw alignment marks
       if compact {
           ::draw_bulls_eye(matrix, matrix_size / 2, 5);
       } else {
           ::draw_bulls_eye(matrix, matrix_size / 2, 7);
            {
                let mut i: i32 = 0, let mut j: i32 = 0;
               while i < base_matrix_size / 2 - 1 {
                   {
                        {
                            let mut k: i32 = (matrix_size / 2) & 1;
                           while k < matrix_size {
                               {
                                   matrix.set(matrix_size / 2 - j, k);
                                   matrix.set(matrix_size / 2 + j, k);
                                   matrix.set(k, matrix_size / 2 - j);
                                   matrix.set(k, matrix_size / 2 + j);
                               }
                               k += 2;
                            }
                        }

                   }
                   i += 15;
                   j += 16;
                }
            }

       }
        let aztec: AztecCode = AztecCode::new();
       aztec.set_compact(compact);
       aztec.set_size(matrix_size);
       aztec.set_layers(layers);
       aztec.set_code_words(message_size_in_words);
       aztec.set_matrix(matrix);
       return aztec;
   }

   fn  draw_bulls_eye( matrix: &BitMatrix,  center: i32,  size: i32)   {
        {
            let mut i: i32 = 0;
           while i < size {
               {
                    {
                        let mut j: i32 = center - i;
                       while j <= center + i {
                           {
                               matrix.set(j, center - i);
                               matrix.set(j, center + i);
                               matrix.set(center - i, j);
                               matrix.set(center + i, j);
                           }
                           j += 1;
                        }
                    }

               }
               i += 2;
            }
        }

       matrix.set(center - size, center - size);
       matrix.set(center - size + 1, center - size);
       matrix.set(center - size, center - size + 1);
       matrix.set(center + size, center - size);
       matrix.set(center + size, center - size + 1);
       matrix.set(center + size, center + size - 1);
   }

   fn  generate_mode_message( compact: bool,  layers: i32,  message_size_in_words: i32) -> BitArray  {
        let mode_message: BitArray = BitArray::new();
       if compact {
           mode_message.append_bits(layers - 1, 2);
           mode_message.append_bits(message_size_in_words - 1, 6);
           mode_message = ::generate_check_words(mode_message, 28, 4);
       } else {
           mode_message.append_bits(layers - 1, 5);
           mode_message.append_bits(message_size_in_words - 1, 11);
           mode_message = ::generate_check_words(mode_message, 40, 4);
       }
       return mode_message;
   }

   fn  draw_mode_message( matrix: &BitMatrix,  compact: bool,  matrix_size: i32,  mode_message: &BitArray)   {
        let center: i32 = matrix_size / 2;
       if compact {
            {
                let mut i: i32 = 0;
               while i < 7 {
                   {
                        let offset: i32 = center - 3 + i;
                       if mode_message.get(i) {
                           matrix.set(offset, center - 5);
                       }
                       if mode_message.get(i + 7) {
                           matrix.set(center + 5, offset);
                       }
                       if mode_message.get(20 - i) {
                           matrix.set(offset, center + 5);
                       }
                       if mode_message.get(27 - i) {
                           matrix.set(center - 5, offset);
                       }
                   }
                   i += 1;
                }
            }

       } else {
            {
                let mut i: i32 = 0;
               while i < 10 {
                   {
                        let offset: i32 = center - 5 + i + i / 5;
                       if mode_message.get(i) {
                           matrix.set(offset, center - 7);
                       }
                       if mode_message.get(i + 10) {
                           matrix.set(center + 7, offset);
                       }
                       if mode_message.get(29 - i) {
                           matrix.set(offset, center + 7);
                       }
                       if mode_message.get(39 - i) {
                           matrix.set(center - 7, offset);
                       }
                   }
                   i += 1;
                }
            }

       }
   }

   fn  generate_check_words( bit_array: &BitArray,  total_bits: i32,  word_size: i32) -> BitArray  {
       // bitArray is guaranteed to be a multiple of the wordSize, so no padding needed
        let message_size_in_words: i32 = bit_array.get_size() / word_size;
        let rs: ReedSolomonEncoder = ReedSolomonEncoder::new(&::get_g_f(word_size));
        let total_words: i32 = total_bits / word_size;
        let message_words: Vec<i32> = ::bits_to_words(bit_array, word_size, total_words);
       rs.encode(&message_words, total_words - message_size_in_words);
        let start_pad: i32 = total_bits % word_size;
        let message_bits: BitArray = BitArray::new();
       message_bits.append_bits(0, start_pad);
       for  let message_word: i32 in message_words {
           message_bits.append_bits(message_word, word_size);
       }
       return message_bits;
   }

   fn  bits_to_words( stuffed_bits: &BitArray,  word_size: i32,  total_words: i32) -> Vec<i32>  {
        let mut message: [i32; total_words] = [0; total_words];
        let mut i: i32;
        let mut n: i32;
        {
           i = 0;
           n = stuffed_bits.get_size() / word_size;
           while i < n {
               {
                    let mut value: i32 = 0;
                    {
                        let mut j: i32 = 0;
                       while j < word_size {
                           {
                               value |=  if stuffed_bits.get(i * word_size + j) { (1 << word_size - j - 1) } else { 0 };
                           }
                           j += 1;
                        }
                    }

                   message[i] = value;
               }
               i += 1;
            }
        }

       return message;
   }

   fn  get_g_f( word_size: i32) -> GenericGF  {
       match word_size {
             4 => 
                {
                   return GenericGF::AZTEC_PARAM;
               }
             6 => 
                {
                   return GenericGF::AZTEC_DATA_6;
               }
             8 => 
                {
                   return GenericGF::AZTEC_DATA_8;
               }
             10 => 
                {
                   return GenericGF::AZTEC_DATA_10;
               }
             12 => 
                {
                   return GenericGF::AZTEC_DATA_12;
               }
           _ => 
                {
                   throw IllegalArgumentException::new(format!("Unsupported word size {}", word_size));
               }
       }
   }

   fn  stuff_bits( bits: &BitArray,  word_size: i32) -> BitArray  {
        let out: BitArray = BitArray::new();
        let n: i32 = bits.get_size();
        let mask: i32 = (1 << word_size) - 2;
        {
            let mut i: i32 = 0;
           while i < n {
               {
                    let mut word: i32 = 0;
                    {
                        let mut j: i32 = 0;
                       while j < word_size {
                           {
                               if i + j >= n || bits.get(i + j) {
                                   word |= 1 << (word_size - 1 - j);
                               }
                           }
                           j += 1;
                        }
                    }

                   if (word & mask) == mask {
                       out.append_bits(word & mask, word_size);
                       i -= 1;
                   } else if (word & mask) == 0 {
                       out.append_bits(word | 1, word_size);
                       i -= 1;
                   } else {
                       out.append_bits(word, word_size);
                   }
               }
               i += word_size;
            }
        }

       return out;
   }

   fn  total_bits_in_layer( layers: i32,  compact: bool) -> i32  {
       return (( if compact { 88 } else { 112 }) + 16 * layers) * layers;
   }
}

// BinaryShiftToken.java
struct BinaryShiftToken {
    super: Token;

     let binary_shift_start: i32;

     let binary_shift_byte_count: i32;
}

impl Token for BinaryShiftToken {
    pub fn  append_to(&self,  bit_array: &BitArray,  text: &Vec<i8>)   {
        let bsbc: i32 = self.binary_shift_byte_count;
        {
            let mut i: i32 = 0;
           while i < bsbc {
               {
                   if i == 0 || (i == 31 && bsbc <= 62) {
                       // We need a header before the first character, and before
                       // character 31 when the total byte code is <= 62
                       // BINARY_SHIFT
                       bit_array.append_bits(31, 5);
                       if bsbc > 62 {
                           bit_array.append_bits(bsbc - 31, 16);
                       } else if i == 0 {
                           // 1 <= binaryShiftByteCode <= 62
                           bit_array.append_bits(&Math::min(bsbc, 31), 5);
                       } else {
                           // 32 <= binaryShiftCount <= 62 and i == 31
                           bit_array.append_bits(bsbc - 31, 5);
                       }
                   }
                   bit_array.append_bits(text[self.binary_shift_start + i], 8);
               }
               i += 1;
            }
        }

   }

   pub fn  to_string(&self) -> String  {
       return format!("<{}::{}>", self.binary_shift_start, (self.binary_shift_start + self.binary_shift_byte_count - 1));
   }
}

impl BinaryShiftToken {

    fn new( previous: &Token,  binary_shift_start: i32,  binary_shift_byte_count: i32) -> BinaryShiftToken {
        super(previous);
        let .binaryShiftStart = binary_shift_start;
        let .binaryShiftByteCount = binary_shift_byte_count;
    }

    
}

// HighLevelEncoder.java
/**
 * This produces nearly optimal encodings of text into the first-level of
 * encoding used by Aztec code.
 *
 * It uses a dynamic algorithm.  For each prefix of the string, it determines
 * a set of encodings that could lead to this prefix.  We repeatedly add a
 * character and generate a new set of optimal encodings until we have read
 * through the entire input.
 *
 * @author Frank Yellin
 * @author Rustam Abdullaev
 */

const MODE_NAMES: vec![Vec<String>; 5] = vec!["UPPER", "LOWER", "DIGIT", "MIXED", "PUNCT", ]
;

// 5 bits
 const MODE_UPPER: i32 = 0;

// 5 bits
 const MODE_LOWER: i32 = 1;

// 4 bits
 const MODE_DIGIT: i32 = 2;

// 5 bits
 const MODE_MIXED: i32 = 3;

// 5 bits
 const MODE_PUNCT: i32 = 4;

// The Latch Table shows, for each pair of Modes, the optimal method for
// getting from one mode to another.  In the worst possible case, this can
// be up to 14 bits.  In the best possible case, we are already there!
// The high half-word of each entry gives the number of bits.
// The low half-word of each entry are the actual bits necessary to change
 const LATCH_TABLE: vec![vec![Vec<Vec<i32>>; 5]; 5] = vec![vec![0, // UPPER -> LOWER
(5 << 16) + 28, // UPPER -> DIGIT
(5 << 16) + 30, // UPPER -> MIXED
(5 << 16) + 29, // UPPER -> MIXED -> PUNCT
(10 << 16) + (29 << 5) + 30, ]
, vec![// LOWER -> DIGIT -> UPPER
(9 << 16) + (30 << 4) + 14, 0, // LOWER -> DIGIT
(5 << 16) + 30, // LOWER -> MIXED
(5 << 16) + 29, // LOWER -> MIXED -> PUNCT
(10 << 16) + (29 << 5) + 30, ]
, vec![// DIGIT -> UPPER
(4 << 16) + 14, // DIGIT -> UPPER -> LOWER
(9 << 16) + (14 << 5) + 28, 0, // DIGIT -> UPPER -> MIXED
(9 << 16) + (14 << 5) + 29, (14 << 16) + (14 << 10) + (29 << 5) + 30, ]
, vec![// MIXED -> UPPER
(5 << 16) + 29, // MIXED -> LOWER
(5 << 16) + 28, // MIXED -> UPPER -> DIGIT
(10 << 16) + (29 << 5) + 30, 0, // MIXED -> PUNCT
(5 << 16) + 30, ]
, vec![// PUNCT -> UPPER
(5 << 16) + 31, // PUNCT -> UPPER -> LOWER
(10 << 16) + (31 << 5) + 28, // PUNCT -> UPPER -> DIGIT
(10 << 16) + (31 << 5) + 30, // PUNCT -> UPPER -> MIXED
(10 << 16) + (31 << 5) + 29, 0, ]
, ]
;

// A reverse mapping from [mode][char] to the encoding for that character
// in that mode.  An entry of 0 indicates no mapping exists.
 const CHAR_MAP: [[i32; 256]; 5] = [[0; 256]; 5];

// A map showing the available shift codes.  (The shifts to BINARY are not
// shown
// mode shift codes, per table
 const SHIFT_TABLE: [[i32; 6]; 6] = [[0; 6]; 6];
pub struct HighLevelEncoder {

     let text: Vec<i8>;

     let mut charset: Charset;
}

impl HighLevelEncoder {

    static {
        CHAR_MAP[MODE_UPPER][' '] = 1;
         {
             let mut c: i32 = 'A';
            while c <= 'Z' {
                {
                    CHAR_MAP[MODE_UPPER][c] = c - 'A' + 2;
                }
                c += 1;
             }
         }

        CHAR_MAP[MODE_LOWER][' '] = 1;
         {
             let mut c: i32 = 'a';
            while c <= 'z' {
                {
                    CHAR_MAP[MODE_LOWER][c] = c - 'a' + 2;
                }
                c += 1;
             }
         }

        CHAR_MAP[MODE_DIGIT][' '] = 1;
         {
             let mut c: i32 = '0';
            while c <= '9' {
                {
                    CHAR_MAP[MODE_DIGIT][c] = c - '0' + 2;
                }
                c += 1;
             }
         }

        CHAR_MAP[MODE_DIGIT][','] = 12;
        CHAR_MAP[MODE_DIGIT]['.'] = 13;
         let mixed_table: vec![Vec<i32>; 28] = vec!['\0', ' ', '\1', '\2', '\3', '\4', '\5', '\6', '\7', '\b', '\t', '\n', '\13', '\f', '\r', '\33', '\34', '\35', '\36', '\37', '@', '\\', '^', '_', '`', '|', '~', '\177', ]
        ;
         {
             let mut i: i32 = 0;
            while i < mixed_table.len() {
                {
                    CHAR_MAP[MODE_MIXED][mixed_table[i]] = i;
                }
                i += 1;
             }
         }

         let punct_table: vec![Vec<i32>; 31] = vec!['\0', '\r', '\0', '\0', '\0', '\0', '!', '\'', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '[', ']', '{', '}', ]
        ;
         {
             let mut i: i32 = 0;
            while i < punct_table.len() {
                {
                    if punct_table[i] > 0 {
                        CHAR_MAP[MODE_PUNCT][punct_table[i]] = i;
                    }
                }
                i += 1;
             }
         }

    }

    static {
        for  let table: Vec<i32> in SHIFT_TABLE {
            Arrays::fill(&table, -1);
        }
        SHIFT_TABLE[MODE_UPPER][MODE_PUNCT] = 0;
        SHIFT_TABLE[MODE_LOWER][MODE_PUNCT] = 0;
        SHIFT_TABLE[MODE_LOWER][MODE_UPPER] = 28;
        SHIFT_TABLE[MODE_MIXED][MODE_PUNCT] = 0;
        SHIFT_TABLE[MODE_DIGIT][MODE_PUNCT] = 0;
        SHIFT_TABLE[MODE_DIGIT][MODE_UPPER] = 15;
    }

    pub fn new( text: &Vec<i8>) -> HighLevelEncoder {
        let .text = text;
        let .charset = null;
    }

    pub fn new( text: &Vec<i8>,  charset: &Charset) -> HighLevelEncoder {
        let .text = text;
        let .charset = charset;
    }

    /**
   * @return text represented by this encoder encoded as a {@link BitArray}
   */
    pub fn  encode(&self) -> BitArray  {
         let initial_state: State = State::INITIAL_STATE;
        if self.charset != null {
             let eci: CharacterSetECI = CharacterSetECI::get_character_set_e_c_i(&self.charset);
            if null == eci {
                throw IllegalArgumentException::new(format!("No ECI code for character set {}", self.charset));
            }
            initial_state = initial_state.append_f_l_gn(&eci.get_value());
        }
         let mut states: Collection<State> = Collections::singleton_list(initial_state);
         {
             let mut index: i32 = 0;
            while index < self.text.len() {
                {
                     let pair_code: i32;
                     let next_char: i32 =  if index + 1 < self.text.len() { self.text[index + 1] } else { 0 };
                    match self.text[index] {
                          '\r' => 
                             {
                                pair_code =  if next_char == '\n' { 2 } else { 0 };
                                break;
                            }
                          '.' => 
                             {
                                pair_code =  if next_char == ' ' { 3 } else { 0 };
                                break;
                            }
                          ',' => 
                             {
                                pair_code =  if next_char == ' ' { 4 } else { 0 };
                                break;
                            }
                          ':' => 
                             {
                                pair_code =  if next_char == ' ' { 5 } else { 0 };
                                break;
                            }
                        _ => 
                             {
                                pair_code = 0;
                            }
                    }
                    if pair_code > 0 {
                        // We have one of the four special PUNCT pairs.  Treat them specially.
                        // Get a new set of states for the two new characters.
                        states = ::update_state_list_for_pair(&states, index, pair_code);
                        index += 1;
                    } else {
                        // Get a new set of states for the new character.
                        states = self.update_state_list_for_char(&states, index);
                    }
                }
                index += 1;
             }
         }

        // We are left with a set of states.  Find the shortest one.
         let min_state: State = Collections::min(&states, Comparator<State>::new() {

            pub fn  compare(&self,  a: &State,  b: &State) -> i32  {
                return a.get_bit_count() - b.get_bit_count();
            }
        });
        // Convert it to a bit array, and return.
        return min_state.to_bit_array(&self.text);
    }

    // We update a set of states for a new character by updating each state
    // for the new character, merging the results, and then removing the
    // non-optimal states.
    fn  update_state_list_for_char(&self,  states: &Iterable<State>,  index: i32) -> Collection<State>  {
         let result: Collection<State> = LinkedList<>::new();
        for  let state: State in states {
            self.update_state_for_char(state, index, &result);
        }
        return ::simplify_states(&result);
    }

    // Return a set of states that represent the possible ways of updating this
    // state for the next character.  The resulting set of states are added to
    // the "result" list.
    fn  update_state_for_char(&self,  state: &State,  index: i32,  result: &Collection<State>)   {
         let ch: char = (self.text[index] & 0xFF) as char;
         let char_in_current_table: bool = CHAR_MAP[state.get_mode()][ch] > 0;
         let state_no_binary: State = null;
         {
             let mut mode: i32 = 0;
            while mode <= MODE_PUNCT {
                {
                     let char_in_mode: i32 = CHAR_MAP[mode][ch];
                    if char_in_mode > 0 {
                        if state_no_binary == null {
                            // Only create stateNoBinary the first time it's required.
                            state_no_binary = state.end_binary_shift(index);
                        }
                        // Try generating the character by latching to its mode
                        if !char_in_current_table || mode == state.get_mode() || mode == MODE_DIGIT {
                            // If the character is in the current table, we don't want to latch to
                            // any other mode except possibly digit (which uses only 4 bits).  Any
                            // other latch would be equally successful *after* this character, and
                            // so wouldn't save any bits.
                             let latch_state: State = state_no_binary.latch_and_append(mode, char_in_mode);
                            result.add(latch_state);
                        }
                        // Try generating the character by switching to its mode.
                        if !char_in_current_table && SHIFT_TABLE[state.get_mode()][mode] >= 0 {
                            // It never makes sense to temporarily shift to another mode if the
                            // character exists in the current mode.  That can never save bits.
                             let shift_state: State = state_no_binary.shift_and_append(mode, char_in_mode);
                            result.add(shift_state);
                        }
                    }
                }
                mode += 1;
             }
         }

        if state.get_binary_shift_byte_count() > 0 || CHAR_MAP[state.get_mode()][ch] == 0 {
            // It's never worthwhile to go into binary shift mode if you're not already
            // in binary shift mode, and the character exists in your current mode.
            // That can never save bits over just outputting the char in the current mode.
             let binary_state: State = state.add_binary_shift_char(index);
            result.add(binary_state);
        }
    }

    fn  update_state_list_for_pair( states: &Iterable<State>,  index: i32,  pair_code: i32) -> Collection<State>  {
         let result: Collection<State> = LinkedList<>::new();
        for  let state: State in states {
            ::update_state_for_pair(state, index, pair_code, &result);
        }
        return ::simplify_states(&result);
    }

    fn  update_state_for_pair( state: &State,  index: i32,  pair_code: i32,  result: &Collection<State>)   {
         let state_no_binary: State = state.end_binary_shift(index);
        // Possibility 1.  Latch to MODE_PUNCT, and then append this code
        result.add(&state_no_binary.latch_and_append(MODE_PUNCT, pair_code));
        if state.get_mode() != MODE_PUNCT {
            // Possibility 2.  Shift to MODE_PUNCT, and then append this code.
            // Every state except MODE_PUNCT (handled above) can shift
            result.add(&state_no_binary.shift_and_append(MODE_PUNCT, pair_code));
        }
        if pair_code == 3 || pair_code == 4 {
            // both characters are in DIGITS.  Sometimes better to just add two digits
             let digit_state: State = state_no_binary.latch_and_append(MODE_DIGIT, // period or comma in DIGIT
            16 - pair_code).latch_and_append(MODE_DIGIT, // space in DIGIT
            1);
            result.add(digit_state);
        }
        if state.get_binary_shift_byte_count() > 0 {
            // It only makes sense to do the characters as binary if we're already
            // in binary mode.
             let binary_state: State = state.add_binary_shift_char(index).add_binary_shift_char(index + 1);
            result.add(binary_state);
        }
    }

    fn  simplify_states( states: &Iterable<State>) -> Collection<State>  {
         let result: Deque<State> = LinkedList<>::new();
        for  let new_state: State in states {
             let mut add: bool = true;
             {
                 let iterator: Iterator<State> = result.iterator();
                while iterator.has_next(){
                     let old_state: State = iterator.next();
                    if old_state.is_better_than_or_equal_to(new_state) {
                        add = false;
                        break;
                    }
                    if new_state.is_better_than_or_equal_to(old_state) {
                        iterator.remove();
                    }
                }
             }

            if add {
                result.add_first(new_state);
            }
        }
        return result;
    }
}

// SimpleToken.java
struct SimpleToken {
    super: Token;

    // For normal words, indicates value and bitCount
     let value: i16;

     let bit_count: i16;
}

impl Token for SimpleToken {
    fn  append_to(&self,  bit_array: &BitArray,  text: &Vec<i8>)   {
        bit_array.append_bits(self.value, self.bit_count);
    }

    pub fn  to_string(&self) -> String  {
         let mut value: i32 = self.value & ((1 << self.bit_count) - 1);
        value |= 1 << self.bit_count;
        return '<' + Integer::to_binary_string(value | (1 << self.bit_count))::substring(1) + '>';
    }
}

impl SimpleToken {

    fn new( previous: &Token,  value: i32,  bit_count: i32) -> SimpleToken {
        super(previous);
        let .value = value as i16;
        let .bitCount = bit_count as i16;
    }

}

// State.java

/**
 * State represents all information about a sequence necessary to generate the current output.
 * Note that a state is immutable.
 */

const INITIAL_STATE: State = State::new(Token::EMPTY, HighLevelEncoder::MODE_UPPER, 0, 0);
struct State {

    // The current mode of the encoding (or the mode to which we'll return if
    // we're in Binary Shift mode.
     let mode: i32;

    // The list of tokens that we output.  If we are in Binary Shift mode, this
    // token list does *not* yet included the token for those bytes
     let token: Token;

    // If non-zero, the number of most recent bytes that should be output
    // in Binary Shift mode.
     let binary_shift_byte_count: i32;

    // The total number of bits generated (including Binary Shift).
     let bit_count: i32;

     let binary_shift_cost: i32;
}

impl State {

    fn new( token: &Token,  mode: i32,  binary_bytes: i32,  bit_count: i32) -> State {
        let .token = token;
        let .mode = mode;
        let .binaryShiftByteCount = binary_bytes;
        let .bitCount = bit_count;
        let .binaryShiftCost = ::calculate_binary_shift_cost(binary_bytes);
    }

    fn  get_mode(&self) -> i32  {
        return self.mode;
    }

    fn  get_token(&self) -> Token  {
        return self.token;
    }

    fn  get_binary_shift_byte_count(&self) -> i32  {
        return self.binary_shift_byte_count;
    }

    fn  get_bit_count(&self) -> i32  {
        return self.bit_count;
    }

    fn  append_f_l_gn(&self,  eci: i32) -> State  {
        // 0: FLG(n)
         let result: State = self.shift_and_append(HighLevelEncoder::MODE_PUNCT, 0);
         let mut token: Token = result.token;
         let bits_added: i32 = 3;
        if eci < 0 {
            // 0: FNC1
            token = token.add(0, 3);
        } else if eci > 999999 {
            throw IllegalArgumentException::new("ECI code must be between 0 and 999999");
        } else {
             let eci_digits: Vec<i8> = Integer::to_string(eci)::get_bytes(StandardCharsets::ISO_8859_1);
            // 1-6: number of ECI digits
            token = token.add(eci_digits.len(), 3);
            for  let eci_digit: i8 in eci_digits {
                token = token.add(eci_digit - '0' + 2, 4);
            }
            bits_added += eci_digits.len() * 4;
        }
        return State::new(token, self.mode, 0, self.bit_count + bits_added);
    }

    // Create a new state representing this state with a latch to a (not
    // necessary different) mode, and then a code.
    fn  latch_and_append(&self,  mode: i32,  value: i32) -> State  {
         let bit_count: i32 = self.bitCount;
         let mut token: Token = self.token;
        if mode != self.mode {
             let latch: i32 = HighLevelEncoder::LATCH_TABLE[self.mode][mode];
            token = token.add(latch & 0xFFFF, latch >> 16);
            bit_count += latch >> 16;
        }
         let latch_mode_bit_count: i32 =  if mode == HighLevelEncoder::MODE_DIGIT { 4 } else { 5 };
        token = token.add(value, latch_mode_bit_count);
        return State::new(token, mode, 0, bit_count + latch_mode_bit_count);
    }

    // Create a new state representing this state, with a temporary shift
    // to a different mode to output a single value.
    fn  shift_and_append(&self,  mode: i32,  value: i32) -> State  {
         let mut token: Token = self.token;
         let this_mode_bit_count: i32 =  if self.mode == HighLevelEncoder::MODE_DIGIT { 4 } else { 5 };
        // Shifts exist only to UPPER and PUNCT, both with tokens size 5.
        token = token.add(HighLevelEncoder::SHIFT_TABLE[self.mode][mode], this_mode_bit_count);
        token = token.add(value, 5);
        return State::new(token, self.mode, 0, self.bitCount + this_mode_bit_count + 5);
    }

    // Create a new state representing this state, but an additional character
    // output in Binary Shift mode.
    fn  add_binary_shift_char(&self,  index: i32) -> State  {
         let mut token: Token = self.token;
         let mut mode: i32 = self.mode;
         let bit_count: i32 = self.bitCount;
        if self.mode == HighLevelEncoder::MODE_PUNCT || self.mode == HighLevelEncoder::MODE_DIGIT {
             let latch: i32 = HighLevelEncoder::LATCH_TABLE[mode][HighLevelEncoder::MODE_UPPER];
            token = token.add(latch & 0xFFFF, latch >> 16);
            bit_count += latch >> 16;
            mode = HighLevelEncoder::MODE_UPPER;
        }
         let delta_bit_count: i32 =  if (self.binary_shift_byte_count == 0 || self.binary_shift_byte_count == 31) { 18 } else {  if (self.binary_shift_byte_count == 62) { 9 } else { 8 } };
         let mut result: State = State::new(token, mode, self.binary_shift_byte_count + 1, bit_count + delta_bit_count);
        if result.binaryShiftByteCount == 2047 + 31 {
            // The string is as long as it's allowed to be.  We should end it.
            result = result.end_binary_shift(index + 1);
        }
        return result;
    }

    // Create the state identical to this one, but we are no longer in
    // Binary Shift mode.
    fn  end_binary_shift(&self,  index: i32) -> State  {
        if self.binary_shift_byte_count == 0 {
            return self;
        }
         let mut token: Token = self.token;
        token = token.add_binary_shift(index - self.binary_shift_byte_count, self.binary_shift_byte_count);
        return State::new(token, self.mode, 0, self.bitCount);
    }

    // Returns true if "this" state is better (or equal) to be in than "that"
    // state under all possible circumstances.
    fn  is_better_than_or_equal_to(&self,  other: &State) -> bool  {
         let new_mode_bit_count: i32 = self.bitCount + (HighLevelEncoder::LATCH_TABLE[self.mode][other.mode] >> 16);
        if self.binaryShiftByteCount < other.binaryShiftByteCount {
            // add additional B/S encoding cost of other, if any
            new_mode_bit_count += other.binaryShiftCost - self.binaryShiftCost;
        } else if self.binaryShiftByteCount > other.binaryShiftByteCount && other.binaryShiftByteCount > 0 {
            // maximum possible additional cost (we end up exceeding the 31 byte boundary and other state can stay beneath it)
            new_mode_bit_count += 10;
        }
        return new_mode_bit_count <= other.bitCount;
    }

    fn  to_bit_array(&self,  text: &Vec<i8>) -> BitArray  {
         let symbols: List<Token> = ArrayList<>::new();
         {
             let mut token: Token = self.end_binary_shift(text.len()).token;
            while token != null {
                {
                    symbols.add(token);
                }
                token = token.get_previous();
             }
         }

         let bit_array: BitArray = BitArray::new();
        // Add each token to the result in forward order
         {
             let mut i: i32 = symbols.size() - 1;
            while i >= 0 {
                {
                    symbols.get(i).append_to(bit_array, &text);
                }
                i -= 1;
             }
         }

        return bit_array;
    }

    pub fn  to_string(&self) -> String  {
        return String::format("%s bits=%d bytes=%d", HighLevelEncoder::MODE_NAMES[self.mode], self.bit_count, self.binary_shift_byte_count);
    }

    fn  calculate_binary_shift_cost( binary_shift_byte_count: i32) -> i32  {
        if binary_shift_byte_count > 62 {
            // B/S with extended length
            return 21;
        }
        if binary_shift_byte_count > 31 {
            // two B/S
            return 20;
        }
        if binary_shift_byte_count > 0 {
            // one B/S
            return 10;
        }
        return 0;
    }
}
