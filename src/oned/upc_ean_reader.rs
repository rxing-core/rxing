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

use crate::{Exceptions, common::BitArray, RXingResult};

use super::OneDReader;

/**
 * <p>Encapsulates functionality and implementation that is common to UPC and EAN families
 * of one-dimensional barcodes.</p>
 *
 * @author dswitkin@google.com (Daniel Switkin)
 * @author Sean Owen
 * @author alasdair@google.com (Alasdair Mackintosh)
 */
pub trait UPCEANReader: OneDReader {


  // These two values are critical for determining how permissive the decoding will be.
  // We've arrived at these values through a lot of trial and error. Setting them any higher
  // lets false positives creep in quickly.
  const MAX_AVG_VARIANCE : f32= 0.48;
  const MAX_INDIVIDUAL_VARIANCE : f32= 0.7;

  /**
   * Start/end guard pattern.
   */
  const START_END_PATTERN : [u32;3]= [1, 1, 1,];

  /**
   * Pattern marking the middle of a UPC/EAN pattern, separating the two halves.
   */
  const MIDDLE_PATTERN : [u32;5]= [1, 1, 1, 1, 1];
  /**
   * end guard pattern.
   */
  const END_PATTERN : [u32;6]= [1, 1, 1, 1, 1, 1];
  /**
   * "Odd", or "L" patterns used to encode UPC/EAN digits.
   */
  const L_PATTERNS : [[u32;4];10]= [
      [3, 2, 1, 1], // 0
      [2, 2, 2, 1], // 1
      [2, 1, 2, 2], // 2
      [1, 4, 1, 1], // 3
      [1, 1, 3, 2], // 4
      [1, 2, 3, 1], // 5
      [1, 1, 1, 4], // 6
      [1, 3, 1, 2], // 7
      [1, 2, 1, 3], // 8
      [3, 1, 1, 2]  // 9
  ];

  /**
   * As above but also including the "even", or "G" patterns used to encode UPC/EAN digits.
   */
  const L_AND_G_PATTERNS : [[u32;4];20] = {
    let new_array = [[0_u32;4];20];//new int[20][];
    new_array[0..10].copy_from_slice(&Self::L_PATTERNS[0..10]);
    // System.arraycopy(L_PATTERNS, 0, L_AND_G_PATTERNS, 0, 10);
    let mut i = 10;
    while i < 20 {
    // for (int i = 10; i < 20; i++) {
      let widths = &Self::L_PATTERNS[i - 10];
      let reversedWidths = [0_u32;4];//new int[widths.length];
      let mut j = 0;
      while j < 4 {
      // for (int j = 0; j < widths.length; j++) {
        reversedWidths[j] = widths[4 - j - 1];
        
        j+=1;
      }
      new_array[i] = reversedWidths;

      i+=1;
    }

    new_array
  };

  // private final StringBuilder decodeRowStringBuffer;
  // private final UPCEANExtensionSupport extensionReader;
  // private final EANManufacturerOrgSupport eanManSupport;

  // protected UPCEANReader() {
  //   decodeRowStringBuffer = new StringBuilder(20);
  //   extensionReader = new UPCEANExtensionSupport();
  //   eanManSupport = new EANManufacturerOrgSupport();
  // }

   fn findStartGuardPattern( row:&BitArray) -> Result<Vec<u32>,Exceptions> {
    boolean foundStart = false;
    int[] startRange = null;
    int nextStart = 0;
    int[] counters = new int[START_END_PATTERN.length];
    while (!foundStart) {
      Arrays.fill(counters, 0, START_END_PATTERN.length, 0);
      startRange = findGuardPattern(row, nextStart, false, START_END_PATTERN, counters);
      int start = startRange[0];
      nextStart = startRange[1];
      // Make sure there is a quiet zone at least as big as the start pattern before the barcode.
      // If this check would run off the left edge of the image, do not accept this barcode,
      // as it is very likely to be a false positive.
      int quietStart = start - (nextStart - start);
      if (quietStart >= 0) {
        foundStart = row.isRange(quietStart, start, false);
      }
    }
    return startRange;
  }

  // @Override
  // public RXingResult decodeRow(int rowNumber, BitArray row, Map<DecodeHintType,?> hints)
  //     throws NotFoundException, ChecksumException, FormatException {
  //   return decodeRow(rowNumber, row, findStartGuardPattern(row), hints);
  // }

