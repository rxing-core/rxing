/*
 * Copyright 2009 ZXing authors
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

package com.google.zxing.pdf417;

import com.google.zxing.BarcodeFormat;
import com.google.zxing.BinaryBitmap;
import com.google.zxing.ChecksumException;
import com.google.zxing.DecodeHintType;
import com.google.zxing.FormatException;
import com.google.zxing.NotFoundException;
import com.google.zxing.Reader;
import com.google.zxing.RXingResult;
import com.google.zxing.RXingResultMetadataType;
import com.google.zxing.RXingResultPoint;
import com.google.zxing.common.DecoderRXingResult;
import com.google.zxing.multi.MultipleBarcodeReader;
import com.google.zxing.pdf417.decoder.PDF417ScanningDecoder;
import com.google.zxing.pdf417.detector.Detector;
import com.google.zxing.pdf417.detector.PDF417DetectorRXingResult;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

/**
 * This implementation can detect and decode PDF417 codes in an image.
 *
 * @author Guenther Grau
 */
public final class PDF417Reader implements Reader, MultipleBarcodeReader {

  private static final RXingResult[] EMPTY_RESULT_ARRAY = new RXingResult[0];

  /**
   * Locates and decodes a PDF417 code in an image.
   *
   * @return a String representing the content encoded by the PDF417 code
   * @throws NotFoundException if a PDF417 code cannot be found,
   * @throws FormatException if a PDF417 cannot be decoded
   */
  @Override
  public RXingResult decode(BinaryBitmap image) throws NotFoundException, FormatException, ChecksumException {
    return decode(image, null);
  }

  @Override
  public RXingResult decode(BinaryBitmap image, Map<DecodeHintType,?> hints) throws NotFoundException, FormatException,
      ChecksumException {
    RXingResult[] result = decode(image, hints, false);
    if (result.length == 0 || result[0] == null) {
      throw NotFoundException.getNotFoundInstance();
    }
    return result[0];
  }

  @Override
  public RXingResult[] decodeMultiple(BinaryBitmap image) throws NotFoundException {
    return decodeMultiple(image, null);
  }

  @Override
  public RXingResult[] decodeMultiple(BinaryBitmap image, Map<DecodeHintType,?> hints) throws NotFoundException {
    try {
      return decode(image, hints, true);
    } catch (FormatException | ChecksumException ignored) {
      throw NotFoundException.getNotFoundInstance();
    }
  }

  private static RXingResult[] decode(BinaryBitmap image, Map<DecodeHintType, ?> hints, boolean multiple)
      throws NotFoundException, FormatException, ChecksumException {
    List<RXingResult> results = new ArrayList<>();
    PDF417DetectorRXingResult detectorRXingResult = Detector.detect(image, hints, multiple);
    for (RXingResultPoint[] points : detectorRXingResult.getPoints()) {
      DecoderRXingResult decoderRXingResult = PDF417ScanningDecoder.decode(detectorRXingResult.getBits(), points[4], points[5],
          points[6], points[7], getMinCodewordWidth(points), getMaxCodewordWidth(points));
      RXingResult result = new RXingResult(decoderRXingResult.getText(), decoderRXingResult.getRawBytes(), points, BarcodeFormat.PDF_417);
      result.putMetadata(RXingResultMetadataType.ERROR_CORRECTION_LEVEL, decoderRXingResult.getECLevel());
      PDF417RXingResultMetadata pdf417RXingResultMetadata = (PDF417RXingResultMetadata) decoderRXingResult.getOther();
      if (pdf417RXingResultMetadata != null) {
        result.putMetadata(RXingResultMetadataType.PDF417_EXTRA_METADATA, pdf417RXingResultMetadata);
      }
      result.putMetadata(RXingResultMetadataType.ORIENTATION, detectorRXingResult.getRotation());
      result.putMetadata(RXingResultMetadataType.SYMBOLOGY_IDENTIFIER, "]L" + decoderRXingResult.getSymbologyModifier());
      results.add(result);
    }
    return results.toArray(EMPTY_RESULT_ARRAY);
  }

  private static int getMaxWidth(RXingResultPoint p1, RXingResultPoint p2) {
    if (p1 == null || p2 == null) {
      return 0;
    }
    return (int) Math.abs(p1.getX() - p2.getX());
  }

  private static int getMinWidth(RXingResultPoint p1, RXingResultPoint p2) {
    if (p1 == null || p2 == null) {
      return Integer.MAX_VALUE;
    }
    return (int) Math.abs(p1.getX() - p2.getX());
  }

  private static int getMaxCodewordWidth(RXingResultPoint[] p) {
    return Math.max(
        Math.max(getMaxWidth(p[0], p[4]), getMaxWidth(p[6], p[2]) * PDF417Common.MODULES_IN_CODEWORD /
            PDF417Common.MODULES_IN_STOP_PATTERN),
        Math.max(getMaxWidth(p[1], p[5]), getMaxWidth(p[7], p[3]) * PDF417Common.MODULES_IN_CODEWORD /
            PDF417Common.MODULES_IN_STOP_PATTERN));
  }

  private static int getMinCodewordWidth(RXingResultPoint[] p) {
    return Math.min(
        Math.min(getMinWidth(p[0], p[4]), getMinWidth(p[6], p[2]) * PDF417Common.MODULES_IN_CODEWORD /
            PDF417Common.MODULES_IN_STOP_PATTERN),
        Math.min(getMinWidth(p[1], p[5]), getMinWidth(p[7], p[3]) * PDF417Common.MODULES_IN_CODEWORD /
            PDF417Common.MODULES_IN_STOP_PATTERN));
  }

  @Override
  public void reset() {
    // nothing needs to be reset
  }

}
