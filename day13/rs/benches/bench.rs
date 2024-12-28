use criterion::{criterion_group, criterion_main, Criterion};

macro_rules! bench_machine0 {
    ($group:expr, $($m:ident => $t:ty),*) => {
        $(
            struct $m;
            impl day13::Machine<$t> for $m {
                const CORRECTION: $t = 0;
            }

            $group.bench_function(stringify!($t), |b| b.iter(|| day13::solve::<$m, _>(day13::INPUT)));
        )*
    };
}

struct Machine1BBL;
impl day13::Machine<i128> for Machine1BBL {
    #[allow(clippy::unreadable_literal)]
    const CORRECTION: i128 = 10000000000000;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part 1");
    group.bench_function("i32", |b| b.iter(|| day13::solve::<day13::Machine0, _>(day13::INPUT)));
    bench_machine0!(
        group,
        Machine0i64 => i64,
        Machine0i128 => i128
    );

    #[cfg(feature = "simd")]
    {
        group.bench_function("i32 simd<4>", |b| b.iter(|| day13::simd::solve_1::<4>(day13::INPUT)));
        group.bench_function("i32 simd<8>", |b| b.iter(|| day13::simd::solve_1::<8>(day13::INPUT)));
        group.bench_function("i32 simd<16>", |b| b.iter(|| day13::simd::solve_1::<16>(day13::INPUT)));
        group.bench_function("i32 simd<32>", |b| b.iter(|| day13::simd::solve_1::<32>(day13::INPUT)));
        group.bench_function("i32 simd<64>", |b| b.iter(|| day13::simd::solve_1::<64>(day13::INPUT)));
    }

    group.finish();

    let mut group = c.benchmark_group("part 2");
    group.bench_function("i64", |b| b.iter(|| day13::solve::<day13::Machine1BB, _>(day13::INPUT)));
    group.bench_function("i128", |b| b.iter(|| day13::solve::<Machine1BBL, _>(day13::INPUT)));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
