use crate::common::BitArray;

pub struct BitArrayRLE {
    pub counts: Vec<usize>,
    pub size: usize,
}

impl BitArrayRLE {
    pub fn new() -> Self {
        BitArrayRLE {
            counts: Vec::new(),
            size: 0,
        }
    }
}

impl From<&BitArray> for BitArrayRLE {
    fn from(bit_array: &BitArray) -> Self {
        let mut counts = Vec::new();
        let array_length = bit_array.get_size();
        let mut position = 0;
        while position < array_length {
            let current_bit = bit_array.get(position);
            let count = if current_bit {
                let pos = bit_array.getNextUnset(position);
                pos - position
            }else {
                let pos = bit_array.getNextSet(position);
                pos - position
            };
            counts.push(count);
            position += count;
        }

        BitArrayRLE {
            counts,
            size: array_length,
        }
    }
}

