/*
* Copyright 2020 Axel Waggershauser
*/
// SPDX-License-Identifier: Apache-2.0

use crate::{
    Point,
    common::{
        BitMatrix, Quadrilateral,
        cpp_essentials::{FixedPattern, LocateConcentricPattern},
    },
    point,
};

const E2E: bool = true;

const COMPACT_BULLSEYE: FixedPattern<7, 7> = FixedPattern::new([1, 1, 1, 1, 1, 1, 1]);
const COMPACT_NB_CENTER_LAYERS: u32 = 5;

const FULL_BULLSEYE: FixedPattern<11, 11> = FixedPattern::new([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
const FULL_NB_CENTER_LAYERS: u32 = 7;

pub struct BullseyeMatch {
    pub corners: Quadrilateral,
    pub compact: bool,
    pub nb_center_layers: u32,
}

/// Locates the Aztec bullseye near `seed`, returning the corners just outside its outermost
/// (reference) ring
pub fn locate_bullseye(image: &BitMatrix, seed: Point) -> Option<BullseyeMatch> {
    // Smallest real Aztec symbol (compact, 1 layer) is 15x15 modules; anything smaller can't
    // possibly contain a bullseye, and would otherwise drive `range` to 0 below, which
    // `LocateConcentricPattern`'s internals assert against.
    const MIN_IMAGE_SIZE: u32 = 15;
    if image.getWidth() < MIN_IMAGE_SIZE || image.getHeight() < MIN_IMAGE_SIZE {
        return None;
    }

    let range = (image.getWidth().min(image.getHeight()) / 2) as i32;

    if let Some(m) = try_pattern::<11, 11>(
        image,
        seed,
        range,
        &FULL_BULLSEYE.into(),
        FULL_NB_CENTER_LAYERS,
        false,
    ) {
        return Some(m);
    }

    try_pattern::<7, 7>(
        image,
        seed,
        range,
        &COMPACT_BULLSEYE.into(),
        COMPACT_NB_CENTER_LAYERS,
        true,
    )
}

fn try_pattern<const LEN: usize, const SUM: usize>(
    image: &BitMatrix,
    seed: Point,
    range: i32,
    pattern: &[u16; LEN],
    nb_center_layers: u32,
    compact: bool,
) -> Option<BullseyeMatch> {
    let found = LocateConcentricPattern::<E2E, LEN, SUM>(image, pattern, seed, range)?;

    let module_size = found.size as f32 / LEN as f32;
    let half_side = nb_center_layers as f32 * module_size;
    let center = found.p;

    let corners = Quadrilateral::new(
        point(center.x + half_side, center.y - half_side), // top right
        point(center.x + half_side, center.y + half_side), // bottom right
        point(center.x - half_side, center.y + half_side), // bottom left
        point(center.x - half_side, center.y - half_side), // top left
    );

    Some(BullseyeMatch {
        corners,
        compact,
        nb_center_layers,
    })
}
