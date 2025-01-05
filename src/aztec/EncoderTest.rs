/*
 * Copyright 2013 ZXing authors
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
    aztec::{
        aztec_detector_result::AztecDetectorRXingResult,
        decoder,
        encoder::HighLevelEncoder,
        shared_test_methods::{stripSpace, toBitArray, toBooleanArray},
    },
    common::CharacterSet,
    BarcodeFormat, EncodeHints, Point,
};

use super::{encoder::aztec_encoder, AztecWriter};

use crate::Writer;

use rand::Rng;

/**
 * Aztec 2D generator unit tests.
 *
 * @author Rustam Abdullaev
 * @author Frank Yellin
 */

const ISO_8859_1: CharacterSet = CharacterSet::ISO8859_1; //StandardCharsets.ISO_8859_1;
const UTF_8: CharacterSet = CharacterSet::UTF8; //StandardCharsets.UTF_8;
const ISO_8859_15: CharacterSet = CharacterSet::ISO8859_15; //Charset.forName("ISO-8859-15");
const WINDOWS_1252: CharacterSet = CharacterSet::Cp1252; //Charset.forName("Windows-1252");

// const DOTX: &str = "[^.X]";
// const SPACES: &str = "\\s+";
const NO_POINTS: [Point; 4] = [Point { x: 0.0, y: 0.0 }; 4];

// real life tests

#[test]
fn testEncode1() {
    testEncode(
        "This is an example Aztec symbol for Wikipedia.",
        true,
        3,
        r"X     X X       X     X X     X     X         
X         X     X X     X   X X   X X       X 
X X   X X X X X   X X X                 X     
X X                 X X   X       X X X X X X 
    X X X   X   X     X X X X         X X     
  X X X   X X X X   X     X   X     X X   X   
        X X X X X     X X X X   X   X     X   
X       X   X X X X X X X X X X X     X   X X 
X   X     X X X               X X X X   X X   
X     X X   X X   X X X X X   X X   X   X X X 
X   X         X   X       X   X X X X       X 
X       X     X   X   X   X   X   X X   X     
      X   X X X   X       X   X     X X X     
    X X X X X X   X X X X X   X X X X X X   X 
  X X   X   X X               X X X   X X X X 
  X   X       X X X X X X X X X X X X   X X   
  X X   X       X X X   X X X       X X       
  X               X   X X     X     X X X     
  X   X X X   X X   X   X X X X   X   X X X X 
    X   X   X X X   X   X   X X X X     X     
        X               X                 X   
        X X     X   X X   X   X   X       X X 
  X   X   X X       X   X         X X X     X 
",
    );
}

#[test]
fn testEncode2() {
    testEncode(
        "Aztec Code is a public domain 2D matrix barcode symbology of nominally square symbols built on a square grid with a distinctive square bullseye pattern at their center.",
        false,
        6,
        r"        X X     X X     X     X     X   X X X         X   X         X   X X       
  X       X X     X   X X   X X       X             X     X   X X   X           X 
  X   X X X     X   X   X X     X X X   X   X X               X X       X X     X 
X X X             X   X         X         X     X     X   X     X X       X   X   
X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X 
    X X   X   X   X X X               X       X       X X     X X   X X       X   
X X     X       X       X X X X   X   X X       X   X X   X       X X   X X   X   
  X       X   X     X X   X   X X   X X   X X X X X X   X X           X   X   X X 
X X   X X   X   X X X X   X X X X X X X X   X   X       X X   X X X X   X X X     
  X       X   X     X       X X     X X   X   X   X     X X   X X X   X     X X X 
  X   X X X   X X       X X X         X X           X   X   X   X X X   X X     X 
    X     X   X X     X X X X     X   X     X X X X   X X   X X   X X X     X   X 
X X X   X             X         X X X X X   X   X X   X   X   X X   X   X   X   X 
          X       X X X   X X     X   X           X   X X X X   X X               
  X     X X   X   X       X X X X X X X X X X X X X X X   X   X X   X   X X X     
    X X                 X   X                       X X   X       X         X X X 
        X   X X   X X X X X X   X X X X X X X X X   X     X X           X X X X   
          X X X   X     X   X   X               X   X X     X X X   X X           
X X     X     X   X   X   X X   X   X X X X X   X   X X X X X X X       X   X X X 
X X X X       X       X   X X   X   X       X   X   X     X X X     X X       X X 
X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X 
    X     X       X         X   X   X       X   X   X     X   X X                 
        X X     X X X X X   X   X   X X X X X   X   X X X     X X X X   X         
X     X   X   X         X   X   X               X   X X   X X   X X X     X   X   
  X   X X X   X   X X   X X X   X X X X X X X X X   X X         X X     X X X X   
    X X   X   X   X X X     X                       X X X   X X   X   X     X     
    X X X X   X         X   X X X X X X X X X X X X X X   X       X X   X X   X X 
            X   X   X X       X X X X X     X X X       X       X X X         X   
X       X         X   X X X X   X     X X     X X     X X           X   X       X 
X     X       X X X X X     X   X X X X   X X X     X       X X X X   X   X X   X 
  X X X X X               X     X X X   X       X X   X X   X X X X     X X       
X             X         X   X X   X X     X     X     X   X   X X X X             
    X   X X       X     X       X   X X X X X X   X X   X X X X X X X X X   X   X 
    X         X X   X       X     X   X   X       X     X X X     X       X X X X 
X     X X     X X X X X X             X X X   X               X   X     X     X X 
X   X X     X               X X X X X     X X     X X X X X X X X     X   X   X X 
X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X 
X           X     X X X X     X     X         X         X   X       X X   X X X   
X   X   X X   X X X   X         X X     X X X X     X X   X   X     X   X       X 
      X     X     X     X X     X   X X   X X   X         X X       X       X   X 
X       X           X   X   X     X X   X               X     X     X X X         
",
    );
}

