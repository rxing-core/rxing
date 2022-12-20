use crate::{common::detector::MathUtils, ResultPoint};

/**
 * Orders an array of three RXingResultPoints in an order [A,B,C] such that AB is less than AC
 * and BC is less than AC, and the angle between BC and BA is less than 180 degrees.
 *
 * @param patterns array of three {@code RXingResultPoint} to order
 */
pub fn orderBestPatterns<T: ResultPoint + Copy + Clone>(patterns: &mut [T; 3]) {
    // Find distances between pattern centers
    let zeroOneDistance = MathUtils::distance_float(
        patterns[0].getX(),
        patterns[0].getY(),
        patterns[1].getX(),
        patterns[1].getY(),
    );
    let oneTwoDistance = MathUtils::distance_float(
        patterns[1].getX(),
        patterns[1].getY(),
        patterns[2].getX(),
        patterns[2].getY(),
    );
    let zeroTwoDistance = MathUtils::distance_float(
        patterns[0].getX(),
        patterns[0].getY(),
        patterns[2].getX(),
        patterns[2].getY(),
    );

    let mut pointA; //: &RXingResultPoint;
    let pointB; //: &RXingResultPoint;
    let mut pointC; //: &RXingResultPoint;
                    // Assume one closest to other two is B; A and C will just be guesses at first
    if oneTwoDistance >= zeroOneDistance && oneTwoDistance >= zeroTwoDistance {
        pointB = patterns[0];
        pointA = patterns[1];
        pointC = patterns[2];
    } else if zeroTwoDistance >= oneTwoDistance && zeroTwoDistance >= zeroOneDistance {
        pointB = patterns[1];
        pointA = patterns[0];
        pointC = patterns[2];
    } else {
        pointB = patterns[2];
        pointA = patterns[0];
        pointC = patterns[1];
    }

    // Use cross product to figure out whether A and C are correct or flipped.
    // This asks whether BC x BA has a positive z component, which is the arrangement
    // we want for A, B, C. If it's negative, then we've got it flipped around and
    // should swap A and C.
    if crossProductZ(pointA, pointB, pointC) < 0.0f32 {
        std::mem::swap(&mut pointA, &mut pointC);
    }

    let pa = pointA;
    let pb = pointB;
    let pc = pointC;

    patterns[0] = pa;
    patterns[1] = pb;
    patterns[2] = pc;
}

/**
 * @param pattern1 first pattern
 * @param pattern2 second pattern
 * @return distance between two points
 */
pub fn distance<T: ResultPoint>(pattern1: &T, pattern2: &T) -> f32 {
    return MathUtils::distance_float(
        pattern1.getX(),
        pattern1.getY(),
        pattern2.getX(),
        pattern2.getY(),
    );
}

/**
 * Returns the z component of the cross product between vectors BC and BA.
 */
pub fn crossProductZ<T: ResultPoint>(pointA: T, pointB: T, pointC: T) -> f32 {
    let bX = pointB.getX();
    let bY = pointB.getY();
    return ((pointC.getX() - bX) * (pointA.getY() - bY))
        - ((pointC.getY() - bY) * (pointA.getX() - bX));
}
