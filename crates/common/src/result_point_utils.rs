use crate::Point;

/**
 * Orders an array of three Points in an order [A,B,C] such that AB is less than AC
 * and BC is less than AC, and the angle between BC and BA is less than 180 degrees.
 *
 * @param patterns array of three {@code Point} to order
 */
pub fn orderBestPatterns<T: Into<Point> + Copy>(patterns: &mut [T; 3]) {
    // Find distances between pattern centers
    let zeroOneDistance = Point::distance(patterns[0].into(), patterns[1].into());
    let oneTwoDistance = Point::distance(patterns[1].into(), patterns[2].into());
    let zeroTwoDistance = Point::distance(patterns[0].into(), patterns[2].into());

    // Assume one closest to other two is B; A and C will just be guesses at first
    let (mut pointA, pointB, mut pointC) =
        if oneTwoDistance >= zeroOneDistance && oneTwoDistance >= zeroTwoDistance {
            (patterns[1], patterns[0], patterns[2])
        } else if zeroTwoDistance >= oneTwoDistance && zeroTwoDistance >= zeroOneDistance {
            (patterns[0], patterns[1], patterns[2])
        } else {
            (patterns[0], patterns[2], patterns[1])
        };

    // Use cross product to figure out whether A and C are correct or flipped.
    // This asks whether BC x BA has a positive z component, which is the arrangement
    // we want for A, B, C. If it's negative, then we've got it flipped around and
    // should swap A and C.
    if crossProductZ(pointA.into(), pointB.into(), pointC.into()) < 0.0 {
        std::mem::swap(&mut pointA, &mut pointC);
    }

    patterns[0] = pointA;
    patterns[1] = pointB;
    patterns[2] = pointC;
}

/**
 * Returns the z component of the cross product between vectors BC and BA.
 */
fn crossProductZ(a: Point, b: Point, c: Point) -> f32 {
    ((c.x - b.x) * (a.y - b.y)) - ((c.y - b.y) * (a.x - b.x))
}
