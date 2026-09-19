use crate::common::BitArray;

/// A row of a [`BitArray`] stored as run lengths instead of individual bits.
///
/// Scanning a row run-by-run rather than bit-by-bit is what makes a 1D reader
/// cheap: the runs are what every reader actually wants, and word-scanning the
/// bit array to find them is far cheaper than testing each bit.
///
/// # White-first convention
///
/// Run lengths alone are ambiguous -- `[1]` could be one white pixel or one
/// black one -- so the sequence always *opens and closes on a white run*,
/// emitting a zero-length run where the row itself starts or ends black. Hence:
///
/// * even indices are white runs, odd indices are black runs;
/// * the length is always odd (for a non-empty row), so [`reverse`] preserves
///   the convention;
/// * only the first and last entries may be zero.
///
/// This matches `GetPatternRow` and zxing-cpp's `PatternRow`.
///
/// ```text
/// row:    0 0 0 1 1 0 1 1 1
/// counts: [3, 2, 1, 3, 0]
///          W  B  W  B  W  <- the trailing 0 pads the black-ending row
/// ```
///
/// [`reverse`]: BitArrayRLE::reverse
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct BitArrayRLE {
    /// Run lengths in pixels, white first. See the type docs.
    pub counts: Vec<u32>,
    /// Width of the original row in pixels; equal to the sum of `counts`.
    pub size: usize,
}

impl BitArrayRLE {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reverse the row in place.
    ///
    /// Safe with respect to colour because of the white-first convention: the
    /// run sequence always opens and closes on white and therefore always has
    /// odd length, so flipping it leaves white on even indices.
    pub fn reverse(&mut self) {
        self.counts.reverse();
    }

    /// Encode `bit_array` into this buffer, reusing its existing allocation.
    ///
    /// A 1D scan encodes one row per scanned line, so readers should hold a
    /// single `BitArrayRLE` and refill it rather than building a new one each
    /// time.
    pub fn fill_from(&mut self, bit_array: &BitArray) {
        self.counts.clear();
        self.size = bit_array.get_size();

        if self.size == 0 {
            return;
        }

        // White-first convention: emit a zero-length white run when the row
        // itself opens on black, so that even indices are always white runs and
        // odd indices always black ones.
        if bit_array.get(0) {
            self.counts.push(0);
        }

        let mut position = 0;
        while position < self.size {
            let count = if bit_array.get(position) {
                bit_array.getNextUnset(position) - position
            } else {
                bit_array.getNextSet(position) - position
            };
            self.counts.push(count as u32);
            position += count;
        }

        // ...and a closing zero-length white run when it ends on black, which
        // keeps the length odd so that reversing preserves the convention.
        if bit_array.get(self.size - 1) {
            self.counts.push(0);
        }
    }
}

impl From<&BitArray> for BitArrayRLE {
    fn from(bit_array: &BitArray) -> Self {
        let mut rle = BitArrayRLE {
            // Runs, not pixels: a barcode row is a few dozen, and a noisy one
            // still lands well under this.
            counts: Vec::with_capacity(64),
            size: 0,
        };
        rle.fill_from(bit_array);
        rle
    }
}

/// A reference pattern in module units (e.g. Code 128's `[2, 1, 2, 2, 2, 2]`),
/// matched against a window of observed pixel runs in a [`BitArrayRLE`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RlePattern<const N: usize> {
    pub counts: [u32; N],
}

impl<const N: usize> From<[u32; N]> for RlePattern<N> {
    fn from(counts: [u32; N]) -> Self {
        RlePattern { counts }
    }
}

