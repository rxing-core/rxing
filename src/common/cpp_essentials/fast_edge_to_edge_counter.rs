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

    pub fn stepToNextEdge(&self, _range: i32) -> i32 {
        todo!()
        // int maxSteps = std::min(stepsToBorder, range);
        // int steps = 0;
        // do {
        // 	if (++steps > maxSteps) {
        // 		if (maxSteps == stepsToBorder)
        // 			break;
        // 		else
        // 			return 0;
        // 	}
        // } while (p[steps * stride] == p[0]);

        // p += steps * stride;
        // stepsToBorder -= steps;

        // return steps;
    }
}
