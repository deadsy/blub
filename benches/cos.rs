#[macro_use]
extern crate criterion;

use criterion::Criterion;

use blub::lut::cos;

fn std_cos(x: f32) -> f32 {
    return x.cos();
}

fn cos_benchmark(c: &mut Criterion) {
    c.bench_function("cos_0", |b| b.iter(|| cos(0.0)));
    c.bench_function("cos_1", |b| b.iter(|| cos(1.0)));
    c.bench_function("cos_10", |b| b.iter(|| cos(10.0)));
    c.bench_function("cos_100", |b| b.iter(|| cos(100.0)));

    c.bench_function("std_cos_0", |b| b.iter(|| std_cos(0.0)));
    c.bench_function("std_cos_1", |b| b.iter(|| std_cos(1.0)));
    c.bench_function("std_cos_10", |b| b.iter(|| std_cos(10.0)));
    c.bench_function("std_cos_100", |b| b.iter(|| std_cos(100.0)));
}

criterion_group!(benches, cos_benchmark);
criterion_main!(benches);
