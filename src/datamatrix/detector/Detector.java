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

package com.google.zxing.datamatrix.detector;

import com.google.zxing.NotFoundException;
import com.google.zxing.RXingResultPoint;
import com.google.zxing.common.BitMatrix;
import com.google.zxing.common.DetectorRXingResult;
import com.google.zxing.common.GridSampler;
import com.google.zxing.common.detector.WhiteRectangleDetector;

/**
 * <p>Encapsulates logic that can detect a Data Matrix Code in an image, even if the Data Matrix Code
 * is rotated or skewed, or partially obscured.</p>
 *
 * @author Sean Owen
 */
public final class Detector {

  private final BitMatrix image;
  private final WhiteRectangleDetector rectangleDetector;

  public Detector(BitMatrix image) throws NotFoundException {
    this.image = image;
    rectangleDetector = new WhiteRectangleDetector(image);
  }

  /**
   * <p>Detects a Data Matrix Code in an image.</p>
   *
   * @return {@link DetectorRXingResult} encapsulating results of detecting a Data Matrix Code
   * @throws NotFoundException if no Data Matrix Code can be found
   */
  public DetectorRXingResult detect() throws NotFoundException {

    RXingResultPoint[] cornerPoints = rectangleDetector.detect();

    RXingResultPoint[] points = detectSolid1(cornerPoints);
    points = detectSolid2(points);
    points[3] = correctTopRight(points);
    if (points[3] == null) {
      throw NotFoundException.getNotFoundInstance();
    }
    points = shiftToModuleCenter(points);

    RXingResultPoint topLeft = points[0];
    RXingResultPoint bottomLeft = points[1];
    RXingResultPoint bottomRight = points[2];
    RXingResultPoint topRight = points[3];

    int dimensionTop = transitionsBetween(topLeft, topRight) + 1;
    int dimensionRight = transitionsBetween(bottomRight, topRight) + 1;
    if ((dimensionTop & 0x01) == 1) {
      dimensionTop += 1;
    }
    if ((dimensionRight & 0x01) == 1) {
      dimensionRight += 1;
    }

    if (4 * dimensionTop < 6 * dimensionRight && 4 * dimensionRight < 6 * dimensionTop) {
      // The matrix is square
      dimensionTop = dimensionRight = Math.max(dimensionTop, dimensionRight);
    }

    BitMatrix bits = sampleGrid(image,
                                topLeft,
                                bottomLeft,
                                bottomRight,
                                topRight,
                                dimensionTop,
                                dimensionRight);

    return new DetectorRXingResult(bits, new RXingResultPoint[]{topLeft, bottomLeft, bottomRight, topRight});
  }

  private static RXingResultPoint shiftPoint(RXingResultPoint point, RXingResultPoint to, int div) {
    float x = (to.getX() - point.getX()) / (div + 1);
    float y = (to.getY() - point.getY()) / (div + 1);
    return new RXingResultPoint(point.getX() + x, point.getY() + y);
  }

  private static RXingResultPoint moveAway(RXingResultPoint point, float fromX, float fromY) {
    float x = point.getX();
    float y = point.getY();

    if (x < fromX) {
      x -= 1;
    } else {
      x += 1;
    }

    if (y < fromY) {
      y -= 1;
    } else {
      y += 1;
    }

    return new RXingResultPoint(x, y);
  }

  /**
   * Detect a solid side which has minimum transition.
   */
  private RXingResultPoint[] detectSolid1(RXingResultPoint[] cornerPoints) {
    // 0  2
    // 1  3
    RXingResultPoint pointA = cornerPoints[0];
    RXingResultPoint pointB = cornerPoints[1];
    RXingResultPoint pointC = cornerPoints[3];
    RXingResultPoint pointD = cornerPoints[2];

    int trAB = transitionsBetween(pointA, pointB);
    int trBC = transitionsBetween(pointB, pointC);
    int trCD = transitionsBetween(pointC, pointD);
    int trDA = transitionsBetween(pointD, pointA);

    // 0..3
    // :  :
    // 1--2
    int min = trAB;
    RXingResultPoint[] points = {pointD, pointA, pointB, pointC};
    if (min > trBC) {
      min = trBC;
      points[0] = pointA;
      points[1] = pointB;
      points[2] = pointC;
      points[3] = pointD;
    }
    if (min > trCD) {
      min = trCD;
      points[0] = pointB;
      points[1] = pointC;
      points[2] = pointD;
      points[3] = pointA;
    }
    if (min > trDA) {
      points[0] = pointC;
      points[1] = pointD;
      points[2] = pointA;
      points[3] = pointB;
    }

    return points;
  }