#[test]
fn testAztecWriter() {
    let shift_jis: CharacterSet =
        CharacterSet::get_character_set_by_name("Shift_JIS").expect("must exist");

    testWriter("Espa\u{00F1}ol", None, 25, true, 1); // Without ECI (implicit ISO-8859-1)
    testWriter("Espa\u{00F1}ol", Some(ISO_8859_1), 25, true, 1); // Explicit ISO-8859-1
    testWriter("\u{20AC} 1 sample data.", Some(WINDOWS_1252), 25, true, 2); // ISO-8859-1 can't encode Euro; Windows-1252 can
    testWriter("\u{20AC} 1 sample data.", Some(WINDOWS_1252), 5, true, 2); // ISO-8859-1 can't encode Euro; Windows-1252 can
    testWriter("\u{20AC} 1 sample data.", Some(ISO_8859_15), 25, true, 2);
    testWriter("\u{20AC} 1 sample data.", Some(ISO_8859_15), 0, true, 2);
    testWriter(
        "\u{20AC} 1 sample data.",
        Some(CharacterSet::UTF16BE),
        0,
        true,
        3,
    );
    testWriter("Espa\u{00F1}ol", Some(UTF_8), 25, true, 2);
    testWriter("\u{20AC} 1 sample data", Some(UTF_8), 0, true, 2);
    testWriter("\u{20AC} 1 sample data. ", Some(UTF_8), 0, true, 2);
    testWriter("\u{20AC} 1 sample data .", Some(UTF_8), 0, true, 2);
    testWriter("\u{20AC} 1 sample data-.", Some(UTF_8), 0, true, 2);
    testWriter("\u{20AC} 1 sample datA.", Some(UTF_8), 0, true, 2);
    testWriter("\u{20AC} 1 sample dat1.", Some(UTF_8), 0, true, 2);
    testWriter("\u{20AC} 1 sample data.", Some(UTF_8), 0, true, 2);
    testWriter("\u{20AC} 1 sample data.", Some(UTF_8), 25, true, 2);
    testWriter("\u{20AC} 1 sample data.", Some(UTF_8), 100, true, 3);
    testWriter("\u{20AC} 1 sample data.", Some(UTF_8), 300, true, 4);
    testWriter("\u{20AC} 1 sample data.", Some(UTF_8), 500, false, 5);
    testWriter(
        "The capital of Japan is named \u{6771}\u{4EAC}.",
        Some(shift_jis),
        25,
        true,
        3,
    );
    // Test AztecWriter defaults
    let data = "In ut magna vel mauris malesuada";
    let writer = AztecWriter {};
    let matrix = writer
        .encode(data, &BarcodeFormat::AZTEC, 0, 0)
        .expect("matrix must exist");
    let aztec = aztec_encoder::encode(
        data,
        aztec_encoder::DEFAULT_EC_PERCENT,
        aztec_encoder::DEFAULT_AZTEC_LAYERS,
    )
    .expect("encode should succeed");
    let expected_matrix = aztec.getMatrix();
    assert_eq!(&matrix, expected_matrix);
}

// synthetic tests (encode-decode round-trip)

#[test]
fn testEncodeDecode1() {
    testEncodeDecode("Abc123!", true, 1);
}

#[test]
fn testEncodeDecode2() {
    testEncodeDecode("Lorem ipsum. http://test/", true, 2);
}

