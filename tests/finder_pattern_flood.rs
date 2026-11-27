#![cfg(all(feature = "decoders", feature = "qrcode"))]

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use rxing::{BarcodeFormat, DecodeHintValue, DecodeHints};

const SIZE: u32 = 1000;

fn finder_pattern_tiling(module: u32, period: u32) -> Vec<u8> {
    let mut pixels = vec![255u8; (SIZE * SIZE) as usize];
    for (index, pixel) in pixels.iter_mut().enumerate() {
        let (x, y) = (index as u32 % SIZE, index as u32 / SIZE);
        let (cx, cy) = ((x / module) % period, (y / module) % period);
        if cx < 7 && cy < 7 {
            let ring = cx == 0 || cx == 6 || cy == 0 || cy == 6;
            let core = (2..=4).contains(&cx) && (2..=4).contains(&cy);
            if ring || core {
                *pixel = 0;
            }
        }
    }
    pixels
}

// An image tiled with thousands of finder patterns must be rejected quickly instead of trying every candidate triple
#[test]
fn finder_pattern_flood() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let started = Instant::now();
        for (module, period) in [(2, 8), (2, 10), (3, 8), (3, 10), (4, 8)] {
            let pixels = finder_pattern_tiling(module, period);
            let mut hints = DecodeHints::default()
                .with(DecodeHintValue::TryHarder(true))
                .with(DecodeHintValue::AlsoInverted(true));
            let result = rxing::helpers::detect_in_luma_slice_with_hints(
                &pixels,
                SIZE,
                SIZE,
                Some(BarcodeFormat::QR_CODE),
                &mut hints,
            );
            assert!(result.is_err(), "{module}x{period}: {result:?}");
        }
        let _ = tx.send(started.elapsed());
    });

    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(elapsed) => println!("finder pattern flood decoded in {elapsed:?}"),
        Err(_) => panic!("finder pattern flood: decode timed out"),
    }
}
