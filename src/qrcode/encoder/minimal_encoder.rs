/*
 * Copyright 2021 ZXing authors
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

use std::{fmt, rc::Rc};

use encoding::EncodingRef;

use crate::{common::{ECIEncoderSet, BitArray}, qrcode::decoder::{ErrorCorrectionLevel, Version, Mode, VersionRef}, Exceptions};

use super::encoder;


enum VersionSize {
  SMALL,//("version 1-9"),
  MEDIUM,//("version 10-26"),
  LARGE,//("version 27-40");

  // private final String description;

  // VersionSize(String description) {
  //   this.description = description;
  // }

  // public String toString() {
  //   return description;
  // }
}

impl fmt::Display for VersionSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", match self {
            VersionSize::SMALL => "version 1-9",
            VersionSize::MEDIUM => "version 10-26",
            VersionSize::LARGE => "version 27-40",
        })
    }
}


/**
 * Encoder that encodes minimally
 *
 * Algorithm:
 *
 * The eleventh commandment was "Thou Shalt Compute" or "Thou Shalt Not Compute" - I forget which (Alan Perilis).
 *
 * This implementation computes. As an alternative, the QR-Code specification suggests heuristics like this one:
 *
 * If initial input data is in the exclusive subset of the Alphanumeric character set AND if there are less than
 * [6,7,8] characters followed by data from the remainder of the 8-bit byte character set, THEN select the 8-
 * bit byte mode ELSE select Alphanumeric mode;
 *
 * This is probably right for 99.99% of cases but there is at least this one counter example: The string "AAAAAAa"
 * encodes 2 bits smaller as ALPHANUMERIC(AAAAAA), BYTE(a) than by encoding it as BYTE(AAAAAAa).
 * Perhaps that is the only counter example but without having proof, it remains unclear.
 *
 * ECI switching:
 *
 * In multi language content the algorithm selects the most compact representation using ECI modes.
 * For example the most compact representation of the string "\u0150\u015C" (O-double-acute, S-circumflex) is
 * ECI(UTF-8), BYTE(\u0150\u015C) while prepending one or more times the same leading character as in
 * "\u0150\u0150\u015C", the most compact representation uses two ECIs so that the string is encoded as
 * ECI(ISO-8859-2), BYTE(\u0150\u0150), ECI(ISO-8859-3), BYTE(\u015C).
 *
 * @author Alex Geller
 */
pub struct MinimalEncoder<'a> {

  stringToEncode: &'a str,
   isGS1: bool,
   encoders: ECIEncoderSet,
   ecLevel:ErrorCorrectionLevel,
}

