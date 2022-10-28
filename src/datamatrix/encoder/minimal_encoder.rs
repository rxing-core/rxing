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

use std::{rc::Rc, fmt};

use encoding::{self,EncodingRef};

use crate::{common::{MinimalECIInput, ECIInput}, Exceptions};

use super::{SymbolShapeHint, high_level_encoder};

const ISO_8859_1_ENCODER : EncodingRef = encoding::all::ISO_8859_1;

/**
 * Encoder that encodes minimally
 *
 * Algorithm:
 *
 * Uses Dijkstra to produce mathematically minimal encodings that are in some cases smaller than the results produced
 * by the algorithm described in annex S in the specification ISO/IEC 16022:200(E). The biggest improvment of this
 * algorithm over that one is the case when the algorithm enters the most inefficient mode, the B256 Mode:: The 
 * algorithm from the specification algorithm will exit this mode only if it encounters digits so that arbitrarily
 * inefficient results can be produced if the postfix contains no digits.
 *
 * Multi ECI support and ECI switching:
 *
 * For multi language content the algorithm selects the most compact representation using ECI modes. Note that unlike
 * the compaction algorithm used for QR-Codes, this implementation operates in two stages and therfore is not
 * mathematically optimal. In the first stage, the input string is encoded minimally as a stream of ECI character set
 * selectors and bytes encoded in the selected encoding. In this stage the algorithm might for example decide to
 * encode ocurrences of the characters "\u0150\u015C" (O-double-acute, S-circumflex) in UTF-8 by a single ECI or
 * alternatively by multiple ECIs that switch between IS0-8859-2 and ISO-8859-3 (e.g. in the case that the input
 * contains many * characters from ISO-8859-2 (Latin 2) and few from ISO-8859-3 (Latin 3)).
 * In a second stage this stream of ECIs and bytes is minimally encoded using the various Data Matrix encoding modes.
 * While both stages encode mathematically minimally it is not ensured that the result is mathematically minimal since
 * the size growth for inserting an ECI in the first stage can only be approximated as the first stage does not know 
 * in which mode the ECI will occur in the second stage (may, or may not require an extra latch to ASCII depending on
 * the current mode). The reason for this shortcoming are difficulties in implementing it in a straightforward and
 * readable manner.
 *
 * GS1 support
 *
 * FNC1 delimiters can be encoded in the input string by using the FNC1 character specified in the encoding function.
 * When a FNC1 character is specified then a leading FNC1 will be encoded and all ocurrences of delimiter characters
 * while result in FNC1 codewords in the symbol.
 *
 * @author Alex Geller
 */

 #[derive(Debug,Copy,Clone,PartialEq, Eq)]
  enum Mode {
    ASCII,
    C40,
    TEXT,
    X12,
    EDF,
    B256
  }

  impl Mode {
    pub fn ordinal(&self) -> usize {
      match self {
        Mode::ASCII => 0,
        Mode::C40 => 1,
        Mode::TEXT => 2,
        Mode::X12 => 3,
        Mode::EDF => 4,
        Mode::B256 => 5,
    }
    }
  }

  const C40_SHIFT2_CHARS :[char;27] = ['!', '"', '#', '$', '%', '&', '\'', '(', ')', '*',  '+', ',', '-', '.', '/',
                                          ':', ';', '<', '=', '>', '?',  '@', '[', '\\', ']', '^', '_' ];


  pub fn isExtendedASCII( ch:char,  fnc1:Option<char>) -> bool{
    let is_fnc1 = if let Some(fnc1) = fnc1 {
      ch != fnc1
    }else {
      true
    };
    is_fnc1 && ch as u8 >= 128 && ch as u8 <= 255
    // return ch != fnc1 && ch as u8 >= 128 && ch as u8 <= 255;
  }

  fn isInC40Shift1Set( ch:char) -> bool{
     ch as u8 <= 31
  }

  fn isInC40Shift2Set( ch:char,  fnc1:Option<char>) -> bool{
    for c40Shift2Char in C40_SHIFT2_CHARS {
    // for (char c40Shift2Char : C40_SHIFT2_CHARS) {
      if c40Shift2Char == ch {
        return true;
      }
    }
    if let Some(fnc1) = fnc1 {
      ch == fnc1
    }else {
      false
    }
    // return ch as u8 as i32 == fnc1;
  }

  fn isInTextShift1Set( ch:char) -> bool{
     isInC40Shift1Set(ch)
  }

  fn isInTextShift2Set( ch:char,  fnc1:Option<char>) -> bool{
     isInC40Shift2Set(ch, fnc1)
  }

  /**
   * Performs message encoding of a DataMatrix message
   *
   * @param msg the message
   * @return the encoded message (the char values range from 0 to 255)
   */
  pub fn encodeHighLevel( msg:&str) -> Result<String,Exceptions>{
     encodeHighLevelWithDetails(msg, None, None, SymbolShapeHint::FORCE_NONE)
  }

  /**
   * Performs message encoding of a DataMatrix message
   *
   * @param msg the message
   * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
   *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
   *   charset to encode any character in the input that can be encoded by it if the charset is among the
   *   supported charsets.
   * @param fnc1 denotes the character in the input that represents the FNC1 character or -1 if this is not a GS1
   *   bar code. If the value is not -1 then a FNC1 is also prepended.
   * @param shape requested shape.
   * @return the encoded message (the char values range from 0 to 255)
   */
  pub fn encodeHighLevelWithDetails( msg:&str,  priorityCharset:Option<EncodingRef>,  fnc1:Option<char>,  shape:SymbolShapeHint)->Result<String,Exceptions> {
    let mut msg = msg;
    let macroId = 0;
    if msg.starts_with(high_level_encoder::MACRO_05_HEADER) && msg.ends_with(high_level_encoder::MACRO_TRAILER) {
      macroId = 5;
      // msg = msg.substring(high_level_encoder::MACRO_05_HEADER.len(), msg.len() - 2);
      msg = &msg[high_level_encoder::MACRO_05_HEADER.len()..(msg.len() - 2)];
    } else if msg.starts_with(high_level_encoder::MACRO_06_HEADER) && msg.ends_with(high_level_encoder::MACRO_TRAILER) {
      macroId = 6;
      // msg = msg.substring(high_level_encoder::MACRO_06_HEADER.len(), msg.len() - 2);
      msg = &msg[high_level_encoder::MACRO_06_HEADER.len()..(msg.len() - 2)];
    }
    Ok(ISO_8859_1_ENCODER.decode(&encode(msg, priorityCharset, fnc1, shape, macroId)?, encoding::DecoderTrap::Strict).expect("should decode").to_owned())
    // return new String(encode(msg, priorityCharset, fnc1, shape, macroId), StandardCharsets.ISO_8859_1);
  }

  /**
   * Encodes input minimally and returns an array of the codewords
   *
   * @param input The string to encode
   * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
   *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
   *   charset to encode any character in the input that can be encoded by it if the charset is among the
   *   supported charsets.
   * @param fnc1 denotes the character in the input that represents the FNC1 character or -1 if this is not a GS1
   *   bar code. If the value is not -1 then a FNC1 is also prepended.
   * @param shape requested shape.
   * @param macroId Prepends the specified macro function in case that a value of 5 or 6 is specified.
   * @return An array of bytes representing the codewords of a minimal encoding.
   */
  fn encode( input:&str,  priorityCharset:Option<EncodingRef>,  fnc1:Option<char>,  shape:SymbolShapeHint,  macroId:i32) -> Result<Vec<u8>,Exceptions> {
     Ok(encodeMinimally( &Input::new(input, priorityCharset, fnc1, shape, macroId))?.getBytes().to_vec())
  }

  fn addEdge( edges:&Vec<Vec<Option<Rc<Edge>>>>,  edge:Rc<Edge>) {
    let vertexIndex = (edge.fromPosition + edge.characterLength) as usize;
    if edges[vertexIndex][edge.getEndMode().ordinal()].is_none() ||
        edges[vertexIndex][edge.getEndMode().ordinal()].as_ref().unwrap().cachedTotalSize > edge.cachedTotalSize {
      edges[vertexIndex][edge.getEndMode().ordinal()] = Some(edge.clone());
    }
  }

  /** @return the number of words in which the string starting at from can be encoded in c40 or text Mode::
   *  The number of characters encoded is returned in characterLength.
   *  The number of characters encoded is also minimal in the sense that the algorithm stops as soon
   *  as a character encoding fills a C40 word competely (three C40 values). An exception is at the
   *  end of the string where two C40 values are allowed (according to the spec the third c40 value 
   *  is filled  with 0 (Shift 1) in this case).
   */
  fn getNumberOfC40Words( input:&Input,  from:u32,  c40:bool, characterLength:&[u32]) -> Result<u32,Exceptions>{
    let thirdsCount = 0;
    for i in (from as usize)..input.length() {
    // for (int i = from; i < input.length(); i++) {
      if input.isECI(i as u32)? {
        characterLength[0] = 0;
        return Ok(0);
      }
      let ci = input.charAt(i)?;
      if c40 && high_level_encoder::isNativeC40(ci) || !c40 && high_level_encoder::isNativeText(ci) {
        thirdsCount+=1; //native
      } else if !isExtendedASCII(ci, Some(input.getFNC1Character())) {
        thirdsCount += 2; //shift
      } else {
        let asciiValue = ci as u8 & 0xff;
        if asciiValue >= 128 && (c40 && high_level_encoder::isNativeC40( (asciiValue - 128) as char) ||
                                  !c40 && high_level_encoder::isNativeText( (asciiValue - 128) as char)) {
          thirdsCount += 3; // shift, Upper shift
        } else {
          thirdsCount += 4; // shift, Upper shift, shift
        }
      }

      if thirdsCount % 3 == 0 || ((thirdsCount - 2) % 3 == 0 && i + 1 == input.length()) {
        characterLength[0] = i as u32 - from + 1;
        // return (int) Math.ceil(((double) thirdsCount) / 3.0);
        return Ok((( thirdsCount as f64) / 3.0).ceil() as u32);
      }
    }
    characterLength[0] = 0;
    
    Ok(0)
  }

  fn addEdges( input:&Input,  edges:&Vec<Vec<Option<Rc<Edge>>>>,  from:u32,  previous:Option<Rc<Edge>>) -> Result<(),Exceptions> {

    if input.isECI(from)? {
      addEdge(edges,  Rc::new(Edge::new(input, Mode::ASCII, from, 1, previous.clone())));
      return Ok(());
    }

    let ch = input.charAt(from as usize)?;
    if previous.is_none() || previous.as_ref().unwrap().getEndMode() != Mode::EDF { //not possible to unlatch a full EDF edge to something
                                                                 //else
      if high_level_encoder::isDigit(ch) && input.haveNCharacters(from as usize, 2) &&
          high_level_encoder::isDigit(input.charAt(from as usize + 1)?) {
        // two digits ASCII encoded
        addEdge(edges,  Rc::new(Edge::new(input, Mode::ASCII, from, 2, previous.clone())));
      } else {
        // one ASCII encoded character or an extended character via Upper Shift
        addEdge(edges,  Rc::new(Edge::new(input, Mode::ASCII, from, 1, previous.clone())));
      }
  
      let modes = [Mode::C40, Mode::TEXT];
      for mode in modes {
      // for (Mode mode : modes) {
        let characterLength = [0u32;1];
        if getNumberOfC40Words(input, from, mode == Mode::C40, &characterLength)? > 0 {
          addEdge(edges,  Rc::new(Edge::new(input, mode, from, characterLength[0], previous.clone())));
        }
      }
  
      if input.haveNCharacters(from as usize,3) &&
          high_level_encoder::isNativeX12(input.charAt(from as usize)?) &&
          high_level_encoder::isNativeX12(input.charAt(from  as usize+ 1)?) &&
          high_level_encoder::isNativeX12(input.charAt(from as usize + 2)?) {
        addEdge(edges,  Rc::new(Edge::new(input, Mode::X12, from, 3, previous.clone())));
      }

      addEdge(edges, Rc::new(Edge::new(input, Mode::B256, from, 1, previous.clone())));
    }

    //We create 4 EDF edges,  with 1, 2 3 or 4 characters length. The fourth normally doesn't have a latch to ASCII
    //unless it is 2 characters away from the end of the input.
    let i = 0u32;
    while i < 3 {
    // for (i = 0; i < 3; i++) {
      let pos = from + i;
      if input.haveNCharacters(pos as usize,1) && high_level_encoder::isNativeEDIFACT(input.charAt(pos as usize)?) {
        addEdge(edges,  Rc::new(Edge::new(input, Mode::EDF, from, i + 1, previous.clone())));
      } else {
        break;
      }
      i+=1;
    }
    if i == 3 && input.haveNCharacters(from as usize, 4) && high_level_encoder::isNativeEDIFACT(input.charAt(from as usize + 3)?) {
      addEdge(edges, Rc::new( Edge::new(input, Mode::EDF, from, 4, previous.clone())));
    }
    Ok(())
  }

  fn encodeMinimally( input:&Input) -> Result<RXingResult,Exceptions>{

    // @SuppressWarnings("checkstyle:lineLength")
    /* The minimal encoding is computed by Dijkstra. The acyclic graph is modeled as follows:
     * A vertex represents a combination of a position in the input and an encoding mode where position 0
     * denotes the position left of the first character, 1 the position left of the second character and so on.
     * Likewise the end vertices are located after the last character at position input.length().
     * For any position there might be up to six vertices, one for each of the encoding types ASCII, C40, TEXT, X12,
     * EDF and B256.
     * 
     * As an example consider the input string "ABC123" then at position 0 there is only one vertex with the default
     * ASCII encodation. At position 3 there might be vertices for the types ASCII, C40, X12, EDF and B256.
     *
     * An edge leading to such a vertex encodes one or more of the characters left of the position that the vertex
     * represents. It encodes the characters in the encoding mode of the vertex that it ends on. In other words,
     * all edges leading to a particular vertex encode the same characters (the length of the suffix can vary) using the same 
     * encoding Mode::
     * As an example consider the input string "ABC123" and the vertex (4,EDF). Possible edges leading to this vertex
     * are: 
     *   (0,ASCII)  --EDF(ABC1)--> (4,EDF)
     *   (1,ASCII)  --EDF(BC1)-->  (4,EDF)
     *   (1,B256)   --EDF(BC1)-->  (4,EDF)
     *   (1,EDF)    --EDF(BC1)-->  (4,EDF)
     *   (2,ASCII)  --EDF(C1)-->   (4,EDF)
     *   (2,B256)   --EDF(C1)-->   (4,EDF)
     *   (2,EDF)    --EDF(C1)-->   (4,EDF)
     *   (3,ASCII)  --EDF(1)-->    (4,EDF)
     *   (3,B256)   --EDF(1)-->    (4,EDF)
     *   (3,EDF)    --EDF(1)-->    (4,EDF)
     *   (3,C40)    --EDF(1)-->    (4,EDF)
     *   (3,X12)    --EDF(1)-->    (4,EDF)
     *
     * The edges leading to a vertex are stored in such a way that there is a fast way to enumerate the edges ending
     * on a particular vertex.
     *
     * The algorithm processes the vertices in order of their position thereby performing the following:
     *
     * For every vertex at position i the algorithm enumerates the edges ending on the vertex and removes all but the
     * shortest from that list.
     * Then it processes the vertices for the position i+1. If i+1 == input.length() then the algorithm ends
     * and chooses the the edge with the smallest size from any of the edges leading to vertices at this position.
     * Otherwise the algorithm computes all possible outgoing edges for the vertices at the position i+1
     *
     * Examples:
     * The process is illustrated by showing the graph (edges) after each iteration from left to right over the input:
     * An edge is drawn as follows "(" + fromVertex + ") -- " + encodingMode + "(" + encodedInput + ") (" +
     * accumulatedSize + ") --> (" + toVertex + ")"
     *
     * Example 1 encoding the string "ABCDEFG":
     *
     *
     * Situation after adding edges to the start vertex (0,ASCII)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII)
     * (0,ASCII) B256(A) (3) --> (1,B256)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF)
     * (0,ASCII) C40(ABC) (3) --> (3,C40)
     * (0,ASCII) TEXT(ABC) (5) --> (3,TEXT)
     * (0,ASCII) X12(ABC) (3) --> (3,X12)
     * (0,ASCII) EDF(ABC) (4) --> (3,EDF)
     * (0,ASCII) EDF(ABCD) (4) --> (4,EDF)
     *
     * Situation after adding edges to vertices at position 1
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII)
     * (0,ASCII) B256(A) (3) --> (1,B256)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF)
     * (0,ASCII) C40(ABC) (3) --> (3,C40)
     * (0,ASCII) TEXT(ABC) (5) --> (3,TEXT)
     * (0,ASCII) X12(ABC) (3) --> (3,X12)
     * (0,ASCII) EDF(ABC) (4) --> (3,EDF)
     * (0,ASCII) EDF(ABCD) (4) --> (4,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) B256(B) (4) --> (2,B256)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) EDF(BC) (5) --> (3,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) C40(BCD) (4) --> (4,C40)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) TEXT(BCD) (6) --> (4,TEXT)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) X12(BCD) (4) --> (4,X12)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) EDF(BCD) (5) --> (4,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) EDF(BCDE) (5) --> (5,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) ASCII(B) (4) --> (2,ASCII)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256)
     * (0,ASCII) B256(A) (3) --> (1,B256) EDF(BC) (6) --> (3,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) C40(BCD) (5) --> (4,C40)
     * (0,ASCII) B256(A) (3) --> (1,B256) TEXT(BCD) (7) --> (4,TEXT)
     * (0,ASCII) B256(A) (3) --> (1,B256) X12(BCD) (5) --> (4,X12)
     * (0,ASCII) B256(A) (3) --> (1,B256) EDF(BCD) (6) --> (4,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) EDF(BCDE) (6) --> (5,EDF)
     *
     * Edge "(1,ASCII) ASCII(B) (2) --> (2,ASCII)" is minimal for the vertex (2,ASCII) so that edge "(1,B256) ASCII(B) (4) --> (2,ASCII)" is removed.
     * Edge "(1,B256) B256(B) (3) --> (2,B256)" is minimal for the vertext (2,B256) so that the edge "(1,ASCII) B256(B) (4) --> (2,B256)" is removed.
     *
     * Situation after adding edges to vertices at position 2
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII)
     * (0,ASCII) B256(A) (3) --> (1,B256)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF)
     * (0,ASCII) C40(ABC) (3) --> (3,C40)
     * (0,ASCII) TEXT(ABC) (5) --> (3,TEXT)
     * (0,ASCII) X12(ABC) (3) --> (3,X12)
     * (0,ASCII) EDF(ABC) (4) --> (3,EDF)
     * (0,ASCII) EDF(ABCD) (4) --> (4,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) EDF(BC) (5) --> (3,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) C40(BCD) (4) --> (4,C40)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) TEXT(BCD) (6) --> (4,TEXT)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) X12(BCD) (4) --> (4,X12)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) EDF(BCD) (5) --> (4,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) EDF(BCDE) (5) --> (5,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256)
     * (0,ASCII) B256(A) (3) --> (1,B256) EDF(BC) (6) --> (3,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) C40(BCD) (5) --> (4,C40)
     * (0,ASCII) B256(A) (3) --> (1,B256) TEXT(BCD) (7) --> (4,TEXT)
     * (0,ASCII) B256(A) (3) --> (1,B256) X12(BCD) (5) --> (4,X12)
     * (0,ASCII) B256(A) (3) --> (1,B256) EDF(BCD) (6) --> (4,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) EDF(BCDE) (6) --> (5,EDF)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF) ASCII(C) (5) --> (3,ASCII)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF) B256(C) (6) --> (3,B256)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF) EDF(CD) (7) --> (4,EDF)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF) C40(CDE) (6) --> (5,C40)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF) TEXT(CDE) (8) --> (5,TEXT)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF) X12(CDE) (6) --> (5,X12)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF) EDF(CDE) (7) --> (5,EDF)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF) EDF(CDEF) (7) --> (6,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) B256(C) (5) --> (3,B256)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) EDF(CD) (6) --> (4,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) C40(CDE) (5) --> (5,C40)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) TEXT(CDE) (7) --> (5,TEXT)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) X12(CDE) (5) --> (5,X12)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) EDF(CDE) (6) --> (5,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) EDF(CDEF) (6) --> (6,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) ASCII(C) (4) --> (3,ASCII)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) B256(C) (4) --> (3,B256)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) EDF(CD) (6) --> (4,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) C40(CDE) (5) --> (5,C40)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) TEXT(CDE) (7) --> (5,TEXT)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) X12(CDE) (5) --> (5,X12)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) EDF(CDE) (6) --> (5,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) EDF(CDEF) (6) --> (6,EDF)
     *
     * Edge "(2,ASCII) ASCII(C) (3) --> (3,ASCII)" is minimal for the vertex (3,ASCII) so that edges "(2,EDF) ASCII(C) (5) --> (3,ASCII)" 
     * and "(2,B256) ASCII(C) (4) --> (3,ASCII)" can be removed.
     * Edge "(0,ASCII) EDF(ABC) (4) --> (3,EDF)" is minimal for the vertex (3,EDF) so that edges "(1,ASCII) EDF(BC) (5) --> (3,EDF)" 
     * and "(1,B256) EDF(BC) (6) --> (3,EDF)" can be removed.
     * Edge "(2,B256) B256(C) (4) --> (3,B256)" is minimal for the vertex (3,B256) so that edges "(2,ASCII) B256(C) (5) --> (3,B256)" 
     * and "(2,EDF) B256(C) (6) --> (3,B256)" can be removed.
     *
     * This continues for vertices 3 thru 7
     *
     * Situation after adding edges to vertices at position 7
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII)
     * (0,ASCII) B256(A) (3) --> (1,B256)
     * (0,ASCII) EDF(AB) (4) --> (2,EDF)
     * (0,ASCII) C40(ABC) (3) --> (3,C40)
     * (0,ASCII) TEXT(ABC) (5) --> (3,TEXT)
     * (0,ASCII) X12(ABC) (3) --> (3,X12)
     * (0,ASCII) EDF(ABC) (4) --> (3,EDF)
     * (0,ASCII) EDF(ABCD) (4) --> (4,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) C40(BCD) (4) --> (4,C40)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) TEXT(BCD) (6) --> (4,TEXT)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) X12(BCD) (4) --> (4,X12)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) EDF(BCDE) (5) --> (5,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256)
     * (0,ASCII) C40(ABC) (3) --> (3,C40) C40(DEF) (5) --> (6,C40)
     * (0,ASCII) X12(ABC) (3) --> (3,X12) X12(DEF) (5) --> (6,X12)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) C40(CDE) (5) --> (5,C40)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) TEXT(CDE) (7) --> (5,TEXT)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) X12(CDE) (5) --> (5,X12)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) EDF(CDEF) (6) --> (6,EDF)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) C40(BCD) (4) --> (4,C40) C40(EFG) (6) --> (7,C40)    //Solution 1
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) X12(BCD) (4) --> (4,X12) X12(EFG) (6) --> (7,X12)    //Solution 2
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) B256(C) (4) --> (3,B256)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII) ASCII(D) (4) --> (4,ASCII)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII) TEXT(DEF) (8) --> (6,TEXT)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII) EDF(DEFG) (7) --> (7,EDF)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) B256(C) (4) --> (3,B256) B256(D) (5) --> (4,B256)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII) ASCII(D) (4) --> (4,ASCII) ASCII(E) (5) --> (5,ASCII)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII) ASCII(D) (4) --> (4,ASCII) TEXT(EFG) (9) --> (7,TEXT)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) B256(C) (4) --> (3,B256) B256(D) (5) --> (4,B256) B256(E) (6) --> (5,B256)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII) ASCII(D) (4) --> (4,ASCII) ASCII(E) (5) --> (5,ASCII) ASCII(F) (6) --> (6,ASCII)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) B256(C) (4) --> (3,B256) B256(D) (5) --> (4,B256) B256(E) (6) --> (5,B256) B256(F) (7) --> (6,B256)
     * (0,ASCII) ASCII(A) (1) --> (1,ASCII) ASCII(B) (2) --> (2,ASCII) ASCII(C) (3) --> (3,ASCII) ASCII(D) (4) --> (4,ASCII) ASCII(E) (5) --> (5,ASCII) ASCII(F) (6) --> (6,ASCII) ASCII(G) (7) --> (7,ASCII)
     * (0,ASCII) B256(A) (3) --> (1,B256) B256(B) (3) --> (2,B256) B256(C) (4) --> (3,B256) B256(D) (5) --> (4,B256) B256(E) (6) --> (5,B256) B256(F) (7) --> (6,B256) B256(G) (8) --> (7,B256)
     *
     * Hence a minimal encoding of "ABCDEFG" is either ASCII(A),C40(BCDEFG) or ASCII(A), X12(BCDEFG) with a size of 5 bytes.
     */

    let inputLength = input.length();

    // Array that represents vertices. There is a vertex for every character and Mode::
    // The last dimension in the array below encodes the 6 modes ASCII, C40, TEXT, X12, EDF and B256
    // let edges = new Edge[inputLength + 1][6];
    let edges = vec![vec![None;6];inputLength + 1];
    addEdges(input, &edges, 0, None);

    for i in 1..=inputLength {
    // for (int i = 1; i <= inputLength; i++) {
      for j in 0..6 {
      // for (int j = 0; j < 6; j++) {
        if edges[i][j].is_some() && i < inputLength {
          addEdges(input, &edges, i as u32, edges[i][j]);
        }
      }
      //optimize memory by removing edges that have been passed.
      for j in 0..6 {
      // for (int j = 0; j < 6; j++) {
        edges[i - 1][j] = None;
      }
    }

    let minimalJ:i32 = -1;
    let minimalSize = i32::MAX;
    for j in 0..6 {
    // for (int j = 0; j < 6; j++) {
      if edges[inputLength][j].is_some() {
        let edge = edges[inputLength][j].as_ref().unwrap();
        let size = if j >= 1 && j <= 3  {edge.cachedTotalSize + 1} else {edge.cachedTotalSize}; //C40, TEXT and X12 need an
                                                                                       // extra unlatch at the end
        if (size as i32) < minimalSize {
          minimalSize = size as i32;
          minimalJ = j as i32;
        }
      }
    }

    if minimalJ < 0 {
      return Err(Exceptions::RuntimeException(format!("Internal error: failed to encode \"{}\"",input)));
    }
    Ok(RXingResult::new(edges[inputLength][minimalJ as usize].clone().unwrap()))
  }

  const allCodewordCapacities : [u32;28] = [3, 5, 8, 10, 12, 16, 18, 22, 30, 32, 36, 44, 49, 62, 86, 114,
    144, 174, 204, 280, 368, 456, 576, 696, 816, 1050, 1304, 1558];
