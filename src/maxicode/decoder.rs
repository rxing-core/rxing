use crate::common::{BitMatrix,DecoderResult};
use crate::{FormatException,ChecksumException,DecodeHintType,FormatException};
use crate::common::reedsolomon::{GenericGF,ReedSolomonDecoder,ReedSolomonException};


// BitMatrixParser.java
/**
 * @author mike32767
 * @author Manuel Kasten
 */

const BITNR: vec![vec![Vec<Vec<i32>>; 30]; 33] = vec![vec![121, 120, 127, 126, 133, 132, 139, 138, 145, 144, 151, 150, 157, 156, 163, 162, 169, 168, 175, 174, 181, 180, 187, 186, 193, 192, 199, 198, -2, -2, ]
, vec![123, 122, 129, 128, 135, 134, 141, 140, 147, 146, 153, 152, 159, 158, 165, 164, 171, 170, 177, 176, 183, 182, 189, 188, 195, 194, 201, 200, 816, -3, ]
, vec![125, 124, 131, 130, 137, 136, 143, 142, 149, 148, 155, 154, 161, 160, 167, 166, 173, 172, 179, 178, 185, 184, 191, 190, 197, 196, 203, 202, 818, 817, ]
, vec![283, 282, 277, 276, 271, 270, 265, 264, 259, 258, 253, 252, 247, 246, 241, 240, 235, 234, 229, 228, 223, 222, 217, 216, 211, 210, 205, 204, 819, -3, ]
, vec![285, 284, 279, 278, 273, 272, 267, 266, 261, 260, 255, 254, 249, 248, 243, 242, 237, 236, 231, 230, 225, 224, 219, 218, 213, 212, 207, 206, 821, 820, ]
, vec![287, 286, 281, 280, 275, 274, 269, 268, 263, 262, 257, 256, 251, 250, 245, 244, 239, 238, 233, 232, 227, 226, 221, 220, 215, 214, 209, 208, 822, -3, ]
, vec![289, 288, 295, 294, 301, 300, 307, 306, 313, 312, 319, 318, 325, 324, 331, 330, 337, 336, 343, 342, 349, 348, 355, 354, 361, 360, 367, 366, 824, 823, ]
, vec![291, 290, 297, 296, 303, 302, 309, 308, 315, 314, 321, 320, 327, 326, 333, 332, 339, 338, 345, 344, 351, 350, 357, 356, 363, 362, 369, 368, 825, -3, ]
, vec![293, 292, 299, 298, 305, 304, 311, 310, 317, 316, 323, 322, 329, 328, 335, 334, 341, 340, 347, 346, 353, 352, 359, 358, 365, 364, 371, 370, 827, 826, ]
, vec![409, 408, 403, 402, 397, 396, 391, 390, 79, 78, -2, -2, 13, 12, 37, 36, 2, -1, 44, 43, 109, 108, 385, 384, 379, 378, 373, 372, 828, -3, ]
, vec![411, 410, 405, 404, 399, 398, 393, 392, 81, 80, 40, -2, 15, 14, 39, 38, 3, -1, -1, 45, 111, 110, 387, 386, 381, 380, 375, 374, 830, 829, ]
, vec![413, 412, 407, 406, 401, 400, 395, 394, 83, 82, 41, -3, -3, -3, -3, -3, 5, 4, 47, 46, 113, 112, 389, 388, 383, 382, 377, 376, 831, -3, ]
, vec![415, 414, 421, 420, 427, 426, 103, 102, 55, 54, 16, -3, -3, -3, -3, -3, -3, -3, 20, 19, 85, 84, 433, 432, 439, 438, 445, 444, 833, 832, ]
, vec![417, 416, 423, 422, 429, 428, 105, 104, 57, 56, -3, -3, -3, -3, -3, -3, -3, -3, 22, 21, 87, 86, 435, 434, 441, 440, 447, 446, 834, -3, ]
, vec![419, 418, 425, 424, 431, 430, 107, 106, 59, 58, -3, -3, -3, -3, -3, -3, -3, -3, -3, 23, 89, 88, 437, 436, 443, 442, 449, 448, 836, 835, ]
, vec![481, 480, 475, 474, 469, 468, 48, -2, 30, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, 0, 53, 52, 463, 462, 457, 456, 451, 450, 837, -3, ]
, vec![483, 482, 477, 476, 471, 470, 49, -1, -2, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -2, -1, 465, 464, 459, 458, 453, 452, 839, 838, ]
, vec![485, 484, 479, 478, 473, 472, 51, 50, 31, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, 1, -2, 42, 467, 466, 461, 460, 455, 454, 840, -3, ]
, vec![487, 486, 493, 492, 499, 498, 97, 96, 61, 60, -3, -3, -3, -3, -3, -3, -3, -3, -3, 26, 91, 90, 505, 504, 511, 510, 517, 516, 842, 841, ]
, vec![489, 488, 495, 494, 501, 500, 99, 98, 63, 62, -3, -3, -3, -3, -3, -3, -3, -3, 28, 27, 93, 92, 507, 506, 513, 512, 519, 518, 843, -3, ]
, vec![491, 490, 497, 496, 503, 502, 101, 100, 65, 64, 17, -3, -3, -3, -3, -3, -3, -3, 18, 29, 95, 94, 509, 508, 515, 514, 521, 520, 845, 844, ]
, vec![559, 558, 553, 552, 547, 546, 541, 540, 73, 72, 32, -3, -3, -3, -3, -3, -3, 10, 67, 66, 115, 114, 535, 534, 529, 528, 523, 522, 846, -3, ]
, vec![561, 560, 555, 554, 549, 548, 543, 542, 75, 74, -2, -1, 7, 6, 35, 34, 11, -2, 69, 68, 117, 116, 537, 536, 531, 530, 525, 524, 848, 847, ]
, vec![563, 562, 557, 556, 551, 550, 545, 544, 77, 76, -2, 33, 9, 8, 25, 24, -1, -2, 71, 70, 119, 118, 539, 538, 533, 532, 527, 526, 849, -3, ]
, vec![565, 564, 571, 570, 577, 576, 583, 582, 589, 588, 595, 594, 601, 600, 607, 606, 613, 612, 619, 618, 625, 624, 631, 630, 637, 636, 643, 642, 851, 850, ]
, vec![567, 566, 573, 572, 579, 578, 585, 584, 591, 590, 597, 596, 603, 602, 609, 608, 615, 614, 621, 620, 627, 626, 633, 632, 639, 638, 645, 644, 852, -3, ]
, vec![569, 568, 575, 574, 581, 580, 587, 586, 593, 592, 599, 598, 605, 604, 611, 610, 617, 616, 623, 622, 629, 628, 635, 634, 641, 640, 647, 646, 854, 853, ]
, vec![727, 726, 721, 720, 715, 714, 709, 708, 703, 702, 697, 696, 691, 690, 685, 684, 679, 678, 673, 672, 667, 666, 661, 660, 655, 654, 649, 648, 855, -3, ]
, vec![729, 728, 723, 722, 717, 716, 711, 710, 705, 704, 699, 698, 693, 692, 687, 686, 681, 680, 675, 674, 669, 668, 663, 662, 657, 656, 651, 650, 857, 856, ]
, vec![731, 730, 725, 724, 719, 718, 713, 712, 707, 706, 701, 700, 695, 694, 689, 688, 683, 682, 677, 676, 671, 670, 665, 664, 659, 658, 653, 652, 858, -3, ]
, vec![733, 732, 739, 738, 745, 744, 751, 750, 757, 756, 763, 762, 769, 768, 775, 774, 781, 780, 787, 786, 793, 792, 799, 798, 805, 804, 811, 810, 860, 859, ]
, vec![735, 734, 741, 740, 747, 746, 753, 752, 759, 758, 765, 764, 771, 770, 777, 776, 783, 782, 789, 788, 795, 794, 801, 800, 807, 806, 813, 812, 861, -3, ]
, vec![737, 736, 743, 742, 749, 748, 755, 754, 761, 760, 767, 766, 773, 772, 779, 778, 785, 784, 791, 790, 797, 796, 803, 802, 809, 808, 815, 814, 863, 862, ]
, ]
;
struct BitMatrixParser {

