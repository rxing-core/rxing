#![allow(dead_code)]
use num::integer::Roots;

use crate::{
    common::{
        detector::MathUtils, BitMatrix, DefaultGridSampler, DetectorRXingResult, GridSampler,
    },
    Exceptions, RXingResultPoint,
};

use super::MaxiCodeReader;

const ROW_SCAN_SKIP: u32 = 2;

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

struct Circle<'a> {
    center: (u32, u32),
    radius: u32,
    horizontal_buckets: [u32; 11],
    vertical_buckets: [u32; 11],
    image: &'a BitMatrix,
}

impl Circle<'_> {
    pub fn calculate_circle_variance(&self) -> f32 {
        let total_width_even = self
            .horizontal_buckets
            .iter()
            .zip(self.vertical_buckets.iter())
            .enumerate()
            .filter_map(|e| {
                if e.0 != 5 && (e.0 == 0 || e.0 % 2 == 0) {
                    Some(*e.1 .0 + *e.1 .1)
                } else {
                    None
                }
            })
            .sum::<u32>() as f32;
        let total_width_odd = self
            .horizontal_buckets
            .iter()
            .zip(self.vertical_buckets.iter())
            .enumerate()
            .filter_map(|e| {
                if e.0 != 5 && (e.0 != 0 && e.0 % 2 != 0) {
                    Some(*e.1 .0 + *e.1 .1)
                } else {
                    None
                }
            })
            .sum::<u32>() as f32;

        let estimated_module_size_even = total_width_even / 10.0;
        let estimated_module_size_odd = total_width_odd / 10.0;

        // let expected_module_size = total_circle_pixels / (self.horizontal_buckets.len() - 1) as f32;
        let total_variance_even = self
            .horizontal_buckets
            .iter()
            .enumerate()
            .filter(|p| p.0 != 5 && (p.0 == 0 || p.0 % 2 == 0))
            .fold(0.0, |acc, (_, module_size)| {
                acc + (estimated_module_size_even - *module_size as f32).abs()
            });

        let total_variance_odd = self
            .horizontal_buckets
            .iter()
            .enumerate()
            .filter(|p| p.0 != 5 && (p.0 != 0 || p.0 % 2 != 0))
            .fold(0.0, |acc, (_, module_size)| {
                acc + (estimated_module_size_odd - *module_size as f32).abs()
            });

        let expected_area_vertical =
            (self.horizontal_buckets[5] / 2).pow(2) as f32 * std::f32::consts::PI;
        let expected_area_horizontal =
            (self.vertical_buckets[5] / 2).pow(2) as f32 * std::f32::consts::PI;
        let circle_area_average = (expected_area_horizontal + expected_area_vertical) / 2.0;

        let circle_area_variance = (expected_area_horizontal - circle_area_average).abs()
            + (expected_area_vertical - circle_area_average).abs();

        (total_variance_even + total_variance_odd + circle_area_variance) / 3.0
    }

    pub fn calculate_center_point_std_dev(circles: &[Self]) -> ((u32, u32), (u32, u32)) {
        let (x_total, y_total) = circles.iter().fold((0, 0), |(x_acc, y_acc), c| {
            (x_acc + c.center.0, y_acc + c.center.1)
        });
        let x_mean = x_total as f64 / circles.len() as f64;
        let y_mean = y_total as f64 / circles.len() as f64;
        let (x_squared_variances, y_squared_variances) =
            circles.iter().fold((0.0, 0.0), |(x_acc, y_acc), c| {
                (
                    x_acc + (c.center.0 as f64 - x_mean).powf(2.0),
                    y_acc + (c.center.1 as f64 - y_mean).powf(2.0),
                )
            });
        let x_squared_variance_mean = x_squared_variances / circles.len() as f64;
        let y_squared_variance_mean = y_squared_variances / circles.len() as f64;
        let x_standard_deviation = x_squared_variance_mean.sqrt();
        let y_standard_deviation = y_squared_variance_mean.sqrt();

        (
            (x_standard_deviation as u32, y_standard_deviation as u32),
            (x_mean as u32, y_mean as u32),
        )
    }

    /// detect a higher accuracy center point for a circle
    pub fn calculate_high_accuracy_center(&mut self) {
        let [point_1, point_2] = self.find_width_at_degree(7.0).1;
        let point_3 = self.find_width_at_degree(97.0).1[0];
        let guessed_center_point = Self::find_center(point_1, point_2, point_3);
        self.center = (
            guessed_center_point.0.round() as u32,
            guessed_center_point.1.round() as u32,
        )
    }

    /// detect an ellipse, and try to find defining points of it.
    /// returns (ellipse, center, semi_major, semi_minor, linear_eccentricity)
    pub fn detect_ellipse(&self) -> (bool, (u32, u32), u32, u32, u32) {
        // find semi-major and semi-minor axi
        let mut lengths = [(0, 0.0, [(0.0_f32, 0.0); 2]); 72];
        let mut circle_points = Vec::new();
        // for i_rotation in 0..72 {
        for (i_rotation, length_set) in lengths.iter_mut().enumerate() {
            let rotation = i_rotation as f32 * 5.0;
            let (length, points) = self.find_width_at_degree(rotation);
            circle_points.extend_from_slice(&points);
            *length_set = (length, rotation, points);
        }
        lengths.sort_by_key(|e| e.0);
        let major_axis = lengths.last().unwrap();
        let minor_axis = lengths.first().unwrap();

        // // find foci
        let linear_eccentricity = ((major_axis.0 / 2).pow(2) - (minor_axis.0 / 2).pow(2)).sqrt();

        if linear_eccentricity == 0 {
            // it's a circle afterall, and we're probably at the center of it
            (false, self.center, self.radius, self.radius, 0)
        } else {
            //it's an elipse, or we're off center, so we need to fix that problem
            let mut good_points = 0;
            let mut bad_points = 0;
            let mut found_all_on_ellipse = true;
            for point in &circle_points {
                let check_result = Self::check_ellipse_point(
                    self.center,
                    point,
                    major_axis.0 / 2,
                    minor_axis.0 / 2,
                );
                if check_result > 1.0 {
                    // a point is off the ellipse
                    bad_points += 1;
                    found_all_on_ellipse = false;
                    // break;
                } else {
                    good_points += 1;
                }
            }
            if !found_all_on_ellipse
                && (good_points as f32 / (good_points + bad_points) as f32) < 0.8
            {
                // probably a circle that we wrongly accused of being an ellipse,
                // try to find the center of that circle given three points on circumference
                let [point_1, point_2] = self.find_width_at_degree(0.0).1;
                let point_3 = self.find_width_at_degree(90.0).1[0];
                let guessed_center_point = Self::find_center(point_1, point_2, point_3);
                (
                    false,
                    (guessed_center_point.0 as u32, guessed_center_point.1 as u32),
                    self.radius,
                    self.radius,
                    0,
                )
            } else {
                // this is a real ellipse

                // find ellipse center
                let [point_1, point_2] = self.find_width_at_degree(0.0).1;
                let point_3 = self.find_width_at_degree(90.0).1[0];
                let ellipse_center = Self::calculate_ellipse_center(
                    major_axis.0 as f32,
                    minor_axis.0 as f32,
                    point_1,
                    point_2,
                    point_3,
                );
                (
                    true,
                    (ellipse_center.0 as u32, ellipse_center.1 as u32),
                    major_axis.0 / 2,
                    minor_axis.0 / 2,
                    linear_eccentricity,
                )
            }
        }
    }

    fn find_center(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> (f32, f32) {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        let (x3, y3) = p3;

        let a = x1 * (y2 - y3) - y1 * (x2 - x3) + (x2 * y3 - x3 * y2);
        let bx = (x1 * x1 + y1 * y1) * (y3 - y2)
            + (x2 * x2 + y2 * y2) * (y1 - y3)
            + (x3 * x3 + y3 * y3) * (y2 - y1);
        let by = (x1 * x1 + y1 * y1) * (x2 - x3)
            + (x2 * x2 + y2 * y2) * (x3 - x1)
            + (x3 * x3 + y3 * y3) * (x1 - x2);

        let x = bx / (2.0 * a);
        let y = by / (2.0 * a);

        (x.abs(), y.abs())
    }

    fn calculate_ellipse_center(
        a: f32,
        _b: f32,
        p1: (f32, f32),
        p2: (f32, f32),
        p3: (f32, f32),
    ) -> (f32, f32) {
        let x1 = p1.0;
        let y1 = p1.1;
        let x2 = p2.0;
        let y2 = p2.1;
        let x3 = p3.0;
        let y3 = p3.1;

        let ma = (x1 * x1 + y1 * y1 - a * a) / 2.0;
        let mb = (x2 * x2 + y2 * y2 - a * a) / 2.0;
        let mc = (x3 * x3 + y3 * y3 - a * a) / 2.0;

        let determinant = (x1 * y2 + x2 * y3 + x3 * y1) - (y1 * x2 + y2 * x3 + y3 * x1);

        let x = (ma * y2 + mb * y3 + mc * y1) / determinant;
        let y = (x1 * mb + x2 * mc + x3 * ma) / determinant;

        (x, y)
    }

    fn check_ellipse_point(
        center: (u32, u32),
        point: &(f32, f32),
        semi_major_axis: u32,
        semi_minor_axis: u32,
    ) -> f64 {
        ((point.0 as f64 - center.0 as f64).powf(2.0) / (semi_major_axis as f64).powf(2.0))
            + ((point.1 as f64 - center.1 as f64).powf(2.0) / (semi_minor_axis as f64).powf(2.0))
    }

    fn find_width_at_degree(&self, rotation: f32) -> (u32, [(f32, f32); 2]) {
        let mut x = self.center.0;
        let y = self.center.1;
        let mut length = 0;

        // count left
        while {
            let point = get_point(self.center, (x, y), rotation);
            !self.image.get(point.0 as u32, point.1 as u32) && x > 0
        } {
            x -= 1;
            length += 1;
        }

        let x_left = x;
        x = self.center.0 + 1;

        // count right
        while {
            let point = get_point(self.center, (x, y), rotation);
            !self.image.get(point.0 as u32, point.1 as u32)
        } {
            x += 1;
            length += 1;
        }

        (
            length,
            [
                get_point(self.center, (x_left, y), rotation),
                get_point(self.center, (x, y), rotation),
            ],
        )
    }
}

