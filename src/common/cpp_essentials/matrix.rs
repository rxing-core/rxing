use crate::common::Result;
use crate::{Exceptions, Point};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Matrix<T: Default + Clone + Copy> {
    width: usize,
    height: usize,
    data: Vec<Option<T>>,
}

impl<T: Default + Clone + Copy> Matrix<T> {
    pub fn with_data(width: usize, height: usize, data: Vec<Option<T>>) -> Result<Matrix<T>> {
        if width != 0 && data.len() / width as usize != height as usize {
            return Err(Exceptions::illegal_argument_with(
                "invalid size: width * height is too big",
            ));
        }
        Ok(Self {
            width,
            height,
            data,
        })
    }

    pub fn new(width: usize, height: usize) -> Result<Matrix<T>> {
        if (width != 0 && (width * height) / width as usize != height as usize) {
            return Err(Exceptions::illegal_argument_with(
                "invalid size: width * height is too big",
            ));
        }
        Ok(Self {
            width,
            height,
            data: vec![None; width * height],
        })
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    // value_t& operator()(int x, int y)
    // {
    // 	assert(x >= 0 && x < _width && y >= 0 && y < _height);
    // 	return _data[y * _width + x];
    // }

    // const T& operator()(int x, int y) const
    // {
    // 	assert(x >= 0 && x < _width && y >= 0 && y < _height);
    // 	return _data[y * _width + x];
    // }

    fn get_offset(x: usize, y: usize, width: usize) -> usize {
        (y * width + x) as usize
    }

    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x >= self.width || y >= self.height {
            None
        } else if let Some(Some(d)) = self.data.get(Self::get_offset(x, y, self.width)) {
            Some(*d)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> T {
        self.data[Self::get_offset(x, y, self.width)] = Some(value);
        self.get(x, y).unwrap()
    }

    pub fn get_point(&self, p: Point) -> Option<T> {
        self.get(p.x as usize, p.y as usize)
    }

    pub fn set_point(&mut self, p: Point, value: T) -> T {
        self.set(p.x as usize, p.y as usize, value)
    }

    pub fn data(&self) -> &[Option<T>] {
        &self.data
    }

    // const value_t* begin() const {
    // 	return _data.data();
    // }

    // const value_t* end() const {
    // 	return _data.data() + _width * _height;
    // }

    pub fn clear_with(&mut self, value: T) {
        self.data.fill(Some(value))
    }

    pub fn clear(&mut self) {
        self.data.fill(None)
    }
}