#[test]
fn testEncodeDecode3() {
    testEncodeDecode(
        "AAAANAAAANAAAANAAAANAAAANAAAANAAAANAAAANAAAANAAAAN",
        true,
        3,
    );
}

#[test]
fn testEncodeDecode4() {
    testEncodeDecode("http://test/~!@#*^%&)__ ;:'\"[]{}\\|-+-=`1029384", true, 4);
}

#[test]
fn testEncodeDecode5() {
    testEncodeDecode(
        r#"http://test/~!@#*^%&)__ ;:'"[]{}\|-+-=`1029384756<>/?abc
Four score and seven our forefathers brought forth"#,
        false,
        5,
    );
}

#[test]
fn testEncodeDecode10() {
    testEncodeDecode(
        "In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus quis diam cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum sit amet. Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar nisi, id elementum sapien dolor et diam.",
        false,
        10,
    );
}

#[test]
fn testEncodeDecode23() {
    testEncodeDecode(
        "In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus quis diam\
 cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec laoreet rutrum\
 est, nec convallis mauris condimentum sit amet. Phasellus gravida, justo et congue\
 auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec lorem. Nulla\
 ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar nisi, id\
 elementum sapien dolor et diam. Donec ac nunc sodales elit placerat eleifend.\
 Sed ornare luctus ornare. Vestibulum vehicula, massa at pharetra fringilla, risus\
 justo faucibus erat, nec porttitor nibh tellus sed est. Ut justo diam, lobortis eu\
 tristique ac, p.In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus\
 quis diam cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec\
 laoreet rutrum est, nec convallis mauris condimentum sit amet. Phasellus gravida\
 justo et congue auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec\
 lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar\
 nisi, id elementum sapien dolor et diam. Donec ac nunc sodales elit placerat\
 eleifend. Sed ornare luctus ornare. Vestibulum vehicula, massa at pharetra\
 fringilla, risus justo faucibus erat, nec porttitor nibh tellus sed est. Ut justo\
 diam, lobortis eu tristique ac, p. In ut magna vel mauris malesuada dictum. Nulla\
 ullamcorper metus quis diam cursus facilisis. Sed mollis quam id justo rutrum\
 sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum sit amet.\
 Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat, eget hendrerit\
 felis turpis nec lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo\
 erat pulvinar nisi, id elementum sapien dolor et diam.",
        false,
        23,
    );
}

#[test]
fn testEncodeDecode31() {
    testEncodeDecode(
        "In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus quis diam\
 cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec laoreet rutrum\
 est, nec convallis mauris condimentum sit amet. Phasellus gravida, justo et congue\
 auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec lorem. Nulla\
 ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar nisi, id\
 elementum sapien dolor et diam. Donec ac nunc sodales elit placerat eleifend.\
 Sed ornare luctus ornare. Vestibulum vehicula, massa at pharetra fringilla, risus\
 justo faucibus erat, nec porttitor nibh tellus sed est. Ut justo diam, lobortis eu\
 tristique ac, p.In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus\
 quis diam cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec\
 laoreet rutrum est, nec convallis mauris condimentum sit amet. Phasellus gravida,\
 justo et congue auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec\
 lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar\
 nisi, id elementum sapien dolor et diam. Donec ac nunc sodales elit placerat\
 eleifend. Sed ornare luctus ornare. Vestibulum vehicula, massa at pharetra\
 fringilla, risus justo faucibus erat, nec porttitor nibh tellus sed est. Ut justo\
 diam, lobortis eu tristique ac, p. In ut magna vel mauris malesuada dictum. Nulla\
 ullamcorper metus quis diam cursus facilisis. Sed mollis quam id justo rutrum\
 sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum sit amet.\
 Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat, eget hendrerit\
 felis turpis nec lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo\
 erat pulvinar nisi, id elementum sapien dolor et diam. Donec ac nunc sodales elit\
 placerat eleifend. Sed ornare luctus ornare. Vestibulum vehicula, massa at\
 pharetra fringilla, risus justo faucibus erat, nec porttitor nibh tellus sed est.\
 Ut justo diam, lobortis eu tristique ac, p.In ut magna vel mauris malesuada\
 dictum. Nulla ullamcorper metus quis diam cursus facilisis. Sed mollis quam id\
 justo rutrum sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum\
 sit amet. Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat,\
 eget hendrerit felis turpis nec lorem. Nulla ultrices, elit pellentesque aliquet\
 laoreet, justo erat pulvinar nisi, id elementum sapien dolor et diam. Donec ac\
 nunc sodales elit placerat eleifend. Sed ornare luctus ornare. Vestibulum vehicula,\
 massa at pharetra fringilla, risus justo faucibus erat, nec porttitor nibh tellus\
 sed est. Ut justo diam, lobortis eu tris. In ut magna vel mauris malesuada dictum.\
 Nulla ullamcorper metus quis diam cursus facilisis. Sed mollis quam id justo rutrum\
 sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum sit amet.\
 Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat, eget\
 hendrerit felis turpis nec lorem.",
        false,
        31,
    );
}