impl<const N: usize> RlePattern<N> {
    /// Score how closely the `N` runs of `array` starting at `start` match this
    /// pattern. Lower is better; [`f32::INFINITY`] means "not a match".
    ///
    /// This mirrors `oned::one_d_reader::pattern_match_variance`, which is the
    /// scoring function every 1D reader already uses, so that a reader can be
    /// moved onto run-length input without changing which candidates it
    /// accepts. In particular the pattern is in module units while the observed
    /// runs are in pixels, so the pattern is scaled by the implied unit bar
    /// width before comparison and matching is scale-invariant.
    ///
    /// `max_individual_variance` bounds how far any single run may deviate,
    /// in module units; it is scaled by the unit bar width internally.
    pub fn calculate_variance(
        &self,
        array: &BitArrayRLE,
        start: usize,
        max_individual_variance: f32,
    ) -> f32 {
        // A short window must be rejected outright. Scoring only the runs that
        // happen to fit would make a truncated match look better than a
        // complete one, since there are fewer deviations to accumulate.
        let Some(end) = start.checked_add(N) else {
            return f32::INFINITY;
        };
        let Some(observed) = array.counts.get(start..end) else {
            return f32::INFINITY;
        };

        let pattern_length: u32 = self.counts.iter().sum();
        if pattern_length == 0 {
            // A zero-width pattern has no scale to match against.
            return f32::INFINITY;
        }

        let total: f32 = observed.iter().sum::<u32>() as f32;
        if total < pattern_length as f32 {
            // Less than one pixel per module: too small to match reliably.
            return f32::INFINITY;
        }

        let unit_bar_width = total / pattern_length as f32;
        let max_individual_variance = max_individual_variance * unit_bar_width;

        let mut total_variance = 0.0;
        for (&counter, &modules) in observed.iter().zip(self.counts.iter()) {
            let scaled_pattern = modules as f32 * unit_bar_width;
            let variance = (counter as f32 - scaled_pattern).abs();
            if variance > max_individual_variance {
                return f32::INFINITY;
            }
            total_variance += variance;
        }

        total_variance / total
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
        assert_eq!(rle.counts, vec![0, 1, 0]);
    }

