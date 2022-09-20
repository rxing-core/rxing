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

// package com.google.zxing.aztec.encoder;

// import com.google.zxing.BarcodeFormat;
// import com.google.zxing.EncodeHintType;
// import com.google.zxing.FormatException;
// import com.google.zxing.RXingResultPoint;
// import com.google.zxing.aztec.AztecDetectorRXingResult;
// import com.google.zxing.aztec.AztecWriter;
// import com.google.zxing.aztec.decoder.Decoder;
// import com.google.zxing.common.BitArray;
// import com.google.zxing.common.BitMatrix;
// import com.google.zxing.common.DecoderRXingResult;
// import org.junit.Assert;
// import org.junit.Test;

// import java.nio.charset.Charset;
// import java.nio.charset.StandardCharsets;
// import java.util.EnumMap;
// import java.util.Map;
// import java.util.Random;
// import java.util.regex.Pattern;

use crate::common::BitArray;

/**
 * Aztec 2D generator unit tests.
 *
 * @author Rustam Abdullaev
 * @author Frank Yellin
 */

  const ISO_8859_1:&'static encoding::Encoding = StandardCharsets.ISO_8859_1;
  const UTF_8 :&'static encoding::Encoding= StandardCharsets.UTF_8;
  const SHIFT_JIS:&'static encoding::Encoding = Charset.forName("Shift_JIS");
  const ISO_8859_15:&'static encoding::Encoding = Charset.forName("ISO-8859-15");
  const WINDOWS_1252 :&'static encoding::Encoding= Charset.forName("Windows-1252");

  const DOTX : &str = "[^.X]";
  const SPACES :&str= "\\s+";
  const  NO_POINTS:Vec<RXingResultPoint> = Vec::new();

  // real life tests

  #[test]
  fn testEncode1() {
    testEncode("This is an example Aztec symbol for Wikipedia.", true, 3,
        "X     X X       X     X X     X     X         \n" +
        "X         X     X X     X   X X   X X       X \n" +
        "X X   X X X X X   X X X                 X     \n" +
        "X X                 X X   X       X X X X X X \n" +
        "    X X X   X   X     X X X X         X X     \n" +
        "  X X X   X X X X   X     X   X     X X   X   \n" +
        "        X X X X X     X X X X   X   X     X   \n" +
        "X       X   X X X X X X X X X X X     X   X X \n" +
        "X   X     X X X               X X X X   X X   \n" +
        "X     X X   X X   X X X X X   X X   X   X X X \n" +
        "X   X         X   X       X   X X X X       X \n" +
        "X       X     X   X   X   X   X   X X   X     \n" +
        "      X   X X X   X       X   X     X X X     \n" +
        "    X X X X X X   X X X X X   X X X X X X   X \n" +
        "  X X   X   X X               X X X   X X X X \n" +
        "  X   X       X X X X X X X X X X X X   X X   \n" +
        "  X X   X       X X X   X X X       X X       \n" +
        "  X               X   X X     X     X X X     \n" +
        "  X   X X X   X X   X   X X X X   X   X X X X \n" +
        "    X   X   X X X   X   X   X X X X     X     \n" +
        "        X               X                 X   \n" +
        "        X X     X   X X   X   X   X       X X \n" +
        "  X   X   X X       X   X         X X X     X \n");
  }

  #[test]
  fn testEncode2() {
    testEncode("Aztec Code is a public domain 2D matrix barcode symbology" +
                " of nominally square symbols built on a square grid with a " +
                "distinctive square bullseye pattern at their center.", false, 6,
          "        X X     X X     X     X     X   X X X         X   X         X   X X       \n" +
          "  X       X X     X   X X   X X       X             X     X   X X   X           X \n" +
          "  X   X X X     X   X   X X     X X X   X   X X               X X       X X     X \n" +
          "X X X             X   X         X         X     X     X   X     X X       X   X   \n" +
          "X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X \n" +
          "    X X   X   X   X X X               X       X       X X     X X   X X       X   \n" +
          "X X     X       X       X X X X   X   X X       X   X X   X       X X   X X   X   \n" +
          "  X       X   X     X X   X   X X   X X   X X X X X X   X X           X   X   X X \n" +
          "X X   X X   X   X X X X   X X X X X X X X   X   X       X X   X X X X   X X X     \n" +
          "  X       X   X     X       X X     X X   X   X   X     X X   X X X   X     X X X \n" +
          "  X   X X X   X X       X X X         X X           X   X   X   X X X   X X     X \n" +
          "    X     X   X X     X X X X     X   X     X X X X   X X   X X   X X X     X   X \n" +
          "X X X   X             X         X X X X X   X   X X   X   X   X X   X   X   X   X \n" +
          "          X       X X X   X X     X   X           X   X X X X   X X               \n" +
          "  X     X X   X   X       X X X X X X X X X X X X X X X   X   X X   X   X X X     \n" +
          "    X X                 X   X                       X X   X       X         X X X \n" +
          "        X   X X   X X X X X X   X X X X X X X X X   X     X X           X X X X   \n" +
          "          X X X   X     X   X   X               X   X X     X X X   X X           \n" +
          "X X     X     X   X   X   X X   X   X X X X X   X   X X X X X X X       X   X X X \n" +
          "X X X X       X       X   X X   X   X       X   X   X     X X X     X X       X X \n" +
          "X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X \n" +
          "    X     X       X         X   X   X       X   X   X     X   X X                 \n" +
          "        X X     X X X X X   X   X   X X X X X   X   X X X     X X X X   X         \n" +
          "X     X   X   X         X   X   X               X   X X   X X   X X X     X   X   \n" +
          "  X   X X X   X   X X   X X X   X X X X X X X X X   X X         X X     X X X X   \n" +
          "    X X   X   X   X X X     X                       X X X   X X   X   X     X     \n" +
          "    X X X X   X         X   X X X X X X X X X X X X X X   X       X X   X X   X X \n" +
          "            X   X   X X       X X X X X     X X X       X       X X X         X   \n" +
          "X       X         X   X X X X   X     X X     X X     X X           X   X       X \n" +
          "X     X       X X X X X     X   X X X X   X X X     X       X X X X   X   X X   X \n" +
          "  X X X X X               X     X X X   X       X X   X X   X X X X     X X       \n" +
          "X             X         X   X X   X X     X     X     X   X   X X X X             \n" +
          "    X   X X       X     X       X   X X X X X X   X X   X X X X X X X X X   X   X \n" +
          "    X         X X   X       X     X   X   X       X     X X X     X       X X X X \n" +
          "X     X X     X X X X X X             X X X   X               X   X     X     X X \n" +
          "X   X X     X               X X X X X     X X     X X X X X X X X     X   X   X X \n" +
          "X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X   X \n" +
          "X           X     X X X X     X     X         X         X   X       X X   X X X   \n" +
          "X   X   X X   X X X   X         X X     X X X X     X X   X   X     X   X       X \n" +
          "      X     X     X     X X     X   X X   X X   X         X X       X       X   X \n" +
          "X       X           X   X   X     X X   X               X     X     X X X         \n");
  }

  
  fn testAztecWriter()  {
    testWriter("Espa\u{00F1}ol", null, 25, true, 1);                   // Without ECI (implicit ISO-8859-1)
    testWriter("Espa\u{00F1}ol", ISO_8859_1, 25, true, 1);             // Explicit ISO-8859-1
    testWriter("\u{20AC} 1 sample data.", WINDOWS_1252, 25, true, 2);  // ISO-8859-1 can't encode Euro; Windows-1252 can
    testWriter("\u{20AC} 1 sample data.", ISO_8859_15, 25, true, 2);
    testWriter("\u{20AC} 1 sample data.", UTF_8, 25, true, 2);
    testWriter("\u{20AC} 1 sample data.", UTF_8, 100, true, 3);
    testWriter("\u{20AC} 1 sample data.", UTF_8, 300, true, 4);
    testWriter("\u{20AC} 1 sample data.", UTF_8, 500, false, 5);
    testWriter("The capital of Japan is named \u{6771}\u{4EAC}.", SHIFT_JIS, 25, true, 3);
    // Test AztecWriter defaults
    let data = "In ut magna vel mauris malesuada";
    let writer =  AztecWriter::new();
    let matrix = writer.encode(data, BarcodeFormat.AZTEC, 0, 0);
    let aztec = Encoder.encode(data,
        Encoder.DEFAULT_EC_PERCENT, Encoder.DEFAULT_AZTEC_LAYERS);
    let expectedMatrix = aztec.getMatrix();
    assertEquals(matrix, expectedMatrix);
  }

  // synthetic tests (encode-decode round-trip)

  #[test]
  fn testEncodeDecode1()  {
    testEncodeDecode("Abc123!", true, 1);
  }

  #[test]
  fn testEncodeDecode2() {
    testEncodeDecode("Lorem ipsum. http://test/", true, 2);
  }

  #[test]
  fn testEncodeDecode3() {
    testEncodeDecode("AAAANAAAANAAAANAAAANAAAANAAAANAAAANAAAANAAAANAAAAN", true, 3);
  }

  #[test]
  fn  testEncodeDecode4()  {
    testEncodeDecode("http://test/~!@#*^%&)__ ;:'\"[]{}\\|-+-=`1029384", true, 4);
  }

  #[test]
  fn  testEncodeDecode5()  {
    testEncodeDecode("http://test/~!@#*^%&)__ ;:'\"[]{}\\|-+-=`1029384756<>/?abc"
        + "Four score and seven our forefathers brought forth", false, 5);
  }

  #[test]
  fn testEncodeDecode10()  {
    testEncodeDecode("In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus quis diam" +
        " cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec laoreet rutrum" +
        " est, nec convallis mauris condimentum sit amet. Phasellus gravida, justo et congue" +
        " auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec lorem. Nulla" +
        " ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar nisi, id" +
        " elementum sapien dolor et diam.", false, 10);
  }

  #[test]
  fn  testEncodeDecode23()  {
    testEncodeDecode("In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus quis diam" +
        " cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec laoreet rutrum" +
        " est, nec convallis mauris condimentum sit amet. Phasellus gravida, justo et congue" +
        " auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec lorem. Nulla" +
        " ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar nisi, id" +
        " elementum sapien dolor et diam. Donec ac nunc sodales elit placerat eleifend." +
        " Sed ornare luctus ornare. Vestibulum vehicula, massa at pharetra fringilla, risus" +
        " justo faucibus erat, nec porttitor nibh tellus sed est. Ut justo diam, lobortis eu" +
        " tristique ac, p.In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus" +
        " quis diam cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec" +
        " laoreet rutrum est, nec convallis mauris condimentum sit amet. Phasellus gravida," +
        " justo et congue auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec" +
        " lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar" +
        " nisi, id elementum sapien dolor et diam. Donec ac nunc sodales elit placerat" +
        " eleifend. Sed ornare luctus ornare. Vestibulum vehicula, massa at pharetra" +
        " fringilla, risus justo faucibus erat, nec porttitor nibh tellus sed est. Ut justo" +
        " diam, lobortis eu tristique ac, p. In ut magna vel mauris malesuada dictum. Nulla" +
        " ullamcorper metus quis diam cursus facilisis. Sed mollis quam id justo rutrum" +
        " sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum sit amet." +
        " Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat, eget hendrerit" +
        " felis turpis nec lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo" +
        " erat pulvinar nisi, id elementum sapien dolor et diam.", false, 23);
  }

  #[test]
  fn testEncodeDecode31()  {
    testEncodeDecode("In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus quis diam" +
        " cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec laoreet rutrum" +
        " est, nec convallis mauris condimentum sit amet. Phasellus gravida, justo et congue" +
        " auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec lorem. Nulla" +
        " ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar nisi, id" +
        " elementum sapien dolor et diam. Donec ac nunc sodales elit placerat eleifend." +
        " Sed ornare luctus ornare. Vestibulum vehicula, massa at pharetra fringilla, risus" +
        " justo faucibus erat, nec porttitor nibh tellus sed est. Ut justo diam, lobortis eu" +
        " tristique ac, p.In ut magna vel mauris malesuada dictum. Nulla ullamcorper metus" +
        " quis diam cursus facilisis. Sed mollis quam id justo rutrum sagittis. Donec" +
        " laoreet rutrum est, nec convallis mauris condimentum sit amet. Phasellus gravida," +
        " justo et congue auctor, nisi ipsum viverra erat, eget hendrerit felis turpis nec" +
        " lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo erat pulvinar" +
        " nisi, id elementum sapien dolor et diam. Donec ac nunc sodales elit placerat" +
        " eleifend. Sed ornare luctus ornare. Vestibulum vehicula, massa at pharetra" +
        " fringilla, risus justo faucibus erat, nec porttitor nibh tellus sed est. Ut justo" +
        " diam, lobortis eu tristique ac, p. In ut magna vel mauris malesuada dictum. Nulla" +
        " ullamcorper metus quis diam cursus facilisis. Sed mollis quam id justo rutrum" +
        " sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum sit amet." +
        " Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat, eget hendrerit" +
        " felis turpis nec lorem. Nulla ultrices, elit pellentesque aliquet laoreet, justo" +
        " erat pulvinar nisi, id elementum sapien dolor et diam. Donec ac nunc sodales elit" +
        " placerat eleifend. Sed ornare luctus ornare. Vestibulum vehicula, massa at" +
        " pharetra fringilla, risus justo faucibus erat, nec porttitor nibh tellus sed est." +
        " Ut justo diam, lobortis eu tristique ac, p.In ut magna vel mauris malesuada" +
        " dictum. Nulla ullamcorper metus quis diam cursus facilisis. Sed mollis quam id" +
        " justo rutrum sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum" +
        " sit amet. Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat," +
        " eget hendrerit felis turpis nec lorem. Nulla ultrices, elit pellentesque aliquet" +
        " laoreet, justo erat pulvinar nisi, id elementum sapien dolor et diam. Donec ac" +
        " nunc sodales elit placerat eleifend. Sed ornare luctus ornare. Vestibulum vehicula," +
        " massa at pharetra fringilla, risus justo faucibus erat, nec porttitor nibh tellus" +
        " sed est. Ut justo diam, lobortis eu tris. In ut magna vel mauris malesuada dictum." +
        " Nulla ullamcorper metus quis diam cursus facilisis. Sed mollis quam id justo rutrum" +
        " sagittis. Donec laoreet rutrum est, nec convallis mauris condimentum sit amet." +
        " Phasellus gravida, justo et congue auctor, nisi ipsum viverra erat, eget" +
        " hendrerit felis turpis nec lorem.", false, 31);
  }

  #[test]
  fn  testGenerateModeMessage() {
    testModeMessage(true, 2, 29, ".X .XXX.. ...X XX.. ..X .XX. .XX.X");
    testModeMessage(true, 4, 64, "XX XXXXXX .X.. ...X ..XX .X.. XX..");
    testModeMessage(false, 21, 660,  "X.X.. .X.X..X..XX .XXX ..X.. .XXX. .X... ..XXX");
    testModeMessage(false, 32, 4096, "XXXXX XXXXXXXXXXX X.X. ..... XXX.X ..X.. X.XXX");
  }

  #[test]
  fn  testStuffBits() {
    testStuffBits(5, ".X.X. X.X.X .X.X.",
        ".X.X. X.X.X .X.X.");
    testStuffBits(5, ".X.X. ..... .X.X",
        ".X.X. ....X ..X.X");
    testStuffBits(3, "XX. ... ... ..X XXX .X. ..",
        "XX. ..X ..X ..X ..X .XX XX. .X. ..X");
    testStuffBits(6, ".X.X.. ...... ..X.XX",
        ".X.X.. .....X. ..X.XX XXXX.");
    testStuffBits(6, ".X.X.. ...... ...... ..X.X.",
        ".X.X.. .....X .....X ....X. X.XXXX");
    testStuffBits(6, ".X.X.. XXXXXX ...... ..X.XX",
        ".X.X.. XXXXX. X..... ...X.X XXXXX.");
    testStuffBits(6,
        "...... ..XXXX X..XX. .X.... .X.X.X .....X .X.... ...X.X .....X ....XX ..X... ....X. X..XXX X.XX.X",
        ".....X ...XXX XX..XX ..X... ..X.X. X..... X.X... ....X. X..... X....X X..X.. .....X X.X..X XXX.XX .XXXXX");
  }

  #[test]
  fn  testHighLevelEncode()  {
    testHighLevelEncodeString("A. b.",
        // 'A'  P/S   '. ' L/L    b    D/L    '.'
        "...X. ..... ...XX XXX.. ...XX XXXX. XX.X");
    testHighLevelEncodeString("Lorem ipsum.",
        // 'L'  L/L   'o'   'r'   'e'   'm'   ' '   'i'   'p'   's'   'u'   'm'   D/L   '.'
        ".XX.X XXX.. X.... X..XX ..XX. .XXX. ....X .X.X. X...X X.X.. X.XX. .XXX. XXXX. XX.X");
    testHighLevelEncodeString("Lo. Test 123.",
        // 'L'  L/L   'o'   P/S   '. '  U/S   'T'   'e'   's'   't'    D/L   ' '  '1'  '2'  '3'  '.'
        ".XX.X XXX.. X.... ..... ...XX XXX.. X.X.X ..XX. X.X.. X.X.X  XXXX. ...X ..XX .X.. .X.X XX.X");
    testHighLevelEncodeString("Lo...x",
        // 'L'  L/L   'o'   D/L   '.'  '.'  '.'  U/L  L/L   'x'
        ".XX.X XXX.. X.... XXXX. XX.X XX.X XX.X XXX. XXX.. XX..X");
    testHighLevelEncodeString(". x://abc/.",
        //P/S   '. '  L/L   'x'   P/S   ':'   P/S   '/'   P/S   '/'   'a'   'b'   'c'   P/S   '/'   D/L   '.'
        "..... ...XX XXX.. XX..X ..... X.X.X ..... X.X.. ..... X.X.. ...X. ...XX ..X.. ..... X.X.. XXXX. XX.X");
    // Uses Binary/Shift rather than Lower/Shift to save two bits.
    testHighLevelEncodeString("ABCdEFG",
        //'A'   'B'   'C'   B/S    =1    'd'     'E'   'F'   'G'
        "...X. ...XX ..X.. XXXXX ....X .XX..X.. ..XX. ..XXX .X...");

    testHighLevelEncodeString(
        // Found on an airline boarding pass.  Several stretches of Binary shift are
        // necessary to keep the bitcount so low.
        "09  UAG    ^160MEUCIQC0sYS/HpKxnBELR1uB85R20OoqqwFGa0q2uEi"
            + "Ygh6utAIgLl1aBVM4EOTQtMQQYH9M2Z3Dp4qnA/fwWuQ+M8L3V8U=",
        823);
  }

  #[test]
  fn  testHighLevelEncodeBinary() {
    // binary short form single byte
    testHighLevelEncodeString("N\0N",
        // 'N'  B/S    =1   '\0'      N
        ".XXXX XXXXX ....X ........ .XXXX");   // Encode "N" in UPPER

    testHighLevelEncodeString("N\0n",
        // 'N'  B/S    =2   '\0'       'n'
        ".XXXX XXXXX ...X. ........ .XX.XXX.");   // Encode "n" in BINARY

    // binary short form consecutive bytes
    testHighLevelEncodeString("N\0\u{0080} A",
        // 'N'  B/S    =2    '\0'    \u0080   ' '  'A'
        ".XXXX XXXXX ...X. ........ X....... ....X ...X.");

    // binary skipping over single character
    testHighLevelEncodeString("\0a\u{00FF}\u{0080} A",
        // B/S  =4    '\0'      'a'     '\3ff'   '\200'   ' '   'A'
        "XXXXX ..X.. ........ .XX....X XXXXXXXX X....... ....X ...X.");

    // getting into binary mode from digit mode
    testHighLevelEncodeString("1234\0",
        //D/L   '1'  '2'  '3'  '4'  U/L  B/S    =1    \0
        "XXXX. ..XX .X.. .X.X .XX. XXX. XXXXX ....X ........"
    );

    // Create a string in which every character requires binary
    let sb = String::new();
    for (int i = 0; i <= 3000; i++) {
      sb.append((char) (128 + (i % 30)));
    }
    // Test the output generated by Binary/Switch, particularly near the
    // places where the encoding changes: 31, 62, and 2047+31=2078
    for (int i : new int[] { 1, 2, 3, 10, 29, 30, 31, 32, 33,
                             60, 61, 62, 63, 64, 2076, 2077, 2078, 2079, 2080, 2100 }) {
      // This is the expected length of a binary string of length "i"
      int expectedLength = (8 * i) +
          ((i <= 31) ? 10 : (i <= 62) ? 20 : (i <= 2078) ? 21 : 31);
      // Verify that we are correct about the length.
      testHighLevelEncodeString(sb.substring(0, i), expectedLength);
      if (i != 1 && i != 32 && i != 2079) {
        // The addition of an 'a' at the beginning or end gets merged into the binary code
        // in those cases where adding another binary character only adds 8 or 9 bits to the result.
        // So we exclude the border cases i=1,32,2079
        // A lower case letter at the beginning will be merged into binary mode
        testHighLevelEncodeString('a' + sb.substring(0, i - 1), expectedLength);
        // A lower case letter at the end will also be merged into binary mode
        testHighLevelEncodeString(sb.substring(0, i - 1) + 'a', expectedLength);
      }
      // A lower case letter at both ends will enough to latch us into LOWER.
      testHighLevelEncodeString('a' + sb.substring(0, i) + 'b', expectedLength + 15);
    }

    sb = new StringBuilder();
    for (int i = 0; i < 32; i++) {
      sb.append('§'); // § forces binary encoding
    }
    sb.setCharAt(1, 'A');
    // expect B/S(1) A B/S(30)
    testHighLevelEncodeString(sb.toString(), 5 + 20 + 31 * 8);

    sb = new StringBuilder();
    for (int i = 0; i < 31; i++) {
      sb.append('§');
    }
    sb.setCharAt(1, 'A');
    // expect B/S(31)
    testHighLevelEncodeString(sb.toString(), 10 + 31 * 8);

    sb = new StringBuilder();
    for (int i = 0; i < 34; i++) {
      sb.append('§');
    }
    sb.setCharAt(1, 'A');
    // expect B/S(31) B/S(3)
    testHighLevelEncodeString(sb.toString(), 20 + 34 * 8);

    sb = new StringBuilder();
    for (int i = 0; i < 64; i++) {
      sb.append('§');
    }
    sb.setCharAt(30, 'A');
    // expect B/S(64)
    testHighLevelEncodeString(sb.toString(), 21 + 64 * 8);
  }

  #[test]
  fn testHighLevelEncodePairs()  {
    // Typical usage
    testHighLevelEncodeString("ABC. DEF\r\n",
        //  A     B    C    P/S   .<sp>   D    E     F    P/S   \r\n
        "...X. ...XX ..X.. ..... ...XX ..X.X ..XX. ..XXX ..... ...X.");

    // We should latch to PUNCT mode, rather than shift.  Also check all pairs
    testHighLevelEncodeString("A. : , \r\n",
        // 'A'    M/L   P/L   ". "  ": "   ", " "\r\n"
        "...X. XXX.X XXXX. ...XX ..X.X  ..X.. ...X.");

    // Latch to DIGIT rather than shift to PUNCT
    testHighLevelEncodeString("A. 1234",
        // 'A'  D/L   '.'  ' '  '1' '2'   '3'  '4'
        "...X. XXXX. XX.X ...X ..XX .X.. .X.X .X X.");
    // Don't bother leaving Binary Shift.
    testHighLevelEncodeString("A\200. \200",
        // 'A'  B/S    =2    \200      "."     " "     \200
        "...X. XXXXX ..X.. X....... ..X.XXX. ..X..... X.......");
  }

  #[test]
  #[should_panic]
  fn  testUserSpecifiedLayers() {
    doTestUserSpecifiedLayers(33);
  }

  #[test]
  #[should_panic]
  fn  testUserSpecifiedLayers2() {
    doTestUserSpecifiedLayers(-1);
  }

  fn doTestUserSpecifiedLayers( userSpecifiedLayers:usize) {
    String alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    AztecCode aztec = Encoder.encode(alphabet, 25, -2);
    assertEquals(2, aztec.getLayers());
    assertTrue(aztec.isCompact());

    aztec = Encoder.encode(alphabet, 25, 32);
    assertEquals(32, aztec.getLayers());
    assertFalse(aztec.isCompact());

    Encoder.encode(alphabet, 25, userSpecifiedLayers);
  }

  #[test]
  #[should_panic]
  fn  testBorderCompact4CaseFailed() {
    // Compact(4) con hold 608 bits of information, but at most 504 can be data.  Rest must
    // be error correction
    String alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // encodes as 26 * 5 * 4 = 520 bits of data
    String alphabet4 = alphabet + alphabet + alphabet + alphabet;
    Encoder.encode(alphabet4, 0, -4);
  }

  #[test]
  fn  testBorderCompact4Case() {
    // Compact(4) con hold 608 bits of information, but at most 504 can be data.  Rest must
    // be error correction
    String alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    // encodes as 26 * 5 * 4 = 520 bits of data
    String alphabet4 = alphabet + alphabet + alphabet + alphabet;

    // If we just try to encode it normally, it will go to a non-compact 4 layer
    AztecCode aztecCode = Encoder.encode(alphabet4, 0, Encoder.DEFAULT_AZTEC_LAYERS);
    assertFalse(aztecCode.isCompact());
    assertEquals(4, aztecCode.getLayers());

    // But shortening the string to 100 bytes (500 bits of data), compact works fine, even if we
    // include more error checking.
    aztecCode = Encoder.encode(alphabet4.substring(0, 100), 10, Encoder.DEFAULT_AZTEC_LAYERS);
    assertTrue(aztecCode.isCompact());
    assertEquals(4, aztecCode.getLayers());
  }

  // Helper routines

  fn testEncode( data:&str,  compact:bool,  layers:usize,  expected:&str) {
    AztecCode aztec = Encoder.encode(data, 33, Encoder.DEFAULT_AZTEC_LAYERS);
    assertEquals("Unexpected symbol format (compact)", compact, aztec.isCompact());
    assertEquals("Unexpected nr. of layers", layers, aztec.getLayers());
    BitMatrix matrix = aztec.getMatrix();
    assertEquals("encode() failed", expected, matrix.toString());
  }

  fn testEncodeDecode( data:&str,  compact:bool,  layers:usize) {
    AztecCode aztec = Encoder.encode(data, 25, Encoder.DEFAULT_AZTEC_LAYERS);
    assertEquals("Unexpected symbol format (compact)", compact, aztec.isCompact());
    assertEquals("Unexpected nr. of layers", layers, aztec.getLayers());
    BitMatrix matrix = aztec.getMatrix();
    AztecDetectorRXingResult r =
        new AztecDetectorRXingResult(matrix, NO_POINTS, aztec.isCompact(), aztec.getCodeWords(), aztec.getLayers());
    DecoderRXingResult res = new Decoder().decode(r);
    assertEquals(data, res.getText());
    // Check error correction by introducing a few minor errors
    Random random = getPseudoRandom();
    matrix.flip(random.nextInt(matrix.getWidth()), random.nextInt(2));
    matrix.flip(random.nextInt(matrix.getWidth()), matrix.getHeight() - 2 + random.nextInt(2));
    matrix.flip(random.nextInt(2), random.nextInt(matrix.getHeight()));
    matrix.flip(matrix.getWidth() - 2 + random.nextInt(2), random.nextInt(matrix.getHeight()));
    r = new AztecDetectorRXingResult(matrix, NO_POINTS, aztec.isCompact(), aztec.getCodeWords(), aztec.getLayers());
    res = new Decoder().decode(r);
    assertEquals(data, res.getText());
  }

  fn testWriter( data:&str,
                                  charset:&'static dyn encoding::Encoding,
                                  eccPercent:u32,
                                  compact:bool,
                                  layers:usize) throws FormatException {
    // Perform an encode-decode round-trip because it can be lossy.
    Map<EncodeHintType,Object> hints = new EnumMap<>(EncodeHintType.class);
    if (null != charset) {
      hints.put(EncodeHintType.CHARACTER_SET, charset.name());
    }
    hints.put(EncodeHintType.ERROR_CORRECTION, eccPercent);
    AztecWriter writer = new AztecWriter();
    BitMatrix matrix = writer.encode(data, BarcodeFormat.AZTEC, 0, 0, hints);
    AztecCode aztec = Encoder.encode(data, eccPercent,
        Encoder.DEFAULT_AZTEC_LAYERS, charset);
    assertEquals("Unexpected symbol format (compact)", compact, aztec.isCompact());
    assertEquals("Unexpected nr. of layers", layers, aztec.getLayers());
    BitMatrix matrix2 = aztec.getMatrix();
    assertEquals(matrix, matrix2);
    AztecDetectorRXingResult r =
        new AztecDetectorRXingResult(matrix, NO_POINTS, aztec.isCompact(), aztec.getCodeWords(), aztec.getLayers());
    DecoderRXingResult res = new Decoder().decode(r);
    assertEquals(data, res.getText());
    // Check error correction by introducing up to eccPercent/2 errors
    int ecWords = aztec.getCodeWords() * eccPercent / 100 / 2;
    Random random = getPseudoRandom();
    for (int i = 0; i < ecWords; i++) {
      // don't touch the core
      int x = random.nextBoolean() ?
                random.nextInt(aztec.getLayers() * 2)
                : matrix.getWidth() - 1 - random.nextInt(aztec.getLayers() * 2);
      int y = random.nextBoolean() ?
                random.nextInt(aztec.getLayers() * 2)
                : matrix.getHeight() - 1 - random.nextInt(aztec.getLayers() * 2);
      matrix.flip(x, y);
    }
    r = new AztecDetectorRXingResult(matrix, NO_POINTS, aztec.isCompact(), aztec.getCodeWords(), aztec.getLayers());
    res = new Decoder().decode(r);
    assertEquals(data, res.getText());
  }

  fn getPseudoRandom() -> rand::Rng{
    return new Random(0xDEADBEEF);
  }

  fn testModeMessage( compact:bool,  layers:usize,  words:usize,  expected:&str) {
    BitArray in = Encoder.generateModeMessage(compact, layers, words);
    assertEquals("generateModeMessage() failed", stripSpace(expected), stripSpace(in.toString()));
  }

  fn testStuffBits( wordSize:usize,  bits:&str,  expected:&str) {
    BitArray in = toBitArray(bits);
    BitArray stuffed = Encoder.stuffBits(in, wordSize);
    assertEquals("stuffBits() failed for input string: " + bits,
                 stripSpace(expected), stripSpace(stuffed.toString()));
  }

  

  fn testHighLevelEncodeString( s:&str,  expectedBits:&str)  {
    BitArray bits = new HighLevelEncoder(s.getBytes(StandardCharsets.ISO_8859_1)).encode();
    String receivedBits = stripSpace(bits.toString());
    assertEquals("highLevelEncode() failed for input string: " + s, stripSpace(expectedBits), receivedBits);
    assertEquals(s, Decoder.highLevelDecode(toBooleanArray(bits)));
  }

  fn testHighLevelEncodeString( s:&str,  expectedReceivedBits:u32) {
    BitArray bits = new HighLevelEncoder(s.getBytes(StandardCharsets.ISO_8859_1)).encode();
    int receivedBitCount = stripSpace(bits.toString()).length();
    assertEquals("highLevelEncode() failed for input string: " + s,
                 expectedReceivedBits, receivedBitCount);
    assertEquals(s, Decoder.highLevelDecode(toBooleanArray(bits)));
  }
