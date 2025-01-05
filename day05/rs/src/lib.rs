#![no_std]
#![allow(clippy::must_use_candidate)]

use heapless::{Vec as HLVec, FnvIndexSet};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

type Num = u8;

type Vec<T> = HLVec<T, 32>;

type HashSet<T> = FnvIndexSet<T, 2048>;

type Rules = HashSet<(Num, Num)>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

fn is_valid(rules: &Rules, pages: &[Num]) -> bool {
    (0..pages.len() - 1).all(|i| {
        pages
            .iter()
            .skip(i)
            .copied()
            .zip(pages.iter().skip(i + 1).copied())
            .all(|p| rules.contains(&p))
    })
}

fn reorder(rules: &Rules, pages: &mut [Num]) {
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
    F: Fn(&Rules, Vec<Num>) -> Option<Num> + Sync + Send,
{
    let mut parts = input.split("\n\n");

    let mut rules = Rules::new();
    for (k, v) in parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            (
                parts.next().unwrap().parse::<Num>().unwrap(),
                parts.next().unwrap().parse::<Num>().unwrap(),
            )
        })
    {
        rules.insert((k, v)).unwrap();
    }

    #[cfg(feature = "parallel")]
    let lines = parts.next().unwrap().par_lines();

    #[cfg(not(feature = "parallel"))]
    let lines = parts.next().unwrap().lines();

    lines
        .filter_map(|line| {
            let pages = line
                .split(',')
                .map(|page| page.parse::<Num>().unwrap())
                .collect::<Vec<_>>();

            check(&rules, pages).map(u32::from)
        })
        .sum()
}

/// # Panics
#[cfg_attr(target_os = "none", inline(never))]
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
#[cfg_attr(target_os = "none", inline(never))]
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

#[cfg(feature = "input")]
pub fn part_1() -> u32 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u32 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"47|53
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
97,13,75,29,47";

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT), 143);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(INPUT), 123);
    }
}
