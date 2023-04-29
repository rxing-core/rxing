/*
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use crate::{
    common::{BitMatrix, Result},
    Exceptions,
};

pub type PatternType = u16;
pub type Pattern<const N: usize> = [PatternType; N];

fn BarAndSpaceSum<
    const LEN: usize,
    T: Into<RT> + Copy,
    RT: Default + std::cmp::PartialEq + std::ops::AddAssign,
>(
    view: &[T],
) -> BarAndSpace<RT> {
    let mut res = BarAndSpace::default();
    for i in 0..LEN {
        // for (int i = 0; i < LEN; ++i)
        res[i] += view[i].into();
    }
    res
}

#[derive(Default, Debug)]
pub struct PatternRow(Vec<PatternType>);

// pub struct PatternRow<T: std::iter::Sum + Into<f32> + Into<usize> + Copy>(Vec<T>);

impl PatternRow {
    pub fn new(v: Vec<PatternType>) -> Self {
        Self(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn into_pattern_view(&self) -> PatternView {
        PatternView::new(self)
    }
}

impl IntoIterator for PatternRow {
    type Item = PatternType;

    type IntoIter = std::vec::IntoIter<PatternType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::ops::Index<usize> for PatternRow {
    type Output = PatternType;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for PatternRow {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<Vec<PatternType>> for PatternRow {
    fn from(value: Vec<PatternType>) -> Self {
        Self(value)
    }
}

pub struct PatternViewIterator<'a> {
    pattern_view: &'a PatternView<'a>,
    current_position: usize,
}

impl<'a> Iterator for PatternViewIterator<'_> {
    type Item = PatternType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_position + 1 > self.pattern_view.count {
            return None;
        }

        self.current_position += 1;

        Some(
            *self.pattern_view.data.0.get(
                self.current_position - 1 + self.pattern_view.start + self.pattern_view.current,
            )?,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PatternView<'a> {
    data: &'a PatternRow,
    start: usize,
    count: usize,
    current: usize,
}

impl<'a> PatternView<'a> {
    // A PatternRow always starts with the width of whitespace in front of the first black bar.
    // The first element of the PatternView is the first bar.
    pub fn new(bars: &'a PatternRow) -> PatternView<'a> {
        PatternView {
            data: bars,
            start: 1,
            count: bars.0.len(),
            current: 0,
        }
    }

    pub fn with_config(
        bars: &'a PatternRow,
        start: usize,
        size: usize,
        base: usize,
        _end: usize,
    ) -> PatternView<'a> {
        PatternView {
            data: bars,
            start,
            count: size,
            current: base,
        }
    }

    pub fn data(&self) -> &PatternRow {
        self.data
    }
    pub fn begin(&self) -> Option<PatternType> {
        Some(*self.data.0.get(self.start)?)
    }
    pub fn end(&self) -> Option<PatternType> {
        // if self.start + self.count < self.data.0.len() {
        //     Some(self.data.0[self.start + self.count])
        // } else {
        //     None
        // }
        Some(self.data.0.len() as PatternType)
    }

    // int sum(int n = 0) const { return std::accumulate(_data, _data + (n == 0 ? _size : n), 0); }
    pub fn sum(&self, n: Option<usize>) -> PatternType {
        if self.count == self.data.len() {
            return self.data.0.iter().sum::<PatternType>();
        }

        let n = n.unwrap_or(self.count);

        self.data
            .0
            .iter()
            .skip(self.start + self.current)
            .take(n)
            .copied()
            .sum::<PatternType>()
    }

    pub fn iter(&'a self) -> PatternViewIterator<'a> {
        PatternViewIterator {
            pattern_view: self,
            current_position: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.count
    }

    // index is the number of bars and spaces from the first bar to the current position
    pub fn index(&self) -> usize {
        self.current /*return narrow_cast<int>(_data - _base) - 1;*/
    }
    pub fn pixelsInFront(&self) -> PatternType {
        self.data
            .0
            .iter()
            .take(self.start + self.current)
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
            && self.start <= self.current + self.start
            && self.current + n <= (self.data.0.len())
        /*return _data && _data >= _base && _data + n <= _end;*/
    }
    pub fn isValid(&self) -> bool {
        self.isValidWithN(self.size())
    }

    pub fn has_quiet_zone_before(&self, scale: f32, acceptIfAtFirstBar: Option<bool>) -> bool {
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

    pub fn subView(&self, offset: usize, size: Option<usize>) -> PatternView<'a> {
        let mut size = size.unwrap_or(0);
        if size == 0 {
            size = self.count - offset;
        } else if size < 0 {
            size += self.count - offset;
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
        self.current += n;
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
        self.count = std::cmp::max(0, self.data.len() - (self.current + self.start))
    }

    fn try_get_index(&self, index: isize) -> Option<PatternType> {
        if index.abs() > self.data.0.len() as isize {
            return None;
        }
        if index >= 0 {
            let fetch_spot = ((self.start + self.current) as isize + index) as usize;
            return Some(self.data.0[fetch_spot]);
        }
        if index.abs() > (self.start + self.current) as isize {
            return None;
        }
        let fetch_spot = ((self.start + self.current) as isize + index) as usize;
        Some(self.data.0[fetch_spot])
    }
}

