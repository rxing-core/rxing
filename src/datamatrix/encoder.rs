use crate::Dimension;
use crate::common::MinimalECIInput;

// ASCIIEncoder.java

struct ASCIIEncoder {
}

impl Encoder for ASCIIEncoder {}

impl ASCIIEncoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::ASCII_ENCODATION;
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
        //step B
         let n: i32 = HighLevelEncoder::determine_consecutive_digit_count(&context.get_message(), context.pos);
        if n >= 2 {
            context.write_codeword(&::encode_a_s_c_i_i_digits(&context.get_message().char_at(context.pos), &context.get_message().char_at(context.pos + 1)));
            context.pos += 2;
        } else {
             let c: char = context.get_current_char();
             let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
            if new_mode != self.get_encoding_mode() {
                match new_mode {
                      HighLevelEncoder::BASE256_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_BASE256);
                            context.signal_encoder_change(HighLevelEncoder::BASE256_ENCODATION);
                            return;
                        }
                      HighLevelEncoder::C40_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_C40);
                            context.signal_encoder_change(HighLevelEncoder::C40_ENCODATION);
                            return;
                        }
                      HighLevelEncoder::X12_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_ANSIX12);
                            context.signal_encoder_change(HighLevelEncoder::X12_ENCODATION);
                            break;
                        }
                      HighLevelEncoder::TEXT_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_TEXT);
                            context.signal_encoder_change(HighLevelEncoder::TEXT_ENCODATION);
                            break;
                        }
                      HighLevelEncoder::EDIFACT_ENCODATION => 
                         {
                            context.write_codeword(HighLevelEncoder::LATCH_TO_EDIFACT);
                            context.signal_encoder_change(HighLevelEncoder::EDIFACT_ENCODATION);
                            break;
                        }
                    _ => 
                         {
                            throw IllegalStateException::new(format!("Illegal mode: {}", new_mode));
                        }
                }
            } else if HighLevelEncoder::is_extended_a_s_c_i_i(c) {
                context.write_codeword(HighLevelEncoder::UPPER_SHIFT);
                context.write_codeword((c - 128 + 1) as char);
                context.pos += 1;
            } else {
                context.write_codeword((c + 1) as char);
                context.pos += 1;
            }
        }
    }

    fn  encode_a_s_c_i_i_digits( digit1: char,  digit2: char) -> char  {
        if HighLevelEncoder::is_digit(digit1) && HighLevelEncoder::is_digit(digit2) {
             let num: i32 = (digit1 - 48) * 10 + (digit2 - 48);
            return (num + 130) as char;
        }
        throw IllegalArgumentException::new(format!("not digits: {}{}", digit1, digit2));
    }
}


// Base256Encoder.java
struct Base256Encoder {
}

impl Encoder for Base256Encoder {}

impl Base256Encoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::BASE256_ENCODATION;
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
         let buffer: StringBuilder = StringBuilder::new();
        //Initialize length field
        buffer.append('\0');
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            buffer.append(c);
            context.pos += 1;
             let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
            if new_mode != self.get_encoding_mode() {
                // Return to ASCII encodation, which will actually handle latch to new mode
                context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
                break;
            }
        }
         let data_count: i32 = buffer.length() - 1;
         let length_field_size: i32 = 1;
         let current_size: i32 = context.get_codeword_count() + data_count + length_field_size;
        context.update_symbol_info(current_size);
         let must_pad: bool = (context.get_symbol_info().get_data_capacity() - current_size) > 0;
        if context.has_more_characters() || must_pad {
            if data_count <= 249 {
                buffer.set_char_at(0, data_count as char);
            } else if data_count <= 1555 {
                buffer.set_char_at(0, ((data_count / 250) + 249) as char);
                buffer.insert(1, (data_count % 250) as char);
            } else {
                throw IllegalStateException::new(format!("Message length not in valid ranges: {}", data_count));
            }
        }
         {
             let mut i: i32 = 0, let c: i32 = buffer.length();
            while i < c {
                {
                    context.write_codeword(&::randomize255_state(&buffer.char_at(i), context.get_codeword_count() + 1));
                }
                i += 1;
             }
         }

    }

    fn  randomize255_state( ch: char,  codeword_position: i32) -> char  {
         let pseudo_random: i32 = ((149 * codeword_position) % 255) + 1;
         let temp_variable: i32 = ch + pseudo_random;
        if temp_variable <= 255 {
            return temp_variable as char;
        } else {
            return (temp_variable - 256) as char;
        }
    }
}


// C40Encoder.java
struct C40Encoder {
}

impl Encoder for C40Encoder{}

impl C40Encoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::C40_ENCODATION;
    }

    fn  encode_maximal(&self,  context: &EncoderContext)   {
         let buffer: StringBuilder = StringBuilder::new();
         let last_char_size: i32 = 0;
         let backtrack_start_position: i32 = context.pos;
         let backtrack_buffer_length: i32 = 0;
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            context.pos += 1;
            last_char_size = self.encode_char(c, &buffer);
            if buffer.length() % 3 == 0 {
                backtrack_start_position = context.pos;
                backtrack_buffer_length = buffer.length();
            }
        }
        if backtrack_buffer_length != buffer.length() {
             let unwritten: i32 = (buffer.length() / 3) * 2;
            // +1 for the latch to C40
             let cur_codeword_count: i32 = context.get_codeword_count() + unwritten + 1;
            context.update_symbol_info(cur_codeword_count);
             let available: i32 = context.get_symbol_info().get_data_capacity() - cur_codeword_count;
             let rest: i32 = buffer.length() % 3;
            if (rest == 2 && available != 2) || (rest == 1 && (last_char_size > 3 || available != 1)) {
                buffer.set_length(backtrack_buffer_length);
                context.pos = backtrack_start_position;
            }
        }
        if buffer.length() > 0 {
            context.write_codeword(HighLevelEncoder::LATCH_TO_C40);
        }
        self.handle_e_o_d(context, &buffer);
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
        //step C
         let buffer: StringBuilder = StringBuilder::new();
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            context.pos += 1;
             let last_char_size: i32 = self.encode_char(c, &buffer);
             let unwritten: i32 = (buffer.length() / 3) * 2;
             let cur_codeword_count: i32 = context.get_codeword_count() + unwritten;
            context.update_symbol_info(cur_codeword_count);
             let available: i32 = context.get_symbol_info().get_data_capacity() - cur_codeword_count;
            if !context.has_more_characters() {
                //Avoid having a single C40 value in the last triplet
                 let removed: StringBuilder = StringBuilder::new();
                if (buffer.length() % 3) == 2 && available != 2 {
                    last_char_size = self.backtrack_one_character(context, &buffer, &removed, last_char_size);
                }
                while (buffer.length() % 3) == 1 && (last_char_size > 3 || available != 1) {
                    last_char_size = self.backtrack_one_character(context, &buffer, &removed, last_char_size);
                }
                break;
            }
             let count: i32 = buffer.length();
            if (count % 3) == 0 {
                 let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
                if new_mode != self.get_encoding_mode() {
                    // Return to ASCII encodation, which will actually handle latch to new mode
                    context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
                    break;
                }
            }
        }
        self.handle_e_o_d(context, &buffer);
    }

    fn  backtrack_one_character(&self,  context: &EncoderContext,  buffer: &StringBuilder,  removed: &StringBuilder,  last_char_size: i32) -> i32  {
         let count: i32 = buffer.length();
        buffer.delete(count - last_char_size, count);
        context.pos -= 1;
         let c: char = context.get_current_char();
        last_char_size = self.encode_char(c, &removed);
        //Deal with possible reduction in symbol size
        context.reset_symbol_info();
        return last_char_size;
    }

    fn  write_next_triplet( context: &EncoderContext,  buffer: &StringBuilder)   {
        context.write_codewords(&::encode_to_codewords(&buffer));
        buffer.delete(0, 3);
    }

    /**
   * Handle "end of data" situations
   *
   * @param context the encoder context
   * @param buffer  the buffer with the remaining encoded characters
   */
    fn  handle_e_o_d(&self,  context: &EncoderContext,  buffer: &StringBuilder)   {
         let unwritten: i32 = (buffer.length() / 3) * 2;
         let rest: i32 = buffer.length() % 3;
         let cur_codeword_count: i32 = context.get_codeword_count() + unwritten;
        context.update_symbol_info(cur_codeword_count);
         let available: i32 = context.get_symbol_info().get_data_capacity() - cur_codeword_count;
        if rest == 2 {
            //Shift 1
            buffer.append('\0');
            while buffer.length() >= 3 {
                ::write_next_triplet(context, &buffer);
            }
            if context.has_more_characters() {
                context.write_codeword(HighLevelEncoder::C40_UNLATCH);
            }
        } else if available == 1 && rest == 1 {
            while buffer.length() >= 3 {
                ::write_next_triplet(context, &buffer);
            }
            if context.has_more_characters() {
                context.write_codeword(HighLevelEncoder::C40_UNLATCH);
            }
            // else no unlatch
            context.pos -= 1;
        } else if rest == 0 {
            while buffer.length() >= 3 {
                ::write_next_triplet(context, &buffer);
            }
            if available > 0 || context.has_more_characters() {
                context.write_codeword(HighLevelEncoder::C40_UNLATCH);
            }
        } else {
            throw IllegalStateException::new("Unexpected case. Please report!");
        }
        context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
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
        if c >= 'A' && c <= 'Z' {
            sb.append((c - 65 + 14) as char);
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
        if c <= '_' {
            //Shift 2 Set
            sb.append('\1');
            sb.append((c - 91 + 22) as char);
            return 2;
        }
        if c <= 127 {
            //Shift 3 Set
            sb.append('\2');
            sb.append((c - 96) as char);
            return 2;
        }
        //Shift 2, Upper Shift
        sb.append("\1");
         let mut len: i32 = 2;
        len += self.encode_char((c - 128) as char, &sb);
        return len;
    }

    fn  encode_to_codewords( sb: &CharSequence) -> String  {
         let v: i32 = (1600 * sb.char_at(0)) + (40 * sb.char_at(1)) + sb.char_at(2) + 1;
         let cw1: char = (v / 256) as char;
         let cw2: char = (v % 256) as char;
        return String::new( : vec![char; 2] = vec![cw1, cw2, ]
        );
    }
}


// DataMatrixSymbolInfo144.java
struct DataMatrixSymbolInfo144 {
    super: SymbolInfo;
}

impl SymbolInfo for DataMatrixSymbolInfo144{}

impl DataMatrixSymbolInfo144 {

    fn new() -> DataMatrixSymbolInfo144 {
        super(false, 1558, 620, 22, 22, 36, -1, 62);
    }

    pub fn  get_interleaved_block_count(&self) -> i32  {
        return 10;
    }

    pub fn  get_data_length_for_interleaved_block(&self,  index: i32) -> i32  {
        return  if (index <= 8) { 156 } else { 155 };
    }
}

// DefaultPlacement.java
/**
 * Symbol Character Placement Program. Adapted from Annex M.1 in ISO/IEC 16022:2000(E).
 */
pub struct DefaultPlacement {

    let codewords: CharSequence;

    let numrows: i32;

    let mut numcols: i32;

    let mut bits: Vec<i8>;
}

impl DefaultPlacement {

   /**
  * Main constructor
  *
  * @param codewords the codewords to place
  * @param numcols   the number of columns
  * @param numrows   the number of rows
  */
   pub fn new( codewords: &CharSequence,  numcols: i32,  numrows: i32) -> DefaultPlacement {
       let .codewords = codewords;
       let .numcols = numcols;
       let .numrows = numrows;
       let .bits = : [i8; numcols * numrows] = [0; numcols * numrows];
       //Initialize with "not set" value
       Arrays::fill(let .bits, -1 as i8);
   }

   fn  get_numrows(&self) -> i32  {
       return self.numrows;
   }

   fn  get_numcols(&self) -> i32  {
       return self.numcols;
   }

   fn  get_bits(&self) -> Vec<i8>  {
       return self.bits;
   }

   pub fn  get_bit(&self,  col: i32,  row: i32) -> bool  {
       return self.bits[row * self.numcols + col] == 1;
   }

   fn  set_bit(&self,  col: i32,  row: i32,  bit: bool)   {
       self.bits[row * self.numcols + col] = ( if bit { 1 } else { 0 }) as i8;
   }

   fn  no_bit(&self,  col: i32,  row: i32) -> bool  {
       return self.bits[row * self.numcols + col] < 0;
   }

