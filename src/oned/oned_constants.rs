// Various constants required for oned barcodes

pub mod coda_bar {
    // These values are critical for determining how permissive the decoding
    // will be. All stripe sizes must be within the window these define, as
    // compared to the average stripe size.
    pub const MAX_ACCEPTABLE: f32 = 2.0;
    pub const PADDING: f32 = 1.5;

    // const ALPHABET_STRING : &str= "0123456789-$:/.+ABCD";
    pub const ALPHABET: [char; 20] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '$', ':', '/', '.', '+', 'A', 'B',
        'C', 'D',
    ];

    /**
     * These represent the encodings of characters, as patterns of wide and narrow bars. The 7 least-significant bits of
     * each int correspond to the pattern of wide and narrow, with 1s representing "wide" and 0s representing narrow.
     */
    pub const CHARACTER_ENCODINGS: [u32; 20] = [
        0x003, 0x006, 0x009, 0x060, 0x012, 0x042, 0x021, 0x024, 0x030, 0x048, // 0-9
        0x00c, 0x018, 0x045, 0x051, 0x054, 0x015, 0x01A, 0x029, 0x00B, 0x00E, // -$:/.+ABCD
    ];

    // minimal number of characters that should be present (including start and stop characters)
    // under normal circumstances this should be set to 3, but can be set higher
    // as a last-ditch attempt to reduce false positives.
    pub const MIN_CHARACTER_LENGTH: u32 = 3;

    // official start and end patterns
    pub const STARTEND_ENCODING: [char; 4] = ['A', 'B', 'C', 'D'];
}

pub mod code_39 {
    pub const ALPHABET_STRING: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%";

    /**
     * These represent the encodings of characters, as patterns of wide and narrow bars.
     * The 9 least-significant bits of each int correspond to the pattern of wide and narrow,
     * with 1s representing "wide" and 0s representing narrow.
     */
    pub const CHARACTER_ENCODINGS: [u32; 43] = [
        0x034, 0x121, 0x061, 0x160, 0x031, 0x130, 0x070, 0x025, 0x124, 0x064, // 0-9
        0x109, 0x049, 0x148, 0x019, 0x118, 0x058, 0x00D, 0x10C, 0x04C, 0x01C, // A-J
        0x103, 0x043, 0x142, 0x013, 0x112, 0x052, 0x007, 0x106, 0x046, 0x016, // K-T
        0x181, 0x0C1, 0x1C0, 0x091, 0x190, 0x0D0, 0x085, 0x184, 0x0C4, 0x0A8, // U-$
        0x0A2, 0x08A, 0x02A, // /-%
    ];

    pub const ASTERISK_ENCODING: u32 = 0x094;
}

pub mod code_93 {
    // Note that 'abcd' are dummy characters in place of control characters.
    pub const ALPHABET_STRING: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%abcd*";
    pub const ALPHABET: [char; 48] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '-', '.', ' ', '$', '/', '+', '%', 'a', 'b', 'c', 'd', '*',
    ];

    /**
     * These represent the encodings of characters, as patterns of wide and narrow bars.
     * The 9 least-significant bits of each int correspond to the pattern of wide and narrow.
     */
    pub const CHARACTER_ENCODINGS: [u32; 48] = [
        0x114, 0x148, 0x144, 0x142, 0x128, 0x124, 0x122, 0x150, 0x112, 0x10A, // 0-9
        0x1A8, 0x1A4, 0x1A2, 0x194, 0x192, 0x18A, 0x168, 0x164, 0x162, 0x134, // A-J
        0x11A, 0x158, 0x14C, 0x146, 0x12C, 0x116, 0x1B4, 0x1B2, 0x1AC, 0x1A6, // K-T
        0x196, 0x19A, 0x16C, 0x166, 0x136, 0x13A, // U-Z
        0x12E, 0x1D4, 0x1D2, 0x1CA, 0x16E, 0x176, 0x1AE, // - - %
        0x126, 0x1DA, 0x1D6, 0x132, 0x15E, // Control chars? $-*
    ];
    pub const ASTERISK_ENCODING: i32 = CHARACTER_ENCODINGS[47] as i32;
}