      bit_matrix: BitMatrix
}

impl BitMatrixParser {

    /**
   * @param bitMatrix {@link BitMatrix} to parse
   */
    fn new( bit_matrix: &BitMatrix) -> Self {
        Self {
            bit_matrix
        }
    }

    fn  read_codewords(&self) -> Vec<i8>  {
         let mut result: [i8; 144] = [0; 144];
         let height: i32 = self.bit_matrix.get_height();
         let width: i32 = self.bit_matrix.get_width();
         {
             let mut y: i32 = 0;
            while y < height {
                {
                     let bitnr_row: Vec<i32> = BITNR[y];
                     {
                         let mut x: i32 = 0;
                        while x < width {
                            {
                                 let mut bit: i32 = bitnr_row[x];
                                if bit >= 0 && self.bit_matrix.get(x, y) {
                                    result[bit / 6] |= (1 << (5 - (bit % 6))) as i8;
                                }
                            }
                            x += 1;
                         }
                     }

                }
                y += 1;
             }
         }

        return result;
    }
}


// DecodedBitStreamParser.java

/**
 * <p>MaxiCodes can encode text or structured information as bits in one of several modes,
 * with multiple character sets in one code. This class decodes the bits back into text.</p>
 *
 * @author mike32767
 * @author Manuel Kasten
 */

