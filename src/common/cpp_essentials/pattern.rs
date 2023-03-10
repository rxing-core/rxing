/*
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use crate::{common::Result, Exceptions};

pub type PatternType = u16;
pub type Pattern<const N: usize> = [PatternType; N];

#[derive(Default)]
pub struct PatternRow(Vec<PatternType>);

// pub struct PatternRow<T: std::iter::Sum + Into<f32> + Into<usize> + Copy>(Vec<T>);

impl PatternRow {
    pub fn into_pattern_view(&self) -> PatternView {
        PatternView::new(self)
    }
}

impl<'a> Iterator for PatternView<'_> {
    type Item = PatternType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current + 1 > self.count {
            return None;
        }

        self.current = self.current + 1;

        Some(*self.data.0.get(self.current + self.start)?)
    }
}

pub struct PatternView<'a> {
    data: &'a PatternRow,
    start: usize,
    count: usize,
    current: usize,
}

impl<'a> PatternView<'_> {
    // A PatternRow always starts with the width of whitespace in front of the first black bar.
    // The first element of the PatternView is the first bar.
    pub fn new(bars: &'a PatternRow) -> PatternView<'a> {
        PatternView {
            data: bars,
            start: 0,
            count: bars.0.len(),
            current: 0,
        }
    }

    pub fn with_config(
        bars: &'a PatternRow,
        start: usize,
        size: usize,
        base: usize,
        end: usize,
    ) -> PatternView<'a> {
        PatternView {
            data: bars,
            start,
            count: size,
            current: base,
        }
    }

    pub fn data(&self) -> &PatternRow {
        &self.data
    }
    pub fn begin(&self) -> Option<PatternType> {
        Some(*self.data.0.get(self.start)?)
    }
    pub fn end(&self) -> Option<PatternType> {
        Some(self.data.0[self.start + self.count])
    }

    // int sum(int n = 0) const { return std::accumulate(_data, _data + (n == 0 ? _size : n), 0); }
    pub fn sum(&self, n: Option<usize>) -> PatternType {
        let n = n.unwrap_or(self.count);

        self.data
            .0
            .iter()
            .skip(self.start)
            .take(n)
            .copied()
            .sum::<PatternType>()
    }

    pub fn size(&self) -> usize {
        self.count
    }

    // index is the number of bars and spaces from the first bar to the current position
    pub fn index(&self) -> usize {
        self.current - self.start - 1 /*return narrow_cast<int>(_data - _base) - 1;*/
    }
    pub fn pixelsInFront(&self) -> PatternType {
        self.data
            .0
            .iter()
            .skip(self.start)
            .take(self.current)
            .copied()
            .sum::<PatternType>() /*return std::accumulate(_base, _data, 0);*/
    }
    pub fn pixelsTillEnd(&self) -> PatternType {
        self.data
            .0
            .iter()
            .skip(self.start + self.current)
            .copied()
            .sum::<PatternType>() /*return std::accumulate(_base, _data + _size, 0) - 1;*/
    }
    pub fn isAtFirstBar(&self) -> bool {
        self.start == (self.current + 1) /*return _data == _base + 1;*/
    }
    pub fn isAtLastBar(&self) -> bool {
        self.current == self.start + self.count - 1 /*return _data + _size == _end - 1;*/
    }
    pub fn isValidWithN(&self, n: usize) -> bool {
        !self.data.0.is_empty()
            && self.start <= self.current
            && self.start + n <= (self.start + self.count)
        /*return _data && _data >= _base && _data + n <= _end;*/
    }
    pub fn isValid(&self) -> bool {
        self.isValidWithN(self.size())
    }

    fn has_quiet_zone_before(&self, scale: f32, acceptIfAtFirstBar: Option<bool>) -> bool {
        (acceptIfAtFirstBar.unwrap_or(false) && self.isAtLastBar())
            || Into::<f32>::into(self.data.0[self.count])
                >= Into::<f32>::into(self.sum(None)) * scale
    }
    // template<bool acceptIfAtFirstBar = false>
    // bool hasQuietZoneBefore(float scale) const
    // {
    // 	return (acceptIfAtFirstBar && isAtFirstBar()) || _data[-1] >= sum() * scale;
    // }

    pub fn hasQuietZoneAfter(&self, scale: f32, acceptIfAtLastBar: Option<bool>) -> bool {
        (acceptIfAtLastBar.unwrap_or(true) && self.isAtLastBar())
            || Into::<f32>::into(self.data.0[self.count])
                >= Into::<f32>::into(self.sum(None)) * scale
    }

    // template<bool acceptIfAtLastBar = true>
    // bool hasQuietZoneAfter(float scale) const
    // {
    // 	return (acceptIfAtLastBar && isAtLastBar()) || _data[_size] >= sum() * scale;
    // }

    pub fn subView(&'a self, offset: usize, size: Option<usize>) -> PatternView<'a> {
        let mut size = size.unwrap_or(0);
        if (size == 0) {
            size = self.count - offset;
        } else if (size < 0) {
            size = self.count - offset + size;
        }

        PatternView {
            data: self.data,
            start: self.start + offset,
            count: size,
            current: self.current,
        }
    }

    // 	PatternView subView(int offset, int size = 0) const
    // 	{
    // //		if(std::abs(size) > count())
    // //			printf("%d > %d\n", std::abs(size), _count);
    // //		assert(std::abs(size) <= count());
    // 		if (size == 0)
    // 			size = _size - offset;
    // 		else if (size < 0)
    // 			size = _size - offset + size;
    // 		return {begin() + offset, std::max(size, 0), _base, _end};
    // 	}

    pub fn shift(&mut self, n: usize) -> bool {
        self.start += n;
        !self.data.0.is_empty() && self.start + self.count <= (self.start + self.count)
    }

    // bool shift(int n)
    // {
    // 	return _data && ((_data += n) + _size <= _end);
    // }

    pub fn skipPair(&mut self) -> bool {
        self.shift(2)
    }

    pub fn skipSymbol(&mut self) -> bool {
        self.shift(self.count)
    }

    pub fn skipSingle(&mut self /*  maxWidth: usize */) -> bool {
        self.shift(1) //&& _data[-1] <= maxWidth;
    }

    pub fn extend(&mut self) {
        self.count = std::cmp::max(0, self.count - self.start)
    }
}