pub mod code_128 {
    pub const CODE_PATTERNS: [[u32; 7]; 107] = [
        [2, 1, 2, 2, 2, 2, 0], // 0
        [2, 2, 2, 1, 2, 2, 0],
        [2, 2, 2, 2, 2, 1, 0],
        [1, 2, 1, 2, 2, 3, 0],
        [1, 2, 1, 3, 2, 2, 0],
        [1, 3, 1, 2, 2, 2, 0], // 5
        [1, 2, 2, 2, 1, 3, 0],
        [1, 2, 2, 3, 1, 2, 0],
        [1, 3, 2, 2, 1, 2, 0],
        [2, 2, 1, 2, 1, 3, 0],
        [2, 2, 1, 3, 1, 2, 0], // 10
        [2, 3, 1, 2, 1, 2, 0],
        [1, 1, 2, 2, 3, 2, 0],
        [1, 2, 2, 1, 3, 2, 0],
        [1, 2, 2, 2, 3, 1, 0],
        [1, 1, 3, 2, 2, 2, 0], // 15
        [1, 2, 3, 1, 2, 2, 0],
        [1, 2, 3, 2, 2, 1, 0],
        [2, 2, 3, 2, 1, 1, 0],
        [2, 2, 1, 1, 3, 2, 0],
        [2, 2, 1, 2, 3, 1, 0], // 20
        [2, 1, 3, 2, 1, 2, 0],
        [2, 2, 3, 1, 1, 2, 0],
        [3, 1, 2, 1, 3, 1, 0],
        [3, 1, 1, 2, 2, 2, 0],
        [3, 2, 1, 1, 2, 2, 0], // 25
        [3, 2, 1, 2, 2, 1, 0],
        [3, 1, 2, 2, 1, 2, 0],
        [3, 2, 2, 1, 1, 2, 0],
        [3, 2, 2, 2, 1, 1, 0],
        [2, 1, 2, 1, 2, 3, 0], // 30
        [2, 1, 2, 3, 2, 1, 0],
        [2, 3, 2, 1, 2, 1, 0],
        [1, 1, 1, 3, 2, 3, 0],
        [1, 3, 1, 1, 2, 3, 0],
        [1, 3, 1, 3, 2, 1, 0], // 35
        [1, 1, 2, 3, 1, 3, 0],
        [1, 3, 2, 1, 1, 3, 0],
        [1, 3, 2, 3, 1, 1, 0],
        [2, 1, 1, 3, 1, 3, 0],
        [2, 3, 1, 1, 1, 3, 0], // 40
        [2, 3, 1, 3, 1, 1, 0],
        [1, 1, 2, 1, 3, 3, 0],
        [1, 1, 2, 3, 3, 1, 0],
        [1, 3, 2, 1, 3, 1, 0],
        [1, 1, 3, 1, 2, 3, 0], // 45
        [1, 1, 3, 3, 2, 1, 0],
        [1, 3, 3, 1, 2, 1, 0],
        [3, 1, 3, 1, 2, 1, 0],
        [2, 1, 1, 3, 3, 1, 0],
        [2, 3, 1, 1, 3, 1, 0], // 50
        [2, 1, 3, 1, 1, 3, 0],
        [2, 1, 3, 3, 1, 1, 0],
        [2, 1, 3, 1, 3, 1, 0],
        [3, 1, 1, 1, 2, 3, 0],
        [3, 1, 1, 3, 2, 1, 0], // 55
        [3, 3, 1, 1, 2, 1, 0],
        [3, 1, 2, 1, 1, 3, 0],
        [3, 1, 2, 3, 1, 1, 0],
        [3, 3, 2, 1, 1, 1, 0],
        [3, 1, 4, 1, 1, 1, 0], // 60
        [2, 2, 1, 4, 1, 1, 0],
        [4, 3, 1, 1, 1, 1, 0],
        [1, 1, 1, 2, 2, 4, 0],
        [1, 1, 1, 4, 2, 2, 0],
        [1, 2, 1, 1, 2, 4, 0], // 65
        [1, 2, 1, 4, 2, 1, 0],
        [1, 4, 1, 1, 2, 2, 0],
        [1, 4, 1, 2, 2, 1, 0],
        [1, 1, 2, 2, 1, 4, 0],
        [1, 1, 2, 4, 1, 2, 0], // 70
        [1, 2, 2, 1, 1, 4, 0],
        [1, 2, 2, 4, 1, 1, 0],
        [1, 4, 2, 1, 1, 2, 0],
        [1, 4, 2, 2, 1, 1, 0],
        [2, 4, 1, 2, 1, 1, 0], // 75
        [2, 2, 1, 1, 1, 4, 0],
        [4, 1, 3, 1, 1, 1, 0],
        [2, 4, 1, 1, 1, 2, 0],
        [1, 3, 4, 1, 1, 1, 0],
        [1, 1, 1, 2, 4, 2, 0], // 80
        [1, 2, 1, 1, 4, 2, 0],
        [1, 2, 1, 2, 4, 1, 0],
        [1, 1, 4, 2, 1, 2, 0],
        [1, 2, 4, 1, 1, 2, 0],
        [1, 2, 4, 2, 1, 1, 0], // 85
        [4, 1, 1, 2, 1, 2, 0],
        [4, 2, 1, 1, 1, 2, 0],
        [4, 2, 1, 2, 1, 1, 0],
        [2, 1, 2, 1, 4, 1, 0],
        [2, 1, 4, 1, 2, 1, 0], // 90
        [4, 1, 2, 1, 2, 1, 0],
        [1, 1, 1, 1, 4, 3, 0],
        [1, 1, 1, 3, 4, 1, 0],
        [1, 3, 1, 1, 4, 1, 0],
        [1, 1, 4, 1, 1, 3, 0], // 95
        [1, 1, 4, 3, 1, 1, 0],
        [4, 1, 1, 1, 1, 3, 0],
        [4, 1, 1, 3, 1, 1, 0],
        [1, 1, 3, 1, 4, 1, 0],
        [1, 1, 4, 1, 3, 1, 0], // 100
        [3, 1, 1, 1, 4, 1, 0],
        [4, 1, 1, 1, 3, 1, 0],
        [2, 1, 1, 4, 1, 2, 0],
        [2, 1, 1, 2, 1, 4, 0],
        [2, 1, 1, 2, 3, 2, 0], // 105
        [2, 3, 3, 1, 1, 1, 2],
    ];
}

