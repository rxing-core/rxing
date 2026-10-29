use crate::Exceptions;
use crate::common::Result;

pub fn calculate_checksum(contents: &str) -> char {
    // Reduce as we go: the plain sum overflows u32 for long or non-ASCII input.
    let remainder = contents.chars().fold(0u32, |sum, c| (sum + c as u32) % 127);
    let diff = 127 - remainder;
    if diff != 127 {
        diff as u8 as char
    } else {
        0 as char
    }
}

pub fn ascii_to_numeric(contents: &str) -> Result<String> {
    let mut number = String::with_capacity(contents.chars().count() * 2);

    for c in contents.chars().map(|x| x as u32) {
        match c {
            // ESC to '~' are the digit pairs 00 to 99.
            27..=126 => number.push_str(&format!("{:0>2}", (c - 27))),
            // DC1 to SUB are 0X to 9X.
            17..=26 => number.push_str(&format!("{}X", (c - 17))),
            // DLE switches the rest of the block to ASCII.
            16 => {
                return Err(Exceptions::format_with(
                    "Telepen numeric mode: DLE (switch to ASCII) is not supported",
                ));
            }
            // The other control characters stay control characters (SB,
            // "Barcode Symbology information and History").
            _ => number.push(c as u8 as char),
        }
    }

    Ok(number)
}

pub fn numeric_to_ascii(contents: &str) -> Result<String> {
    if contents.len() % 2 != 0 {
        return Err(Exceptions::illegal_argument_with(
            "Input must contain an even number of characters.",
        ));
    }

    let mut ascii = Vec::with_capacity(contents.len() / 2);

    // Bytes, not chars: a non-ASCII char cast to u8 can pass for a digit.
    let cached_contents = contents.as_bytes();

    for (i, chunk) in cached_contents.chunks_exact(2).enumerate() {
        let first = chunk[0];
        let second = chunk[1];

        if second == 88 && (48..=57).contains(&first) {
            ascii.push((17 + first - 48) as char);
        } else if (48..=57).contains(&second) && (48..=57).contains(&first) {
            ascii.push((27 + (first - 48) * 10 + (second - 48)) as char);
        } else {
            return Err(Exceptions::illegal_argument_with(format!(
                "Input contains an invalid character around position {}.",
                i * 2
            )));
        }
    }

    Ok(ascii.iter().collect())
}

#[test]
fn telepen_checksum_test1() {
    let contents = "Hello world!";
    let checksum = calculate_checksum(contents);
    assert_eq!('\u{1a}', checksum);
}

#[test]
fn telepen_checksum_test2() {
    let contents = "ABC123456";
    let checksum = calculate_checksum(contents);
    assert_eq!('\u{1}', checksum);
}

#[test]
fn telepen_checksum_long_input_test() {
    // The plain sum of these chars is 4,296,012,016, above u32::MAX.
    let contents = "\u{10FFFF}".repeat(3856);
    let expected = 127 - (3856 * 0x10FFFF_u64 % 127) as u8;
    assert_eq!(expected as char, calculate_checksum(&contents));
}

#[test]
fn telepen_alpha_to_numeric_test() {
    let mut ascii = "'=Siu";
    let mut result = ascii_to_numeric(ascii).unwrap();
    assert_eq!("1234567890", result);

    ascii = "& oe";
    result = ascii_to_numeric(ascii).unwrap();
    assert_eq!("11058474", result);

    // 17 to 26 are 0X to 9X (SB Telepen Barcode Fonts, E2326U, Table 1).
    ascii = "I\u{17}<";
    result = ascii_to_numeric(ascii).unwrap();
    assert_eq!("466X33", result);

    ascii = "\u{11}\u{1a}";
    result = ascii_to_numeric(ascii).unwrap();
    assert_eq!("0X9X", result);

    // The other control characters stay as they are.
    result = ascii_to_numeric("\0A\u{5}B\u{f}\u{7f}").unwrap();
    assert_eq!("\u{0}38\u{5}39\u{f}\u{7f}", result);

    // DLE, the switch to ASCII, is not implemented.
    for ascii in ["\u{10}", "12\u{10}"] {
        assert!(matches!(
            ascii_to_numeric(ascii),
            Err(Exceptions::FormatException(_))
        ));
    }
}

#[test]
fn telepen_numeric_to_ascii_test() {
    let mut numeric = "1234567890";
    let mut result = numeric_to_ascii(numeric).unwrap();
    assert_eq!("'=Siu", result);

    numeric = "11058474";
    result = numeric_to_ascii(numeric).unwrap();
    assert_eq!("& oe", result);

    numeric = "466X33";
    result = numeric_to_ascii(numeric).unwrap();
    assert_eq!("I\u{17}<", result);

    // Not digits, although their low bytes are.
    for numeric in ["1\u{135}2\u{135}", "\u{131}\u{132}"] {
        assert!(numeric_to_ascii(numeric).is_err());
    }

    // Not dropped, although it makes the byte length even.
    assert!(numeric_to_ascii("12\u{e9}").is_err());
}
