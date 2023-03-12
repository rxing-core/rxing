use crate::Point;

#[derive(Clone, Copy, Debug)]
pub struct Quadrilateral(pub [Point; 4]);

impl Quadrilateral {
    // 	using Base = std::array<T, 4>;
    // 	using Base::at;
    // public:
    // using Point = T;

    #[allow(dead_code)]
    pub fn new(tl: Point, tr: Point, br: Point, bl: Point) -> Self {
        Self([tl, tr, br, bl])
    }
    // pub fn with_f32( tl:f32,  tr:f32,  br:f32,  bl:f32) -> Self {
    //     Self([tl, tr,br, bl ])
    // }

    pub fn with_points(tl: Point, tr: Point, br: Point, bl: Point) -> Self {
        Self([tl, tr, br, bl])
    }

    pub fn top_left(&self) -> &Point {
        &self.0[0]
    } //const noexcept { return at(0); }
    pub fn top_right(&self) -> &Point {
        &self.0[1]
    } //const noexcept { return at(1); }
    pub fn bottom_right(&self) -> &Point {
        &self.0[2]
    } //const noexcept { return at(2); }
    pub fn bottom_left(&self) -> &Point {
        &self.0[3]
    } //const noexcept { return at(3); }

    #[allow(dead_code)]
    pub fn orientation(&self) -> f64 {
        let centerLine =
            (*self.top_right() + *self.bottom_right()) - (*self.top_left() + *self.bottom_left());
        if (centerLine == Point { x: 0.0, y: 0.0 }) {
            return 0.0;
        }
        let centerLineF = Point::normalized(centerLine);
        f32::atan2(centerLineF.y, centerLineF.x).into()
    }
    pub fn points(&self) -> &[Point] {
        &self.0
    }
}

impl Quadrilateral {
    #[allow(dead_code)]
    pub fn rectangle(width: i32, height: i32, margin: Option<i32>) -> Quadrilateral {
        let margin = margin.unwrap_or(0);

        Quadrilateral([
            Point {
                x: margin as f32,
                y: margin as f32,
            },
            Point {
                x: width as f32 - margin as f32,
                y: margin as f32,
            },
            Point {
                x: width as f32 - margin as f32,
                y: height as f32 - margin as f32,
            },
            Point {
                x: margin as f32,
                y: height as f32 - margin as f32,
            },
        ])
    }

    #[allow(dead_code)]
    pub fn centered_square(size: i32) -> Quadrilateral {
        Self::scale(
            &Quadrilateral([
                Point { x: -1.0, y: -1.0 },
                Point { x: 1.0, y: -1.0 },
                Point { x: 1.0, y: 1.0 },
                Point { x: -1.0, y: 1.0 },
            ]),
            size / 2,
        )
    }

    #[allow(dead_code)]
    pub fn line(y: i32, xStart: i32, xStop: i32) -> Quadrilateral {
        Quadrilateral([
            Point {
                x: xStart as f32,
                y: y as f32,
            },
            Point {
                x: xStop as f32,
                y: y as f32,
            },
            Point {
                x: xStop as f32,
                y: y as f32,
            },
            Point {
                x: xStart as f32,
                y: y as f32,
            },
        ])
    }

    #[allow(dead_code)]
    pub fn is_convex(&self) -> bool {
        let N = self.0.len();
        let mut sign = false;

        let mut m = f32::INFINITY;
        let mut M = 0.0_f32;

        for i in 0..N
        // for(int i = 0; i < N; i++)
        {
            let d1 = self.0[(i + 2) % N] - self.0[(i + 1) % N];
            let d2 = self.0[i] - self.0[(i + 1) % N];
            let cp = d1.cross(d2);

            // m = if m.abs() > cp { cp } else { m.abs() };

            // M = if M.abs() > cp { M.abs() } else { cp };
            m = f32::min((m).abs(), cp);
            M = f32::max((M).abs(), cp);

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
    pub fn scale(&self, factor: i32) -> Quadrilateral {
        Quadrilateral([
            self.0[0] * factor as f32,
            self.0[1] * factor as f32,
            self.0[2] * factor as f32,
            self.0[3] * factor as f32,
        ])
    }

    #[allow(dead_code)]
    pub fn center(&self) -> Point {
        let reduced: Point = self.0.iter().sum();
        let size = self.0.len() as f32;
        reduced / size
        // return Reduce(q) / Size(q);
    }

    #[allow(dead_code)]
    pub fn rotated_corners(&self, n: Option<i32>, mirror: Option<bool>) -> Quadrilateral {
        let n = if let Some(n) = n { n } else { 1 };

        let mirror = if let Some(m) = mirror { m } else { false };

        let mut res = *self;
        res.0.rotate_left(((n + 4) % 4) as usize);
        // std::rotate_copy(q.begin(), q.begin() + ((n + 4) % 4), q.end(), res.begin());
        if mirror {
            res.0.swap(1, 3);
        }
        // {std::swap(res[1], res[3]);}
        res
    }

    #[allow(dead_code)]
    pub fn is_inside(&self, p: Point) -> bool {
        // Test if p is on the same side (right or left) of all polygon segments
        let mut pos = 0;
        let mut neg = 0;
        for i in 0..self.0.len()
        // for (int i = 0; i < Size(q); ++i)
        {
            if Point::cross(p - self.0[i], self.0[(i + 1) % self.0.len()] - self.0[i]) < 0.0 {
                neg += 1;
            } else {
                pos += 1;
            }
            // (cross(p - q[i], q[(i + 1) % Size(q)] - q[i]) < 0 ? neg : pos)++;
        }

        pos == 0 || neg == 0
    }

    #[allow(dead_code)]
    pub fn have_intersecting_bounding_boxes(&self, b: &Quadrilateral) -> bool {
        // TODO: this is only a quick and dirty approximation that works for the trivial standard cases
        let x = b.top_right().x < self.top_left().x || b.top_left().x > self.top_right().x;
        let y = b.bottom_left().y < self.top_left().y || b.top_left().y > self.bottom_left().y;

        !(x || y)
    }

    pub fn blend(a: &Quadrilateral, b: &Quadrilateral) -> Self {
        let c = a[0];
        let dist2First = |a, b| Point::distance(a, c) < Point::distance(b, c);
        // rotate points such that the the two topLeft points are closest to each other
        let min_element =
            b.0.iter()
                .copied()
                .min_by(|a, b| match dist2First(*a, *b) {
                    true => std::cmp::Ordering::Less,
                    false => std::cmp::Ordering::Greater,
                })
                .unwrap_or_default();
        let offset =
            b.0.iter()
                .position(|v| *v == min_element)
                .unwrap_or_default();
        // let offset = std::min_element(b.begin(), b.end(), dist2First) - b.begin();

        let mut res = Quadrilateral::default();
        for i in 0..4 {
            // for (int i = 0; i < 4; ++i){
            res[i] = (a[i] + b[(i + offset) % 4]) / 2.0;
        }

        res
    }
}

impl Default for Quadrilateral {
    fn default() -> Self {
        Self([Point { x: 0.0, y: 0.0 }; 4])
    }
}

impl std::ops::Index<usize> for Quadrilateral {
    type Output = Point;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Quadrilateral {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
