use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(day15::part_1));

    let mut group = c.benchmark_group("part 2");
    group.bench_function("rec", |b| b.iter(|| day15::solve_2_rec(day15::INPUT)));
    group.bench_function("bfs", |b| b.iter(|| day15::solve_2_bfs(day15::INPUT)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
