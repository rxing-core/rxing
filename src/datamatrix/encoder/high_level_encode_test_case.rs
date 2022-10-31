/*
 * Copyright 2006 Jeremias Maerki.
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

use lazy_static::lazy_static;

use crate::datamatrix::encoder::{SymbolInfo, SymbolShapeHint};

use super::{high_level_encoder, minimal_encoder, symbol_info, SymbolInfoLookup};

lazy_static! {
  /**
 * Tests for {@link HighLevelEncoder} and {@link MinimalEncoder}
 */
  static ref  TEST_SYMBOLS :Vec<SymbolInfo>= vec![
     SymbolInfo::new(false, 3, 5, 8, 8, 1),
     SymbolInfo::new(false, 5, 7, 10, 10, 1),
      /*rect*/ SymbolInfo::new(true, 5, 7, 16, 6, 1),
     SymbolInfo::new(false, 8, 10, 12, 12, 1),
      /*rect*/ SymbolInfo::new(true, 10, 11, 14, 6, 2),
     SymbolInfo::new(false, 13, 0, 0, 0, 1),
     SymbolInfo::new(false, 77, 0, 0, 0, 1)
    //The last entries are fake entries to test special conditions with C40 encoding
  ];
}

// const SIL: SymbolInfoLookup = SymbolInfoLookup::new();

// fn useTestSymbols(lookup: SymbolInfoLookup) -> SymbolInfoLookup {
//     lookup.overrideSymbolSet(&TEST_SYMBOLS);
//     lookup
// }

// fn resetSymbols(lookup: SymbolInfoLookup) -> SymbolInfoLookup {
//     lookup.overrideSymbolSet(&symbol_info::PROD_SYMBOLS);
//     lookup
// }

#[test]
fn testASCIIEncodation() {
    let mut visualized = encodeHighLevel("123456");
    assert_eq!("142 164 186", visualized);

    visualized = encodeHighLevel("123456£");
    assert_eq!("142 164 186 235 36", visualized);

    visualized = encodeHighLevel("30Q324343430794<OQQ");
    assert_eq!("160 82 162 173 173 173 137 224 61 80 82 82", visualized);
}

#[test]
fn testC40EncodationBasic1() {
    let visualized = encodeHighLevel("AIMAIMAIM");
    assert_eq!("230 91 11 91 11 91 11 254", visualized);
    //230 shifts to C40 encodation, 254 unlatches, "else" case
}

#[test]
fn testC40EncodationBasic2() {
    let mut visualized = encodeHighLevel("AIMAIAB");
    assert_eq!("230 91 11 90 255 254 67 129", visualized);
    //"B" is normally encoded as "15" (one C40 value)
    //"else" case: "B" is encoded as ASCII

    visualized = encodeHighLevel("AIMAIAb");
    assert_eq!("66 74 78 66 74 66 99 129", visualized); //Encoded as ASCII
                                                        //Alternative solution:
                                                        //assert_eq!("230 91 11 90 255 254 99 129", visualized);
                                                        //"b" is normally encoded as "Shift 3, 2" (two C40 values)
                                                        //"else" case: "b" is encoded as ASCII

    visualized = encodeHighLevel("AIMAIMAIMË");
    assert_eq!("230 91 11 91 11 91 11 254 235 76", visualized);
    //Alternative solution:
    //assert_eq!("230 91 11 91 11 91 11 11 9 254", visualized);
    //Expl: 230 = shift to C40, "91 11" = "AIM",
    //"11 9" = "�" = "Shift 2, UpperShift, <char>
    //"else" case

    visualized = encodeHighLevel("AIMAIMAIMë");
    assert_eq!("230 91 11 91 11 91 11 254 235 108", visualized); //Activate when additional rectangulars are available
                                                                 //Expl: 230 = shift to C40, "91 11" = "AIM",
                                                                 //"�" in C40 encodes to: 1 30 2 11 which doesn't fit into a triplet
                                                                 //"10 243" =
                                                                 //254 = unlatch, 235 = Upper Shift, 108 = � = 0xEB/235 - 128 + 1
                                                                 //"else" case
}

