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

import java.util.Locale;

import com.google.zxing.BarcodeFormat;
import com.google.zxing.RXingResult;
import org.junit.Assert;
import org.junit.Test;

/**
 * Tests {@link GeoParsedRXingResult}.
 *
 * @author Sean Owen
 */
public final class GeoParsedRXingResultTestCase extends Assert {

  private static final double EPSILON = 1.0E-10;

  @Test
  public void testGeo() {
    doTest("geo:1,2", 1.0, 2.0, 0.0, null, "geo:1.0,2.0");
    doTest("geo:80.33,-32.3344,3.35", 80.33, -32.3344, 3.35, null, null);
    doTest("geo:-20.33,132.3344,0.01", -20.33, 132.3344, 0.01, null, null);
    doTest("geo:-20.33,132.3344,0.01?q=foobar", -20.33, 132.3344, 0.01, "q=foobar", null);
    doTest("GEO:-20.33,132.3344,0.01?q=foobar", -20.33, 132.3344, 0.01, "q=foobar", null);
  }

  private static void doTest(String contents,
                             double latitude,
                             double longitude,
                             double altitude,
                             String query,
                             String uri) {
    RXingResult fakeRXingResult = new RXingResult(contents, null, null, BarcodeFormat.QR_CODE);
    ParsedRXingResult result = RXingResultParser.parseRXingResult(fakeRXingResult);
    assertSame(ParsedRXingResultType.GEO, result.getType());
    GeoParsedRXingResult geoRXingResult = (GeoParsedRXingResult) result;
    assertEquals(latitude, geoRXingResult.getLatitude(), EPSILON);
    assertEquals(longitude, geoRXingResult.getLongitude(), EPSILON);
    assertEquals(altitude, geoRXingResult.getAltitude(), EPSILON);
    assertEquals(query, geoRXingResult.getQuery());
    assertEquals(uri == null ? contents.toLowerCase(Locale.ENGLISH) : uri, geoRXingResult.getGeoURI());
  }

}