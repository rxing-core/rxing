/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use crate::common::cpp_essentials::{GetPatternRow, PatternRow, PatternView};
use crate::{multi::MultipleBarcodeReader, RXingResult, Reader};
use crate::{
    point, BarcodeFormat, BinaryBitmap, DecodingHintDictionary, Exceptions, PointT, ResultPoint,
};
use crate::{Binarizer, DecodeHintType, DecodeHintValue};

use crate::common::{LineOrientation, Quadrilateral, Result};

use super::dxfilm_edge_reader::DXFilmEdgeReader;
use super::row_reader::{DecodingState, RowReader};

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
        _returnErrors: bool,
    ) -> Vec<RXingResult> {
        let mut res: Vec<Option<RXingResult>> = Vec::new();

        let mut decodingState: Vec<Option<DecodingState>> = vec![Some(DecodingState::default()); 1];
        // std::vector<std::unique_ptr<RowReader::DecodingState>> decodingState(readers.size());

        let mut minLineCount = minLineCount;

        let mut width: i32 = image.get_width() as i32;
        let mut height: i32 = image.get_height() as i32;

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
        let maxLines: i32 = if tryHarder {
            height // Look at the whole image, not just the center
        } else {
            15 // 15 rows spaced 1/32 apart is roughly the middle half of the image
        };

        if (isPure) {
            minLineCount = 1;
        }
        let mut checkRows = Vec::new();

        let mut bars: PatternRow = PatternRow::new(vec![0; 128]); // e.g. EAN-13 has 59 bars/spaces
                                                                  // bars.reserve(128); // e.g. EAN-13 has 59 bars/spaces

        // #ifdef PRINT_DEBUG
        // BitMatrix dbg(width, height);
        // #endif

        let mut i = 0;
        'outer: while i < maxLines {
            // for (int i = 0; i < maxLines; i++) {

            // Scanning from the middle out. Determine which row we're looking at next:
            let rowStepsAboveOrBelow: i32 = (i + 1) / 2;
            let isAbove: bool = (i & 0x01) == 0; // i.e. is x even?
            let mut rowNumber: i32 = middle
                + rowStep
                    * (if isAbove {
                        rowStepsAboveOrBelow
                    } else {
                        -rowStepsAboveOrBelow
                    });
            let mut isCheckRow: bool = false;
            if (rowNumber < 0 || rowNumber >= height) {
                // Oops, if we run off the top or bottom, stop
                break;
            }

            // See if we have additional check rows (see below) to process
            if (!checkRows.is_empty()) {
                //--i;
                i -= 1;
                rowNumber = *checkRows.last().unwrap_or(&0);
                checkRows.pop();
                isCheckRow = true;
                if (rowNumber < 0 || rowNumber >= height) {
                    i += 1;
                    continue;
                }
            }

            let br: Vec<bool> = if let Ok(r) = image.get_black_line(
                rowNumber as usize,
                if rotate {
                    LineOrientation::Column
                } else {
                    LineOrientation::Row
                },
            ) {
                r.as_ref().into()
            } else {
                i += 1;
                continue;
            };
            // let img = if rotate {let a = image.rotate_counter_clockwise(); &a} else {image};
            //  let Ok(black_row ) = img.get_black_row(rowNumber as usize) else {continue;};
            //  let br : Vec<bool> = black_row.as_ref().into();
            GetPatternRow(&br, &mut bars);

            // if (!image.getPatternRow(rowNumber, if rotate { 90 } else { 0 }, bars)) {
            //     continue;
            // }

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
                    bars.rev();
                }
                let readers = vec![reader];
                // Look for a barcode
                for r in 0..readers.len() {
                    // for (size_t r = 0; r < readers.size(); ++r) {
                    // If this is a pure symbol, then checking a single non-empty line is sufficient for all but the stacked
                    // DataBar codes. They are the only ones using the decodingState, which we can use as a flag here.
                    if (isPure && i > 0 && decodingState[r].is_none()) {
                        continue;
                    }

                    let mut next = PatternView::new(&bars);
                    loop {
                        let mut result_hld = readers[r]
                            .decodePattern(rowNumber as u32, &mut next, &mut decodingState[r])
                            .ok();
                        if result_hld.is_some()
                        /*|| (returnErrors && result.is_none())*/
                        {
                            let mut result = result_hld.as_mut().unwrap();
                            IncrementLineCount(&mut result);
                            if (upsideDown) {
                                // update position (flip horizontally).
                                let points = result.getPointsMut();
                                for p in points {
                                    // for (auto& p : points) {
                                    *p = point(width as f32 - p.getX() - 1.0, p.getY());
                                }
                                // result.addPoints(points);
                                // result.setPosition(std::move(points));
                            }
                            if (rotate) {
                                let points = result.getPointsMut();
                                for p in points {
                                    // for (auto& p : points) {
                                    *p = point(p.getY(), width as f32 - p.getX() - 1.0);
                                }
                                // result.addPoints(points);
                                // result.setPosition(std::move(points));
                            }

                            // check if we know this code already
                            for other_hld in res.iter_mut() {
                                let Some(mut other) = other_hld.as_mut() else {
                                    continue;
                                };
                                // for (auto& other : res) {
                                if (result == &other) {
                                    // merge the position information
                                    let dTop = PointT::maxAbsComponent(
                                        other.getPoints()[0] - result.getPoints()[0],
                                    );
                                    let dBot = PointT::maxAbsComponent(
                                        other.getPoints()[2] - result.getPoints()[0],
                                    );
                                    let mut points = other.getPoints().clone();
                                    if (dTop < dBot
                                        || (dTop == dBot
                                            && rotate
                                                ^ (PointT::sumAbsComponent(points[0])
                                                    > PointT::sumAbsComponent(
                                                        result.getPoints()[0],
                                                    ))))
                                    {
                                        points[0] = result.getPoints()[0];
                                        points[1] = result.getPoints()[1];
                                    } else {
                                        points[2] = result.getPoints()[2];
                                        points[3] = result.getPoints()[3];
                                    }
                                    other.replace_points(points);
                                    IncrementLineCount(&mut other);
                                    // clear the result, so we don't insert it again below
                                    // result_hld = None; //Result();
                                    break;
                                }
                            }

                            if (result.getBarcodeFormat() != &BarcodeFormat::UNSUPORTED_FORMAT) {
                                res.push(Some(result.clone()));
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

                            if (maxSymbols > 0
                                && res.iter().fold(0, |acc, e| {
                                    if let Some(itm) = &res[r] {
                                        acc + i32::from((itm.line_count() >= minLineCount as usize))
                                    } else {
                                        acc
                                    }
                                }) == maxSymbols as i32)
                            {
                                break 'outer;
                            }
                        }
                        // make sure we make progress and we start the next try on a bar
                        next.shift(2 - (next.index() % 2));
                        next.extend();

                        if !(tryHarder && next.size() > 0) {
                            break;
                        }
                    } //while (tryHarder && next.size());
                }
            }
            i += 1;
        }

        // out:
        // remove all symbols with insufficient line count
        res.retain(|e| {
            if let Some(itm) = e {
                itm.line_count() < minLineCount as usize
            } else {
                false
            }
        });
        // let it = std::remove_if(res.begin(), res.end(), [&](auto&& r) { return r.lineCount() < minLineCount; });
        // res.erase(it, res.end());

        // if symbols overlap, remove the one with a lower line count
        for i in 0..res.len() {
            for j in i..res.len() {
                if res[i].is_some() && res[j].is_some() {
                    let Ok(q1) = Quadrilateral::try_from(res[i].as_ref().unwrap().getPoints())
                    else {
                        continue;
                    };
                    let Ok(q2) = Quadrilateral::try_from(res[j].as_ref().unwrap().getPoints())
                    else {
                        continue;
                    };
                    if (Quadrilateral::have_intersecting_bounding_boxes(&q1, &q2)) {
                        let a_lc = res[i].as_ref().unwrap().line_count();
                        let b_lc = res[j].as_ref().unwrap().line_count();
                        if a_lc < b_lc {
                            res[i] = None;
                        } else {
                            res[j] = None;
                        }
                    }
                }
            }
        }
        // // if symbols overlap, remove the one with a lower line count
        // for (i, a_hld) in res.iter().enumerate() {
        //     let a = a_hld.as_ref().unwrap();
        //     // let a_lc = a_hld.as_ref().unwrap().line_count();
        //     // let a_pts = a_hld.as_ref().unwrap().getPoints().clone();
        //     // for (auto a = res.begin(); a != res.end(); ++a){
        //     for (j, b_hld) in res.iter().skip(i).enumerate() {
        //         let b = b_hld.as_ref().unwrap();
        //         // for (auto b = std::next(a); b != res.end(); ++b){
        //             let Ok(q1) =Quadrilateral::try_from(a.getPoints()) else {continue;};
        //             let Ok(q2) = Quadrilateral::try_from(b.getPoints()) else {continue;};
        //         if (Quadrilateral::have_intersecting_bounding_boxes(&q1, &q2)) {
        //             if a.line_count() < b.line_count() {
        //                 // delete_list.insert(i);
        //                 // *a_hld = None;
        //                 res.remove(i);
        //             } else {
        //                 // delete_list.insert(i+j);
        //                 res.remove(i+j);
        //             }
        //         }
        //     }
        // }

        //TODO: C++20 res.erase_if()
        res.retain(|r| {
            if let Some(itm) = r {
                itm.getBarcodeFormat() == &BarcodeFormat::UNSUPORTED_FORMAT
            } else {
                false
            }
        });
        // it = std::remove_if(res.begin(), res.end(), [](auto&& r) { return r.format() == BarcodeFormat::None; });
        // res.erase(it, res.end());

        // #ifdef PRINT_DEBUG
        // SaveAsPBM(dbg, rotate ? "od-log-r.pnm" : "od-log.pnm");
        // #endif

        res.iter().cloned().filter_map(|e| e).collect()
    }
}

