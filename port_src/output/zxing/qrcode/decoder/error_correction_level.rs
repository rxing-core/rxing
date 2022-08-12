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
// package com::google::zxing::qrcode::decoder;

/**
 * <p>See ISO 18004:2006, 6.5.1. This enum encapsulates the four error correction levels
 * defined by the QR code standard.</p>
 *
 * @author Sean Owen
 */
pub enum ErrorCorrectionLevel {

    /** L = ~7% correction */
    L(0x01), /** M = ~15% correction */
    M(0x00), /** Q = ~25% correction */
    Q(0x03), /** H = ~30% correction */
    H(0x02);

     const FOR_BITS: vec![Vec<ErrorCorrectionLevel>; 4] = vec![M, L, H, Q, ]
    ;

     let bits: i32;

    fn new( bits: i32) -> ErrorCorrectionLevel {
        let .bits = bits;
    }

    pub fn  get_bits(&self) -> i32  {
        return self.bits;
    }

    /**
   * @param bits int containing the two bits encoding a QR Code's error correction level
   * @return ErrorCorrectionLevel representing the encoded error correction level
   */
    pub fn  for_bits( bits: i32) -> ErrorCorrectionLevel  {
        if bits < 0 || bits >= FOR_BITS.len() {
            throw IllegalArgumentException::new();
        }
        return FOR_BITS[bits];
    }
}