impl<'a> std::ops::Index<isize> for PatternView<'_> {
    type Output = PatternType;

    fn index(&self, index: isize) -> &Self::Output {
        if self.count == self.data.len() {
            return &self.data[index.abs() as usize];
        }

        if index > self.data.0.len() as isize {
            panic!("array index out of bounds")
        }
        if index >= 0 {
            let fetch_spot = ((self.start + self.current) as isize + index) as usize;
            return &self.data.0[fetch_spot];
        }
        if index.abs() > self.start as isize {
            panic!("array index out of bounds")
        }
        let fetch_spot = ((self.start + self.current) as isize + index) as usize;
        &self.data.0[fetch_spot]
    }
}

impl<'a> std::ops::Index<usize> for PatternView<'_> {
    type Output = PatternType;

    fn index(&self, index: usize) -> &Self::Output {
        if self.count == self.data.len() {
            return &self.data[index];
        }

        if index > self.data.0.len() {
            panic!("array index out of bounds")
        }
        self.data.0.get(self.start + self.current + index).unwrap()
    }
}

impl<'a> std::ops::Index<i32> for PatternView<'_> {
    type Output = PatternType;

    fn index(&self, index: i32) -> &Self::Output {
        std::ops::Index::<isize>::index(self, index as isize)
    }
}

impl<'a> Into<Vec<PatternType>> for &PatternView<'a> {
    fn into(self) -> Vec<PatternType> {
        let mut v = vec![PatternType::default(); self.count as usize];
        for i in 0..self.count {
            v[i] = self[i];
        }
        v
    }
}

/**
 * @brief The BarAndSpace struct is a simple 2 element data structure to hold information about bar(s) and space(s).
 *
 * The operator[](int) can be used in combination with a PatternView
 */
#[derive(Default)]
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
        match index & 1 {
            0 => &self.bar,
            1 => &self.space,
            _ => panic!("Index out of range for BarAndSpace"),
        }
    }
}

impl<T: Default + std::cmp::PartialEq> std::ops::IndexMut<usize> for BarAndSpace<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index & 1 {
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

type BarAndSpaceI = BarAndSpace<PatternType>;

/**
 * @brief FixedPattern describes a compile-time constant (start/stop) pattern.
 *
 * @param N  number of bars/spaces
 * @param SUM  sum over all N elements (size of pattern in modules)
 * @param IS_SPARCE  whether or not the pattern contains '0's denoting 'wide' bars/spaces
 */
pub struct FixedPattern<const N: usize, const SUM: usize, const IS_SPARCE: bool = false> {
    data: [PatternType; N],
}

impl<const N: usize, const SUM: usize, const IS_SPARCE: bool> Into<Pattern<N>>
    for FixedPattern<N, SUM, IS_SPARCE>
{
    fn into(self) -> Pattern<N> {
        self.data
    }
}

impl<const N: usize, const SUM: usize, const IS_SPARCE: bool> FixedPattern<N, SUM, IS_SPARCE> {
    pub const fn new(data: [PatternType; N]) -> Self {
        FixedPattern { data }
    }

    pub fn with_reference(data: &[PatternType; N]) -> Self {
        FixedPattern { data: *data }
    }

    fn as_slice(&self) -> &[PatternType] {
        &self.data
    }

    fn size(&self) -> usize {
        N
    }