   pub fn  place(&self)   {
        let mut pos: i32 = 0;
        let mut row: i32 = 4;
        let mut col: i32 = 0;
       loop { {
           // repeatedly first check for one of the special corner cases, then...
           if (row == self.numrows) && (col == 0) {
               self.corner1(pos += 1 !!!check!!! post increment);
           }
           if (row == self.numrows - 2) && (col == 0) && ((self.numcols % 4) != 0) {
               self.corner2(pos += 1 !!!check!!! post increment);
           }
           if (row == self.numrows - 2) && (col == 0) && (self.numcols % 8 == 4) {
               self.corner3(pos += 1 !!!check!!! post increment);
           }
           if (row == self.numrows + 4) && (col == 2) && ((self.numcols % 8) == 0) {
               self.corner4(pos += 1 !!!check!!! post increment);
           }
           // sweep upward diagonally, inserting successive characters...
           loop { {
               if (row < self.numrows) && (col >= 0) && self.no_bit(col, row) {
                   self.utah(row, col, pos += 1 !!!check!!! post increment);
               }
               row -= 2;
               col += 2;
           }if !(row >= 0 && (col < self.numcols)) break;}
           row += 1;
           col += 3;
           // and then sweep downward diagonally, inserting successive characters, ...
           loop { {
               if (row >= 0) && (col < self.numcols) && self.no_bit(col, row) {
                   self.utah(row, col, pos += 1 !!!check!!! post increment);
               }
               row += 2;
               col -= 2;
           }if !((row < self.numrows) && (col >= 0)) break;}
           row += 3;
           col += 1;
       // ...until the entire array is scanned
       }if !((row < self.numrows) || (col < self.numcols)) break;}
       // Lastly, if the lower right-hand corner is untouched, fill in fixed pattern
       if self.no_bit(self.numcols - 1, self.numrows - 1) {
           self.set_bit(self.numcols - 1, self.numrows - 1, true);
           self.set_bit(self.numcols - 2, self.numrows - 2, true);
       }
   }

   fn  module(&self,  row: i32,  col: i32,  pos: i32,  bit: i32)   {
       if row < 0 {
           row += self.numrows;
           col += 4 - ((self.numrows + 4) % 8);
       }
       if col < 0 {
           col += self.numcols;
           row += 4 - ((self.numcols + 4) % 8);
       }
       // Note the conversion:
        let mut v: i32 = self.codewords.char_at(pos);
       v &= 1 << (8 - bit);
       self.set_bit(col, row, v != 0);
   }

   /**
  * Places the 8 bits of a utah-shaped symbol character in ECC200.
  *
  * @param row the row
  * @param col the column
  * @param pos character position
  */
   fn  utah(&self,  row: i32,  col: i32,  pos: i32)   {
       self.module(row - 2, col - 2, pos, 1);
       self.module(row - 2, col - 1, pos, 2);
       self.module(row - 1, col - 2, pos, 3);
       self.module(row - 1, col - 1, pos, 4);
       self.module(row - 1, col, pos, 5);
       self.module(row, col - 2, pos, 6);
       self.module(row, col - 1, pos, 7);
       self.module(row, col, pos, 8);
   }

   fn  corner1(&self,  pos: i32)   {
       self.module(self.numrows - 1, 0, pos, 1);
       self.module(self.numrows - 1, 1, pos, 2);
       self.module(self.numrows - 1, 2, pos, 3);
       self.module(0, self.numcols - 2, pos, 4);
       self.module(0, self.numcols - 1, pos, 5);
       self.module(1, self.numcols - 1, pos, 6);
       self.module(2, self.numcols - 1, pos, 7);
       self.module(3, self.numcols - 1, pos, 8);
   }

   fn  corner2(&self,  pos: i32)   {
       self.module(self.numrows - 3, 0, pos, 1);
       self.module(self.numrows - 2, 0, pos, 2);
       self.module(self.numrows - 1, 0, pos, 3);
       self.module(0, self.numcols - 4, pos, 4);
       self.module(0, self.numcols - 3, pos, 5);
       self.module(0, self.numcols - 2, pos, 6);
       self.module(0, self.numcols - 1, pos, 7);
       self.module(1, self.numcols - 1, pos, 8);
   }

   fn  corner3(&self,  pos: i32)   {
       self.module(self.numrows - 3, 0, pos, 1);
       self.module(self.numrows - 2, 0, pos, 2);
       self.module(self.numrows - 1, 0, pos, 3);
       self.module(0, self.numcols - 2, pos, 4);
       self.module(0, self.numcols - 1, pos, 5);
       self.module(1, self.numcols - 1, pos, 6);
       self.module(2, self.numcols - 1, pos, 7);
       self.module(3, self.numcols - 1, pos, 8);
   }

   fn  corner4(&self,  pos: i32)   {
       self.module(self.numrows - 1, 0, pos, 1);
       self.module(self.numrows - 1, self.numcols - 1, pos, 2);
       self.module(0, self.numcols - 3, pos, 3);
       self.module(0, self.numcols - 2, pos, 4);
       self.module(0, self.numcols - 1, pos, 5);
       self.module(1, self.numcols - 3, pos, 6);
       self.module(1, self.numcols - 2, pos, 7);
       self.module(1, self.numcols - 1, pos, 8);
   }
}


// EdifactEncoder.java
struct EdifactEncoder {
}

impl Encoder for EdifactEncoder {}

impl EdifactEncoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::EDIFACT_ENCODATION;
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
        //step F
         let buffer: StringBuilder = StringBuilder::new();
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            ::encode_char(c, &buffer);
            context.pos += 1;
             let count: i32 = buffer.length();
            if count >= 4 {
                context.write_codewords(&::encode_to_codewords(&buffer));
                buffer.delete(0, 4);
                 let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
                if new_mode != self.get_encoding_mode() {
                    // Return to ASCII encodation, which will actually handle latch to new mode
                    context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
                    break;
                }
            }
        }
        //Unlatch
        buffer.append(31 as char);
        ::handle_e_o_d(context, &buffer);
    }

    /**
   * Handle "end of data" situations
   *
   * @param context the encoder context
   * @param buffer  the buffer with the remaining encoded characters
   */
    fn  handle_e_o_d( context: &EncoderContext,  buffer: &CharSequence)   {
        let tryResult1 = 0;
        'try1: loop {
        {
             let count: i32 = buffer.length();
            if count == 0 {
                //Already finished
                return;
            }
            if count == 1 {
                //Only an unlatch at the end
                context.update_symbol_info();
                 let mut available: i32 = context.get_symbol_info().get_data_capacity() - context.get_codeword_count();
                 let remaining: i32 = context.get_remaining_characters();
                // The following two lines are a hack inspired by the 'fix' from https://sourceforge.net/p/barcode4j/svn/221/
                if remaining > available {
                    context.update_symbol_info(context.get_codeword_count() + 1);
                    available = context.get_symbol_info().get_data_capacity() - context.get_codeword_count();
                }
                if remaining <= available && available <= 2 {
                    //No unlatch
                    return;
                }
            }
            if count > 4 {
                throw IllegalStateException::new("Count must not exceed 4");
            }
             let rest_chars: i32 = count - 1;
             let encoded: String = ::encode_to_codewords(&buffer);
             let end_of_symbol_reached: bool = !context.has_more_characters();
             let rest_in_ascii: bool = end_of_symbol_reached && rest_chars <= 2;
            if rest_chars <= 2 {
                context.update_symbol_info(context.get_codeword_count() + rest_chars);
                 let available: i32 = context.get_symbol_info().get_data_capacity() - context.get_codeword_count();
                if available >= 3 {
                    rest_in_ascii = false;
                    context.update_symbol_info(context.get_codeword_count() + encoded.length());
                //available = context.symbolInfo.dataCapacity - context.getCodewordCount();
                }
            }
            if rest_in_ascii {
                context.reset_symbol_info();
                context.pos -= rest_chars;
            } else {
                context.write_codewords(&encoded);
            }
        }
        break 'try1
        }
        match tryResult1 {
              0 => break
        }
         finally {
            context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
        }
    }

    fn  encode_char( c: char,  sb: &StringBuilder)   {
        if c >= ' ' && c <= '?' {
            sb.append(c);
        } else if c >= '@' && c <= '^' {
            sb.append((c - 64) as char);
        } else {
            HighLevelEncoder::illegal_character(c);
        }
    }

    fn  encode_to_codewords( sb: &CharSequence) -> String  {
         let len: i32 = sb.length();
        if len == 0 {
            throw IllegalStateException::new("StringBuilder must not be empty");
        }
         let c1: char = sb.char_at(0);
         let c2: char =  if len >= 2 { sb.char_at(1) } else { 0 };
         let c3: char =  if len >= 3 { sb.char_at(2) } else { 0 };
         let c4: char =  if len >= 4 { sb.char_at(3) } else { 0 };
         let v: i32 = (c1 << 18) + (c2 << 12) + (c3 << 6) + c4;
         let cw1: char = ((v >> 16) & 255) as char;
         let cw2: char = ((v >> 8) & 255) as char;
         let cw3: char = (v & 255) as char;
         let res: StringBuilder = StringBuilder::new(3);
        res.append(cw1);
        if len >= 2 {
            res.append(cw2);
        }
        if len >= 3 {
            res.append(cw3);
        }
        return res.to_string();
    }
}

// Encoder.java
trait Encoder {

    fn  get_encoding_mode(&self) -> i32 ;

    fn  encode(&self,  context: &EncoderContext)  ;
}

// EncoderContext.java
struct EncoderContext {

    let msg: String;

    let mut shape: SymbolShapeHint;

    let min_size: Dimension;

    let max_size: Dimension;

    let mut codewords: StringBuilder;

    let pos: i32;

    let new_encoding: i32;

    let symbol_info: SymbolInfo;

    let skip_at_end: i32;
}

impl EncoderContext {

   fn new( msg: &String) -> EncoderContext {
       //From this point on Strings are not Unicode anymore!
        let msg_binary: Vec<i8> = msg.get_bytes(StandardCharsets::ISO_8859_1);
        let sb: StringBuilder = StringBuilder::new(msg_binary.len());
        {
            let mut i: i32 = 0, let c: i32 = msg_binary.len();
           while i < c {
               {
                    let ch: char = (msg_binary[i] & 0xff) as char;
                   if ch == '?' && msg.char_at(i) != '?' {
                       throw IllegalArgumentException::new("Message contains characters outside ISO-8859-1 encoding.");
                   }
                   sb.append(ch);
               }
               i += 1;
            }
        }

       //Not Unicode here!
       let .msg = sb.to_string();
       shape = SymbolShapeHint::FORCE_NONE;
       let .codewords = StringBuilder::new(&msg.length());
       new_encoding = -1;
   }

   pub fn  set_symbol_shape(&self,  shape: &SymbolShapeHint)   {
       self.shape = shape;
   }

   pub fn  set_size_constraints(&self,  min_size: &Dimension,  max_size: &Dimension)   {
       self.minSize = min_size;
       self.maxSize = max_size;
   }

   pub fn  get_message(&self) -> String  {
       return self.msg;
   }

   pub fn  set_skip_at_end(&self,  count: i32)   {
       self.skipAtEnd = count;
   }

   pub fn  get_current_char(&self) -> char  {
       return self.msg.char_at(self.pos);
   }

   pub fn  get_current(&self) -> char  {
       return self.msg.char_at(self.pos);
   }

   pub fn  get_codewords(&self) -> StringBuilder  {
       return self.codewords;
   }

   pub fn  write_codewords(&self,  codewords: &String)   {
       self.codewords.append(&codewords);
   }

   pub fn  write_codeword(&self,  codeword: char)   {
       self.codewords.append(codeword);
   }

   pub fn  get_codeword_count(&self) -> i32  {
       return self.codewords.length();
   }

   pub fn  get_new_encoding(&self) -> i32  {
       return self.new_encoding;
   }

   pub fn  signal_encoder_change(&self,  encoding: i32)   {
       self.newEncoding = encoding;
   }

   pub fn  reset_encoder_signal(&self)   {
       self.newEncoding = -1;
   }

   pub fn  has_more_characters(&self) -> bool  {
       return self.pos < self.get_total_message_char_count();
   }

   fn  get_total_message_char_count(&self) -> i32  {
       return self.msg.length() - self.skip_at_end;
   }

   pub fn  get_remaining_characters(&self) -> i32  {
       return self.get_total_message_char_count() - self.pos;
   }

   pub fn  get_symbol_info(&self) -> SymbolInfo  {
       return self.symbol_info;
   }