impl MinimalEncoder<'_> {

  /**
   * Creates a MinimalEncoder
   *
   * @param stringToEncode The string to encode
   * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
   *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
   *   charset to encode any character in the input that can be encoded by it if the charset is among the
   *   supported charsets.
   * @param isGS1 {@code true} if a FNC1 is to be prepended; {@code false} otherwise
   * @param ecLevel The error correction level.
   * @see RXingResultList#getVersion
   */
  pub fn new( stringToEncode:&str,  priorityCharset: EncodingRef,  isGS1:bool,  ecLevel:ErrorCorrectionLevel) -> Self {
    Self {
        stringToEncode,
        isGS1,
        encoders: ECIEncoderSet::new(&stringToEncode, priorityCharset, -1),
        ecLevel,
    }
    
    // let encoders =  ECIEncoderSet::new(&stringToEncode, priorityCharset, -1);

  }

  /**
   * Encodes the string minimally
   *
   * @param stringToEncode The string to encode
   * @param version The preferred {@link Version}. A minimal version is computed (see
   *   {@link RXingResultList#getVersion method} when the value of the argument is null
   * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
   *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
   *   charset to encode any character in the input that can be encoded by it if the charset is among the
   *   supported charsets.
   * @param isGS1 {@code true} if a FNC1 is to be prepended; {@code false} otherwise
   * @param ecLevel The error correction level.
   * @return An instance of {@code RXingResultList} representing the minimal solution.
   * @see RXingResultList#getBits
   * @see RXingResultList#getVersion
   * @see RXingResultList#getSize
   */
  pub fn encode_with_details<'a>( stringToEncode:&str,  version:Option<VersionRef>, priorityCharset:EncodingRef,  isGS1:bool,
       ecLevel:ErrorCorrectionLevel) -> Result<RXingResultList<'a>,Exceptions> {
     MinimalEncoder::new(stringToEncode, priorityCharset, isGS1, ecLevel).encode(version)
  }

  pub fn encode(&self,  version:Option<VersionRef>) -> Result<RXingResultList,Exceptions> {
    if version.is_none() { // compute minimal encoding trying the three version sizes.
      let versions = [ Self::getVersion(VersionSize::SMALL),
      Self::getVersion(VersionSize::MEDIUM),
      Self::getVersion(VersionSize::LARGE) ];
      let results = [ self.encodeSpecificVersion(&versions[0])?,
                               self.encodeSpecificVersion(&versions[1])?,
                               self.encodeSpecificVersion(&versions[2])? ];
      let smallestSize = u32::MAX;
      let smallestRXingResult:i32 = -1;
      for i in 0..3 {
      // for (int i = 0; i < 3; i++) {
        let size = results[i].getSize();
        if encoder::willFit(size, versions[i], &self.ecLevel) && size < smallestSize {
          smallestSize = size;
          smallestRXingResult = i as i32;
        }
      }
      if smallestRXingResult < 0 {
        return Err(Exceptions::WriterException("Data too big for any version".to_owned()));
      }
      Ok(results[smallestRXingResult as usize])
    } else { // compute minimal encoding for a given version
      let version = version.unwrap();
      let result = self.encodeSpecificVersion(version)?;
      if !encoder::willFit(result.getSize(), Self::getVersion(Self::getVersionSize(result.getVersion())), &self.ecLevel) {
        return Err(Exceptions::WriterException(format!("Data too big for version {}" , version)));
      }
      Ok(result)
    }
  }

  pub fn getVersionSize( version:&Version) -> VersionSize {
    return if version.getVersionNumber() <= 9 { VersionSize::SMALL} else {if version.getVersionNumber() <= 26 
      {VersionSize::MEDIUM} else {VersionSize::LARGE}};
  }

  pub fn  getVersion( versionSize:VersionSize)->VersionRef {
    match versionSize {
        VersionSize::SMALL => Version::getVersionForNumber(9).expect("should always exist"),
        VersionSize::MEDIUM => Version::getVersionForNumber(26).expect("should always exist"),
        VersionSize::LARGE => Version::getVersionForNumber(40).expect("should always exist"),
    }
    // switch (versionSize) {
    //   case SMALL:
    //     return Version.getVersionForNumber(9);
    //   case MEDIUM:
    //     return Version.getVersionForNumber(26);
    //   case LARGE:
    //   default:
    //     return Version.getVersionForNumber(40);
    // }
  }

  pub fn isNumeric( c:char) -> bool{
    return c >= '0' && c <= '9';
  }

  pub fn isDoubleByteKanji( c:char) -> bool{
    return encoder::isOnlyDoubleByteKanji(&String::from(c));
  }

  pub fn isAlphanumeric( c: char) -> bool{
    return encoder::getAlphanumericCode(c as u8 as u32) != -1;
  }

  pub fn canEncode(&self,  mode:&Mode,  c:char) -> bool {
    match mode {
        Mode::NUMERIC => Self::isNumeric(c),
        Mode::ALPHANUMERIC => Self::isAlphanumeric(c),
        Mode::STRUCTURED_APPEND => todo!(),
        Mode::BYTE => true,
        Mode::KANJI => Self::isDoubleByteKanji(c),
        _=> false,// any character can be encoded as byte(s). Up to the caller to manage splitting into
        // multiple bytes when String.getBytes(Charset) return more than one byte.
    }
  }

  pub fn getCompactedOrdinal( mode:Option<Mode>) -> Result<u32,Exceptions>{
    if mode.is_none() {
      return Ok(0);
    }
    match &mode.unwrap() {
        Mode::NUMERIC => Ok(2),
        Mode::ALPHANUMERIC => Ok(1),
        Mode::BYTE => Ok(3),
        Mode::KANJI => Ok(0),
        _=> Err(Exceptions::IllegalArgumentException(format!("Illegal mode {:?}", mode))),
    }
    // switch (mode) {
    //   case KANJI:
    //     return 0;
    //   case ALPHANUMERIC:
    //     return 1;
    //   case NUMERIC:
    //     return 2;
    //   case BYTE:
    //     return 3;
    //   default:
    //     throw new IllegalStateException("Illegal mode " + mode);
    // }
  }

  pub fn addEdge(&self,  edges:&Vec<Vec<Vec<Option<&'_ Edge>>>>,  position:usize,  edge:Option<&Edge>) {
    let vertexIndex = position + edge.as_ref().unwrap().characterLength as usize;
    let modeEdges = edges[vertexIndex as usize][edge.as_ref().unwrap().charsetEncoderIndex as usize];
    let modeOrdinal = Self::getCompactedOrdinal(Some(edge.as_ref().unwrap().mode)).expect("value") as usize;
    if modeEdges[modeOrdinal].is_none() || modeEdges[modeOrdinal].as_ref().unwrap().cachedTotalSize > (&edge).unwrap().cachedTotalSize {
      modeEdges[modeOrdinal] = edge;
    }
  }

  pub fn addEdges( &self, version:VersionRef, edges:&Vec<Vec<Vec<Option<&Edge>>>>,  from:usize,  previous:Option<&Edge>) {
    let start = 0;
    let end = self.encoders.len();
    let priorityEncoderIndex = self.encoders.getPriorityEncoderIndex();
    if priorityEncoderIndex >= 0 && self.encoders.canEncode(self.stringToEncode.chars().nth(from as usize).unwrap() as i16,priorityEncoderIndex) {
      start = priorityEncoderIndex;
      end = priorityEncoderIndex + 1;
    }

    for i in start..end {
    // for (int i = start; i < end; i++) {
      if self.encoders.canEncode(self.stringToEncode.chars().nth(from).unwrap() as i16, i) {
        self.addEdge(edges, from,  Some(&Edge::new(Mode::BYTE, from, i, 1, previous, version,&self.encoders,&self.stringToEncode)));
      }
    }

    if self.canEncode(&Mode::KANJI, self.stringToEncode.chars().nth(from).unwrap()) {
      self.addEdge(edges, from,  Some(&Edge::new(Mode::KANJI, from, 0, 1, previous, version,&self.encoders,&self.stringToEncode)));
    }

    let inputLength = self.stringToEncode.len();
    if self.canEncode(&Mode::ALPHANUMERIC, self.stringToEncode.chars().nth(from).unwrap()) {
      self.addEdge(edges, from,  Some(&Edge::new(Mode::ALPHANUMERIC, from, 0, if from + 1 >= inputLength ||
          !self.canEncode(&Mode::ALPHANUMERIC, self.stringToEncode.chars().nth(from + 1).unwrap())  {1} else {2}, previous, version,&self.encoders,&self.stringToEncode)));
    }

    if self.canEncode(&Mode::NUMERIC, self.stringToEncode.chars().nth(from).unwrap()) {
      self.addEdge(edges, from,  Some(&Edge::new(Mode::NUMERIC, from, 0, if from + 1 >= inputLength ||
          !self.canEncode(&Mode::NUMERIC, self.stringToEncode.chars().nth(from + 1).unwrap())  {1} else {if from + 2 >= inputLength ||
          !self.canEncode(&Mode::NUMERIC, self.stringToEncode.chars().nth(from + 2).unwrap())  {2} else {3}}, previous, version,&self.encoders,&self.stringToEncode)));
    }
  }
  pub fn encodeSpecificVersion(&self,  version:VersionRef) ->Result< RXingResultList, Exceptions >{

    // @SuppressWarnings("checkstyle:lineLength")
    /* A vertex represents a tuple of a position in the input, a mode and a character encoding where position 0
     * denotes the position left of the first character, 1 the position left of the second character and so on.
     * Likewise the end vertices are located after the last character at position stringToEncode.length().
     *
     * An edge leading to such a vertex encodes one or more of the characters left of the position that the vertex
     * represents and encodes it in the same encoding and mode as the vertex on which the edge ends. In other words,
     * all edges leading to a particular vertex encode the same characters in the same mode with the same character
     * encoding. They differ only by their source vertices who are all located at i+1 minus the number of encoded
     * characters.
     *
     * The edges leading to a vertex are stored in such a way that there is a fast way to enumerate the edges ending
     * on a particular vertex.
     *
     * The algorithm processes the vertices in order of their position thereby performing the following:
     *
     * For every vertex at position i the algorithm enumerates the edges ending on the vertex and removes all but the
     * shortest from that list.
     * Then it processes the vertices for the position i+1. If i+1 == stringToEncode.length() then the algorithm ends
     * and chooses the the edge with the smallest size from any of the edges leading to vertices at this position.
     * Otherwise the algorithm computes all possible outgoing edges for the vertices at the position i+1
     *
     * Examples:
     * The process is illustrated by showing the graph (edges) after each iteration from left to right over the input:
     * An edge is drawn as follows "(" + fromVertex + ") -- " + encodingMode + "(" + encodedInput + ") (" +
     * accumulatedSize + ") --> (" + toVertex + ")"
     *
     * Example 1 encoding the string "ABCDE":
     * Note: This example assumes that alphanumeric encoding is only possible in multiples of two characters so that
     * the example is both short and showing the principle. In reality this restriction does not exist.
     *
     * Initial situation
     * (initial) -- BYTE(A) (20) --> (1_BYTE)
     * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC)
     *
     * Situation after adding edges to vertices at position 1
     * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE)
     *                               (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC)
     * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC)
     *
     * Situation after adding edges to vertices at position 2
     * (initial) -- BYTE(A) (20) --> (1_BYTE)
     * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC)
     * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE)
                                   * (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC)
     * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC) -- BYTE(C) (44) --> (3_BYTE)
     *                                                            (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC)
     *
     * Situation after adding edges to vertices at position 3
     * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE) -- BYTE(C)         (36) --> (3_BYTE)
     *                               (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC) -- BYTE(D) (64) --> (4_BYTE)
     *                                                                                                 (3_ALPHANUMERIC) -- ALPHANUMERIC(DE)                             (55) --> (5_ALPHANUMERIC)
     * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC)
     *                                                            (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC)
     *
     * Situation after adding edges to vertices at position 4
     * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE) -- BYTE(C)         (36) --> (3_BYTE) -- BYTE(D) (44) --> (4_BYTE)
     *                               (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC) -- ALPHANUMERIC(DE)                             (55) --> (5_ALPHANUMERIC)
     * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC) -- BYTE(E) (55) --> (5_BYTE)
     *
     * Situation after adding edges to vertices at position 5
     * (initial) -- BYTE(A) (20) --> (1_BYTE) -- BYTE(B) (28) --> (2_BYTE) -- BYTE(C)         (36) --> (3_BYTE) -- BYTE(D)         (44) --> (4_BYTE) -- BYTE(E)         (52) --> (5_BYTE)
     *                               (1_BYTE) -- ALPHANUMERIC(BC)                             (44) --> (3_ALPHANUMERIC) -- ALPHANUMERIC(DE)                             (55) --> (5_ALPHANUMERIC)
     * (initial) -- ALPHANUMERIC(AB)                     (24) --> (2_ALPHANUMERIC) -- ALPHANUMERIC(CD)                             (35) --> (4_ALPHANUMERIC)
     *
     * Encoding as BYTE(ABCDE) has the smallest size of 52 and is hence chosen. The encodation ALPHANUMERIC(ABCD),
     * BYTE(E) is longer with a size of 55.
     *
     * Example 2 encoding the string "XXYY" where X denotes a character unique to character set ISO-8859-2 and Y a
     * character unique to ISO-8859-3. Both characters encode as double byte in UTF-8:
     *
     * Initial situation
     * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE)
     *
     * Situation after adding edges to vertices at position 1
     * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2) -- BYTE(X) (40) --> (2_BYTE_ISO-8859-2)
     *                               (1_BYTE_ISO-8859-2) -- BYTE(X) (72) --> (2_BYTE_UTF-8)
     *                               (1_BYTE_ISO-8859-2) -- BYTE(X) (72) --> (2_BYTE_UTF-16BE)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE)
     *
     * Situation after adding edges to vertices at position 2
     * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2) -- BYTE(X) (40) --> (2_BYTE_ISO-8859-2)
     *                                                                       (2_BYTE_ISO-8859-2) -- BYTE(Y) (72) --> (3_BYTE_ISO-8859-3)
     *                                                                       (2_BYTE_ISO-8859-2) -- BYTE(Y) (80) --> (3_BYTE_UTF-8)
     *                                                                       (2_BYTE_ISO-8859-2) -- BYTE(Y) (80) --> (3_BYTE_UTF-16BE)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8) -- BYTE(X) (56) --> (2_BYTE_UTF-8)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE) -- BYTE(X) (56) --> (2_BYTE_UTF-16BE)
     *
     * Situation after adding edges to vertices at position 3
     * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2) -- BYTE(X) (40) --> (2_BYTE_ISO-8859-2) -- BYTE(Y) (72) --> (3_BYTE_ISO-8859-3)
     *                                                                                                               (3_BYTE_ISO-8859-3) -- BYTE(Y) (80) --> (4_BYTE_ISO-8859-3)
     *                                                                                                               (3_BYTE_ISO-8859-3) -- BYTE(Y) (112) --> (4_BYTE_UTF-8)
     *                                                                                                               (3_BYTE_ISO-8859-3) -- BYTE(Y) (112) --> (4_BYTE_UTF-16BE)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8) -- BYTE(X) (56) --> (2_BYTE_UTF-8) -- BYTE(Y) (72) --> (3_BYTE_UTF-8)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE) -- BYTE(X) (56) --> (2_BYTE_UTF-16BE) -- BYTE(Y) (72) --> (3_BYTE_UTF-16BE)
     *
     * Situation after adding edges to vertices at position 4
     * (initial) -- BYTE(X) (32) --> (1_BYTE_ISO-8859-2) -- BYTE(X) (40) --> (2_BYTE_ISO-8859-2) -- BYTE(Y) (72) --> (3_BYTE_ISO-8859-3) -- BYTE(Y) (80) --> (4_BYTE_ISO-8859-3)
     *                                                                                                               (3_BYTE_UTF-8) -- BYTE(Y) (88) --> (4_BYTE_UTF-8)
     *                                                                                                               (3_BYTE_UTF-16BE) -- BYTE(Y) (88) --> (4_BYTE_UTF-16BE)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-8) -- BYTE(X) (56) --> (2_BYTE_UTF-8) -- BYTE(Y) (72) --> (3_BYTE_UTF-8)
     * (initial) -- BYTE(X) (40) --> (1_BYTE_UTF-16BE) -- BYTE(X) (56) --> (2_BYTE_UTF-16BE) -- BYTE(Y) (72) --> (3_BYTE_UTF-16BE)
     *
     * Encoding as ECI(ISO-8859-2),BYTE(XX),ECI(ISO-8859-3),BYTE(YY) has the smallest size of 80 and is hence chosen.
     * The encodation ECI(UTF-8),BYTE(XXYY) is longer with a size of 88.
     */

    let inputLength = self.stringToEncode.len();

    // Array that represents vertices. There is a vertex for every character, encoding and mode. The vertex contains
    // a list of all edges that lead to it that have the same encoding and mode.
    // The lists are created lazily

    // The last dimension in the array below encodes the 4 modes KANJI, ALPHANUMERIC, NUMERIC and BYTE via the
    // function getCompactedOrdinal(Mode)
    let edges = vec![vec![vec![None;4];self.encoders.len()];inputLength+1];//new Edge[inputLength + 1][encoders.length()][4];
   self. addEdges(version, &edges, 0, None);

   for i in 1..=inputLength {
    // for (int i = 1; i <= inputLength; i++) {
      for j in 0..self.encoders.len() {
      // for (int j = 0; j < encoders.length(); j++) {
        for k in 0..4 {
        // for (int k = 0; k < 4; k++) {
          if edges[i][j][k].is_some() && i < inputLength {
            self.addEdges(version, &edges, i, edges[i][j][k]);
          }
        }
      }

    }
    let minimalJ = None;
    let minimalK = None;
    let minimalSize = u32::MAX;
    for j in 0..self.encoders.len() {
    // for (int j = 0; j < encoders.length(); j++) {
      for k in 0..4 {
      // for (int k = 0; k < 4; k++) {
        if edges[inputLength][j][k].is_some() {
          let edge = edges[inputLength][j][k].as_ref().unwrap();
          if edge.cachedTotalSize < minimalSize {
            minimalSize = edge.cachedTotalSize;
            minimalJ = Some(j);
            minimalK = Some(k);
          }
        }
      }
    }
    if minimalJ.is_none() {
      return Err(Exceptions::WriterException(format!(r#"Internal error: failed to encode "{}"#,self.stringToEncode)));
    }
      Ok(RXingResultList::new(version, edges[inputLength][minimalJ.unwrap()][minimalK.unwrap()].unwrap(),self.isGS1,&self.ecLevel, &self.encoders, &self.stringToEncode))
  }
}


struct Edge<'a> {
  pub mode:Mode,
  fromPosition:usize,
  charsetEncoderIndex:usize,
  characterLength:u32,
  previous:Option<&'a Edge<'a>>,
  cachedTotalSize:u32,
  encoders:&'a ECIEncoderSet,
  stringToEncode: &'a str,
}
impl Edge<'_> {

  pub fn new( mode:Mode,  fromPosition:usize,  charsetEncoderIndex:usize,  characterLength:u32,  previous:Option<&'_ Edge>,
                version:&Version, encoders: &'_ ECIEncoderSet, stringToEncode: &'_ str) -> Self {
                  let nci = if mode == Mode::BYTE || previous.is_none()  {charsetEncoderIndex} else
                    {previous.as_ref().unwrap().charsetEncoderIndex};
                  Self {
                    mode,
                    fromPosition,
                    charsetEncoderIndex: nci,
                    characterLength,
                    previous,
                    stringToEncode,
                    cachedTotalSize: {
                      let size = if previous.is_some()  {previous.as_ref().unwrap().cachedTotalSize} else {0};

                      let needECI = mode == Mode::BYTE &&
                          (previous.is_none() && nci != 0) || // at the beginning and charset is not ISO-8859-1
                          (previous.is_some() && nci != previous.as_ref().unwrap().charsetEncoderIndex);
                  
                      if previous.is_none()|| mode != previous.as_ref().unwrap().mode || needECI {
                        size += 4 + mode.getCharacterCountBits(&version) as u32;
                      }
                      match mode {
                        Mode::NUMERIC =>  size += if characterLength == 1  {4} else {if characterLength == 2  {7} else {10}},
                        Mode::ALPHANUMERIC => size += if characterLength == 1  {6} else {11},
                        Mode::BYTE =>{
                          size += 8 * encoders.encode_string(&stringToEncode[fromPosition as usize..(fromPosition + characterLength as usize)],
                              charsetEncoderIndex as usize).len() as u32;
                          if needECI {
                            size += 4 + 8; // the ECI assignment numbers for ISO-8859-x, UTF-8 and UTF-16 are all 8 bit long
                          }
                        },
                        Mode::KANJI => size += 13,
                        _=> {},
                    }
                      // switch (mode) {
                      //   case KANJI:
                      //     size += 13;
                      //     break;
                      //   case ALPHANUMERIC:
                      //     size += characterLength == 1 ? 6 : 11;
                      //     break;
                      //   case NUMERIC:
                      //     size += characterLength == 1 ? 4 : characterLength == 2 ? 7 : 10;
                      //     break;
                      //   case BYTE:
                      //     size += 8 * encoders.encode(stringToEncode.substring(fromPosition, fromPosition + characterLength),
                      //         charsetEncoderIndex).length;
                      //     if (needECI) {
                      //       size += 4 + 8; // the ECI assignment numbers for ISO-8859-x, UTF-8 and UTF-16 are all 8 bit long
                      //     }
                      //     break;
                      // }
                      size
                    },
                    encoders
                }
    // this.mode = mode;
    // this.fromPosition = fromPosition;
    // this.charsetEncoderIndex = mode == Mode.BYTE || previous == null ? charsetEncoderIndex :
    //     previous.charsetEncoderIndex; // inherit the encoding if not of type BYTE
    // this.characterLength = characterLength;
    // this.previous = previous;

    // int size = previous != null ? previous.cachedTotalSize : 0;

    // boolean needECI = mode == Mode.BYTE &&
    //     (previous == null && this.charsetEncoderIndex != 0) || // at the beginning and charset is not ISO-8859-1
    //     (previous != null && this.charsetEncoderIndex != previous.charsetEncoderIndex);

    // if (previous == null || mode != previous.mode || needECI) {
    //   size += 4 + mode.getCharacterCountBits(version);
    // }
    // switch (mode) {
    //   case KANJI:
    //     size += 13;
    //     break;
    //   case ALPHANUMERIC:
    //     size += characterLength == 1 ? 6 : 11;
    //     break;
    //   case NUMERIC:
    //     size += characterLength == 1 ? 4 : characterLength == 2 ? 7 : 10;
    //     break;
    //   case BYTE:
    //     size += 8 * encoders.encode(stringToEncode.substring(fromPosition, fromPosition + characterLength),
    //         charsetEncoderIndex).length;
    //     if (needECI) {
    //       size += 4 + 8; // the ECI assignment numbers for ISO-8859-x, UTF-8 and UTF-16 are all 8 bit long
    //     }
    //     break;
    // }
    // cachedTotalSize = size;
  }
}