const SHIFTA: char = '\u{FFF0}';

const SHIFTB: char = '\u{FFF1}';

const SHIFTC: char = '\u{FFF2}';

const SHIFTD: char = '\u{FFF3}';

const SHIFTE: char = '\u{FFF4}';

const TWOSHIFTA: char ='\u{FFF5}';

const THREESHIFTA: char ='\u{FFF6}';

const LATCHA: char =  '\u{FFF7}';

const LATCHB: char = '\u{FFF8}';

const LOCK: char = '\u{FFF9}';

const ECI: char ='\u{FFFA}';

const NS: char ='\u{FFFB}';

const PAD: char =  '\u{FFFC}';

const FS: char = '\u{001C}';

const GS: char = '\u{001D}';

const RS: char = '\u{001E}';

const COUNTRY_BYTES: vec![Vec<i8>; 10] = vec![53, 54, 43, 44, 45, 46, 47, 48, 37, 38, ]
;

const SERVICE_CLASS_BYTES: vec![Vec<i8>; 10] = vec![55, 56, 57, 58, 59, 60, 49, 50, 51, 52, ]
;

const POSTCODE_2_LENGTH_BYTES: vec![Vec<i8>; 6] = vec![39, 40, 41, 42, 31, 32, ]
;

const POSTCODE_2_BYTES: vec![Vec<i8>; 30] = vec![33, 34, 35, 36, 25, 26, 27, 28, 29, 30, 19, 20, 21, 22, 23, 24, 13, 14, 15, 16, 17, 18, 7, 8, 9, 10, 11, 12, 1, 2, ]
;

const POSTCODE_3_BYTES: vec![vec![Vec<Vec<i8>>; 6]; 6] = vec![vec![39, 40, 41, 42, 31, 32, ]
, vec![33, 34, 35, 36, 25, 26, ]
, vec![27, 28, 29, 30, 19, 20, ]
, vec![21, 22, 23, 24, 13, 14, ]
, vec![15, 16, 17, 18, 7, 8, ]
, vec![9, 10, 11, 12, 1, 2, ]
, ]
;