#[test]
fn testGenerateModeMessage() {
    testModeMessageComplex(true, 2, 29, ".X .XXX.. ...X XX.. ..X .XX. .XX.X");
    testModeMessageComplex(true, 4, 64, "XX XXXXXX .X.. ...X ..XX .X.. XX..");
    testModeMessageComplex(
        false,
        21,
        660,
        "X.X.. .X.X..X..XX .XXX ..X.. .XXX. .X... ..XXX",
    );
    testModeMessageComplex(
        false,
        32,
        4096,
        "XXXXX XXXXXXXXXXX X.X. ..... XXX.X ..X.. X.XXX",
    );
}

#[test]
fn testStuffBitsTest() {
    testStuffBits(5, ".X.X. X.X.X .X.X.", ".X.X. X.X.X .X.X.");
    testStuffBits(5, ".X.X. ..... .X.X", ".X.X. ....X ..X.X");
    testStuffBits(
        3,
        "XX. ... ... ..X XXX .X. ..",
        "XX. ..X ..X ..X ..X .XX XX. .X. ..X",
    );
    testStuffBits(6, ".X.X.. ...... ..X.XX", ".X.X.. .....X. ..X.XX XXXX.");
    testStuffBits(
        6,
        ".X.X.. ...... ...... ..X.X.",
        ".X.X.. .....X .....X ....X. X.XXXX",
    );
    testStuffBits(
        6,
        ".X.X.. XXXXXX ...... ..X.XX",
        ".X.X.. XXXXX. X..... ...X.X XXXXX.",
    );
    testStuffBits(6,
        "...... ..XXXX X..XX. .X.... .X.X.X .....X .X.... ...X.X .....X ....XX ..X... ....X. X..XXX X.XX.X",
        ".....X ...XXX XX..XX ..X... ..X.X. X..... X.X... ....X. X..... X....X X..X.. .....X X.X..X XXX.XX .XXXXX");
}

#[test]
fn testHighLevelEncode() {
    testHighLevelEncodeString(
        "A. b.",
        // 'A'  P/S   '. ' L/L    b    D/L    '.'
        "...X. ..... ...XX XXX.. ...XX XXXX. XX.X",
    );
    testHighLevelEncodeString(
        "Lorem ipsum.",
        // 'L'  L/L   'o'   'r'   'e'   'm'   ' '   'i'   'p'   's'   'u'   'm'   D/L   '.'
        ".XX.X XXX.. X.... X..XX ..XX. .XXX. ....X .X.X. X...X X.X.. X.XX. .XXX. XXXX. XX.X",
    );
    testHighLevelEncodeString("Lo. Test 123.",
        // 'L'  L/L   'o'   P/S   '. '  U/S   'T'   'e'   's'   't'    D/L   ' '  '1'  '2'  '3'  '.'
        ".XX.X XXX.. X.... ..... ...XX XXX.. X.X.X ..XX. X.X.. X.X.X  XXXX. ...X ..XX .X.. .X.X XX.X");
    testHighLevelEncodeString(
        "Lo...x",
        // 'L'  L/L   'o'   D/L   '.'  '.'  '.'  U/L  L/L   'x'
        ".XX.X XXX.. X.... XXXX. XX.X XX.X XX.X XXX. XXX.. XX..X",
    );
    testHighLevelEncodeString(". x://abc/.",
        //P/S   '. '  L/L   'x'   P/S   ':'   P/S   '/'   P/S   '/'   'a'   'b'   'c'   P/S   '/'   D/L   '.'
        "..... ...XX XXX.. XX..X ..... X.X.X ..... X.X.. ..... X.X.. ...X. ...XX ..X.. ..... X.X.. XXXX. XX.X");
    // Uses Binary/Shift rather than Lower/Shift to save two bits.
    testHighLevelEncodeString(
        "ABCdEFG",
        //'A'   'B'   'C'   B/S    =1    'd'     'E'   'F'   'G'
        "...X. ...XX ..X.. XXXXX ....X .XX..X.. ..XX. ..XXX .X...",
    );

    testHighLevelEncodeStringCount(
        // Found on an airline boarding pass.  Several stretches of Binary shift are
        // necessary to keep the bitcount so low.
        "09  UAG    ^160MEUCIQC0sYS/HpKxnBELR1uB85R20OoqqwFGa0q2uEiYgh6utAIgLl1aBVM4EOTQtMQQYH9M2Z3Dp4qnA/fwWuQ+M8L3V8U=",
        823);
}

