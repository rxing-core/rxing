use crate::{Error, common::Result};
/**
 * Computes the UPC/EAN checksum on a string of digits, and reports
 * whether the checksum is correct or not.
 *
 * @param s string of digits to check
 * @return true iff string of digits passes the UPC/EAN checksum algorithm
 * @throws FormatException if the string does not contain only digits
 */
pub fn checkStandardUPCEANChecksum(s: &str) -> Result<bool> {
    let s = s.chars().collect::<Vec<_>>();
    let length = s.len();
    if length == 0 {
        return Ok(false);
    }
    let char_in_question = *s.get(length - 1).ok_or(Error::INDEX_OUT_OF_BOUNDS)?;
    let check = char_in_question.is_ascii_digit();

    let check_against = &s[..length - 1]; //s.subSequence(0, length - 1);
    let calculated_checksum = getStandardUPCEANChecksum(check_against)?;

    Ok(calculated_checksum
        == if check {
            char_in_question.to_digit(10).ok_or(Error::Checksum("checksum could not be verified".into()))?
        } else {
            u32::MAX
        })
}

pub fn getStandardUPCEANChecksum(s: &[char]) -> Result<u32> {
    let length = s.len();
    let mut sum = 0;
    let mut i = length as isize - 1;
    while i >= 0 {
        // for (int i = length - 1; i >= 0; i -= 2) {
        let digit = (*s.get(i as usize).ok_or(Error::INDEX_OUT_OF_BOUNDS)? as i32) - ('0' as i32);
        if !(0..=9).contains(&digit) {
            return Err(Error::Format {
                message: "digit is not between 0 and 9".into(),
                source: None,
            });
        }
        sum += digit;

        i -= 2;
    }
    sum *= 3;
    let mut i = length as isize - 2;
    while i >= 0 {
        // for (int i = length - 2; i >= 0; i -= 2) {
        let digit = (*s.get(i as usize).ok_or(Error::INDEX_OUT_OF_BOUNDS)? as i32) - ('0' as i32);
        if !(0..=9).contains(&digit) {
            return Err(Error::Format {
                message: "digit is not between 0 and 9".into(),
                source: None,
            });
        }
        sum += digit;

        i -= 2;
    }
    Ok(((1000 - sum) % 10) as u32)
}

/**
 * Expands a UPC-E value back into its full, equivalent UPC-A code value.
 *
 * @param upce UPC-E code as string of digits
 * @return equivalent UPC-A code as string of digits
 */
pub fn convertUPCEtoUPCA(upce: &str) -> Option<String> {
    let upce = upce.chars().collect::<Vec<_>>();
    let upceChars = &upce[1..7];

    let mut result = Vec::with_capacity(12);

    result.push(*upce.first()?);
    let lastChar = *upceChars.get(5)?;
    match lastChar {
        '0' | '1' | '2' => {
            result.extend_from_slice(&upceChars[0..2]);
            // result.push(upceChars, 0, 2);
            result.push(lastChar);
            result.extend("0000".chars());
            result.extend_from_slice(&upceChars[2..3 + 2]);
        }
        '3' => {
            result.extend_from_slice(&upceChars[0..3]);
            result.extend("00000".chars());
            result.extend_from_slice(&upceChars[3..2 + 3]);
        }
        '4' => {
            result.extend_from_slice(&upceChars[0..4]);
            result.extend("00000".chars());
            result.push(upceChars.get(4).copied()?);
        }
        _ => {
            result.extend_from_slice(&upceChars[0..5]);
            result.extend("0000".chars());
            result.push(lastChar);
        }
    }
    // Only append check digit in conversion if supplied
    if upce.len() >= 8 {
        result.push(*upce.get(7)?);
    }

    Some(String::from_iter(result))
}