  /**
   * Detect a second solid side next to first solid side.
   */
  private RXingResultPoint[] detectSolid2(RXingResultPoint[] points) {
    // A..D
    // :  :
    // B--C
    RXingResultPoint pointA = points[0];
    RXingResultPoint pointB = points[1];
    RXingResultPoint pointC = points[2];
    RXingResultPoint pointD = points[3];

    // Transition detection on the edge is not stable.
    // To safely detect, shift the points to the module center.
    int tr = transitionsBetween(pointA, pointD);
    RXingResultPoint pointBs = shiftPoint(pointB, pointC, (tr + 1) * 4);
    RXingResultPoint pointCs = shiftPoint(pointC, pointB, (tr + 1) * 4);
    int trBA = transitionsBetween(pointBs, pointA);
    int trCD = transitionsBetween(pointCs, pointD);

    // 0..3
    // |  :
    // 1--2
    if (trBA < trCD) {
      // solid sides: A-B-C
      points[0] = pointA;
      points[1] = pointB;
      points[2] = pointC;
      points[3] = pointD;
    } else {
      // solid sides: B-C-D
      points[0] = pointB;
      points[1] = pointC;
      points[2] = pointD;
      points[3] = pointA;
    }

    return points;
  }

  /**
   * Calculates the corner position of the white top right module.
   */
  private RXingResultPoint correctTopRight(RXingResultPoint[] points) {
    // A..D
    // |  :
    // B--C
    RXingResultPoint pointA = points[0];
    RXingResultPoint pointB = points[1];
    RXingResultPoint pointC = points[2];
    RXingResultPoint pointD = points[3];

    // shift points for safe transition detection.
    int trTop = transitionsBetween(pointA, pointD);
    int trRight = transitionsBetween(pointB, pointD);
    RXingResultPoint pointAs = shiftPoint(pointA, pointB, (trRight + 1) * 4);
    RXingResultPoint pointCs = shiftPoint(pointC, pointB, (trTop + 1) * 4);

    trTop = transitionsBetween(pointAs, pointD);
    trRight = transitionsBetween(pointCs, pointD);

    RXingResultPoint candidate1 = new RXingResultPoint(
        pointD.getX() + (pointC.getX() - pointB.getX()) / (trTop + 1),
        pointD.getY() + (pointC.getY() - pointB.getY()) / (trTop + 1));
    RXingResultPoint candidate2 = new RXingResultPoint(
        pointD.getX() + (pointA.getX() - pointB.getX()) / (trRight + 1),
        pointD.getY() + (pointA.getY() - pointB.getY()) / (trRight + 1));

    if (!isValid(candidate1)) {
      if (isValid(candidate2)) {
        return candidate2;
      }
      return null;
    }
    if (!isValid(candidate2)) {
      return candidate1;
    }

    int sumc1 = transitionsBetween(pointAs, candidate1) + transitionsBetween(pointCs, candidate1);
    int sumc2 = transitionsBetween(pointAs, candidate2) + transitionsBetween(pointCs, candidate2);

    if (sumc1 > sumc2) {
      return candidate1;
    } else {
      return candidate2;
    }
  }