    fn sums(&self) -> BarAndSpace<PatternType> {
        return BarAndSpaceSum::<N, PatternType, PatternType>(&self.data);
    }
}

impl<const N: usize, const SUM: usize, const IS_SPARCE: bool> std::ops::Index<usize>
    for FixedPattern<N, SUM, IS_SPARCE>
{
    type Output = PatternType;

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

pub fn IsPattern<const E2E: bool, const LEN: usize, const SUM: usize, const SPARSE: bool>(
    view: &PatternView,
    pattern: &FixedPattern<LEN, SUM, SPARSE>,
    space_in_pixel: Option<f32>,
    min_quiet_zone: f32,
    module_size_ref: f32,
    // e2e: Option<bool>,
) -> f32 {
    //let e2e = E2E; //e2e.unwrap_or(false);
    let mut module_size_ref = module_size_ref;

    if E2E {
        //using float_t = double;
        let v_src: Vec<PatternType> = view.into();
        let widths = BarAndSpaceSum::<LEN, PatternType, f64>(&v_src);
        let sums = pattern.sums();
        let modSize: BarAndSpace<f64> = BarAndSpace {
            bar: widths[0] / sums[0] as f64,
            space: widths[1] / sums[1] as f64,
        };

        let [m, M] = [
            f64::min(modSize[0], modSize[1]),
            f64::max(modSize[0], modSize[1]),
        ];
        if M > 4.0 * m {
            // make sure module sizes of bars and spaces are not too far away from each other
            return 0.0;
        }

        if min_quiet_zone != 0.0
            && (space_in_pixel.unwrap_or_default()) < min_quiet_zone * modSize.space as f32
        {
            return 0.0;
        }

        let thr: BarAndSpace<f64> = BarAndSpace {
            bar: modSize[0] * 0.75 + 0.5,
            space: modSize[1] / (2.0 + f64::from(LEN < 6)) + 0.5,
        };

        for x in 0..LEN {
            // for (int x = 0; x < LEN; ++x){
            if (view[x] as f64 - pattern[x] as f64 * modSize[x]).abs() > thr[x] {
                return 0.0;
            }
        }

        let moduleSize: f64 = (modSize[0] + modSize[1]) / 2.0;
        return moduleSize as f32;
    }

    let width = view.sum(Some(LEN));
    if SUM > LEN && Into::<usize>::into(width) < SUM {
        return 0.0;
    }

    let module_size: f32 = (Into::<f32>::into(width)) / (SUM as f32);

    if min_quiet_zone != 0.0
        && (space_in_pixel.unwrap_or(f32::MAX)) < min_quiet_zone * module_size - 1.0
    {
        return 0.0;
    }

    if module_size_ref == 0.0 {
        module_size_ref = module_size;
    }

    let threshold = module_size_ref * (0.5 + (E2E as u8) as f32 * 0.25) + 0.5;

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

    const E2E: bool = false;

    IsPattern::<E2E, N, SUM, IS_SPARCE>(
        view,
        pattern,
        spaceInPixel,
        minQuietZone,
        moduleSizeRef,
        // None,
    ) != 0.0
}

pub fn FindLeftGuardBy<'a, const LEN: usize, Pred: Fn(&PatternView, Option<f32>) -> bool>(
    view: PatternView<'a>,
    minSize: usize,
    isGuard: Pred,
) -> Result<PatternView<'a>> {
    const PREV_IDX: isize = -1;

    if view.size() < minSize {
        return Err(Exceptions::ILLEGAL_STATE);
    }

    let mut window = view.subView(0, Some(LEN));
    if window.isAtFirstBar() && isGuard(&window, Some(f32::MAX)) {
        return Ok(window);
    }
    let end = Into::<usize>::into(view.end().ok_or(Exceptions::INDEX_OUT_OF_BOUNDS)?) - minSize;
    while (window.start + window.current) < end {
        let prev = if let Some(v) = window.try_get_index(PREV_IDX) {
            Some(v as f32)
        } else {
            None
        };
        if isGuard(&window, prev) {
            return Ok(window);
        }

        window.skipPair();
    }

    Err(Exceptions::ILLEGAL_STATE)
}

