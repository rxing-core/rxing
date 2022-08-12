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
 * The general exception class throw when something goes wrong during decoding of a barcode.
 * This includes, but is not limited to, failing checksums / error correction algorithms, being
 * unable to locate finder timing patterns, and so on.
 *
 * @author Sean Owen
 */

// disable stack traces when not running inside test units
 let is_stack_trace: bool = System::get_property("surefire.test.class.path") != null;

 const NO_TRACE: [Option<StackTraceElement>; 0] = [None; 0];
pub struct ReaderException {
    super: Exception;
}

impl ReaderException {

    fn new() -> ReaderException {
    // do nothing
    }

    fn new( cause: &Throwable) -> ReaderException {
        super(&cause);
    }

    // Prevent stack traces from being taken
    pub fn  fill_in_stack_trace(&self) -> Throwable  {
        return null;
    }

    /**
   * For testing only. Controls whether library exception classes include stack traces or not.
   * Defaults to false, unless running in the project's unit testing harness.
   *
   * @param enabled if true, enables stack traces in library exception classes
   * @since 3.5.0
   */
    pub fn  set_stack_trace( enabled: bool)   {
        is_stack_trace = enabled;
    }
}