pub fn detect(image: &BitMatrix, try_harder: bool) -> Result<MaxicodeDetectionResult, Exceptions> {
    // find concentric circles
    let Some( mut circles) = find_concentric_circles(image) else {
        return Err(Exceptions::NotFoundException(None));
    };

    // we should have an idea where the center is at this point,
    // so we should be able to remove points that are widly far
    // from what we have otherwise found.
    let center_point_std_dev = Circle::calculate_center_point_std_dev(&circles);
    circles.retain(|c| {
        (c.center.0 as i32 - center_point_std_dev.1 .0 as i32).unsigned_abs()
            <= center_point_std_dev.0 .0
            && (c.center.1 as i32 - center_point_std_dev.1 .1 as i32).unsigned_abs()
                <= center_point_std_dev.0 .1
    });

    // Sort the points based on variance
    circles.sort_by(compare_circle);

    for circle in circles.iter_mut() {
        // build a box around this circle, trying to find the barcode
        let Ok(symbol_box) = box_symbol(image, circle) else {
            if try_harder {
                continue;
            }else {
                return Err(Exceptions::NotFoundException(None))
            }
        };
        let grid_sampler = DefaultGridSampler::default();

        let [tl, bl, tr, br] = &symbol_box;

        let target_width = MathUtils::distance_float(tl.0, tl.1, tr.0, tr.1);
        let target_height = MathUtils::distance_float(br.0, br.1, tr.0, tr.1);

        // let target_width = (tr.0 - tl.0).round().abs() as u32;
        // let target_height = (br.1 - tr.1).round().abs() as u32;

        let Ok(bits) = grid_sampler.sample_grid_detailed(
            image,
            target_width.round() as u32,
            target_height.round() as u32,
            0.0,
            0.0,
            target_width ,
            0.0,
            target_width,
            target_height,
            0.0,
            target_height,
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
            bits,
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
            if let Some((center, radius, horizontal_buckets)) =
                find_next_bullseye_horizontal(image, row, current_column)
            {
                // check that the bullseye is not just a figment of our one-dimensional imagination
                let (target_good, vertical_buckets) = verify_bullseye_vertical(image, row, center);
                if target_good {
                    // found a bullseye!

                    // add it
                    bullseyes.push(Circle {
                        center: (center, row),
                        radius,
                        horizontal_buckets,
                        vertical_buckets,
                        image,
                    });

                    // update the search to the next possible location
                    current_column = center + radius;
                    continue;
                } else {
                    // false alarm, go on with the row
                    let new_column = center - radius + (radius / 4);
                    if new_column == current_column {
                        // this is necessary because sometimes the loop can get 
                        // stuck when the result always comes out the same.
                        row += ROW_SCAN_SKIP;
                        break;
                    }
                    current_column = new_column;
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
) -> Option<(u32, u32, [u32; 11])> {
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
                if validate_bullseye_widths(&buckets) && last_color == Color::White {
                    // bullseye widths look good, this is a bullseye
                    return Some(get_bullseye_metadata(&buckets, column));
                } else {
                    // false alarm, this pattern doesn't look enough like it's evenly distributed
                    // move on to the next set.
                    pointer -= 1;
                    buckets.copy_within(1.., 0);
                    buckets[10] = 0;
                }
            }
        }

        buckets[pointer] += 1;

        column += 1;
    }

    None
}

/// look up and down from the provided column to verify that a possible bullseye exists
fn verify_bullseye_vertical(image: &BitMatrix, row: u32, column: u32) -> (bool, [u32; 11]) {
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

    if validate_bullseye_widths(&potential_bullseye) {
        (
            validate_bullseye_widths(&potential_bullseye),
            potential_bullseye,
        )
    } else {
        // try to nudge one in either direction and try again
        // look up
        let up_vector = get_column_vector(image, column + 1, row, true);

        // look down
        let down_vector = get_column_vector(image, column + 1, row, false);

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

        if validate_bullseye_widths(&potential_bullseye) {
            (
                validate_bullseye_widths(&potential_bullseye),
                potential_bullseye,
            )
        } else {
            // look up
            let up_vector = get_column_vector(image, column - 1, row, true);

            // look down
            let down_vector = get_column_vector(image, column - 1, row, false);

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

            if validate_bullseye_widths(&potential_bullseye) {
                (
                    validate_bullseye_widths(&potential_bullseye),
                    potential_bullseye,
                )
            } else {
                (false, potential_bullseye)
            }
        }
    }
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
    let total_width_even = buckets
        .iter()
        .enumerate()
        .filter_map(|e| {
            if e.0 != 5 && (e.0 == 0 || e.0 % 2 == 0) {
                Some(*e.1)
            } else {
                None
            }
        })
        .sum::<u32>() as f32;
    let total_width_odd = buckets
        .iter()
        .enumerate()
        .filter_map(|e| {
            if e.0 != 5 && (e.0 != 0 && e.0 % 2 != 0) {
                Some(*e.1)
            } else {
                None
            }
        })
        .sum::<u32>() as f32;

    let estimated_module_size_even = total_width_even / 5.0;
    let estimated_module_size_odd = total_width_odd / 5.0;

    let max_variance_even = estimated_module_size_even / 2.0;
    let max_variance_odd = estimated_module_size_odd / 2.0;

    let b1 = (estimated_module_size_even - buckets[0] as f32).abs();
    let b2 = (estimated_module_size_odd - buckets[1] as f32).abs();
    let b3 = (estimated_module_size_even - buckets[2] as f32).abs();
    let b4 = (estimated_module_size_odd - buckets[3] as f32).abs();
    let b5 = (estimated_module_size_even - buckets[4] as f32).abs();
    // let b6 = (estimated_module_size - buckets[5] as f32).abs();
    let b7 = (estimated_module_size_even - buckets[6] as f32).abs();
    let b8 = (estimated_module_size_odd - buckets[7] as f32).abs();
    let b9 = (estimated_module_size_even - buckets[8] as f32).abs();
    let b10 = (estimated_module_size_odd - buckets[9] as f32).abs();
    let b11 = (estimated_module_size_even - buckets[10] as f32).abs();

    b1 < max_variance_even
        && b2 < max_variance_odd
        && b3 < max_variance_even
        && b4 < max_variance_odd
        && b5 < max_variance_even
        // && b6 < max_variance * 2.0
        && b7 < max_variance_even
        && b8 < max_variance_odd
        && b9 < max_variance_even
        && b10 < max_variance_odd
        && b11 < max_variance_even
}

/// returns the (center , radius) of the possible bullseye
fn get_bullseye_metadata(buckets: &[u32; 11], column: u32) -> (u32, u32, [u32; 11]) {
    let radius = ((buckets.iter().sum::<u32>() as f32) / 2.0).round() as u32;
    let center = column - radius;
    (center, radius, *buckets)
}

const LEFT_SHIFT_PERCENT_ADJUST: f32 = 0.03;
const RIGHT_SHIFT_PERCENT_ADJUST: f32 = 0.03;
const ACCEPTED_SCALES: [f64; 5] = [0.065, 0.069, 0.07, 0.075, 0.08];

fn box_symbol(image: &BitMatrix, circle: &mut Circle) -> Result<[(f32, f32); 4], Exceptions> {
    let (left_boundary, right_boundary, top_boundary, bottom_boundary) =
        calculate_simple_boundary(circle, Some(image), None, false);

    let naive_box = [
        RXingResultPoint::new(left_boundary as f32, bottom_boundary as f32),
        RXingResultPoint::new(left_boundary as f32, top_boundary as f32),
        RXingResultPoint::new(right_boundary as f32, bottom_boundary as f32),
        RXingResultPoint::new(right_boundary as f32, top_boundary as f32),
    ];

    #[allow(unused_mut)]
    let mut result_box = naive_box;

    // check and see if we're dealing with an ellipse
    #[cfg(feature = "experimental_features")]
    let (is_ellipse, _, _, _, _) = circle.detect_ellipse();
    #[cfg(feature = "experimental_features")]
    if is_ellipse {
        // we don't deal with ellipses yet
        return Err(Exceptions::NotFoundException(None));
    }

    for scale in ACCEPTED_SCALES {
        if let Some(found_rotation) = attempt_rotation_box(image, circle, &naive_box, scale) {
            result_box = found_rotation;
            break;
        }
    }

    Ok([
        (result_box[0].x, result_box[0].y),
        (result_box[1].x, result_box[1].y),
        (result_box[2].x, result_box[2].y),
        (result_box[3].x, result_box[3].y),
    ])
}

fn calculate_simple_boundary(
    circle: &Circle,
    image: Option<&BitMatrix>,
    center_scale: Option<f64>,
    tight: bool,
) -> (u32, u32, u32, u32) {
    let (symbol_width, symbol_height) = if !tight {
        guess_barcode_size(circle)
    } else if let Some(s) = center_scale {
        guess_barcode_size_general(circle, 0.05, s, 0.95)
    } else {
        guess_barcode_size_tighter(circle)
    };

    let (image_width, image_height) = if let Some(i) = image {
        (i.getWidth(), i.getHeight())
    } else {
        (symbol_width, symbol_height)
    };

    let up_down_shift = symbol_height as i32 / 2;

    let left_shift =
        ((symbol_width as f32 / 2.0) - (symbol_width as f32 * LEFT_SHIFT_PERCENT_ADJUST)) as i32;
    let right_shift =
        ((symbol_width as f32 / 2.0) + (symbol_width as f32 * RIGHT_SHIFT_PERCENT_ADJUST)) as i32;

    let left_boundary =
        (circle.center.0 as i32 - left_shift).clamp(0, image_width as i32 - 33) as u32;
    let right_boundary =
        (circle.center.0 as i32 + right_shift).clamp(33, image_width as i32) as u32;
    let top_boundary =
        (circle.center.1 as i32 + up_down_shift).clamp(33, image_height as i32) as u32;
    let bottom_boundary =
        (circle.center.1 as i32 - up_down_shift).clamp(0, image_height as i32 - 30) as u32;

    (left_boundary, right_boundary, top_boundary, bottom_boundary)
}

const TOP_LEFT_ORIENTATION_POS: ((u32, u32), (u32, u32), (u32, u32)) = ((10, 9), (11, 9), (11, 10));
const TOP_RIGHT_ORIENTATION_POS: ((u32, u32), (u32, u32), (u32, u32)) =
    ((17, 9), (17, 10), (18, 10));
const LEFT_ORIENTATION_POS: ((u32, u32), (u32, u32), (u32, u32)) = ((7, 15), (7, 16), (8, 16));
const RIGHT_ORIENTATION_POS: ((u32, u32), (u32, u32), (u32, u32)) = ((20, 16), (21, 16), (20, 17));
const BOTTOM_LEFT_ORIENTATION_POS: ((u32, u32), (u32, u32), (u32, u32)) =
    ((10, 22), (11, 22), (10, 23));
const BOTTOM_RIGHT_ORIENTATION_POS: ((u32, u32), (u32, u32), (u32, u32)) =
    ((17, 22), (16, 23), (17, 23));

fn attempt_rotation_box(
    image: &BitMatrix,
    circle: &mut Circle,
    naive_box: &[RXingResultPoint; 4],
    center_scale: f64,
) -> Option<[RXingResultPoint; 4]> {
    // update our circle with a more accurate center point
    circle.calculate_high_accuracy_center();

    // we know that the locator symbols should appear at 60 degree increments around the circle

    // top left
    let (topl_p1, topl_p2, topl_p3) =
        get_adjusted_points(TOP_LEFT_ORIENTATION_POS, circle, center_scale);

    // top right
    let (topr_p1, topr_p2, topr_p3) =
        get_adjusted_points(TOP_RIGHT_ORIENTATION_POS, circle, center_scale);

    // left
    let (l_p1, l_p2, l_p3) = get_adjusted_points(LEFT_ORIENTATION_POS, circle, center_scale);

    // right
    let (r_p1, r_p2, r_p3) = get_adjusted_points(RIGHT_ORIENTATION_POS, circle, center_scale);

    // bottom left
    let (bottoml_p1, bottoml_p2, bottoml_p3) =
        get_adjusted_points(BOTTOM_LEFT_ORIENTATION_POS, circle, center_scale);

    // bottom right
    let (bottomr_p1, bottomr_p2, bottomr_p3) =
        get_adjusted_points(BOTTOM_RIGHT_ORIENTATION_POS, circle, center_scale);

    let mut found = false;
    let mut final_rotation = 0.0;

    for int_rotation in 0..175 {
        let rotation = (int_rotation * 2) as f32;
        // look for top left
        //  * *
        //   *
        let p1_rot = get_point(circle.center, topl_p1, rotation);
        let p2_rot = get_point(circle.center, topl_p2, rotation);
        let p3_rot = get_point(circle.center, topl_p3, rotation);
        let found_tl = image.try_get_area(p1_rot.0 as u32, p1_rot.1 as u32, 3)?
            && image.try_get_area(p2_rot.0 as u32, p2_rot.1 as u32, 3)?
            && image.try_get_area(p3_rot.0 as u32, p3_rot.1 as u32, 3)?;
        if !found_tl {
            continue;
        }

        // look for top right
        //  /\
        //  __
        let p1_rot = get_point(circle.center, topr_p1, rotation);
        let p2_rot = get_point(circle.center, topr_p2, rotation);
        let p3_rot = get_point(circle.center, topr_p3, rotation);
        let found_tr = !image.try_get_area(p1_rot.0 as u32, p1_rot.1 as u32, 3)?
            && !image.try_get_area(p2_rot.0 as u32, p2_rot.1 as u32, 3)?
            && !image.try_get_area(p3_rot.0 as u32, p3_rot.1 as u32, 3)?;
        if !found_tr {
            continue;
        }

        // look for left
        //   *
        //    *
        let p1_rot = get_point(circle.center, l_p1, rotation);
        let p2_rot = get_point(circle.center, l_p2, rotation);
        let p3_rot = get_point(circle.center, l_p3, rotation);
        let found_l = image.try_get_area(p1_rot.0 as u32, p1_rot.1 as u32, 3)?
            && !image.try_get_area(p2_rot.0 as u32, p2_rot.1 as u32, 3)?
            && image.try_get_area(p3_rot.0 as u32, p3_rot.1 as u32, 3)?;
        if !found_l {
            continue;
        }

        // look for right
        //   *
        //    *
        let p1_rot = get_point(circle.center, r_p1, rotation);
        let p2_rot = get_point(circle.center, r_p2, rotation);
        let p3_rot = get_point(circle.center, r_p3, rotation);
        let found_r = image.try_get_area(p1_rot.0 as u32, p1_rot.1 as u32, 3)?
            && !image.try_get_area(p2_rot.0 as u32, p2_rot.1 as u32, 3)?
            && image.try_get_area(p3_rot.0 as u32, p3_rot.1 as u32, 3)?;
        if !found_r {
            continue;
        }

        // look for bottom left
        //   *
        //    *
        let p1_rot = get_point(circle.center, bottoml_p1, rotation);
        let p2_rot = get_point(circle.center, bottoml_p2, rotation);
        let p3_rot = get_point(circle.center, bottoml_p3, rotation);
        let found_bl = image.try_get_area(p1_rot.0 as u32, p1_rot.1 as u32, 3)?
            && !image.try_get_area(p2_rot.0 as u32, p2_rot.1 as u32, 3)?
            && image.try_get_area(p3_rot.0 as u32, p3_rot.1 as u32, 3)?;
        if !found_bl {
            continue;
        }

        // look for bottom right
        //   *
        //    *
        let p1_rot = get_point(circle.center, bottomr_p1, rotation);
        let p2_rot = get_point(circle.center, bottomr_p2, rotation);
        let p3_rot = get_point(circle.center, bottomr_p3, rotation);
        let found_br = image.try_get_area(p1_rot.0 as u32, p1_rot.1 as u32, 3)?
            && !image.try_get_area(p2_rot.0 as u32, p2_rot.1 as u32, 3)?
            && image.try_get_area(p3_rot.0 as u32, p3_rot.1 as u32, 3)?;
        if !found_br {
            continue;
        }

        // did we find it?
        found = found_tl && found_tr && found_l && found_r && found_bl && found_br;

        if found {
            final_rotation = rotation;
            break;
        }
    }

    if found {
        // if final_rotation > 180.0 { final_rotation = final_rotation + 0.0 }

        let new_1 = get_point(
            circle.center,
            (naive_box[0].x as u32, naive_box[0].y as u32),
            final_rotation,
        );
        let new_2 = get_point(
            circle.center,
            (naive_box[1].x as u32, naive_box[1].y as u32),
            final_rotation,
        );
        let new_3 = get_point(
            circle.center,
            (naive_box[2].x as u32, naive_box[2].y as u32),
            final_rotation,
        );
        let new_4 = get_point(
            circle.center,
            (naive_box[3].x as u32, naive_box[3].y as u32),
            final_rotation,
        );

        Some([
            RXingResultPoint::new(new_1.0, new_1.1),
            RXingResultPoint::new(new_2.0, new_2.1),
            RXingResultPoint::new(new_3.0, new_3.1),
            RXingResultPoint::new(new_4.0, new_4.1),
        ])
    } else {
        // panic!("couldn't find");
        None
    }
}

fn get_adjusted_points(
    origin: ((u32, u32), (u32, u32), (u32, u32)),
    circle: &Circle,
    center_scale: f64,
) -> ((u32, u32), (u32, u32), (u32, u32)) {
    (
        adjust_point_alternate(origin.0, circle, center_scale),
        adjust_point_alternate(origin.1, circle, center_scale),
        adjust_point_alternate(origin.2, circle, center_scale),
    )
}

fn get_point(center: (u32, u32), original: (u32, u32), angle: f32) -> (f32, f32) {
    let radians = angle.to_radians();
    let x = radians.cos() * (original.0 as f32 - center.0 as f32)
        - radians.sin() * (original.1 as f32 - center.1 as f32)
        + center.0 as f32;
    let y = radians.sin() * (original.0 as f32 - center.0 as f32)
        + radians.cos() * (original.1 as f32 - center.1 as f32)
        + center.1 as f32;

    (x.abs(), y.abs())
}

fn adjust_point_alternate(point: (u32, u32), circle: &Circle, center_scale: f64) -> (u32, u32) {
    let (left_boundary, right_boundary, top_boundary, bottom_boundary) =
        calculate_simple_boundary(circle, Some(circle.image), Some(center_scale), true);

    let top = bottom_boundary;
    let height = top_boundary - bottom_boundary;
    let width = right_boundary - left_boundary;
    let left = left_boundary;

    let y = point.1;
    let x = point.0;

    let iy = (top + (y * height + height / 2) / MaxiCodeReader::MATRIX_HEIGHT).min(height - 1);
    let ix = left
        + ((x * width + width / 2 + (y & 0x01) * width / 2) / MaxiCodeReader::MATRIX_WIDTH)
            .min(width - 1);

    (ix, iy)
}

/// calculate a likely size for the barcode.
/// returns (width, height)
fn guess_barcode_size(circle: &Circle) -> (u32, u32) {
    guess_barcode_size_general(circle, 0.03, 0.066, 1.0)
}

fn guess_barcode_size_tighter(circle: &Circle) -> (u32, u32) {
    guess_barcode_size_general(circle, 0.025, 0.0695, 0.97)
}

fn guess_barcode_size_general(
    circle: &Circle,
    height_adjust_percent: f64,
    circle_area_percent: f64,
    height_final_adjust_percent: f64,
) -> (u32, u32) {
    let circle_area = std::f64::consts::PI * circle.radius.pow(2) as f64;
    let ideal_symbol_area = (circle_area / circle_area_percent) / (1.0 - height_adjust_percent);
    let ideal_symbol_side = ideal_symbol_area.sqrt();

    (
        ideal_symbol_side.floor() as u32,
        (ideal_symbol_side * height_final_adjust_percent).floor() as u32,
    )
}

/// compare two circles to determine which has a better variance.
fn compare_circle(a: &Circle, b: &Circle) -> std::cmp::Ordering {
    let a_var = a.calculate_circle_variance();
    let b_var = b.calculate_circle_variance();

    a_var.partial_cmp(&b_var).unwrap()
}

/// Read appropriate bits from a bitmatrix for the maxicode decoder
pub fn read_bits(image: &BitMatrix) -> Result<BitMatrix, Exceptions> {
    let enclosingRectangle = image.getEnclosingRectangle().unwrap();

    let left = enclosingRectangle[0];
    let top = enclosingRectangle[1];
    let width = enclosingRectangle[2];
    let height = enclosingRectangle[3];

    // Now just read off the bits
    let mut bits = BitMatrix::new(MaxiCodeReader::MATRIX_WIDTH, MaxiCodeReader::MATRIX_HEIGHT)?;
    for y in 0..MaxiCodeReader::MATRIX_HEIGHT {
        // for (int y = 0; y < MATRIX_HEIGHT; y++) {
        let iy = (top + (y * height + height / 2) / MaxiCodeReader::MATRIX_HEIGHT).min(height - 1);
        for x in 0..MaxiCodeReader::MATRIX_WIDTH {
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

#[cfg(feature = "image")]
#[cfg(test)]
mod detector_test {
    use std::io::Read;

    use crate::{
        common::{DetectorRXingResult, HybridBinarizer},
        maxicode::detector::read_bits,
        Binarizer, BufferedImageLuminanceSource,
    };

    #[test]
    fn mode_1() {
        finder_test(
            "test_resources/blackbox/maxicode-1/1.png",
            "test_resources/blackbox/maxicode-1/1.txt",
        )
    }

    #[test]
    fn mode_2() {
        finder_test(
            "test_resources/blackbox/maxicode-1/MODE2.png",
            "test_resources/blackbox/maxicode-1/MODE2.txt",
        )
    }

    #[test]
    fn mode_2_rot90() {
        finder_test(
            "test_resources/blackbox/maxicode-1/MODE2-rotate-90.png",
            "test_resources/blackbox/maxicode-1/MODE2-rotate-90.txt",
        )
    }

    #[test]
    fn mode3() {
        finder_test(
            "test_resources/blackbox/maxicode-1/MODE3.png",
            "test_resources/blackbox/maxicode-1/MODE3.txt",
        )
    }

    #[test]
    fn mixed_sets() {
        finder_test(
            "test_resources/blackbox/maxicode-1/mode4-mixed-sets.png",
            "test_resources/blackbox/maxicode-1/mode4-mixed-sets.txt",
        )
    }

    #[test]
    fn mode4() {
        finder_test(
            "test_resources/blackbox/maxicode-1/MODE4.png",
            "test_resources/blackbox/maxicode-1/MODE4.txt",
        )
    }

    #[test]
    fn mode5() {
        finder_test(
            "test_resources/blackbox/maxicode-1/MODE5.png",
            "test_resources/blackbox/maxicode-1/MODE5.txt",
        )
    }

    #[test]
    fn mode6() {
        finder_test(
            "test_resources/blackbox/maxicode-1/MODE6.png",
            "test_resources/blackbox/maxicode-1/MODE6.txt",
        )
    }

    fn finder_test(image: &str, data: &str) {
        let filename = image;
        let img = image::open(filename).unwrap();
        let lum_src = BufferedImageLuminanceSource::new(img);
        let binarizer = HybridBinarizer::new(Box::new(lum_src));
        let bitmatrix = binarizer.getBlackMatrix().unwrap();

        // let i: image::DynamicImage = bitmatrix.into();
        // i.save("dbgfle.png").expect("should write image");

        let mut expected_result = String::new();

        std::fs::File::open(data)
            .unwrap()
            .read_to_string(&mut expected_result)
            .unwrap();

        let detection = super::detect(bitmatrix, true).unwrap();

        // let i: image::DynamicImage = detection.getBits().into();
        // i.save("dbgfle-transformed.png")
        //     .expect("should write image");

        let bits = read_bits(detection.getBits()).expect("read bits");

        // std::fs::File::create("dbgfle-read").unwrap().write_all(bits.to_string().as_bytes()).expect("write");

        let result = crate::maxicode::decoder::decode(&bits).expect("must decode");

        assert_eq!(expected_result, result.getText());
    }
}
