/*
 * Copyright 2008 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    common::BitArray,
    qrcode::{
        decoder::{ErrorCorrectionLevel, Version},
        encoder::matrix_util,
    },
};

use super::ByteMatrix;

/**
 * @author satorux@google.com (Satoru Takabayashi) - creator
 * @author mysen@google.com (Chris Mysen) - ported from C++
 */

#[test]
fn testToString() {
    let mut array = ByteMatrix::new(3, 3);
    array.set(0, 0, 0);
    array.set(1, 0, 1);
    array.set(2, 0, 0);
    array.set(0, 1, 1);
    array.set(1, 1, 0);
    array.set(2, 1, 1);
    array.set(0, 2, -1i8 as u8);
    array.set(1, 2, -1i8 as u8);
    array.set(2, 2, -1i8 as u8);
    let expected = " 0 1 0\n 1 0 1\n      \n";
    assert_eq!(expected, array.to_string());
}

#[test]
fn testClearMatrix() {
    let mut matrix = ByteMatrix::new(2, 2);
    matrix_util::clearMatrix(&mut matrix);
    assert_eq!(-1i8 as u8, matrix.get(0, 0));
    assert_eq!(-1i8 as u8, matrix.get(1, 0));
    assert_eq!(-1i8 as u8, matrix.get(0, 1));
    assert_eq!(-1i8 as u8, matrix.get(1, 1));
}

#[test]
fn test_embed_basic_patterns1() {
    // Version 1.
    let mut matrix = ByteMatrix::new(21, 21);
    matrix_util::clearMatrix(&mut matrix);
    matrix_util::embedBasicPatterns(
        Version::getVersionForNumber(1).expect("version ok"),
        &mut matrix,
    )
    .expect("op ok");
    let expected = r" 1 1 1 1 1 1 1 0           0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0           0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0           0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0           0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0           0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0           0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0           0 0 0 0 0 0 0 0
             1                            
             0                            
             1                            
             0                            
             1                            
 0 0 0 0 0 0 0 0 1                        
 1 1 1 1 1 1 1 0                          
 1 0 0 0 0 0 1 0                          
 1 0 1 1 1 0 1 0                          
 1 0 1 1 1 0 1 0                          
 1 0 1 1 1 0 1 0                          
 1 0 0 0 0 0 1 0                          
 1 1 1 1 1 1 1 0                          
";
    assert_eq!(expected, matrix.to_string());
}

#[test]
fn testEmbedBasicPatterns2() {
    // Version 2.  Position adjustment pattern should apppear at right
    // bottom corner.
    let mut matrix = ByteMatrix::new(25, 25);
    matrix_util::clearMatrix(&mut matrix);
    matrix_util::embedBasicPatterns(
        Version::getVersionForNumber(2).expect("version ok"),
        &mut matrix,
    )
    .expect("op ok");
    let expected = r" 1 1 1 1 1 1 1 0                   0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0                   0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0                   0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0                   0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0                   0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0                   0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0                   0 0 0 0 0 0 0 0
             1                                    
             0                                    
             1                                    
             0                                    
             1                                    
             0                                    
             1                                    
             0                                    
             1                   1 1 1 1 1        
 0 0 0 0 0 0 0 0 1               1 0 0 0 1        
 1 1 1 1 1 1 1 0                 1 0 1 0 1        
 1 0 0 0 0 0 1 0                 1 0 0 0 1        
 1 0 1 1 1 0 1 0                 1 1 1 1 1        
 1 0 1 1 1 0 1 0                                  
 1 0 1 1 1 0 1 0                                  
 1 0 0 0 0 0 1 0                                  
 1 1 1 1 1 1 1 0                                  
";
    assert_eq!(expected, matrix.to_string());
}

