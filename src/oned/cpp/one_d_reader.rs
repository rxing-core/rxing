/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use std::any::Any;
use std::collections::HashMap;

use crate::common::cpp_essentials::{PatternRow, PatternView};
use crate::Binarizer;
use crate::{multi::MultipleBarcodeReader, RXingResult, Reader};
use crate::{
    point, BarcodeFormat, BinaryBitmap, DecodingHintDictionary, Exceptions, PointT, ResultPoint,
};

use crate::common::Result;

use super::dxfilm_edge_reader::DXFilmEdgeReader;
use super::row_reader::RowReader;

pub struct ODReader<'a> {
    reader: DXFilmEdgeReader<'a>, // THIS IS WRONG, SEE BELOW ONLY DOES ONE
    // readers: Vec<dyn RowReader>,
    try_harder: bool,
    is_pure: bool,
    min_line_count: u32,
    return_errors: bool,
    try_rotate: bool,
}

impl<'a> ODReader<'_> {
    /**
    * We're going to examine rows from the middle outward, searching alternately above and below the
    * middle, and farther out each time. rowStep is the number of rows between each successive
    * attempt above and below the middle. So we'd scan row middle, then middle - rowStep, then
    * middle + rowStep, then middle - (2 * rowStep), etc.
    * rowStep is bigger as the image is taller, but is always at least 1. We've somewhat arbitrarily
    * decided that moving up and down by about 1/16 of the image is pretty good; we try more of the
    * image if "trying harder".
    */
    pub fn DoDecode<B: Binarizer>(
        reader: &DXFilmEdgeReader,
        image: &BinaryBitmap<B>,
        tryHarder: bool,
        rotate: bool,
        isPure: bool,
        maxSymbols: u32,
        minLineCount: u32,
        returnErrors: bool,
    ) -> Vec<RXingResult> {
        let res: Vec<RXingResult> = Vec::new();

        let decodingState = Vec::new();
        // std::vector<std::unique_ptr<RowReader::DecodingState>> decodingState(readers.size());

        let width: i32 = image.get_width() as i32;
        let height: i32 = image.get_height() as i32;

        if (rotate) {
            std::mem::swap(&mut width, &mut height);
        }

        let middle: i32 = height / 2;
        // TODO: find a better heuristic/parameterization if maxSymbols != 1
        let rowStep: i32 = std::cmp::max(
            1,
            height
                / (if (tryHarder && !isPure) {
                    (if maxSymbols == 1 { 256 } else { 512 })
                } else {
                    32
                }),
        );
        let maxLines: i32 = if tryHarder 
{height} else	// Look at the whole image, not just the center
{15}; // 15 rows spaced 1/32 apart is roughly the middle half of the image

        if (isPure) {
            minLineCount = 1;
        }
        let checkRows = Vec::new();

        let bars: PatternRow = PatternRow::new(vec![0; 128]); // e.g. EAN-13 has 59 bars/spaces
                                                              // bars.reserve(128); // e.g. EAN-13 has 59 bars/spaces

        // #ifdef PRINT_DEBUG
        // BitMatrix dbg(width, height);
        // #endif

        'outer: for i in 0..maxLines {
            // for (int i = 0; i < maxLines; i++) {

            // Scanning from the middle out. Determine which row we're looking at next:
            let rowStepsAboveOrBelow: i32 = (i + 1) / 2;
            let isAbove: bool = (i & 0x01) == 0; // i.e. is x even?
            let rowNumber: i32 = middle
                + rowStep
                    * (if isAbove {
                        rowStepsAboveOrBelow
                    } else {
                        -rowStepsAboveOrBelow
                    });
            let isCheckRow: bool = false;
            if (rowNumber < 0 || rowNumber >= height) {
                // Oops, if we run off the top or bottom, stop
                break;
            }

            // See if we have additional check rows (see below) to process
            if (!checkRows.is_empty()) {
                --i;
                rowNumber = checkRows.back();
                checkRows.pop_back();
                isCheckRow = true;
                if (rowNumber < 0 || rowNumber >= height) {
                    continue;
                }
            }

            if (!image.getPatternRow(rowNumber, if rotate { 90 } else { 0 }, bars)) {
                continue;
            }

            // #ifdef PRINT_DEBUG
            // bool val = false;
            // int x = 0;
            // for (auto b : bars) {
            // for(int j = 0; j < b; ++j)
            // dbg.set(x++, rowNumber, val);
            // val = !val;
            // }
            // #endif

            // While we have the image data in a PatternRow, it's fairly cheap to reverse it in place to
            // handle decoding upside down barcodes.
            // TODO: the DataBarExpanded (stacked) decoder depends on seeing each line from both directions. This
            // 'surprising' and inconsistent. It also requires the decoderState to be shared between normal and reversed
            // scans, which makes no sense in general because it would mix partial detection data from two codes of the same
            // type next to each other. See also https://github.com/zxing-cpp/zxing-cpp/issues/87
            for upsideDown in [false, true] {
                // for (bool upsideDown : {false, true}) {
                // trying again?
                if (upsideDown) {
                    // reverse the row and continue
                    // std::reverse(bars.begin(), bars.end());
                    bars.reverse();
                }
                let readers = vec![reader];
                // Look for a barcode
                for r in 0..readers.len() {
                    // for (size_t r = 0; r < readers.size(); ++r) {
                    // If this is a pure symbol, then checking a single non-empty line is sufficient for all but the stacked
                    // DataBar codes. They are the only ones using the decodingState, which we can use as a flag here.
                    if (isPure && i && !decodingState[r]) {
                        continue;
                    }

                    let next = PatternView::from(bars);
                    loop {
                        let result = readers[r]
                            .decodePattern(rowNumber, &mut next, decodingState[r])
                            .ok();
                        if (result.isValid() || (returnErrors && result.error())) {
                            IncrementLineCount(&result);
                            if (upsideDown) {
                                // update position (flip horizontally).
                                let points = result.position();
                                for p in points {
                                    // for (auto& p : points) {
                                    p = point(width - p.getX() - 1, p.getY());
                                }
                                result.addPoints(points);
                                // result.setPosition(std::move(points));
                            }
                            if (rotate) {
                                let points = result.position();
                                for p in points {
                                    // for (auto& p : points) {
                                    p = point(p.getY(), width - p.getX() - 1);
                                }
                                result.addPoints(points);
                                // result.setPosition(std::move(points));
                            }

                            // check if we know this code already
                            for other in res {
                                // for (auto& other : res) {
                                if (result == other) {
                                    // merge the position information
                                    let dTop = PointT::maxAbsComponent(
                                        other.position().topLeft() - result.position().topLeft(),
                                    );
                                    let dBot = PointT::maxAbsComponent(
                                        other.position().bottomLeft() - result.position().topLeft(),
                                    );
                                    let points = other.position();
                                    if (dTop < dBot
                                        || (dTop == dBot
                                            && rotate
                                                ^ (PointT::sumAbsComponent(points[0])
                                                    > PointT::sumAbsComponent(
                                                        result.position()[0],
                                                    ))))
                                    {
                                        points[0] = result.position()[0];
                                        points[1] = result.position()[1];
                                    } else {
                                        points[2] = result.position()[2];
                                        points[3] = result.position()[3];
                                    }
                                    other.setPosition(points);
                                    IncrementLineCount(&other);
                                    // clear the result, so we don't insert it again below
                                    result = None; //Result();
                                    break;
                                }
                            }

                            if (result.format() != BarcodeFormat::UNSUPORTED_FORMAT) {
                                res.push(result);
                                // res.push_back(std::move(result));

                                // if we found a valid code we have not seen before but a minLineCount > 1,
                                // add additional check rows above and below the current one
                                if (!isCheckRow && minLineCount > 1 && rowStep > 1) {
                                    checkRows = vec![rowNumber - 1, rowNumber + 1];
                                    if (rowStep > 2) {
                                        checkRows.push(rowNumber - 2);
                                        checkRows.push(rowNumber + 2);
                                        // checkRows.insert(checkRows.end(), {rowNumber - 2, rowNumber + 2});
                                    }
                                }
                            }

                            if (maxSymbols
                                && res.iter().fold(0, |acc, e| {
                                    acc + i32::from((r.lineCount() >= minLineCount))
                                }) == maxSymbols)
                            {
                                break 'outer;
                            }
                        }
                        // make sure we make progress and we start the next try on a bar
                        next.shift(2 - (next.index() % 2));
                        next.extend();
                        if !(tryHarder && next.size()) {
                            break;
                        }
                    } //while (tryHarder && next.size());
                }
            }
        }

