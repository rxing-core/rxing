/*
 * Copyright 2017 Huy Cuong Nguyen
 * Copyright 2008 ZXing authors
*/
// SPDX-License-Identifier: Apache-2.0

use crate::{qrcode::cpp_port::decoder::Decode, common::BitMatrix, Exceptions};

#[test]
fn MQRCodeM3L()
{
	const CODE_STR : &str = 
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
";
	let bitMatrix = BitMatrix::parse_strings(CODE_STR,
										  "X", " ").unwrap();

	let result = Decode(&bitMatrix).unwrap();
	assert!(result.isValid());
}

#[test]
fn  MQRCodeM3M()
{
	const CODE_STR : &str = 
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
";
	let bitMatrix = BitMatrix::parse_strings(CODE_STR,
		"X", " ").unwrap();

	let result = Decode(&bitMatrix).unwrap();
	assert!(result.isValid());
}

#[test]
fn  MQRCodeM1()
{
	const CODE_STR : &str = 
r"XXXXXXX X X
X     X    
X XXX X XXX
X XXX X  XX
X XXX X   X
X     X XX 
XXXXXXX X  
        X  
XX     X   
 X  XXXXX X
X  XXXXXX X
";
	let bitMatrix = BitMatrix::parse_strings(CODE_STR,
		"X", " ").unwrap();
	let result = Decode(&bitMatrix).unwrap();
	assert!(result.isValid());
	assert_eq!("123", result.text());
}

#[test]
fn  MQRCodeM1Error4Bits()
{
	const CODE_STR : &str = 
r"XXXXXXX X X
X     X  XX
X XXX X X  
X XXX X  XX
X XXX X   X
X     X XX 
XXXXXXX X  
        X  
XX     X   
 X  XXXXXX 
X  XXXXXXX 
";
	let bitMatrix = BitMatrix::parse_strings(CODE_STR,
		"X", " ").unwrap();
	let result = Decode(&bitMatrix);
	dbg!(&result);
	assert!(matches!(result.err(), Some(Exceptions::ReedSolomonException(_))));
	// assert_eq!(Error::Checksum, result.error());
	// assert!(result.text().is_empty());
}

#[test]
fn  MQRCodeM4()
{
	const CODE_STR : &str = 
r"XXXXXXX X X X X X
X     X XX X   XX
X XXX X  X  X  XX
X XXX X XX  XX XX
X XXX X  X  XXXXX
X     X XX      X
XXXXXXX XX  X  XX
         X  XX XX
X  X XXX    X XXX
 XX  X  XX XX  X 
XX  XXXX X XX  XX
    XX XX X XX XX
XXX XXX XXX XX XX
  X X   X   XX  X
X X XX   XXXXX   
  X X X X   X    
X   XXXXXXX X X X
";
	let bitMatrix = BitMatrix::parse_strings(CODE_STR,
		"X", " ").unwrap();
	let result = Decode(&bitMatrix).unwrap();
	assert!(result.isValid());
}
