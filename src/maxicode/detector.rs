use crate::{
    common::{BitMatrix, DefaultGridSampler, DetectorRXingResult, GridSampler},
    Exceptions, RXingResultPoint,
};

const PATTERN_VARIANCE_ALLOWANCE: f32 = 1.3;

pub struct MaxicodeDetectionResult {
    bits: BitMatrix,
    points: Vec<RXingResultPoint>,
}

impl DetectorRXingResult for MaxicodeDetectionResult {
    fn getBits(&self) -> &BitMatrix {
        &self.bits
    }

    fn getPoints(&self) -> &[RXingResultPoint] {
        &self.points
    }
}

pub fn detect(image: &BitMatrix) -> Result<MaxicodeDetectionResult, Exceptions> {
    // find concentric circles
    let Some(circles) = find_concentric_circles(image) else {
        return Err(Exceptions::NotFoundException(None));
    };

    for circle in circles {
        // build a box around this circle, trying to find the barcode
        let symbol_box = box_symbol(image, circle);
        let grid_sampler = DefaultGridSampler::default();

        let [tl, tr, bl, br] = &symbol_box;

        let target_width = (tr.0 - tl.0).round() as u32;
        let target_height = (tr.1 - br.1).round() as u32;

        let bits = grid_sampler.sample_grid_detailed(
            image,
            target_width,
            target_height,
            0.0,
            target_height as f32, // tl p1ToX, p1ToY,
            target_width as f32,
            target_height as f32, // tr p2ToX, p2ToY,
            0.0,
            0.0, // bl p3ToX, p3ToY,
            target_width as f32,
            0.0, // br p4ToX, p4ToY,
            tl.0,
            tl.1, //p1FromX, p1FromY,
            tr.0,
            tr.1, // p2FromX, p2FromY,
            bl.0,
            bl.1, // p3FromX, p3FromY,
            br.0,
            br.1, //p4FromX, p4FromY,
        )?;
        return Ok(MaxicodeDetectionResult {
            bits: bits,
            points: symbol_box
                .iter()
                .map(|p| RXingResultPoint { x: p.0, y: p.1 })
                .collect(),
        });
    }

    Err(Exceptions::NotFoundException(None))
}

/// Locate concentric circles.
/// A bullseye looks like:
///       -
///       +
///       -
///       +
///       -
///       +
/// -+-+-+-+-+-+-
///       +
///       -
///       +
///       -
///       +
///       -
fn find_concentric_circles(image: &BitMatrix) -> Option<Vec<((u32, u32), u32)>> {
    let mut bullseyes = Vec::new();

    // find things that might be bullseye patterns, we start 6 in because a bullseye is at least six pixels in diameter
    let mut row = 6;
    while row < image.getHeight() - 6 {
        let mut current_column = 6;
        while current_column < image.getWidth() - 6 {
            // check if we can find something that looks like a bullseye
            if let Some((center, radius)) =
                find_next_bullseye_horizontal(image, row, current_column)
            {
                // check that the bullseye is not just a figment of our one-dimensional imagination
                if verify_bullseye_vertical(image, row, center, radius) {
                    // found a bullseye!

                    // add it
                    bullseyes.push(((center, row), row));

                    // update the search to the next possible location
                    current_column = center + radius;
                    continue;
                } else {
                    // false alarm, go on with the row
                    current_column = center;
                    continue;
                }
            } else {
                row += 1;
                break;
            }
        }
    }

    if bullseyes.is_empty() {
        None
    } else {
        Some(bullseyes)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Color {
    Black,
    White,
}

/// If a bullseye is found, returns (center, radius)
fn find_next_bullseye_horizontal(
    image: &BitMatrix,
    row: u32,
    start_column: u32,
) -> Option<(u32, u32)> {
    let mut buckets = [0_u32; 13];

    let mut column = start_column;
    let mut last_color = Color::White;
    let mut pointer = 0;
    while column < image.getWidth() - 6 {
        let local_bit = image.get(column, row);
        if local_bit && last_color == Color::White {
            pointer += 1;
            last_color = Color::Black;
        }
        buckets[pointer] += 1;

        // if we reached the end of our buckets, validate the segment
        if pointer == 6 {
            if validate_bullseye_widths(&buckets) {
                // bullseye widths look good, this is a bullseye
                return Some(get_bullseye_metadata(&buckets, column));
            } else {
                // false alarm, this pattern doesn't look enough like its evenly distributed
                // move on to the next set.
                pointer -= 1;
                buckets.copy_within(1.., 0);
                buckets[6] = 0;
            }
        }

        column += 1;
    }

    None
}

/// look up and down from the provided column to verify that a possible bullseye exists
fn verify_bullseye_vertical(
    image: &BitMatrix,
    row: u32,
    column: u32,
    expected_radius: u32,
) -> bool {
    // look up
    let up_vector = get_column_vector(image, column, row, true);

    // look down
    let down_vector = get_column_vector(image, column, row, false);

    let potential_bullseye = [
        down_vector[6],
        down_vector[5],
        down_vector[4],
        down_vector[3],
        down_vector[2],
        down_vector[1],
        down_vector[0] + up_vector[0],
        up_vector[1],
        up_vector[2],
        up_vector[3],
        up_vector[4],
        up_vector[5],
        up_vector[6],
    ];

    validate_bullseye_widths(&potential_bullseye)
}

fn get_column_vector(image: &BitMatrix, column: u32, start_row: u32, looking_up: bool) -> [u32; 7] {
    let mut buckets = [0_u32; 7];

    let mut row = start_row;
    let mut last_color = Color::White;
    let mut pointer = 0;
    while pointer < 6 && row > 0 && row < image.getHeight() {
        let local_bit = image.get(column, row);
        if local_bit && last_color == Color::White {
            pointer += 1;
            last_color = Color::Black;
        }
        buckets[pointer] += 1;

        row = if looking_up { row + 1 } else { row - 1 };
        pointer += 1;
    }

    buckets
}

fn validate_bullseye_widths(buckets: &[u32; 13]) -> bool {
    let sum = buckets.iter().sum::<u32>() as f32;
    let variance = sum / 13.0;

    variance < PATTERN_VARIANCE_ALLOWANCE
}

/// returns the (center , radius) of the possible bullseye
fn get_bullseye_metadata(buckets: &[u32; 13], column: u32) -> (u32, u32) {
    let radius = buckets.iter().skip(7).sum::<u32>() + buckets[7] / 2;
    let center = column + buckets.iter().take(6).sum::<u32>() + buckets[7] / 2;
    (center, radius)
}

fn box_symbol(image: &BitMatrix, circle: ((u32, u32), u32)) -> [(f32, f32); 4] {
    todo!()
}
