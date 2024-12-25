use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(day25::part_1));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
