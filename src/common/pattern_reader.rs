#![allow(dead_code)]
use super::BitArray;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub struct Pattern<const PATTERN_SIZE: usize>([usize; PATTERN_SIZE]);

impl<const PATTERN_SIZE: usize> Pattern<PATTERN_SIZE> {
    pub fn calculate_variance(
        &self,
        reference: &Pattern<PATTERN_SIZE>,
        max_individual_variance: f32,
    ) -> Option<f32> {
        let total: f32 = reference.0.iter().sum::<usize>() as f32;
        let pattern_length: usize = self.0.iter().sum::<usize>();
        if total < pattern_length as f32 {
            // If we don't even have one pixel per unit of bar width, assume this is too small
            // to reliably match, so fail:
            return None;
        }

        let unit_bar_width = total / pattern_length as f32;
        let max_individual_variance = max_individual_variance * unit_bar_width;

        let mut total_variance = 0.0;
        for (&counter, scaled_pattern) in reference.0.iter().zip(
            self.0
                .iter()
                .take(PATTERN_SIZE)
                .map(|&p| (p as f32) * unit_bar_width),
        ) {
            let variance = if (counter as f32) > scaled_pattern {
                counter as f32 - scaled_pattern
            } else {
                scaled_pattern - counter as f32
            };
            if variance > max_individual_variance {
                return None;
            }
            total_variance += variance;
        }
        Some(total_variance / total)
    }
}

#[derive(Copy, Clone)]
pub struct PatternReader<'a, const PATTERN_SIZE: usize> {
    source: &'a BitArray,
    position: bool,
    stored_pattern: [usize; PATTERN_SIZE],
    cache_internal_position: usize,
    cache_last_set_state: bool,
}

impl<'a, const PATTERN_SIZE: usize> PatternReader<'a, PATTERN_SIZE> {
    pub fn new(source: &'a BitArray) -> PatternReader<'a, PATTERN_SIZE> {
        let BuildInitialPatternReturn {
            stored_pattern,
            cache_internal_position,
            cache_last_set_state,
        } = build_initial_pattern(source);
        PatternReader {
            source,
            position: false,
            stored_pattern,
            cache_internal_position,
            cache_last_set_state,
        }
    }
}

struct BuildInitialPatternReturn<const PATTERN_SIZE: usize> {
    stored_pattern: [usize; PATTERN_SIZE],
    cache_internal_position: usize,
    cache_last_set_state: bool,
}

fn build_initial_pattern<const PATTERN_SIZE: usize>(
    source: &BitArray,
) -> BuildInitialPatternReturn<PATTERN_SIZE> {
    let mut buffer = [0; PATTERN_SIZE];
    let total_length = source.get_size();
    let mut current = source.get(0);
    let mut position = 0;

    for pattern_position in buffer.iter_mut() {
        // for _ in 0..pattern_length {
        let next = if current {
            source.getNextUnset(position)
        } else {
            source.getNextSet(position)
        };
        *pattern_position = next - position;
        current = !current;
        position = next;
        if next >= total_length {
            break;
        }
    }
    BuildInitialPatternReturn {
        stored_pattern: buffer,
        cache_internal_position: position,
        cache_last_set_state: current,
    }
}

impl<const PATTERN_SIZE: usize> PatternReader<'_, PATTERN_SIZE> {
    fn read_next_pattern(&mut self) -> bool {
        if self.cache_internal_position >= self.source.get_size() {
            return false;
        }

        self.stored_pattern.rotate_left(1);
        self.position = true;
        let next = if self.cache_last_set_state {
            self.source.getNextUnset(self.cache_internal_position)
        } else {
            self.source.getNextSet(self.cache_internal_position)
        };
        let val = next - self.cache_internal_position;
        self.stored_pattern[PATTERN_SIZE - 1] = val;
        self.cache_internal_position = next;
        self.cache_last_set_state = !self.cache_last_set_state;

        true
    }
}