        // out:
        // remove all symbols with insufficient line count
        let it = res.iter().filter(|e| e.lineCount() < minLineCount);
        // let it = std::remove_if(res.begin(), res.end(), [&](auto&& r) { return r.lineCount() < minLineCount; });
        res.erase(it, res.end());

        // if symbols overlap, remove the one with a lower line count
        for (i, a) in res.iter().enumerate() {
            // for (auto a = res.begin(); a != res.end(); ++a){
            for b in res.iter().skip(i) {
                // for (auto b = std::next(a); b != res.end(); ++b){
                if (PointT::HaveIntersectingBoundingBoxes(a.position(), b.position())) {
                    *(if a.lineCount() < b.lineCount() { a } else { b }) = None;
                }
            }
        }

        //TODO: C++20 res.erase_if()
        it = res
            .iter()
            .filter(|r| r.getBarcodeFormat() == BarcodeFormat::None);
        // it = std::remove_if(res.begin(), res.end(), [](auto&& r) { return r.format() == BarcodeFormat::None; });
        res.erase(it, res.end());

        // #ifdef PRINT_DEBUG
        // SaveAsPBM(dbg, rotate ? "od-log-r.pnm" : "od-log.pnm");
        // #endif

        res
    }
}

impl<'a> ODReader<'_> {
    pub fn decode_single<B: crate::Binarizer>(
        &self,
        hints: &DecodingHintDictionary,
        image: &BinaryBitmap<B>,
    ) -> Result<RXingResult> {
        let result = Self::DoDecode(
            &self.reader,
            image,
            self.try_harder,
            false,
            self.is_pure,
            1,
            self.min_line_count,
            self.return_errors,
        );

        if (result.is_empty() && self.try_rotate) {
            result = Self::DoDecode(
                &self.reader,
                image,
                self.try_harder,
                true,
                self.is_pure,
                1,
                self.min_line_count,
                self.return_errors,
            );
        }

        result.first().ok_or(Exceptions::NOT_FOUND)
        // return FirstOrDefault(std::move(result));
    }

