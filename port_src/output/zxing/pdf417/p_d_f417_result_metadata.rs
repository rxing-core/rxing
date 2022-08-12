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
// package com::google::zxing::pdf417;

/**
 * @author Guenther Grau
 */
pub struct PDF417ResultMetadata {

     let segment_index: i32;

     let file_id: String;

     let last_segment: bool;

     let segment_count: i32 = -1;

     let sender: String;

     let addressee: String;

     let file_name: String;

     let file_size: i64 = -1;

     let timestamp: i64 = -1;

     let checksum: i32 = -1;

     let optional_data: Vec<i32>;
}

impl PDF417ResultMetadata {

    /**
   * The Segment ID represents the segment of the whole file distributed over different symbols.
   *
   * @return File segment index
   */
    pub fn  get_segment_index(&self) -> i32  {
        return self.segment_index;
    }

    pub fn  set_segment_index(&self,  segment_index: i32)   {
        self.segmentIndex = segment_index;
    }

    /**
   * Is the same for each related PDF417 symbol
   *
   * @return File ID
   */
    pub fn  get_file_id(&self) -> String  {
        return self.file_id;
    }

    pub fn  set_file_id(&self,  file_id: &String)   {
        self.fileId = file_id;
    }

    /**
   * @return always null
   * @deprecated use dedicated already parsed fields
   */
    pub fn  get_optional_data(&self) -> Vec<i32>  {
        return self.optional_data;
    }

    /**
   * @param optionalData old optional data format as int array
   * @deprecated parse and use new fields
   */
    pub fn  set_optional_data(&self,  optional_data: &Vec<i32>)   {
        self.optionalData = optional_data;
    }

    /**
   * @return true if it is the last segment
   */
    pub fn  is_last_segment(&self) -> bool  {
        return self.last_segment;
    }

    pub fn  set_last_segment(&self,  last_segment: bool)   {
        self.lastSegment = last_segment;
    }

    /**
   * @return count of segments, -1 if not set
   */
    pub fn  get_segment_count(&self) -> i32  {
        return self.segment_count;
    }

    pub fn  set_segment_count(&self,  segment_count: i32)   {
        self.segmentCount = segment_count;
    }

    pub fn  get_sender(&self) -> String  {
        return self.sender;
    }

    pub fn  set_sender(&self,  sender: &String)   {
        self.sender = sender;
    }

    pub fn  get_addressee(&self) -> String  {
        return self.addressee;
    }

    pub fn  set_addressee(&self,  addressee: &String)   {
        self.addressee = addressee;
    }

    /**
   * Filename of the encoded file
   *
   * @return filename
   */
    pub fn  get_file_name(&self) -> String  {
        return self.file_name;
    }

    pub fn  set_file_name(&self,  file_name: &String)   {
        self.fileName = file_name;
    }

    /**
   * filesize in bytes of the encoded file
   *
   * @return filesize in bytes, -1 if not set
   */
    pub fn  get_file_size(&self) -> i64  {
        return self.file_size;
    }

    pub fn  set_file_size(&self,  file_size: i64)   {
        self.fileSize = file_size;
    }

    /**
   * 16-bit CRC checksum using CCITT-16
   *
   * @return crc checksum, -1 if not set
   */
    pub fn  get_checksum(&self) -> i32  {
        return self.checksum;
    }

    pub fn  set_checksum(&self,  checksum: i32)   {
        self.checksum = checksum;
    }

    /**
   * unix epock timestamp, elapsed seconds since 1970-01-01
   *
   * @return elapsed seconds, -1 if not set
   */
    pub fn  get_timestamp(&self) -> i64  {
        return self.timestamp;
    }

    pub fn  set_timestamp(&self,  timestamp: i64)   {
        self.timestamp = timestamp;
    }
}