  /**
   * <p>Like {@link #decodeRow(int, BitArray, Map)}, but
   * allows caller to inform method about where the UPC/EAN start pattern is
   * found. This allows this to be computed once and reused across many implementations.</p>
   *
   * @param rowNumber row index into the image
   * @param row encoding of the row of the barcode image
   * @param startGuardRange start/end column where the opening start pattern was found
   * @param hints optional hints that influence decoding
   * @return {@link RXingResult} encapsulating the result of decoding a barcode in the row
   * @throws NotFoundException if no potential barcode is found
   * @throws ChecksumException if a potential barcode is found but does not pass its checksum
   * @throws FormatException if a potential barcode is found but format is invalid
   */
  fn  decodeRowWithGuardRange(rowNumber:u32,
                           row:&BitArray,
                           startGuardRange:&[u32;2],
                          hints:&crate::DecodingHintDictionary)
      -> Result<RXingResult,Exceptions> {

    RXingResultPointCallback resultPointCallback = hints == null ? null :
        (RXingResultPointCallback) hints.get(DecodeHintType.NEED_RESULT_POINT_CALLBACK);
    int symbologyIdentifier = 0;

    if (resultPointCallback != null) {
      resultPointCallback.foundPossibleRXingResultPoint(new RXingResultPoint(
          (startGuardRange[0] + startGuardRange[1]) / 2.0f, rowNumber
      ));
    }

    StringBuilder result = decodeRowStringBuffer;
    result.setLength(0);
    int endStart = decodeMiddle(row, startGuardRange, result);

    if (resultPointCallback != null) {
      resultPointCallback.foundPossibleRXingResultPoint(new RXingResultPoint(
          endStart, rowNumber
      ));
    }

    int[] endRange = decodeEnd(row, endStart);

    if (resultPointCallback != null) {
      resultPointCallback.foundPossibleRXingResultPoint(new RXingResultPoint(
          (endRange[0] + endRange[1]) / 2.0f, rowNumber
      ));
    }


    // Make sure there is a quiet zone at least as big as the end pattern after the barcode. The
    // spec might want more whitespace, but in practice this is the maximum we can count on.
    int end = endRange[1];
    int quietEnd = end + (end - endRange[0]);
    if (quietEnd >= row.getSize() || !row.isRange(end, quietEnd, false)) {
      throw NotFoundException.getNotFoundInstance();
    }

    String resultString = result.toString();
    // UPC/EAN should never be less than 8 chars anyway
    if (resultString.length() < 8) {
      throw FormatException.getFormatInstance();
    }
    if (!checkChecksum(resultString)) {
      throw ChecksumException.getChecksumInstance();
    }

    float left = (startGuardRange[1] + startGuardRange[0]) / 2.0f;
    float right = (endRange[1] + endRange[0]) / 2.0f;
    BarcodeFormat format = getBarcodeFormat();
    RXingResult decodeRXingResult = new RXingResult(resultString,
        null, // no natural byte representation for these barcodes
        new RXingResultPoint[]{
            new RXingResultPoint(left, rowNumber),
            new RXingResultPoint(right, rowNumber)},
        format);

    int extensionLength = 0;

    try {
      RXingResult extensionRXingResult = extensionReader.decodeRow(rowNumber, row, endRange[1]);
      decodeRXingResult.putMetadata(RXingResultMetadataType.UPC_EAN_EXTENSION, extensionRXingResult.getText());
      decodeRXingResult.putAllMetadata(extensionRXingResult.getRXingResultMetadata());
      decodeRXingResult.addRXingResultPoints(extensionRXingResult.getRXingResultPoints());
      extensionLength = extensionRXingResult.getText().length();
    } catch (ReaderException re) {
      // continue
    }

    int[] allowedExtensions =
        hints == null ? null : (int[]) hints.get(DecodeHintType.ALLOWED_EAN_EXTENSIONS);
    if (allowedExtensions != null) {
      boolean valid = false;
      for (int length : allowedExtensions) {
        if (extensionLength == length) {
          valid = true;
          break;
        }
      }
      if (!valid) {
        throw NotFoundException.getNotFoundInstance();
      }
    }

    if (format == BarcodeFormat.EAN_13 || format == BarcodeFormat.UPC_A) {
      String countryID = eanManSupport.lookupCountryIdentifier(resultString);
      if (countryID != null) {
        decodeRXingResult.putMetadata(RXingResultMetadataType.POSSIBLE_COUNTRY, countryID);
      }
    }
    if (format == BarcodeFormat.EAN_8) {
      symbologyIdentifier = 4;
    }

    decodeRXingResult.putMetadata(RXingResultMetadataType.SYMBOLOGY_IDENTIFIER, "]E" + symbologyIdentifier);

    return decodeRXingResult;
  }

  /**
   * @param s string of digits to check
   * @return {@link #checkStandardUPCEANChecksum(CharSequence)}
   * @throws FormatException if the string does not contain only digits
   */
  fn checkChecksum(&self,  s:&str) -> Result<String,Exceptions> {
     Self::checkStandardUPCEANChecksum(s)
  }

  /**
   * Computes the UPC/EAN checksum on a string of digits, and reports
   * whether the checksum is correct or not.
   *
   * @param s string of digits to check
   * @return true iff string of digits passes the UPC/EAN checksum algorithm
   * @throws FormatException if the string does not contain only digits
   */
  fn checkStandardUPCEANChecksum( s:&str) -> Result<bool,Exceptions> {
    int length = s.length();
    if (length == 0) {
      return false;
    }
    int check = Character.digit(s.charAt(length - 1), 10);
    return getStandardUPCEANChecksum(s.subSequence(0, length - 1)) == check;
  }

