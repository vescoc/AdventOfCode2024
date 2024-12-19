use criterion::{criterion_group, criterion_main, Criterion};

use day18::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(part_1));

    let mut group = c.benchmark_group("part 2 1024");
    group.bench_function("bs/bfs", |b| {
        b.iter(|| {
            solve_2_bs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE, 1024>(
                &INPUT,
                bfs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE>,
            )
        })
    });
    group.bench_function("bs/dfs", |b| {
        b.iter(|| {
            solve_2_bs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE, 1024>(
                &INPUT,
                dfs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE>,
            )
        })
    });
    group.finish();

    let mut group = c.benchmark_group("part 2 0");
    group.bench_function("bs/bfs", |b| {
        b.iter(|| {
            solve_2_bs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE, 0>(
                &INPUT,
                bfs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE>,
            )
        })
    });
    group.bench_function("bs/dfs", |b| {
        b.iter(|| {
            solve_2_bs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE, 0>(
                &INPUT,
                dfs::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE>,
            )
        })
    });
    group.bench_function("bru", |b| {
        b.iter(|| solve_2_bru::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE>(&INPUT))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
