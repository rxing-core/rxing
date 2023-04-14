/*
 * Copyright 2017 Huy Cuong Nguyen
 * Copyright 2008 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::{qrcode::cpp_port::{ bitmatrix_parser::{ReadFormatInformation, ReadCodewords, ReadVersion}}, common::BitMatrix};

#[test]
fn MQRCodeM3L()
{
	let bitMatrix = 
	BitMatrix::parse_strings(
r"XXXXXXX X X X X
X     X    X X 
X XXX X XXXXXXX
X XXX X X X  XX
X XXX X    X XX
X     X X X X X
XXXXXXX  X  XX 
         X X  X
XXXXXX    X X X
   X  XX    XXX
XXX XX XXXX XXX
 X    X  XXX X 
X XXXXX XXX X X
 X    X  X XXX 
XXX XX X X XXXX
", "X", " ").expect("parse must parse");

	let version = ReadVersion(&bitMatrix).unwrap();
	assert_eq!(3, version.getVersionNumber());
	let format = ReadFormatInformation(&bitMatrix, true).expect("could not read format information");
	let codewords = ReadCodewords(&bitMatrix, version, &format).expect("could not read codewords");
	assert_eq!(17, codewords.len());
	assert_eq!(0x0, codewords[10]);
	assert_eq!(0xd1, codewords[11]);
}

#[test]
fn  MQRCodeM3M()
{
	let bitMatrix = BitMatrix::parse_strings(
r"XXXXXXX X X X X
X     X      XX
X XXX X X XX XX
X XXX X X X    
X XXX X XX XXXX
X     X XX     
XXXXXXX  X XXXX
        X  XXX 
X    XX XX X  X
   X X     XX  
XX  XX  XXXXXXX
 X    X       X
XX X X      X  
   X X    X    
X X XXXX    XXX
"
,"X", " ").unwrap();

	let version = ReadVersion(&bitMatrix).unwrap();
	assert_eq!(3, version.getVersionNumber());
	let format = ReadFormatInformation(&bitMatrix, true).expect("could not read format information");
	let codewords = ReadCodewords(&bitMatrix, version, &format).expect("could not read codewords");
	assert_eq!(17, codewords.len());
	assert_eq!(0x0, codewords[8]);
	assert_eq!(0x89, codewords[9]);
}
