use crate::RXingResultPoint;

#[derive(Clone, Debug)]
pub struct Quadrilateral([RXingResultPoint; 4]);

impl Quadrilateral {
    // 	using Base = std::array<T, 4>;
    // 	using Base::at;
    // public:
    // using Point = T;

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self([RXingResultPoint { x: 0.0, y: 0.0 }; 4])
    }
    // pub fn with_f32( tl:f32,  tr:f32,  br:f32,  bl:f32) -> Self {
    //     Self([tl, tr,br, bl ])
    // }

    pub fn with_points(
        tl: RXingResultPoint,
        tr: RXingResultPoint,
        br: RXingResultPoint,
        bl: RXingResultPoint,
    ) -> Self {
        Self([tl, tr, br, bl])
    }

    pub fn topLeft(&self) -> &RXingResultPoint {
        &self.0[0]
    } //const noexcept { return at(0); }
    pub fn topRight(&self) -> &RXingResultPoint {
        &self.0[1]
    } //const noexcept { return at(1); }
    pub fn bottomRight(&self) -> &RXingResultPoint {
        &self.0[2]
    } //const noexcept { return at(2); }
    pub fn bottomLeft(&self) -> &RXingResultPoint {
        &self.0[3]
    } //const noexcept { return at(3); }

    #[allow(dead_code)]
    pub fn orientation(&self) -> f64 {
        let centerLine =
            (*self.topRight() + *self.bottomRight()) - (*self.topLeft() + *self.bottomLeft());
        if (centerLine == RXingResultPoint { x: 0.0, y: 0.0 }) {
            return 0.0;
        }
        let centerLineF = RXingResultPoint::normalized(centerLine);
        f32::atan2(centerLineF.y, centerLineF.x).into()
    }
    pub fn points(&self) -> &[RXingResultPoint] {
        &self.0
    }
}

#[allow(dead_code)]
pub fn Rectangle(width: i32, height: i32, margin: Option<i32>) -> Quadrilateral {
    let margin = if let Some(m) = margin { m } else { 0 };

    Quadrilateral([
        RXingResultPoint {
            x: margin as f32,
            y: margin as f32,
        },
        RXingResultPoint {
            x: width as f32 - margin as f32,
            y: margin as f32,
        },
        RXingResultPoint {
            x: width as f32 - margin as f32,
            y: height as f32 - margin as f32,
        },
        RXingResultPoint {
            x: margin as f32,
            y: height as f32 - margin as f32,
        },
    ])
}

#[allow(dead_code)]
pub fn CenteredSquare(size: i32) -> Quadrilateral {
    Scale(
        &Quadrilateral([
            RXingResultPoint { x: -1.0, y: -1.0 },
            RXingResultPoint { x: 1.0, y: -1.0 },
            RXingResultPoint { x: 1.0, y: 1.0 },
            RXingResultPoint { x: -1.0, y: 1.0 },
        ]),
        size / 2,
    )
}

#[allow(dead_code)]
pub fn Line(y: i32, xStart: i32, xStop: i32) -> Quadrilateral {
    Quadrilateral([
        RXingResultPoint {
            x: xStart as f32,
            y: y as f32,
        },
        RXingResultPoint {
            x: xStop as f32,
            y: y as f32,
        },
        RXingResultPoint {
            x: xStop as f32,
            y: y as f32,
        },
        RXingResultPoint {
            x: xStart as f32,
            y: y as f32,
        },
    ])
}

#[allow(dead_code)]
pub fn IsConvex(poly: &Quadrilateral) -> bool {
    let N = poly.0.len();
    let mut sign = false;

    let mut m = f32::INFINITY;
    let mut M = 0.0_f32;

    for i in 0..N
    // for(int i = 0; i < N; i++)
    {
        let d1 = poly.0[(i + 2) % N] - poly.0[(i + 1) % N];
        let d2 = poly.0[i] - poly.0[(i + 1) % N];
        let cp = d1.cross(d2);

        m = if m.abs() > cp { cp } else { m.abs() };

        M = if M.abs() > cp { M.abs() } else { cp };
        // m = std::cmp::min((m).abs(), cp);
        // M = std::cmp::max((M).abs(), cp);

        if i == 0 {
            sign = cp > 0.0;
        } else if sign != (cp > 0.0) {
            return false;
        }
    }

    // It turns out being convex is not enough to prevent a "numerical instability"
    // that can cause the corners being projected inside the image boundaries but
    // some points near the corners being projected outside. This has been observed
    // where one corner is almost in line with two others. The M/m ratio is below 2
    // for the complete existing sample set. For very "skewed" QRCodes a value of
    // around 3 is realistic. A value of 14 has been observed to trigger the
    // instability.
    M / m < 4.0
}

#[allow(dead_code)]
pub fn Scale(q: &Quadrilateral, factor: i32) -> Quadrilateral {
    Quadrilateral([
        q.0[0] * factor as f32,
        q.0[1] * factor as f32,
        q.0[2] * factor as f32,
        q.0[3] * factor as f32,
    ])
}

#[allow(dead_code)]
pub fn Center(q: &Quadrilateral) -> RXingResultPoint {
    let reduced: RXingResultPoint = q.0.iter().sum();
    let size = q.0.len() as f32;
    reduced / size
    // return Reduce(q) / Size(q);
}

#[allow(dead_code)]
pub fn RotatedCorners(q: &Quadrilateral, n: Option<i32>, mirror: Option<bool>) -> Quadrilateral {
    let n = if let Some(n) = n { n } else { 1 };

    let mirror = if let Some(m) = mirror { m } else { false };

    let mut res = q.clone();
    res.0.rotate_left(((n + 4) % 4) as usize);
    // std::rotate_copy(q.begin(), q.begin() + ((n + 4) % 4), q.end(), res.begin());
    if mirror {
        res.0.swap(1, 3);
    }
    // {std::swap(res[1], res[3]);}
    res
}

#[allow(dead_code)]
pub fn IsInside(p: RXingResultPoint, q: &Quadrilateral) -> bool {
    // Test if p is on the same side (right or left) of all polygon segments
    let mut pos = 0;
    let mut neg = 0;
    for i in 0..q.0.len()
    // for (int i = 0; i < Size(q); ++i)
    {
        if RXingResultPoint::cross(p - q.0[i], q.0[(i + 1) % q.0.len()] - q.0[i]) < 0.0 {
            neg += 1;
        } else {
            pos += 1;
        }
        // (cross(p - q[i], q[(i + 1) % Size(q)] - q[i]) < 0 ? neg : pos)++;
    }

    pos == 0 || neg == 0
}

#[allow(dead_code)]
pub fn HaveIntersectingBoundingBoxes(a: &Quadrilateral, b: &Quadrilateral) -> bool {
    // TODO: this is only a quick and dirty approximation that works for the trivial standard cases
    let x = b.topRight().x < a.topLeft().x || b.topLeft().x > a.topRight().x;
    let y = b.bottomLeft().y < a.topLeft().y || b.topLeft().y > a.bottomLeft().y;

    !(x || y)
}