#[test]
fn testC40EncodationSpecExample() {
    //Example in Figure 1 in the spec
    let visualized = encodeHighLevel("A1B2C3D4E5F6G7H8I9J0K1L2");
    assert_eq!(
        "230 88 88 40 8 107 147 59 67 126 206 78 126 144 121 35 47 254",
        visualized
    );
}

#[test]
fn testC40EncodationSpecialCases1() {
    unimplemented!();
    // //Special tests avoiding ultra-long test strings because these tests are only used
    // //with the 16x48 symbol (47 data codewords)
    // useTestSymbols();

    // let visualized = encodeHighLevelCompare("AIMAIMAIMAIMAIMAIM", false);
    // assert_eq!("230 91 11 91 11 91 11 91 11 91 11 91 11", visualized);
    // //case "a": Unlatch is not required

    // visualized = encodeHighLevelCompare("AIMAIMAIMAIMAIMAI", false);
    // assert_eq!("230 91 11 91 11 91 11 91 11 91 11 90 241", visualized);
    // //case "b": Add trailing shift 0 and Unlatch is not required

    // visualized = encodeHighLevel("AIMAIMAIMAIMAIMA");
    // assert_eq!("230 91 11 91 11 91 11 91 11 91 11 254 66", visualized);
    // //case "c": Unlatch and write last character in ASCII

    // resetSymbols();

    // visualized = encodeHighLevel("AIMAIMAIMAIMAIMAI");
    // assert_eq!(
    //     "230 91 11 91 11 91 11 91 11 91 11 254 66 74 129 237",
    //     visualized
    // );

    // visualized = encodeHighLevel("AIMAIMAIMA");
    // assert_eq!("230 91 11 91 11 91 11 66", visualized);
    // //case "d": Skip Unlatch and write last character in ASCII
}

#[test]
fn testC40EncodationSpecialCases2() {
    let visualized = encodeHighLevel("AIMAIMAIMAIMAIMAIMAI");
    assert_eq!(
        "230 91 11 91 11 91 11 91 11 91 11 91 11 254 66 74",
        visualized
    );
    //available > 2, rest = 2 --> unlatch and encode as ASCII
}

#[test]
fn testTextEncodation() {
    let mut visualized = encodeHighLevel("aimaimaim");
    assert_eq!("239 91 11 91 11 91 11 254", visualized);
    //239 shifts to Text encodation, 254 unlatches

    visualized = encodeHighLevel("aimaimaim'");
    assert_eq!("239 91 11 91 11 91 11 254 40 129", visualized);
    //assert_eq!("239 91 11 91 11 91 11 7 49 254", visualized);
    //This is an alternative, but doesn't strictly follow the rules in the spec.

    visualized = encodeHighLevel("aimaimaIm");
    assert_eq!("239 91 11 91 11 87 218 110", visualized);

    visualized = encodeHighLevel("aimaimaimB");
    assert_eq!("239 91 11 91 11 91 11 254 67 129", visualized);

    visualized = encodeHighLevel("aimaimaim{txt}\u{0004}");
    assert_eq!(
        "239 91 11 91 11 91 11 254 124 117 121 117 126 5 129 237",
        visualized
    );
}

#[test]
fn testX12Encodation() {
    //238 shifts to X12 encodation, 254 unlatches

    let mut visualized = encodeHighLevel("ABC>ABC123>AB");
    assert_eq!("238 89 233 14 192 100 207 44 31 67", visualized);

    visualized = encodeHighLevel("ABC>ABC123>ABC");
    assert_eq!("238 89 233 14 192 100 207 44 31 254 67 68", visualized);

    visualized = encodeHighLevel("ABC>ABC123>ABCD");
    assert_eq!("238 89 233 14 192 100 207 44 31 96 82 254", visualized);

    visualized = encodeHighLevel("ABC>ABC123>ABCDE");
    assert_eq!("238 89 233 14 192 100 207 44 31 96 82 70", visualized);

    visualized = encodeHighLevel("ABC>ABC123>ABCDEF");
    assert_eq!(
        "238 89 233 14 192 100 207 44 31 96 82 254 70 71 129 237",
        visualized
    );
}

