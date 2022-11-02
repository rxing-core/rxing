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

// package com.google.zxing.common;

// import java.nio.charset.Charset;
// import java.util.ArrayList;
// import java.util.List;

use std::{fmt, rc::Rc};

use encoding::EncodingRef;
use unicode_segmentation::UnicodeSegmentation;

use crate::Exceptions;

use super::{ECIEncoderSet, ECIInput};

//* approximated (latch + 2 codewords)
pub const COST_PER_ECI: usize = 3;

/**
 * Class that converts a character string into a sequence of ECIs and bytes
 *
 * The implementation uses the Dijkstra algorithm to produce minimal encodings
 *
 * @author Alex Geller
 */
pub struct MinimalECIInput {
    bytes: Vec<u16>,
    fnc1: u16,
}

impl ECIInput for MinimalECIInput {
    /**
     * Returns the length of this input.  The length is the number
     * of {@code byte}s, FNC1 characters or ECIs in the sequence.
     *
     * @return  the number of {@code char}s in this sequence
     */
    fn length(&self) -> usize {
        return self.bytes.len();
    }

    /**
     * Returns the {@code byte} value at the specified index.  An index ranges from zero
     * to {@code length() - 1}.  The first {@code byte} value of the sequence is at
     * index zero, the next at index one, and so on, as for array
     * indexing.
     *
     * @param   index the index of the {@code byte} value to be returned
     *
     * @return  the specified {@code byte} value as character or the FNC1 character
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     * @throws  IllegalArgumentException
     *          if the value at the {@code index} argument is an ECI (@see #isECI)
     */
    fn charAt(&self, index: usize) -> Result<char, Exceptions> {
        if index >= self.length() {
            return Err(Exceptions::IndexOutOfBoundsException(index.to_string()));
        }
        if self.isECI(index as u32)? {
            return Err(Exceptions::IllegalArgumentException(format!(
                "value at {} is not a character but an ECI",
                index
            )));
        }
        if self.isFNC1(index)? {
            Ok(self.fnc1 as u8 as char)
        } else {
            Ok(self.bytes[index] as u8 as char)
        }
    }

    /**
     * Returns a {@code CharSequence} that is a subsequence of this sequence.
     * The subsequence starts with the {@code char} value at the specified index and
     * ends with the {@code char} value at index {@code end - 1}.  The length
     * (in {@code char}s) of the
     * returned sequence is {@code end - start}, so if {@code start == end}
     * then an empty sequence is returned.
     *
     * @param   start   the start index, inclusive
     * @param   end     the end index, exclusive
     *
     * @return  the specified subsequence
     *
     * @throws  IndexOutOfBoundsException
     *          if {@code start} or {@code end} are negative,
     *          if {@code end} is greater than {@code length()},
     *          or if {@code start} is greater than {@code end}
     * @throws  IllegalArgumentException
     *          if a value in the range {@code start}-{@code end} is an ECI (@see #isECI)
     */
    fn subSequence(&self, start: usize, end: usize) -> Result<Vec<char>, Exceptions> {
        if start > end || end > self.length() {
            return Err(Exceptions::IndexOutOfBoundsException(start.to_string()));
        }
        let mut result = String::new();
        for i in start..end {
            //   for (int i = start; i < end; i++) {
            if self.isECI(i as u32)? {
                return Err(Exceptions::IllegalArgumentException(format!(
                    "value at {} is not a character but an ECI",
                    i
                )));
            }
            result.push_str(&self.charAt(i)?.to_string());
        }
        Ok(result.chars().collect())
    }

    /**
     * Determines if a value is an ECI
     *
     * @param   index the index of the value
     *
     * @return  true if the value at position {@code index} is an ECI
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     */
    fn isECI(&self, index: u32) -> Result<bool, Exceptions> {
        if index >= self.length() as u32 {
            return Err(Exceptions::IndexOutOfBoundsException(index.to_string()));
        }
        Ok(self.bytes[index as usize] > 255) // && self.bytes[index as usize] <= u16::MAX)
    }

