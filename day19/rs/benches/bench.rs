use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part 1");
    group.bench_function("r", |b| b.iter(|| day19::solve_1_r(day19::INPUT)));
    group.bench_function("dp", |b| b.iter(|| day19::solve_1_dp(day19::INPUT)));
    group.finish();

    let mut group = c.benchmark_group("part 2");
    group.bench_function("r", |b| b.iter(|| day19::solve_2_r(day19::INPUT)));
    group.bench_function("dp", |b| b.iter(|| day19::solve_2_dp(day19::INPUT)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