#[test]
fn testEDIFACTEncodation() {
    //240 shifts to EDIFACT encodation

    let mut visualized = encodeHighLevel(".A.C1.3.DATA.123DATA.123DATA");
    assert_eq!(
        "240 184 27 131 198 236 238 16 21 1 187 28 179 16 21 1 187 28 179 16 21 1",
        visualized
    );

    visualized = encodeHighLevel(".A.C1.3.X.X2..");
    assert_eq!("240 184 27 131 198 236 238 98 230 50 47 47", visualized);

    visualized = encodeHighLevel(".A.C1.3.X.X2.");
    assert_eq!("240 184 27 131 198 236 238 98 230 50 47 129", visualized);

    visualized = encodeHighLevel(".A.C1.3.X.X2");
    assert_eq!("240 184 27 131 198 236 238 98 230 50", visualized);

    visualized = encodeHighLevel(".A.C1.3.X.X");
    assert_eq!("240 184 27 131 198 236 238 98 230 31", visualized);

    visualized = encodeHighLevel(".A.C1.3.X.");
    assert_eq!("240 184 27 131 198 236 238 98 231 192", visualized);

    visualized = encodeHighLevel(".A.C1.3.X");
    assert_eq!("240 184 27 131 198 236 238 89", visualized);

    //Checking temporary unlatch from EDIFACT
    visualized = encodeHighLevel(".XXX.XXX.XXX.XXX.XXX.XXX.üXX.XXX.XXX.XXX.XXX.XXX.XXX");
    assert_eq!(
        "240 185 134 24 185 134 24 185 134 24 185 134 24 185 134 24 185 134 24 124 47 235 125 240 97 139 152 97 139 152 97 139 152 97 139 152 97 139 152 97 139 152 89 89",
                  //   + " 124 47 235 125 240" //<-- this is the temporary unlatch
        visualized
    );
}

#[test]
fn testBase256Encodation() {
    //231 shifts to Base256 encodation

    let mut visualized = encodeHighLevel("\u{00AB}äöüé\u{00BB}");
    assert_eq!("231 44 108 59 226 126 1 104", visualized);
    visualized = encodeHighLevel("\u{00AB}äöüéà\u{00BB}");
    assert_eq!("231 51 108 59 226 126 1 141 254 129", visualized);
    visualized = encodeHighLevel("\u{00AB}äöüéàá\u{00BB}");
    assert_eq!("231 44 108 59 226 126 1 141 36 147", visualized);

    visualized = encodeHighLevel(" 23£"); //ASCII only (for reference)
    assert_eq!("33 153 235 36 129", visualized);

    visualized = encodeHighLevel("\u{00AB}äöüé\u{00BB} 234"); //Mixed Base256 + ASCII
    assert_eq!("231 50 108 59 226 126 1 104 33 153 53 129", visualized);

    visualized = encodeHighLevel("\u{00AB}äöüé\u{00BB} 23£ 1234567890123456789");
    assert_eq!("231 54 108 59 226 126 1 104 99 10 161 167 33 142 164 186 208 220 142 164 186 208 58 129 59 209 104 254 150 45", visualized);

    visualized = encodeHighLevel(&createBinaryMessage(20));
    assert_eq!(
        "231 44 108 59 226 126 1 141 36 5 37 187 80 230 123 17 166 60 210 103 253 150",
        visualized
    );
    visualized = encodeHighLevel(&createBinaryMessage(19)); //padding necessary at the end
    assert_eq!(
        "231 63 108 59 226 126 1 141 36 5 37 187 80 230 123 17 166 60 210 103 1 129",
        visualized
    );

    visualized = encodeHighLevel(&createBinaryMessage(276));
    assertStartsWith("231 38 219 2 208 120 20 150 35", &visualized);
    assertEndsWith("146 40 194 129", &visualized);

    visualized = encodeHighLevel(&createBinaryMessage(277));
    assertStartsWith("231 38 220 2 208 120 20 150 35", &visualized);
    assertEndsWith("146 40 190 87", &visualized);
}

