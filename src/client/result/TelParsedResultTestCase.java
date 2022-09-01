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

package com.google.zxing.client.result;

import com.google.zxing.BarcodeFormat;
import com.google.zxing.RXingResult;
import org.junit.Assert;
import org.junit.Test;

/**
 * Tests {@link TelParsedRXingResult}.
 *
 * @author Sean Owen
 */
public final class TelParsedRXingResultTestCase extends Assert {

  @Test
  public void testTel() {
    doTest("tel:+15551212", "+15551212", null);
    doTest("tel:2125551212", "2125551212", null);
  }

  private static void doTest(String contents, String number, String title) {
    RXingResult fakeRXingResult = new RXingResult(contents, null, null, BarcodeFormat.QR_CODE);
    ParsedRXingResult result = RXingResultParser.parseRXingResult(fakeRXingResult);
    assertSame(ParsedRXingResultType.TEL, result.getType());
    TelParsedRXingResult telRXingResult = (TelParsedRXingResult) result;
    assertEquals(number, telRXingResult.getNumber());
    assertEquals(title, telRXingResult.getTitle());
    assertEquals("tel:" + number, telRXingResult.getTelURI());
  }

}