    /**
     * Returns the {@code int} ECI value at the specified index.  An index ranges from zero
     * to {@code length() - 1}.  The first {@code byte} value of the sequence is at
     * index zero, the next at index one, and so on, as for array
     * indexing.
     *
     * @param   index the index of the {@code int} value to be returned
     *
     * @return  the specified {@code int} ECI value.
     *          The ECI specified the encoding of all bytes with a higher index until the
     *          next ECI or until the end of the input if no other ECI follows.
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     * @throws  IllegalArgumentException
     *          if the value at the {@code index} argument is not an ECI (@see #isECI)
     */
    fn getECIValue(&self, index: usize) -> Result<u32, Exceptions> {
        if index >= self.length() {
            return Err(Exceptions::IndexOutOfBoundsException(index.to_string()));
        }
        if !self.isECI(index as u32)? {
            return Err(Exceptions::IllegalArgumentException(format!(
                "value at {} is not an ECI but a character",
                index
            )));
        }
        Ok((self.bytes[index] as u32 - 256) as u32)
    }

    fn haveNCharacters(&self, index: usize, n: usize) -> bool {
        if index + n - 1 >= self.bytes.len() {
            return false;
        }
        for i in 0..n {
            //   for (int i = 0; i < n; i++) {
            if self.isECI(index as u32 + i as u32).unwrap() {
                return false;
            }
        }
        return true;
    }
}
impl MinimalECIInput {
    /**
     * Constructs a minimal input
     *
     * @param stringToEncode the character string to encode
     * @param priorityCharset The preferred {@link Charset}. When the value of the argument is null, the algorithm
     *   chooses charsets that leads to a minimal representation. Otherwise the algorithm will use the priority
     *   charset to encode any character in the input that can be encoded by it if the charset is among the
     *   supported charsets.
     * @param fnc1 denotes the character in the input that represents the FNC1 character or -1 if this is not GS1
     *   input.
     */
    pub fn new(
        stringToEncodeInput: &str,
        priorityCharset: Option<EncodingRef>,
        fnc1: Option<&str>,
    ) -> Self {
        let stringToEncode = stringToEncodeInput.graphemes(true).collect::<Vec<&str>>();
        let encoderSet = ECIEncoderSet::new(stringToEncodeInput, priorityCharset, fnc1);
        let bytes = if encoderSet.len() == 1 {
            //optimization for the case when all can be encoded without ECI in ISO-8859-1
            let mut bytes_hld = vec![0; stringToEncode.len()];
            for i in 0..stringToEncode.len() {
                //   for (int i = 0; i < bytes.length; i++) {
                let c = stringToEncode.get(i).unwrap();
                bytes_hld[i] = if fnc1.is_some() && c == fnc1.as_ref().unwrap() {
                    1000
                } else {
                    c.chars().nth(0).unwrap() as u16
                };
            }
            bytes_hld
        } else {
            Self::encodeMinimally(
                stringToEncodeInput,
                &encoderSet,
                fnc1
            )
        };

        Self {
            bytes: bytes,
            fnc1: if let Some(fnc1_exists) = fnc1 {
                //}.as_ref().unwrap().chars().nth(0).unwrap() as u16,
                fnc1_exists.chars().nth(0).unwrap() as u16
            } else {
                1000
            },
        }
    }

    pub fn getFNC1Character(&self) -> u16 {
        self.fnc1
    }

    /**
     * Determines if a value is the FNC1 character
     *
     * @param   index the index of the value
     *
     * @return  true if the value at position {@code index} is the FNC1 character
     *
     * @throws  IndexOutOfBoundsException
     *          if the {@code index} argument is negative or not less than
     *          {@code length()}
     */
    pub fn isFNC1(&self, index: usize) -> Result<bool, Exceptions> {
        if index >= self.length() {
            return Err(Exceptions::IndexOutOfBoundsException(index.to_string()));
        }
        Ok(self.bytes[index] == 1000)
    }

