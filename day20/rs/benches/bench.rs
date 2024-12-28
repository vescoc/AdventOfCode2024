use criterion::{criterion_group, criterion_main, Criterion};

use day20::{solve_m, solve_v, INPUT};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part 1");
    group.bench_function("m", |b| b.iter(|| solve_m::<100, 2>(INPUT)));
    group.bench_function("v", |b| b.iter(|| solve_v::<100, 2>(INPUT)));
    group.finish();

    let mut group = c.benchmark_group("part 2");
    group.bench_function("m", |b| b.iter(|| solve_m::<100, 20>(INPUT)));
    group.bench_function("v", |b| b.iter(|| solve_v::<100, 20>(INPUT)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