#[test]
fn testEmbedTypeInfo() {
    // Type info bits = 100000011001110.
    let mut matrix = ByteMatrix::new(21, 21);
    matrix_util::clearMatrix(&mut matrix);
    matrix_util::embedTypeInfo(&ErrorCorrectionLevel::M, 5, &mut matrix).expect("op ok");
    let expected = r"                 0                        
                 1                        
                 1                        
                 1                        
                 0                        
                 0                        
                                          
                 1                        
 1 0 0 0 0 0   0 1         1 1 0 0 1 1 1 0
                                          
                                          
                                          
                                          
                                          
                 0                        
                 0                        
                 0                        
                 0                        
                 0                        
                 0                        
                 1                        
";
    assert_eq!(expected, matrix.to_string());
}

#[test]
fn testEmbedVersionInfo() {
    // Version info bits = 000111 110010 010100
    // Actually, version 7 QR Code has 45x45 matrix but we use 21x21 here
    // since 45x45 matrix is too big to depict.
    let mut matrix = ByteMatrix::new(21, 21);
    matrix_util::clearMatrix(&mut matrix);
    matrix_util::maybeEmbedVersionInfo(
        Version::getVersionForNumber(7).expect("Version"),
        &mut matrix,
    )
    .expect("op ok");
    let expected = r"                     0 0 1                
                     0 1 0                
                     0 1 0                
                     0 1 1                
                     1 1 1                
                     0 0 0                
                                          
                                          
                                          
                                          
 0 0 0 0 1 0                              
 0 1 1 1 1 0                              
 1 0 0 1 1 0                              
                                          
                                          
                                          
                                          
                                          
                                          
                                          
                                          
";
    assert_eq!(expected, matrix.to_string());
}

#[test]
fn testEmbedDataBits() {
    // Cells other than basic patterns should be filled with zero.
    let mut matrix = ByteMatrix::new(21, 21);
    matrix_util::clearMatrix(&mut matrix);
    matrix_util::embedBasicPatterns(
        Version::getVersionForNumber(1).expect("version"),
        &mut matrix,
    )
    .expect("op ok");
    let bits = BitArray::new();
    matrix_util::embedDataBits(&bits, -1, &mut matrix).expect("append");
    let expected = r" 1 1 1 1 1 1 1 0 0 0 0 0 0 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 0 0 0 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 0 0 0 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 0 0 0 0 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0
 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 1 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 1 0 1 1 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 1 0 1 1 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 1 0 1 1 1 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 1 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0
";
    assert_eq!(expected, matrix.to_string());
}

#[test]
fn testBuildMatrix() {
    // From http://www.swetake.com/qr/qr7.html
    let bytes = [
        32, 65, 205, 69, 41, 220, 46, 128, 236, 42, 159, 74, 221, 244, 169, 239, 150, 138, 70, 237,
        85, 224, 96, 74, 219, 61,
    ];
    let mut bits = BitArray::new();
    for c in bytes {
        // for (char c: bytes) {
        bits.appendBits(c, 8);
    }
    let mut matrix = ByteMatrix::new(21, 21);
    matrix_util::buildMatrix(
        &bits,
        &ErrorCorrectionLevel::H,
        Version::getVersionForNumber(1).expect("version"), // Version 1
        3,                                                 // Mask pattern 3
        &mut matrix,
    ).expect("append");
    let expected = r" 1 1 1 1 1 1 1 0 0 1 1 0 0 0 1 1 1 1 1 1 1
 1 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 1
 1 0 1 1 1 0 1 0 0 0 0 1 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 0 1 1 0 0 0 1 0 1 1 1 0 1
 1 0 1 1 1 0 1 0 1 1 0 0 1 0 1 0 1 1 1 0 1
 1 0 0 0 0 0 1 0 0 0 1 1 1 0 1 0 0 0 0 0 1
 1 1 1 1 1 1 1 0 1 0 1 0 1 0 1 1 1 1 1 1 1
 0 0 0 0 0 0 0 0 1 1 0 1 1 0 0 0 0 0 0 0 0
 0 0 1 1 0 0 1 1 1 0 0 1 1 1 1 0 1 0 0 0 0
 1 0 1 0 1 0 0 0 0 0 1 1 1 0 0 1 0 1 1 1 0
 1 1 1 1 0 1 1 0 1 0 1 1 1 0 0 1 1 1 0 1 0
 1 0 1 0 1 1 0 1 1 1 0 0 1 1 1 0 0 1 0 1 0
 0 0 1 0 0 1 1 1 0 0 0 0 0 0 1 0 1 1 1 1 1
 0 0 0 0 0 0 0 0 1 1 0 1 0 0 0 0 0 1 0 1 1
 1 1 1 1 1 1 1 0 1 1 1 1 0 0 0 0 1 0 1 1 0
 1 0 0 0 0 0 1 0 0 0 0 1 0 1 1 1 0 0 0 0 0
 1 0 1 1 1 0 1 0 0 1 0 0 1 1 0 0 1 0 0 1 1
 1 0 1 1 1 0 1 0 1 1 0 1 0 0 0 0 0 1 1 1 0
 1 0 1 1 1 0 1 0 1 1 1 1 0 0 0 0 1 1 1 0 0
 1 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 1 0 1 0 0
 1 1 1 1 1 1 1 0 0 0 1 1 1 1 1 0 1 0 0 1 0
";
    assert_eq!(expected, matrix.to_string());
}

