use super::BitArray;

#[derive(Debug, Default)]
pub struct Pattern {
    pattern: Vec<usize>,
    // soruce: &'a BitArray
}

pub struct PatternReader<'a> {
    pattern_array: &'a Pattern,
    length: usize,
    position: usize,
}

impl Pattern {
    pub fn new(source: &BitArray) -> Self {
        Pattern {
            pattern: build_pattern(source),
        }
    }

    pub fn iter(&self, length: usize) -> PatternReader {
        PatternReader::new(&self, length)
    }
}

impl<'a> PatternReader<'a> {
    pub fn new(source: &Pattern, length: usize) -> PatternReader {
        PatternReader {
            pattern_array: source,
            length,
            position: 0,
        }
    }

    pub fn read(&mut self) -> Option<&'a [usize]> {
        let end_point = self.position + self.length;

        if end_point > self.pattern_array.pattern.len() {
            return None;
        }

        let ret = &self.pattern_array.pattern[self.position..end_point];
        self.position += 1;
        Some(ret)
    }

    pub fn read_constant<const PATTERN_SIZE: usize>(&mut self) -> Option<[usize; PATTERN_SIZE]> {
        assert_eq!(PATTERN_SIZE, self.length);

        let end_point = self.position + PATTERN_SIZE;

        if end_point > self.pattern_array.pattern.len() {
            return None;
        }

        let mut ret = [0; PATTERN_SIZE];
        ret.copy_from_slice(&self.pattern_array.pattern[self.position..end_point]);

        self.position += 1;
        Some(ret)
    }
}

impl<'a> Iterator for PatternReader<'a> {
    type Item = &'a [usize];

    fn next(&mut self) -> Option<Self::Item> {
        self.read()
    }
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

#[cfg(test)]
mod test {
    use crate::common::{pattern_reader::Pattern, BitArray};

    #[test]
    fn test_build_pattern() {
        let mut bit_array = BitArray::new();
        bit_array
            .appendBits(0b00001111001101, 14)
            .expect("must build array");
        let pattern = Pattern::new(&bit_array);
        let p_a = pattern;
        assert_eq!(&p_a.pattern, &[4, 4, 2, 2, 1, 1]);
    }

    #[test]
    fn test_pattern_reader() {
        let mut bit_array = BitArray::new();
        bit_array
            .appendBits(0b00001111001101, 14)
            .expect("must build array");
        let pattern = Pattern::new(&bit_array);
        let mut p_r = pattern.iter(3); //PatternReader::new(&pattern, 3);
        assert_eq!(p_r.next().unwrap(), &[4, 4, 2]);
        assert_eq!(p_r.next().unwrap(), &[4, 2, 2]);
        assert_eq!(p_r.next().unwrap(), &[2, 2, 1]);
        assert_eq!(p_r.next().unwrap(), &[2, 1, 1]);
        assert_eq!(p_r.next(), None);
    }

    #[test]
    fn test_pattern_reader_too_long() {
        let mut bit_array = BitArray::new();
        bit_array
            .appendBits(0b00001111001101, 14)
            .expect("must build array");
        let pattern = Pattern::new(&bit_array);

        let mut p_r = pattern.iter(6); //PatternReader::new(&pattern, 6);
        assert_eq!(p_r.next().unwrap(), &[4, 4, 2, 2, 1, 1]);
        assert_eq!(p_r.next(), None);

        let mut p_r = pattern.iter(7); //PatternReader::new(&pattern, 7);
        assert_eq!(p_r.next(), None);
    }
}
