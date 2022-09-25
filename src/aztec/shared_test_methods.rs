use regex::Regex;

use crate::common::BitArray;

const SPACES :&str= "\\s+";
const DOTX : &str = "[^.X]";

pub fn toBitArray( bits:&str) -> BitArray{
    let mut ba_in =  BitArray::new();
    let replacer_regex = Regex::new(DOTX).unwrap();
    let str = replacer_regex.replace_all(bits, "");
    for a_str in str.chars() {
    // for (char aStr : str) {
        ba_in.appendBit(a_str == 'X');
    }
    
    ba_in
  }

  pub fn toBooleanArray( bitArray:&BitArray) ->Vec<bool>{
    let mut result = vec![false;bitArray.getSize()];
    for i in 0..result.len() {
    // for (int i = 0; i < result.length; i++) {
      result[i] = bitArray.get(i);
    }
     result
  }

  pub fn stripSpace( s:&str) -> String{
    let replacer_regex = Regex::new(SPACES).unwrap();
    replacer_regex.replace_all(s, "").to_string()
  }