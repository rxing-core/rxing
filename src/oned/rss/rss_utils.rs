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

/** Adapted from listings in ISO/IEC 24724 Appendix B and Appendix G. */

pub fn getRSSvalue(widths: &[u32], maxWidth: u32, noNarrow: bool) -> u32 {
    let mut n = widths.iter().sum::<u32>();

    let mut val = 0;
    let mut narrowMask = 0;
    let elements = widths.len() as u32;
    for bar in 0..(elements - 1) {
        let mut elmWidth = 1;
        narrowMask |= 1 << bar;
        while elmWidth < widths[bar as usize] {
            let mut subVal = combins(n - elmWidth - 1, elements - bar - 2);
            if noNarrow
                && (narrowMask == 0)
                && (n - elmWidth - (elements - bar - 1) >= elements - bar - 1)
            {
                subVal -= combins(n - elmWidth - (elements - bar), elements - bar - 2);
            }
            if elements - bar - 1 > 1 {
                let mut lessVal = 0;
                let mut mxwElement = n - elmWidth - (elements - bar - 2);
                while mxwElement > maxWidth {
                    lessVal += combins(n - elmWidth - mxwElement - 1, elements - bar - 3);

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

fn combins(n: u32, r: u32) -> u32 {
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