  /**
   * Shift the edge points to the module center.
   */
  private RXingResultPoint[] shiftToModuleCenter(RXingResultPoint[] points) {
    // A..D
    // |  :
    // B--C
    RXingResultPoint pointA = points[0];
    RXingResultPoint pointB = points[1];
    RXingResultPoint pointC = points[2];
    RXingResultPoint pointD = points[3];

    // calculate pseudo dimensions
    int dimH = transitionsBetween(pointA, pointD) + 1;
    int dimV = transitionsBetween(pointC, pointD) + 1;

    // shift points for safe dimension detection
    RXingResultPoint pointAs = shiftPoint(pointA, pointB, dimV * 4);
    RXingResultPoint pointCs = shiftPoint(pointC, pointB, dimH * 4);

    //  calculate more precise dimensions
    dimH = transitionsBetween(pointAs, pointD) + 1;
    dimV = transitionsBetween(pointCs, pointD) + 1;
    if ((dimH & 0x01) == 1) {
      dimH += 1;
    }
    if ((dimV & 0x01) == 1) {
      dimV += 1;
    }

    // WhiteRectangleDetector returns points inside of the rectangle.
    // I want points on the edges.
    float centerX = (pointA.getX() + pointB.getX() + pointC.getX() + pointD.getX()) / 4;
    float centerY = (pointA.getY() + pointB.getY() + pointC.getY() + pointD.getY()) / 4;
    pointA = moveAway(pointA, centerX, centerY);
    pointB = moveAway(pointB, centerX, centerY);
    pointC = moveAway(pointC, centerX, centerY);
    pointD = moveAway(pointD, centerX, centerY);

    RXingResultPoint pointBs;
    RXingResultPoint pointDs;

    // shift points to the center of each modules
    pointAs = shiftPoint(pointA, pointB, dimV * 4);
    pointAs = shiftPoint(pointAs, pointD, dimH * 4);
    pointBs = shiftPoint(pointB, pointA, dimV * 4);
    pointBs = shiftPoint(pointBs, pointC, dimH * 4);
    pointCs = shiftPoint(pointC, pointD, dimV * 4);
    pointCs = shiftPoint(pointCs, pointB, dimH * 4);
    pointDs = shiftPoint(pointD, pointC, dimV * 4);
    pointDs = shiftPoint(pointDs, pointA, dimH * 4);

    return new RXingResultPoint[]{pointAs, pointBs, pointCs, pointDs};
  }

  private boolean isValid(RXingResultPoint p) {
    return p.getX() >= 0 && p.getX() <= image.getWidth() - 1 && p.getY() > 0 && p.getY() <= image.getHeight() - 1;
  }

  private static BitMatrix sampleGrid(BitMatrix image,
                                      RXingResultPoint topLeft,
                                      RXingResultPoint bottomLeft,
                                      RXingResultPoint bottomRight,
                                      RXingResultPoint topRight,
                                      int dimensionX,
                                      int dimensionY) throws NotFoundException {

    GridSampler sampler = GridSampler.getInstance();

    return sampler.sampleGrid(image,
                              dimensionX,
                              dimensionY,
                              0.5f,
                              0.5f,
                              dimensionX - 0.5f,
                              0.5f,
                              dimensionX - 0.5f,
                              dimensionY - 0.5f,
                              0.5f,
                              dimensionY - 0.5f,
                              topLeft.getX(),
                              topLeft.getY(),
                              topRight.getX(),
                              topRight.getY(),
                              bottomRight.getX(),
                              bottomRight.getY(),
                              bottomLeft.getX(),
                              bottomLeft.getY());
  }

  /**
   * Counts the number of black/white transitions between two points, using something like Bresenham's algorithm.
   */
  private int transitionsBetween(RXingResultPoint from, RXingResultPoint to) {
    // See QR Code Detector, sizeOfBlackWhiteBlackRun()
    int fromX = (int) from.getX();
    int fromY = (int) from.getY();
    int toX = (int) to.getX();
    int toY = Math.min(image.getHeight() - 1, (int) to.getY());

    boolean steep = Math.abs(toY - fromY) > Math.abs(toX - fromX);
    if (steep) {
      int temp = fromX;
      fromX = fromY;
      fromY = temp;
      temp = toX;
      toX = toY;
      toY = temp;
    }

    int dx = Math.abs(toX - fromX);
    int dy = Math.abs(toY - fromY);
    int error = -dx / 2;
    int ystep = fromY < toY ? 1 : -1;
    int xstep = fromX < toX ? 1 : -1;
    int transitions = 0;
    boolean inBlack = image.get(steep ? fromY : fromX, steep ? fromX : fromY);
    for (int x = fromX, y = fromY; x != toX; x += xstep) {
      boolean isBlack = image.get(steep ? y : x, steep ? x : y);
      if (isBlack != inBlack) {
        transitions++;
        inBlack = isBlack;
      }
      error += dy;
      if (error > 0) {
        if (y == toY) {
          break;
        }
        y += ystep;
        error -= dx;
      }
    }
    return transitions;
  }

}