#[test]
fn testHighLevelEncodeBinary() {
    // binary short form single byte
    testHighLevelEncodeString(
        "N\0N",
        // 'N'  B/S    =1   '\0'      N
        ".XXXX XXXXX ....X ........ .XXXX",
    ); // Encode "N" in UPPER

    testHighLevelEncodeString(
        "N\0n",
        // 'N'  B/S    =2   '\0'       'n'
        ".XXXX XXXXX ...X. ........ .XX.XXX.",
    ); // Encode "n" in BINARY

    // binary short form consecutive bytes
    testHighLevelEncodeString(
        "N\0\u{0080} A",
        // 'N'  B/S    =2    '\0'    \u0080   ' '  'A'
        ".XXXX XXXXX ...X. ........ X....... ....X ...X.",
    );

    // binary skipping over single character
    testHighLevelEncodeString(
        "\0a\u{00FF}\u{0080} A",
        // B/S  =4    '\0'      'a'     '\3ff'   '\200'   ' '   'A'
        "XXXXX ..X.. ........ .XX....X XXXXXXXX X....... ....X ...X.",
    );

    // getting into binary mode from digit mode
    testHighLevelEncodeString(
        "1234\0",
        //D/L   '1'  '2'  '3'  '4'  U/L  B/S    =1    \0
        "XXXX. ..XX .X.. .X.X .XX. XXX. XXXXX ....X ........",
    );

    testHighLevelEncodeStringUtf8("\u{20AC} 1 sample data.", 
    "...........X..X..X...XXXXX...XXXXX...X.X.....X.X.X.XX..XXXX....X..XX...XXXX.XXX..X.X.....X..XXX.X...X.XX.X..XX.....X..X.X...X.X.X.X...X.XXXX.XX.X");

    // Create a string in which every character requires binary
    let mut sb = String::new();
    for i in 0..3000 {
        // for (int i = 0; i <= 3000; i++) {
        sb.push(char::from_u32(128 + (i % 30)).unwrap());
    }
    // Test the output generated by Binary/Switch, particularly near the
    // places where the encoding changes: 31, 62, and 2047+31=2078
    for i in [
        1, 2, 3, 10, 29, 30, 31, 32, 33, 60, 61, 62, 63, 64, 2076, 2077, 2078, 2079, 2080, 2100,
    ] {
        // for (int i : new int[] { 1, 2, 3, 10, 29, 30, 31, 32, 33,
        //                          60, 61, 62, 63, 64, 2076, 2077, 2078, 2079, 2080, 2100 }) {
        // This is the expected length of a binary string of length "i"
        let expected_length = (8 * i)
            + if i <= 31 {
                10
            } else if i <= 62 {
                20
            } else if i <= 2078 {
                21
            } else {
                31
            };
        // ( (i <= 31) ? 10 : (i <= 62) ? 20 : (i <= 2078) ? 21 : 31);
        // Verify that we are correct about the length.
        let substring_for_test: String = sb.chars().take(i).collect();
        testHighLevelEncodeStringCount(&substring_for_test, expected_length as u32);
        if i != 1 && i != 32 && i != 2079 {
            // The addition of an 'a' at the beginning or end gets merged into the binary code
            // in those cases where adding another binary character only adds 8 or 9 bits to the result.
            // So we exclude the border cases i=1,32,2079
            // A lower case letter at the beginning will be merged into binary mode
            let substring_for_sub_test: String = sb.chars().take(i - 1).collect();
            testHighLevelEncodeStringCount(
                &format!("a{}", &substring_for_sub_test),
                expected_length as u32,
            );
            // A lower case letter at the end will also be merged into binary mode
            testHighLevelEncodeStringCount(
                &format!("{}a", &substring_for_sub_test),
                expected_length as u32,
            );
        }
        // A lower case letter at both ends will enough to latch us into LOWER.
        testHighLevelEncodeStringCount(
            &format!("a{}b", &substring_for_test),
            expected_length as u32 + 15,
        );
    }

    sb.clear();

    // sb.push('A');
    for _i in 0..32 {
        // for (int i = 0; i < 32; i++) {
        sb.push('§'); // § forces binary encoding
    }
    sb.replace_range(
        sb.char_indices()
            .nth(1)
            .map(|(pos, ch)| pos..pos + ch.len_utf8())
            .unwrap(),
        "A",
    );
    // sb.setCharAt(1, 'A');
    // expect B/S(1) A B/S(30)
    testHighLevelEncodeStringCount(&sb, 5 + 20 + 31 * 8);

    sb.clear();

    // sb.push('A');
    for _i in 0..31 {
        // for (int i = 0; i < 31; i++) {
        sb.push('§');
    }
    // sb.replace_range(1..2, "A");
    sb.replace_range(
        sb.char_indices()
            .nth(1)
            .map(|(pos, ch)| pos..pos + ch.len_utf8())
            .unwrap(),
        "A",
    );
    // sb.setCharAt(1, 'A');
    // expect B/S(31)
    testHighLevelEncodeStringCount(&sb, 10 + 31 * 8);

    sb.clear();

    // sb.push('A');
    for _i in 0..34 {
        // for (int i = 0; i < 34; i++) {
        sb.push('§');
    }
    //sb.replace_range(1..2, "A");
    sb.replace_range(
        sb.char_indices()
            .nth(1)
            .map(|(pos, ch)| pos..pos + ch.len_utf8())
            .unwrap(),
        "A",
    );
    // sb.setCharAt(1, 'A');
    // expect B/S(31) B/S(3)
    testHighLevelEncodeStringCount(&sb, 20 + 34 * 8);

    sb.clear();

    for _i in 0..64 {
        // for (int i = 0; i < 64; i++) {
        sb.push('§');
    }
    //sb.replace_range(30..31, "A");
    sb.replace_range(
        sb.char_indices()
            .nth(30)
            .map(|(pos, ch)| pos..pos + ch.len_utf8())
            .unwrap(),
        "A",
    );
    // sb.setCharAt(30, 'A');
    // expect B/S(64)
    testHighLevelEncodeStringCount(&sb, 21 + 64 * 8);
}

