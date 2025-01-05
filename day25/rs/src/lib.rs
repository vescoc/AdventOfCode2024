#![no_std]
#![allow(clippy::must_use_candidate)]
#![cfg_attr(feature = "simd", feature(portable_simd))]

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "simd")]
use core::simd::{prelude::*, LaneCount, SupportedLaneCount};

use heapless::{String as HLString, Vec as HLVec};

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

type Vec<T> = HLVec<T, 1024>;
type String = HLString<1>;

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_1(input: &str) -> usize {
    let (mut keys, mut locks) = const { (Vec::new(), Vec::new()) };
    for part in input.split("\n\n") {
        let list = if part.starts_with("#####") {
            &mut locks
        } else {
            &mut keys
        };

        let acc = part.as_bytes().chunks(6).skip(1).take(5).enumerate().fold(
            0u64,
            |mut acc, (j, row)| {
                for (i, &c) in row.iter().take(5).enumerate() {
                    if c == b'#' {
                        acc |= 1 << (5 * i + j);
                    }
                }
                acc
            },
        );

        list.push(acc).unwrap();
    }

    #[cfg(feature = "parallel")]
    let keys = keys.par_iter();

    #[cfg(not(feature = "parallel"))]
    let keys = keys.iter();

    keys.map(|&key| locks.iter().filter(|&lock| key & lock == 0).count())
        .sum()
}

/// # Panics
#[cfg(feature = "simd")]
#[cfg_attr(target_os = "none", inline(never))]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
pub fn solve_1_simd<const N: usize>(input: &str) -> usize
where
    LaneCount<N>: SupportedLaneCount,
{
    type Num = u32;

    #[repr(align(16))]
    struct Data {
        keys: Vec<Num>,
        locks: Vec<Num>,
    }

    let mut data = Data {
        keys: Vec::new(),
        locks: Vec::new(),
    };

    for part in input.split("\n\n") {
        let list = if part.starts_with("#####") {
            &mut data.locks
        } else {
            &mut data.keys
        };

        let acc =
            part.as_bytes()
                .chunks(6)
                .skip(1)
                .take(5)
                .enumerate()
                .fold(0, |mut acc, (j, row)| {
                    for (i, &c) in row.iter().take(5).enumerate() {
                        if c == b'#' {
                            acc |= 1 << (5 * i + j);
                        }
                    }
                    acc
                });

        list.push(acc).unwrap();
    }

    #[cfg(feature = "parallel")]
    let keys = data.keys.par_iter();

    #[cfg(not(feature = "parallel"))]
    let keys = data.keys.iter();

    keys.map(|&key| {
        let (locks_prefix, locks, locks_suffix) = data.locks.as_simd::<N>();

        let prefix = locks_prefix.iter().filter(|&lock| lock & key == 0).count();
        let suffix = locks_suffix.iter().filter(|&lock| lock & key == 0).count();

        let mut total = Simd::<i16, N>::splat(0);
        total[0] = prefix as i16;
        total[1] = suffix as i16;
        for lock in locks {
            let mask = (lock & Simd::splat(key))
                .simd_eq(Simd::splat(0))
                .cast::<i16>();
            total += mask.select(Simd::splat(1), Simd::splat(0));
        }
        total.reduce_sum() as usize
    })
    .sum()
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
pub fn solve_2(_input: &str) -> String {
    let mut result = String::new();
    result.push('*').unwrap();
    result
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> String {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT), 3);
    }

    #[cfg(feature = "simd")]
    #[test]
    fn same_results_1_simd() {
        assert_eq!(solve_1_simd::<64>(INPUT), 3);
    }

    #[cfg(all(feature = "simd", feature = "input"))]
    #[test]
    fn same_results_1_normal_vs_simd() {
        assert_eq!(solve_1_simd::<64>(super::INPUT), solve_1(super::INPUT));
    }
}
