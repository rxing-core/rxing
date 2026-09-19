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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RlePattern<const N: usize> {
    pub counts: [usize; N],
    pub size: usize,
}

impl<const N: usize> From<[usize; N]> for RlePattern<N> {
    fn from(counts: [usize; N]) -> Self {
        RlePattern { counts, size: N }
    }
}

impl<const N: usize> RlePattern<N> {
    pub fn calculate_variance(&self, array: &BitArrayRLE, start: usize) -> f64 {
        let mut variance = 0.0;
        let mut array_index = start;
        for &count in &self.counts {
            if array_index >= array.counts.len() {
                break;
            }
            let diff = count as f64 - array.counts[array_index] as f64;
            variance += diff * diff;
            array_index += 1;
        }
        variance
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

    // =========================================================================
    // Tests for RlePattern
    // =========================================================================

    #[test]
    fn test_rle_pattern_direct_construction() {
        let pattern = RlePattern {
            counts: [1, 2, 3],
            size: 3,
        };
        assert_eq!(pattern.counts, [1, 2, 3]);
        assert_eq!(pattern.size, 3);

        // Pattern with custom size
        let custom = RlePattern {
            counts: [2, 4],
            size: 10,
        };
        assert_eq!(custom.counts, [2, 4]);
        assert_eq!(custom.size, 10);
    }

    #[test]
    fn test_rle_pattern_from_array() {
        let pattern = RlePattern::from([1, 3, 5, 7]);
        assert_eq!(pattern.counts, [1, 3, 5, 7]);
        assert_eq!(pattern.size, 4);

        // Zero-length array
        let empty = RlePattern::from([]);
        assert_eq!(empty.counts, []);
        assert_eq!(empty.size, 0);

        // Single element
        let single = RlePattern::from([42]);
        assert_eq!(single.counts, [42]);
        assert_eq!(single.size, 1);
    }

    #[test]
    fn test_rle_pattern_into() {
        let pattern: RlePattern<3> = [2, 4, 6].into();
        assert_eq!(pattern.counts, [2, 4, 6]);
        assert_eq!(pattern.size, 3);
    }

    #[test]
    fn test_rle_pattern_clone_and_copy() {
        let p1 = RlePattern::from([1, 2, 3]);
        let p2 = p1; // Copy semantics
        assert_eq!(p1, p2); // p1 is still valid because of Copy

        let p3 = p1.clone(); // Clone
        assert_eq!(p1, p3);
    }

    #[test]
    fn test_rle_pattern_eq_and_ne() {
        let p1 = RlePattern::from([1, 2, 3]);
        let p2 = RlePattern::from([1, 2, 3]);
        let p3 = RlePattern::from([1, 2, 4]);
        let p4 = RlePattern {
            counts: [1, 2, 3],
            size: 99,
        };

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, p4);
    }

    #[test]
    fn test_rle_pattern_debug() {
        let pattern = RlePattern::from([1, 2, 3]);
        let debug_str = format!("{pattern:?}");
        assert!(debug_str.contains("RlePattern"));
        assert!(debug_str.contains("counts"));
        assert!(debug_str.contains("[1, 2, 3]"));
        assert!(debug_str.contains("size: 3"));
    }

