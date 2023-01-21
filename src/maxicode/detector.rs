use std::cmp;

use crate::{
    common::{
        detector::WhiteRectangleDetector, BitMatrix, DefaultGridSampler, DetectorRXingResult,
        GridSampler,
    },
    oned::MAX_AVG_VARIANCE,
    Exceptions, RXingResultPoint,
};

use super::MaxiCodeReader;

const PATTERN_VARIANCE_ALLOWANCE: f32 = 1.3;
const ROW_SCAN_SKIP: u32 = 5;

#[derive(Debug)]
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

struct Circle {
    center: (u32,u32),
    radius: u32,
    horizontal_buckets: [u32;11],
}

impl Circle {
    pub fn calculate_circle_variance(&self) -> f32 {
        let total_circle_pixels = self.horizontal_buckets.iter().sum::<u32>() as f32;
        let expected_module_size = total_circle_pixels / self.horizontal_buckets.len() as f32;
        let total_variance = self.horizontal_buckets.iter().fold(0.0, |acc, module_size| acc + (expected_module_size - *module_size as f32).abs());

        total_variance
    }
}

pub fn detect(image: &BitMatrix, try_harder: bool) -> Result<MaxicodeDetectionResult, Exceptions> {
    // find concentric circles
    let Some( mut circles) = find_concentric_circles(image) else {
        return Err(Exceptions::NotFoundException(None));
    };

    circles.sort_by(|a, b| compare_circle(a,b));

    for circle in &circles {
        // build a box around this circle, trying to find the barcode
        let Ok(symbol_box) = box_symbol(image, circle) else {
            if try_harder {
                continue;
            }else {
                return Err(Exceptions::NotFoundException(None))
            }
        };
        let grid_sampler = DefaultGridSampler::default();

        let [ tl, bl, tr, br ] = &symbol_box;

        let target_width = (tr.0 - tl.0).round() as u32;
        let target_height = (br.1 - tr.1).round() as u32;

        let Ok(bits) = grid_sampler.sample_grid_detailed(
            image,
            target_width,
            target_height,
            0.0,
            0.0, 
            target_width as f32 ,
            0.0, 
            target_width as f32,
            target_height as f32, 
            0.0,
            target_height as f32, 
            tl.0,
            tl.1, 
            tr.0,
            tr.1, 
            br.0,
            br.1, 
            bl.0,
            bl.1, 
        ) else {
            if try_harder {
                continue;
            }else {
                return Err(Exceptions::NotFoundException(None))
            }
        };
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
///       +
///       -
///       +
///       -
///       +
///  +-+-+-+-+-+
///       +
///       -
///       +
///       -
///       +
fn find_concentric_circles(image: &BitMatrix) -> Option<Vec<Circle>> {
    let mut bullseyes = Vec::new();

    // find things that might be bullseye patterns, we start 6 in because a bullseye is at least six pixels in diameter
    let mut row = 6;
    while row < image.getHeight() - 6 {
        let mut current_column = 6;
        while current_column < image.getWidth() - 6 {
            // check if we can find something that looks like a bullseye
            if let Some((center, radius, buckets)) =
                find_next_bullseye_horizontal(image, row, current_column)
            {
                // check that the bullseye is not just a figment of our one-dimensional imagination
                if verify_bullseye_vertical(image, row, center, radius) {
                    // found a bullseye!

                    // add it
                    bullseyes.push(Circle {
                        center: (center,row),
                        radius,
                        horizontal_buckets: buckets,
                    });

                    // update the search to the next possible location
                    current_column = center + radius;
                    continue;
                } else {
                    // false alarm, go on with the row
                    current_column = center - radius + (radius / 4);
                    continue;
                }
            } else {
                row += ROW_SCAN_SKIP;
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

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

impl From<bool> for Color {
    fn from(value: bool) -> Self {
        match value {
            true => Color::Black,
            false => Color::White,
        }
    }
}

/// If a bullseye is found, returns (center, radius)
fn find_next_bullseye_horizontal(
    image: &BitMatrix,
    row: u32,
    start_column: u32,
) -> Option<(u32, u32, [u32;11])> {
    let mut buckets = [0_u32; 11];

    let mut column = start_column;
    let mut last_color = Color::Black;
    let mut pointer = 0;

    // remove leading white space
    while !image.get(column, row) && column < image.getWidth() - 6 {
        column += 1;
    }

    while column < image.getWidth() - 6 {
        let local_bit = image.get(column, row);
        if Color::from(local_bit) != last_color {
            last_color = !last_color;
            pointer += 1;

            // if we reached the end of our buckets, validate the segment
            if pointer == 11 {
                if validate_bullseye_widths(&buckets) {
                    // bullseye widths look good, this is a bullseye
                    return Some(get_bullseye_metadata(&buckets, column));
                } else {
                    // false alarm, this pattern doesn't look enough like it's evenly distributed
                    // move on to the next set.
                    pointer -= 1;
                    buckets.copy_within(1.., 0);
                    buckets[10] = 0;
                    column += 1;
                    continue;
                }
            }
        }

        buckets[pointer] += 1;

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
    ];

    validate_bullseye_widths(&potential_bullseye)
}

fn get_column_vector(image: &BitMatrix, column: u32, start_row: u32, looking_up: bool) -> [u32; 6] {
    let mut buckets = [0_u32; 6];

    let mut row = start_row;
    let mut last_color = Color::White;
    let mut pointer = 0;
    while row > 0 && row < image.getHeight() {
        let local_bit = image.get(column, row);
        if Color::from(local_bit) != last_color {
            last_color = !last_color;
            pointer += 1;
        }
        if pointer > 5 {
            break;
        }
        buckets[pointer] += 1;

        row = if looking_up { row + 1 } else { row - 1 };
    }

    buckets
}

fn validate_bullseye_widths(buckets: &[u32; 11]) -> bool {
    let total_width = buckets.iter().sum::<u32>() as f32;
    let estimated_module_size = total_width / buckets.len() as f32;

    let max_variance = estimated_module_size / 3.0;

    let b1 = (estimated_module_size - buckets[0] as f32).abs();
    let b2 = (estimated_module_size - buckets[1] as f32).abs();
    let b3 = (estimated_module_size - buckets[2] as f32).abs();
    let b4 = (estimated_module_size - buckets[3] as f32).abs();
    let b5 = (estimated_module_size - buckets[4] as f32).abs();
    let b6 = (estimated_module_size - buckets[5] as f32).abs();
    let b7 = (estimated_module_size - buckets[6] as f32).abs();
    let b8 = (estimated_module_size - buckets[7] as f32).abs();
    let b9 = (estimated_module_size - buckets[8] as f32).abs();
    let b10 = (estimated_module_size - buckets[9] as f32).abs();
    let b11 = (estimated_module_size - buckets[10] as f32).abs();

    (
        b1 < max_variance &&
        b2 < max_variance &&
        b3 < max_variance &&
        b4 < max_variance &&
        b5 < max_variance &&
        b6  < max_variance * 2.0 &&
        b7 < max_variance &&
        b8 < max_variance &&
        b9 < max_variance &&
        b10 < max_variance &&
        b11 < max_variance 
    )
}

/// returns the (center , radius) of the possible bullseye
fn get_bullseye_metadata(buckets: &[u32; 11], column: u32) -> (u32, u32, [u32;11]) {
    let radius = buckets.iter().sum::<u32>() / 2; //buckets.iter().skip(6).sum::<u32>() + buckets[5] / 2;
    let center = column - radius; //buckets.iter().take(5).sum::<u32>() - (buckets[5] / 2);
    (center, radius, *buckets)
}

fn box_symbol(image: &BitMatrix, circle: &Circle) -> Result<[(f32, f32); 4], Exceptions> {
    let (barcode_width, barcode_height) = guess_barcode_size(circle);
let results = if let Ok(res) = || -> Result<[RXingResultPoint;4], Exceptions> {
    let wrd = WhiteRectangleDetector::new(
        image,
        barcode_width as i32,
        circle.center .0 as i32,
        circle.center .1 as i32,
    )?;
     wrd.detect()}(){
        res
     }else {
        // let center = circle.center;
        // let left_boundary = cmp::max(center.0 as i32 - (barcode_width as f32/ 2.0).ceil() as i32, 0) as u32;
        // let right_boundary = center.0 + (barcode_width as f32 / 2.0).ceil() as u32;
        // let top_boundary = center.1 + (barcode_height as f32 / 2.0).ceil() as u32;
        // let bottom_boundary = cmp::max(center.1 as i32 - (barcode_height as f32 / 2.0).ceil() as i32, 0) as u32;
        let left_boundary = 0;
        let right_boundary = barcode_width;
        let top_boundary = barcode_height;
        let bottom_boundary = 0;

        [
            RXingResultPoint::new(left_boundary as f32, bottom_boundary as f32),
            RXingResultPoint::new(left_boundary as f32, top_boundary as f32),
        RXingResultPoint::new(right_boundary as f32, bottom_boundary as f32),
        RXingResultPoint::new(right_boundary as f32, top_boundary as f32),
        ]
     };

    let adjusted_results = adjust_wrt_detection_box(results);

    Ok([
        (adjusted_results[0].x, adjusted_results[0].y),
        (adjusted_results[1].x, adjusted_results[1].y),
        (adjusted_results[2].x, adjusted_results[2].y),
        (adjusted_results[3].x, adjusted_results[3].y),
    ])
}

// adjusts the bounding box a bit, order is [ bl, tl, br, tr ]
fn adjust_wrt_detection_box( input: [RXingResultPoint;4]) -> [RXingResultPoint;4]{
    let [bl, tl, br, tr] = input;
    let selected_top = max_float(tl.y, tr.y);
    let selected_bottom = min_float(bl.y, br.y);
    let selected_left = min_float(tl.x, bl.x);
    let selected_right = max_float(tr.x, br.x);

    [
        RXingResultPoint::new(selected_left, selected_bottom),
        RXingResultPoint::new(selected_left, selected_top),
        RXingResultPoint::new(selected_right, selected_bottom),
        RXingResultPoint::new(selected_right, selected_top),
    ]
}

/// calculate a likely size for the barcode.
/// we know that maxicode symbols are square,
/// and that the central bullseye is roughly
/// 1/3 the width of the image.
/// returns (width, height)
fn guess_barcode_size(circle: &Circle) -> (u32,u32) {
    let module_size = circle.horizontal_buckets.iter().sum::<u32>() as f32 / circle.horizontal_buckets.len() as f32;
    
    let height = (module_size * 1.1 * MaxiCodeReader::MATRIX_HEIGHT as f32).round() + (module_size /2.0);
    let width = height * 1.03;

(width as u32, height as u32)

    // ((module_size * 1.13 * MaxiCodeReader::MATRIX_WIDTH as f32).round() as u32 + (module_size /2.0) as u32,

    // (module_size * 1.1 * MaxiCodeReader::MATRIX_HEIGHT as f32).round() as u32 + (module_size /2.0) as u32
// )
    // ((radius as f32 / 5.0) * 33.0).round() as u32
}

fn compare_circle(a: &Circle, b: &Circle) -> std::cmp::Ordering {
    let a_var = a.calculate_circle_variance();
    let b_var = b.calculate_circle_variance();

    if a_var < b_var {
        std::cmp::Ordering::Greater
    }else if a_var > b_var {
        std::cmp::Ordering::Less
    }else {
        std::cmp::Ordering::Equal
    }
}

pub fn read_bits(image:&BitMatrix) -> Result<BitMatrix,Exceptions> {
    let enclosingRectangle = image.getEnclosingRectangle().unwrap();
    
    let left = enclosingRectangle[0];
    let top = enclosingRectangle[1];
    let width = enclosingRectangle[2];
    let height = enclosingRectangle[3];

    // let top = image.getHeight();
    // let left = 0;
    // let width = image.getWidth();
    // let height = image.getHeight(); 

    // Now just read off the bits
    let mut bits = BitMatrix::new(MaxiCodeReader::MATRIX_WIDTH, MaxiCodeReader::MATRIX_HEIGHT)?;
    for y in 0..MaxiCodeReader::MATRIX_HEIGHT {
        // for (int y = 0; y < MATRIX_HEIGHT; y++) {
        let iy = (top + (y * height + height / 2) / MaxiCodeReader::MATRIX_HEIGHT).min(height - 1);
        for x in 0..MaxiCodeReader::MATRIX_WIDTH {
            // for (int x = 0; x < MATRIX_WIDTH; x++) {
            // srowen: I don't quite understand why the formula below is necessary, but it
            // can walk off the image if left + width = the right boundary. So cap it.
            let ix = left
                + ((x * width + width / 2 + (y & 0x01) * width / 2) / MaxiCodeReader::MATRIX_WIDTH)
                    .min(width - 1);
            if image.get(ix, iy) {
                bits.set(x, y);
            }
        }
    }
    
    Ok(bits)
}

#[cfg(test)]
mod detector_test {
    use std::io::{Write, Read};

    use crate::{common::{HybridBinarizer, DetectorRXingResult}, BufferedImageLuminanceSource, Binarizer, maxicode::detector::read_bits};

#[test]
fn simple() {
    finder_test("test_resources/blackbox/maxicode-1/1.png", "test_resources/blackbox/maxicode-1/1.txt")
}

#[test]
fn mode_2() {
    finder_test("test_resources/blackbox/maxicode-1/MODE2.png", "test_resources/blackbox/maxicode-1/MODE2.txt")

    
    
}

#[test]
fn mode3() {
    finder_test("test_resources/blackbox/maxicode-1/MODE3.png", "test_resources/blackbox/maxicode-1/MODE3.txt")
}

#[test]
fn mixed_sets() {
    finder_test("test_resources/blackbox/maxicode-1/mode4-mixed-sets.png", "test_resources/blackbox/maxicode-1/mode4-mixed-sets.txt")
}

#[test]
fn mode4() {
    finder_test("test_resources/blackbox/maxicode-1/MODE4.png", "test_resources/blackbox/maxicode-1/MODE4.txt")
}

#[test]
fn mode5() {
    finder_test("test_resources/blackbox/maxicode-1/MODE5.png", "test_resources/blackbox/maxicode-1/MODE5.txt")
}

#[test]
fn mode6() {
    finder_test("test_resources/blackbox/maxicode-1/MODE6.png", "test_resources/blackbox/maxicode-1/MODE6.txt")
}

fn finder_test(image: &str, data: &str) {
    let filename = image;
    let img = image::open(filename).unwrap();
    let lum_src = BufferedImageLuminanceSource::new(img);
    let binarizer = HybridBinarizer::new(Box::new(lum_src));
    let bitmatrix = binarizer.getBlackMatrix().unwrap();

    // std::fs::File::create("dbgfle").unwrap().write_all(bitmatrix.to_string().as_bytes()).expect("write");
    let mut expected_result = String::new();
     std::fs::File::open(data).unwrap().read_to_string(&mut expected_result).unwrap();

    let detection = super::detect(&bitmatrix, true).unwrap();
    
    // std::fs::File::create("dbgfle-transformed").unwrap().write_all(detection.getBits().to_string().as_bytes()).expect("write");

    let bits = read_bits(detection.getBits()).expect("read bits");

    // std::fs::File::create("dbgfle-read").unwrap().write_all(bits.to_string().as_bytes()).expect("write");

    let result = crate::maxicode::decoder::decode(&bits).expect("must decode");

    assert_eq!(expected_result, result.getText());
}
}

fn min_float<T:PartialOrd>(a:T,b:T) -> T {
    if a > b { b } else { a }
}

fn max_float<T:PartialOrd>(a:T,b:T) -> T {
    if a < b { b } else { a }
}