const SETS: vec![Vec<String>; 5] = vec![
    format!("\rABCDEFGHIJKLMNOPQRSTUVWXYZ{}{}{}{}{} {}\"#$%&'()*+,-./0123456789:{}{}{}{}{}" , ECI , FS , GS , RS , NS , PAD ,SHIFTB , SHIFTC , SHIFTD , SHIFTE , LATCHB),
    format!("`abcdefghijklmnopqrstuvwxyz{}{}{}{}{}\{{}\}{}{}{}{}{}{}{}{}{}", ECI, FS, GS , RS , NS , PAD ,PAD , TWOSHIFTA , THREESHIFTA , PAD ,SHIFTA , SHIFTC , SHIFTD, SHIFTE , LATCHA),
    format!("\u{00C0}\u{00C1}\u{00C2}\u{00C3}\u00C4\u00C5\u00C6\u00C7\u00C8\u00C9\u00CA\u00CB\u00CC\u00CD\u00CE\u00CF\u00D0\u00D1\u00D2\u00D3\u00D4\u00D5\u00D6\u00D7\u00D8\u00D9\u00DA{}{}{}{}{}\u00DB\u00DC\u00DD\u00DE\u00DF\u00AA\u00AC\u00B1\u00B2\u00B3\u00B5\u00B9\u00BA\u00BC\u00BD\u00BE\u0080\u0081\u0082\u0083\u0084\u0085\u0086\u0087\u0088\u0089{} {}{}{}{}",ECI , FS , GS , RS , NS ,LATCHA ,  LOCK , SHIFTD , SHIFTE , LATCHB),
    format!("\u{00E0}\u{00E1}\u{00E2}\u{00E3}\u00E4\u00E5\u00E6\u00E7\u00E8\u00E9\u00EA\u00EB\u00EC\u00ED\u00EE\u00EF\u00F0\u00F1\u00F2\u00F3\u00F4\u00F5\u00F6\u00F7\u00F8\u00F9\u00FA{}{}{}{}{}\u00FB\u00FC\u00FD\u00FE\u00FF\u00A1\u00A8\u00AB\u00AF\u00B0\u00B4\u00B7\u00B8\u00BB\u00BF\u008A\u008B\u008C\u008D\u008E\u008F\u0090\u0091\u0092\u0093\u0094{} {}{}{}{}" ,ECI , FS , GS, RS , NS ,LATCHA , SHIFTC , LOCK , SHIFTE , LATCHB),
    format!("\u{0000}\u{0001}\u{0002}\u{0003}\u0004\u0005\u0006\u0007\u0008\u0009\n\u000B\u000C\r\u000E\u000F\u0010\u0011\u0012\u0013\u0014\u0015\u0016\u0017\u0018\u0019\u001A{}{}{}\u001B{}{}{}{}\u001F\u009F\u00A0\u00A2\u00A3\u00A4\u00A5\u00A6\u00A7\u00A9\u00AD\u00AE\u00B6\u0095\u0096\u0097\u0098\u0099\u009A\u009B\u009C\u009D\u009E{} {}{}{}{}" ,ECI , PAD, PAD , NS , FS , GS , RS ,        LATCHA , SHIFTC , SHIFTD , LOCK , LATCHB)
];

struct DecodedBitStreamParser {
}

impl DecodedBitStreamParser {

    fn new() -> DecodedBitStreamParser {
    }

