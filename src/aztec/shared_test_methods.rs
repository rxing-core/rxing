use regex::Regex;

use crate::common::BitArray;

use lazy_static::lazy_static;

lazy_static! {
    static ref SPACES: Regex = Regex::new("\\s+").unwrap();
    static ref DOTX: Regex = Regex::new("[^.X]").unwrap();
}

pub fn toBitArray(bits: &str) -> BitArray {
    let mut ba_in = BitArray::new();
    let str = DOTX.replace_all(bits, "");
    for a_str in str.chars() {
        // for (char aStr : str) {
        ba_in.appendBit(a_str == 'X');
    }

    ba_in
}

pub fn toBooleanArray(bitArray: &BitArray) -> Vec<bool> {
    let mut result = vec![false; bitArray.getSize()];
    for i in 0..result.len() {
        // for (int i = 0; i < result.length; i++) {
        result[i] = bitArray.get(i);
    }
    result
}

pub fn stripSpace(s: &str) -> String {
    SPACES.replace_all(s, "").to_string()
}