const squareCodewordCapacities : [u32;24]= [3, 5, 8, 12, 18, 22, 30, 36, 44, 62, 86, 114, 144, 174, 204,
       280, 368, 456, 576, 696, 816, 1050, 1304, 1558];
const rectangularCodewordCapacities :[u32;6]= [5, 10, 16, 33, 32, 49];

  struct Edge {
    input:Input,
    mode:Mode, //the mode at the start of this edge.
    fromPosition:u32,
    characterLength:u32,
    previous:Rc<Edge>,
    cachedTotalSize:u32,
  }
  impl Edge{

    
    fn new( input:&Input,  mode:Mode,  fromPosition:u32,  characterLength:u32,  previous:Option<Rc<Edge>>) -> Self{
      this.input = input;
      this.mode = mode;
      this.fromPosition = fromPosition;
      this.characterLength = characterLength;
      this.previous = previous;
      assert fromPosition + characterLength <= input.length();

      int size = previous != null ? previous.cachedTotalSize : 0;

      Mode previousMode = getPreviousMode();

     /*
      * Switching modes
      * ASCII -> C40: latch 230
      * ASCII -> TEXT: latch 239
      * ASCII -> X12: latch 238
      * ASCII -> EDF: latch 240
      * ASCII -> B256: latch 231
      * C40 -> ASCII: word(c1,c2,c3), 254
      * TEXT -> ASCII: word(c1,c2,c3), 254
      * X12 -> ASCII: word(c1,c2,c3), 254
      * EDIFACT -> ASCII: Unlatch character,0,0,0 or c1,Unlatch character,0,0 or c1,c2,Unlatch character,0 or 
      * c1,c2,c3,Unlatch character
      * B256 -> ASCII: without latch after n bytes
      */
      switch (mode) {
        case ASCII:
          size++;
          if (input.isECI(fromPosition) || isExtendedASCII(input.charAt(fromPosition), input.getFNC1Character())) {
            size++;
          }
          if (previousMode == Mode::C40 ||
              previousMode == Mode::TEXT ||
              previousMode == Mode::X12) {
            size++; // unlatch 254 to ASCII
          }
          break;
        case B256:
          size++;
          if (previousMode != Mode::B256) {
            size++; //byte count
          } else if (getB256Size() == 250) {
            size++; //extra byte count
          }
          if (previousMode == Mode::ASCII) {
            size++; //latch to B256
          } else if (previousMode == Mode::C40 ||
                     previousMode == Mode::TEXT ||
                     previousMode == Mode::X12) {
            size += 2; //unlatch to ASCII, latch to B256
          }
          break;
        case C40:
        case TEXT:
        case X12:
          if (mode == Mode::X12) {
            size += 2;
          } else {
            int[] charLen = new int[1];
            size += getNumberOfC40Words(input, fromPosition, mode == Mode::C40, charLen) * 2;
          }

          if (previousMode == Mode::ASCII || previousMode == Mode::B256) {
            size++; //additional byte for latch from ASCII to this mode
          } else if (previousMode != mode && (previousMode == Mode::C40 ||
                                             previousMode == Mode::TEXT ||
                                             previousMode == Mode::X12)) {
            size += 2; //unlatch 254 to ASCII followed by latch to this mode
          }
          break;
        case EDF:
          size += 3;
          if (previousMode == Mode::ASCII || previousMode == Mode::B256) {
            size++; //additional byte for latch from ASCII to this mode
          } else if (previousMode == Mode::C40 ||
                    previousMode == Mode::TEXT ||
                    previousMode == Mode::X12) {
            size += 2; //unlatch 254 to ASCII followed by latch to this mode
          }
          break;
      }
      cachedTotalSize = size;
    }

    // does not count beyond 250
    pub fn getB256Size(&self) -> u32{
      int cnt = 0;
      Edge current = this;
      while (current != null && current.mode == Mode::B256 && cnt <= 250) {
        cnt++;
        current = current.previous;
      }
      return cnt;
    }

    pub fn getPreviousStartMode(&self) -> Mode{
      return  previous == null ? Mode::ASCII : previous.mode;
    }

    pub fn  getPreviousMode(&self) -> Mode{
      return  previous == null ? Mode::ASCII : previous.getEndMode();
    }

    /** Returns Mode::ASCII in case that:
     *  - Mode is EDIFACT and characterLength is less than 4 or the remaining characters can be encoded in at most 2
     *    ASCII bytes.
     *  - Mode is C40, TEXT or X12 and the remaining characters can be encoded in at most 1 ASCII byte.
     *  Returns mode in all other cases.
     * */
    pub fn getEndMode(&self)->Mode {
      if (mode == Mode::EDF) {
        if (characterLength < 4) {
          return Mode::ASCII;
        }
        int lastASCII = getLastASCII(); // see 5.2.8.2 EDIFACT encodation Rules
        if (lastASCII > 0 && getCodewordsRemaining(cachedTotalSize + lastASCII) <= 2 - lastASCII) {
          return Mode::ASCII;
        }
      }
      if (mode == Mode::C40 ||
          mode == Mode::TEXT ||
          mode == Mode::X12) {

        // see 5.2.5.2 C40 encodation rules and 5.2.7.2 ANSI X12 encodation rules
        if (fromPosition + characterLength >= input.length() && getCodewordsRemaining(cachedTotalSize) == 0) {
          return Mode::ASCII; 
        }
        int lastASCII = getLastASCII();
        if (lastASCII == 1 && getCodewordsRemaining(cachedTotalSize + 1) == 0) {
          return Mode::ASCII;
        }
      }
      return mode;
    }

    pub fn getMode(&self) -> Mode{
      return mode;
    }

    /** Peeks ahead and returns 1 if the postfix consists of exactly two digits, 2 if the postfix consists of exactly
     *  two consecutive digits and a non extended character or of 4 digits. 
     *  Returns 0 in any other case
     **/
    pub fn getLastASCII(&self) -> u32{
      int length = input.length();
      int from = fromPosition + characterLength;
      if (length - from > 4 || from >= length) {
        return 0;
      }
      if (length - from == 1) {
        if (isExtendedASCII(input.charAt(from), input.getFNC1Character())) {
          return 0;
        }
        return 1;
      }
      if (length - from == 2) {
        if (isExtendedASCII(input.charAt(from), input.getFNC1Character()) || isExtendedASCII(input.charAt(from + 1),
            input.getFNC1Character())) {
          return 0;
        }
        if (high_level_encoder::isDigit(input.charAt(from)) && high_level_encoder::isDigit(input.charAt(from + 1))) {
          return 1;
        }
        return 2;
      }
      if (length - from == 3) {
        if (high_level_encoder::isDigit(input.charAt(from)) && high_level_encoder::isDigit(input.charAt(from + 1))
            && !isExtendedASCII(input.charAt(from + 2), input.getFNC1Character())) {
          return 2;
        }
        if (high_level_encoder::isDigit(input.charAt(from + 1)) && high_level_encoder::isDigit(input.charAt(from + 2))
            && !isExtendedASCII(input.charAt(from), input.getFNC1Character())) {
          return 2;
        }
        return 0;
      }
      if (high_level_encoder::isDigit(input.charAt(from)) && high_level_encoder::isDigit(input.charAt(from + 1))
          && high_level_encoder::isDigit(input.charAt(from + 2)) && high_level_encoder::isDigit(input.charAt(from + 3))) {
        return 2;
      }
      return 0;
    }

    /** Returns the capacity in codewords of the smallest symbol that has enough capacity to fit the given minimal
     * number of codewords.
     **/
    pub fn getMinSymbolSize(&self,  minimum:u32) -> u32{
      switch (input.getShapeHint()) {
        case FORCE_SQUARE:
          for (int capacity : squareCodewordCapacities) {
            if (capacity >= minimum) {
              return capacity;
            }
          }
          break;
        case FORCE_RECTANGLE:
          for (int capacity : rectangularCodewordCapacities) {
            if (capacity >= minimum) {
              return capacity;
            }
          }
          break;
      }
      for (int capacity : allCodewordCapacities) {
        if (capacity >= minimum) {
          return capacity;
        }
      }
      return allCodewordCapacities[allCodewordCapacities.length - 1];
    }

    /** Returns the remaining capacity in codewords of the smallest symbol that has enough capacity to fit the given
     * minimal number of codewords.
     **/
    pub fn getCodewordsRemaining( minimum:u32) -> u32{
      return getMinSymbolSize(minimum) - minimum;
    }

    pub fn getBytes1( c:u32) -> Vec<u8>{
      byte[] result = new byte[1];
      result[0] = (byte) c;
      return result;
    }

     pub fn getBytes2( c1:u32, c2:u32) -> Vec<u8>{
      byte[] result = new byte[2];
      result[0] = (byte) c1;
      result[1] = (byte) c2;
      return result;
    }

    pub fn setC40Word( bytes:&[u8],  offset:u32,  c1:u32,  c2:u32,  c3:u32) {
      int val16 = (1600 * (c1 & 0xff)) + (40 * (c2 & 0xff)) + (c3 & 0xff) + 1;
      bytes[offset] = (byte) (val16 / 256);
      bytes[offset + 1] = (byte) (val16 % 256);
    }

    fn getX12Value( c:char) -> u32{
      return c == 13 ? 0 :
             c == 42 ? 1 :
             c == 62 ? 2 :
             c == 32 ? 3 :
             c >= 48 && c <= 57 ? c - 44 :
             c >= 65 && c <= 90 ? c - 51 : c;
    }

    pub fn getX12Words(&self) -> Vec<u8> {
      assert characterLength % 3 == 0;
      byte[] result = new byte[characterLength / 3 * 2];
      for (int i = 0; i < result.length; i += 2) {
        setC40Word(result,i,getX12Value(input.charAt(fromPosition + i / 2 * 3)),
                              getX12Value(input.charAt(fromPosition + i / 2 * 3 + 1)),
                              getX12Value(input.charAt(fromPosition + i / 2 * 3 + 2)));
      }
      return result;
    }

    pub fn getShiftValue( c:char,  c40:bool,  fnc1:Option<char>) -> u32{
      return (c40 && isInC40Shift1Set(c) ||
             !c40 && isInTextShift1Set(c)) ? 0 :
             (c40 && isInC40Shift2Set(c, fnc1) ||
             !c40 && isInTextShift2Set(c, fnc1)) ? 1 : 2;
    }

    fn getC40Value( c40:bool,  setIndex:u32,  c:char, fnc1:Option<char>) -> u32{
      if (c == fnc1) {
        assert setIndex ==  2;
        return  27;
      }
      if (c40) {
        return c <= 31 ? c :
               c == 32 ? 3 :
               c <= 47 ? c - 33 :
               c <= 57 ? c - 44 :
               c <= 64 ? c - 43 :
               c <= 90 ? c - 51 :
               c <= 95 ? c - 69 :
               c <= 127 ? c - 96 : c;
      } else {
        return c == 0 ? 0 :
               setIndex == 0 && c <= 3 ? c - 1 : //is this a bug in the spec?
               setIndex == 1 && c <= 31 ? c :
               c == 32 ? 3 :
               c >= 33 && c <= 47 ? c - 33 :
               c >= 48 && c <= 57 ? c - 44 :
               c >= 58 && c <= 64 ? c - 43 :
               c >= 65 && c <= 90 ? c - 64 :
               c >= 91 && c <= 95 ? c - 69 :
               c == 96 ? 0 :
               c >= 97 && c <= 122 ? c - 83 :
               c >= 123 && c <= 127 ? c - 96 : c;
      }
    }

    pub fn getC40Words(&self,  c40:bool,  fnc1:Option<char>) -> Vec<u8>{
      List<Byte> c40Values = new ArrayList<>();
      for (int i = 0; i < characterLength; i++) {
        char ci = input.charAt(fromPosition + i);
        if (c40 && high_level_encoder::isNativeC40(ci) || !c40 && high_level_encoder::isNativeText(ci)) {
          c40Values.add((byte) getC40Value(c40, 0, ci, fnc1));
        } else if (!isExtendedASCII(ci, fnc1)) {
          int shiftValue = getShiftValue(ci, c40, fnc1);
          c40Values.add((byte) shiftValue); //Shift[123]
          c40Values.add((byte) getC40Value(c40, shiftValue, ci, fnc1));
        } else {
          char asciiValue = (char) ((ci & 0xff) - 128);
          if (c40 && high_level_encoder::isNativeC40(asciiValue) ||
              !c40 && high_level_encoder::isNativeText(asciiValue)) {
            c40Values.add((byte) 1); //Shift 2
            c40Values.add((byte) 30); //Upper Shift
            c40Values.add((byte) getC40Value(c40, 0, asciiValue, fnc1));
          } else {
            c40Values.add((byte) 1); //Shift 2
            c40Values.add((byte) 30); //Upper Shift
            int shiftValue = getShiftValue(asciiValue, c40, fnc1);
            c40Values.add((byte) shiftValue); // Shift[123]
            c40Values.add((byte) getC40Value(c40, shiftValue, asciiValue, fnc1));
          }
        }
      }

      if ((c40Values.size() % 3) != 0) {
        assert (c40Values.size() - 2) % 3 == 0 && fromPosition + characterLength == input.length();
        c40Values.add((byte) 0); // pad with 0 (Shift 1)
      }

      byte[] result = new byte[c40Values.size() / 3 * 2];
      int byteIndex = 0;
      for (int i = 0; i < c40Values.size(); i += 3) {
        setC40Word(result,byteIndex, c40Values.get(i) & 0xff, c40Values.get(i + 1) & 0xff, c40Values.get(i + 2) & 0xff);
        byteIndex += 2;
      }
      return result;
    }

    pub fn getEDFBytes(&self) -> Vec<u8> {
      int numberOfThirds = (int) Math.ceil(characterLength / 4.0);
      byte[] result = new byte[numberOfThirds * 3];
      int pos = fromPosition;
      int endPos = Math.min(fromPosition + characterLength - 1 , input.length() - 1);
      for (int i = 0; i < numberOfThirds; i += 3) {
        int[] edfValues = new int[4];
        for (int j = 0; j < 4; j++) {
          if (pos <= endPos) {
            edfValues[j] = input.charAt(pos++) & 0x3f;
          } else {
            edfValues[j] = pos == endPos + 1 ? 0x1f : 0;
          }
        }
        int val24 = edfValues[0] << 18;
        val24 |= edfValues[1] << 12;
        val24 |= edfValues[2] << 6;
        val24 |= edfValues[3];
        result[i] = (byte) ((val24 >> 16) & 0xff);
        result[i + 1] = (byte) ((val24 >> 8) & 0xff);
        result[i + 2] = (byte) (val24 & 0xff);
      }
      return result;
    }

    pub fn getLatchBytes(&self) -> Vec<u8> {
      switch (getPreviousMode()) {
        case ASCII:
        case B256: //after B256 ends (via length) we are back to ASCII
          switch (mode) {
            case B256:
              return getBytes(231);
            case C40:
              return getBytes(230);
            case TEXT:
              return getBytes(239);
            case X12:
              return getBytes(238);
            case EDF:
              return getBytes(240);
          }
          break;
        case C40:
        case TEXT:
        case X12:
          if (mode != getPreviousMode()) {
            switch (mode) {
              case ASCII:
                return getBytes(254);
              case B256:
                return getBytes(254, 231);
              case C40:
                return getBytes(254, 230);
              case TEXT:
                return getBytes(254, 239);
              case X12:
                return getBytes(254, 238);
              case EDF:
                return getBytes(254, 240);
            }
          }
          break;
        case EDF:
          assert mode == Mode::EDF; //The rightmost EDIFACT edge always contains an unlatch character
          break;
      }
      return new byte[0];
    }

    // Important: The function does not return the length bytes (one or two) in case of B256 encoding
    pub fn getDataBytes(&self) -> Vec<u8> {
      switch (mode) {
        case ASCII:
          if (input.isECI(fromPosition)) {
            return getBytes(241,input.getECIValue(fromPosition) + 1);
          } else if (isExtendedASCII(input.charAt(fromPosition), input.getFNC1Character())) {
            return getBytes(235,input.charAt(fromPosition) - 127);
          } else if (characterLength == 2) {
            return getBytes((input.charAt(fromPosition) - '0') * 10 + input.charAt(fromPosition + 1) - '0' + 130);
          } else if (input.isFNC1(fromPosition)) {
            return getBytes(232);
          } else {
            return getBytes(input.charAt(fromPosition) + 1);
          }
        case B256:
          return getBytes(input.charAt(fromPosition));
        case C40:
          return getC40Words(true, input.getFNC1Character());
        case TEXT:
          return getC40Words(false, input.getFNC1Character());
        case X12:
          return getX12Words();
        case EDF:
          return getEDFBytes();
      }
      assert false;
      return new byte[0];
    }
  }

  struct RXingResult {

    bytes:Vec<u8>,
  }
  impl RXingResult{

    pub fn new( solution:Rc<Edge>) -> Self {
      let input = solution.input;
      let size = 0;
      let bytesAL = new ArrayList<>();
      let randomizePostfixLength = new ArrayList<>();
      let randomizeLengths = new ArrayList<>();
      if ((solution.mode == Mode::C40 ||
           solution.mode == Mode::TEXT ||
           solution.mode == Mode::X12) &&
           solution.getEndMode() != Mode::ASCII) {
        size += prepend(Edge::getBytes(254),bytesAL);
      }
      let current = solution;
      while (current != null) {
        size += prepend(current.getDataBytes(),bytesAL);

        if (current.previous == null || current.getPreviousStartMode() != current.getMode()) {
          if (current.getMode() == Mode::B256) {
            if (size <= 249) {
              bytesAL.add(0, (byte) size);
              size++;
            } else {
              bytesAL.add(0, (byte) (size % 250));
              bytesAL.add(0, (byte) (size / 250 + 249));
              size += 2;
            }
            randomizePostfixLength.add(bytesAL.size());
            randomizeLengths.add(size);
          }
          prepend(current.getLatchBytes(), bytesAL);
          size = 0;
        }

        current = current.previous;
      }
      if (input.getMacroId() == 5) {
        size += prepend(MinimalEncoder.Edge.getBytes(236), bytesAL);
      } else if (input.getMacroId() == 6) {
        size += prepend(MinimalEncoder.Edge.getBytes(237), bytesAL);
      }
   
      if (input.getFNC1Character() > 0) {
        size += prepend(MinimalEncoder.Edge.getBytes(232), bytesAL);
      }
      for (int i = 0; i < randomizePostfixLength.size(); i++) {
        applyRandomPattern(bytesAL,bytesAL.size() - randomizePostfixLength.get(i), randomizeLengths.get(i));
      }
      //add padding
      int capacity = solution.getMinSymbolSize(bytesAL.size());
      if (bytesAL.size() < capacity) {
        bytesAL.add((byte) 129);
      }
      while (bytesAL.size() < capacity) {
        bytesAL.add((byte) randomize253State(bytesAL.size() + 1));
      }

      bytes = new byte[bytesAL.size()];
      for (int i = 0; i < bytes.length; i++) {
        bytes[i] = bytesAL.get(i);
      }
    }

    pub fn prepend(bytes:&[u8],  into:&[u8]) -> u32{
      for (int i = bytes.length - 1; i >= 0; i--) {
        into.add(0, bytes[i]);
      }
      return bytes.length;
    }

    fn randomize253State( codewordPosition:u32) -> u32{
      let pseudoRandom = ((149 * codewordPosition) % 253) + 1;
      let tempVariable = 129 + pseudoRandom;
      return tempVariable <= 254 ? tempVariable : tempVariable - 254;
    }

    pub fn applyRandomPattern(bytesAL:&[u8], startPosition:u32,  length:u32) {
      for i in 0..length {
      // for (int i = 0; i < length; i++) {
        //See "B.1 253-state algorithm
        let Pad_codeword_position = startPosition + i;
        let Pad_codeword_value = bytesAL.get(Pad_codeword_position) & 0xff;
        let pseudo_random_number = ((149 * (Pad_codeword_position + 1)) % 255) + 1;
        let temp_variable = Pad_codeword_value + pseudo_random_number;
        bytesAL.set(Pad_codeword_position, (byte) (temp_variable <= 255 ? temp_variable : temp_variable - 256));
      }
    }

    pub fn getBytes(&self) -> &[u8] {
      &self.bytes
    }

  }

  struct Input  {
    shape: SymbolShapeHint,
     macroId:i32,
    internal: MinimalECIInput
  }

  impl Input{

    pub fn new( stringToEncode:&str,  priorityCharset:Option<EncodingRef>,  fnc1:Option<char>,  shape:SymbolShapeHint,  macroId:i32) -> Self{
      Self {
        shape,
        macroId,
        internal: MinimalECIInput::new(stringToEncode, priorityCharset, if fnc1 >= 0 {Some(&(fnc1 as u8 as char).to_string())} else {None})
    }
      // super(stringToEncode, priorityCharset, fnc1);
      // this.shape = shape;
      // this.macroId = macroId;
    }

    pub fn getMacroId(&self) -> i32{
      self.macroId
    }

    pub fn getShapeHint(&self) -> SymbolShapeHint{
      self.shape
    }

    pub fn length(&self) -> usize {
       self.internal.length()
  }
  pub fn isECI(&self, index: u32) -> Result<bool, Exceptions> {
    self.internal.isECI(index)
}
pub fn charAt(&self, index: usize) -> Result<char, Exceptions> {
  self.internal.charAt(index)
}
pub fn getFNC1Character(&self) -> char {
  self.internal.getFNC1Character() as u8 as char
  }
  fn haveNCharacters(&self, index: usize, n: usize) -> bool {
    self.internal.haveNCharacters(index, n)
  }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.internal.fmt(f)
    }
}