  fn getStandardUPCEANChecksum( s:&str) -> Result<u32,Exceptions> {
    int length = s.length();
    int sum = 0;
    for (int i = length - 1; i >= 0; i -= 2) {
      int digit = s.charAt(i) - '0';
      if (digit < 0 || digit > 9) {
        throw FormatException.getFormatInstance();
      }
      sum += digit;
    }
    sum *= 3;
    for (int i = length - 2; i >= 0; i -= 2) {
      int digit = s.charAt(i) - '0';
      if (digit < 0 || digit > 9) {
        throw FormatException.getFormatInstance();
      }
      sum += digit;
    }
    return (1000 - sum) % 10;
  }

  fn decodeEnd(&self,  row:&BitArray,  endStart:usize) -> Result<[usize;2],Exceptions> {
     Self::findGuardPattern(row, endStart, false, Self::START_END_PATTERN)
  }

  fn findGuardPattern( row:&BitArray,
                                 rowOffset:usize,
                                 whiteFirst:bool,
                                 pattern:[u32;3]) -> Result<[usize;2],Exceptions> {
     Self::findGuardPatternWithCounters(row, rowOffset, whiteFirst, pattern, vec![0u32;pattern.len()])
  }

  /**
   * @param row row of black/white values to search
   * @param rowOffset position to start search
   * @param whiteFirst if true, indicates that the pattern specifies white/black/white/...
   * pixel counts, otherwise, it is interpreted as black/white/black/...
   * @param pattern pattern of counts of number of black and white pixels that are being
   * searched for as a pattern
   * @param counters array of counters, as long as pattern, to re-use
   * @return start/end horizontal offset of guard pattern, as an array of two ints
   * @throws NotFoundException if pattern is not found
   */
  fn findGuardPatternWithCounters( row:&BitArray,
                                         rowOffset:usize,
                                         whiteFirst:bool,
                                         pattern:&[u32;3],
                                         counters:&Vec<u32>) -> Result<[usize;2],Exceptions> {
    int width = row.getSize();
    rowOffset = whiteFirst ? row.getNextUnset(rowOffset) : row.getNextSet(rowOffset);
    int counterPosition = 0;
    int patternStart = rowOffset;
    int patternLength = pattern.length;
    boolean isWhite = whiteFirst;
    for (int x = rowOffset; x < width; x++) {
      if (row.get(x) != isWhite) {
        counters[counterPosition]++;
      } else {
        if (counterPosition == patternLength - 1) {
          if (patternMatchVariance(counters, pattern, MAX_INDIVIDUAL_VARIANCE) < MAX_AVG_VARIANCE) {
            return new int[]{patternStart, x};
          }
          patternStart += counters[0] + counters[1];
          System.arraycopy(counters, 2, counters, 0, counterPosition - 1);
          counters[counterPosition - 1] = 0;
          counters[counterPosition] = 0;
          counterPosition--;
        } else {
          counterPosition++;
        }
        counters[counterPosition] = 1;
        isWhite = !isWhite;
      }
    }
    throw NotFoundException.getNotFoundInstance();
  }

  /**
   * Attempts to decode a single UPC/EAN-encoded digit.
   *
   * @param row row of black/white values to decode
   * @param counters the counts of runs of observed black/white/black/... values
   * @param rowOffset horizontal offset to start decoding from
   * @param patterns the set of patterns to use to decode -- sometimes different encodings
   * for the digits 0-9 are used, and this indicates the encodings for 0 to 9 that should
   * be used
   * @return horizontal offset of first pixel beyond the decoded digit
   * @throws NotFoundException if digit cannot be decoded
   */
   decodeDigit( row:&BitArray,  counters:&Vec<u32>,  rowOffset:usize,  patterns:&Vec<Vec<u32>>)
      -> Result<u32,Exceptions> {
    recordPattern(row, rowOffset, counters);
    float bestVariance = MAX_AVG_VARIANCE; // worst variance we'll accept
    int bestMatch = -1;
    int max = patterns.length;
    for (int i = 0; i < max; i++) {
      int[] pattern = patterns[i];
      float variance = patternMatchVariance(counters, pattern, MAX_INDIVIDUAL_VARIANCE);
      if (variance < bestVariance) {
        bestVariance = variance;
        bestMatch = i;
      }
    }
    if (bestMatch >= 0) {
      return bestMatch;
    } else {
      throw NotFoundException.getNotFoundInstance();
    }
  }

  /**
   * Get the format of this decoder.
   *
   * @return The 1D format.
   */
   fn  getBarcodeFormat() -> BarcodeFormat;

  /**
   * Subclasses override this to decode the portion of a barcode between the start
   * and end guard patterns.
   *
   * @param row row of black/white values to search
   * @param startRange start/end offset of start guard pattern
   * @param resultString {@link StringBuilder} to append decoded chars to
   * @return horizontal offset of first pixel after the "middle" that was decoded
   * @throws NotFoundException if decoding could not complete successfully
   */
  fn decodeMiddle( row:&BitArray,
                                       startRange:&[u32;2],
                                       resultString:&mut String) -> Result<u32,Exceptions>;

}