pub mod ean_8 {}

pub mod ean_13 {
    // For an EAN-13 barcode, the first digit is represented by the parities used
    // to encode the next six digits, according to the table below. For example,
    // if the barcode is 5 123456 789012 then the value of the first digit is
    // signified by using odd for '1', even for '2', even for '3', odd for '4',
    // odd for '5', and even for '6'. See http://en.wikipedia.org/wiki/EAN-13
    //
    //                Parity of next 6 digits
    //    Digit   0     1     2     3     4     5
    //       0    Odd   Odd   Odd   Odd   Odd   Odd
    //       1    Odd   Odd   Even  Odd   Even  Even
    //       2    Odd   Odd   Even  Even  Odd   Even
    //       3    Odd   Odd   Even  Even  Even  Odd
    //       4    Odd   Even  Odd   Odd   Even  Even
    //       5    Odd   Even  Even  Odd   Odd   Even
    //       6    Odd   Even  Even  Even  Odd   Odd
    //       7    Odd   Even  Odd   Even  Odd   Even
    //       8    Odd   Even  Odd   Even  Even  Odd
    //       9    Odd   Even  Even  Odd   Even  Odd
    //
    // Note that the encoding for '0' uses the same parity as a UPC barcode. Hence
    // a UPC barcode can be converted to an EAN-13 barcode by prepending a 0.
    //
    // The encoding is represented by the following array, which is a bit pattern
    // using Odd = 0 and Even = 1. For example, 5 is represented by:
    //
    //              Odd Even Even Odd Odd Even
    // in binary:
    //                0    1    1   0   0    1   == 0x19
    //
    pub const FIRST_DIGIT_ENCODINGS: [usize; 10] =
        [0x00, 0x0B, 0x0D, 0xE, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A];
}

