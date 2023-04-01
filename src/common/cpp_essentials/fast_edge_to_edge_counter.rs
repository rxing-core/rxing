use crate::common::BitArray;

use super::BitMatrixCursorTrait;

pub struct FastEdgeToEdgeCounter {
    // const uint8_t* p = nullptr;
    // int stride = 0;
    // int stepsToBorder = 0;
    p: u32,             // = nullptr;
    stride: u32,        // = 0;
    stepsToBorder: u32, // = 0;
    arr: BitArray,
}

impl FastEdgeToEdgeCounter {
    pub fn new<T: BitMatrixCursorTrait>(cur: &T) -> Self {
        let stride = cur.d().y as u32 * cur.img().width() as u32 + cur.d().x as u32;
        let p = /*cur.img().getRow(cur.p().y).begin()*/ 0 + cur.p().x as u32;

        let maxStepsX = if cur.d().x != 0.0 {
            (if cur.d().x > 0.0 {
                cur.img().width() - 1 - cur.p().x as u32
            } else {
                cur.p().x as u32
            })
        } else {
            u32::MAX
        };
        let maxStepsY = if cur.d().y != 0.0 {
            (if cur.d().y > 0.0 {
                cur.img().height() - 1 - cur.p().y as u32
            } else {
                cur.p().y as u32
            })
        } else {
            u32::MAX
        };
        let stepsToBorder = std::cmp::min(maxStepsX, maxStepsY);

        Self {
            p,
            stride,
            stepsToBorder,
            arr: cur.img().getRow(cur.p().y as u32),
        }
    }

    pub fn stepToNextEdge(&mut self, range: u32) -> u32 {
        let maxSteps = std::cmp::min(self.stepsToBorder, range);
        let steps = 0;
        loop {
            steps += 1;
            if (steps > maxSteps) {
                if (maxSteps == self.stepsToBorder) {
                    break;
                } else {
                    return 0;
                }
            }
            if !(self.arr.get((self.p + steps * self.stride) as usize)
                == self.arr.get((self.p + 0) as usize))
            {
                break;
            }
        } // while (p[steps * stride] == p[0]);

        self.p += steps * self.stride;
        self.stepsToBorder -= steps;

        return steps;
    }
}