   pub fn  update_symbol_info(&self)   {
       self.update_symbol_info(&self.get_codeword_count());
   }

   pub fn  update_symbol_info(&self,  len: i32)   {
       if self.symbolInfo == null || len > self.symbolInfo.get_data_capacity() {
           self.symbolInfo = SymbolInfo::lookup(len, self.shape, self.min_size, self.max_size, true);
       }
   }

   pub fn  reset_symbol_info(&self)   {
       self.symbolInfo = null;
   }
}


// ErrorCorrection.java
/**
 * Error Correction Code for ECC200.
 */

/**
   * Lookup table which factors to use for which number of error correction codewords.
   * See FACTORS.
   */
  const FACTOR_SETS: vec![Vec<i32>; 16] = vec![5, 7, 10, 11, 12, 14, 18, 20, 24, 28, 36, 42, 48, 56, 62, 68, ]
  ;
  
  /**
     * Precomputed polynomial factors for ECC 200.
     */
   const FACTORS: vec![vec![Vec<Vec<i32>>; 68]; 16] = vec![vec![228, 48, 15, 111, 62, ]
  , vec![23, 68, 144, 134, 240, 92, 254, ]
  , vec![28, 24, 185, 166, 223, 248, 116, 255, 110, 61, ]
  , vec![175, 138, 205, 12, 194, 168, 39, 245, 60, 97, 120, ]
  , vec![41, 153, 158, 91, 61, 42, 142, 213, 97, 178, 100, 242, ]
  , vec![156, 97, 192, 252, 95, 9, 157, 119, 138, 45, 18, 186, 83, 185, ]
  , vec![83, 195, 100, 39, 188, 75, 66, 61, 241, 213, 109, 129, 94, 254, 225, 48, 90, 188, ]
  , vec![15, 195, 244, 9, 233, 71, 168, 2, 188, 160, 153, 145, 253, 79, 108, 82, 27, 174, 186, 172, ]
  , vec![52, 190, 88, 205, 109, 39, 176, 21, 155, 197, 251, 223, 155, 21, 5, 172, 254, 124, 12, 181, 184, 96, 50, 193, ]
  , vec![211, 231, 43, 97, 71, 96, 103, 174, 37, 151, 170, 53, 75, 34, 249, 121, 17, 138, 110, 213, 141, 136, 120, 151, 233, 168, 93, 255, ]
  , vec![245, 127, 242, 218, 130, 250, 162, 181, 102, 120, 84, 179, 220, 251, 80, 182, 229, 18, 2, 4, 68, 33, 101, 137, 95, 119, 115, 44, 175, 184, 59, 25, 225, 98, 81, 112, ]
  , vec![77, 193, 137, 31, 19, 38, 22, 153, 247, 105, 122, 2, 245, 133, 242, 8, 175, 95, 100, 9, 167, 105, 214, 111, 57, 121, 21, 1, 253, 57, 54, 101, 248, 202, 69, 50, 150, 177, 226, 5, 9, 5, ]
  , vec![245, 132, 172, 223, 96, 32, 117, 22, 238, 133, 238, 231, 205, 188, 237, 87, 191, 106, 16, 147, 118, 23, 37, 90, 170, 205, 131, 88, 120, 100, 66, 138, 186, 240, 82, 44, 176, 87, 187, 147, 160, 175, 69, 213, 92, 253, 225, 19, ]
  , vec![175, 9, 223, 238, 12, 17, 220, 208, 100, 29, 175, 170, 230, 192, 215, 235, 150, 159, 36, 223, 38, 200, 132, 54, 228, 146, 218, 234, 117, 203, 29, 232, 144, 238, 22, 150, 201, 117, 62, 207, 164, 13, 137, 245, 127, 67, 247, 28, 155, 43, 203, 107, 233, 53, 143, 46, ]
  , vec![242, 93, 169, 50, 144, 210, 39, 118, 202, 188, 201, 189, 143, 108, 196, 37, 185, 112, 134, 230, 245, 63, 197, 190, 250, 106, 185, 221, 175, 64, 114, 71, 161, 44, 147, 6, 27, 218, 51, 63, 87, 10, 40, 130, 188, 17, 163, 31, 176, 170, 4, 107, 232, 7, 94, 166, 224, 124, 86, 47, 11, 204, ]
  , vec![220, 228, 173, 89, 251, 149, 159, 56, 89, 33, 147, 244, 154, 36, 73, 127, 213, 136, 248, 180, 234, 197, 158, 177, 68, 122, 93, 213, 15, 160, 227, 236, 66, 139, 153, 185, 202, 167, 179, 25, 220, 232, 96, 210, 231, 136, 223, 239, 181, 241, 59, 52, 172, 25, 49, 232, 211, 189, 64, 54, 108, 153, 132, 63, 96, 103, 82, 186, ]
  , ]
  ;
  
   const MODULO_VALUE: i32 = 0x12D;
  
   const LOG: Vec<i32>;
  
   const ALOG: Vec<i32>;
  pub struct ErrorCorrection {
  }
  
  impl ErrorCorrection {
  
      static {
          //Create log and antilog table
          LOG = : [i32; 256] = [0; 256];
          ALOG = : [i32; 255] = [0; 255];
           let mut p: i32 = 1;
           {
               let mut i: i32 = 0;
              while i < 255 {
                  {
                      ALOG[i] = p;
                      LOG[p] = i;
                      p *= 2;
                      if p >= 256 {
                          p ^= MODULO_VALUE;
                      }
                  }
                  i += 1;
               }
           }
  
      }
  
      fn new() -> ErrorCorrection {
      }
  
      /**
     * Creates the ECC200 error correction for an encoded message.
     *
     * @param codewords  the codewords
     * @param symbolInfo information about the symbol to be encoded
     * @return the codewords with interleaved error correction.
     */
      pub fn  encode_e_c_c200( codewords: &String,  symbol_info: &SymbolInfo) -> String  {
          if codewords.length() != symbol_info.get_data_capacity() {
              throw IllegalArgumentException::new("The number of codewords does not match the selected symbol");
          }
           let sb: StringBuilder = StringBuilder::new(symbol_info.get_data_capacity() + symbol_info.get_error_codewords());
          sb.append(&codewords);
           let block_count: i32 = symbol_info.get_interleaved_block_count();
          if block_count == 1 {
               let ecc: String = ::create_e_c_c_block(&codewords, &symbol_info.get_error_codewords());
              sb.append(&ecc);
          } else {
              sb.set_length(&sb.capacity());
               let data_sizes: [i32; block_count] = [0; block_count];
               let error_sizes: [i32; block_count] = [0; block_count];
               {
                   let mut i: i32 = 0;
                  while i < block_count {
                      {
                          data_sizes[i] = symbol_info.get_data_length_for_interleaved_block(i + 1);
                          error_sizes[i] = symbol_info.get_error_length_for_interleaved_block(i + 1);
                      }
                      i += 1;
                   }
               }
  
               {
                   let mut block: i32 = 0;
                  while block < block_count {
                      {
                           let temp: StringBuilder = StringBuilder::new(data_sizes[block]);
                           {
                               let mut d: i32 = block;
                              while d < symbol_info.get_data_capacity() {
                                  {
                                      temp.append(&codewords.char_at(d));
                                  }
                                  d += block_count;
                               }
                           }
  
                           let ecc: String = ::create_e_c_c_block(&temp.to_string(), error_sizes[block]);
                           let mut pos: i32 = 0;
                           {
                               let mut e: i32 = block;
                              while e < error_sizes[block] * block_count {
                                  {
                                      sb.set_char_at(symbol_info.get_data_capacity() + e, &ecc.char_at(pos += 1 !!!check!!! post increment));
                                  }
                                  e += block_count;
                               }
                           }
  
                      }
                      block += 1;
                   }
               }
  
          }
          return sb.to_string();
      }
  
      fn  create_e_c_c_block( codewords: &CharSequence,  num_e_c_words: i32) -> String  {
           let mut table: i32 = -1;
           {
               let mut i: i32 = 0;
              while i < FACTOR_SETS.len() {
                  {
                      if FACTOR_SETS[i] == num_e_c_words {
                          table = i;
                          break;
                      }
                  }
                  i += 1;
               }
           }
  
          if table < 0 {
              throw IllegalArgumentException::new(format!("Illegal number of error correction codewords specified: {}", num_e_c_words));
          }
           let poly: Vec<i32> = FACTORS[table];
           let mut ecc: [Option<char>; num_e_c_words] = [None; num_e_c_words];
           {
               let mut i: i32 = 0;
              while i < num_e_c_words {
                  {
                      ecc[i] = 0;
                  }
                  i += 1;
               }
           }
  
           {
               let mut i: i32 = 0;
              while i < codewords.length() {
                  {
                       let m: i32 = ecc[num_e_c_words - 1] ^ codewords.char_at(i);
                       {
                           let mut k: i32 = num_e_c_words - 1;
                          while k > 0 {
                              {
                                  if m != 0 && poly[k] != 0 {
                                      ecc[k] = (ecc[k - 1] ^ ALOG[(LOG[m] + LOG[poly[k]]) % 255]) as char;
                                  } else {
                                      ecc[k] = ecc[k - 1];
                                  }
                              }
                              k -= 1;
                           }
                       }
  
                      if m != 0 && poly[0] != 0 {
                          ecc[0] = ALOG[(LOG[m] + LOG[poly[0]]) % 255] as char;
                      } else {
                          ecc[0] = 0;
                      }
                  }
                  i += 1;
               }
           }
  
           let ecc_reversed: [Option<char>; num_e_c_words] = [None; num_e_c_words];
           {
               let mut i: i32 = 0;
              while i < num_e_c_words {
                  {
                      ecc_reversed[i] = ecc[num_e_c_words - i - 1];
                  }
                  i += 1;
               }
           }
  
          return String::value_of(&ecc_reversed);
      }
  }
  

  // HighLevelEncoder.java
  /**
 * DataMatrix ECC 200 data encoder following the algorithm described in ISO/IEC 16022:200(E) in
 * annex S.
 */

