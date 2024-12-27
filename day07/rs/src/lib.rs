#![no_std]
#![allow(clippy::must_use_candidate)]

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use heapless::Vec as HLVec;

type Vec<T> = HLVec<T, 16>;
type Stack<T> = HLVec<T, 32>;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

trait Part {
    const OPS: &[u32];
}

struct Part1;
impl Part for Part1 {
    const OPS: &[u32] = &[0, 1];
}

struct Part2;
impl Part for Part2 {
    const OPS: &[u32] = &[0, 1, 2];
}

fn solve<P: Part>(input: &str) -> u64 {
    #[cfg(feature = "parallel")]
    let lines = input.par_lines();

    #[cfg(not(feature = "parallel"))]
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

            if let Some((&a, ax)) = numbers.split_first() {
                let Some((&b, bx)) = ax.split_first() else {
                    if a == target {
                        return Some(target);
                    }

                    return None;
                };

                let mut stack = Stack::new();
                for op in P::OPS {
                    stack.push((a, b, bx, op)).unwrap();
                }

                while let Some((a, b, rx, op)) = stack.pop() {
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

                    if let Some((&b, bx)) = rx.split_first() {
                        if n <= target {
                            for op in P::OPS {
                                stack.push((n, b, bx, op)).unwrap();
                            }
                        }
                    } else if n == target {
                        return Some(target);
                    }
                }
            }

            None
        })
        .sum()
}

/// # Panics
pub fn solve_1(input: &str) -> u64 {
    solve::<Part1>(input)
}

/// # Panics
pub fn solve_2(input: &str) -> u64 {
    solve::<Part2>(input)
}

#[cfg(feature = "input")]
pub fn part_1() -> u64 {
    solve_1(&INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> u64 {
    solve_2(&INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(&INPUT), 3749);
    }

    #[test]
    fn same_results_2() {
        assert_eq!(solve_2(&INPUT), 11387);
    }
}
