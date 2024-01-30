/*
 * Copyright 2023 Antoine Mérino
 * Copyright 2023 Axel Waggershauser
 */
// SPDX-License-Identifier: Apache-2.0

use crate::{
    common::{
        cpp_essentials::{FindLeftGuardBy, FixedPattern, IsRightGuard, PatternView, ToIntPos},
        BitArray,
    },
    point, BarcodeFormat, DecodeHintValue, DecodingHintDictionary, Exceptions, PointI, RXingResult,
};

use super::row_reader::{DecodingState, RowReader};

use crate::common::Result;

// Detection is made from center outward.
// We ensure the clock track is decoded before the data track to avoid false positives.
// They are two version of a DX Edge codes : with and without frame number.
// The clock track is longer if the DX code contains the frame number (more recent version)
const CLOCK_LENGTH_FN: usize = 31;
const CLOCK_LENGTH_NO_FN: usize = 23;

// data track length, without the start and stop patterns
const DATA_LENGTH_FN: u32 = 23;
const DATA_LENGTH_NO_FN: u32 = 15;

const CLOCK_PATTERN_FN: FixedPattern<25, CLOCK_LENGTH_FN> = FixedPattern::new([
    5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3,
]);
const CLOCK_PATTERN_NO_FN: FixedPattern<17, CLOCK_LENGTH_NO_FN> =
    FixedPattern::new([5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3]);
const DATA_START_PATTERN: FixedPattern<5, 5> = FixedPattern::new([1, 1, 1, 1, 1]);
const DATA_STOP_PATTERN: FixedPattern<3, 3> = FixedPattern::new([1, 1, 1]);

pub struct DXFilmEdgeReader<'a> {
    options: &'a DecodingHintDictionary,
}

impl<'a> DXFilmEdgeReader<'_> {
    pub fn new(hints: &'a DecodingHintDictionary) -> DXFilmEdgeReader<'a> {
        DXFilmEdgeReader { options: hints }
    }
}

fn IsPattern<const N: usize, const SUM: usize>(
    view: &PatternView,
    pattern: &FixedPattern<N, SUM>,
    minQuietZone: f32,
) -> bool {
    const E2E: bool = false;
    let view = view.subView(0, Some(N));
    view.isValid()
        && crate::common::cpp_essentials::pattern::IsPattern::<E2E, N, SUM, false>(
            &view,
            pattern,
            Some(if view.isAtFirstBar() {
                u32::MAX as f32
            } else {
                view[-1] as f32
            }),
            minQuietZone,
            0.0,
        ) != 0.0
}

fn DistIsBelowThreshold(a: PointI, b: PointI, threshold: PointI) -> bool {
    (a.x - b.x).abs() < threshold.x && (a.y - b.y).abs() < threshold.y
}

// DX Film Edge clock track found on 35mm films.
#[derive(Debug, Clone)]
pub(super) struct Clock {
    hasFrameNr: bool, // = false; // Clock track (thus data track) with frame number (longer version)
    rowNumber: u32,   // = 0,
    xStart: u32,      // = 0; // Beginning of the clock track on the X-axis, in pixels
    xStop: u32,       // = 0; // End of the clock track on the X-axis, in pixels
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            hasFrameNr: false,
            rowNumber: 0,
            xStart: 0,
            xStop: 0,
        }
    }
}

impl Clock {
    pub const fn dataLength(&self) -> u32 {
        if self.hasFrameNr {
            DATA_LENGTH_FN
        } else {
            DATA_LENGTH_NO_FN
        }
    }

    pub fn moduleSize(&self) -> f32 {
        (self.xStop as f32 - self.xStart as f32)
            / (if self.hasFrameNr {
                CLOCK_LENGTH_FN
            } else {
                CLOCK_LENGTH_NO_FN
            }) as f32
    }

    pub fn isCloseTo(&self, p: PointI, x: u32) -> bool {
        return DistIsBelowThreshold(
            p,
            point(x as i32, self.rowNumber as i32),
            (self.moduleSize() * point(0.5, 4.0)).into(),
        );
    }

    pub fn isCloseToStart(&self, x: u32, y: u32) -> bool {
        return self.isCloseTo(point(x as i32, y as i32), self.xStart);
    }
    pub fn isCloseToStop(&self, x: u32, y: u32) -> bool {
        return self.isCloseTo(point(x as i32, y as i32), self.xStop);
    }
}

