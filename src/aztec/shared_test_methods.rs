use regex::Regex;

use crate::common::BitArray;

use once_cell::sync::Lazy;

static SPACES: Lazy<Regex> = Lazy::new(|| Regex::new("\\s+").unwrap());
static DOTX: Lazy<Regex> = Lazy::new(|| Regex::new("[^.X]").unwrap());

#[allow(dead_code)]
pub fn toBitArray(bits: &str) -> BitArray {
    let mut ba_in = BitArray::new();
    let str = DOTX.replace_all(bits, "");
    for a_str in str.chars() {
        // for (char aStr : str) {
        ba_in.appendBit(a_str == 'X');
    }

    ba_in
}

#[allow(dead_code)]
pub fn toBooleanArray(bitArray: &BitArray) -> Vec<bool> {
    let mut result = vec![false; bitArray.get_size()];
    // for i in 0..result.len() {
    for (i, res) in result.iter_mut().enumerate() {
        // for (int i = 0; i < result.length; i++) {
        *res = bitArray.get(i);
    }
    result
}

#[allow(dead_code)]
pub fn stripSpace(s: &str) -> String {
    SPACES.replace_all(s, "").to_string()
}
