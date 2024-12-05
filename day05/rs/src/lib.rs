#![no_std]

#![allow(clippy::must_use_candidate)]

use heapless::{Vec as HLVec, FnvIndexSet};

#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;

use lazy_static::lazy_static;

type Vec<T> = HLVec<T, 32>;

type HashSet<T> = FnvIndexSet<T, 2048>;

type Rules = HashSet<(u32, u32)>;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

fn is_valid(rules: &Rules, pages: &[u32]) -> bool {
    (0..pages.len() - 1).all(|i| {
        pages
            .iter()
            .skip(i)
            .copied()
            .zip(pages.iter().skip(i + 1).copied())
            .all(|p| rules.contains(&p))
    })
}

fn reorder(rules: &Rules, pages: &mut [u32]) {
    for i in 1..pages.len() {
        let (prefix, postfix) = pages.split_at_mut(i);
        let x = &mut prefix[i - 1];
        for y in postfix {
            if rules.contains(&(*y, *x)) {
                core::mem::swap(x, y);
            }
        }
    }

    assert!(is_valid(rules, pages));
}

fn solve<F>(input: &str, check: F) -> u32
where
    F: Fn(&Rules, Vec<u32>) -> Option<u32> + Sync + Send,
{
    let mut parts = input.split("\n\n");

    let rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Rules>();

    #[cfg(not(target_family = "wasm"))]
    let lines = parts.next().unwrap().par_lines();

    #[cfg(target_family = "wasm")]
    let lines = parts.next().unwrap().lines();

    lines
        .filter_map(|line| {
            let pages = line
                .split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            check(&rules, pages)
        })
        .sum()
}

/// # Panics
pub fn solve_1(input: &str) -> u32 {
    solve(input, |rules, pages| {
        if is_valid(rules, &pages) {
            Some(pages[pages.len() / 2])
        } else {
            None
        }
    })
}

/// # Panics
pub fn solve_2(input: &str) -> u32 {
    solve(input, |rules, mut pages| {
        if is_valid(rules, &pages) {
            None
        } else {
            reorder(rules, &mut pages);

            Some(pages[pages.len() / 2])
        }
    })
}

pub fn part_1() -> u32 {
    solve_1(&INPUT)
}

pub fn part_2() -> u32 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT: &'static str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 143);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 123);
    }
}
