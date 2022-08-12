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
// package com::google::zxing::common;

/**
 * Class that converts a character string into a sequence of ECIs and bytes
 *
 * The implementation uses the Dijkstra algorithm to produce minimal encodings
 *
 * @author Alex Geller
 */

// approximated (latch + 2 codewords)
 const COST_PER_ECI: i32 = 3;
#[derive(ECIInput)]
pub struct MinimalECIInput {

     let mut bytes: Vec<i32>;

     let fnc1: i32;
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
    pub fn new( string_to_encode: &String,  priority_charset: &Charset,  fnc1: i32) -> MinimalECIInput {
        let .fnc1 = fnc1;
         let encoder_set: ECIEncoderSet = ECIEncoderSet::new(&string_to_encode, &priority_charset, fnc1);
        if encoder_set.length() == 1 {
            //optimization for the case when all can be encoded without ECI in ISO-8859-1
            bytes = : [i32; string_to_encode.length()] = [0; string_to_encode.length()];
             {
                 let mut i: i32 = 0;
                while i < bytes.len() {
                    {
                         let c: char = string_to_encode.char_at(i);
                        bytes[i] =  if c == fnc1 { 1000 } else { c as i32 };
                    }
                    i += 1;
                 }
             }

        } else {
            bytes = ::encode_minimally(&string_to_encode, encoder_set, fnc1);
        }
    }

    pub fn  get_f_n_c1_character(&self) -> i32  {
        return self.fnc1;
    }

    /**
  * Returns the length of this input.  The length is the number
  * of {@code byte}s, FNC1 characters or ECIs in the sequence.
  *
  * @return  the number of {@code char}s in this sequence
  */
    pub fn  length(&self) -> i32  {
        return self.bytes.len();
    }