impl DecodingState {
    // see if we a clock that starts near {x, y}
    pub fn findClock(&mut self, x: u32, y: u32) -> Option<&mut Clock> {
        let start = point(x, y);
        if let Some(i) = self
            .clocks
            .iter()
            .position(|c| c.isCloseToStart(start.x, start.y))
        {
            self.clocks.get_mut(i) //self.clocks[i]
        } else {
            None
        }
        // let i = FindIf(clocks, [start = PointI{x, y}](auto& v) { return v.isCloseToStart(start.x, start.y); });
        // return if i != clocks.end()  {&(*i)} else {nullptr};
    }

    // add/update clock
    pub fn addClock(&mut self, clock: Clock) {
        if let Some(clockf) = self.findClock(clock.xStart, clock.rowNumber) {
            *clockf = clock
        } else {
            self.clocks.push(clock)
        }
        // if (Clock* i = findClock(clock.xStart, clock.rowNumber))
        // 	{*i = clock;}
        // else
        // 	{clocks.push_back(clock);}
    }
}

fn CheckForClock(rowNumber: u32, view: &PatternView) -> Option<Clock> {
    let mut clock = Clock::default();

    if (IsPattern(view, &CLOCK_PATTERN_FN, 0.5))
    // On FN versions, the decimal number can be really close to the clock
    {
        clock.hasFrameNr = true;
    } else if (IsPattern(view, &CLOCK_PATTERN_NO_FN, 2.0)) {
        clock.hasFrameNr = false;
    } else {
        return None;
    }

    clock.rowNumber = rowNumber;
    clock.xStart = view.pixelsInFront() as u32;
    clock.xStop = view.pixelsTillEnd() as u32;

    return Some(clock);
}