    pub fn decode_with_max_symbols<B: crate::Binarizer>(
        &self,
        hints: &DecodingHintDictionary,
        image: &BinaryBitmap<B>,
        maxSymbols: u32,
    ) -> Result<Vec<RXingResult>> {
        let resH = Self::DoDecode(
            &self.reader,
            image,
            self.try_harder,
            false,
            self.is_pure,
            maxSymbols,
            self.min_line_count,
            self.return_errors,
        );
        if ((!maxSymbols || (resH) < maxSymbols) && self.try_rotate) {
            let resV = Self::DoDecode(
                &self.reader,
                image,
                self.try_harder,
                true,
                self.is_pure,
                maxSymbols - resH.len() as u32,
                self.min_line_count,
                self.return_errors,
            );
            // resH.insert(resH.end(), resV.begin(), resV.end());
            resH.append(&mut resV);
        }
        if resH.is_empty() {
            Err(Exceptions::NOT_FOUND)
        } else {
            Ok(resH)
        }
    }
}

impl<'a> Reader for ODReader<'_> {
    fn decode<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
    ) -> crate::common::Result<crate::RXingResult> {
        self.decode_with_hints(image, &HashMap::new())
    }

    fn decode_with_hints<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &crate::DecodingHintDictionary,
    ) -> crate::common::Result<crate::RXingResult> {
        self.decode_single(hints, image)
    }
}

impl<'a> MultipleBarcodeReader for ODReader<'_> {
    fn decode_multiple<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
    ) -> crate::common::Result<Vec<crate::RXingResult>> {
        self.decode_multiple_with_hints(image, &HashMap::new())
    }

    fn decode_multiple_with_hints<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &crate::DecodingHintDictionary,
    ) -> crate::common::Result<Vec<crate::RXingResult>> {
        self.decode_with_max_symbols(hints, image, u32::MAX)
    }
}

impl<'a> ODReader<'_> {
    pub fn new(hints: &DecodingHintDictionary) -> Self {
        unimplemented!()
    }
}

fn IncrementLineCount(r: &RXingResult) {
    unimplemented!()
    // ++r._lineCount;
}
