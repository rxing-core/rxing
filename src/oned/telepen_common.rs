use crate::common::Result;
use crate::Exceptions;

pub fn calculate_checksum(contents: &str) -> char {
    let sum: u32 = contents.chars().map(|c| c as u32).sum();

    let remainder = sum % 127;
    let diff = 127 - remainder;
    if diff != 127 {
        diff as u8 as char
    } else {
        0 as char
    }
}

pub fn ascii_to_numeric(contents: &str) -> String {
    let mut number = String::with_capacity(contents.chars().count() * 2);

    for c in contents.chars().map(|x| x as u32) {
        if c >= 27 {
            number.push_str(&format!("{:0>2}", (c - 27)));
        } else {
            number.push_str(&format!("{:0>2}", (c - 17)));
        }
    }

    number
}

pub fn numeric_to_ascii(contents: &str) -> Result<String> {
    if contents.len() % 2 != 0 {
        return Err(Exceptions::illegal_argument_with(
            "Input must contain an even number of characters.",
        ));
    }

    let mut ascii = Vec::with_capacity(contents.chars().count() / 2);
    let mut i = 0;

    let cached_contents = contents.chars().map(|c| c as u8).collect::<Vec<_>>();

    while i < cached_contents.len() {
        let first = *cached_contents.get(i).unwrap();
        let second = *cached_contents.get(i + 1).unwrap();

        if second == 88 && (48..=57).contains(&first) {
            ascii.push((17 + first - 48) as char);
        } else if (48..=57).contains(&second) && (48..=57).contains(&first) {
            ascii.push((27 + (first - 48) * 10 + (second - 48)) as char);
        } else {
            return Err(Exceptions::illegal_argument_with(format!(
                "Input contains an invalid character around position {i}."
            )));
        }

        i += 2;
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
fn telepen_alpha_to_numeric_test() {
    let mut ascii = "'=Siu";
    let mut result = ascii_to_numeric(ascii);
    assert_eq!("1234567890", result);

    ascii = "& oe";
    result = ascii_to_numeric(ascii);
    assert_eq!("11058474", result);
}

#[test]
fn telepen_numeric_to_ascii_test() {
    let mut numeric = "1234567890";
    let mut result = numeric_to_ascii(numeric).unwrap();
    assert_eq!("'=Siu", result);

    numeric = "11058474";
    result = numeric_to_ascii(numeric).unwrap();
    assert_eq!("& oe", result);
}