fn createBinaryMessage(len: usize) -> String {
    let mut sb = String::new();
    sb.push_str("\u{00AB}äöüéàá-");
    for _i in 0..len - 9 {
        // for (int i = 0; i < len - 9; i++) {
        sb.push('\u{00B7}');
    }
    sb.push('\u{00BB}');

    sb
}

fn assertStartsWith(expected: &str, actual: &str) {
    if !actual.starts_with(expected) {
        panic!(
            "got {} expected {}, with len ({},{})",
            actual,
            expected,
            actual.len(),
            expected.len()
        )
        // throw new ComparisonFailure(null, expected, actual.substring(0, expected.length()));
    }
}

fn assertEndsWith(expected: &str, actual: &str) {
    if !actual.ends_with(expected) {
        panic!(
            "got {} expected {}, with len ({},{})",
            actual,
            expected,
            actual.len(),
            expected.len()
        )
        // throw new ComparisonFailure(null, expected, actual.substring(actual.length() - expected.length()));
    }
}

#[test]
fn testUnlatchingFromC40() {
    let visualized = encodeHighLevel("AIMAIMAIMAIMaimaimaim");
    assert_eq!(
        "230 91 11 91 11 91 11 254 66 74 78 239 91 11 91 11 91 11",
        visualized
    );
}

#[test]
fn testUnlatchingFromText() {
    let visualized = encodeHighLevel("aimaimaimaim12345678");
    assert_eq!(
        "239 91 11 91 11 91 11 91 11 254 142 164 186 208 129 237",
        visualized
    );
}

#[test]
fn testHelloWorld() {
    let visualized = encodeHighLevel("Hello World!");
    assert_eq!("73 239 116 130 175 123 148 64 158 233 254 34", visualized);
}

#[test]
fn testBug1664266() {
    //There was an exception and the encoder did not handle the unlatching from
    //EDIFACT encoding correctly

    let mut visualized = encodeHighLevel("CREX-TAN:h");
    assert_eq!("68 83 70 89 46 85 66 79 59 105", visualized);

    visualized = encodeHighLevel("CREX-TAN:hh");
    assert_eq!("68 83 70 89 46 85 66 79 59 105 105 129", visualized);

    visualized = encodeHighLevel("CREX-TAN:hhh");
    assert_eq!("68 83 70 89 46 85 66 79 59 105 105 105", visualized);
}

#[test]
fn testX12Unlatch() {
    let visualized = encodeHighLevel("*DTCP01");
    assert_eq!("43 69 85 68 81 131 129 56", visualized);
}

#[test]
fn testX12Unlatch2() {
    let visualized = encodeHighLevel("*DTCP0");
    assert_eq!("238 9 10 104 141", visualized);
}

#[test]
fn testBug3048549() {
    //There was an IllegalArgumentException for an illegal character here because
    //of an encoding problem of the character 0x0060 in Java source code.

    let visualized = encodeHighLevel("fiykmj*Rh2`,e6");
    assert_eq!(
        "103 106 122 108 110 107 43 83 105 51 97 45 102 55 129 237",
        visualized
    );
}

#[test]
fn testMacroCharacters() {
    let visualized = encodeHighLevel("[)>\u{001E}05\u{001D}5555\u{001C}6666\u{001E}\u{0004}");
    //assert_eq!("92 42 63 31 135 30 185 185 29 196 196 31 5 129 87 237", visualized);
    assert_eq!("236 185 185 29 196 196 129 56", visualized);
}

#[test]
fn testEncodingWithStartAsX12AndLatchToEDIFACTInTheMiddle() {
    let visualized = encodeHighLevel("*MEMANT-1F-MESTECH");
    assert_eq!(
        "240 168 209 77 4 229 45 196 107 77 21 53 5 12 135 192",
        visualized
    );
}

