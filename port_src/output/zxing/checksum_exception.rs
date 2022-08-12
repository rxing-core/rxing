/*
 * Copyright 2007 ZXing authors
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
 * Thrown when a barcode was successfully detected and decoded, but
 * was not returned because its checksum feature failed.
 *
 * @author Sean Owen
 */

 const INSTANCE: ChecksumException = ChecksumException::new();
pub struct ChecksumException {
    super: ReaderException;
}

impl ChecksumException {

    static {
        // since it's meaningless
        INSTANCE::set_stack_trace(NO_TRACE);
    }

    fn new() -> ChecksumException {
    // do nothing
    }

    fn new( cause: &Throwable) -> ChecksumException {
        super(&cause);
    }

    pub fn  get_checksum_instance() -> ChecksumException  {
        return  if is_stack_trace { ChecksumException::new() } else { INSTANCE };
    }

    pub fn  get_checksum_instance( cause: &Throwable) -> ChecksumException  {
        return  if is_stack_trace { ChecksumException::new(&cause) } else { INSTANCE };
    }
}