#[test]
fn testHighLevelEncodePairs() {
    // Typical usage
    testHighLevelEncodeString(
        "ABC. DEF\r\n",
        //  A     B    C    P/S   .<sp>   D    E     F    P/S   \r\n
        "...X. ...XX ..X.. ..... ...XX ..X.X ..XX. ..XXX ..... ...X.",
    );

    // We should latch to PUNCT mode, rather than shift.  Also check all pairs
    testHighLevelEncodeString(
        "A. : , \r\n",
        // 'A'    M/L   P/L   ". "  ": "   ", " "\r\n"
        "...X. XXX.X XXXX. ...XX ..X.X  ..X.. ...X.",
    );

    // Latch to DIGIT rather than shift to PUNCT
    testHighLevelEncodeString(
        "A. 1234",
        // 'A'  D/L   '.'  ' '  '1' '2'   '3'  '4'
        "...X. XXXX. XX.X ...X ..XX .X.. .X.X .X X.",
    );
    // Don't bother leaving Binary Shift.
    testHighLevelEncodeString(
        "A\u{80}. \u{80}",
        // 'A'  B/S    =2    \200      "."     " "     \200
        "...X. XXXXX ..X.. X....... ..X.XXX. ..X..... X.......",
    );
}

#[test]
#[should_panic]
fn testUserSpecifiedLayers() {
    doTestUserSpecifiedLayers(33);
}

#[test]
#[should_panic]
fn testUserSpecifiedLayers2() {
    doTestUserSpecifiedLayers(-1);
}

fn doTestUserSpecifiedLayers(userSpecifiedLayers: i32) {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut aztec = aztec_encoder::encode(alphabet, 25, -2).expect("should encode");
    assert_eq!(2, aztec.getLayers());
    assert!(aztec.isCompact());

    aztec = aztec_encoder::encode(alphabet, 25, 32).expect("should encode");
    assert_eq!(32, aztec.getLayers());
    assert!(!aztec.isCompact());

    aztec_encoder::encode(alphabet, 25, userSpecifiedLayers).expect("encode");
}

#[test]
#[should_panic]
fn testBorderCompact4CaseFailed() {
    // Compact(4) con hold 608 bits of information, but at most 504 can be data.  Rest must
    // be error correction
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // encodes as 26 * 5 * 4 = 520 bits of data
    let alphabet4 = format!("{alphabet}{alphabet}{alphabet}{alphabet}");
    aztec_encoder::encode(&alphabet4, 0, -4).expect("encode");
}

#[test]
fn testBorderCompact4Case() {
    // Compact(4) con hold 608 bits of information, but at most 504 can be data.  Rest must
    // be error correction
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // encodes as 26 * 5 * 4 = 520 bits of data
    let alphabet4 = format!("{alphabet}{alphabet}{alphabet}{alphabet}");

    // If we just try to encode it normally, it will go to a non-compact 4 layer
    let mut aztecCode = aztec_encoder::encode(&alphabet4, 0, aztec_encoder::DEFAULT_AZTEC_LAYERS)
        .expect("Should encode");
    assert!(!aztecCode.isCompact());
    assert_eq!(4, aztecCode.getLayers());

    // But shortening the string to 100 bytes (500 bits of data), compact works fine, even if we
    // include more error checking.
    aztecCode = aztec_encoder::encode(&alphabet4[..100], 10, aztec_encoder::DEFAULT_AZTEC_LAYERS)
        .expect("should encode");
    assert!(aztecCode.isCompact());
    assert_eq!(4, aztecCode.getLayers());
}

