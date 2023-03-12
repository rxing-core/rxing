use super::BitMatrixCursor;

pub struct FastEdgeToEdgeCounter {
    // const uint8_t* p = nullptr;
	// int stride = 0;
	// int stepsToBorder = 0;
}

impl FastEdgeToEdgeCounter
{
    pub fn new<T: BitMatrixCursor>(cur: &T) -> Self {
        todo!()
        // stride = cur.d.y * cur.img->width() + cur.d.x;
		// p = cur.img->row(cur.p.y).begin() + cur.p.x;

		// int maxStepsX = cur.d.x ? (cur.d.x > 0 ? cur.img->width() - 1 - cur.p.x : cur.p.x) : INT_MAX;
		// int maxStepsY = cur.d.y ? (cur.d.y > 0 ? cur.img->height() - 1 - cur.p.y : cur.p.y) : INT_MAX;
		// stepsToBorder = std::min(maxStepsX, maxStepsY);
    }

	 pub fn stepToNextEdge(&self,  range: i32) -> i32
	{
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