impl<'a> RowReader for DXFilmEdgeReader<'_> {
    fn decodePattern(
        &self,
        rowNumber: u32,
        next: &mut PatternView,
        state: &mut Option<DecodingState>,
    ) -> Result<RXingResult> {
        // if (!state) {
        //     state.reset(new DXFEState);
        //     static_cast<DXFEState*>(state.get())->centerRow = rowNumber;
        // }

        if state.is_none() {
            *state = Some(DecodingState::default())
        };

        let dxState = state.as_mut().unwrap();

        // Only consider rows below the center row of the image

        if (!matches!(
            self.options.get(&crate::DecodeHintType::TRY_HARDER),
            Some(DecodeHintValue::TryHarder(true))
        ) && rowNumber < dxState.centerRow)
        {
            return Err(Exceptions::NOT_FOUND);
        }

        // Look for a pattern that is part of both the clock as well as the data track (ommitting the first bar)
        let Is4x1 = |view: &PatternView, spaceInPixel: Option<f32>| {
            let spaceInPixel = spaceInPixel.unwrap_or_default();
            // find min/max of 4 consecutive bars/spaces and make sure they are close together
            let tmp_arr: [u16; 4] = [view[1], view[2], view[3], view[4]];
            let m = *tmp_arr.iter().min().unwrap_or(&0);
            let M = *tmp_arr.iter().max().unwrap_or(&0);
            // let [m, M] = std::minmax({view[1], view[2], view[3], view[4]});
            return M <= m * 4 / 3 + 1 && spaceInPixel > m as f32 / 2.0;
        };

        // 12 is the minimum size of the data track (at least one product class bit + one parity bit)
        *next = FindLeftGuardBy::<12, _>(*next, 10, Is4x1)?; // THIS IS WRONG WRONG WRONG ISSUE
                                                             // next = FindLeftGuard<4>(next, 10, Is4x1);
        if (!next.isValid()) {
            return Err(Exceptions::NOT_FOUND);
        }

        // Check if the 4x1 pattern is part of a clock track
        if let Some(clock) = CheckForClock(rowNumber, &next) {
            dxState.addClock(clock);
            next.skipSymbol();
            return Err(Exceptions::NOT_FOUND);
        }
        // if (auto clock = CheckForClock(rowNumber, next)) {
        //     dxState->addClock(*clock);
        //     next.skipSymbol();
        //     return {};
        // }

        // Without at least one clock track, we stop here
        if (dxState.clocks.is_empty()) {
            return Err(Exceptions::NOT_FOUND);
        }

        let minDataQuietZone: f32 = 0.5;

        if (!IsPattern(&next, &DATA_START_PATTERN, minDataQuietZone)) {
            return Err(Exceptions::NOT_FOUND);
        }

        let xStart = next.pixelsInFront();

        // Only consider data tracks that are next to a clock track
        let Some(clock) = dxState.findClock(xStart as u32, rowNumber) else {
            return Err(Exceptions::NOT_FOUND);
        };

        // Skip the data start pattern (black, white, black, white, black)
        // The first signal bar is always white: this is the
        // separation between the start pattern and the product number
        next.skipSymbol();

        // Read the data bits
        let mut dataBits = BitArray::default();
        while (next.isValidWithN(1) && dataBits.get_size() < clock.dataLength() as usize) {
            let modules = (next[0] as f32 / clock.moduleSize() + 0.5) as u32;
            // even index means we are at a bar, otherwise at a space
            // dataBits.appendBits(if next.index() % 2 == 0  {0xFFFFFFFF} else {0x0}, modules);
            for _i in 0..modules {
                dataBits.appendBits(
                    if next.index() % 2 == 0 {
                        0xFFFFFFFF
                    } else {
                        0x0
                    },
                    modules as usize,
                )?;
                // should it be 0xFFFFFFFF
            }

            next.shift(1);
        }

        // Check the data track length
        if (dataBits.get_size() != clock.dataLength() as usize) {
            return Err(Exceptions::NOT_FOUND);
        }

        *next = next.subView(0, Some(DATA_STOP_PATTERN.size()));

        // Check there is the Stop pattern at the end of the data track
        if (!next.isValid() || !IsRightGuard(&next, &DATA_STOP_PATTERN, minDataQuietZone, 0.0)) {
            return Err(Exceptions::NOT_FOUND);
        }

        // The following bits are always white (=false), they are separators.
        if (dataBits.get(0) != false //0
            || dataBits.get(8) != false //0
            || (if clock.hasFrameNr {
                (dataBits.get(20) != false/*0*/ || dataBits.get(22) != false/*0*/)
            } else {
                dataBits.get(14) != false//0
            }))
        {
            return Err(Exceptions::NOT_FOUND);
        }

        // Check the parity bit
        let db_hld = Into::<Vec<bool>>::into(&dataBits); //.iter().rev().skip(2).fold(0, |acc, e| acc + u8::from(*e));
        let signalSum = db_hld
            .iter()
            .rev()
            .skip(2)
            .fold(0, |acc, e| acc + u8::from(*e)); //dataBits.iter().rev().skip(2).sum::<u8>(); //Reduce(dataBits.begin(), dataBits.end() - 2, 0);
        let parityBit = u8::from(*(db_hld.last().unwrap_or(&false)));
        if (signalSum % 2 != parityBit) {
            return Err(Exceptions::NOT_FOUND);
        }

        // Compute the DX 1 number (product number)
        let Some(productNumber) = ToIntPos(&Into::<Vec<u8>>::into(&dataBits), 1, 7) else {
            return Err(Exceptions::NOT_FOUND);
        };

        // Compute the DX 2 number (generation number)
        let Some(generationNumber) = ToIntPos(&Into::<Vec<u8>>::into(&dataBits), 9, 4) else {
            return Err(Exceptions::NOT_FOUND);
        };

        // Generate the textual representation.
        // Eg: 115-10/11A means: DX1 = 115, DX2 = 10, Frame number = 11A
        let mut txt = String::with_capacity(10);
        // txt.reserve(10);
        txt = (productNumber.to_string()) + "-" + (&generationNumber.to_string());
        if (clock.hasFrameNr) {
            let frameNr = ToIntPos(&Into::<Vec<u8>>::into(&dataBits), 13, 6).unwrap_or(0);
            txt += &("/".to_owned() + &(frameNr.to_string()));
            if (dataBits.get(19) != false/*0*/) {
                txt += "A";
            }
        }

        let xStop = next.pixelsTillEnd();

        // The found data track must end near the clock track
        if (!clock.isCloseToStop(xStop as u32, rowNumber)) {
            return Err(Exceptions::NOT_FOUND);
        }

        // Update the clock coordinates with the latest corresponding data track
        // This may improve signal detection for next row iterations
        clock.xStart = xStart as u32;
        clock.xStop = xStop as u32;

        Ok(RXingResult::new(
            &txt,
            dataBits.into(),
            Vec::new(),
            BarcodeFormat::DXFilmEdge,
        ))
        // return RXingResult(txt, rowNumber, xStart, xStop, BarcodeFormat::DXFilmEdge, {});
    }
}