    pub fn  have_n_characters(&self,  index: i32,  n: i32) -> bool  {
        if index + n - 1 >= self.bytes.len() {
            return false;
        }
         {
             let mut i: i32 = 0;
            while i < n {
                {
                    if self.is_e_c_i(index + i) {
                        return false;
                    }
                }
                i += 1;
             }
         }

        return true;
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
    pub fn  char_at(&self,  index: i32) -> char  {
        if index < 0 || index >= self.length() {
            throw IndexOutOfBoundsException::new(format!("{}", index));
        }
        if self.is_e_c_i(index) {
            throw IllegalArgumentException::new(format!("value at {} is not a character but an ECI", index));
        }
        return  if self.is_f_n_c1(index) { self.fnc1 as char } else { self.bytes[index] as char };
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
    pub fn  sub_sequence(&self,  start: i32,  end: i32) -> CharSequence  {
        if start < 0 || start > end || end > self.length() {
            throw IndexOutOfBoundsException::new(format!("{}", start));
        }
         let result: StringBuilder = StringBuilder::new();
         {
             let mut i: i32 = start;
            while i < end {
                {
                    if self.is_e_c_i(i) {
                        throw IllegalArgumentException::new(format!("value at {} is not a character but an ECI", i));
                    }
                    result.append(&self.char_at(i));
                }
                i += 1;
             }
         }

        return result;
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
    pub fn  is_e_c_i(&self,  index: i32) -> bool  {
        if index < 0 || index >= self.length() {
            throw IndexOutOfBoundsException::new(format!("{}", index));
        }
        return self.bytes[index] > 255 && self.bytes[index] <= 999;
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
    pub fn  is_f_n_c1(&self,  index: i32) -> bool  {
        if index < 0 || index >= self.length() {
            throw IndexOutOfBoundsException::new(format!("{}", index));
        }
        return self.bytes[index] == 1000;
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
    pub fn  get_e_c_i_value(&self,  index: i32) -> i32  {
        if index < 0 || index >= self.length() {
            throw IndexOutOfBoundsException::new(format!("{}", index));
        }
        if !self.is_e_c_i(index) {
            throw IllegalArgumentException::new(format!("value at {} is not an ECI but a character", index));
        }
        return self.bytes[index] - 256;
    }

    pub fn  to_string(&self) -> String  {
         let result: StringBuilder = StringBuilder::new();
         {
             let mut i: i32 = 0;
            while i < self.length() {
                {
                    if i > 0 {
                        result.append(", ");
                    }
                    if self.is_e_c_i(i) {
                        result.append("ECI(");
                        result.append(&self.get_e_c_i_value(i));
                        result.append(')');
                    } else if self.char_at(i) < 128 {
                        result.append('\'');
                        result.append(&self.char_at(i));
                        result.append('\'');
                    } else {
                        result.append(self.char_at(i) as i32);
                    }
                }
                i += 1;
             }
         }

        return result.to_string();
    }

    fn  add_edge( edges: &Vec<Vec<InputEdge>>,  to: i32,  edge: &InputEdge)   {
        if edges[to][edge.encoderIndex] == null || edges[to][edge.encoderIndex].cachedTotalSize > edge.cachedTotalSize {
            edges[to][edge.encoderIndex] = edge;
        }
    }

    fn  add_edges( string_to_encode: &String,  encoder_set: &ECIEncoderSet,  edges: &Vec<Vec<InputEdge>>,  from: i32,  previous: &InputEdge,  fnc1: i32)   {
         let ch: char = string_to_encode.char_at(from);
         let mut start: i32 = 0;
         let mut end: i32 = encoder_set.length();
        if encoder_set.get_priority_encoder_index() >= 0 && (ch == fnc1 || encoder_set.can_encode(ch, &encoder_set.get_priority_encoder_index())) {
            start = encoder_set.get_priority_encoder_index();
            end = start + 1;
        }
         {
             let mut i: i32 = start;
            while i < end {
                {
                    if ch == fnc1 || encoder_set.can_encode(ch, i) {
                        ::add_edge(edges, from + 1, InputEdge::new(ch, encoder_set, i, previous, fnc1));
                    }
                }
                i += 1;
             }
         }

    }

    fn  encode_minimally( string_to_encode: &String,  encoder_set: &ECIEncoderSet,  fnc1: i32) -> Vec<i32>  {
         let input_length: i32 = string_to_encode.length();
        // Array that represents vertices. There is a vertex for every character and encoding.
         let mut edges: [[Option<InputEdge>; encoder_set.length()]; input_length + 1] = [[None; encoder_set.length()]; input_length + 1];
        ::add_edges(&string_to_encode, encoder_set, edges, 0, null, fnc1);
         {
             let mut i: i32 = 1;
            while i <= input_length {
                {
                     {
                         let mut j: i32 = 0;
                        while j < encoder_set.length() {
                            {
                                if edges[i][j] != null && i < input_length {
                                    ::add_edges(&string_to_encode, encoder_set, edges, i, edges[i][j], fnc1);
                                }
                            }
                            j += 1;
                         }
                     }

                    //optimize memory by removing edges that have been passed.
                     {
                         let mut j: i32 = 0;
                        while j < encoder_set.length() {
                            {
                                edges[i - 1][j] = null;
                            }
                            j += 1;
                         }
                     }

                }
                i += 1;
             }
         }

         let minimal_j: i32 = -1;
         let minimal_size: i32 = Integer::MAX_VALUE;
         {
             let mut j: i32 = 0;
            while j < encoder_set.length() {
                {
                    if edges[input_length][j] != null {
                         let edge: InputEdge = edges[input_length][j];
                        if edge.cachedTotalSize < minimal_size {
                            minimal_size = edge.cachedTotalSize;
                            minimal_j = j;
                        }
                    }
                }
                j += 1;
             }
         }

        if minimal_j < 0 {
            throw RuntimeException::new(format!("Internal error: failed to encode \"{}\"", string_to_encode));
        }
         let ints_a_l: List<Integer> = ArrayList<>::new();
         let mut current: InputEdge = edges[input_length][minimal_j];
        while current != null {
            if current.is_f_n_c1() {
                ints_a_l.add(0, 1000);
            } else {
                 let bytes: Vec<i8> = encoder_set.encode(current.c, current.encoderIndex);
                 {
                     let mut i: i32 = bytes.len() - 1;
                    while i >= 0 {
                        {
                            ints_a_l.add(0, (bytes[i] & 0xFF));
                        }
                        i -= 1;
                     }
                 }

            }
             let previous_encoder_index: i32 =  if current.previous == null { 0 } else { current.previous.encoderIndex };
            if previous_encoder_index != current.encoderIndex {
                ints_a_l.add(0, 256 + encoder_set.get_e_c_i_value(current.encoderIndex));
            }
            current = current.previous;
        }
         let mut ints: [i32; ints_a_l.size()] = [0; ints_a_l.size()];
         {
             let mut i: i32 = 0;
            while i < ints.len() {
                {
                    ints[i] = ints_a_l.get(i);
                }
                i += 1;
             }
         }

        return ints;
    }

    struct InputEdge {

         let c: char;

        //the encoding of this edge
         let encoder_index: i32;

         let previous: InputEdge;

         let cached_total_size: i32;
    }
    
    impl InputEdge {

        fn new( c: char,  encoder_set: &ECIEncoderSet,  encoder_index: i32,  previous: &InputEdge,  fnc1: i32) -> InputEdge {
            let .c =  if c == fnc1 { 1000 } else { c };
            let .encoderIndex = encoder_index;
            let .previous = previous;
             let mut size: i32 =  if let .c == 1000 { 1 } else { encoder_set.encode(c, encoder_index).len() };
             let previous_encoder_index: i32 =  if previous == null { 0 } else { previous.encoderIndex };
            if previous_encoder_index != encoder_index {
                size += COST_PER_ECI;
            }
            if previous != null {
                size += previous.cachedTotalSize;
            }
            let .cachedTotalSize = size;
        }

        fn  is_f_n_c1(&self) -> bool  {
            return self.c == 1000;
        }
    }

}

