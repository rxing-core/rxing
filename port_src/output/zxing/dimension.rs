/*
 * Copyright 2012 ZXing authors
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
// package com::google::zxing;

/**
 * Simply encapsulates a width and height.
 */
pub struct Dimension {

     let width: i32;

     let height: i32;
}

impl Dimension {

    pub fn new( width: i32,  height: i32) -> Dimension {
        if width < 0 || height < 0 {
            throw IllegalArgumentException::new();
        }
        let .width = width;
        let .height = height;
    }

    pub fn  get_width(&self) -> i32  {
        return self.width;
    }

    pub fn  get_height(&self) -> i32  {
        return self.height;
    }

    pub fn  equals(&self,  other: &Object) -> bool  {
        if other instanceof Dimension {
             let d: Dimension = other as Dimension;
            return self.width == d.width && self.height == d.height;
        }
        return false;
    }

    pub fn  hash_code(&self) -> i32  {
        return self.width * 32713 + self.height;
    }

    pub fn  to_string(&self) -> String  {
        return format!("{}x{}", self.width, self.height);
    }
}

