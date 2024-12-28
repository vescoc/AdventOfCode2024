#![no_std]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![allow(clippy::must_use_candidate)]

use core::{cmp, convert, iter, ops};

#[cfg(feature = "simd")]
pub mod simd;

#[cfg(feature = "input")]
pub const INPUT: &str = include_str!("../../input");

#[cfg(not(feature = "input"))]
pub const INPUT: &str = "";

type Coord<T> = (T, T);

pub trait Machine<T> {
    const CORRECTION: T;
}

pub struct Machine0;
pub struct Machine1BB;

impl Machine<i32> for Machine0 {
    const CORRECTION: i32 = 0;
}

impl Machine<i64> for Machine1BB {
    #[allow(clippy::unreadable_literal)]
    const CORRECTION: i64 = 10000000000000;
}

fn parse<T>(data: &[u8]) -> T
where
    T: Default,
    T: ops::Add<Output = T> + ops::Mul<Output = T>,
    T: convert::From<u8>,
{
    data.iter().fold(T::default(), |acc, digit| {
        acc * T::from(10) + T::from(digit - b'0')
    })
}

fn parse_machine<T>(machine: &str) -> (Coord<T>, Coord<T>, Coord<T>)
where
    T: Default,
    T: ops::Add<Output = T> + ops::Mul<Output = T>,
    T: convert::From<u8>,
{
    let mut button_a = <(T, T)>::default();
    let mut button_b = <(T, T)>::default();
    let mut prize = <(T, T)>::default();
    for line in machine.lines() {
        if line.starts_with("Button") {
            let mut parts = line.split_whitespace().skip(1);

            let button = parts.next().unwrap().chars().next().unwrap();
            let x = {
                let data = parts.next().unwrap().as_bytes();
                parse(&data[2..data.len() - 1])
            };
            let y = parse(&parts.next().unwrap().as_bytes()[2..]);

            if button == 'A' {
                button_a = (x, y);
            } else {
                button_b = (x, y);
            }
        } else if line.starts_with("Prize: ") {
            let mut parts = line.split_whitespace().skip(1);

            let x = {
                let data = parts.next().unwrap().as_bytes();
                parse(&data[2..data.len() - 1])
            };
            let y = parse(&parts.next().unwrap().as_bytes()[2..]);

            prize = (x, y);
        }
    }

    (button_a, button_b, prize)
}

fn push_machine<M, T>(machine: &str) -> Option<T>
where
    M: Machine<T>,
    T: Default,
    T: Copy,
    T: convert::From<u8>,
    T: ops::Add<Output = T>
        + ops::Mul<Output = T>
        + ops::Sub<Output = T>
        + ops::Rem<Output = T>
        + ops::Neg<Output = T>
        + ops::Div<Output = T>,
    T: cmp::PartialEq<T>,
{
    let ((ax, ay), (bx, by), (px, py)) = parse_machine::<T>(machine);

    let (px, py) = (px + M::CORRECTION, py + M::CORRECTION);

    // ax x + bx y = px
    // ay x + by y = py

    let det = ax * by - ay * bx;
    if det == T::default() {
        return None;
    }

    let num_a = px * by - py * bx;
    if num_a % det != T::default() {
        return None;
    }

    let num_b = ax * py - ay * px;
    if num_b % det != T::default() {
        return None;
    }

    Some(num_a / det * T::from(3) + num_b / det)
}

/// # Panics
#[allow(private_bounds)]
pub fn solve<M, T>(input: &str) -> T
where
    M: Machine<T>,
    T: iter::Sum,
    T: Default,
    T: Copy,
    T: convert::From<u8>,
    T: ops::Add<Output = T>
        + ops::Mul<Output = T>
        + ops::Sub<Output = T>
        + ops::Rem<Output = T>
        + ops::Neg<Output = T>
        + ops::Div<Output = T>,
    T: cmp::PartialEq<T>,
{
    input
        .split("\n\n")
        .filter_map(|line| push_machine::<M, T>(line))
        .sum()
}

/// # Panics
pub fn solve_1(input: &str) -> i32 {
    solve::<Machine0, _>(input)
}

/// # Panics
#[allow(clippy::unreadable_literal)]
pub fn solve_2(input: &str) -> i64 {
    solve::<Machine1BB, _>(input)
}

#[cfg(feature = "input")]
pub fn part_1() -> i32 {
    solve_1(INPUT)
}

#[cfg(feature = "input")]
pub fn part_2() -> i64 {
    solve_2(INPUT)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const INPUT: &str = r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn same_results_1_1() {
        assert_eq!(
            push_machine::<Machine0, _>(
                r"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"
            ),
            Some(280)
        );
    }

    #[test]
    fn same_results_1_2() {
        assert_eq!(
            push_machine::<Machine0, _>(
                r"Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176"
            ),
            None
        );
    }

    #[test]
    fn same_results_1_3() {
        assert_eq!(
            push_machine::<Machine0, _>(
                r"Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450"
            ),
            Some(200)
        );
    }

    #[test]
    fn same_results_1_4() {
        assert_eq!(
            push_machine::<Machine0, _>(
                r"Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            ),
            None
        );
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1(INPUT), 480);
    }
}