/**
   * Padding character
   */
 const PAD: char = 129;

 /**
    * mode latch to C40 encodation mode
    */
  const LATCH_TO_C40: char = 230;
 
 /**
    * mode latch to Base 256 encodation mode
    */
  const LATCH_TO_BASE256: char = 231;
 
 /**
    * FNC1 Codeword
    */
 //private static final char FNC1 = 232;
 /**
    * Structured Append Codeword
    */
 //private static final char STRUCTURED_APPEND = 233;
 /**
    * Reader Programming
    */
 //private static final char READER_PROGRAMMING = 234;
 /**
    * Upper Shift
    */
  const UPPER_SHIFT: char = 235;
 
 /**
    * 05 Macro
    */
  const MACRO_05: char = 236;
 
 /**
    * 06 Macro
    */
  const MACRO_06: char = 237;
 
 /**
    * mode latch to ANSI X.12 encodation mode
    */
  const LATCH_TO_ANSIX12: char = 238;
 
 /**
    * mode latch to Text encodation mode
    */
  const LATCH_TO_TEXT: char = 239;
 
 /**
    * mode latch to EDIFACT encodation mode
    */
  const LATCH_TO_EDIFACT: char = 240;
 
 /**
    * ECI character (Extended Channel Interpretation)
    */
 //private static final char ECI = 241;
 /**
    * Unlatch from C40 encodation
    */
  const C40_UNLATCH: char = 254;
 
 /**
    * Unlatch from X12 encodation
    */
  const X12_UNLATCH: char = 254;
 
 /**
    * 05 Macro header
    */
  const MACRO_05_HEADER: &'static str = "[)>05";
 
 /**
    * 06 Macro header
    */
  const MACRO_06_HEADER: &'static str = "[)>06";
 
 /**
    * Macro trailer
    */
  const MACRO_TRAILER: &'static str = "";
 
  const ASCII_ENCODATION: i32 = 0;
 
  const C40_ENCODATION: i32 = 1;
 
  const TEXT_ENCODATION: i32 = 2;
 
  const X12_ENCODATION: i32 = 3;
 
  const EDIFACT_ENCODATION: i32 = 4;
 
  const BASE256_ENCODATION: i32 = 5;
 pub struct HighLevelEncoder {
 }
 
 impl HighLevelEncoder {
 
     fn new() -> HighLevelEncoder {
     }
 
     fn  randomize253_state( codeword_position: i32) -> char  {
          let pseudo_random: i32 = ((149 * codeword_position) % 253) + 1;
          let temp_variable: i32 = PAD + pseudo_random;
         return ( if temp_variable <= 254 { temp_variable } else { temp_variable - 254 }) as char;
     }
 
     /**
    * Performs message encoding of a DataMatrix message using the algorithm described in annex P
    * of ISO/IEC 16022:2000(E).
    *
    * @param msg the message
    * @return the encoded message (the char values range from 0 to 255)
    */
     pub fn  encode_high_level( msg: &String) -> String  {
         return ::encode_high_level(&msg, SymbolShapeHint::FORCE_NONE, null, null, false);
     }
 
     /**
    * Performs message encoding of a DataMatrix message using the algorithm described in annex P
    * of ISO/IEC 16022:2000(E).
    *
    * @param msg     the message
    * @param shape   requested shape. May be {@code SymbolShapeHint.FORCE_NONE},
    *                {@code SymbolShapeHint.FORCE_SQUARE} or {@code SymbolShapeHint.FORCE_RECTANGLE}.
    * @param minSize the minimum symbol size constraint or null for no constraint
    * @param maxSize the maximum symbol size constraint or null for no constraint
    * @return the encoded message (the char values range from 0 to 255)
    */
     pub fn  encode_high_level( msg: &String,  shape: &SymbolShapeHint,  min_size: &Dimension,  max_size: &Dimension) -> String  {
         return ::encode_high_level(&msg, shape, min_size, max_size, false);
     }
 
     /**
    * Performs message encoding of a DataMatrix message using the algorithm described in annex P
    * of ISO/IEC 16022:2000(E).
    *
    * @param msg     the message
    * @param shape   requested shape. May be {@code SymbolShapeHint.FORCE_NONE},
    *                {@code SymbolShapeHint.FORCE_SQUARE} or {@code SymbolShapeHint.FORCE_RECTANGLE}.
    * @param minSize the minimum symbol size constraint or null for no constraint
    * @param maxSize the maximum symbol size constraint or null for no constraint
    * @param forceC40 enforce C40 encoding
    * @return the encoded message (the char values range from 0 to 255)
    */
     pub fn  encode_high_level( msg: &String,  shape: &SymbolShapeHint,  min_size: &Dimension,  max_size: &Dimension,  force_c40: bool) -> String  {
         //the codewords 0..255 are encoded as Unicode characters
          let c40_encoder: C40Encoder = C40Encoder::new();
          let encoders: vec![Vec<Encoder>; 6] = vec![ASCIIEncoder::new(), c40_encoder, TextEncoder::new(), X12Encoder::new(), EdifactEncoder::new(), Base256Encoder::new(), ]
         ;
          let mut context: EncoderContext = EncoderContext::new(&msg);
         context.set_symbol_shape(shape);
         context.set_size_constraints(min_size, max_size);
         if msg.starts_with(&MACRO_05_HEADER) && msg.ends_with(&MACRO_TRAILER) {
             context.write_codeword(MACRO_05);
             context.set_skip_at_end(2);
             context.pos += MACRO_05_HEADER::length();
         } else if msg.starts_with(&MACRO_06_HEADER) && msg.ends_with(&MACRO_TRAILER) {
             context.write_codeword(MACRO_06);
             context.set_skip_at_end(2);
             context.pos += MACRO_06_HEADER::length();
         }
         //Default mode
          let encoding_mode: i32 = ASCII_ENCODATION;
         if force_c40 {
             c40_encoder.encode_maximal(context);
             encoding_mode = context.get_new_encoding();
             context.reset_encoder_signal();
         }
         while context.has_more_characters() {
             encoders[encoding_mode].encode(context);
             if context.get_new_encoding() >= 0 {
                 encoding_mode = context.get_new_encoding();
                 context.reset_encoder_signal();
             }
         }
          let len: i32 = context.get_codeword_count();
         context.update_symbol_info();
          let capacity: i32 = context.get_symbol_info().get_data_capacity();
         if len < capacity && encoding_mode != ASCII_ENCODATION && encoding_mode != BASE256_ENCODATION && encoding_mode != EDIFACT_ENCODATION {
             //Unlatch (254)
             context.write_codeword('þ');
         }
         //Padding
          let codewords: StringBuilder = context.get_codewords();
         if codewords.length() < capacity {
             codewords.append(PAD);
         }
         while codewords.length() < capacity {
             codewords.append(&::randomize253_state(codewords.length() + 1));
         }
         return context.get_codewords().to_string();
     }
 
     fn  look_ahead_test( msg: &CharSequence,  startpos: i32,  current_mode: i32) -> i32  {
          let new_mode: i32 = ::look_ahead_test_intern(&msg, startpos, current_mode);
         if current_mode == X12_ENCODATION && new_mode == X12_ENCODATION {
              let endpos: i32 = Math::min(startpos + 3, &msg.length());
              {
                  let mut i: i32 = startpos;
                 while i < endpos {
                     {
                         if !::is_native_x12(&msg.char_at(i)) {
                             return ASCII_ENCODATION;
                         }
                     }
                     i += 1;
                  }
              }
 
         } else if current_mode == EDIFACT_ENCODATION && new_mode == EDIFACT_ENCODATION {
              let endpos: i32 = Math::min(startpos + 4, &msg.length());
              {
                  let mut i: i32 = startpos;
                 while i < endpos {
                     {
                         if !::is_native_e_d_i_f_a_c_t(&msg.char_at(i)) {
                             return ASCII_ENCODATION;
                         }
                     }
                     i += 1;
                  }
              }
 
         }
         return new_mode;
     }
 
     fn  look_ahead_test_intern( msg: &CharSequence,  startpos: i32,  current_mode: i32) -> i32  {
         if startpos >= msg.length() {
             return current_mode;
         }
          let char_counts: Vec<f32>;
         //step J
         if current_mode == ASCII_ENCODATION {
             char_counts =  : vec![f32; 6] = vec![0.0, 1.0, 1.0, 1.0, 1.0, 1.25f, ]
             ;
         } else {
             char_counts =  : vec![f32; 6] = vec![1.0, 2.0, 2.0, 2.0, 2.0, 2.25f, ]
             ;
             char_counts[current_mode] = 0.0;
         }
          let chars_processed: i32 = 0;
          let mins: [i8; 6] = [0; 6];
          let int_char_counts: [i32; 6] = [0; 6];
         while true {
             //step K
             if (startpos + chars_processed) == msg.length() {
                 Arrays::fill(&mins, 0 as i8);
                 Arrays::fill(&int_char_counts, 0);
                  let min: i32 = ::find_minimums(&char_counts, &int_char_counts, Integer::MAX_VALUE, &mins);
                  let min_count: i32 = ::get_minimum_count(&mins);
                 if int_char_counts[ASCII_ENCODATION] == min {
                     return ASCII_ENCODATION;
                 }
                 if min_count == 1 {
                     if mins[BASE256_ENCODATION] > 0 {
                         return BASE256_ENCODATION;
                     }
                     if mins[EDIFACT_ENCODATION] > 0 {
                         return EDIFACT_ENCODATION;
                     }
                     if mins[TEXT_ENCODATION] > 0 {
                         return TEXT_ENCODATION;
                     }
                     if mins[X12_ENCODATION] > 0 {
                         return X12_ENCODATION;
                     }
                 }
                 return C40_ENCODATION;
             }
              let c: char = msg.char_at(startpos + chars_processed);
             chars_processed += 1;
             //step L
             if ::is_digit(c) {
                 char_counts[ASCII_ENCODATION] += 0.5f;
             } else if ::is_extended_a_s_c_i_i(c) {
                 char_counts[ASCII_ENCODATION] = Math::ceil(char_counts[ASCII_ENCODATION]) as f32;
                 char_counts[ASCII_ENCODATION] += 2.0f;
             } else {
                 char_counts[ASCII_ENCODATION] = Math::ceil(char_counts[ASCII_ENCODATION]) as f32;
                 char_counts[ASCII_ENCODATION] += 1;
             }
             //step M
             if ::is_native_c40(c) {
                 char_counts[C40_ENCODATION] += 2.0f / 3.0f;
             } else if ::is_extended_a_s_c_i_i(c) {
                 char_counts[C40_ENCODATION] += 8.0f / 3.0f;
             } else {
                 char_counts[C40_ENCODATION] += 4.0f / 3.0f;
             }
             //step N
             if ::is_native_text(c) {
                 char_counts[TEXT_ENCODATION] += 2.0f / 3.0f;
             } else if ::is_extended_a_s_c_i_i(c) {
                 char_counts[TEXT_ENCODATION] += 8.0f / 3.0f;
             } else {
                 char_counts[TEXT_ENCODATION] += 4.0f / 3.0f;
             }
             //step O
             if ::is_native_x12(c) {
                 char_counts[X12_ENCODATION] += 2.0f / 3.0f;
             } else if ::is_extended_a_s_c_i_i(c) {
                 char_counts[X12_ENCODATION] += 13.0f / 3.0f;
             } else {
                 char_counts[X12_ENCODATION] += 10.0f / 3.0f;
             }
             //step P
             if ::is_native_e_d_i_f_a_c_t(c) {
                 char_counts[EDIFACT_ENCODATION] += 3.0f / 4.0f;
             } else if ::is_extended_a_s_c_i_i(c) {
                 char_counts[EDIFACT_ENCODATION] += 17.0f / 4.0f;
             } else {
                 char_counts[EDIFACT_ENCODATION] += 13.0f / 4.0f;
             }
             // step Q
             if ::is_special_b256(c) {
                 char_counts[BASE256_ENCODATION] += 4.0f;
             } else {
                 char_counts[BASE256_ENCODATION] += 1;
             }
             //step R
             if chars_processed >= 4 {
                 Arrays::fill(&mins, 0 as i8);
                 Arrays::fill(&int_char_counts, 0);
                 ::find_minimums(&char_counts, &int_char_counts, Integer::MAX_VALUE, &mins);
                 if int_char_counts[ASCII_ENCODATION] < ::min(int_char_counts[BASE256_ENCODATION], int_char_counts[C40_ENCODATION], int_char_counts[TEXT_ENCODATION], int_char_counts[X12_ENCODATION], int_char_counts[EDIFACT_ENCODATION]) {
                     return ASCII_ENCODATION;
                 }
                 if int_char_counts[BASE256_ENCODATION] < int_char_counts[ASCII_ENCODATION] || int_char_counts[BASE256_ENCODATION] + 1 < ::min(int_char_counts[C40_ENCODATION], int_char_counts[TEXT_ENCODATION], int_char_counts[X12_ENCODATION], int_char_counts[EDIFACT_ENCODATION]) {
                     return BASE256_ENCODATION;
                 }
                 if int_char_counts[EDIFACT_ENCODATION] + 1 < ::min(int_char_counts[BASE256_ENCODATION], int_char_counts[C40_ENCODATION], int_char_counts[TEXT_ENCODATION], int_char_counts[X12_ENCODATION], int_char_counts[ASCII_ENCODATION]) {
                     return EDIFACT_ENCODATION;
                 }
                 if int_char_counts[TEXT_ENCODATION] + 1 < ::min(int_char_counts[BASE256_ENCODATION], int_char_counts[C40_ENCODATION], int_char_counts[EDIFACT_ENCODATION], int_char_counts[X12_ENCODATION], int_char_counts[ASCII_ENCODATION]) {
                     return TEXT_ENCODATION;
                 }
                 if int_char_counts[X12_ENCODATION] + 1 < ::min(int_char_counts[BASE256_ENCODATION], int_char_counts[C40_ENCODATION], int_char_counts[EDIFACT_ENCODATION], int_char_counts[TEXT_ENCODATION], int_char_counts[ASCII_ENCODATION]) {
                     return X12_ENCODATION;
                 }
                 if int_char_counts[C40_ENCODATION] + 1 < ::min(int_char_counts[ASCII_ENCODATION], int_char_counts[BASE256_ENCODATION], int_char_counts[EDIFACT_ENCODATION], int_char_counts[TEXT_ENCODATION]) {
                     if int_char_counts[C40_ENCODATION] < int_char_counts[X12_ENCODATION] {
                         return C40_ENCODATION;
                     }
                     if int_char_counts[C40_ENCODATION] == int_char_counts[X12_ENCODATION] {
                          let mut p: i32 = startpos + chars_processed + 1;
                         while p < msg.length() {
                              let tc: char = msg.char_at(p);
                             if ::is_x12_term_sep(tc) {
                                 return X12_ENCODATION;
                             }
                             if !::is_native_x12(tc) {
                                 break;
                             }
                             p += 1;
                         }
                         return C40_ENCODATION;
                     }
                 }
             }
         }
     }
 
     fn  min( f1: i32,  f2: i32,  f3: i32,  f4: i32,  f5: i32) -> i32  {
         return Math::min(&::min(f1, f2, f3, f4), f5);
     }
 
     fn  min( f1: i32,  f2: i32,  f3: i32,  f4: i32) -> i32  {
         return Math::min(f1, &Math::min(f2, &Math::min(f3, f4)));
     }
 
     fn  find_minimums( char_counts: &Vec<f32>,  int_char_counts: &Vec<i32>,  min: i32,  mins: &Vec<i8>) -> i32  {
          {
              let mut i: i32 = 0;
             while i < 6 {
                 {
                      let current: i32 = (int_char_counts[i] = Math::ceil(char_counts[i]) as i32);
                     if min > current {
                         min = current;
                         Arrays::fill(&mins, 0 as i8);
                     }
                     if min == current {
                         mins[i] += 1;
                     }
                 }
                 i += 1;
              }
          }
 
         return min;
     }
 
     fn  get_minimum_count( mins: &Vec<i8>) -> i32  {
          let min_count: i32 = 0;
          {
              let mut i: i32 = 0;
             while i < 6 {
                 {
                     min_count += mins[i];
                 }
                 i += 1;
              }
          }
 
         return min_count;
     }
 
     fn  is_digit( ch: char) -> bool  {
         return ch >= '0' && ch <= '9';
     }
 
     fn  is_extended_a_s_c_i_i( ch: char) -> bool  {
         return ch >= 128 && ch <= 255;
     }
 
     fn  is_native_c40( ch: char) -> bool  {
         return (ch == ' ') || (ch >= '0' && ch <= '9') || (ch >= 'A' && ch <= 'Z');
     }
 
     fn  is_native_text( ch: char) -> bool  {
         return (ch == ' ') || (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'z');
     }
 
     fn  is_native_x12( ch: char) -> bool  {
         return ::is_x12_term_sep(ch) || (ch == ' ') || (ch >= '0' && ch <= '9') || (ch >= 'A' && ch <= 'Z');
     }
 
     fn  is_x12_term_sep( ch: char) -> bool  {
         return //CR
         (ch == '\r') || (ch == '*') || (ch == '>');
     }
 
     fn  is_native_e_d_i_f_a_c_t( ch: char) -> bool  {
         return ch >= ' ' && ch <= '^';
     }
 
     fn  is_special_b256( ch: char) -> bool  {
         //TODO NOT IMPLEMENTED YET!!!
         return false;
     }
 
     /**
    * Determines the number of consecutive characters that are encodable using numeric compaction.
    *
    * @param msg      the message
    * @param startpos the start position within the message
    * @return the requested character count
    */
     pub fn  determine_consecutive_digit_count( msg: &CharSequence,  startpos: i32) -> i32  {
          let len: i32 = msg.length();
          let mut idx: i32 = startpos;
         while idx < len && ::is_digit(&msg.char_at(idx)) {
             idx += 1;
         }
         return idx - startpos;
     }
 
     fn  illegal_character( c: char)   {
          let mut hex: String = Integer::to_hex_string(c);
         hex = format!("{}{}", "0000".substring(0, 4 - hex.length()), hex);
         throw IllegalArgumentException::new(format!("Illegal character: {} (0x{})", c, hex));
     }
 }
 

 // MinimalEncoder.java
 /**
 * Encoder that encodes minimally
 *
 * Algorithm:
 *
 * Uses Dijkstra to produce mathematically minimal encodings that are in some cases smaller than the results produced
 * by the algorithm described in annex S in the specification ISO/IEC 16022:200(E). The biggest improvment of this
 * algorithm over that one is the case when the algorithm enters the most inefficient mode, the B256 mode. The 
 * algorithm from the specification algorithm will exit this mode only if it encounters digits so that arbitrarily
 * inefficient results can be produced if the postfix contains no digits.
 *
 * Multi ECI support and ECI switching:
 *
 * For multi language content the algorithm selects the most compact representation using ECI modes. Note that unlike
 * the compaction algorithm used for QR-Codes, this implementation operates in two stages and therfore is not
 * mathematically optimal. In the first stage, the input string is encoded minimally as a stream of ECI character set
 * selectors and bytes encoded in the selected encoding. In this stage the algorithm might for example decide to
 * encode ocurrences of the characters "\u0150\u015C" (O-double-acute, S-circumflex) in UTF-8 by a single ECI or
 * alternatively by multiple ECIs that switch between IS0-8859-2 and ISO-8859-3 (e.g. in the case that the input
 * contains many * characters from ISO-8859-2 (Latin 2) and few from ISO-8859-3 (Latin 3)).
 * In a second stage this stream of ECIs and bytes is minimally encoded using the various Data Matrix encoding modes.
 * While both stages encode mathematically minimally it is not ensured that the result is mathematically minimal since
 * the size growth for inserting an ECI in the first stage can only be approximated as the first stage does not know 
 * in which mode the ECI will occur in the second stage (may, or may not require an extra latch to ASCII depending on
 * the current mode). The reason for this shortcoming are difficulties in implementing it in a straightforward and
 * readable manner.
 *
 * GS1 support
 *
 * FNC1 delimiters can be encoded in the input string by using the FNC1 character specified in the encoding function.
 * When a FNC1 character is specified then a leading FNC1 will be encoded and all ocurrences of delimiter characters
 * while result in FNC1 codewords in the symbol.
 *
 * @author Alex Geller
 */

