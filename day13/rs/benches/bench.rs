use criterion::{criterion_group, criterion_main, Criterion};

use day13::*;

macro_rules! bench_machine0 {
    ($group:expr, $($m:ident => $t:ty),*) => {
        $(
            struct $m;
            impl Machine<$t> for $m {
                const CORRECTION: $t = 0;
            }

            $group.bench_function(stringify!($t), |b| b.iter(|| solve::<$m, _>(&INPUT)));
        )*
    };
}

struct Machine1BBL;
impl Machine<i128> for Machine1BBL {
    const CORRECTION: i128 = 10000000000000;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part 1");
    group.bench_function("i32", |b| b.iter(|| solve::<Machine0, _>(&INPUT)));
    bench_machine0!(
        group,
        Machine0i64 => i64,
        Machine0i128 => i128
    );
    group.finish();

    let mut group = c.benchmark_group("part 2");
    group.bench_function("i64", |b| b.iter(|| solve::<Machine1BB, _>(&INPUT)));
    group.bench_function("i128", |b| b.iter(|| solve::<Machine1BBL, _>(&INPUT)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
