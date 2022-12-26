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

/**
 * @author Guenther Grau
 */
#[derive(Debug, PartialEq, Eq)]
pub struct PDF417RXingResultMetadata {
    segmentIndex: usize,
    fileId: String,
    lastSegment: bool,
    segmentCount: isize,
    sender: String,
    addressee: String,
    fileName: String,
    fileSize: i64,
    timestamp: i64,
    checksum: i32,
    optionalData: Vec<u32>,
}

impl Default for PDF417RXingResultMetadata {
    fn default() -> Self {
        Self {
            segmentIndex: Default::default(),
            fileId: Default::default(),
            lastSegment: Default::default(),
            segmentCount: -1,
            sender: Default::default(),
            addressee: Default::default(),
            fileName: Default::default(),
            fileSize: -1,
            timestamp: -1,
            checksum: -1,
            optionalData: Default::default(),
        }
    }
}

impl PDF417RXingResultMetadata {
    /**
     * The Segment ID represents the segment of the whole file distributed over different symbols.
     *
     * @return File segment index
     */
    pub fn getSegmentIndex(&self) -> usize {
        self.segmentIndex
    }

    pub fn setSegmentIndex(&mut self, segmentIndex: usize) {
        self.segmentIndex = segmentIndex;
    }

    /**
     * Is the same for each related PDF417 symbol
     *
     * @return File ID
     */
    pub fn getFileId(&self) -> &str {
        &self.fileId
    }

    pub fn setFileId(&mut self, fileId: String) {
        self.fileId = fileId;
    }

    /**
     * @return always null
     * @deprecated use dedicated already parsed fields
     */
    #[deprecated]
    pub fn getOptionalData(&self) -> &[u32] {
        &self.optionalData
    }

    /**
     * @param optionalData old optional data format as int array
     * @deprecated parse and use new fields
     */
    #[deprecated]
    pub fn setOptionalData(&mut self, optionalData: Vec<u32>) {
        self.optionalData = optionalData;
    }

    /**
     * @return true if it is the last segment
     */
    pub fn isLastSegment(&self) -> bool {
        self.lastSegment
    }

    pub fn setLastSegment(&mut self, lastSegment: bool) {
        self.lastSegment = lastSegment;
    }

    /**
     * @return count of segments, -1 if not set
     */
    pub fn getSegmentCount(&self) -> isize {
        self.segmentCount
    }

    pub fn setSegmentCount(&mut self, segmentCount: isize) {
        self.segmentCount = segmentCount;
    }

    pub fn getSender(&self) -> &str {
        &self.sender
    }

    pub fn setSender(&mut self, sender: String) {
        self.sender = sender;
    }

    pub fn getAddressee(&self) -> &str {
        &self.addressee
    }

    pub fn setAddressee(&mut self, addressee: String) {
        self.addressee = addressee;
    }

    /**
     * Filename of the encoded file
     *
     * @return filename
     */
    pub fn getFileName(&self) -> &str {
        &self.fileName
    }

    pub fn setFileName(&mut self, fileName: String) {
        self.fileName = fileName;
    }

    /**
     * filesize in bytes of the encoded file
     *
     * @return filesize in bytes, -1 if not set
     */
    pub fn getFileSize(&self) -> i64 {
        self.fileSize
    }

    pub fn setFileSize(&mut self, fileSize: i64) {
        self.fileSize = fileSize;
    }

    /**
     * 16-bit CRC checksum using CCITT-16
     *
     * @return crc checksum, -1 if not set
     */
    pub fn getChecksum(&self) -> i32 {
        self.checksum
    }

    pub fn setChecksum(&mut self, checksum: i32) {
        self.checksum = checksum;
    }

    /**
     * unix epock timestamp, elapsed seconds since 1970-01-01
     *
     * @return elapsed seconds, -1 if not set
     */
    pub fn getTimestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn setTimestamp(&mut self, timestamp: i64) {
        self.timestamp = timestamp;
    }
}
