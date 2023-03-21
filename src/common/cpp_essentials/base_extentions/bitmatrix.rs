use crate::common::BitMatrix;
use crate::common::Result;
use crate::point;

impl BitMatrix {
    pub fn Deflate(
        &self,
        width: u32,
        height: u32,
        top: f32,
        left: f32,
        subSampling: f32,
    ) -> Result<Self> {
        let mut result = BitMatrix::new(width, height)?;

        for y in 0..result.height() {
            // for (int y = 0; y < result.height(); y++) {
            let yOffset = top + y as f32 * subSampling;
            for x in 0..result.width() {
                // for (int x = 0; x < result.width(); x++) {
                if self.get_point(point(left + x as f32 * subSampling, yOffset)) {
                    result.set(x, y);
                }
            }
        }

        Ok(result)
    }

    pub fn getTopLeftOnBitWithPosition(&self, left: &mut u32, top: &mut u32) -> bool {
        todo!()
    //     int bitsOffset = (int)std::distance(_bits.begin(), std::find_if(_bits.begin(), _bits.end(), isSet));
	// if (bitsOffset == Size(_bits)) {
	// 	return false;
	// }
	// top = bitsOffset / _width;
	// left = (bitsOffset % _width);
	// return true;
    }

    pub fn getBottomRightOnBitWithPosition(&self, right: &mut u32, bottom: &mut u32) -> bool {
        todo!()
    //     int bitsOffset = Size(_bits) - 1 - (int)std::distance(_bits.rbegin(), std::find_if(_bits.rbegin(), _bits.rend(), isSet));
	// if (bitsOffset < 0) {
	// 	return false;
	// }

	// bottom = bitsOffset / _width;
	// right = (bitsOffset % _width);
	// return true;
    }

    pub fn findBoundingBox(
        &self,
        left: u32,
        top: u32,
        width: u32,
        height: u32,
        minSize: u32,
    ) -> (bool, u32, u32, u32, u32) {
        todo!()
    //     int right, bottom;
	// if (!getTopLeftOnBit(left, top) || !getBottomRightOnBit(right, bottom) || bottom - top + 1 < minSize)
	// 	return false;

	// for (int y = top; y <= bottom; y++ ) {
	// 	for (int x = 0; x < left; ++x)
	// 		if (get(x, y)) {
	// 			left = x;
	// 			break;
	// 		}
	// 	for (int x = _width-1; x > right; x--)
	// 		if (get(x, y)) {
	// 			right = x;
	// 			break;
	// 		}
	// }

	// width = right - left + 1;
	// height = bottom - top + 1;
	// return width >= minSize && height >= minSize;
    }
}
