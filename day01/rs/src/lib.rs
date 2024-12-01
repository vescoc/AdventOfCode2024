#![allow(clippy::must_use_candidate)]

use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

/// # Panics
pub fn solve_1(input: &str) -> u32 {
    let (mut line1, mut line2) = (Vec::with_capacity(1024), Vec::with_capacity(1024));
    for line in input.lines() {
        let mut parts = line.split_whitespace();

        line1.push(parts.next().unwrap().parse::<u32>().unwrap());
        line2.push(parts.next().unwrap().parse::<u32>().unwrap());
    }

    line1.sort_unstable();
    line2.sort_unstable();

    line1
        .into_iter()
        .zip(line2)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

/// # Panics
pub fn solve_2(input: &str) -> u32 {
    let (mut line1, mut line2) = (
        Vec::with_capacity(1024),
        HashMap::<u32, u32>::with_capacity(1024),
    );
    for line in input.lines() {
        let mut parts = line.split_whitespace();

        line1.push(parts.next().unwrap().parse::<u32>().unwrap());

        *line2
            .entry(parts.next().unwrap().parse::<u32>().unwrap())
            .or_default() += 1;
    }

    line1
        .into_iter()
        .map(|id| id * line2.get(&id).copied().unwrap_or_default())
        .sum()
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
        static ref INPUT_1: &'static str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        static ref INPUT_2: &'static str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT_1), 11);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT_2), 31);
    }
}