    fn  decode( bytes: &Vec<i8>,  mode: i32) -> /*  throws FormatException */Result<DecoderResult, Rc<Exception>>   {
         let result: StringBuilder = StringBuilder::new(144);
        match mode {
              2 => 
                 {
                }
              3 => 
                 {
                     let mut postcode: String;
                    if mode == 2 {
                         let pc: i32 = ::get_post_code2(&bytes);
                         let ps2_length: i32 = ::get_post_code2_length(&bytes);
                        if ps2_length > 10 {
                            return Err( FormatException::get_format_instance());
                        }
                         let df: NumberFormat = DecimalFormat::new(&"0000000000".substring(0, ps2_length));
                        postcode = df.format(pc);
                    } else {
                        postcode = ::get_post_code3(&bytes);
                    }
                     let three_digits: NumberFormat = DecimalFormat::new("000");
                     let country: String = three_digits.format(&::get_country(&bytes));
                     let service: String = three_digits.format(&::get_service_class(&bytes));
                    result.append(&::get_message(&bytes, 10, 84));
                    if result.to_string().starts_with(format!("[)>{}01{}", RS, GS)) {
                        result.insert(9, format!("{}{}{}{}{}{}", postcode, GS, country, GS, service, GS));
                    } else {
                        result.insert(0, format!("{}{}{}{}{}{}", postcode, GS, country, GS, service, GS));
                    }
                }
              4 => 
                 {
                    result.append(&::get_message(&bytes, 1, 93));
                }
              5 => 
                 {
                    result.append(&::get_message(&bytes, 1, 77));
                }
        }
        return Ok(DecoderResult::new(&bytes, &result.to_string(), null, &String::value_of(mode), None, None, None));
    }

    fn  get_bit( bit: i32,  bytes: &Vec<i8>) -> i32  {
        bit -= 1;
        return  if (bytes[bit / 6] & (1 << (5 - (bit % 6)))) == 0 { 0 } else { 1 };
    }

    fn  get_int( bytes: &Vec<i8>,  x: &Vec<i8>) -> i32  {
         let mut val: i32 = 0;
         {
             let mut i: i32 = 0;
            while i < x.len() {
                {
                    val += ::get_bit(x[i], &bytes) << (x.len() - i - 1);
                }
                i += 1;
             }
         }

        return val;
    }

    fn  get_country( bytes: &Vec<i8>) -> i32  {
        return ::get_int(&bytes, &COUNTRY_BYTES);
    }

    fn  get_service_class( bytes: &Vec<i8>) -> i32  {
        return ::get_int(&bytes, &SERVICE_CLASS_BYTES);
    }

    fn  get_post_code2_length( bytes: &Vec<i8>) -> i32  {
        return ::get_int(&bytes, &POSTCODE_2_LENGTH_BYTES);
    }

    fn  get_post_code2( bytes: &Vec<i8>) -> i32  {
        return ::get_int(&bytes, &POSTCODE_2_BYTES);
    }

    fn  get_post_code3( bytes: &Vec<i8>) -> String  {
         let sb: StringBuilder = StringBuilder::new(POSTCODE_3_BYTES.len());
        for   p3bytes in POSTCODE_3_BYTES {
            sb.append(&SETS[0].char_at(&::get_int(&bytes, &p3bytes)));
        }
        return sb.to_string();
    }

    fn  get_message( bytes: &Vec<i8>,  start: i32,  len: i32) -> String  {
         let sb: StringBuilder = StringBuilder::new();
         let mut shift: i32 = -1;
         let mut set: i32 = 0;
         let mut lastset: i32 = 0;
         {
             let mut i: i32 = start;
            while i < start + len {
                {
                     let c: char = SETS[set].char_at(bytes[i]);
                    match c {
                          LATCHA => 
                             {
                                set = 0;
                                shift = -1;
                                break;
                            }
                          LATCHB => 
                             {
                                set = 1;
                                shift = -1;
                                break;
                            }
                          SHIFTA => 
                             {
                            }
                          SHIFTB => 
                             {
                            }
                          SHIFTC => 
                             {
                            }
                          SHIFTD => 
                             {
                            }
                          SHIFTE => 
                             {
                                lastset = set;
                                set = c - SHIFTA;
                                shift = 1;
                                break;
                            }
                          TWOSHIFTA => 
                             {
                                lastset = set;
                                set = 0;
                                shift = 2;
                                break;
                            }
                          THREESHIFTA => 
                             {
                                lastset = set;
                                set = 0;
                                shift = 3;
                                break;
                            }
                          NS => 
                             {
                                 let nsval: i32 = (bytes[i += 1] << 24) + (bytes[i += 1] << 18) + (bytes[i += 1] << 12) + (bytes[i += 1] << 6) + bytes[i += 1];
                                sb.append(&DecimalFormat::new("000000000").format(nsval));
                                break;
                            }
                          LOCK => 
                             {
                                shift = -1;
                                break;
                            }
                        _ => 
                             {
                                sb.append(c);
                            }
                    }
                    if shift -= 1  == 0 {
                        set = lastset;
                    }
                }
                i += 1;
             }
         }

        while sb.length() > 0 && sb.char_at(sb.length() - 1) == PAD {
            sb.set_length(sb.length() - 1);
        }
        return sb.to_string();
    }
}