    fn addEdge(edges: &mut Vec<Vec<Option<Rc<InputEdge>>>>, to: usize, edge: Rc<InputEdge>) {
        if edges[to][edge.encoderIndex].is_none()
            || edges[to][edge.encoderIndex]
                .clone()
                .unwrap()
                .cachedTotalSize
                > edge.cachedTotalSize
        {
            edges[to][edge.encoderIndex] = Some(edge.clone());
        }
    }

    fn addEdges(
        stringToEncode: &str,
        encoderSet: &ECIEncoderSet,
        edges: &mut Vec<Vec<Option<Rc<InputEdge>>>>,
        from: usize,
        previous: Option<Rc<InputEdge>>,
        fnc1: Option<&str>,
    ) {
        // let ch = stringToEncode.chars().nth(from).unwrap() as i16;
        let ch = stringToEncode.graphemes(true).nth(from).unwrap();

        let mut start = 0;
        let mut end = encoderSet.len();
        if let Some(fnc1) = fnc1 {
        if encoderSet.getPriorityEncoderIndex().is_some()
            && (ch.chars().nth(0).unwrap() == fnc1.chars().nth(1).unwrap()
                || encoderSet.canEncode(ch, encoderSet.getPriorityEncoderIndex().unwrap()))
        {
            start = encoderSet.getPriorityEncoderIndex().unwrap();
            end = start + 1;
        }}

        for i in start..end {
            // for (int i = start; i < end; i++) {
            if (fnc1.is_some() && ch.chars().nth(0).unwrap() == fnc1.as_ref().unwrap().chars().nth(0).unwrap()) || encoderSet.canEncode(ch, i) {
                Self::addEdge(
                    edges,
                    from + 1,
                    Rc::new(InputEdge::new(ch, encoderSet, i, previous.clone(), fnc1)),
                );
            }
        }
    }

    pub fn encodeMinimally(
        stringToEncode: &str,
        encoderSet: &ECIEncoderSet,
        fnc1: Option<&str>,
    ) -> Vec<u16> {
        // let inputLength = stringToEncode.chars().count();
        let inputLength = stringToEncode.graphemes(true).count();

        // Array that represents vertices. There is a vertex for every character and encoding.
        let mut edges = vec![vec![None; encoderSet.len()]; inputLength + 1]; //InputEdge[inputLength + 1][encoderSet.length()];
        Self::addEdges(stringToEncode, encoderSet, &mut edges, 0, None, fnc1);

        for i in 1..=inputLength {
            // for (int i = 1; i <= inputLength; i++) {
            for j in 0..encoderSet.len() {
                //   for (int j = 0; j < encoderSet.length(); j++) {
                if edges[i][j].is_some() && i < inputLength {
                    let edg = edges[i][j].clone();
                    Self::addEdges(stringToEncode, encoderSet, &mut edges, i, edg, fnc1);
                }
            }
            //optimize memory by removing edges that have been passed.
            for j in 0..encoderSet.len() {
                //   for (int j = 0; j < encoderSet.length(); j++) {
                edges[i - 1][j] = None;
            }
        }
        let mut minimalJ: i32 = -1;
        let mut minimalSize: i32 = i32::MAX;
        for j in 0..encoderSet.len() {
            // for (int j = 0; j < encoderSet.length(); j++) {
            if edges[inputLength][j].is_some() {
                let edge = edges[inputLength][j].clone().unwrap();
                if (edge.cachedTotalSize as i32) < minimalSize {
                    minimalSize = edge.cachedTotalSize as i32;
                    minimalJ = j as i32;
                }
            }
        }
        if minimalJ < 0 {
            panic!("Internal error: failed to encode \"{}\"", stringToEncode);
        }
        let mut intsAL: Vec<u16> = Vec::new();
        let mut current = edges[inputLength][minimalJ as usize].clone();
        while current.is_some() {
            let c = current.unwrap().clone();
            if c.isFNC1() {
                intsAL.splice(0..0, [1000]);
            } else {
                let bytes: Vec<u16> = encoderSet
                    .encode_char(&c.c, c.encoderIndex)
                    .iter()
                    .map(|x| *x as u16)
                    .collect();
                let mut i = bytes.len() as i32 - 1;
                while i >= 0 {
                    // for (int i = bytes.length - 1; i >= 0; i--) {
                    intsAL.splice(0..0, [bytes[i as usize]]);
                    i = -1;
                }
            }
            let previousEncoderIndex = if c.previous.is_none() {
                0
            } else {
                c.previous.clone().unwrap().encoderIndex
            };
            if previousEncoderIndex != c.encoderIndex {
                intsAL.splice(
                    0..0,
                    [256 as u16 + encoderSet.getECIValue(c.encoderIndex) as u16],
                );
            }
            current = c.previous.clone();
        }
        let mut ints = vec![0; intsAL.len()];
        for i in 0..ints.len() {
            // for (int i = 0; i < ints.length; i++) {
            ints[i] = *intsAL.get(i).unwrap() as u16;
        }
        return ints;
    }
}

