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

use crate::RXingResultPoint;

/**
 * Meta-data container for QR Code decoding. Instances of this class may be used to convey information back to the
 * decoding caller. Callers are expected to process this.
 *
 * @see com.google.zxing.common.DecoderRXingResult#getOther()
 */
pub struct QRCodeDecoderMetaData(bool);

impl  QRCodeDecoderMetaData {
  pub fn new( mirrored:bool) -> Self {
    Self(mirrored)
  }

  /**
   * @return true if the QR Code was mirrored.
   */
  pub fn isMirrored(&self) -> bool{
    self.0
  }

  /**
   * Apply the result points' order correction due to mirroring.
   *
   * @param points Array of points to apply mirror correction to.
   */
  pub fn applyMirroredCorrection(&self, points : &mut [RXingResultPoint]) {
    if !self.0 || points.is_empty() || points.len() < 3 {
      return
    }
    let bottom_left = points[0];
    points[0] = points[2];
    points[2] = bottom_left;
    // No need to 'fix' top-left and alignment pattern.
  }

}
