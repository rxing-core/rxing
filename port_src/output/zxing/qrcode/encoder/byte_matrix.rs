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
// package com::google::zxing::qrcode::encoder;

/**
 * JAVAPORT: The original code was a 2D array of ints, but since it only ever gets assigned
 * -1, 0, and 1, I'm going to use less memory and go with bytes.
 *
 * @author dswitkin@google.com (Daniel Switkin)
 */
pub struct ByteMatrix {

     let mut bytes: Vec<Vec<i8>>;

     let width: i32;

     let height: i32;
}

impl ByteMatrix {

    pub fn new( width: i32,  height: i32) -> ByteMatrix {
        bytes = : [[i8; width]; height] = [[0; width]; height];
        let .width = width;
        let .height = height;
    }

    pub fn  get_height(&self) -> i32  {
        return self.height;
    }

    pub fn  get_width(&self) -> i32  {
        return self.width;
    }

    pub fn  get(&self,  x: i32,  y: i32) -> i8  {
        return self.bytes[y][x];
    }

    /**
   * @return an internal representation as bytes, in row-major order. array[y][x] represents point (x,y)
   */
    pub fn  get_array(&self) -> Vec<Vec<i8>>  {
        return self.bytes;
    }

    pub fn  set(&self,  x: i32,  y: i32,  value: i8)   {
        self.bytes[y][x] = value;
    }

    pub fn  set(&self,  x: i32,  y: i32,  value: i32)   {
        self.bytes[y][x] = value as i8;
    }

    pub fn  set(&self,  x: i32,  y: i32,  value: bool)   {
        self.bytes[y][x] = ( if value { 1 } else { 0 }) as i8;
    }

    pub fn  clear(&self,  value: i8)   {
        for  let a_byte: Vec<i8> in self.bytes {
            Arrays::fill(&a_byte, value);
        }
    }

    pub fn  to_string(&self) -> String  {
         let result: StringBuilder = StringBuilder::new(2 * self.width * self.height + 2);
         {
             let mut y: i32 = 0;
            while y < self.height {
                {
                     let bytes_y: Vec<i8> = self.bytes[y];
                     {
                         let mut x: i32 = 0;
                        while x < self.width {
                            {
                                match bytes_y[x] {
                                      0 => 
                                         {
                                            result.append(" 0");
                                            break;
                                        }
                                      1 => 
                                         {
                                            result.append(" 1");
                                            break;
                                        }
                                    _ => 
                                         {
                                            result.append("  ");
                                            break;
                                        }
                                }
                            }
                            x += 1;
                         }
                     }

                    result.append('\n');
                }
                y += 1;
             }
         }

        return result.to_string();
    }
}

