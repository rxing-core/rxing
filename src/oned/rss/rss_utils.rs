/*
 * Copyright 2009 ZXing authors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

/* Adapted from listings in ISO/IEC 24724 Appendix B and Appendix G. */
#[inline]
pub fn getRSSvalue<const S: usize>(widths: &[u32; S], maxWidth: u32, noNarrow: bool) -> u32 {
    let elements = S as u32;
    let mut n = widths.iter().sum::<u32>();

    let mut val = 0;
    let mut narrowMask = 0;
    for bar in 0..(elements - 1) {
        let mut elmWidth = 1;
        narrowMask |= 1 << bar;
        while elmWidth < widths[bar as usize] {
            let mut subVal = combins_pre(n - elmWidth - 1, elements - bar - 2);
            if noNarrow
                && (narrowMask == 0)
                && (n - elmWidth - (elements - bar - 1) >= elements - bar - 1)
            {
                subVal -= combins_pre(n - elmWidth - (elements - bar), elements - bar - 2);
            }
            if elements - bar - 1 > 1 {
                let mut lessVal = 0;
                let mut mxwElement = n - elmWidth - (elements - bar - 2);
                while mxwElement > maxWidth {
                    lessVal += combins_pre(n - elmWidth - mxwElement - 1, elements - bar - 3);

                    mxwElement -= 1;
                }
                subVal -= lessVal * (elements - 1 - bar);
            } else if n - elmWidth > maxWidth {
                subVal -= 1;
            }
            val += subVal;

            elmWidth += 1;
            narrowMask &= !(1 << bar)
        }
        n -= elmWidth;
    }
    val
}

#[inline(always)]
const fn combins(n: u32, r: u32) -> u32 {
    if n as usize <= N_MAX && r as usize <= R_MAX {
        return combins_pre(n, r);
    }

    let maxDenom;
    let minDenom;

    // if n - r > r {
    if n.checked_sub(r).is_none() {
        minDenom = r;
        maxDenom = n - r;
    } else {
        minDenom = n - r;
        maxDenom = r;
    }
    let mut val = 1;
    let mut j = 1;
    let mut i = n;
    while i > maxDenom {
        val *= i;
        if j <= minDenom {
            val /= j;
            j += 1;
        }

        i -= 1;
    }
    while j <= minDenom {
        val /= j;
        j += 1;
    }

    val
}

// Maximum n such that all binomial(n, r) fit in u32 without overflow
const N_MAX: usize = 34;
const R_MAX: usize = N_MAX;

// 2. Build the table at compile time
const fn build_pascal() -> [[u32; R_MAX + 1]; N_MAX + 1] {
    // table[i][j] == i choose j
    let mut table = [[0u32; R_MAX + 1]; N_MAX + 1];
    let mut i = 0;
    while i <= N_MAX {
        let mut j = 0;
        while j <= i {
            table[i][j] = if j == 0 || j == i {
                1
            } else {
                // by the time we index here, table[i-1][j-1] and table[i-1][j] are already set
                table[i - 1][j - 1] + table[i - 1][j]
            };
            j += 1;
        }
        i += 1;
    }
    table
}

// Precomputed binomial table for 0 <= n <= 34, 0 <= r <= n
const COMBIN_TABLE: [[u32; R_MAX + 1]; N_MAX + 1] = build_pascal();

// 3. New `combins` just indexes the table
#[inline(always)]
pub const fn combins_pre(n: u32, r: u32) -> u32 {
    let ni = n as usize;
    let ri = r as usize;

    if ni > N_MAX || ri > R_MAX {
        return combins(n, r);
    }

    COMBIN_TABLE[ni][ri]
}