// Helper routines

fn testEncode(data: &str, compact: bool, layers: u32, expected: &str) {
    let aztec = aztec_encoder::encode(data, 33, aztec_encoder::DEFAULT_AZTEC_LAYERS)
        .expect("should encode");
    assert_eq!(
        compact,
        aztec.isCompact(),
        "Unexpected symbol format (compact)"
    );
    assert_eq!(layers, aztec.getLayers(), "Unexpected nr. of layers");
    let matrix = aztec.getMatrix();

    // let mut xored = BitMatrix::parse_strings(&stripSpace(expected), "X ", "  ").expect("should parse");
    // xored.xor(matrix).expect("should xor");
    assert_eq!(expected, matrix.to_string(), "encode({data}) failed");
}

fn testEncodeDecode(data: &str, compact: bool, layers: u32) {
    let aztec = aztec_encoder::encode(data, 25, aztec_encoder::DEFAULT_AZTEC_LAYERS)
        .expect("should encode");
    assert_eq!(
        compact,
        aztec.isCompact(),
        "Unexpected symbol format (compact)"
    );
    assert_eq!(layers, aztec.getLayers(), "Unexpected nr. of layers");
    let mut matrix = aztec.getMatrix().clone();
    let mut r = AztecDetectorRXingResult::new(
        matrix.clone(),
        NO_POINTS,
        aztec.isCompact(),
        aztec.getCodeWords(),
        aztec.getLayers(),
    );
    let mut res = decoder::decode(&r).expect("decode ok");
    assert_eq!(data, res.getText());
    // Check error correction by introducing a few minor errors
    let mut random = getPseudoRandom();
    matrix.flip_coords(
        random.gen_range(0..matrix.getWidth()),
        random.gen_range(0..=2),
    );
    matrix.flip_coords(
        random.gen_range(0..matrix.getWidth()),
        matrix.getHeight() - 2 + random.gen_range(0..2),
    );
    matrix.flip_coords(
        random.gen_range(0..=2),
        random.gen_range(0..matrix.getHeight()),
    );
    matrix.flip_coords(
        matrix.getWidth() - 2 + random.gen_range(0..2),
        random.gen_range(0..matrix.getHeight()),
    );
    r = AztecDetectorRXingResult::new(
        matrix,
        NO_POINTS,
        aztec.isCompact(),
        aztec.getCodeWords(),
        aztec.getLayers(),
    );
    res = decoder::decode(&r).expect("decode should work");
    assert_eq!(data, res.getText());
}

fn testWriter(
    data: &str,
    charset: Option<CharacterSet>,
    ecc_percent: u32,
    compact: bool,
    layers: u32,
) {
    // Perform an encode-decode round-trip because it can be lossy.
    let mut hints = EncodeHints::default();
    if let Some(cs) = charset {
        hints.CharacterSet = Some(cs.get_charset_name().to_string());
    }
    // if (null != charset) {
    //   hints.put(EncodeHintType.CHARACTER_SET, charset.name());
    // }
    hints.ErrorCorrection = Some(ecc_percent.to_string());

    let mut matrix = AztecWriter {}
        .encode_with_hints(data, &BarcodeFormat::AZTEC, 0, 0, &hints)
        .expect("encoder created");

    let cset = match charset {
        Some(cs) => cs,
        None => CharacterSet::ISO8859_1,
    };
    let aztec = aztec_encoder::encode_with_charset(
        data,
        ecc_percent,
        aztec_encoder::DEFAULT_AZTEC_LAYERS,
        cset,
    )
    .expect("encode should encode");
    assert_eq!(
        compact,
        aztec.isCompact(),
        "Unexpected symbol format (compact)"
    );
    assert_eq!(layers, aztec.getLayers(), "Unexpected nr. of layers");
    let matrix2 = aztec.getMatrix();
    assert_eq!(&matrix, matrix2);

    let mut r = AztecDetectorRXingResult::new(
        matrix.clone(),
        NO_POINTS,
        aztec.isCompact(),
        aztec.getCodeWords(),
        aztec.getLayers(),
    );

    let mut res = decoder::decode(&r).expect("should decode");
    assert_eq!(data, res.getText());

    // Check error correction by introducing up to eccPercent/2 errors
    let ec_words = aztec.getCodeWords() * ecc_percent / 100 / 2;
    let mut random = getPseudoRandom();
    for _i in 0..ec_words {
        // for (int i = 0; i < ecWords; i++) {
        // don't touch the core
        let x = if random.gen_bool(0.50) {
            random.gen_range(0..=aztec.getLayers() * 2)
        } else {
            matrix.getWidth() - 1 - random.gen_range(0..=aztec.getLayers() * 2)
        };
        let y = if random.gen_bool(0.50) {
            random.gen_range(0..=aztec.getLayers() * 2)
        } else {
            matrix.getHeight() - 1 - random.gen_range(0..=aztec.getLayers() * 2)
        };
        matrix.flip_coords(x, y);
    }
    r = AztecDetectorRXingResult::new(
        matrix,
        NO_POINTS,
        aztec.isCompact(),
        aztec.getCodeWords(),
        aztec.getLayers(),
    );
    res = decoder::decode(&r).expect("must decode");
    assert_eq!(data, res.getText());
}

