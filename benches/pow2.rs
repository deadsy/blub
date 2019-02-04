#[macro_use]
extern crate criterion;

use criterion::Criterion;

use blub::lut::pow2;

fn std_pow2(x: f32) -> f32 {
    return 2.0_f32.powf(x);
}

fn pow2_benchmark(c: &mut Criterion) {
    c.bench_function("pow2_0", |b| b.iter(|| pow2(0.0)));
    c.bench_function("pow2_11", |b| b.iter(|| pow2(1.1)));
    c.bench_function("pow2_22", |b| b.iter(|| pow2(2.2)));
    c.bench_function("pow2_73", |b| b.iter(|| pow2(7.3)));

    c.bench_function("std_pow2_0", |b| b.iter(|| std_pow2(0.0)));
    c.bench_function("std_pow2_11", |b| b.iter(|| std_pow2(1.1)));
    c.bench_function("std_pow2_22", |b| b.iter(|| std_pow2(2.2)));
    c.bench_function("std_pow2_73", |b| b.iter(|| std_pow2(7.3)));
}

criterion_group!(benches, pow2_benchmark);
criterion_main!(benches);
