use crate::common::BitArray;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct BitArrayRLE {
    pub counts: Vec<usize>,
    pub size: usize,
}

impl BitArrayRLE {
    pub fn new() -> Self {
        Self::default()
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
            } else {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::BitArray;

    #[test]
    fn test_new() {
        let rle = BitArrayRLE::new();
        assert_eq!(rle.size, 0);
        assert!(rle.counts.is_empty());
        assert_eq!(rle.counts.len(), 0);
    }

    #[test]
    fn test_default() {
        let rle = BitArrayRLE::default();
        assert_eq!(rle.size, 0);
        assert!(rle.counts.is_empty());
        assert_eq!(rle, BitArrayRLE::new());
    }

    #[test]
    fn test_clone_and_eq() {
        let mut rle1 = BitArrayRLE::new();
        rle1.size = 10;
        rle1.counts = vec![3, 7];

        let rle2 = rle1.clone();
        assert_eq!(rle1, rle2);

        let rle3 = BitArrayRLE {
            size: 10,
            counts: vec![4, 6],
        };
        assert_ne!(rle1, rle3);

        let rle4 = BitArrayRLE {
            size: 11,
            counts: vec![3, 7],
        };
        assert_ne!(rle1, rle4);
    }

    #[test]
    fn test_debug_format() {
        let rle = BitArrayRLE {
            counts: vec![2, 5],
            size: 7,
        };
        let debug_str = format!("{rle:?}");
        assert!(debug_str.contains("BitArrayRLE"));
        assert!(debug_str.contains("counts"));
        assert!(debug_str.contains("[2, 5]"));
        assert!(debug_str.contains("7"));
    }

    #[test]
    fn test_empty_bit_array() {
        let ba = BitArray::new();
        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 0);
        assert!(rle.counts.is_empty());

        let rle_into: BitArrayRLE = (&ba).into();
        assert_eq!(rle_into.size, 0);
        assert!(rle_into.counts.is_empty());
        assert_eq!(rle, rle_into);
    }