impl<const PATTERN_SIZE: usize> Iterator for PatternReader<'_, PATTERN_SIZE> {
    type Item = Pattern<PATTERN_SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        let ok = if self.position {
            self.read_next_pattern()
        } else {
            self.position = true;
            true
        };

        if ok {
            Some(Pattern(self.stored_pattern))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::common::BitArray;

    use super::PatternReader;

    #[test]
    fn bluesky_case() {
        let data = 0b11110000111000110010;
        let mut bit_array = BitArray::with_capacity(20);
        bit_array
            .appendBits(data, 20)
            .expect("must build bit_array");
        let mut pattern_reader = PatternReader::new(&bit_array);
        assert_eq!(pattern_reader.stored_pattern, [4, 4, 3, 3]);
        assert!(pattern_reader.read_next_pattern());
        assert_eq!(pattern_reader.stored_pattern, [4, 3, 3, 2]);
        assert!(pattern_reader.read_next_pattern());
        assert_eq!(pattern_reader.stored_pattern, [3, 3, 2, 2]);
        assert!(pattern_reader.read_next_pattern());
        assert_eq!(pattern_reader.stored_pattern, [3, 2, 2, 1]);
        assert!(pattern_reader.read_next_pattern());
        assert_eq!(pattern_reader.stored_pattern, [2, 2, 1, 1]);
        assert!(!pattern_reader.read_next_pattern())
    }

    #[test]
    fn iterator_case() {
        let data = 0b11110000111000110010;
        let mut bit_array = BitArray::with_capacity(20);
        bit_array
            .appendBits(data, 20)
            .expect("must build bit_array");
        let mut pattern_reader = PatternReader::new(&bit_array);
        assert_eq!(pattern_reader.next().unwrap().0, [4, 4, 3, 3]);
        assert_eq!(pattern_reader.next().unwrap().0, [4, 3, 3, 2]);
        assert_eq!(pattern_reader.next().unwrap().0, [3, 3, 2, 2]);
        assert_eq!(pattern_reader.next().unwrap().0, [3, 2, 2, 1]);
        assert_eq!(pattern_reader.next().unwrap().0, [2, 2, 1, 1]);
        assert_eq!(pattern_reader.next(), None);
    }
}

#[cfg(test)]
mod more_tests {
    use super::{Pattern, PatternReader};
    use crate::common::BitArray;

    /// calculate_variance: identical patterns → zero variance
    #[test]
    fn calculate_variance_uniform() {
        let p = Pattern([1, 1, 1, 1]);
        let r = Pattern([1, 1, 1, 1]);
        // total = 4, pattern_length = 4, unit_bar_width = 1.0, no differences
        assert_eq!(p.calculate_variance(&r, 1.0), Some(0.0));
    }

    /// calculate_variance: nonzero variance, within threshold
    #[test]
    fn calculate_variance_some_variance() {
        let p = Pattern([2, 1, 1, 0]);
        let r = Pattern([1, 1, 1, 1]);
        // scaled p = [2,1,1,0], diffs = [1,0,0,1] so total_variance = 2, normalized = 0.5
        assert_eq!(p.calculate_variance(&r, 1.0), Some(0.5));
    }

    /// calculate_variance: threshold too tight → None
    #[test]
    fn calculate_variance_threshold_exceeded() {
        let p = Pattern([2, 1, 1, 0]);
        let r = Pattern([1, 1, 1, 1]);
        // each individual variance is 1, but max_individual_variance * unit = 0.2,
        // so it should short-circuit to None
        assert_eq!(p.calculate_variance(&r, 0.2), None);
    }

    /// calculate_variance: reference total smaller than pattern total → None
    #[test]
    fn calculate_variance_reference_smaller() {
        let p = Pattern([2, 2, 2, 2]);
        let r = Pattern([1, 1, 1, 1]);
        // total_ref = 4, pattern_length = 8 → immediately returns None
        assert_eq!(p.calculate_variance(&r, 1.0), None);
    }

    /// PatternReader on all-zeros → one big run then end
    #[test]
    fn pattern_reader_all_unset() {
        let mut bits = BitArray::with_capacity(8);
        bits.appendBits(0, 8).expect("build all-zero array");
        let mut reader = PatternReader::<4>::new(&bits);

        // first (and only) pattern: one run of 8 zeros, then the rest 0
        assert_eq!(reader.next().unwrap().0, [8, 0, 0, 0]);
        assert!(reader.next().is_none());
    }

