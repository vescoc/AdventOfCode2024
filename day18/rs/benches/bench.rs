use criterion::{criterion_group, criterion_main, Criterion};

use day18::*;

const INPUT_213X213: &str = include_str!("../../input-213x213");

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
    group.finish();
    
    c.bench_function("bru", |b| b.iter(|| solve_2_bru::<PUZZLE_WIDTH, PUZZLE_HEIGHT, BITSET_SIZE>(&INPUT)));

    {
        const BITSET_SIZE: usize = bitset::BitSet::with_capacity(213 * 213);
        
        let mut group = c.benchmark_group("part 2 213x213");
        group.bench_function("bru", |b| {
            b.iter(|| {
                solve_2_bru::<213, 213, BITSET_SIZE>(
                    &INPUT_213X213,
                )
            })
        });
        group.bench_function("bs/bfs", |b| {
            b.iter(|| {
                solve_2_bs::<213, 213, BITSET_SIZE, 0>(
                    &INPUT,
                    bfs::<213, 213, BITSET_SIZE>,
                )
            })
        });
        group.finish()
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
