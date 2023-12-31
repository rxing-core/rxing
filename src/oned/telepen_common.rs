pub mod TelepenCommon {
    use crate::Exceptions;
    use crate::common::Result;

    pub fn calculate_checksum(contents: &str) -> char {
        let mut sum = 0;

        for c in contents.chars() {
            sum += c as u32;
        }

        let remainder = sum % 127;
        let diff = 127 - remainder;
        return if diff != 127 {
            diff as u8 as char
        }
        else {
            0 as char
        };
    }

    pub fn ascii_to_numeric(contents: &str) -> String {
        let mut number = String::new();

        for i in 0 .. contents.len() {
            let temp = contents.chars().nth(i).unwrap() as u32;

            if temp >= 27 {
                number.push_str(&(temp - 27).to_string());
            }
            else {
                number.push_str(&(temp - 17).to_string());
            }
        }
        
        return number;
    }

    pub fn numeric_to_ascii(contents: &str) -> Result<String> {
        if contents.len() % 2 != 0 {
            return Err(Exceptions::illegal_argument_with("Input must contain an even number of characters."));
        }

        let mut ascii = String::new();
        let mut i = 0;

        while i < contents.len() {
            let first = contents.chars().nth(i).unwrap() as u8;
            let second = contents.chars().nth(i + 1).unwrap() as u8;

            if second == 88 && first >= 48 && first <= 57 {
                ascii.push((17 + first - 48) as char);
            }
            else if second >= 48 && second <= 57 && first >= 48 && first <= 57 {
                ascii.push((27 + (first - 48) * 10 + (second - 48)) as char);
            }
            else {
                return Err(Exceptions::illegal_argument_with(format!("Input contains an invalid character around position {}.", i.to_string())));
            }

            i += 2;
        }

        return Ok(ascii);
    }
}