    /// PatternReader on alternating bits (1010...) with PATTERN_SIZE=3
    #[test]
    fn pattern_reader_alternating_bits() {
        // 0b1010_1010 → [1,0,1,0,1,0,1,0]
        let data = 0b1010_1010u8;
        let mut bits = BitArray::with_capacity(8);
        bits.appendBits(data as usize, 8).unwrap();

        let patterns: Vec<_> = PatternReader::<3>::new(&bits).map(|p| p.0).collect();

        // Each run is length 1; with 7 transitions + initial we get 6 patterns
        let expected = vec![[1, 1, 1]; 6];
        assert_eq!(patterns, expected);
    }
}

#[cfg(test)]
mod noisy_data_tests {
    use super::{Pattern, PatternReader};
    use crate::common::BitArray;

    /// A one-pixel bump in one run (±1) should be accepted if
    /// max_individual_variance ≥ 1.0 (since we’re comparing two 4-run patterns).
    #[test]
    fn calc_variance_one_pixel_noise_allowed() {
        let reference = Pattern([4, 4, 4, 4]);
        let noisy = Pattern([3, 5, 4, 4]); // diffs = [1,1,0,0]
                                           // total_variance = 2, normalized = 2/16 = 0.125
        assert_eq!(noisy.calculate_variance(&reference, 1.0), Some(2.0 / 16.0));
    }

    /// That same ±1 noise is rejected if the threshold is set below 1.0
    #[test]
    fn calc_variance_one_pixel_noise_rejected_if_threshold_too_strict() {
        let reference = Pattern([4, 4, 4, 4]);
        let noisy = Pattern([3, 5, 4, 4]);
        // max_individual_variance = 0.9*1.0 = 0.9 < 1 → reject on first diff
        assert_eq!(noisy.calculate_variance(&reference, 0.9), None);
    }

    /// A two-pixel error in one run should be rejected even with threshold=1.0
    #[test]
    fn calc_variance_two_pixel_noise_rejected() {
        let reference = Pattern([4, 4, 4, 4]);
        let noisy = Pattern([2, 6, 4, 4]); // diffs = [2,2,0,0]
                                           // first variance = 2 > max_individual_variance (1.0*1.0) → reject
        assert_eq!(noisy.calculate_variance(&reference, 1.0), None);
    }

    /// Inject a stray zero into what should be a 4-pixel run of ones.
    /// That yields stored_pattern [3,1,4,4].  To accept that at all,
    /// we need a per-run threshold > 2.0 (since the zero-run is off by 3 pixels
    /// after scaling).  Here we use 3.0 so it passes.
    #[test]
    fn reader_initial_pattern_noise_allowed() {
        let mut bits = BitArray::with_capacity(12);
        bits.appendBits(0b111, 3).unwrap(); // 3 ones
        bits.appendBits(0b0, 1).unwrap(); // stray zero
        bits.appendBits(0b1111, 4).unwrap(); // 4 ones
        bits.appendBits(0, 4).unwrap(); // 4 zeros

        let reader = PatternReader::<4>::new(&bits);
        let observed = Pattern(reader.stored_pattern);
        let reference = Pattern([4, 4, 4, 4]);

        // use a high threshold so this noisy pattern is accepted
        assert!(
            observed.calculate_variance(&reference, 3.0).is_some(),
            "should accept single-pixel noise at threshold=3.0"
        );
    }

    /// And of course if we tighten it again, it should reject:
    #[test]
    fn reader_initial_pattern_noise_rejected_if_threshold_strict() {
        let mut bits = BitArray::with_capacity(12);
        bits.appendBits(0b111, 3).unwrap();
        bits.appendBits(0b0, 1).unwrap();
        bits.appendBits(0b1111, 4).unwrap();
        bits.appendBits(0, 4).unwrap();

        let reader = PatternReader::<4>::new(&bits);
        let observed = Pattern(reader.stored_pattern);
        let reference = Pattern([4, 4, 4, 4]);

        // require per-run threshold < 2.0 → reject
        assert!(
            observed.calculate_variance(&reference, 1.5).is_none(),
            "should reject single-pixel noise at threshold=1.5"
        );
    }
}