    #[test]
    fn test_rle_pattern_calculate_variance_exact_match() {
        let pattern = RlePattern::from([1, 2, 3, 2, 1]);
        let array = BitArrayRLE {
            counts: vec![1, 2, 3, 2, 1],
            size: 9,
        };

        let variance = pattern.calculate_variance(&array, 0);
        assert_eq!(variance, 0.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_known_differences() {
        // Pattern: [1, 5, 2]
        // Array:   [2, 3, 2]
        // Diffs: (1 - 2)^2 + (5 - 3)^2 + (2 - 2)^2 = (-1)^2 + 2^2 + 0^2 = 1 + 4 + 0 = 5.0
        let pattern = RlePattern::from([1, 5, 2]);
        let array = BitArrayRLE {
            counts: vec![2, 3, 2],
            size: 7,
        };

        let variance = pattern.calculate_variance(&array, 0);
        assert_eq!(variance, 5.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_with_start_offset() {
        let array = BitArrayRLE {
            counts: vec![99, 100, 1, 2, 3, 50],
            size: 255,
        };
        let pattern = RlePattern::from([1, 2, 3]);

        // At start = 2, counts match exactly [1, 2, 3]
        assert_eq!(pattern.calculate_variance(&array, 2), 0.0);

        // At start = 0, compares with [99, 100, 1]
        // (1 - 99)^2 + (2 - 100)^2 + (3 - 1)^2 = (-98)^2 + (-98)^2 + 2^2 = 9604 + 9604 + 4 = 19212.0
        assert_eq!(pattern.calculate_variance(&array, 0), 19212.0);

        // At start = 1, compares with [100, 1, 2]
        // (1 - 100)^2 + (2 - 1)^2 + (3 - 2)^2 = (-99)^2 + 1^2 + 1^2 = 9801 + 1 + 1 = 9803.0
        assert_eq!(pattern.calculate_variance(&array, 1), 9803.0);

        // At start = 3, compares with [2, 3, 50]
        // (1 - 2)^2 + (2 - 3)^2 + (3 - 50)^2 = (-1)^2 + (-1)^2 + (-47)^2 = 1 + 1 + 2209 = 2211.0
        assert_eq!(pattern.calculate_variance(&array, 3), 2211.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_start_at_or_past_end() {
        let array = BitArrayRLE {
            counts: vec![1, 2, 3],
            size: 6,
        };
        let pattern = RlePattern::from([1, 2]);

        // start exactly at array.counts.len()
        assert_eq!(pattern.calculate_variance(&array, 3), 0.0);

        // start past array.counts.len()
        assert_eq!(pattern.calculate_variance(&array, 4), 0.0);
        assert_eq!(pattern.calculate_variance(&array, 100), 0.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_partial_overlap() {
        // Pattern has 4 elements, but array only has 2 elements from start.
        // The loop breaks when array_index reaches array.counts.len().
        let pattern = RlePattern::from([2, 3, 4, 5]);
        let array = BitArrayRLE {
            counts: vec![1, 5],
            size: 6,
        };

        // start = 0: compares pattern[0..2] with array[0..2], then breaks
        // (2 - 1)^2 + (3 - 5)^2 = 1 + 4 = 5.0
        assert_eq!(pattern.calculate_variance(&array, 0), 5.0);

        // start = 1: compares pattern[0..1] with array[1..2], then breaks
        // (2 - 5)^2 = 9.0
        assert_eq!(pattern.calculate_variance(&array, 1), 9.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_empty_array() {
        let pattern = RlePattern::from([1, 2, 3]);
        let empty_array = BitArrayRLE::new();

        assert_eq!(pattern.calculate_variance(&empty_array, 0), 0.0);
        assert_eq!(pattern.calculate_variance(&empty_array, 5), 0.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_empty_pattern() {
        let empty_pattern = RlePattern::<0>::from([]);
        let array = BitArrayRLE {
            counts: vec![1, 2, 3],
            size: 6,
        };

        assert_eq!(empty_pattern.calculate_variance(&array, 0), 0.0);
        assert_eq!(empty_pattern.calculate_variance(&array, 1), 0.0);
        assert_eq!(empty_pattern.calculate_variance(&array, 5), 0.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_single_element() {
        let pattern = RlePattern::from([7]);

        let array_match = BitArrayRLE {
            counts: vec![7],
            size: 7,
        };
        assert_eq!(pattern.calculate_variance(&array_match, 0), 0.0);

        let array_diff = BitArrayRLE {
            counts: vec![10],
            size: 10,
        };
        // (7 - 10)^2 = 9.0
        assert_eq!(pattern.calculate_variance(&array_diff, 0), 9.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_symmetry_of_diffs() {
        // Positive and negative differences should both produce squared positive variance
        let pattern_pos = RlePattern::from([10, 4]);
        let pattern_neg = RlePattern::from([4, 10]);
        let array = BitArrayRLE {
            counts: vec![7, 7],
            size: 14,
        };

        // (10 - 7)^2 + (4 - 7)^2 = 9 + 9 = 18.0
        assert_eq!(pattern_pos.calculate_variance(&array, 0), 18.0);
        // (4 - 7)^2 + (10 - 7)^2 = 9 + 9 = 18.0
        assert_eq!(pattern_neg.calculate_variance(&array, 0), 18.0);
    }

    #[test]
    fn test_rle_pattern_calculate_variance_large_values() {
        let pattern = RlePattern::from([10_000, 50_000, 100_000]);
        let array = BitArrayRLE {
            counts: vec![10_010, 49_990, 100_005],
            size: 160_005,
        };

        // (-10)^2 + (10)^2 + (-5)^2 = 100 + 100 + 25 = 225.0
        assert_eq!(pattern.calculate_variance(&array, 0), 225.0);
    }

    #[test]
    fn test_rle_pattern_barcode_finder_pattern_scan() {
        // Simulate scanning a 1D row for a QR-like 1:1:3:1:1 pattern
        // quiet: 5 unset, pattern: 1 set, 1 unset, 3 set, 1 unset, 1 set, trailing: 6 unset
        let finder = RlePattern::from([1, 1, 3, 1, 1]);

        let mut ba = BitArray::with_size(18);
        // 0..5 unset (count 5)
        // 5..6 set (count 1)
        ba.set(5);
        // 6..7 unset (count 1)
        // 7..10 set (count 3)
        ba.set(7);
        ba.set(8);
        ba.set(9);
        // 10..11 unset (count 1)
        // 11..12 set (count 1)
        ba.set(11);
        // 12..18 unset (count 6)

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.counts, vec![5, 1, 1, 3, 1, 1, 6]);

        // At start = 1, it should find an exact match with 0 variance
        assert_eq!(finder.calculate_variance(&rle, 1), 0.0);

        // At start = 0, compares [1, 1, 3, 1, 1] with [5, 1, 1, 3, 1]
        // (1-5)^2 + (1-1)^2 + (3-1)^2 + (1-3)^2 + (1-1)^2 = 16 + 0 + 4 + 4 + 0 = 24.0
        assert_eq!(finder.calculate_variance(&rle, 0), 24.0);

        // At start = 2, compares [1, 1, 3, 1, 1] with [1, 3, 1, 1, 6]
        // (1-1)^2 + (1-3)^2 + (3-1)^2 + (1-1)^2 + (1-6)^2 = 0 + 4 + 4 + 0 + 25 = 33.0
        assert_eq!(finder.calculate_variance(&rle, 2), 33.0);

        // Scan all start positions and verify start = 1 has minimum variance
        let mut min_variance = f64::MAX;
        let mut best_start = None;
        for start in 0..rle.counts.len() {
            // Only consider full windows
            if start + finder.counts.len() <= rle.counts.len() {
                let v = finder.calculate_variance(&rle, start);
                if v < min_variance {
                    min_variance = v;
                    best_start = Some(start);
                }
            }
        }
        assert_eq!(best_start, Some(1));
        assert_eq!(min_variance, 0.0);
    }
}