const C40_SHIFT2_CHARS: vec![Vec<char>; 27] = vec!['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', ]
;
pub struct MinimalEncoder {
}

impl MinimalEncoder {

    enum Mode {

        ASCII(), C40(), TEXT(), X12(), EDF(), B256()
    }

    fn new() -> MinimalEncoder {
    }

    fn  is_extended_a_s_c_i_i( ch: char,  fnc1: i32) -> bool  {
        return ch != fnc1 && ch >= 128 && ch <= 255;
    }

    fn  is_in_c40_shift1_set( ch: char) -> bool  {
        return ch <= 31;
    }

    fn  is_in_c40_shift2_set( ch: char,  fnc1: i32) -> bool  {
        for  let c40_shift2_char: char in C40_SHIFT2_CHARS {
            if c40_shift2_char == ch {
                return true;
            }
        }
        return ch == fnc1;
    }

    fn  is_in_text_shift1_set( ch: char) -> bool  {
        return ::is_in_c40_shift1_set(ch);
    }

    fn  is_in_text_shift2_set( ch: char,  fnc1: i32) -> bool  {
        return ::is_in_c40_shift2_set(ch, fnc1);
    }

    pub fn  encode_high_level( msg: &String) -> String  {
        return ::encode_high_level(&msg, null, -1, SymbolShapeHint::FORCE_NONE);
    }

    pub fn  encode_high_level( msg: &String,  priority_charset: &Charset,  fnc1: i32,  shape: &SymbolShapeHint) -> String  {
         let macro_id: i32 = 0;
        if msg.starts_with(HighLevelEncoder::MACRO_05_HEADER) && msg.ends_with(HighLevelEncoder::MACRO_TRAILER) {
            macro_id = 5;
            msg = msg.substring(&HighLevelEncoder::MACRO_05_HEADER::length(), msg.length() - 2);
        } else if msg.starts_with(HighLevelEncoder::MACRO_06_HEADER) && msg.ends_with(HighLevelEncoder::MACRO_TRAILER) {
            macro_id = 6;
            msg = msg.substring(&HighLevelEncoder::MACRO_06_HEADER::length(), msg.length() - 2);
        }
        return String::new(&::encode(&msg, &priority_charset, fnc1, shape, macro_id), StandardCharsets::ISO_8859_1);
    }

    fn  encode( input: &String,  priority_charset: &Charset,  fnc1: i32,  shape: &SymbolShapeHint,  macro_id: i32) -> Vec<i8>  {
        return ::encode_minimally(Input::new(&input, &priority_charset, fnc1, shape, macro_id)).get_bytes();
    }

    fn  add_edge( edges: &Vec<Vec<Edge>>,  edge: &Edge)   {
         let vertex_index: i32 = edge.fromPosition + edge.characterLength;
        if edges[vertex_index][edge.get_end_mode().ordinal()] == null || edges[vertex_index][edge.get_end_mode().ordinal()].cachedTotalSize > edge.cachedTotalSize {
            edges[vertex_index][edge.get_end_mode().ordinal()] = edge;
        }
    }

    fn  get_number_of_c40_words( input: &Input,  from: i32,  c40: bool,  character_length: &Vec<i32>) -> i32  {
         let thirds_count: i32 = 0;
         {
             let mut i: i32 = from;
            while i < input.length() {
                {
                    if input.is_e_c_i(i) {
                        character_length[0] = 0;
                        return 0;
                    }
                     let ci: char = input.char_at(i);
                    if c40 && HighLevelEncoder::is_native_c40(ci) || !c40 && HighLevelEncoder::is_native_text(ci) {
                        thirds_count += 1;
                    } else if !::is_extended_a_s_c_i_i(ci, &input.get_f_n_c1_character()) {
                        thirds_count += 2;
                    } else {
                         let ascii_value: i32 = ci & 0xff;
                        if ascii_value >= 128 && (c40 && HighLevelEncoder::is_native_c40((ascii_value - 128) as char) || !c40 && HighLevelEncoder::is_native_text((ascii_value - 128) as char)) {
                            thirds_count += 3;
                        } else {
                            thirds_count += 4;
                        }
                    }
                    if thirds_count % 3 == 0 || ((thirds_count - 2) % 3 == 0 && i + 1 == input.length()) {
                        character_length[0] = i - from + 1;
                        return Math::ceil((thirds_count as f64) / 3.0) as i32;
                    }
                }
                i += 1;
             }
         }

        character_length[0] = 0;
        return 0;
    }

    fn  add_edges( input: &Input,  edges: &Vec<Vec<Edge>>,  from: i32,  previous: &Edge)   {
        if input.is_e_c_i(from) {
            ::add_edge(edges, Edge::new(input, Mode::ASCII, from, 1, previous));
            return;
        }
         let ch: char = input.char_at(from);
        if previous == null || previous.get_end_mode() != Mode::EDF {
            if HighLevelEncoder::is_digit(ch) && input.have_n_characters(from, 2) && HighLevelEncoder::is_digit(&input.char_at(from + 1)) {
                ::add_edge(edges, Edge::new(input, Mode::ASCII, from, 2, previous));
            } else {
                ::add_edge(edges, Edge::new(input, Mode::ASCII, from, 1, previous));
            }
             let modes: vec![Vec<Mode>; 2] = vec![Mode::C40, Mode::TEXT, ]
            ;
            for  let mode: Mode in modes {
                 let character_length: [i32; 1] = [0; 1];
                if ::get_number_of_c40_words(input, from, mode == Mode::C40, &character_length) > 0 {
                    ::add_edge(edges, Edge::new(input, mode, from, character_length[0], previous));
                }
            }
            if input.have_n_characters(from, 3) && HighLevelEncoder::is_native_x12(&input.char_at(from)) && HighLevelEncoder::is_native_x12(&input.char_at(from + 1)) && HighLevelEncoder::is_native_x12(&input.char_at(from + 2)) {
                ::add_edge(edges, Edge::new(input, Mode::X12, from, 3, previous));
            }
            ::add_edge(edges, Edge::new(input, Mode::B256, from, 1, previous));
        }
        //unless it is 2 characters away from the end of the input.
         let mut i: i32;
         {
            i = 0;
            while i < 3 {
                {
                     let pos: i32 = from + i;
                    if input.have_n_characters(pos, 1) && HighLevelEncoder::is_native_e_d_i_f_a_c_t(&input.char_at(pos)) {
                        ::add_edge(edges, Edge::new(input, Mode::EDF, from, i + 1, previous));
                    } else {
                        break;
                    }
                }
                i += 1;
             }
         }

        if i == 3 && input.have_n_characters(from, 4) && HighLevelEncoder::is_native_e_d_i_f_a_c_t(&input.char_at(from + 3)) {
            ::add_edge(edges, Edge::new(input, Mode::EDF, from, 4, previous));
        }
    }

