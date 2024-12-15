use criterion::{criterion_group, criterion_main, Criterion};

use day15::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(part_1));

    let mut group = c.benchmark_group("part 2");
    group.bench_function("rec", |b| b.iter(|| solve_2_rec(&INPUT)));
    group.bench_function("bfs", |b| b.iter(|| solve_2_bfs(&INPUT)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
