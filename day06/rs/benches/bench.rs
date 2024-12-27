use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(day06::part_1));

    let mut group = c.benchmark_group("part 2");
    group.bench_function("sync", |b| {
        b.iter(|| day06::solve_2_sync(include_str!("../../input")));
    });
    group.bench_function("par", |b| {
        b.iter(|| day06::solve_2_par(include_str!("../../input")));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