    fn  encode_minimally( input: &Input) -> Result  {
         let input_length: i32 = input.length();
        // Array that represents vertices. There is a vertex for every character and mode.
        // The last dimension in the array below encodes the 6 modes ASCII, C40, TEXT, X12, EDF and B256
         let mut edges: [[Option<Edge>; 6]; input_length + 1] = [[None; 6]; input_length + 1];
        ::add_edges(input, edges, 0, null);
         {
             let mut i: i32 = 1;
            while i <= input_length {
                {
                     {
                         let mut j: i32 = 0;
                        while j < 6 {
                            {
                                if edges[i][j] != null && i < input_length {
                                    ::add_edges(input, edges, i, edges[i][j]);
                                }
                            }
                            j += 1;
                         }
                     }

                    //optimize memory by removing edges that have been passed.
                     {
                         let mut j: i32 = 0;
                        while j < 6 {
                            {
                                edges[i - 1][j] = null;
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

         let minimal_j: i32 = -1;
         let minimal_size: i32 = Integer::MAX_VALUE;
         {
             let mut j: i32 = 0;
            while j < 6 {
                {
                    if edges[input_length][j] != null {
                         let edge: Edge = edges[input_length][j];
                        //C40, TEXT and X12 need an
                         let size: i32 =  if j >= 1 && j <= 3 { edge.cachedTotalSize + 1 } else { edge.cachedTotalSize };
                        // extra unlatch at the end
                        if size < minimal_size {
                            minimal_size = size;
                            minimal_j = j;
                        }
                    }
                }
                j += 1;
             }
         }

        if minimal_j < 0 {
            throw RuntimeException::new(format!("Internal error: failed to encode \"{}\"", input));
        }
        return Result::new(edges[input_length][minimal_j]);
    }


     let all_codeword_capacities: vec![Vec<i32>; 28] = vec![3, 5, 8, 10, 12, 16, 18, 22, 30, 32, 36, 44, 49, 62, 86, 114, 144, 174, 204, 280, 368, 456, 576, 696, 816, 1050, 1304, 1558, ]
    ;

     let square_codeword_capacities: vec![Vec<i32>; 24] = vec![3, 5, 8, 12, 18, 22, 30, 36, 44, 62, 86, 114, 144, 174, 204, 280, 368, 456, 576, 696, 816, 1050, 1304, 1558, ]
    ;

     let rectangular_codeword_capacities: vec![Vec<i32>; 6] = vec![5, 10, 16, 33, 32, 49, ]
    ;
    struct Edge {

         let input: Input;

        //the mode at the start of this edge.
         let mode: Mode;

         let from_position: i32;

         let character_length: i32;

         let previous: Edge;

         let cached_total_size: i32;
    }
    
    impl Edge {

        fn new( input: &Input,  mode: &Mode,  from_position: i32,  character_length: i32,  previous: &Edge) -> Edge {
            let .input = input;
            let .mode = mode;
            let .fromPosition = from_position;
            let .characterLength = character_length;
            let .previous = previous;
            assert!( from_position + character_length <= input.length());
             let mut size: i32 =  if previous != null { previous.cachedTotalSize } else { 0 };
             let previous_mode: Mode = self.get_previous_mode();
            /*
      * Switching modes
      * ASCII -> C40: latch 230
      * ASCII -> TEXT: latch 239
      * ASCII -> X12: latch 238
      * ASCII -> EDF: latch 240
      * ASCII -> B256: latch 231
      * C40 -> ASCII: word(c1,c2,c3), 254
      * TEXT -> ASCII: word(c1,c2,c3), 254
      * X12 -> ASCII: word(c1,c2,c3), 254
      * EDIFACT -> ASCII: Unlatch character,0,0,0 or c1,Unlatch character,0,0 or c1,c2,Unlatch character,0 or 
      * c1,c2,c3,Unlatch character
      * B256 -> ASCII: without latch after n bytes
      */
            match mode {
                  ASCII => 
                     {
                        size += 1;
                        if input.is_e_c_i(from_position) || ::is_extended_a_s_c_i_i(&input.char_at(from_position), &input.get_f_n_c1_character()) {
                            size += 1;
                        }
                        if previous_mode == Mode::C40 || previous_mode == Mode::TEXT || previous_mode == Mode::X12 {
                            // unlatch 254 to ASCII
                            size += 1;
                        }
                        break;
                    }
                  B256 => 
                     {
                        size += 1;
                        if previous_mode != Mode::B256 {
                            //byte count
                            size += 1;
                        } else if self.get_b256_size() == 250 {
                            //extra byte count
                            size += 1;
                        }
                        if previous_mode == Mode::ASCII {
                            //latch to B256
                            size += 1;
                        } else if previous_mode == Mode::C40 || previous_mode == Mode::TEXT || previous_mode == Mode::X12 {
                            //unlatch to ASCII, latch to B256
                            size += 2;
                        }
                        break;
                    }
                  C40 => 
                     {
                    }
                  TEXT => 
                     {
                    }
                  X12 => 
                     {
                        if mode == Mode::X12 {
                            size += 2;
                        } else {
                             let char_len: [i32; 1] = [0; 1];
                            size += ::get_number_of_c40_words(input, from_position, mode == Mode::C40, &char_len) * 2;
                        }
                        if previous_mode == Mode::ASCII || previous_mode == Mode::B256 {
                            //additional byte for latch from ASCII to this mode
                            size += 1;
                        } else if previous_mode != mode && (previous_mode == Mode::C40 || previous_mode == Mode::TEXT || previous_mode == Mode::X12) {
                            //unlatch 254 to ASCII followed by latch to this mode
                            size += 2;
                        }
                        break;
                    }
                  EDF => 
                     {
                        size += 3;
                        if previous_mode == Mode::ASCII || previous_mode == Mode::B256 {
                            //additional byte for latch from ASCII to this mode
                            size += 1;
                        } else if previous_mode == Mode::C40 || previous_mode == Mode::TEXT || previous_mode == Mode::X12 {
                            //unlatch 254 to ASCII followed by latch to this mode
                            size += 2;
                        }
                        break;
                    }
            }
            cached_total_size = size;
        }

        // does not count beyond 250
        fn  get_b256_size(&self) -> i32  {
             let mut cnt: i32 = 0;
             let mut current: Edge = self;
            while current != null && current.mode == Mode::B256 && cnt <= 250 {
                cnt += 1;
                current = current.previous;
            }
            return cnt;
        }

        fn  get_previous_start_mode(&self) -> Mode  {
            return  if self.previous == null { Mode::ASCII } else { self.previous.mode };
        }

        fn  get_previous_mode(&self) -> Mode  {
            return  if self.previous == null { Mode::ASCII } else { self.previous.get_end_mode() };
        }

        /** Returns Mode.ASCII in case that:
     *  - Mode is EDIFACT and characterLength is less than 4 or the remaining characters can be encoded in at most 2
     *    ASCII bytes.
     *  - Mode is C40, TEXT or X12 and the remaining characters can be encoded in at most 1 ASCII byte.
     *  Returns mode in all other cases.
     * */
        fn  get_end_mode(&self) -> Mode  {
            if self.mode == Mode::EDF {
                if self.character_length < 4 {
                    return Mode::ASCII;
                }
                // see 5.2.8.2 EDIFACT encodation Rules
                 let last_a_s_c_i_i: i32 = self.get_last_a_s_c_i_i();
                if last_a_s_c_i_i > 0 && self.get_codewords_remaining(self.cached_total_size + last_a_s_c_i_i) <= 2 - last_a_s_c_i_i {
                    return Mode::ASCII;
                }
            }
            if self.mode == Mode::C40 || self.mode == Mode::TEXT || self.mode == Mode::X12 {
                // see 5.2.5.2 C40 encodation rules and 5.2.7.2 ANSI X12 encodation rules
                if self.from_position + self.character_length >= self.input.length() && self.get_codewords_remaining(self.cached_total_size) == 0 {
                    return Mode::ASCII;
                }
                 let last_a_s_c_i_i: i32 = self.get_last_a_s_c_i_i();
                if last_a_s_c_i_i == 1 && self.get_codewords_remaining(self.cached_total_size + 1) == 0 {
                    return Mode::ASCII;
                }
            }
            return self.mode;
        }

        fn  get_mode(&self) -> Mode  {
            return self.mode;
        }

        /** Peeks ahead and returns 1 if the postfix consists of exactly two digits, 2 if the postfix consists of exactly
     *  two consecutive digits and a non extended character or of 4 digits. 
     *  Returns 0 in any other case
     **/
        fn  get_last_a_s_c_i_i(&self) -> i32  {
             let length: i32 = self.input.length();
             let from: i32 = self.from_position + self.character_length;
            if length - from > 4 || from >= length {
                return 0;
            }
            if length - from == 1 {
                if ::is_extended_a_s_c_i_i(&self.input.char_at(from), &self.input.get_f_n_c1_character()) {
                    return 0;
                }
                return 1;
            }
            if length - from == 2 {
                if ::is_extended_a_s_c_i_i(&self.input.char_at(from), &self.input.get_f_n_c1_character()) || ::is_extended_a_s_c_i_i(&self.input.char_at(from + 1), &self.input.get_f_n_c1_character()) {
                    return 0;
                }
                if HighLevelEncoder::is_digit(&self.input.char_at(from)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 1)) {
                    return 1;
                }
                return 2;
            }
            if length - from == 3 {
                if HighLevelEncoder::is_digit(&self.input.char_at(from)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 1)) && !::is_extended_a_s_c_i_i(&self.input.char_at(from + 2), &self.input.get_f_n_c1_character()) {
                    return 2;
                }
                if HighLevelEncoder::is_digit(&self.input.char_at(from + 1)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 2)) && !::is_extended_a_s_c_i_i(&self.input.char_at(from), &self.input.get_f_n_c1_character()) {
                    return 2;
                }
                return 0;
            }
            if HighLevelEncoder::is_digit(&self.input.char_at(from)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 1)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 2)) && HighLevelEncoder::is_digit(&self.input.char_at(from + 3)) {
                return 2;
            }
            return 0;
        }

        /** Returns the capacity in codewords of the smallest symbol that has enough capacity to fit the given minimal
     * number of codewords.
     **/
        fn  get_min_symbol_size(&self,  minimum: i32) -> i32  {
            match self.input.get_shape_hint() {
                  FORCE_SQUARE => 
                     {
                        for  let capacity: i32 in square_codeword_capacities {
                            if capacity >= minimum {
                                return capacity;
                            }
                        }
                        break;
                    }
                  FORCE_RECTANGLE => 
                     {
                        for  let capacity: i32 in rectangular_codeword_capacities {
                            if capacity >= minimum {
                                return capacity;
                            }
                        }
                        break;
                    }
            }
            for  let capacity: i32 in all_codeword_capacities {
                if capacity >= minimum {
                    return capacity;
                }
            }
            return all_codeword_capacities[all_codeword_capacities.len() - 1];
        }

        /** Returns the remaining capacity in codewords of the smallest symbol that has enough capacity to fit the given
     * minimal number of codewords.
     **/
        fn  get_codewords_remaining(&self,  minimum: i32) -> i32  {
            return self.get_min_symbol_size(minimum) - minimum;
        }

        fn  get_bytes( c: i32) -> Vec<i8>  {
             let mut result: [i8; 1] = [0; 1];
            result[0] = c as i8;
            return result;
        }

        fn  get_bytes( c1: i32,  c2: i32) -> Vec<i8>  {
             let mut result: [i8; 2] = [0; 2];
            result[0] = c1 as i8;
            result[1] = c2 as i8;
            return result;
        }

        fn  set_c40_word( bytes: &Vec<i8>,  offset: i32,  c1: i32,  c2: i32,  c3: i32)   {
             let val16: i32 = (1600 * (c1 & 0xff)) + (40 * (c2 & 0xff)) + (c3 & 0xff) + 1;
            bytes[offset] = (val16 / 256) as i8;
            bytes[offset + 1] = (val16 % 256) as i8;
        }

        fn  get_x12_value( c: char) -> i32  {
            return  if c == 13 { 0 } else {  if c == 42 { 1 } else {  if c == 62 { 2 } else {  if c == 32 { 3 } else {  if c >= 48 && c <= 57 { c - 44 } else {  if c >= 65 && c <= 90 { c - 51 } else { c } } } } } };
        }

        fn  get_x12_words(&self) -> Vec<i8>  {
            assert!( self.character_length % 3 == 0);
             let result: [i8; self.character_length / 3 * 2] = [0; self.character_length / 3 * 2];
             {
                 let mut i: i32 = 0;
                while i < result.len() {
                    {
                        ::set_c40_word(&result, i, &::get_x12_value(&self.input.char_at(self.from_position + i / 2 * 3)), &::get_x12_value(&self.input.char_at(self.from_position + i / 2 * 3 + 1)), &::get_x12_value(&self.input.char_at(self.from_position + i / 2 * 3 + 2)));
                    }
                    i += 2;
                 }
             }

            return result;
        }

        fn  get_shift_value( c: char,  c40: bool,  fnc1: i32) -> i32  {
            return  if (c40 && ::is_in_c40_shift1_set(c) || !c40 && ::is_in_text_shift1_set(c)) { 0 } else {  if (c40 && ::is_in_c40_shift2_set(c, fnc1) || !c40 && ::is_in_text_shift2_set(c, fnc1)) { 1 } else { 2 } };
        }

        fn  get_c40_value( c40: bool,  set_index: i32,  c: char,  fnc1: i32) -> i32  {
            if c == fnc1 {
                assert!( set_index == 2);
                return 27;
            }
            if c40 {
                return  if c <= 31 { c } else {  if c == 32 { 3 } else {  if c <= 47 { c - 33 } else {  if c <= 57 { c - 44 } else {  if c <= 64 { c - 43 } else {  if c <= 90 { c - 51 } else {  if c <= 95 { c - 69 } else {  if c <= 127 { c - 96 } else { c } } } } } } } };
            } else {
                return  if c == 0 { 0 } else {  if //is this a bug in the spec?
                set_index == 0 && c <= 3 { //is this a bug in the spec?
                c - 1 } else {  if set_index == 1 && c <= 31 { c } else {  if c == 32 { 3 } else {  if c >= 33 && c <= 47 { c - 33 } else {  if c >= 48 && c <= 57 { c - 44 } else {  if c >= 58 && c <= 64 { c - 43 } else {  if c >= 65 && c <= 90 { c - 64 } else {  if c >= 91 && c <= 95 { c - 69 } else {  if c == 96 { 0 } else {  if c >= 97 && c <= 122 { c - 83 } else {  if c >= 123 && c <= 127 { c - 96 } else { c } } } } } } } } } } } };
            }
        }

        fn  get_c40_words(&self,  c40: bool,  fnc1: i32) -> Vec<i8>  {
             let c40_values: List<Byte> = ArrayList<>::new();
             {
                 let mut i: i32 = 0;
                while i < self.character_length {
                    {
                         let ci: char = self.input.char_at(self.from_position + i);
                        if c40 && HighLevelEncoder::is_native_c40(ci) || !c40 && HighLevelEncoder::is_native_text(ci) {
                            c40_values.add(::get_c40_value(c40, 0, ci, fnc1) as i8);
                        } else if !::is_extended_a_s_c_i_i(ci, fnc1) {
                             let shift_value: i32 = ::get_shift_value(ci, c40, fnc1);
                            //Shift[123]
                            c40_values.add(shift_value as i8);
                            c40_values.add(::get_c40_value(c40, shift_value, ci, fnc1) as i8);
                        } else {
                             let ascii_value: char = ((ci & 0xff) - 128) as char;
                            if c40 && HighLevelEncoder::is_native_c40(ascii_value) || !c40 && HighLevelEncoder::is_native_text(ascii_value) {
                                //Shift 2
                                c40_values.add(1 as i8);
                                //Upper Shift
                                c40_values.add(30 as i8);
                                c40_values.add(::get_c40_value(c40, 0, ascii_value, fnc1) as i8);
                            } else {
                                //Shift 2
                                c40_values.add(1 as i8);
                                //Upper Shift
                                c40_values.add(30 as i8);
                                 let shift_value: i32 = ::get_shift_value(ascii_value, c40, fnc1);
                                // Shift[123]
                                c40_values.add(shift_value as i8);
                                c40_values.add(::get_c40_value(c40, shift_value, ascii_value, fnc1) as i8);
                            }
                        }
                    }
                    i += 1;
                 }
             }

            if (c40_values.size() % 3) != 0 {
                assert!( (c40_values.size() - 2) % 3 == 0 && self.from_position + self.character_length == self.input.length());
                // pad with 0 (Shift 1)
                c40_values.add(0 as i8);
            }
             let result: [i8; c40_values.size() / 3 * 2] = [0; c40_values.size() / 3 * 2];
             let byte_index: i32 = 0;
             {
                 let mut i: i32 = 0;
                while i < c40_values.size() {
                    {
                        ::set_c40_word(&result, byte_index, c40_values.get(i) & 0xff, c40_values.get(i + 1) & 0xff, c40_values.get(i + 2) & 0xff);
                        byte_index += 2;
                    }
                    i += 3;
                 }
             }

            return result;
        }

        fn  get_e_d_f_bytes(&self) -> Vec<i8>  {
             let number_of_thirds: i32 = Math::ceil(self.character_length / 4.0) as i32;
             let mut result: [i8; number_of_thirds * 3] = [0; number_of_thirds * 3];
             let mut pos: i32 = self.from_position;
             let end_pos: i32 = Math::min(self.from_position + self.character_length - 1, self.input.length() - 1);
             {
                 let mut i: i32 = 0;
                while i < number_of_thirds {
                    {
                         let edf_values: [i32; 4] = [0; 4];
                         {
                             let mut j: i32 = 0;
                            while j < 4 {
                                {
                                    if pos <= end_pos {
                                        edf_values[j] = self.input.char_at(pos += 1 !!!check!!! post increment) & 0x3f;
                                    } else {
                                        edf_values[j] =  if pos == end_pos + 1 { 0x1f } else { 0 };
                                    }
                                }
                                j += 1;
                             }
                         }

                         let mut val24: i32 = edf_values[0] << 18;
                        val24 |= edf_values[1] << 12;
                        val24 |= edf_values[2] << 6;
                        val24 |= edf_values[3];
                        result[i] = ((val24 >> 16) & 0xff) as i8;
                        result[i + 1] = ((val24 >> 8) & 0xff) as i8;
                        result[i + 2] = (val24 & 0xff) as i8;
                    }
                    i += 3;
                 }
             }

            return result;
        }

        fn  get_latch_bytes(&self) -> Vec<i8>  {
            match self.get_previous_mode() {
                  ASCII => 
                     {
                    }
                  //after B256 ends (via length) we are back to ASCII
                B256 => 
                     {
                        match self.mode {
                              B256 => 
                                 {
                                    return ::get_bytes(231);
                                }
                              C40 => 
                                 {
                                    return ::get_bytes(230);
                                }
                              TEXT => 
                                 {
                                    return ::get_bytes(239);
                                }
                              X12 => 
                                 {
                                    return ::get_bytes(238);
                                }
                              EDF => 
                                 {
                                    return ::get_bytes(240);
                                }
                        }
                        break;
                    }
                  C40 => 
                     {
                    }
                  TEXT => 
                     {
                    }
                  X12 => 
                     {
                        if self.mode != self.get_previous_mode() {
                            match self.mode {
                                  ASCII => 
                                     {
                                        return ::get_bytes(254);
                                    }
                                  B256 => 
                                     {
                                        return ::get_bytes(254, 231);
                                    }
                                  C40 => 
                                     {
                                        return ::get_bytes(254, 230);
                                    }
                                  TEXT => 
                                     {
                                        return ::get_bytes(254, 239);
                                    }
                                  X12 => 
                                     {
                                        return ::get_bytes(254, 238);
                                    }
                                  EDF => 
                                     {
                                        return ::get_bytes(254, 240);
                                    }
                            }
                        }
                        break;
                    }
                  EDF => 
                     {
                        //The rightmost EDIFACT edge always contains an unlatch character
                        assert!( self.mode == Mode::EDF);
                        break;
                    }
            }
            return : [i8; 0] = [0; 0];
        }

        // Important: The function does not return the length bytes (one or two) in case of B256 encoding
        fn  get_data_bytes(&self) -> Vec<i8>  {
            match self.mode {
                  ASCII => 
                     {
                        if self.input.is_e_c_i(self.from_position) {
                            return ::get_bytes(241, self.input.get_e_c_i_value(self.from_position) + 1);
                        } else if ::is_extended_a_s_c_i_i(&self.input.char_at(self.from_position), &self.input.get_f_n_c1_character()) {
                            return ::get_bytes(235, self.input.char_at(self.from_position) - 127);
                        } else if self.character_length == 2 {
                            return ::get_bytes((self.input.char_at(self.from_position) - '0') * 10 + self.input.char_at(self.from_position + 1) - '0' + 130);
                        } else if self.input.is_f_n_c1(self.from_position) {
                            return ::get_bytes(232);
                        } else {
                            return ::get_bytes(self.input.char_at(self.from_position) + 1);
                        }
                    }
                  B256 => 
                     {
                        return ::get_bytes(&self.input.char_at(self.from_position));
                    }
                  C40 => 
                     {
                        return self.get_c40_words(true, &self.input.get_f_n_c1_character());
                    }
                  TEXT => 
                     {
                        return self.get_c40_words(false, &self.input.get_f_n_c1_character());
                    }
                  X12 => 
                     {
                        return self.get_x12_words();
                    }
                  EDF => 
                     {
                        return self.get_e_d_f_bytes();
                    }
            }
            assert!( false);
            return : [i8; 0] = [0; 0];
        }
    }


    struct Result {

         let mut bytes: Vec<i8>;
    }
    
    impl Result {

        fn new( solution: &Edge) -> Result {
             let input: Input = solution.input;
             let mut size: i32 = 0;
             let bytes_a_l: List<Byte> = ArrayList<>::new();
             let randomize_postfix_length: List<Integer> = ArrayList<>::new();
             let randomize_lengths: List<Integer> = ArrayList<>::new();
            if (solution.mode == Mode::C40 || solution.mode == Mode::TEXT || solution.mode == Mode::X12) && solution.get_end_mode() != Mode::ASCII {
                size += ::prepend(&MinimalEncoder::Edge::get_bytes(254), &bytes_a_l);
            }
             let mut current: Edge = solution;
            while current != null {
                size += ::prepend(&current.get_data_bytes(), &bytes_a_l);
                if current.previous == null || current.get_previous_start_mode() != current.get_mode() {
                    if current.get_mode() == Mode::B256 {
                        if size <= 249 {
                            bytes_a_l.add(0, size as i8);
                            size += 1;
                        } else {
                            bytes_a_l.add(0, (size % 250) as i8);
                            bytes_a_l.add(0, (size / 250 + 249) as i8);
                            size += 2;
                        }
                        randomize_postfix_length.add(&bytes_a_l.size());
                        randomize_lengths.add(size);
                    }
                    ::prepend(&current.get_latch_bytes(), &bytes_a_l);
                    size = 0;
                }
                current = current.previous;
            }
            if input.get_macro_id() == 5 {
                size += ::prepend(&MinimalEncoder::Edge::get_bytes(236), &bytes_a_l);
            } else if input.get_macro_id() == 6 {
                size += ::prepend(&MinimalEncoder::Edge::get_bytes(237), &bytes_a_l);
            }
            if input.get_f_n_c1_character() > 0 {
                size += ::prepend(&MinimalEncoder::Edge::get_bytes(232), &bytes_a_l);
            }
             {
                 let mut i: i32 = 0;
                while i < randomize_postfix_length.size() {
                    {
                        ::apply_random_pattern(&bytes_a_l, bytes_a_l.size() - randomize_postfix_length.get(i), &randomize_lengths.get(i));
                    }
                    i += 1;
                 }
             }

            //add padding
             let capacity: i32 = solution.get_min_symbol_size(&bytes_a_l.size());
            if bytes_a_l.size() < capacity {
                bytes_a_l.add(129 as i8);
            }
            while bytes_a_l.size() < capacity {
                bytes_a_l.add(::randomize253_state(bytes_a_l.size() + 1) as i8);
            }
            bytes = : [i8; bytes_a_l.size()] = [0; bytes_a_l.size()];
             {
                 let mut i: i32 = 0;
                while i < bytes.len() {
                    {
                        bytes[i] = bytes_a_l.get(i);
                    }
                    i += 1;
                 }
             }

        }

        fn  prepend( bytes: &Vec<i8>,  into: &List<Byte>) -> i32  {
             {
                 let mut i: i32 = bytes.len() - 1;
                while i >= 0 {
                    {
                        into.add(0, bytes[i]);
                    }
                    i -= 1;
                 }
             }

            return bytes.len();
        }

        fn  randomize253_state( codeword_position: i32) -> i32  {
             let pseudo_random: i32 = ((149 * codeword_position) % 253) + 1;
             let temp_variable: i32 = 129 + pseudo_random;
            return  if temp_variable <= 254 { temp_variable } else { temp_variable - 254 };
        }

        fn  apply_random_pattern( bytes_a_l: &List<Byte>,  start_position: i32,  length: i32)   {
             {
                 let mut i: i32 = 0;
                while i < length {
                    {
                        //See "B.1 253-state algorithm
                         const Pad_codeword_position: i32 = start_position + i;
                         const Pad_codeword_value: i32 = bytes_a_l.get(Pad_codeword_position) & 0xff;
                         let pseudo_random_number: i32 = ((149 * (Pad_codeword_position + 1)) % 255) + 1;
                         let temp_variable: i32 = Pad_codeword_value + pseudo_random_number;
                        bytes_a_l.set(Pad_codeword_position, ( if temp_variable <= 255 { temp_variable } else { temp_variable - 256 }) as i8);
                    }
                    i += 1;
                 }
             }

        }

        pub fn  get_bytes(&self) -> Vec<i8>  {
            return self.bytes;
        }
    }


    struct Input {
        super: MinimalECIInput;

         let shape: SymbolShapeHint;

         let macro_id: i32;
    }
    
    impl Input {

        fn new( string_to_encode: &String,  priority_charset: &Charset,  fnc1: i32,  shape: &SymbolShapeHint,  macro_id: i32) -> Input {
            super(&string_to_encode, &priority_charset, fnc1);
            let .shape = shape;
            let .macroId = macro_id;
        }

        fn  get_macro_id(&self) -> i32  {
            return self.macro_id;
        }

        fn  get_shape_hint(&self) -> SymbolShapeHint  {
            return self.shape;
        }
    }

}

// SymbolInfo.java
/**
 * Symbol info table for DataMatrix.
 *
 * @version $Id$
 */

const PROD_SYMBOLS: vec![Vec<SymbolInfo>; 30] = vec![SymbolInfo::new(false, 3, 5, 8, 8, 1), SymbolInfo::new(false, 5, 7, 10, 10, 1), /*rect*/
SymbolInfo::new(true, 5, 7, 16, 6, 1), SymbolInfo::new(false, 8, 10, 12, 12, 1), /*rect*/
SymbolInfo::new(true, 10, 11, 14, 6, 2), SymbolInfo::new(false, 12, 12, 14, 14, 1), /*rect*/
SymbolInfo::new(true, 16, 14, 24, 10, 1), SymbolInfo::new(false, 18, 14, 16, 16, 1), SymbolInfo::new(false, 22, 18, 18, 18, 1), /*rect*/
SymbolInfo::new(true, 22, 18, 16, 10, 2), SymbolInfo::new(false, 30, 20, 20, 20, 1), /*rect*/
SymbolInfo::new(true, 32, 24, 16, 14, 2), SymbolInfo::new(false, 36, 24, 22, 22, 1), SymbolInfo::new(false, 44, 28, 24, 24, 1), /*rect*/
SymbolInfo::new(true, 49, 28, 22, 14, 2), SymbolInfo::new(false, 62, 36, 14, 14, 4), SymbolInfo::new(false, 86, 42, 16, 16, 4), SymbolInfo::new(false, 114, 48, 18, 18, 4), SymbolInfo::new(false, 144, 56, 20, 20, 4), SymbolInfo::new(false, 174, 68, 22, 22, 4), SymbolInfo::new(false, 204, 84, 24, 24, 4, 102, 42), SymbolInfo::new(false, 280, 112, 14, 14, 16, 140, 56), SymbolInfo::new(false, 368, 144, 16, 16, 16, 92, 36), SymbolInfo::new(false, 456, 192, 18, 18, 16, 114, 48), SymbolInfo::new(false, 576, 224, 20, 20, 16, 144, 56), SymbolInfo::new(false, 696, 272, 22, 22, 16, 174, 68), SymbolInfo::new(false, 816, 336, 24, 24, 16, 136, 56), SymbolInfo::new(false, 1050, 408, 18, 18, 36, 175, 68), SymbolInfo::new(false, 1304, 496, 20, 20, 36, 163, 62), DataMatrixSymbolInfo144::new(), ]
;

