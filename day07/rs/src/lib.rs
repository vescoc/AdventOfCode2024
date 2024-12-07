#![no_std]
#![allow(clippy::must_use_candidate)]

#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;

use heapless::Vec as HLVec;

use lazy_static::lazy_static;

type Vec<T> = HLVec<T, 16>;
type Stack<T> = HLVec<T, 32>;

lazy_static! {
    pub static ref INPUT: &'static str = include_str!("../../input");
}

fn solve<F>(input: &str, valid: F) -> u64
where
    F: Fn(u64, &[u64]) -> bool + Sync,
{
    #[cfg(not(target_family = "wasm"))]
    let lines = input.par_lines();

    #[cfg(target_family = "wasm")]
    let lines = input.lines();

    lines
        .filter_map(|line| {
            let mut parts = line.split(": ");

            let target = parts.next().unwrap().parse::<u64>().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            if valid(target, &numbers) {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

/// # Panics
pub fn solve_1(input: &str) -> u64 {
    solve(input, |target, numbers| {
        if let Some((&a, ax)) = numbers.split_first() {
            if ax.is_empty() {
                return a == target;
            }

            let mut stack = Stack::new();
            stack.push((a, ax, 0)).unwrap();
            stack.push((a, ax, 1)).unwrap();

            while let Some((a, ax, op)) = stack.pop() {
                let (&b, bx) = ax.split_first().unwrap();

                let n = match op {
                    0 => a + b,
                    1 => a * b,
                    _ => unreachable!(),
                };

                if bx.is_empty() {
                    if n == target {
                        return true;
                    }
                } else if n <= target {
                    stack.push((n, bx, 0)).unwrap();
                    stack.push((n, bx, 1)).unwrap();
                }
            }
        }

        false
    })
}

/// # Panics
#[allow(clippy::cast_possible_truncation)]
pub fn solve_2(input: &str) -> u64 {
    solve(input, |target, numbers| {
        if let Some((&a, ax)) = numbers.split_first() {
            if ax.is_empty() {
                return a == target;
            }

            let mut stack = Stack::new();
            stack.push((a, ax, 0)).unwrap();
            stack.push((a, ax, 1)).unwrap();
            stack.push((a, ax, 2)).unwrap();

            while let Some((a, ax, op)) = stack.pop() {
                let (&b, bx) = ax.split_first().unwrap();

                let n = match op {
                    0 => a + b,
                    1 => a * b,
                    2 => {
                        let mut bb = b;
                        let mut a = a;
                        while bb > 0 {
                            bb /= 10;
                            a *= 10;
                        }
                        a + b
                    }
                    _ => unreachable!(),
                };

                if bx.is_empty() {
                    if n == target {
                        return true;
                    }
                } else if n <= target {
                    stack.push((n, bx, 0)).unwrap();
                    stack.push((n, bx, 1)).unwrap();
                    stack.push((n, bx, 2)).unwrap();
                }
            }
        }

        false
    })
}

pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

pub fn part_2() -> u64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref INPUT: &'static str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 3749);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 11387);
    }
}
