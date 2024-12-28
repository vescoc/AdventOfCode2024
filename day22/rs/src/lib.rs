#![no_std]
#![allow(clippy::must_use_candidate)]

use bitset::BitSet;

use itertools::Itertools;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

const SIZE: usize = 19 * 19 * 19 * 19;

#[allow(clippy::unreadable_literal)]
const DIVISOR: u64 = 16777216;

struct Generator(u64);

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let secret = &mut self.0;

        let result = *secret;

        *secret = ((*secret * 64) ^ *secret) % DIVISOR;
        *secret = ((*secret / 32) ^ *secret) % DIVISOR;
        *secret = ((*secret * 2048) ^ *secret) % DIVISOR;

        Some(result)
    }
}

/// # Panics
pub fn solve_1(input: &str) -> u64 {
    #[cfg(feature = "parallel")]
    let lines = input.par_lines();

    #[cfg(not(feature = "parallel"))]
    let lines = input.lines();

    lines
        .map(|line| Generator(line.parse().unwrap()).nth(2000).unwrap())
        .sum()
}

#[allow(clippy::cast_sign_loss)]
fn key((a, b, c, d): &(i32, i32, i32, i32)) -> usize {
    (a + 9) as usize * 19 * 19 * 19
        + (b + 9) as usize * 19 * 19
        + (c + 9) as usize * 19
        + (d + 9) as usize
}

/// # Panics
#[cfg(feature = "parallel")]
#[allow(clippy::cast_possible_truncation)]
pub fn solve_2_par(input: &str) -> u16 {
    use core::sync::atomic::{AtomicU16, Ordering};

    let map = [const { AtomicU16::new(0) }; { SIZE }];
    input.par_lines().for_each(|line| {
        let mut inserted = BitSet::<_, _, { BitSet::with_capacity(SIZE) }>::new(key);
        for ((a, b, c, d), bananas) in Generator(line.parse().unwrap())
            .take(2000)
            .map(|secret| secret % 10)
            .tuple_windows()
            .map(|(a, b)| (b as u16, b as i32 - a as i32))
            .tuple_windows()
            .map(|((_, a), (_, b), (_, c), (bananas, d))| ((a, b, c, d), bananas))
        {
            if !inserted.insert((a, b, c, d)).unwrap() {
                map[key(&(a, b, c, d))].fetch_add(bananas, Ordering::Relaxed);
            }
        }
    });

    map.iter().map(|v| v.load(Ordering::Relaxed)).max().unwrap()
}

/// # Panics
#[cfg(not(feature = "parallel"))]
#[allow(clippy::cast_possible_truncation)]
pub fn solve_2_seq(input: &str) -> u16 {
    let mut map = [0u16; { SIZE }];
    input.lines().for_each(|line| {
        let mut inserted = BitSet::<_, _, { BitSet::with_capacity(SIZE) }>::new(key);
        for ((a, b, c, d), bananas) in Generator(line.parse().unwrap())
            .take(2000)
            .map(|secret| secret % 10)
            .tuple_windows()
            .map(|(a, b)| (b as u16, b as i32 - a as i32))
            .tuple_windows()
            .map(|((_, a), (_, b), (_, c), (bananas, d))| ((a, b, c, d), bananas))
        {
            if !inserted.insert((a, b, c, d)).unwrap() {
                map[key(&(a, b, c, d))] += bananas;
            }
        }
    });

    map.iter().max().copied().unwrap()
}

#[cfg(feature = "parallel")]
pub use solve_2_par as solve_2;

#[cfg(not(feature = "parallel"))]
pub use solve_2_seq as solve_2;

#[cfg(feature = "input")]
pub fn part_1() -> u64 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u16 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r"1
10
100
2024";
    const INPUT_2: &str = r"1
2
3
2024";

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT_1), 37327623);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(INPUT_2), 23);
    }
}
