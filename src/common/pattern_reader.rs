#![allow(dead_code)]
use super::BitArray;

#[derive(PartialEq, Eq, Debug, Hash)]
pub struct Pattern<const PATTERN_SIZE: usize>([usize; PATTERN_SIZE]);

impl<const PATTERN_SIZE: usize> Pattern<PATTERN_SIZE> {
    pub fn calculate_variance(
        &self,
        reference: &Pattern<PATTERN_SIZE>,
        max_individual_variance: f32,
    ) -> f32 {
        let total: f32 = reference.0.iter().sum::<usize>() as f32;
        let pattern_length: usize = self.0.iter().sum::<usize>();
        if total < pattern_length as f32 {
            // If we don't even have one pixel per unit of bar width, assume this is too small
            // to reliably match, so fail:
            return f32::INFINITY;
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
                return f32::INFINITY;
            }
            total_variance += variance;
        }
        total_variance / total
    }
}

pub struct PatternReader<'a, const PATTERN_SIZE: usize> {
    source: &'a BitArray,
    position: usize,
    stored_pattern: [usize; PATTERN_SIZE],
    cache_internal_position: usize,
    cache_last_set_state: bool,
}

impl<'a, const PATTERN_SIZE: usize> PatternReader<'a, PATTERN_SIZE> {
    pub fn new(source: &'a BitArray) -> PatternReader<'a, PATTERN_SIZE> {
        let (stored_pattern, cache_internal_position, cache_last_set_state) =
            build_initial_pattern(source);
        PatternReader {
            source,
            position: 0,
            stored_pattern,
            cache_internal_position,
            cache_last_set_state,
        }
    }
}

fn build_initial_pattern<const PATTERN_SIZE: usize>(
    source: &BitArray,
) -> ([usize; PATTERN_SIZE], usize, bool) {
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
    (buffer, position, current)
}

impl<const PATTERN_SIZE: usize> PatternReader<'_, PATTERN_SIZE> {
    fn read_next_pattern(&mut self) -> bool {
        if self.cache_internal_position >= self.source.get_size() {
            return false;
        }

        self.stored_pattern.rotate_left(1);
        self.position += 1;
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
        let ok = if self.position != 0 {
            self.read_next_pattern()
        } else {
            self.position += 1;
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

/*

pub fn read(&mut self) -> Option<&'a [usize]> {
        let end_point = self.position + self.length;

        if end_point > self.pattern_array.pattern.len() {
            return None;
        }

        let ret = &self.pattern_array.pattern[self.position..end_point];
        self.position += 1;
        Some(ret)
    }

    fn build_pattern(source: &BitArray) -> Vec<usize> {
    let mut buffer = Vec::default();
    let total_length = source.get_size();
    let mut current = source.get(0);
    let mut position = 0;

    loop {
        let next = if current {
            source.getNextUnset(position)
        } else {
            source.getNextSet(position)
        };
        buffer.push(next - position);
        current = !current;
        position = next;
        if next >= total_length {
            break;
        }
    }
    buffer
}

*/
