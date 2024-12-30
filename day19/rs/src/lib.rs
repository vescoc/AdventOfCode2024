#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::{FnvIndexMap, FnvIndexSet, Vec as HLVec};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

type Set<T> = FnvIndexSet<T, 64>;
type Map<K, V> = FnvIndexMap<K, V, 64>;
type Vec<T> = HLVec<T, 4096>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// Panics
pub fn solve_1(input: &str) -> usize {
    solve_1_r(input)
}

pub fn solve_2(input: &str) -> u64 {
    solve_2_r(input)
}

/// # Panics
pub fn solve_1_dp(input: &str) -> usize {
    let (ps, designs) = input.split_once("\n\n").unwrap();

    let mut patterns = Vec::new();
    for pattern in ps.split(", ") {
        patterns.push(pattern).unwrap();
    }

    let have_options = |design: &str| {
        let mut options = [0u64; 64];
        for pattern in &patterns {
            if design.starts_with(pattern) {
                options[pattern.len()] += 1;
            }
        }

        for i in 1..=design.len() {
            let current = options[i];
            if current == 0 {
                continue;
            }

            for pattern in &patterns {
                let len = i + pattern.len();
                if len > design.len() {
                    continue;
                }

                if design[i..].starts_with(pattern) {
                    if len == design.len() {
                        return true;
                    }
                    options[len] += current;
                }
            }
        }

        false
    };

    #[cfg(feature = "parallel")]
    let lines = designs.par_lines();

    #[cfg(not(feature = "parallel"))]
    let lines = designs.lines();

    lines.filter(|&design| have_options(design)).count()
}

/// # Panics
pub fn solve_1_r(input: &str) -> usize {
    fn is_match<'a>(memoize: &mut Set<&'a str>, patterns: &[&str], design: &'a str) -> bool {
        if design.is_empty() {
            return true;
        }

        if memoize.contains(design) {
            return false;
        }

        let result = patterns.iter().any(|pattern| {
            design
                .strip_prefix(pattern)
                .is_some_and(|design| is_match(memoize, patterns, design))
        });

        if result {
            true
        } else {
            memoize.insert(design).unwrap();

            false
        }
    }

    let (ps, designs) = input.split_once("\n\n").unwrap();

    let mut patterns = Vec::new();
    for pattern in ps.split(", ") {
        patterns.push(pattern).unwrap();
    }

    #[cfg(feature = "parallel")]
    let lines = designs.par_lines();

    #[cfg(not(feature = "parallel"))]
    let lines = designs.lines();

    lines
        .filter(|&design| is_match(&mut Set::new(), &patterns, design))
        .count()
}

/// # Panics
pub fn solve_2_r(input: &str) -> u64 {
    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let patterns = patterns.split(", ").collect::<Vec<_>>();

    let count_options = |design: &str| {
        let mut options = [0u64; 64];
        for pattern in &patterns {
            if design.starts_with(pattern) {
                options[pattern.len()] += 1;
            }
        }

        for i in 1..=design.len() {
            let current = options[i];
            if current == 0 {
                continue;
            }

            for pattern in &patterns {
                let len = i + pattern.len();
                if len <= design.len() && design[i..].starts_with(pattern) {
                    options[len] += current;
                }
            }
        }

        options[design.len()]
    };

    #[cfg(feature = "parallel")]
    let lines = designs.par_lines();

    #[cfg(not(feature = "parallel"))]
    let lines = designs.lines();

    lines.map(count_options).sum()
}

/// # Panics
pub fn solve_2_dp(input: &str) -> u64 {
    fn ways<'a>(memoize: &mut Map<&'a str, u64>, patterns: &[&str], design: &'a str) -> u64 {
        if design.is_empty() {
            return 1;
        }

        if let Some(&count) = memoize.get(design) {
            return count;
        }

        let count = patterns
            .iter()
            .filter_map(|pattern| {
                design
                    .strip_prefix(pattern)
                    .map(|design| ways(memoize, patterns, design))
            })
            .sum();

        memoize.insert(design, count).unwrap();

        count
    }

    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let patterns = patterns.split(", ").collect::<Vec<_>>();

    #[cfg(feature = "parallel")]
    let lines = designs.par_lines();

    #[cfg(not(feature = "parallel"))]
    let lines = designs.lines();

    lines
        .map(|design| ways(&mut Map::new(), &patterns, design))
        .sum()
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u64 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn same_results_1_r() {
        assert_eq!(solve_1_r(INPUT), 6);
    }

    #[test]
    fn same_results_1_dp() {
        assert_eq!(solve_1_dp(INPUT), 6);
    }

    #[cfg(feature = "input")]
    #[test]
    fn same_results_1_r_vs_dp() {
        assert_eq!(solve_1_r(super::INPUT), solve_1_dp(super::INPUT));
    }

    #[test]
    fn same_results_2_r() {
        assert_eq!(solve_2_r(INPUT), 16);
    }

    #[test]
    fn same_results_2_dp() {
        assert_eq!(solve_2_dp(INPUT), 16);
    }

    #[cfg(feature = "input")]
    #[test]
    fn same_results_2_r_vs_dp() {
        assert_eq!(solve_2_r(super::INPUT), solve_2_dp(super::INPUT));
    }
}