impl<'a> std::ops::Index<isize> for PatternView<'_> {
    type Output = PatternType;

    fn index(&self, index: isize) -> &Self::Output {
        if index > self.data.0.len() as isize {
            panic!("array index out of bounds")
        }
        if index >= 0 {
            return &self[index as usize];
        }
        if index.abs() > self.start as isize {
            panic!("array index out of bounds")
        }
        let fetch_spot = (self.start as isize + index) as usize;
        &self.data.0[fetch_spot]
    }
}

impl<'a> std::ops::Index<usize> for PatternView<'_> {
    type Output = PatternType;

    fn index(&self, index: usize) -> &Self::Output {
        if index > self.data.0.len() {
            panic!("array index out of bounds")
        }
        &self.data.0.get(self.start + self.current).unwrap()
    }
}

/**
 * @brief The BarAndSpace struct is a simple 2 element data structure to hold information about bar(s) and space(s).
 *
 * The operator[](int) can be used in combination with a PatternView
 */
struct BarAndSpace<T: Default + std::cmp::PartialEq> {
    bar: T,
    space: T,
}
impl<T: Default + std::cmp::PartialEq> BarAndSpace<T> {
    pub fn isValid(&self) -> bool {
        self.bar != T::default() && self.space != T::default()
    }
}

impl<T: Default + std::cmp::PartialEq> std::ops::Index<usize> for BarAndSpace<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.bar,
            1 => &self.space,
            _ => panic!("Index out of range for BarAndSpace"),
        }
    }
}

impl<T: Default + std::cmp::PartialEq> std::ops::IndexMut<usize> for BarAndSpace<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.bar,
            1 => &mut self.space,
            _ => panic!("Index out of range for BarAndSpace"),
        }
    }
}

// 	using value_type = T;
// 	T bar = {}, space = {};
// 	// even index -> bar, odd index -> space
// 	T& operator[](int i) { return reinterpret_cast<T*>(this)[i & 1]; }
// 	T operator[](int i) const { return reinterpret_cast<const T*>(this)[i & 1]; }
// 	bool isValid() const { return bar != T{} && space != T{}; }
// };

type BarAndSpaceI = BarAndSpace<u16>;

/**
 * @brief FixedPattern describes a compile-time constant (start/stop) pattern.
 *
 * @param N  number of bars/spaces
 * @param SUM  sum over all N elements (size of pattern in modules)
 * @param IS_SPARCE  whether or not the pattern contains '0's denoting 'wide' bars/spaces
 */
pub struct FixedPattern<const N: usize, const SUM: usize, const IS_SPARCE: bool = false> {
    data: [u16; N],
}

impl<const N: usize, const SUM: usize, const IS_SPARCE: bool> FixedPattern<N, SUM, IS_SPARCE> {
    fn new(data: [u16; N]) -> Self {
        FixedPattern { data }
    }

    fn as_slice(&self) -> &[u16] {
        &self.data
    }