fn getPseudoRandom() -> rand::rngs::ThreadRng {
    rand::thread_rng()
}

fn testModeMessageComplex(compact: bool, layers: u32, words: u32, expected: &str) {
    let indata = aztec_encoder::generateModeMessage(compact, layers, words).expect("generate mode");
    assert_eq!(
        stripSpace(expected),
        stripSpace(&indata.to_string()),
        "generateModeMessage() failed"
    );
}

fn testStuffBits(wordSize: usize, bits: &str, expected: &str) {
    let indata = toBitArray(bits);
    let stuffed = aztec_encoder::stuffBits(&indata, wordSize).unwrap();
    assert_eq!(
        stripSpace(expected),
        stripSpace(&stuffed.to_string()),
        "stuffBits() failed for input string: {bits}"
    );
}

fn testHighLevelEncodeStringUtf8(s: &str, expectedBits: &str) {
    let bits = HighLevelEncoder::with_charset(
        CharacterSet::UTF8
            .encode(s)
            .expect("should encode to bytes"),
        CharacterSet::UTF8,
    )
    .encode()
    .expect("high level ok");
    // let bits =  HighLevelEncoder::new(s.getBytes(StandardCharsets.ISO_8859_1)).encode();
    let receivedBits = stripSpace(&bits.to_string());
    assert_eq!(
        s,
        decoder::highLevelDecode(&toBooleanArray(&bits)).expect("must decode")
    );
    // dbg!(s, decoder::highLevelDecode(&toBooleanArray(&bits)).expect("must decode"));
    assert_eq!(
        stripSpace(expectedBits),
        receivedBits,
        "highLevelEncode() failed for input string: {s}"
    );
}

fn testHighLevelEncodeString(s: &str, expectedBits: &str) {
    let bits = HighLevelEncoder::new(
        CharacterSet::ISO8859_1
            .encode(s)
            .expect("should encode to bytes"),
    )
    .encode()
    .expect("high level ok");
    // let bits =  HighLevelEncoder::new(s.getBytes(StandardCharsets.ISO_8859_1)).encode();
    let receivedBits = stripSpace(&bits.to_string());
    assert_eq!(
        stripSpace(expectedBits),
        receivedBits,
        "highLevelEncode() failed for input string: {s}"
    );
    assert_eq!(
        s,
        decoder::highLevelDecode(&toBooleanArray(&bits)).expect("must decode")
    );
}

fn testHighLevelEncodeStringCount(s: &str, expectedReceivedBits: u32) {
    let bits = HighLevelEncoder::new(
        CharacterSet::ISO8859_1
            .encode(s)
            .expect("should encode to bytes"),
    )
    .encode()
    .expect("high level ok");
    //let bits =  HighLevelEncoder::new(s.getBytes(StandardCharsets.ISO_8859_1)).encode().unwrap();
    let receivedBitCount = stripSpace(&bits.to_string()).len();
    // dbg!(s, decoder::highLevelDecode(&toBooleanArray(&bits)).expect("should decode"));
    assert_eq!(
        s,
        decoder::highLevelDecode(&toBooleanArray(&bits)).expect("should decode")
    );
    // assert!(
    //     expectedReceivedBits as usize >= receivedBitCount,
    //     "encode size too high ({} >= {}) failed for input string: {}",
    //     expectedReceivedBits, receivedBitCount,
    //     s
    // );
    assert_eq!(
        expectedReceivedBits as usize, receivedBitCount,
        "highLevelEncode() failed for input string: {s} with byte count ({expectedReceivedBits}!={receivedBitCount})"
    );
}