 let mut symbols: Vec<SymbolInfo> = PROD_SYMBOLS;
pub struct SymbolInfo {

     let rectangular: bool;

     let data_capacity: i32;

     let error_codewords: i32;

     let matrix_width: i32;

     let matrix_height: i32;

     let data_regions: i32;

     let rs_block_data: i32;

     let rs_block_error: i32;
}

impl SymbolInfo {

    /**
   * Overrides the symbol info set used by this class. Used for testing purposes.
   *
   * @param override the symbol info set to use
   */
    pub fn  override_symbol_set( override: &Vec<SymbolInfo>)   {
        symbols = override;
    }

    pub fn new( rectangular: bool,  data_capacity: i32,  error_codewords: i32,  matrix_width: i32,  matrix_height: i32,  data_regions: i32) -> SymbolInfo {
        this(rectangular, data_capacity, error_codewords, matrix_width, matrix_height, data_regions, data_capacity, error_codewords);
    }

    fn new( rectangular: bool,  data_capacity: i32,  error_codewords: i32,  matrix_width: i32,  matrix_height: i32,  data_regions: i32,  rs_block_data: i32,  rs_block_error: i32) -> SymbolInfo {
        let .rectangular = rectangular;
        let .dataCapacity = data_capacity;
        let .errorCodewords = error_codewords;
        let .matrixWidth = matrix_width;
        let .matrixHeight = matrix_height;
        let .dataRegions = data_regions;
        let .rsBlockData = rs_block_data;
        let .rsBlockError = rs_block_error;
    }

