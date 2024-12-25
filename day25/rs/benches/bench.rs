use criterion::{criterion_group, criterion_main, Criterion};

use day25::{solve_1, INPUT};

#[cfg(feature = "simd")]
use day25::solve_1_simd;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part 1");

    group.bench_function("normal", |b| b.iter(|| solve_1(INPUT)));

    #[cfg(feature = "simd")]
    {
        group.bench_function("simd/2", |b| b.iter(|| solve_1_simd::<2>(INPUT)));
        group.bench_function("simd/4", |b| b.iter(|| solve_1_simd::<4>(INPUT)));
        group.bench_function("simd/8", |b| b.iter(|| solve_1_simd::<8>(INPUT)));
        group.bench_function("simd/16", |b| b.iter(|| solve_1_simd::<16>(INPUT)));
        group.bench_function("simd/32", |b| b.iter(|| solve_1_simd::<32>(INPUT)));
        group.bench_function("simd/64", |b| b.iter(|| solve_1_simd::<64>(INPUT)));
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
