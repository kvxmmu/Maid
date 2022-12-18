use std::hint::black_box;

use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
};
use maid_codec::arm64::decoder::BufferedDecoder;

fn decode_all(data: &[u8]) {
    let mut decoder = BufferedDecoder::new(black_box(data));
    while let Ok(_) = decoder.decode_next() {}
}

fn benchmark(c: &mut Criterion) {
    let data = &b"\xffC\x00\xd1@\x01\x80R"[..];
    c.bench_function("decode sub & mov", |b| {
        b.iter(|| {
            decode_all(data);
        });
    });
}

criterion_group!(simple_benches, benchmark);
criterion_main!(simple_benches);