pub mod upc_ean_shared {
    /**
     * Start/end guard pattern.
     */
    pub const START_END_PATTERN: [u32; 3] = [1, 1, 1];

    /**
     * Pattern marking the middle of a UPC/EAN pattern, separating the two halves.
     */
    pub const MIDDLE_PATTERN: [u32; 5] = [1, 1, 1, 1, 1];
    /**
     * end guard pattern.
     */
    pub const END_PATTERN: [u32; 6] = [1, 1, 1, 1, 1, 1];
    /**
     * "Odd", or "L" patterns used to encode UPC/EAN digits.
     */
    pub const L_PATTERNS: [[u32; 4]; 10] = [
        [3, 2, 1, 1], // 0
        [2, 2, 2, 1], // 1
        [2, 1, 2, 2], // 2
        [1, 4, 1, 1], // 3
        [1, 1, 3, 2], // 4
        [1, 2, 3, 1], // 5
        [1, 1, 1, 4], // 6
        [1, 3, 1, 2], // 7
        [1, 2, 1, 3], // 8
        [3, 1, 1, 2], // 9
    ];

    /**
     * As above but also including the "even", or "G" patterns used to encode UPC/EAN digits.
     */
    pub const L_AND_G_PATTERNS: [[u32; 4]; 20] = {
        let mut new_array = [[0_u32; 4]; 20];
        let mut i = 0;
        while i < 10 {
            new_array[i] = L_PATTERNS[i];
            i += 1;
        }
        let mut i = 10;
        while i < 20 {
            let widths = &L_PATTERNS[i - 10];
            let mut reversedWidths = [0_u32; 4];
            let mut j = 0;
            while j < 4 {
                reversedWidths[j] = widths[4 - j - 1];

                j += 1;
            }
            new_array[i] = reversedWidths;

            i += 1;
        }

        new_array
    };
}

pub mod upc_e {
    /**
     * The pattern that marks the middle, and end, of a UPC-E pattern.
     * There is no "second half" to a UPC-E barcode.
     */
    pub const MIDDLE_END_PATTERN: [u32; 6] = [1, 1, 1, 1, 1, 1];

    // For an UPC-E barcode, the final digit is represented by the parities used
    // to encode the middle six digits, according to the table below.
    //
    //                Parity of next 6 digits
    //    Digit   0     1     2     3     4     5
    //       0    Even   Even  Even Odd  Odd   Odd
    //       1    Even   Even  Odd  Even Odd   Odd
    //       2    Even   Even  Odd  Odd  Even  Odd
    //       3    Even   Even  Odd  Odd  Odd   Even
    //       4    Even   Odd   Even Even Odd   Odd
    //       5    Even   Odd   Odd  Even Even  Odd
    //       6    Even   Odd   Odd  Odd  Even  Even
    //       7    Even   Odd   Even Odd  Even  Odd
    //       8    Even   Odd   Even Odd  Odd   Even
    //       9    Even   Odd   Odd  Even Odd   Even
    //
    // The encoding is represented by the following array, which is a bit pattern
    // using Odd = 0 and Even = 1. For example, 5 is represented by:
    //
    //              Odd Even Even Odd Odd Even
    // in binary:
    //                0    1    1   0   0    1   == 0x19
    //

    /**
     * See {@link #L_AND_G_PATTERNS}; these values similarly represent patterns of
     * even-odd parity encodings of digits that imply both the number system (0 or 1)
     * used, and the check digit.
     */
    pub const NUMSYS_AND_CHECK_DIGIT_PATTERNS: [[usize; 10]; 2] = [
        [0x38, 0x34, 0x32, 0x31, 0x2C, 0x26, 0x23, 0x2A, 0x29, 0x25],
        [0x07, 0x0B, 0x0D, 0x0E, 0x13, 0x19, 0x1C, 0x15, 0x16, 0x1A],
    ];
}