#[test]
fn testX12AndEDIFACTSpecErrors() {
    //X12 encoding error with spec conform float point comparisons in lookAheadTest()
    let mut visualized = encodeHighLevel("AAAAAAAAAAA**\u{00FC}AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert_eq!("230 89 191 89 191 89 191 89 178 56 114 10 243 177 63 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 254 66 129", visualized);
    //X12 encoding error with integer comparisons in lookAheadTest()
    visualized = encodeHighLevel("AAAAAAAAAAAA0+****AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert_eq!("238 89 191 89 191 89 191 89 191 254 240 194 186 170 170 160 65 4 16 65 4 16 65 4 16 65 4 16 65 4 16 65 4 16 65 4 16 65 124 129 167 62 212 107", visualized);
    //EDIFACT encoding error with spec conform float point comparisons in lookAheadTest()
    visualized = encodeHighLevel("AAAAAAAAAAA++++\u{00FC}AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert_eq!("230 89 191 89 191 89 191 254 66 66 44 44 44 44 235 125 230 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 254 129 17 167 62 212 107", visualized);
    //EDIFACT encoding error with integer comparisons in lookAheadTest()
    visualized = encodeHighLevel("++++++++++AAa0 0++++++++++++++++++++++++++++++");
    assert_eq!("240 174 186 235 174 186 235 174 176 65 124 98 240 194 12 43 174 186 235 174 186 235 174 186 235 174 186 235 174 186 235 174 186 235 174 186 235 173 240 129 167 62 212 107", visualized);
    visualized = encodeHighLevel("AAAAAAAAAAAA*+AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert_eq!("230 89 191 89 191 89 191 89 191 7 170 64 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 66", visualized);
    visualized = encodeHighLevel("AAAAAAAAAAA*0a0 *AAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert_eq!("230 89 191 89 191 89 191 89 178 56 227 6 228 7 183 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 89 191 254 66 66", visualized);
}
#[test]
fn testSizes() {
    let mut sizes = [0usize; 2];
    encodeHighLevelWithSizes("A", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("AB", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("ABC", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("ABCD", &mut sizes);
    assert_eq!(5, sizes[0]);
    assert_eq!(5, sizes[1]);

    encodeHighLevelWithSizes("ABCDE", &mut sizes);
    assert_eq!(5, sizes[0]);
    assert_eq!(5, sizes[1]);

    encodeHighLevelWithSizes("ABCDEF", &mut sizes);
    assert_eq!(5, sizes[0]);
    assert_eq!(5, sizes[1]);

    encodeHighLevelWithSizes("ABCDEFG", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("ABCDEFGH", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("ABCDEFGHI", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("ABCDEFGHIJ", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("a", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("ab", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("abc", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("abcd", &mut sizes);
    assert_eq!(5, sizes[0]);
    assert_eq!(5, sizes[1]);

    encodeHighLevelWithSizes("abcdef", &mut sizes);
    assert_eq!(5, sizes[0]);
    assert_eq!(5, sizes[1]);

    encodeHighLevelWithSizes("abcdefg", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("abcdefgh", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("+", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("++", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("+++", &mut sizes);
    assert_eq!(3, sizes[0]);
    assert_eq!(3, sizes[1]);

    encodeHighLevelWithSizes("++++", &mut sizes);
    assert_eq!(5, sizes[0]);
    assert_eq!(5, sizes[1]);

    encodeHighLevelWithSizes("+++++", &mut sizes);
    assert_eq!(5, sizes[0]);
    assert_eq!(5, sizes[1]);

    encodeHighLevelWithSizes("++++++", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("+++++++", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("++++++++", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("+++++++++", &mut sizes);
    assert_eq!(8, sizes[0]);
    assert_eq!(8, sizes[1]);

    encodeHighLevelWithSizes("\u{00F0}\u{00F0}ABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEFGHIJKLMNOPQRSTUVWXYZABCDEF", &mut sizes);
    assert_eq!(114, sizes[0]);
    assert_eq!(62, sizes[1]);
}

#[test]
fn testECIs() {
    let mut visualized = visualize(&minimal_encoder::encodeHighLevel("that particularly stands out to me is \u{0625}\u{0650}\u{062C}\u{064E}\u{0651}\u{0627}\u{0635} (\u{02BE}\u{0101}\u{1E63}) \"pear\", suggested to have originated from Hebrew \u{05D0}\u{05B7}\u{05D2}\u{05B8}\u{05BC}\u{05E1} (ag\u{00E1}s)").expect("encode"));
    assert_eq!("239 209 151 206 214 92 122 140 35 158 144 162 52 205 55 171 137 23 67 206 218 175 147 113 15 254 116 33 241 25 231 186 14 212 64 253 151 252 159 33 41 241 27 231 83 171 53 209 35 25 134 6 42 33 35 239 184 31 193 234 7 252 205 101 127 241 209 34 24 5 22 23 221 148 179 239 128 140 92 187 106 204 198 59 19 25 114 248 118 36 254 231 106 196 19 239 101 27 107 69 189 112 236 156 252 16 174 125 24 10 125 116 42 129",
        visualized);

    visualized = visualize(&minimal_encoder::encodeHighLevelWithDetails("that particularly stands out to me is \u{0625}\u{0650}\u{062C}\u{064E}\u{0651}\u{0627}\u{0635} (\u{02BE}\u{0101}\u{1E63}) \"pear\", suggested to have originated from Hebrew \u{05D0}\u{05B7}\u{05D2}\u{05B8}\u{05BC}\u{05E1} (ag\u{00E1}s)", Some(encoding::all::UTF_8), None , SymbolShapeHint::FORCE_NONE).expect("encode"));
    assert_eq!("241 27 239 209 151 206 214 92 122 140 35 158 144 162 52 205 55 171 137 23 67 206 218 175 147 113 15 254 116 33 231 202 33 131 77 154 119 225 163 238 206 28 249 93 36 150 151 53 108 246 145 228 217 71 199 42 33 35 239 184 31 193 234 7 252 205 101 127 241 209 34 24 5 22 23 221 148 179 239 128 140 92 187 106 204 198 59 19 25 114 248 118 36 254 231 43 133 212 175 38 220 44 6 125 49 172 93 189 209 111 61 217 203 62 116 42 129 1 151 46 196 91 241 137 32 182 77 227 122 18 168 63 213 108 4 154 49 199 94 244 140 35 185 80",
         visualized);
}

#[test]
fn testPadding() {
    let mut sizes = [0usize; 2];
    encodeHighLevelWithSizes("IS010000000000000000000000S1118058599124123S21.2.250.1.213.1.4.8 S3FIRST NAMETEST S5MS618-06-1985S713201S4LASTNAMETEST", &mut sizes);
    assert_eq!(86, sizes[0]);
    assert_eq!(86, sizes[1]);
}

fn encodeHighLevelWithSizes(msg: &str, sizes: &mut [usize]) {
    sizes[0] = high_level_encoder::encodeHighLevel(msg)
        .expect("encodes")
        .len();
    sizes[1] = minimal_encoder::encodeHighLevel(msg)
        .expect("encodes")
        .len();
}

fn encodeHighLevel(msg: &str) -> String {
    encodeHighLevelCompare(msg, true)
}

fn encodeHighLevelCompare(msg: &str, compareSizeToMinimalEncoder: bool) -> String {
    let encoded = high_level_encoder::encodeHighLevel(msg).expect("encodes");
    // let encoded2 = minimal_encoder::encodeHighLevel(msg).expect("encodes");
    // assert!(!compareSizeToMinimalEncoder || encoded2.len() <= encoded.len(), "{} <= {}", encoded2.len() , encoded.len());
    visualize(&encoded)
}

/**
 * Convert a string of char codewords into a different string which lists each character
 * using its decimal value.
 *
 * @param codewords the codewords
 * @return the visualized codewords
 */
fn visualize(codewords: &str) -> String {
    let mut sb = String::new();
    for i in 0..codewords.chars().count() {
        // for (int i = 0; i < codewords.length(); i++) {
        if i > 0 {
            sb.push(' ');
        }
        sb.push_str(&format!("{}", codewords.chars().nth(i).unwrap() as u32));
    }

    sb
}