pub fn FindLeftGuard<'a, const LEN: usize, const SUM: usize, const IS_SPARCE: bool>(
    view: PatternView<'a>,
    minSize: usize,
    pattern: &FixedPattern<LEN, SUM, IS_SPARCE>,
    minQuietZone: f32,
) -> Result<PatternView<'a>> {
    FindLeftGuardBy::<LEN, _>(view, std::cmp::max(minSize, LEN), |window, spaceInPixel| {
        // perform a fast plausability test for 1:1:3:1:1 pattern
        // dbg!(window[2], 2 as PatternType * std::cmp::max(window[0], window[4]));
        // dbg!(window[2] < std::cmp::max(window[1], window[3]));
        if window[2] < 2 as PatternType * std::cmp::max(window[0], window[4])
            || window[2] < std::cmp::max(window[1], window[3])
        {
            return false;
        }
        return IsPattern::<false, LEN, SUM, IS_SPARCE>(
            window,
            pattern,
            spaceInPixel,
            minQuietZone,
            0.0,
        ) != 0.0;
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

    e2e
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

    if err.abs() > 1 {
        return Err(Exceptions::NOT_FOUND);
    }

    if err != 0 {
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
        is[*mi as usize] += err as PatternType;
        rs[*mi as usize] -= err as f32;
    }

    Ok(is)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color {
    White = 0,
    Black = 1,
}

impl<T: Into<PatternType>> From<T> for Color {
    fn from(value: T) -> Self {
        match value.into() {
            0 => Color::White,
            _ => Color::Black,
        }
    }
}

pub fn GetPatternRowTP(matrix: &BitMatrix, r: u32, pr: &mut PatternRow, transpose: bool) {
    let row = if transpose {
        matrix.getCol(r)
    } else {
        matrix.getRow(r)
    };

    let pixel_states: Vec<bool> = row.into();

    GetPatternRow(&pixel_states, pr)
}

pub fn GetPatternRow<T: Into<PatternType> + Copy + Default + From<T>>(
    b_row: &[T],
    p_row: &mut PatternRow,
) {
    p_row.0.clear();

    if Color::from(p_row.0.first().copied().unwrap_or_default()) == Color::Black {
        // first
        p_row.0.push(0);
    }

    let mut current_color = Color::from(p_row.0.first().copied().unwrap_or_default()); //if p_row.0.first().copied().unwrap_or_default() == 1 {Color::Black} else { Color::White};
    let mut count = 0;

    for bit in b_row.iter() {
        let this_color = Color::from(*bit);

        if current_color != this_color {
            p_row.0.push(count);
            count = 0;

            current_color = this_color;
        }

        count += 1;
    }

    // dbg!(&p_row.0);

    if count != 0 {
        p_row.0.push(count);
    }

    if current_color == Color::Black {
        p_row.0.push(0);
    }
}

#[cfg(test)]
mod tests {
    use crate::common::cpp_essentials::PatternType;

    use super::{GetPatternRow, PatternRow, PatternView};
    const N: usize = 33;

    #[test]
    fn all_white() {
        for s in 1..=N {
            // for (int s = 1; s <= N; ++s) {
            let t_in: Vec<PatternType> = vec![0; s];
            // std::vector<uint8_t> in(s, 0);
            let mut pr = PatternRow::default();
            GetPatternRow(&t_in, &mut pr);

            assert_eq!(pr.0.len(), 1);
            assert_eq!(pr.0[0], s as PatternType);
        }
    }

    #[test]
    fn all_black() {
        for s in 1..=N {
            // for (int s = 1; s <= N; ++s) {
            let t_in: Vec<PatternType> = vec![0xff; s];
            let mut pr = PatternRow::default();
            GetPatternRow(&t_in, &mut pr);

            assert_eq!(pr.0.len(), 3);
            assert_eq!(pr.0[0], 0);
            assert_eq!(pr.0[1], s as PatternType);
            assert_eq!(pr.0[2], 0);
        }
    }

    #[test]
    fn black_white() {
        for s in 1..=N {
            // for (int s = 1; s <= N; ++s) {
            let mut t_in: Vec<PatternType> = vec![0; N];
            t_in[..s].copy_from_slice(&vec![1; s]);
            // std::fill_n(in.data(), s, 0xff);
            let mut pr = PatternRow::default();
            GetPatternRow(&t_in, &mut pr);

            assert_eq!(pr.0.len(), 3);
            assert_eq!(pr.0[0], 0);
            assert_eq!(pr.0[1], s as PatternType);
            assert_eq!(pr.0[2], (N - s) as PatternType);
        }
    }

    #[test]
    fn white_black() {
        for s in 0..N {
            // for (int s = 0; s < N; ++s) {
            let mut t_in: Vec<PatternType> = vec![0xff; N];
            t_in[..s].copy_from_slice(&vec![0; s]);
            let mut pr = PatternRow::default();
            GetPatternRow(&t_in, &mut pr);

            assert_eq!(pr.0.len(), 3);
            assert_eq!(pr.0[0], s as PatternType);
            assert_eq!(pr.0[1], (N - s) as PatternType);
            assert_eq!(pr.0[2], 0);
        }
    }

    #[test]
    fn basic_pattern_view() {
        let mut p_row = PatternRow::default();
        GetPatternRow(
            &[
                0_u16, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1,
            ],
            &mut p_row,
        );

        let mut pv = PatternView::new(&p_row);

        assert_eq!(pv.data().0, p_row.0);

        assert_eq!(pv[0], 1_u16);
        assert_eq!(pv[1], 1_u16);
        assert_eq!(pv[4], 2_u16);
        assert_eq!(pv[7], 6_u16);

        assert_eq!(pv.index(), 0);
        assert!(pv.shift(1));
        assert_eq!(pv.index(), 1);
        assert!(pv.skipPair());
        assert_eq!(pv.index(), 3);
    }
}