    #[test]
    fn test_single_bit_unset() {
        let ba = BitArray::with_size(1);
        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 1);
        assert_eq!(rle.counts, vec![1]);
    }

    #[test]
    fn test_single_bit_set() {
        let mut ba = BitArray::with_size(1);
        ba.set(0);
        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 1);
        assert_eq!(rle.counts, vec![1]);
    }

    #[test]
    fn test_all_unset_bits() {
        for size in [2, 5, 16, 31, 32, 33, 63, 64, 65, 100, 256] {
            let ba = BitArray::with_size(size);
            let rle = BitArrayRLE::from(&ba);
            assert_eq!(rle.size, size, "size mismatch for all-unset size {size}");
            assert_eq!(
                rle.counts,
                vec![size],
                "counts mismatch for all-unset size {size}"
            );
        }
    }

    #[test]
    fn test_all_set_bits() {
        for size in [2, 5, 16, 31, 32, 33, 63, 64, 65, 100, 256] {
            let mut ba = BitArray::with_size(size);
            for i in 0..size {
                ba.set(i);
            }
            let rle = BitArrayRLE::from(&ba);
            assert_eq!(rle.size, size, "size mismatch for all-set size {size}");
            assert_eq!(
                rle.counts,
                vec![size],
                "counts mismatch for all-set size {size}"
            );
        }
    }

    #[test]
    fn test_alternating_single_bits_starting_unset() {
        // 01010101 (8 bits)
        let mut ba = BitArray::with_size(8);
        ba.set(1);
        ba.set(3);
        ba.set(5);
        ba.set(7);

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 8);
        assert_eq!(rle.counts, vec![1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);
    }

    #[test]
    fn test_alternating_single_bits_starting_set() {
        // 10101010 (8 bits)
        let mut ba = BitArray::with_size(8);
        ba.set(0);
        ba.set(2);
        ba.set(4);
        ba.set(6);

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 8);
        assert_eq!(rle.counts, vec![1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);
    }

    #[test]
    fn test_multiple_runs_starting_unset() {
        // 000 11111 00 1111111 0 (3 unsets, 5 sets, 2 unsets, 7 sets, 1 unset) -> size 18
        let mut ba = BitArray::with_size(18);
        for i in 3..8 {
            ba.set(i);
        }
        for i in 10..17 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 18);
        assert_eq!(rle.counts, vec![3, 5, 2, 7, 1]);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);
    }

    #[test]
    fn test_multiple_runs_starting_set() {
        // 1111 00 111111 000 (4 sets, 2 unsets, 6 sets, 3 unsets) -> size 15
        let mut ba = BitArray::with_size(15);
        for i in 0..4 {
            ba.set(i);
        }
        for i in 6..12 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 15);
        assert_eq!(rle.counts, vec![4, 2, 6, 3]);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);
    }

    #[test]
    fn test_runs_spanning_word_boundaries() {
        // Word boundaries are typically 32 or 64 bits.
        // Run 1: 0..20 unset (len 20)
        // Run 2: 20..75 set (len 55, spans across 32 and 64)
        // Run 3: 75..120 unset (len 45, spans across 96)
        let mut ba = BitArray::with_size(120);
        for i in 20..75 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 120);
        assert_eq!(rle.counts, vec![20, 55, 45]);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);
    }

    #[test]
    fn test_transitions_at_exact_word_boundaries() {
        // Exactly at bit 32 and bit 64:
        // [0..32 unset, 32..64 set, 64..96 unset] -> size 96
        let mut ba = BitArray::with_size(96);
        for i in 32..64 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 96);
        assert_eq!(rle.counts, vec![32, 32, 32]);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);

        // Inverted: [0..32 set, 32..64 unset, 64..96 set] -> size 96
        let mut ba_inv = BitArray::with_size(96);
        for i in 0..32 {
            ba_inv.set(i);
        }
        for i in 64..96 {
            ba_inv.set(i);
        }

        let rle_inv = BitArrayRLE::from(&ba_inv);
        assert_eq!(rle_inv.size, 96);
        assert_eq!(rle_inv.counts, vec![32, 32, 32]);
        assert_eq!(rle_inv.counts.iter().sum::<usize>(), rle_inv.size);
    }

    #[test]
    fn test_off_by_one_word_boundaries() {
        // Run transitions 1 bit before/after 32:
        // 0..31 unset (31 bits)
        // 31..65 set (34 bits)
        // 65..96 unset (31 bits)
        let mut ba = BitArray::with_size(96);
        for i in 31..65 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 96);
        assert_eq!(rle.counts, vec![31, 34, 31]);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);

        // 0..33 unset (33 bits)
        // 33..63 set (30 bits)
        // 63..96 unset (33 bits)
        let mut ba2 = BitArray::with_size(96);
        for i in 33..63 {
            ba2.set(i);
        }

        let rle2 = BitArrayRLE::from(&ba2);
        assert_eq!(rle2.size, 96);
        assert_eq!(rle2.counts, vec![33, 30, 33]);
        assert_eq!(rle2.counts.iter().sum::<usize>(), rle2.size);
    }

    #[test]
    fn test_single_bit_islands() {
        // Single set bits surrounded by unset runs
        // 00000 1 00000 1 00000 -> size 13
        let mut ba = BitArray::with_size(13);
        ba.set(5);
        ba.set(11);

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, 13);
        assert_eq!(rle.counts, vec![5, 1, 5, 1, 1]);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);

        // Single unset bits surrounded by set runs
        // 11111 0 11111 0 11111 -> size 13
        let mut ba_inv = BitArray::with_size(13);
        for i in 0..13 {
            ba_inv.set(i);
        }
        ba_inv.flip(5);
        ba_inv.flip(11);

        let rle_inv = BitArrayRLE::from(&ba_inv);
        assert_eq!(rle_inv.size, 13);
        assert_eq!(rle_inv.counts, vec![5, 1, 5, 1, 1]);
        assert_eq!(rle_inv.counts.iter().sum::<usize>(), rle_inv.size);
    }

    #[test]
    fn test_simulated_barcode_row() {
        // Typical 1D barcode row: quiet zone + start pattern + data + stop pattern + quiet zone
        let expected_runs = vec![10, 2, 1, 2, 1, 3, 2, 1, 4, 2, 1, 2, 1, 2, 10];
        let total_size: usize = expected_runs.iter().sum();

        let mut ba = BitArray::with_size(total_size);
        let mut pos = 0;
        let mut is_set = false;
        for &run_len in &expected_runs {
            if is_set {
                for i in pos..(pos + run_len) {
                    ba.set(i);
                }
            }
            pos += run_len;
            is_set = !is_set;
        }

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, total_size);
        assert_eq!(rle.counts, expected_runs);
        assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);
    }

    #[test]
    fn test_reconstruction_property() {
        // Property test: For various bit array patterns, reconstructing the bit array
        // from rle.counts must match the original bit values bit-for-bit.
        let test_patterns: Vec<Vec<bool>> = vec![
            vec![],
            vec![false],
            vec![true],
            vec![false, false],
            vec![true, true],
            vec![false, true],
            vec![true, false],
            vec![true, false, true, false, true],
            vec![
                false, true, true, false, false, false, true, true, true, true,
            ],
            (0..150).map(|i| ((i * 7 + 3) % 11) > 5).collect(),
        ];

        for pattern in test_patterns {
            let mut ba = BitArray::with_size(pattern.len());
            for (i, &bit) in pattern.iter().enumerate() {
                if bit {
                    ba.set(i);
                }
            }

            let rle = BitArrayRLE::from(&ba);
            assert_eq!(rle.size, pattern.len());
            assert_eq!(rle.counts.iter().sum::<usize>(), rle.size);

            if !pattern.is_empty() {
                let mut current_bit = ba.get(0);
                let mut bit_index = 0;
                for &count in &rle.counts {
                    assert!(count > 0, "run counts must always be non-zero");
                    for _ in 0..count {
                        assert_eq!(
                            ba.get(bit_index),
                            current_bit,
                            "bit mismatch at index {bit_index}"
                        );
                        bit_index += 1;
                    }
                    current_bit = !current_bit;
                }
                assert_eq!(bit_index, pattern.len());
            } else {
                assert!(rle.counts.is_empty());
            }
        }
    }

    #[test]
    fn test_large_alternating_array() {
        // 500 alternating bits (010101...)
        let size = 500;
        let mut ba = BitArray::with_size(size);
        for i in 0..size {
            if i % 2 == 1 {
                ba.set(i);
            }
        }

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.size, size);
        assert_eq!(rle.counts.len(), size);
        assert!(rle.counts.iter().all(|&c| c == 1));
        assert_eq!(rle.counts.iter().sum::<usize>(), size);
    }
}