impl<'a> ODReader<'_> {
    pub fn decode_single<B: crate::Binarizer>(
        &self,
        _hints: &DecodingHintDictionary,
        image: &BinaryBitmap<B>,
    ) -> Result<RXingResult> {
        let result = self.decode_with_max_symbols(_hints, image, u32::MAX)?;

        result.first().cloned().ok_or(Exceptions::NOT_FOUND)
        // return FirstOrDefault(std::move(result));
    }

    pub fn decode_with_max_symbols<B: crate::Binarizer>(
        &self,
        _hints: &DecodingHintDictionary,
        image: &BinaryBitmap<B>,
        maxSymbols: u32,
    ) -> Result<Vec<RXingResult>> {
        let mut resH = Self::DoDecode(
            &self.reader,
            image,
            self.try_harder,
            false,
            self.is_pure,
            maxSymbols,
            self.min_line_count,
            self.return_errors,
        );
        if ((!(maxSymbols != 0) || (resH.len()) < maxSymbols as usize) && self.try_rotate) {
            let mut resV = Self::DoDecode(
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
    pub fn new(hints: &DecodingHintDictionary) -> ODReader {
        ODReader {
            reader: DXFilmEdgeReader::new(hints),
            try_harder: matches!(
                hints.get(&DecodeHintType::TRY_HARDER),
                Some(DecodeHintValue::TryHarder(true))
            ),
            is_pure: matches!(
                hints.get(&DecodeHintType::PURE_BARCODE),
                Some(DecodeHintValue::PureBarcode(true))
            ),
            min_line_count: 2,
            return_errors: false,
            try_rotate: matches!(
                hints.get(&DecodeHintType::TRY_HARDER),
                Some(DecodeHintValue::TryHarder(true))
            ),
        }
    }
}

fn IncrementLineCount(r: &mut RXingResult) {
    r.set_line_count(r.line_count() + 1)
}