struct RXingResultList<'a> {
  list: Vec<RXingResultNode<'a>>,
  version: VersionRef,
}
impl RXingResultList<'_> {

  pub fn new( version:VersionRef,  solution:&'_ Edge, isGS1:bool, ecLevel: &ErrorCorrectionLevel, encoders:&'_ ECIEncoderSet, stringToEncode: &'_ str) -> Self {
    let length = 0;
    let current = Some(solution);
    let containsECI = false;
    let list = Vec::new();

    while current.is_some() {
      length += current.as_ref().unwrap().characterLength;
      let previous = current.as_ref().unwrap().previous;

      let needECI = current.as_ref().unwrap().mode == Mode::BYTE &&
          (previous.is_none() && current.as_ref().unwrap().charsetEncoderIndex != 0) || // at the beginning and charset is not ISO-8859-1
          (previous.is_some() && current.as_ref().unwrap().charsetEncoderIndex != previous.as_ref().unwrap().charsetEncoderIndex);

      if needECI {
        containsECI = true;
      }

      if previous.is_none() || previous.as_ref().unwrap().mode != current.as_ref().unwrap().mode || needECI {
        list.push(  RXingResultNode::new(current.as_ref().unwrap().mode, current.as_ref().unwrap().fromPosition, current.as_ref().unwrap().charsetEncoderIndex, length,encoders,stringToEncode, version));
        length = 0;
      }

      if needECI {
        list.push(  RXingResultNode::new(Mode::ECI, current.as_ref().unwrap().fromPosition, current.as_ref().unwrap().charsetEncoderIndex, 0,encoders,stringToEncode, version));
      }
      current = previous;
    }

    // prepend FNC1 if needed. If the bits contain an ECI then the FNC1 must be preceeded by an ECI.
    // If there is no ECI at the beginning then we put an ECI to the default charset (ISO-8859-1)
    if isGS1 {
      if let Some(first) = list.get(0){
      // if first != null && first.mode != Mode.ECI && containsECI {
      if first.mode != Mode::ECI && containsECI {
        // prepend a default character set ECI
        list.push(  RXingResultNode::new(Mode::ECI, 0, 0, 0,encoders,stringToEncode,version));
      }}
      let first = list.get(0).unwrap();
      // prepend or insert a FNC1_FIRST_POSITION after the ECI (if any)
      // if first != null && first.mode != Mode.ECI && containsECI {
      list.push(   RXingResultNode::new(Mode::FNC1_FIRST_POSITION, 0, 0, 0,encoders,stringToEncode,version));
    }

    // set version to smallest version into which the bits fit.
    let versionNumber = version.getVersionNumber();
    let (lowerLimit,upperLimit) = 
    match MinimalEncoder::getVersionSize(&version) {
      VersionSize::SMALL =>
        (1,9),
        VersionSize::MEDIUM=>(10,26),
      _=>(27,40),
    };
    // let lowerLimit;
    // let upperLimit;
    // switch (getVersionSize(version)) {
    //   case SMALL:
    //     lowerLimit = 1;
    //     upperLimit = 9;
    //     break;
    //   case MEDIUM:
    //     lowerLimit = 10;
    //     upperLimit = 26;
    //     break;
    //   case LARGE:
    //   default:
    //     lowerLimit = 27;
    //     upperLimit = 40;
    //     break;
    // }
    let size = Self::internal_static_get_size(version,&list);
    // increase version if needed
    while versionNumber < upperLimit && !encoder::willFit(size, Version::getVersionForNumber(versionNumber).unwrap(),
      ecLevel) {
      versionNumber+=1;
    }
    // shrink version if possible
    while versionNumber > lowerLimit && encoder::willFit(size, Version::getVersionForNumber(versionNumber - 1).unwrap(),
      ecLevel) {
      versionNumber-=1;
    }
    let version =  Version::getVersionForNumber(versionNumber).unwrap();
    Self {
        list,
        version,
    }
  }

  /**
   * returns the size in bits
   */
  pub fn  getSize(&self) -> u32{
    self. getSizeLocal(self.version)
  }

  fn getSizeLocal(&self, version:VersionRef) -> u32{
    let result = 0;
    for  resultNode in &self.list {
      result += resultNode.getSize(version);
    }
    return result;
  }

  fn internal_static_get_size(version:VersionRef, list: &Vec<RXingResultNode>) -> u32 {
    let result = 0;
    for  resultNode in list {
      result += resultNode.getSize(version);
    }
    return result;
  }

  /**
   * appends the bits
   */
  pub fn getBits(&self,  bits:&BitArray) -> Result<(),Exceptions> {
    for  resultNode in &self.list {
      resultNode.getBits(bits);
    }
    Ok(())
  }

  pub fn getVersion(&self) -> &Version{
    &self. version
  }
}