    fn size(&self) -> usize {
        N
    }
}

impl<const N: usize, const SUM: usize, const IS_SPARCE: bool> std::ops::Index<usize>
    for FixedPattern<N, SUM, IS_SPARCE>
{
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

pub type FixedSparcePattern<const N: usize, const SUM: usize> = FixedPattern<N, SUM, true>;

// template <int N, int SUM, bool IS_SPARCE = false>
// struct FixedPattern
// {
// 	using value_type = PatternRow::value_type;
// 	value_type _data[N];
// 	constexpr value_type operator[](int i) const noexcept { return _data[i]; }
// 	constexpr const value_type* data() const noexcept { return _data; }
// 	constexpr int size() const noexcept { return N; }
// };

// template <int N, int SUM>
// using FixedSparcePattern = FixedPattern<N, SUM, true>;

pub fn IsPattern<const LEN: usize, const SUM: usize, const SPARSE: bool>(
    view: &PatternView,
    pattern: &FixedPattern<LEN, SUM, SPARSE>,
    space_in_pixel: Option<f32>,
    min_quiet_zone: f32,
    module_size_ref: f32,
    relaxed_threshold: Option<bool>,
) -> f32 {
    let relaxed_threshold = relaxed_threshold.unwrap_or(false);

    let width = view.sum(Some(LEN));
    if SUM > LEN && Into::<usize>::into(width) < SUM {
        return 0.0;
    }

    let module_size: f32 = (Into::<f32>::into(width)) / (SUM as f32);

    if min_quiet_zone != 0.0
        && (space_in_pixel.unwrap_or(f32::MAX)) < min_quiet_zone as f32 * module_size as f32 - 1.0
    {
        return 0.0;
    }

    let threshold = module_size_ref * (0.5 + (relaxed_threshold as u8) as f32 * 0.25) + 0.5;

    // the offset of 0.5 is to make the code less sensitive to quantization errors for small (near 1) module sizes.
    // TODO: review once we have upsampling in the binarizer in place.

    for x in 0..LEN {
        if (Into::<f32>::into(view[x]) - Into::<f32>::into(pattern[x]) * module_size_ref).abs()
            > threshold
        {
            return 0.0;
        }
    }

    module_size
}

pub fn IsRightGuard<const N: usize, const SUM: usize, const IS_SPARCE: bool>(
    view: &PatternView,
    pattern: &FixedPattern<N, SUM, IS_SPARCE>,
    minQuietZone: f32,
    moduleSizeRef: f32,
) -> bool {
    let spaceInPixel = if view.isAtLastBar() {
        None
    } else {
        Some(view.end().unwrap().into())
    };

    IsPattern(
        view,
        pattern,
        spaceInPixel,
        minQuietZone,
        moduleSizeRef,
        None,
    ) != 0.0
}

pub fn FindLeftGuardBy<'a, const LEN: usize, Pred: Fn(&PatternView, Option<f32>) -> bool>(
    view: &'a PatternView,
    minSize: usize,
    isGuard: Pred,
) -> Result<PatternView<'a>> {
    const PREV_IDX: isize = -1;

    if (view.size() < minSize) {
        return Err(Exceptions::ILLEGAL_STATE);
    }

    let mut window = view.subView(0, Some(LEN));
    if (window.isAtFirstBar() && isGuard(&window, None)) {
        return Ok(window);
    }
    let end = Into::<usize>::into(view.end().unwrap()) - minSize;
    while window.start < end {
        if (isGuard(&window, Some(Into::<f32>::into(window[PREV_IDX])))) {
            return Ok(window);
        }

        window.skipPair();
    }
    // for (auto end = view.end() - minSize; window.data() < end; window.skipPair())
    // 	{

    // 	}

    Err(Exceptions::ILLEGAL_STATE)
}

pub fn FindLeftGuard<'a, const LEN: usize, const SUM: usize, const IS_SPARCE: bool>(
    view: &'a PatternView,
    minSize: usize,
    pattern: &FixedPattern<LEN, SUM, IS_SPARCE>,
    minQuietZone: f32,
) -> Result<PatternView<'a>> {
    FindLeftGuardBy::<LEN, _>(view, std::cmp::max(minSize, LEN), |window, spaceInPixel| {
        IsPattern(&window, pattern, spaceInPixel, minQuietZone, 0.0, None) != 0.0
    })
}

