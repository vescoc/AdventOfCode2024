use criterion::{criterion_group, criterion_main, Criterion};

use day19::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part 1");
    group.bench_function("r", |b| b.iter(|| solve_1_r(&INPUT)));
    group.bench_function("pd", |b| b.iter(|| solve_1_dp(&INPUT)));
    group.finish();

    let mut group = c.benchmark_group("part 2");
    group.bench_function("r", |b| b.iter(|| solve_2_r(&INPUT)));
    group.bench_function("pd", |b| b.iter(|| solve_2_dp(&INPUT)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