impl fmt::Display for RXingResultList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let mut result =  String::new();
      let previous = None;
      for current in &self.list {
      // for (RXingResultNode current : list) {
        if previous.is_some() {
          result.push_str(",");
        }
        result.push_str(&current.to_string());
        previous = Some(current);
      }
      write!(f,"{}", result)
    }
}

struct RXingResultNode<'a> {

    mode:Mode,
    fromPosition:usize,
    charsetEncoderIndex:usize,
    characterLength:u32,
    encoders:&'a ECIEncoderSet,
    version: VersionRef,
    stringToEncode: &'a str,
}

impl RXingResultNode<'_> {

    pub fn new( mode:Mode,  fromPosition:usize,  charsetEncoderIndex:usize,  characterLength:u32, encoders:&'_ ECIEncoderSet, stringToEncode: &'_ str, version: &'_ Version) -> Self {
      Self {
        mode,
        fromPosition,
        charsetEncoderIndex,
        characterLength,
        encoders,
        stringToEncode,
        version,
    }
    }

    /**
     * returns the size in bits
     */
    fn getSize(&self, version:&Version) -> u32{
      let size = 4 + self.mode.getCharacterCountBits(version) as u32;
      match self.mode {
        Mode::NUMERIC => {
          size += (self.characterLength / 3) * 10;
          let rest = self.characterLength % 3;
          size += if rest == 1  {4} else { if rest == 2  {7} else {0}};
        },
        Mode::ALPHANUMERIC => {
          size += (self.characterLength / 2) * 11;
          size += if (self.characterLength % 2) == 1  {6} else {0};
        },
        Mode::BYTE => size += 8 * self.getCharacterCountIndicator(),
        Mode::ECI => size += 8,
        Mode::KANJI => size += 13 * self.characterLength,
        _ => {},
    }
      // switch (mode) {
        // case KANJI:
        //   size += 13 * characterLength;
        //   break;
        // case ALPHANUMERIC:
        //   size += (characterLength / 2) * 11;
        //   size += (characterLength % 2) == 1 ? 6 : 0;
        //   break;
        // case NUMERIC:
        //   size += (characterLength / 3) * 10;
        //   int rest = characterLength % 3;
        //   size += rest == 1 ? 4 : rest == 2 ? 7 : 0;
        //   break;
        // case BYTE:
        //   size += 8 * getCharacterCountIndicator();
        //   break;
        // case ECI:
        //   size += 8; // the ECI assignment numbers for ISO-8859-x, UTF-8 and UTF-16 are all 8 bit long
      // }
       size
    }

    /**
     * returns the length in characters according to the specification (differs from getCharacterLength() in BYTE mode
     * for multi byte encoded characters)
     */
    fn getCharacterCountIndicator(&self) -> u32{
      if self.mode == Mode::BYTE 
          {self.encoders.encode_string(&self.stringToEncode[self.fromPosition as usize..(self.fromPosition + self.characterLength as usize)],
          self.charsetEncoderIndex as usize).len() as u32} else {self.characterLength}
    }

    /**
     * appends the bits
     */
    fn getBits(&self,  bits:&BitArray) -> Result<(),Exceptions> {
      bits.appendBits(self.mode.getBits() as u32, 4);
      if self.characterLength > 0 {
        let length = self.getCharacterCountIndicator();
        bits.appendBits(length, self.mode.getCharacterCountBits(self.version) as usize);
      }
      if self.mode == Mode::ECI {
        bits.appendBits(self.encoders.getECIValue(self.charsetEncoderIndex as usize), 8);
      } else if self.characterLength > 0 {
        // append data
        encoder::appendBytes(&self.stringToEncode[self.fromPosition as usize..(self.fromPosition + self.characterLength as usize)], self.mode, bits,
            self.encoders.getCharset(self.charsetEncoderIndex as usize));
      }
      Ok(())
    }

    fn makePrintable( s:&str) -> String {
      let mut result =  String::new();
      for i in 0..s.chars().count() {
      // for (int i = 0; i < s.length(); i++) {
        if (s.chars().nth(i).unwrap() as u8) < 32 || (s.chars().nth(i).unwrap() as u8) > 126 {
          result.push('.');
        } else {
          result.push(s.chars().nth(i).unwrap());
        }
      }
      result
    }
  }

  impl fmt::Display for RXingResultNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let result = String::new();
      result.push_str(&format!("{:?}",self.mode));
      result.push('(');
      if self.mode == Mode::ECI {
        result.push_str(self.encoders.getCharset(self.charsetEncoderIndex as usize).name());
      } else {
        result.push_str(&Self::makePrintable(&self.stringToEncode[self.fromPosition as usize..(self.fromPosition + self.characterLength as usize)]));
      }
      result.push(')');
      
      write!(f,"{}", result)
    }
}