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

package com.google.zxing.multi.qrcode;

import com.google.zxing.BarcodeFormat;
import com.google.zxing.BinaryBitmap;
import com.google.zxing.DecodeHintType;
import com.google.zxing.NotFoundException;
import com.google.zxing.ReaderException;
import com.google.zxing.RXingResult;
import com.google.zxing.RXingResultMetadataType;
import com.google.zxing.RXingResultPoint;
import com.google.zxing.common.DecoderRXingResult;
import com.google.zxing.common.DetectorRXingResult;
import com.google.zxing.multi.MultipleBarcodeReader;
import com.google.zxing.multi.qrcode.detector.MultiDetector;
import com.google.zxing.qrcode.QRCodeReader;
import com.google.zxing.qrcode.decoder.QRCodeDecoderMetaData;

import java.io.ByteArrayOutputStream;
import java.io.Serializable;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.Collections;
import java.util.Comparator;

/**
 * This implementation can detect and decode multiple QR Codes in an image.
 *
 * @author Sean Owen
 * @author Hannes Erven
 */
public final class QRCodeMultiReader extends QRCodeReader implements MultipleBarcodeReader {

  private static final RXingResult[] EMPTY_RESULT_ARRAY = new RXingResult[0];
  private static final RXingResultPoint[] NO_POINTS = new RXingResultPoint[0];

  @Override
  public RXingResult[] decodeMultiple(BinaryBitmap image) throws NotFoundException {
    return decodeMultiple(image, null);
  }

  @Override
  public RXingResult[] decodeMultiple(BinaryBitmap image, Map<DecodeHintType,?> hints) throws NotFoundException {
    List<RXingResult> results = new ArrayList<>();
    DetectorRXingResult[] detectorRXingResults = new MultiDetector(image.getBlackMatrix()).detectMulti(hints);
    for (DetectorRXingResult detectorRXingResult : detectorRXingResults) {
      try {
        DecoderRXingResult decoderRXingResult = getDecoder().decode(detectorRXingResult.getBits(), hints);
        RXingResultPoint[] points = detectorRXingResult.getPoints();
        // If the code was mirrored: swap the bottom-left and the top-right points.
        if (decoderRXingResult.getOther() instanceof QRCodeDecoderMetaData) {
          ((QRCodeDecoderMetaData) decoderRXingResult.getOther()).applyMirroredCorrection(points);
        }
        RXingResult result = new RXingResult(decoderRXingResult.getText(), decoderRXingResult.getRawBytes(), points,
                                   BarcodeFormat.QR_CODE);
        List<byte[]> byteSegments = decoderRXingResult.getByteSegments();
        if (byteSegments != null) {
          result.putMetadata(RXingResultMetadataType.BYTE_SEGMENTS, byteSegments);
        }
        String ecLevel = decoderRXingResult.getECLevel();
        if (ecLevel != null) {
          result.putMetadata(RXingResultMetadataType.ERROR_CORRECTION_LEVEL, ecLevel);
        }
        if (decoderRXingResult.hasStructuredAppend()) {
          result.putMetadata(RXingResultMetadataType.STRUCTURED_APPEND_SEQUENCE,
                             decoderRXingResult.getStructuredAppendSequenceNumber());
          result.putMetadata(RXingResultMetadataType.STRUCTURED_APPEND_PARITY,
                             decoderRXingResult.getStructuredAppendParity());
        }
        results.add(result);
      } catch (ReaderException re) {
        // ignore and continue
      }
    }
    if (results.isEmpty()) {
      return EMPTY_RESULT_ARRAY;
    } else {
      results = processStructuredAppend(results);
      return results.toArray(EMPTY_RESULT_ARRAY);
    }
  }

  static List<RXingResult> processStructuredAppend(List<RXingResult> results) {
    List<RXingResult> newRXingResults = new ArrayList<>();
    List<RXingResult> saRXingResults = new ArrayList<>();
    for (RXingResult result : results) {
      if (result.getRXingResultMetadata().containsKey(RXingResultMetadataType.STRUCTURED_APPEND_SEQUENCE)) {
        saRXingResults.add(result);
      } else {
        newRXingResults.add(result);
      }
    }
    if (saRXingResults.isEmpty()) {
      return results;
    }

    // sort and concatenate the SA list items
    Collections.sort(saRXingResults, new SAComparator());
    StringBuilder newText = new StringBuilder();
    ByteArrayOutputStream newRawBytes = new ByteArrayOutputStream();
    ByteArrayOutputStream newByteSegment = new ByteArrayOutputStream();
    for (RXingResult saRXingResult : saRXingResults) {
      newText.append(saRXingResult.getText());
      byte[] saBytes = saRXingResult.getRawBytes();
      newRawBytes.write(saBytes, 0, saBytes.length);
      @SuppressWarnings("unchecked")
      Iterable<byte[]> byteSegments =
          (Iterable<byte[]>) saRXingResult.getRXingResultMetadata().get(RXingResultMetadataType.BYTE_SEGMENTS);
      if (byteSegments != null) {
        for (byte[] segment : byteSegments) {
          newByteSegment.write(segment, 0, segment.length);
        }
      }
    }

    RXingResult newRXingResult = new RXingResult(newText.toString(), newRawBytes.toByteArray(), NO_POINTS, BarcodeFormat.QR_CODE);
    if (newByteSegment.size() > 0) {
      newRXingResult.putMetadata(RXingResultMetadataType.BYTE_SEGMENTS, Collections.singletonList(newByteSegment.toByteArray()));
    }
    newRXingResults.add(newRXingResult);
    return newRXingResults;
  }

  private static final class SAComparator implements Comparator<RXingResult>, Serializable {
    @Override
    public int compare(RXingResult a, RXingResult b) {
      int aNumber = (int) a.getRXingResultMetadata().get(RXingResultMetadataType.STRUCTURED_APPEND_SEQUENCE);
      int bNumber = (int) b.getRXingResultMetadata().get(RXingResultMetadataType.STRUCTURED_APPEND_SEQUENCE);
      return Integer.compare(aNumber, bNumber);
    }
  }

}