#[test]
fn testFindMSBSet() {
    assert_eq!(0, matrix_util::findMSBSet(0));
    assert_eq!(1, matrix_util::findMSBSet(1));
    assert_eq!(8, matrix_util::findMSBSet(0x80));
    assert_eq!(32, matrix_util::findMSBSet(0x80000000));
}

#[test]
fn testCalculateBCHCode() {
    // Encoding of type information.
    // From Appendix C in JISX0510:2004 (p 65)
    assert_eq!(0xdc, matrix_util::calculateBCHCode(5, 0x537).expect("ok"));
    // From http://www.swetake.com/qr/qr6.html
    assert_eq!(
        0x1c2,
        matrix_util::calculateBCHCode(0x13, 0x537).expect("ok")
    );
    // From http://www.swetake.com/qr/qr11.html
    assert_eq!(
        0x214,
        matrix_util::calculateBCHCode(0x1b, 0x537).expect("ok")
    );

    // Encoding of version information.
    // From Appendix D in JISX0510:2004 (p 68)
    assert_eq!(0xc94, matrix_util::calculateBCHCode(7, 0x1f25).expect("ok"));
    assert_eq!(0x5bc, matrix_util::calculateBCHCode(8, 0x1f25).expect("ok"));
    assert_eq!(0xa99, matrix_util::calculateBCHCode(9, 0x1f25).expect("ok"));
    assert_eq!(
        0x4d3,
        matrix_util::calculateBCHCode(10, 0x1f25).expect("ok")
    );
    assert_eq!(
        0x9a6,
        matrix_util::calculateBCHCode(20, 0x1f25).expect("ok")
    );
    assert_eq!(
        0xd75,
        matrix_util::calculateBCHCode(30, 0x1f25).expect("ok")
    );
    assert_eq!(
        0xc69,
        matrix_util::calculateBCHCode(40, 0x1f25).expect("ok")
    );
}

// We don't test a lot of cases in this function since we've already
// tested them in TEST(calculateBCHCode).
#[test]
fn testMakeVersionInfoBits() {
    // From Appendix D in JISX0510:2004 (p 68)
    let mut bits = BitArray::new();
    matrix_util::makeVersionInfoBits(Version::getVersionForNumber(7).expect("version"), &mut bits)
        .expect("op ok");
    assert_eq!(" ...XXXXX ..X..X.X ..", bits.to_string());
}

// We don't test a lot of cases in this function since we've already
// tested them in TEST(calculateBCHCode).
#[test]
fn testMakeTypeInfoInfoBits() {
    // From Appendix C in JISX0510:2004 (p 65)
    let mut bits = BitArray::new();
    matrix_util::makeTypeInfoBits(&ErrorCorrectionLevel::M, 5, &mut bits).expect("append");
    assert_eq!(" X......X X..XXX.", bits.to_string());
}