    pub fn  lookup( data_codewords: i32) -> SymbolInfo  {
        return ::lookup(data_codewords, SymbolShapeHint::FORCE_NONE, true);
    }

    pub fn  lookup( data_codewords: i32,  shape: &SymbolShapeHint) -> SymbolInfo  {
        return ::lookup(data_codewords, shape, true);
    }

    pub fn  lookup( data_codewords: i32,  allow_rectangular: bool,  fail: bool) -> SymbolInfo  {
         let shape: SymbolShapeHint =  if allow_rectangular { SymbolShapeHint::FORCE_NONE } else { SymbolShapeHint::FORCE_SQUARE };
        return ::lookup(data_codewords, shape, fail);
    }

    fn  lookup( data_codewords: i32,  shape: &SymbolShapeHint,  fail: bool) -> SymbolInfo  {
        return ::lookup(data_codewords, shape, null, null, fail);
    }

    pub fn  lookup( data_codewords: i32,  shape: &SymbolShapeHint,  min_size: &Dimension,  max_size: &Dimension,  fail: bool) -> SymbolInfo  {
        for  let symbol: SymbolInfo in symbols {
            if shape == SymbolShapeHint::FORCE_SQUARE && symbol.rectangular {
                continue;
            }
            if shape == SymbolShapeHint::FORCE_RECTANGLE && !symbol.rectangular {
                continue;
            }
            if min_size != null && (symbol.get_symbol_width() < min_size.get_width() || symbol.get_symbol_height() < min_size.get_height()) {
                continue;
            }
            if max_size != null && (symbol.get_symbol_width() > max_size.get_width() || symbol.get_symbol_height() > max_size.get_height()) {
                continue;
            }
            if data_codewords <= symbol.dataCapacity {
                return symbol;
            }
        }
        if fail {
            throw IllegalArgumentException::new(format!("Can't find a symbol arrangement that matches the message. Data codewords: {}", data_codewords));
        }
        return null;
    }

    fn  get_horizontal_data_regions(&self) -> i32  {
        match self.data_regions {
              1 => 
                 {
                    return 1;
                }
              2 => 
                 {
                }
              4 => 
                 {
                    return 2;
                }
              16 => 
                 {
                    return 4;
                }
              36 => 
                 {
                    return 6;
                }
            _ => 
                 {
                    throw IllegalStateException::new("Cannot handle this number of data regions");
                }
        }
    }

    fn  get_vertical_data_regions(&self) -> i32  {
        match self.data_regions {
              1 => 
                 {
                }
              2 => 
                 {
                    return 1;
                }
              4 => 
                 {
                    return 2;
                }
              16 => 
                 {
                    return 4;
                }
              36 => 
                 {
                    return 6;
                }
            _ => 
                 {
                    throw IllegalStateException::new("Cannot handle this number of data regions");
                }
        }
    }

    pub fn  get_symbol_data_width(&self) -> i32  {
        return self.get_horizontal_data_regions() * self.matrix_width;
    }

    pub fn  get_symbol_data_height(&self) -> i32  {
        return self.get_vertical_data_regions() * self.matrix_height;
    }

    pub fn  get_symbol_width(&self) -> i32  {
        return self.get_symbol_data_width() + (self.get_horizontal_data_regions() * 2);
    }

    pub fn  get_symbol_height(&self) -> i32  {
        return self.get_symbol_data_height() + (self.get_vertical_data_regions() * 2);
    }

    pub fn  get_codeword_count(&self) -> i32  {
        return self.data_capacity + self.error_codewords;
    }

    pub fn  get_interleaved_block_count(&self) -> i32  {
        return self.data_capacity / self.rs_block_data;
    }

    pub fn  get_data_capacity(&self) -> i32  {
        return self.data_capacity;
    }

    pub fn  get_error_codewords(&self) -> i32  {
        return self.error_codewords;
    }

    pub fn  get_data_length_for_interleaved_block(&self,  index: i32) -> i32  {
        return self.rs_block_data;
    }

    pub fn  get_error_length_for_interleaved_block(&self,  index: i32) -> i32  {
        return self.rs_block_error;
    }

    pub fn  to_string(&self) -> String  {
        return format!("{} data region {}x{}, symbol size {}x{}, symbol data size {}x{}, codewords {}+{}", ( if self.rectangular { "Rectangular Symbol:" } else { "Square Symbol:" }), self.matrix_width, self.matrix_height, self.get_symbol_width(), self.get_symbol_height(), self.get_symbol_data_width(), self.get_symbol_data_height(), self.data_capacity, self.error_codewords);
    }
}


// SymbolShapeHint.java
/**
 * Enumeration for DataMatrix symbol shape hint. It can be used to force square or rectangular
 * symbols.
 */
pub enum SymbolShapeHint {

    FORCE_NONE(), FORCE_SQUARE(), FORCE_RECTANGLE()
}

// TextEncoder.java
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

// X12Encoder.java
struct X12Encoder {
    super: C40Encoder;
}

impl X12Encoder {

    pub fn  get_encoding_mode(&self) -> i32  {
        return HighLevelEncoder::X12_ENCODATION;
    }

    pub fn  encode(&self,  context: &EncoderContext)   {
        //step C
         let buffer: StringBuilder = StringBuilder::new();
        while context.has_more_characters() {
             let c: char = context.get_current_char();
            context.pos += 1;
            self.encode_char(c, &buffer);
             let count: i32 = buffer.length();
            if (count % 3) == 0 {
                write_next_triplet(context, &buffer);
                 let new_mode: i32 = HighLevelEncoder::look_ahead_test(&context.get_message(), context.pos, &self.get_encoding_mode());
                if new_mode != self.get_encoding_mode() {
                    // Return to ASCII encodation, which will actually handle latch to new mode
                    context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
                    break;
                }
            }
        }
        self.handle_e_o_d(context, &buffer);
    }

    fn  encode_char(&self,  c: char,  sb: &StringBuilder) -> i32  {
        match c {
              '\r' => 
                 {
                    sb.append('\0');
                    break;
                }
              '*' => 
                 {
                    sb.append('\1');
                    break;
                }
              '>' => 
                 {
                    sb.append('\2');
                    break;
                }
              ' ' => 
                 {
                    sb.append('\3');
                    break;
                }
            _ => 
                 {
                    if c >= '0' && c <= '9' {
                        sb.append((c - 48 + 4) as char);
                    } else if c >= 'A' && c <= 'Z' {
                        sb.append((c - 65 + 14) as char);
                    } else {
                        HighLevelEncoder::illegal_character(c);
                    }
                    break;
                }
        }
        return 1;
    }

    fn  handle_e_o_d(&self,  context: &EncoderContext,  buffer: &StringBuilder)   {
        context.update_symbol_info();
         let available: i32 = context.get_symbol_info().get_data_capacity() - context.get_codeword_count();
         let count: i32 = buffer.length();
        context.pos -= count;
        if context.get_remaining_characters() > 1 || available > 1 || context.get_remaining_characters() != available {
            context.write_codeword(HighLevelEncoder::X12_UNLATCH);
        }
        if context.get_new_encoding() < 0 {
            context.signal_encoder_change(HighLevelEncoder::ASCII_ENCODATION);
        }
    }
}