// Decoder.java
/**
 * <p>The main class which implements MaxiCode decoding -- as opposed to locating and extracting
 * the MaxiCode from an image.</p>
 *
 * @author Manuel Kasten
 */

const ALL: i32 = 0;

const EVEN: i32 = 1;

const ODD: i32 = 2;
pub struct Decoder {

     rs_decoder: ReedSolomonDecoder
}

impl Decoder {

   pub fn new() -> Decoder {
       rs_decoder = ReedSolomonDecoder::new(GenericGF::MAXICODE_FIELD_64);
   }

   pub fn  decode_simple(&self,  bits: &BitMatrix) -> Result<DecoderResult, ChecksumException+ FormatException>   {
       return Ok(self.decode(bits, null));
   }

   pub fn  decode(&self,  bits: &BitMatrix,  hints: &Map<DecodeHintType, _>) -> Result<DecoderResult, ChecksumException+ FormatException>   {
        let parser: BitMatrixParser = BitMatrixParser::new(bits);
        let codewords: Vec<i8> = parser.read_codewords();
       self.correct_errors(&codewords, 0, 10, 10, ALL);
        let mode: i32 = codewords[0] & 0x0F;
        let mut datawords: Vec<i8>;
       match mode {
             2 => 
                {
               }
             3 => 
                {
               }
             4 => 
                {
                   self.correct_errors(&codewords, 20, 84, 40, EVEN);
                   self.correct_errors(&codewords, 20, 84, 40, ODD);
                   datawords =  [0; 94];
               }
             5 => 
                {
                   self.correct_errors(&codewords, 20, 68, 56, EVEN);
                   self.correct_errors(&codewords, 20, 68, 56, ODD);
                   datawords = [0; 78];
               }
           _ => 
                {
                   return Err( FormatException::get_format_instance());
               }
       }
       System::arraycopy(&codewords, 0, &datawords, 0, 10);
       System::arraycopy(&codewords, 20, &datawords, 10, datawords.len() - 10);
       return Ok(DecodedBitStreamParser::decode(&datawords, mode));
   }

   fn  correct_errors(&self,  codeword_bytes: &Vec<i8>,  start: i32,  data_codewords: i32,  ec_codewords: i32,  mode: i32)  -> Result<(), ChecksumException>   {
        let codewords: i32 = data_codewords + ec_codewords;
       // in EVEN or ODD mode only half the codewords
        let mut divisor: i32 =  if mode == ALL { 1 } else { 2 };
       // First read into an array of ints
        let codewords_ints: [i32; codewords / divisor] = [0; codewords / divisor];
        {
            let mut i: i32 = 0;
           while i < codewords {
               {
                   if (mode == ALL) || (i % 2 == (mode - 1)) {
                       codewords_ints[i / divisor] = codeword_bytes[i + start] & 0xFF;
                   }
               }
               i += 1;
            }
        }

       
           self.rs_decoder.decode(&codewords_ints, ec_codewords / divisor);
       

       // We don't care about errors in the error-correction codewords
        {
            let mut i: i32 = 0;
           while i < data_codewords {
               {
                   if (mode == ALL) || (i % 2 == (mode - 1)) {
                       codeword_bytes[i + start] = codewords_ints[i / divisor] as i8;
                   }
               }
               i += 1;
            }
        }

        Ok(())

   }
}