    #[test]
    fn test_all_unset_bits() {
        for size in [2, 5, 16, 31, 32, 33, 63, 64, 65, 100, 256] {
            let ba = BitArray::with_size(size);
            let rle = BitArrayRLE::from(&ba);
            assert_eq!(rle.size, size, "size mismatch for all-unset size {size}");
            assert_eq!(
                rle.counts,
                vec![size as u32],
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
                vec![0, size as u32, 0],
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
        assert_eq!(rle.counts, vec![1, 1, 1, 1, 1, 1, 1, 1, 0]);
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);
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
        assert_eq!(rle.counts, vec![0, 1, 1, 1, 1, 1, 1, 1, 1]);
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);
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
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);
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
        assert_eq!(rle.counts, vec![0, 4, 2, 6, 3]);
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);
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
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);
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
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);

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
        assert_eq!(rle_inv.counts, vec![0, 32, 32, 32, 0]);
        assert_eq!(rle_inv.counts.iter().sum::<u32>() as usize, rle_inv.size);
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
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);

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
        assert_eq!(rle2.counts.iter().sum::<u32>() as usize, rle2.size);
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
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);

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
        assert_eq!(rle_inv.counts, vec![0, 5, 1, 5, 1, 1, 0]);
        assert_eq!(rle_inv.counts.iter().sum::<u32>() as usize, rle_inv.size);
    }

    #[test]
    fn test_simulated_barcode_row() {
        // Typical 1D barcode row: quiet zone + start pattern + data + stop pattern + quiet zone
        let expected_runs: Vec<u32> = vec![10, 2, 1, 2, 1, 3, 2, 1, 4, 2, 1, 2, 1, 2, 10];
        let total_size: usize = expected_runs.iter().sum::<u32>() as usize;

        let mut ba = BitArray::with_size(total_size);
        let mut pos = 0;
        let mut is_set = false;
        for &run_len in &expected_runs {
            let run_len = run_len as usize;
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
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);
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
            assert_eq!(rle.counts.iter().sum::<u32>() as usize, rle.size);

            if !pattern.is_empty() {
                // Runs always start white under the white-first convention; only
                // the first and last entries may be zero-length pads.
                let mut bit_index = 0;
                for (run, &count) in rle.counts.iter().enumerate() {
                    let is_black = run % 2 == 1;
                    if count == 0 {
                        assert!(
                            run == 0 || run == rle.counts.len() - 1,
                            "only the leading/trailing pads may be zero-length, got zero at {run}"
                        );
                    }
                    for _ in 0..count {
                        assert_eq!(
                            ba.get(bit_index),
                            is_black,
                            "bit mismatch at index {bit_index}"
                        );
                        bit_index += 1;
                    }
                }
                assert_eq!(bit_index, pattern.len());
            } else {
                assert!(rle.counts.is_empty());
            }
        }
    }

    // =========================================================================
    // White-first convention
    //
    // A run-length row records only lengths, so `[1]` is ambiguous: it could be
    // one white pixel or one black pixel. Consumers need the colour to index
    // bars vs spaces, and `reverse` needs it to stay meaningful.
    //
    // The convention (matching `GetPatternRow` and zxing-cpp's PatternRow) is
    // that the sequence always begins and ends with a white run, emitting a
    // zero-length pad where the row itself starts or ends black. Even indices
    // are therefore always white runs and odd indices always black runs, and
    // the length is always odd, which makes reversal parity-safe.
    // =========================================================================

    #[test]
    fn row_starting_black_gets_a_leading_white_pad() {
        // 111 000 -> white 0, black 3, white 3
        let mut ba = BitArray::with_size(6);
        for i in 0..3 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);

        assert_eq!(rle.counts, vec![0, 3, 3]);
    }

    #[test]
    fn row_ending_black_gets_a_trailing_white_pad() {
        // 000 111 -> white 3, black 3, white 0
        let mut ba = BitArray::with_size(6);
        for i in 3..6 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);

        assert_eq!(rle.counts, vec![3, 3, 0]);
    }

    #[test]
    fn all_black_row_is_padded_at_both_ends() {
        let mut ba = BitArray::with_size(4);
        for i in 0..4 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);

        assert_eq!(rle.counts, vec![0, 4, 0]);
    }

    #[test]
    fn single_set_and_unset_bits_no_longer_encode_identically() {
        let unset = BitArray::with_size(1);
        let mut set = BitArray::with_size(1);
        set.set(0);

        assert_eq!(BitArrayRLE::from(&unset).counts, vec![1]);
        assert_eq!(BitArrayRLE::from(&set).counts, vec![0, 1, 0]);
    }

    #[test]
    fn encoded_length_is_always_odd_so_reversal_preserves_parity() {
        for seed in 0..40u32 {
            let len = 1 + (seed as usize % 37);
            let mut ba = BitArray::with_size(len);
            for i in 0..len {
                if (i as u32 * 7 + seed) % 5 > 2 {
                    ba.set(i);
                }
            }

            let rle = BitArrayRLE::from(&ba);

            assert_eq!(
                rle.counts.len() % 2,
                1,
                "seed {seed}: expected odd run count, got {:?}",
                rle.counts
            );
        }
    }

    #[test]
    fn even_indices_are_white_runs_and_odd_indices_are_black_runs() {
        // 00 111 0 11 000
        let mut ba = BitArray::with_size(11);
        for i in 2..5 {
            ba.set(i);
        }
        for i in 6..8 {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);

        let mut position = 0;
        for (index, &count) in rle.counts.iter().enumerate() {
            let expect_black = index % 2 == 1;
            for _ in 0..count {
                assert_eq!(
                    ba.get(position),
                    expect_black,
                    "index {index} (count {count}) at bit {position}"
                );
                position += 1;
            }
        }
        assert_eq!(position, rle.size);
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
        // 500 single-bit runs, plus the trailing white pad (the row ends black).
        assert_eq!(rle.counts.len(), size + 1);
        assert!(rle.counts[..size].iter().all(|&c| c == 1));
        assert_eq!(*rle.counts.last().unwrap(), 0);
        assert_eq!(rle.counts.iter().sum::<u32>() as usize, size);
    }

    // =========================================================================
    // fill_from (reusable buffer)
    //
    // A 1D scan encodes one row per scanned line. Allocating a fresh Vec each
    // time defeats the point, so readers hold one BitArrayRLE and refill it.
    // =========================================================================

    fn row_from_bits(bits: &[bool]) -> BitArray {
        let mut ba = BitArray::with_size(bits.len());
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                ba.set(i);
            }
        }
        ba
    }

    #[test]
    fn fill_from_produces_the_same_result_as_from() {
        let rows: Vec<Vec<bool>> = vec![
            vec![false, false, true, true, false],
            vec![true, true, false, true],
            vec![false],
            (0..91).map(|i| (i * 3 + 2) % 8 > 4).collect(),
        ];

        let mut reused = BitArrayRLE::new();
        for bits in rows {
            let ba = row_from_bits(&bits);

            reused.fill_from(&ba);

            assert_eq!(reused, BitArrayRLE::from(&ba), "mismatch for {bits:?}");
        }
    }

    #[test]
    fn fill_from_reuses_the_existing_allocation() {
        let busy = row_from_bits(&(0..200).map(|i| i % 2 == 0).collect::<Vec<_>>());
        let quiet = row_from_bits(&[false, false, true, false]);

        let mut rle = BitArrayRLE::new();
        rle.fill_from(&busy);
        let capacity_after_warmup = rle.counts.capacity();
        assert!(capacity_after_warmup >= 200);

        // Checked while the short row is loaded: an implementation that threw
        // the buffer away and started a fresh Vec would be holding a handful of
        // slots here. (Comparing capacity after re-loading the *long* row would
        // pass either way, since both paths grow back to the same power of two.)
        rle.fill_from(&quiet);
        assert_eq!(
            rle.counts.capacity(),
            capacity_after_warmup,
            "a shorter row must keep the buffer grown for the longer one"
        );

        rle.fill_from(&busy);
        assert_eq!(
            rle.counts.capacity(),
            capacity_after_warmup,
            "cycling between rows must not reallocate"
        );
    }

    #[test]
    fn fill_from_does_not_leave_stale_runs_behind() {
        let long = row_from_bits(&(0..40).map(|i| i % 2 == 0).collect::<Vec<_>>());
        let short = row_from_bits(&[false, true, false]);

        let mut rle = BitArrayRLE::new();
        rle.fill_from(&long);
        rle.fill_from(&short);

        assert_eq!(rle.counts, vec![1, 1, 1]);
        assert_eq!(rle.size, 3);
    }

    // =========================================================================
    // reverse
    // =========================================================================

    /// Reverse a BitArray by hand so the test has an independent oracle.
    fn reversed_bit_array(ba: &BitArray) -> BitArray {
        let size = ba.get_size();
        let mut out = BitArray::with_size(size);
        for i in 0..size {
            if ba.get(size - 1 - i) {
                out.set(i);
            }
        }
        out
    }

    #[test]
    fn reverse_matches_encoding_the_reversed_row() {
        // Every combination of opening/closing colour, so the zero-length pads
        // are exercised at both ends.
        let cases: Vec<Vec<bool>> = vec![
            vec![false, false, true, true, true, false],
            vec![true, true, true, false, false, false],
            vec![false, false, false, true, true, true],
            vec![true, true, false, false, true, true],
            vec![true],
            vec![false],
            (0..77).map(|i| (i * 5 + 1) % 7 > 3).collect(),
        ];

        for pattern in cases {
            let mut ba = BitArray::with_size(pattern.len());
            for (i, &bit) in pattern.iter().enumerate() {
                if bit {
                    ba.set(i);
                }
            }

            let reversed_row = reversed_bit_array(&ba);

            let mut rle = BitArrayRLE::from(&ba);
            rle.reverse();

            assert_eq!(
                rle,
                BitArrayRLE::from(&reversed_row),
                "reversing the RLE of {pattern:?} must equal the RLE of the reversed row"
            );
            // Compare against the reversed row itself, not just against another
            // encoding of it: an encoder that dropped the colour convention
            // would keep both sides of the equality above in step while still
            // putting black runs on even indices.
            assert_runs_match_row(&rle, &reversed_row);
        }
    }

    /// Assert that `rle` describes `row` with white runs on even indices and
    /// black runs on odd ones.
    fn assert_runs_match_row(rle: &BitArrayRLE, row: &BitArray) {
        let mut position = 0;
        for (index, &count) in rle.counts.iter().enumerate() {
            let expect_black = index % 2 == 1;
            for _ in 0..count {
                assert!(
                    position < row.get_size(),
                    "run {index} overruns the row at bit {position}"
                );
                assert_eq!(
                    row.get(position),
                    expect_black,
                    "run {index} (count {count}) should be {} at bit {position}",
                    if expect_black { "black" } else { "white" }
                );
                position += 1;
            }
        }
        assert_eq!(position, row.get_size(), "runs do not cover the whole row");
    }

    #[test]
    fn reverse_is_its_own_inverse() {
        let mut ba = BitArray::with_size(23);
        for i in [0, 1, 2, 7, 8, 15, 22] {
            ba.set(i);
        }

        let original = BitArrayRLE::from(&ba);
        let mut roundtrip = original.clone();
        roundtrip.reverse();
        roundtrip.reverse();

        assert_eq!(roundtrip, original);
    }

    #[test]
    fn reverse_preserves_the_white_first_convention() {
        // Opens black and closes black, so both pads must survive the flip.
        let mut ba = BitArray::with_size(7);
        for i in [0, 1, 5, 6] {
            ba.set(i);
        }

        let mut rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.counts, vec![0, 2, 3, 2, 0]);

        rle.reverse();

        assert_eq!(rle.counts, vec![0, 2, 3, 2, 0]);
        assert_eq!(rle.counts.len() % 2, 1);
    }

    // =========================================================================
    // Tests for RlePattern
    // =========================================================================

    #[test]
    fn test_rle_pattern_direct_construction() {
        let pattern = RlePattern { counts: [1, 2, 3] };
        assert_eq!(pattern.counts, [1, 2, 3]);

        let custom = RlePattern { counts: [2, 4] };
        assert_eq!(custom.counts, [2, 4]);
    }

    #[test]
    fn test_rle_pattern_from_array() {
        let pattern = RlePattern::from([1, 3, 5, 7]);
        assert_eq!(pattern.counts, [1, 3, 5, 7]);

        let empty = RlePattern::<0>::from([]);
        assert_eq!(empty.counts, []);

        let single = RlePattern::from([42]);
        assert_eq!(single.counts, [42]);
    }

    #[test]
    fn test_rle_pattern_into() {
        let pattern: RlePattern<3> = [2, 4, 6].into();
        assert_eq!(pattern.counts, [2, 4, 6]);
    }

    #[test]
    fn test_rle_pattern_clone_and_copy() {
        let p1 = RlePattern::from([1, 2, 3]);
        let p2 = p1; // Copy semantics
        assert_eq!(p1, p2); // p1 is still valid because of Copy

        let p3 = p1; // Copy again
        assert_eq!(p1, p3);
    }

    #[test]
    fn test_rle_pattern_eq_and_ne() {
        let p1 = RlePattern::from([1, 2, 3]);
        let p2 = RlePattern::from([1, 2, 3]);
        let p3 = RlePattern::from([1, 2, 4]);

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }

    #[test]
    fn test_rle_pattern_debug() {
        let pattern = RlePattern::from([1, 2, 3]);
        let debug_str = format!("{pattern:?}");
        assert!(debug_str.contains("RlePattern"));
        assert!(debug_str.contains("counts"));
        assert!(debug_str.contains("[1, 2, 3]"));
    }

    // =========================================================================
    // calculate_variance
    //
    // This must agree with `oned::one_d_reader::pattern_match_variance`, which
    // is what every 1D reader uses today. That contract is:
    //
    //   * the pattern is in module units and the observed runs are in pixels,
    //     so the pattern is scaled by (observed total / pattern total) before
    //     comparison -- matching must be scale-invariant;
    //   * the score is the mean absolute deviation, normalised by the observed
    //     total, so it is comparable across candidates;
    //   * any single element deviating by more than `max_individual_variance`
    //     (itself scaled by the unit bar width) rejects the whole match;
    //   * fewer observed pixels than the pattern has modules is too small to
    //     judge, and is rejected;
    //   * lower is better, and rejection is `f32::INFINITY`.
    // =========================================================================

    const MIV: f32 = 0.7;

    #[test]
    fn variance_is_scale_invariant() {
        // Code 128's start-code B pattern, observed at 11 pixels per module.
        let pattern = RlePattern::from([2, 1, 2, 2, 2, 2]);
        let array = BitArrayRLE {
            counts: vec![22, 11, 22, 22, 22, 22],
            size: 121,
        };

        assert_eq!(pattern.calculate_variance(&array, 0, MIV), 0.0);
    }

    #[test]
    fn variance_is_the_same_at_every_scale() {
        let pattern = RlePattern::from([2, 1, 2, 2, 2, 2]);
        let scales = [1u32, 3, 7, 40];

        let scores: Vec<f32> = scales
            .iter()
            .map(|&k| {
                let array = BitArrayRLE {
                    counts: pattern.counts.iter().map(|&c| c * k).collect(),
                    size: (pattern.counts.iter().sum::<u32>() * k) as usize,
                };
                pattern.calculate_variance(&array, 0, MIV)
            })
            .collect();

        assert!(
            scores.iter().all(|&s| s == 0.0),
            "an exact match must score 0 at every scale, got {scores:?}"
        );
    }

    #[test]
    fn variance_rejects_a_window_that_runs_off_the_end() {
        // Four runs available from `start`, but the pattern needs six. A partial
        // comparison would score better than a full one, so it must be rejected.
        let pattern = RlePattern::from([2, 1, 2, 2, 2, 2]);
        let array = BitArrayRLE {
            counts: vec![9, 2, 1, 2, 2],
            size: 16,
        };

        assert_eq!(
            pattern.calculate_variance(&array, 1, MIV),
            f32::INFINITY,
            "a truncated window must be rejected, not scored on what fits"
        );
    }

    #[test]
    fn variance_rejects_a_start_at_or_past_the_end() {
        let pattern = RlePattern::from([1, 2]);
        let array = BitArrayRLE {
            counts: vec![1, 2, 3],
            size: 6,
        };

        assert_eq!(pattern.calculate_variance(&array, 3, MIV), f32::INFINITY);
        assert_eq!(pattern.calculate_variance(&array, 4, MIV), f32::INFINITY);
        assert_eq!(pattern.calculate_variance(&array, 100, MIV), f32::INFINITY);
    }

    #[test]
    fn variance_rejects_an_empty_array() {
        let pattern = RlePattern::from([1, 2, 3]);
        let empty = BitArrayRLE::new();

        assert_eq!(pattern.calculate_variance(&empty, 0, MIV), f32::INFINITY);
        assert_eq!(pattern.calculate_variance(&empty, 5, MIV), f32::INFINITY);
    }

    #[test]
    fn variance_rejects_a_single_element_deviating_too_far() {
        // Five of six runs match exactly; the second is wildly wrong. The mean
        // deviation alone would still look tolerable, so the per-element limit
        // is what has to reject this.
        let pattern = RlePattern::from([2, 1, 2, 2, 2, 2]);
        let array = BitArrayRLE {
            counts: vec![20, 90, 20, 20, 20, 20],
            size: 190,
        };

        assert_eq!(pattern.calculate_variance(&array, 0, MIV), f32::INFINITY);
    }

    #[test]
    fn variance_rejects_a_window_too_small_to_judge() {
        // Fewer observed pixels than the pattern has modules: under one pixel
        // per module there is nothing to measure.
        //
        // The observation is exactly proportional to the pattern, so every
        // per-element deviation is zero and the size check is the only thing
        // that can reject it. (An observation that is *not* proportional gets
        // caught by `max_individual_variance` instead, which would let this
        // test pass even with the size check removed.)
        let pattern = RlePattern::from([2, 2, 2, 2, 2, 2]);
        let array = BitArrayRLE {
            counts: vec![1, 1, 1, 1, 1, 1],
            size: 6,
        };

        assert_eq!(pattern.calculate_variance(&array, 0, MIV), f32::INFINITY);
    }

    #[test]
    fn variance_rejects_a_degenerate_zero_width_pattern() {
        let pattern = RlePattern::from([0, 0, 0]);
        let array = BitArrayRLE {
            counts: vec![4, 4, 4],
            size: 12,
        };

        assert_eq!(pattern.calculate_variance(&array, 0, MIV), f32::INFINITY);
    }

    #[test]
    fn a_closer_match_scores_lower() {
        let pattern = RlePattern::from([2, 1, 2, 2, 2, 2]);
        let exact = BitArrayRLE {
            counts: vec![20, 10, 20, 20, 20, 20],
            size: 110,
        };
        let slightly_off = BitArrayRLE {
            counts: vec![21, 10, 19, 20, 20, 20],
            size: 110,
        };

        let near = pattern.calculate_variance(&exact, 0, MIV);
        let far = pattern.calculate_variance(&slightly_off, 0, MIV);

        assert!(near < far, "exact {near} should score below inexact {far}");
        assert!(
            far.is_finite(),
            "a small deviation should still be accepted"
        );
    }

    #[test]
    fn a_single_run_always_matches_after_scaling() {
        // One run carries no shape information: whatever its width, scaling the
        // pattern to it makes the fit exact. Documented so nobody reads the
        // zero as a bug.
        let pattern = RlePattern::from([7]);

        for observed in [7u32, 10, 400] {
            let array = BitArrayRLE {
                counts: vec![observed],
                size: observed as usize,
            };
            assert_eq!(pattern.calculate_variance(&array, 0, MIV), 0.0);
        }
    }

    #[test]
    fn scanning_a_row_finds_the_finder_pattern() {
        // A 1:1:3:1:1 finder in a quiet zone: 00000 1 0 111 0 1 000000
        let finder = RlePattern::from([1, 1, 3, 1, 1]);

        let mut ba = BitArray::with_size(18);
        for i in [5, 7, 8, 9, 11] {
            ba.set(i);
        }

        let rle = BitArrayRLE::from(&ba);
        assert_eq!(rle.counts, vec![5, 1, 1, 3, 1, 1, 6]);

        // The caller no longer needs to bound `start` itself; windows that run
        // off the end reject themselves.
        let scores: Vec<f32> = (0..rle.counts.len())
            .map(|start| finder.calculate_variance(&rle, start, MIV))
            .collect();

        let best = scores
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i);

        assert_eq!(best, Some(1), "scores were {scores:?}");
        assert_eq!(scores[1], 0.0);
        assert!(
            scores.iter().enumerate().all(|(i, &v)| i == 1 || v > 0.0),
            "only the true finder position should score zero: {scores:?}"
        );
        assert!(
            scores[3..].iter().all(|v| v.is_infinite()),
            "windows running off the end must be rejected: {scores:?}"
        );
    }

    #[cfg(feature = "oned")]
    #[test]
    fn variance_agrees_with_pattern_match_variance() {
        use crate::oned::pattern_match_variance;

        let patterns: [[u32; 6]; 5] = [
            [2, 1, 2, 2, 2, 2],
            [2, 2, 2, 1, 2, 2],
            [1, 2, 1, 2, 2, 3],
            [3, 1, 2, 1, 3, 1],
            // Proportional to the all-ones observation below, so it exercises
            // the "too small to judge" rejection on its own.
            [2, 2, 2, 2, 2, 2],
        ];
        let observations: [[u32; 6]; 7] = [
            [20, 10, 20, 20, 20, 20],
            [22, 11, 22, 22, 22, 22],
            [21, 9, 20, 20, 21, 19],
            [2, 1, 2, 2, 2, 2],
            [30, 5, 20, 20, 20, 20],
            [1, 1, 1, 1, 1, 1],
            [7, 3, 8, 6, 7, 7],
        ];

        for pattern in patterns {
            for observed in observations {
                let expected = pattern_match_variance(&observed, &pattern, MIV);

                let rle = BitArrayRLE {
                    counts: observed.to_vec(),
                    size: observed.iter().sum::<u32>() as usize,
                };
                let actual = RlePattern::from(pattern).calculate_variance(&rle, 0, MIV);

                assert_eq!(
                    actual.is_finite(),
                    expected.is_finite(),
                    "accept/reject disagreement for pattern {pattern:?} vs {observed:?}"
                );
                if expected.is_finite() {
                    assert!(
                        (actual - expected).abs() < 1e-6,
                        "score disagreement for pattern {pattern:?} vs {observed:?}: \
                         rle={actual} pattern_match_variance={expected}"
                    );
                }
            }
        }
    }

    #[cfg(feature = "oned")]
    #[test]
    fn variance_agrees_with_pattern_match_variance_at_an_offset() {
        use crate::oned::pattern_match_variance;

        let pattern: [u32; 4] = [1, 1, 3, 1];
        let row = BitArrayRLE {
            counts: vec![14, 3, 9, 2, 6, 6, 18, 6, 4],
            size: 68,
        };

        for start in 0..=(row.counts.len() - pattern.len()) {
            let window: [u32; 4] = row.counts[start..start + 4].try_into().unwrap();
            let expected = pattern_match_variance(&window, &pattern, MIV);

            let actual = RlePattern::from(pattern).calculate_variance(&row, start, MIV);

            assert_eq!(
                actual.is_finite(),
                expected.is_finite(),
                "accept/reject disagreement at start {start}"
            );
            if expected.is_finite() {
                assert!(
                    (actual - expected).abs() < 1e-6,
                    "score disagreement at start {start}: rle={actual} expected={expected}"
                );
            }
        }
    }
}