pub fn NormalizedE2EPattern<'a, const LEN: usize, const LEN_MINUS_2: usize, const SUM: usize>(
    view: &'a PatternView,
) -> [PatternType; LEN_MINUS_2] {
    let moduleSize: f32 = Into::<f32>::into(view.sum(Some(LEN))) / SUM as f32;

    let mut e2e = [PatternType::default(); LEN_MINUS_2];

    for i in 0..LEN_MINUS_2 {
        let v: f32 = (Into::<f32>::into(view[i]) + Into::<f32>::into(view[i + 1])) / moduleSize;
        e2e[i] = (v + 0.5) as PatternType;
    }

    return e2e;
}

pub fn NormalizedPattern<'a, const LEN: usize, const SUM: usize>(
    view: &'a PatternView,
) -> Result<[PatternType; LEN]> {
    let moduleSize: f32 = (Into::<usize>::into(view.sum(Some(LEN))) / SUM) as f32;
    let mut err = SUM as isize;
    let mut is = [PatternType::default(); LEN];
    let mut rs = [0.0; LEN];
    for i in 0..LEN {
        // for (int i = 0; i < LEN; i++) {
        let v: f32 = Into::<f32>::into(view[i]) / moduleSize;
        is[i] = (v + 0.5) as PatternType;
        rs[i] = v - Into::<f32>::into(is[i]);
        err -= Into::<usize>::into(is[i]) as isize;
    }

    if (err.abs() > 1) {
        return Err(Exceptions::NOT_FOUND);
    }

    if (err != 0) {
        // let mi =if  err > 0 { std::max_element(std::begin(rs), std::end(rs)) - std::begin(rs)}
        // 				  else {std::min_element(std::begin(rs), std::end(rs)) - std::begin(rs)};
        let mi = if err > 0 {
            rs.iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        } else {
            rs.iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
        };
        let mi = mi.ok_or(Exceptions::ILLEGAL_STATE)?;
        is[*mi as usize] += err as u16;
        rs[*mi as usize] -= err as f32;
    }

    Ok(is)
}

fn GetPatternRow<T: Into<u16> + Copy>(b_row: &[T], p_row: &mut PatternRow) {
    p_row.0.clear();
    p_row.0.push(0); // first value is number of white pixels, here 0

    let mut bit_pos = 0u16;
    let mut count = 0;

    for next_bit in b_row.iter().copied() {
        count += 1;

        if next_bit.into() != bit_pos {
            p_row.0.push(count);
            count = 0;
        }

        bit_pos = next_bit.into();
    }

    count += 1;
    p_row.0.push(count); // final value is number of white pixels after last black pixel

    if bit_pos != 0 {
        p_row.0.push(0); // add final black pixel
    }
}

#[cfg(test)]
mod tests {
    use super::{GetPatternRow, PatternRow};
    const N: usize = 33;

    #[test]
    fn all_white() {
        for s in 1..=N {
            // for (int s = 1; s <= N; ++s) {
            let t_in = vec![0_u16; s];
            // std::vector<uint8_t> in(s, 0);
            let mut pr = PatternRow::default();
            GetPatternRow(&t_in, &mut pr);

            assert_eq!(pr.0.len(), 1);
            assert_eq!(pr.0[0], s as u16);
        }
    }

    #[test]
    fn all_black() {
        for s in 1..=N {
            // for (int s = 1; s <= N; ++s) {
            let t_in: Vec<u16> = vec![0xff; s];
            let mut pr = PatternRow::default();
            GetPatternRow(&t_in, &mut pr);

            assert_eq!(pr.0.len(), 3);
            assert_eq!(pr.0[0], 0);
            assert_eq!(pr.0[1], s as u16);
            assert_eq!(pr.0[2], 0);
        }
    }

    #[test]
    fn black_white() {
        for s in 1..=N {
            // for (int s = 1; s <= N; ++s) {
            let mut t_in = vec![0_u16; N];
            t_in[..s].copy_from_slice(&vec![1; s]);
            // std::fill_n(in.data(), s, 0xff);
            let mut pr = PatternRow::default();
            GetPatternRow(&t_in, &mut pr);

            assert_eq!(pr.0.len(), 3);
            assert_eq!(pr.0[0], 0);
            assert_eq!(pr.0[1], s as u16);
            assert_eq!(pr.0[2], (N - s) as u16);
        }
    }

    #[test]
    fn white_black() {
        for s in 0..N {
            // for (int s = 0; s < N; ++s) {
            let mut t_in: Vec<u16> = vec![0xff; N];
            t_in[..s].copy_from_slice(&vec![0; s]);
            let mut pr = PatternRow::default();
            GetPatternRow(&t_in, &mut pr);

            assert_eq!(pr.0.len(), 3);
            assert_eq!(pr.0[0], s as u16);
            assert_eq!(pr.0[1], (N - s) as u16);
            assert_eq!(pr.0[2], 0);
        }
    }
}
