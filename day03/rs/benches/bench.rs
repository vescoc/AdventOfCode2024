use criterion::{criterion_group, criterion_main, Criterion};

use day03::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part 1");
    group.bench_function("handmade", |b| b.iter(|| solve_1_handmade(&day03::INPUT)));
    group.bench_function("nom", |b| b.iter(|| solve_1_nom(&day03::INPUT)));
    group.finish();

    let mut group = c.benchmark_group("part 2");
    group.bench_function("handmade", |b| b.iter(|| solve_2_handmade(&day03::INPUT)));
    group.bench_function("nom", |b| b.iter(|| solve_2_nom(&day03::INPUT)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
