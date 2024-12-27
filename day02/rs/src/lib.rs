#![no_std]
#![allow(clippy::must_use_candidate)]

use core::cmp::Ordering;

use heapless::Vec;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use itertools::Itertools;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

trait SafeReport: Iterator<Item = u32> + Sized {
    fn safe_report(self) -> bool {
        let mut direction = None;
        self.tuple_windows()
            .all(|(a, b)| match (a.cmp(&b), &mut direction) {
                (Ordering::Less, Some(direction)) if *direction == Ordering::Less => {
                    (1..=3).contains(&(b - a))
                }
                (Ordering::Less, None) => {
                    if (1..=3).contains(&(b - a)) {
                        direction = Some(Ordering::Less);
                        true
                    } else {
                        false
                    }
                }
                (Ordering::Greater, Some(direction)) if *direction == Ordering::Greater => {
                    (1..=3).contains(&(a - b))
                }
                (Ordering::Greater, None) => {
                    if (1..=3).contains(&(a - b)) {
                        direction = Some(Ordering::Greater);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            })
    }
}

impl<I: Iterator<Item = u32>> SafeReport for I {}

/// # Panics
pub fn solve_1(input: &str) -> usize {
    #[cfg(feature = "parallel")]
    let lines = input.par_lines();

    #[cfg(not(feature = "parallel"))]
    let lines = input.lines();

    lines
        .filter(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<u32>().unwrap())
                .safe_report()
        })
        .count()
}

/// # Panics
pub fn solve_2(input: &str) -> usize {
    #[cfg(feature = "parallel")]
    let lines = input.par_lines();

    #[cfg(not(feature = "parallel"))]
    let lines = input.lines();

    lines
        .filter(|line| {
            let samples = line
                .split_whitespace()
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<_, 16>>();
            if samples.iter().copied().safe_report() {
                return true;
            }

            (0..samples.len()).any(|i| {
                samples
                    .iter()
                    .enumerate()
                    .filter_map(|(j, sample)| if i == j { None } else { Some(*sample) })
                    .safe_report()
            })
        })
        .count()
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> usize {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    const INPUT_2: &str = r"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT_1), 2);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(INPUT_2), 4);
    }
}
