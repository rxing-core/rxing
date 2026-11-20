/*
* Copyright 2016 Nu-book Inc.
* Copyright 2016 ZXing authors
* Copyright 2022 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

// Result Reader::decode(const BinaryBitmap& image) const
// {
// #if 1
// 	if (!_hints.isPure())
// 		return FirstOrDefault(decode(image, 1));
// #endif

// 	auto binImg = image.getBitMatrix();
// 	if (binImg == nullptr)
// 		return {};

// 	DetectorResult detectorResult;
// 	if (_hints.hasFormat(BarcodeFormat::QRCode))
// 		detectorResult = DetectPureQR(*binImg);
// 	if (_hints.hasFormat(BarcodeFormat::MicroQRCode) && !detectorResult.isValid())
// 		detectorResult = DetectPureMQR(*binImg);

// 	if (!detectorResult.isValid())
// 		return {};

// 	auto decoderResult = Decode(detectorResult.bits());
// 	auto position = detectorResult.position();

// 	return Result(std::move(decoderResult), std::move(position),
// 				  detectorResult.bits().width() < 21 ? BarcodeFormat::MicroQRCode : BarcodeFormat::QRCode);
// }

// void logFPSet(const FinderPatternSet& fps [[maybe_unused]])
// {
// #ifdef PRINT_DEBUG
// 	auto drawLine = [](PointF a, PointF b) {
// 		int steps = maxAbsComponent(b - a);
// 		PointF dir = bresenhamDirection(PointF(b - a));
// 		for (int i = 0; i < steps; ++i)
// 			log(centered(a + i * dir), 2);
// 	};

// 	drawLine(fps.bl, fps.tl);
// 	drawLine(fps.tl, fps.tr);
// 	drawLine(fps.tr, fps.bl);
// #endif
// }

// Results Reader::decode(const BinaryBitmap& image, int maxSymbols) const
// {
// 	auto binImg = image.getBitMatrix();
// 	if (binImg == nullptr)
// 		return {};

// #ifdef PRINT_DEBUG
// 	LogMatrixWriter lmw(log, *binImg, 5, "qr-log.pnm");
// #endif

// 	auto allFPs = FindFinderPatterns(*binImg, _hints.tryHarder());

// #ifdef PRINT_DEBUG
// 	printf("allFPs: %d\n", Size(allFPs));
// #endif

// 	std::vector<ConcentricPattern> usedFPs;
// 	Results results;

// 	if (_hints.hasFormat(BarcodeFormat::QRCode)) {
// 		auto allFPSets = GenerateFinderPatternSets(allFPs);
// 		for (const auto& fpSet : allFPSets) {
// 			if (Contains(usedFPs, fpSet.bl) || Contains(usedFPs, fpSet.tl) || Contains(usedFPs, fpSet.tr))
// 				continue;

// 			logFPSet(fpSet);

// 			auto detectorResult = SampleQR(*binImg, fpSet);
// 			if (detectorResult.isValid()) {
// 				auto decoderResult = Decode(detectorResult.bits());
// 				auto position = detectorResult.position();
// 				if (decoderResult.isValid()) {
// 					usedFPs.push_back(fpSet.bl);
// 					usedFPs.push_back(fpSet.tl);
// 					usedFPs.push_back(fpSet.tr);
// 				}
// 				if (decoderResult.isValid(_hints.returnErrors())) {
// 					results.emplace_back(std::move(decoderResult), std::move(position), BarcodeFormat::QRCode);
// 					if (maxSymbols && Size(results) == maxSymbols)
// 						break;
// 				}
// 			}
// 		}
// 	}

// 	if (_hints.hasFormat(BarcodeFormat::MicroQRCode) && !(maxSymbols && Size(results) == maxSymbols)) {
// 		for (const auto& fp : allFPs) {
// 			if (Contains(usedFPs, fp))
// 				continue;

// 			auto detectorResult = SampleMQR(*binImg, fp);
// 			if (detectorResult.isValid()) {
// 				auto decoderResult = Decode(detectorResult.bits());
// 				auto position = detectorResult.position();
// 				if (decoderResult.isValid(_hints.returnErrors())) {
// 					results.emplace_back(std::move(decoderResult), std::move(position), BarcodeFormat::MicroQRCode);
// 					if (maxSymbols && Size(results) == maxSymbols)
// 						break;
// 				}

// 			}
// 		}
// 	}

// 	return results;
// }

// } // namespace ZXing::QRCode

use crate::{
    BarcodeFormat, DecodeHints, Exceptions, ImmutableReader, Point, RXingResult, Reader,
    common::{
        BitMatrix, DetectorRXingResult,
        cpp_essentials::{ConcentricPattern, DecoderResult},
    },
};

#[cfg(feature = "multi_barcode_readers")]
use crate::multi::MultipleBarcodeReader;

use super::{
    decoder::Decode,
    detector::{
        DetectPureMQR, DetectPureQR, DetectPureRMQR, FindFinderPatterns, FinderPatternSet,
        GenerateFinderPatternSets, SampleMQR, SampleQR, SampleRMQR,
    },
};

/// Dilation radii tried by [`QrReader::decode_qr_dilated`], as fractions of the module size.
const DILATION_MODULE_FRACTIONS: [f32; 2] = [0.2, 0.35];

#[derive(Default)]
pub struct QrReader;

impl Reader for QrReader {
    fn decode<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
    ) -> crate::common::Result<crate::RXingResult> {
        self.decode_with_hints(image, &DecodeHints::default())
    }

    fn decode_with_hints<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> crate::common::Result<RXingResult> {
        self.internal_decode_with_hints(image, hints)
    }
}

impl ImmutableReader for QrReader {
    fn immutable_decode_with_hints<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> crate::common::Result<RXingResult> {
        self.internal_decode_with_hints(image, hints)
    }
}

#[cfg(feature = "multi_barcode_readers")]
impl MultipleBarcodeReader for QrReader {
    fn decode_multiple<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
    ) -> crate::common::Result<Vec<crate::RXingResult>> {
        self.decode_multiple_with_hints(image, &DecodeHints::default())
    }

    fn decode_multiple_with_hints<B: crate::Binarizer>(
        &mut self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> crate::common::Result<Vec<crate::RXingResult>> {
        self.decode_set_number_with_hints(image, hints, u32::MAX)
    }
}

impl QrReader {
    /// Decode sampled QR bits, returning the result only if it is valid.
    fn decode_qr_bits(bits: &BitMatrix, hints: &DecodeHints) -> Option<DecoderResult<bool>> {
        Decode(bits)
            .ok()
            .map(|r| r.withCharacterSetHint(hints.CharacterSet.clone()))
            .filter(|r| r.isValid())
    }

    /// Fallback for stylized QR codes whose dark modules are drawn as marks much
    /// smaller than the module cell (dots, thin strokes): centre-point sampling
    /// then misses many dark modules even though detection succeeded. Re-sample
    /// the same finder pattern set from a dilated copy of the image, at radii
    /// proportional to the estimated module size. Reed-Solomon still validates
    /// the result, so this only rescues symbols, it does not invent them.
    ///
    /// Not part of the upstream C++ port. See <https://github.com/rxing-core/rxing/issues/128>.
    fn decode_qr_dilated(
        binImg: &BitMatrix,
        fpSet: &FinderPatternSet,
        dimension: u32,
        hints: &DecodeHints,
        cache: &mut Vec<(u32, BitMatrix)>,
    ) -> Option<(DecoderResult<bool>, Vec<Point>)> {
        if dimension <= 7 {
            return None;
        }
        // Finder centres are (dimension - 7) modules apart.
        let module_size =
            fpSet.tl.distance(fpSet.tr).max(fpSet.tl.distance(fpSet.bl)) / (dimension - 7) as f32;

        let mut last_radius = 0;
        for fraction in DILATION_MODULE_FRACTIONS {
            let radius = (module_size * fraction).round() as u32;
            if radius == 0 || radius == last_radius {
                continue;
            }
            last_radius = radius;

            let idx = match cache.iter().position(|(r, _)| *r == radius) {
                Some(idx) => idx,
                None => {
                    cache.push((radius, dilate(binImg, radius)?));
                    cache.len() - 1
                }
            };
            let dilated = &cache[idx].1;

            let Ok(detectorResult) = SampleQR(dilated, fpSet) else {
                continue;
            };
            if let Some(result) = Self::decode_qr_bits(detectorResult.getBits(), hints) {
                return Some((result, detectorResult.getPoints().to_vec()));
            }
        }
        None
    }

    fn decode_set_number_with_hints<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
        count: u32,
    ) -> crate::common::Result<Vec<RXingResult>> {
        let binImg = image.get_black_matrix(); //image.getBitMatrix();
        let maxSymbols = count;
        // if (binImg == nullptr)
        // 	{return {};}

        // #ifdef PRINT_DEBUG
        // 	LogMatrixWriter lmw(log, *binImg, 5, "qr-log.pnm");
        // #endif
        let try_harder = hints.TryHarder.unwrap_or(false);
        let min_module_size = hints.MinimumModuleSize.unwrap_or(0);

        let mut allFPs = FindFinderPatterns(binImg, try_harder, min_module_size);

        // #ifdef PRINT_DEBUG
        // 	printf("allFPs: %d\n", Size(allFPs));
        // #endif

        let mut usedFPs: Vec<ConcentricPattern> = Vec::new();
        let mut results: Vec<RXingResult> = Vec::new();

        let (check_qr, check_mqr, check_rmqr) = if let Some(formats) = &hints.PossibleFormats {
            (
                formats.contains(&BarcodeFormat::QR_CODE),
                formats.contains(&BarcodeFormat::MICRO_QR_CODE),
                formats.contains(&BarcodeFormat::RECTANGULAR_MICRO_QR_CODE),
            )
        } else {
            (true, true, true)
        };

        if check_qr {
            // if (_hints.hasFormat(BarcodeFormat::QRCode)) {
            let allFPSets = GenerateFinderPatternSets(&mut allFPs);
            // Lazily built dilated copies of `binImg`, keyed by radius (see `decode_qr_dilated`).
            let mut dilated_cache: Vec<(u32, BitMatrix)> = Vec::new();
            for fpSet in allFPSets {
                // for (const auto& fpSet : allFPSets) {
                if usedFPs.contains(&fpSet.bl)
                    || usedFPs.contains(&fpSet.tl)
                    || usedFPs.contains(&fpSet.tr)
                {
                    continue;
                }

                // logFPSet(fpSet);

                let detectorResult = SampleQR(binImg, &fpSet);
                if let Ok(detectorResult) = detectorResult {
                    // if (detectorResult.is_ok()) {
                    let decoded = Self::decode_qr_bits(detectorResult.getBits(), hints)
                        .map(|r| (r, detectorResult.getPoints().to_vec()))
                        .or_else(|| {
                            // Not part of the C++ port: stylized QR codes (#128).
                            if try_harder {
                                Self::decode_qr_dilated(
                                    binImg,
                                    &fpSet,
                                    detectorResult.getBits().getWidth(),
                                    hints,
                                    &mut dilated_cache,
                                )
                            } else {
                                None
                            }
                        });

                    if let Some((decoderResult, position)) = decoded {
                        usedFPs.push(fpSet.bl);
                        usedFPs.push(fpSet.tl);
                        usedFPs.push(fpSet.tr);

                        results.push(RXingResult::with_decoder_result(
                            decoderResult,
                            &position,
                            BarcodeFormat::QR_CODE,
                        ));

                        if maxSymbols != 0 && (results.len() as u32) == maxSymbols {
                            break;
                        }
                    }
                }
            }
        }
        if check_mqr && !(maxSymbols != 0 && (results.len() as u32) == maxSymbols) {
            // if (_hints.hasFormat(BarcodeFormat::MicroQRCode) && !(maxSymbols && Size(results) == maxSymbols)) {
            for fp in &allFPs {
                // for (const auto& fp : allFPs) {
                if usedFPs.contains(fp) {
                    continue;
                }

                let detectorResult = SampleMQR(binImg, *fp);
                if let Ok(detectorResult) = detectorResult {
                    // if (detectorResult.is_ok()) {
                    let decoderResult = Decode(detectorResult.getBits())
                        .map(|r| r.withCharacterSetHint(hints.CharacterSet.clone()));
                    let position = detectorResult.getPoints();
                    if let Ok(decoderResult) = decoderResult {
                        if decoderResult.isValid() {
                            results.push(RXingResult::with_decoder_result(
                                decoderResult,
                                position,
                                BarcodeFormat::MICRO_QR_CODE,
                            ));

                            if maxSymbols != 0 && (results.len() as u32) == maxSymbols {
                                break;
                            }
                        }
                    }
                }
            }
        }
        if check_rmqr && !(maxSymbols != 0 && (results.len() as u32) == maxSymbols) {
            for fp in &allFPs {
                // for (const auto& fp : allFPs) {
                if usedFPs.contains(fp) {
                    continue;
                }

                let detectorResult = SampleRMQR(binImg, *fp);
                if let Ok(detectorResult) = detectorResult {
                    // if (detectorResult.is_ok()) {
                    let decoderResult = Decode(detectorResult.getBits())
                        .map(|r| r.withCharacterSetHint(hints.CharacterSet.clone()));
                    let position = detectorResult.getPoints();
                    if let Ok(decoderResult) = decoderResult {
                        if decoderResult.isValid() {
                            results.push(RXingResult::with_decoder_result(
                                decoderResult,
                                position,
                                BarcodeFormat::RECTANGULAR_MICRO_QR_CODE,
                            ));

                            if maxSymbols != 0 && (results.len() as u32) == maxSymbols {
                                break;
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn internal_decode_with_hints<B: crate::Binarizer>(
        &self,
        image: &mut crate::BinaryBitmap<B>,
        hints: &DecodeHints,
    ) -> crate::common::Result<RXingResult> {
        // #if 1
        if !matches!(hints.PureBarcode, Some(true))
        // if !matches!(Some(hints.get(&DecodeHintType::PURE_BARCODE)))
        // if (!_hints.isPure())
        {
            return Ok(self
                .decode_set_number_with_hints(image, hints, 1)?
                .first()
                .ok_or(Exceptions::NOT_FOUND)?
                .clone());
            // return FirstOrDefault(decode(image, 1));
        }
        // #endif

        let binImg = image.get_black_matrix(); //image.getBitMatrix();
        // if (binImg == nullptr)
        // 	{return {};}

        let mut detectorResult = Err(Exceptions::NOT_FOUND);
        if let Some(formats) = &hints.PossibleFormats {
            if formats.contains(&BarcodeFormat::QR_CODE) {
                detectorResult = DetectPureQR(binImg);
            }
            if formats.contains(&BarcodeFormat::MICRO_QR_CODE) && detectorResult.is_err() {
                detectorResult = DetectPureMQR(binImg);
            }
            if formats.contains(&BarcodeFormat::RECTANGULAR_MICRO_QR_CODE)
                && detectorResult.is_err()
            {
                detectorResult = DetectPureRMQR(binImg);
            }
        }

        if detectorResult.is_err() {
            for decode_function in [DetectPureQR, DetectPureMQR, DetectPureRMQR] {
                detectorResult = decode_function(binImg);
                if detectorResult.is_ok() {
                    break;
                }
            }
        }

        let detectorResult = detectorResult?;

        // let detectorResult: DetectorResult;
        // if (_hints.hasFormat(BarcodeFormat::QR_CODE))
        // 	{detectorResult = DetectPureQR(binImg);}

        // if (_hints.hasFormat(BarcodeFormat::MICRO_QR_CODE) && !detectorResult.isValid())
        // 	{detectorResult = DetectPureMQR(binImg);}

        // if (!detectorResult.isValid())
        // 	{return {};}

        let decoderResult =
            Decode(detectorResult.getBits())?.withCharacterSetHint(hints.CharacterSet.clone());
        let position = detectorResult.getPoints();

        Ok(RXingResult::with_decoder_result(
            decoderResult,
            position,
            if detectorResult.getBits().width() != detectorResult.getBits().height() {
                BarcodeFormat::RECTANGULAR_MICRO_QR_CODE
            } else if detectorResult.getBits().width() < 21 {
                BarcodeFormat::MICRO_QR_CODE
            } else {
                BarcodeFormat::QR_CODE
            },
        ))
    }
}

/// Dilate set (dark) pixels by `radius` using a square kernel of side `2 * radius + 1`.
/// Separable (horizontal then vertical sliding window), so O(width * height).
fn dilate(image: &BitMatrix, radius: u32) -> Option<BitMatrix> {
    let (width, height) = (image.getWidth(), image.getHeight());
    let mut horizontal = BitMatrix::new(width, height).ok()?;
    for y in 0..height {
        dilate_line(width, radius, |x| image.get(x, y), |x| horizontal.set(x, y));
    }
    let mut result = BitMatrix::new(width, height).ok()?;
    for x in 0..width {
        dilate_line(
            height,
            radius,
            |y| horizontal.get(x, y),
            |y| result.set(x, y),
        );
    }
    Some(result)
}

/// One-dimensional dilation: `set(i)` is called for every `i` that has a set pixel
/// within `radius` of it, tracked via the last set position seen ahead of `i`.
fn dilate_line(len: u32, radius: u32, get: impl Fn(u32) -> bool, mut set: impl FnMut(u32)) {
    let mut last_set: Option<u32> = None;
    // Scan ahead by `radius` so `last_set` covers [i - radius, i + radius].
    for lead in 0..len + radius {
        if lead < len && get(lead) {
            last_set = Some(lead);
        }
        let Some(i) = lead.checked_sub(radius) else {
            continue;
        };
        if last_set.is_some_and(|s| s + radius >= i) {
            set(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::dilate;
    use crate::common::BitMatrix;

    #[test]
    fn dilate_square_kernel() {
        let mut m = BitMatrix::new(9, 7).unwrap();
        m.set(4, 3);
        m.set(0, 0);
        let d = dilate(&m, 2).unwrap();
        for y in 0u32..7 {
            for x in 0u32..9 {
                let expected = (x.abs_diff(4) <= 2 && y.abs_diff(3) <= 2) || (x <= 2 && y <= 2);
                assert_eq!(d.get(x, y), expected, "pixel ({x},{y})");
            }
        }
    }

    #[test]
    fn dilate_radius_zero_is_identity() {
        let mut m = BitMatrix::new(5, 5).unwrap();
        m.set(1, 2);
        m.set(3, 4);
        assert_eq!(dilate(&m, 0).unwrap(), m);
    }
}
