use crate::common::BitMatrix;

use super::BitMatrixCursorTrait;

pub struct FastEdgeToEdgeCounter<'a> {
    // const uint8_t* p = nullptr;
    // int stride = 0;
    // int stepsToBorder = 0;
    p: u32,             // = nullptr;
    stride: isize,      // = 0;
    stepsToBorder: i32, // = 0;
    //arr: BitArray,
    _arr: isize,
    under_arry: &'a BitMatrix, //,Vec<bool>
}

impl<'a> FastEdgeToEdgeCounter<'a> {
    pub fn new<T: BitMatrixCursorTrait>(cur: &T) -> FastEdgeToEdgeCounter {
        let stride = cur.d().y as isize * cur.img().width() as isize + cur.d().x as isize;
        let p = ((cur.p().y as isize * cur.img().width() as isize).abs() as i32 + cur.p().x as i32)
            as u32; // P IS SET WRONG IN REVERSE

        let maxStepsX: i32 = if cur.d().x != 0.0 {
            if cur.d().x > 0.0 {
                cur.img().width() as i32 - 1 - cur.p().x as i32
            } else {
                cur.p().x as i32
            }
        } else {
            i32::MAX
        };
        let maxStepsY: i32 = if cur.d().y != 0.0 {
            if cur.d().y > 0.0 {
                cur.img().height() as i32 - 1 - cur.p().y as i32
            } else {
                cur.p().y as i32
            }
        } else {
            i32::MAX
        };
        let stepsToBorder = std::cmp::min(maxStepsX, maxStepsY) as i32;

        FastEdgeToEdgeCounter {
            p,
            stride,
            stepsToBorder,
            _arr: cur.p().y as isize * stride as isize, //cur.img().getRow(cur.p().y as u32),
            under_arry: cur.img(),                      //.into(),
        }
    }

    pub fn stepToNextEdge(&mut self, range: u32) -> u32 {
        let maxSteps = std::cmp::min(self.stepsToBorder, range as i32);
        let mut steps = 0;
        loop {
            steps += 1;
            if steps > maxSteps {
                if maxSteps == self.stepsToBorder {
                    break;
                } else {
                    return 0;
                }
            }

            let idx_pt = self.get_array_check_index(steps);

            // if !(self.under_arry[idx_pt]
            //     == self.under_arry[self.p as usize])
            if !(self.under_arry.get_index(idx_pt) == self.under_arry.get_index(self.p as usize)) {
                break;
            }
        } // while (p[steps * stride] == p[0]);

        self.p = (self.p as isize + (steps as isize * self.stride)).abs() as u32;
        self.stepsToBorder -= steps;

        return steps as u32;
    }

    #[inline(always)]
    fn get_array_check_index(&self, steps: i32) -> usize {
        (self.p as isize + (steps as isize * self.stride)) as usize
    }
}
