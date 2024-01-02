use criterion::{criterion_group, criterion_main, Criterion};
use rxing::aztec::AztecReader;
use rxing::common::HybridBinarizer;
use rxing::datamatrix::DataMatrixReader;
use rxing::maxicode::MaxiCodeReader;
use rxing::multi::{GenericMultipleBarcodeReader, MultipleBarcodeReader};
use rxing::oned::rss::expanded::RSSExpandedReader;
use rxing::oned::rss::RSS14Reader;
use rxing::oned::{
    CodaBarReader, Code39Reader, Code93Reader, EAN13Reader, EAN8Reader, ITFReader, TelepenReader,
    UPCAReader, UPCEReader,
};
use rxing::pdf417::PDF417Reader;
use rxing::qrcode::QRCodeReader;
use rxing::MultiFormatReader;
use rxing::{BinaryBitmap, BufferedImageLuminanceSource, Reader};
use std::path::Path;

fn get_image(
    path: impl AsRef<Path>,
) -> BinaryBitmap<HybridBinarizer<BufferedImageLuminanceSource>> {
    BinaryBitmap::new(HybridBinarizer::new(BufferedImageLuminanceSource::new(
        image::open(path).unwrap(),
    )))
}

fn aztec_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/aztec-1/abc-37x37.png");
    let mut reader = AztecReader;

    c.bench_function("aztec", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn codabar_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/codabar-1/02.png");
    let mut reader = CodaBarReader::default();

    c.bench_function("codabar", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn code39_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/code39-1/1.png");
    let mut reader = Code39Reader::default();

    c.bench_function("code39", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn code93_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/code93-1/1.png");
    let mut reader = Code93Reader::default();

    c.bench_function("code93", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn datamatrix_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/datamatrix-1/C40.png");
    let mut reader = DataMatrixReader;

    c.bench_function("datamatrix", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn ean8_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/ean8-1/1.png");
    let mut reader = EAN8Reader;

    c.bench_function("ean8", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn ean13_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/ean13-1/1.png");
    let mut reader = EAN13Reader;

    c.bench_function("ean13", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn itf_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/itf-1/1.png");
    let mut reader = ITFReader::default();

    c.bench_function("itf", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn maxicode_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/maxicode-1/1.png");
    let mut reader = MaxiCodeReader::default();

    c.bench_function("maxicode", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn pdf417_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/pdf417-1/01.png");
    let mut reader = PDF417Reader;

    c.bench_function("pdf417", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn qrcode_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/qrcode-2/1.png");
    let mut reader = QRCodeReader;

    c.bench_function("qrcode", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn rss14_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/rss14-1/3.png");
    c.bench_function("rss14", |b| {
        b.iter(|| {
            let mut reader = RSS14Reader::default();
            let _res = reader.decode(&mut image);
        });
    });
}

fn rss_expanded_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/rssexpanded-1/1.png");
    c.bench_function("rss_expanded", |b| {
        b.iter(|| {
            let mut reader = RSSExpandedReader::default();
            let _res = reader.decode(&mut image);
        });
    });
}

fn telepen_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/telepen-1/02.png");
    let mut reader = TelepenReader::default();

    c.bench_function("telepen", |b| {
        b.iter(|| {
            let _res = reader.decode(&mut image);
        });
    });
}

fn upca_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/upca-4/1.png");
    c.bench_function("upca", |b| {
        b.iter(|| {
            let mut reader = UPCAReader::default();
            let _res = reader.decode(&mut image);
        });
    });
}

fn upce_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/upce-1/1.png");
    c.bench_function("upce", |b| {
        b.iter(|| {
            let mut reader = UPCEReader;
            let _res = reader.decode(&mut image);
        });
    });
}

fn multi_barcode_benchmark(c: &mut Criterion) {
    let mut image = get_image("test_resources/blackbox/multi-1/1.png");
    c.bench_function("multi_barcode", |b| {
        b.iter(|| {
            let mut reader = GenericMultipleBarcodeReader::new(MultiFormatReader::default());
            let _res = reader.decode_multiple(&mut image);
        });
    });
}

criterion_group!(
    benches,
    aztec_benchmark,
    codabar_benchmark,
    code39_benchmark,
    code93_benchmark,
    datamatrix_benchmark,
    ean8_benchmark,
    ean13_benchmark,
    itf_benchmark,
    maxicode_benchmark,
    pdf417_benchmark,
    qrcode_benchmark,
    rss14_benchmark,
    rss_expanded_benchmark,
    telepen_benchmark,
    upca_benchmark,
    upce_benchmark,
    multi_barcode_benchmark
);
criterion_main!(benches);
