pub struct TelepenCommon;

impl TelepenCommon {
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
}