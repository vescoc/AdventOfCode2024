#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::{FnvIndexMap, FnvIndexSet, Vec as HLVec};

type Set<T> = FnvIndexSet<T, 64>;
type Map<K, V> = FnvIndexMap<K, V, 64>;
type Vec<T> = HLVec<T, 4096>;

#[cfg(feature = "input")]
use lazy_static::lazy_static;

#[cfg(feature = "input")]
lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

/// # Panics
pub fn solve_1(input: &str) -> usize {
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

    let (patterns, designs) = input.split_once("\n\n").unwrap();

    let patterns = patterns.split(", ").collect::<Vec<_>>();

    designs
        .lines()
        .filter(|&design| is_match(&mut Set::new(), &patterns, design))
        .count()
}

/// # Panics
pub fn solve_2(input: &str) -> usize {
    fn ways<'a>(memoize: &mut Map<&'a str, usize>, patterns: &[&str], design: &'a str) -> usize {
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

    designs
        .lines()
        .map(|design| ways(&mut Map::new(), &patterns, design))
        .sum()
}

#[cfg(feature = "input")]
pub fn part_1() -> usize {
    solve_1(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> usize {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: &'static str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 6);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 16);
    }
}