struct InputEdge {
    c: String,
    encoderIndex: usize, //the encoding of this edge
    previous: Option<Rc<InputEdge>>,
    cachedTotalSize: usize,
}
impl InputEdge {
    pub fn new(
        c: &str,
        encoderSet: &ECIEncoderSet,
        encoderIndex: usize,
        previous: Option<Rc<InputEdge>>,
        fnc1: Option<&str>,
    ) -> Self {
        let mut size = if c == "\u{1000}" {
            1
        } else {
            encoderSet.encode_char(c, encoderIndex).len()
        };

        //let fnc1Str = String::from_utf16(&[fnc1]).unwrap();

        if let Some(prev) = previous {
            let previousEncoderIndex = prev.encoderIndex;
            if previousEncoderIndex != encoderIndex {
                size += COST_PER_ECI;
            }
            size += prev.cachedTotalSize;

            Self {
                c: if fnc1.is_some() &&  &c == fnc1.as_ref().unwrap() {
                    String::from("\u{1000}")
                } else {
                    String::from(c)
                },
                encoderIndex,
                previous: Some(prev.clone()),
                cachedTotalSize: size,
            }
        } else {
            let previousEncoderIndex = 0;
            if previousEncoderIndex != encoderIndex {
                size += COST_PER_ECI;
            }

            Self {
                c: if fnc1.is_some() &&  &c == fnc1.as_ref().unwrap() {
                    String::from("\u{1000}")
                } else {
                    String::from(c)
                },
                encoderIndex,
                previous: None,
                cachedTotalSize: size,
            }
        }

        //   int size = this.c == 1000 ? 1 : encoderSet.encode(c, encoderIndex).length;
        // let previousEncoderIndex = if previous.is_none() {
        //     0
        // } else {
        //     previous.unwrap().encoderIndex
        // };
        //   int previousEncoderIndex = previous == null ? 0 : previous.encoderIndex;
        // if previousEncoderIndex != encoderIndex {
        //     size += COST_PER_ECI;
        // }
        // if prev_is_some {
        //     size += previous.unwrap().cachedTotalSize;
        // }

        // Self {
        //     c: if c == fnc1 { 1000 as char } else { c },
        //     encoderIndex,
        //     previous: previous,
        //     cachedTotalSize: size,
        // }
        //   this.c = c == fnc1 ? 1000 : c;
        //   this.encoderIndex = encoderIndex;
        //   this.previous = previous;
        //   this.cachedTotalSize = size;
    }

    pub fn isFNC1(&self) -> bool {
        self.c == "\u{1000}"
    }
}

impl fmt::Display for MinimalECIInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();
        for i in 0..self.length() {
            // for (int i = 0; i < length(); i++) {
            if i > 0 {
                result.push_str(", ");
            }
            if self.isECI(i as u32).unwrap() {
                result.push_str("ECI(");
                result.push_str(&self.getECIValue(i).unwrap().to_string());
                result.push(')');
            } else if (self.charAt(i).unwrap() as u8) < 128 {
                result.push('\'');
                result.push(self.charAt(i).unwrap());
                result.push('\'');
            } else {
                result.push(self.charAt(i).unwrap());
            }
        }
        write!(f, "{}", result)
    }
}
